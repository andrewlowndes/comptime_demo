# Comptime Example
A demo of using [comptime-rs](https://github.com/andrewlowndes/comptime-rs) to produce clean flexible procedural macros without writing any procedural macros.

## How it works
There are three crates comprising of:
- `test_types` - a library crate that contains some test types enums
- `test_lib` - a library crate that contains functions to generate some Rust code
- `test_example` - a example binary for generating and using the Rust generated code

In the example crate there is a call to the library function inside a `comptime` macro call. The contents of this macro are copied in a separate file and executed (the workspace project libraries are shared and accessible). The resultant Rust code is then output as a token stream from the comptime macro and injected back into the page

## Running the example
Run `cargo run -p test_example` to see the output of the example.
