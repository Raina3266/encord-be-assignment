use std::io::{stdin, stdout, Result};

use tetris::main_loop;

fn main() -> Result<()> {
    main_loop(stdin(), stdout(), 10)
}
