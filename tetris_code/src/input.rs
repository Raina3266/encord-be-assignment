use std::io::{BufReader, Bytes, Read, Result};

use crate::shape::Shape;

#[derive(Debug, PartialEq, Eq)]
pub enum InputToken {
    End,
    NewLine,
    Entry { shape: Shape, position: u8 },
}

/// A wrapper around something that implements [`Read`] that produces [`InputToken`]s
/// 
/// It's like an `Iterator`, but uses a custom function [`Input::next_token`].
pub struct Input<R: Read> {
    // Iterator of Result<u8>
    //
    // BufReader adds buffering. Rust IO is unbuffered by default
    inner: Bytes<BufReader<R>>,
}

impl<R: Read> Input<R> {
    pub fn new(inner: R) -> Self {
        Self {
            inner: BufReader::new(inner).bytes(),
        }
    }

    pub fn next_token(&mut self) -> Result<InputToken> {
        let Some(next_byte) = self.inner.next() else {
            return Ok(InputToken::End);
        };

        let mut next_byte = next_byte?;

        // handle special cases
        match next_byte {
            b'\n' => return Ok(InputToken::NewLine),
            // if it's a comma, skip one byte
            b',' => next_byte = self.inner.next().expect("input is valid")?,
            _ => {}
        }

        let shape = Shape::from_byte(next_byte).expect("input is valid");
        let position = self.inner.next().expect("input is valid")?;
        let position = position - b'0';
        
        Ok(InputToken::Entry { shape, position })

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_parses_input() {
        let mut input = Input::new("Q0,Z1\nQ0".as_bytes());

        assert_eq!(
            input.next_token().unwrap(),
            InputToken::Entry {
                shape: Shape::Q,
                position: 0
            }
        );

        assert_eq!(
            input.next_token().unwrap(),
            InputToken::Entry {
                shape: Shape::Z,
                position: 1,
            }
        );

        assert_eq!(input.next_token().unwrap(), InputToken::NewLine);

        assert_eq!(
            input.next_token().unwrap(),
            InputToken::Entry {
                shape: Shape::Q,
                position: 0
            }
        );

        assert_eq!(input.next_token().unwrap(), InputToken::End);
    }
}
