use std::io::{Read, Result};

use crate::shape::Shape;

#[derive(Debug, PartialEq, Eq)]
pub enum InputToken {
    End,
    NewLine,
    Entry { shape: Shape, position: usize },
}

pub struct Input<R: Read> {
    inner: R,
}

impl<R: Read> Input<R> {
    pub fn new(inner: R) -> Self {
        todo!()
    }

    pub fn next_token(&mut self) -> Result<InputToken> {
        todo!()
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
