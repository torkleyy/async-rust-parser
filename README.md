# Async parser experiment

This is a learning experiment to write an async parser. The following sections describe a minimum parser
which will be implemented to evaluate the feasibility & learn potential difficulties when writing
async parsing code.

## Data format

The parser should accept UTF-8 data containing key value pairs.
The key and value are separated by `=`, while pairs of key & value are separated by a comma (`,`).

In WSN:

````
SYNTAX = { PAIR "," }
PAIR = KEY "=" VALUE
KEY = character { character }
VALUE = digit { digit }
````

## On the Rust side

Parse the incoming data and produce a stream of `Pair`.
