use futures::StreamExt;
use crate::parser::CsvParser;

mod parser;

#[tokio::main]
async fn main() {
    println!("Hello async world!");

    let input = futures::stream::iter("hello,world,".to_owned().into_bytes());
    let mut stream = CsvParser::new(input);
    while let Some(item) = stream.next().await {
        println!("Received: {:?}", item);
    }
}
