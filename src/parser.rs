use std::pin::Pin;
use std::task::{Context, Poll};
use futures::{Stream, StreamExt, pin_mut};

#[derive(Debug)]
pub enum Error {
    InvalidUtf8,
    UnexpectedEof,
}

pub struct CsvParser<S> {
    stream: S,
    buffer: Vec<u8>,
}

impl<S> CsvParser<S>
where
    S: Unpin
{
    pub fn new(stream: S) -> Self {
        CsvParser {
            stream,
            buffer: Default::default(),
        }
    }

    fn try_parse_next(&mut self) -> Option<Result<String, Error>> {
        if let Some(comma_pos) = self.buffer.iter().position(|byte| *byte == b',') {
            let result = std::str::from_utf8(&self.buffer[..comma_pos]).map(str::to_owned).map_err(|_| Error::InvalidUtf8);
            self.buffer.drain(..comma_pos + 1);
            return Some(result);
        }

        None
    }
}

// For a first poc, return a string when we reached `,`
impl<S> Stream for CsvParser<S>
where
    S: Stream<Item = u8> + Unpin
{
    type Item = Result<String, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // not eof
        let mut pending = true;

        loop {
            match Pin::new(&mut self.stream).poll_next(cx) {
                Poll::Ready(Some(byte)) => {
                    self.buffer.push(byte);
                }
                Poll::Pending => break,
                Poll::Ready(None) => {
                    pending = false;
                    break;
                },
            }
        }

        if let Some(result) = self.try_parse_next() {
            return Poll::Ready(Some(result));
        } else if pending {
            Poll::Pending
        } else if self.buffer.is_empty() {
            Poll::Ready(None)
        } else {
            self.buffer.clear();
            Poll::Ready(Some(Err(Error::UnexpectedEof)))
        }
    }
}
