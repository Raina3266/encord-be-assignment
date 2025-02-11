#![allow(clippy::bool_assert_comparison)]

use std::io::{stdin, stdout, Read, Result, Write};

use grid::Grid;
use input::{Input, InputToken};

mod grid;
mod input;
mod shape;

#[cfg(test)]
mod tests;

fn main() -> Result<()> {
    main_loop(stdin(), stdout(), 10)
}

/// The main program logic
/// 
/// This function works with anything that implements [`Read`] or [`Write`], which could be [`stdin`] or [`stdout`]
fn main_loop<R: Read, W: Write>(input: R, mut output: W, width: usize) -> Result<()> {
    let mut input = Input::new(input);
    let mut grid = Grid::new(width);

    loop {
        match input.next_token()? {
            InputToken::End => return Ok(()),
            InputToken::NewLine => {
                writeln!(output, "{}", grid.max_height())?;
                grid = Grid::new(width);
            }
            InputToken::Entry { shape, position } => {
                grid.add_shape(shape, position);
            }
        }
    }
}
