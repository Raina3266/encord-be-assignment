# Solution

## Building

Install Rust by following instructions here: https://www.rust-lang.org/tools/install and then run `rustup default stable`

To build the program, run `tetris_code/build.sh` from the root of the git repo. This will compile the program and copy it to `./tetris`

## Running tests

`cd tetris_code` and then run `cargo test --release`. There is a test that takes a minute to run because it is testing a very big input, so running in release mode is recommended.