#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Q,
    Z,
    S,
    T,
    I,
    L,
    J,
}

impl Shape {
    /// Makes a `Shape` from an ASCII byte for its letter
    pub fn from_byte(letter: u8) -> Option<Shape> {
        match letter {
            b'Q' => Some(Shape::Q),
            b'Z' => Some(Shape::Z),
            b'S' => Some(Shape::S),
            b'T' => Some(Shape::T),
            b'I' => Some(Shape::I),
            b'L' => Some(Shape::L),
            b'J' => Some(Shape::J),
            _ => None,
        }
    }

    /// Which squares does this shape occupy, relative to the bottom left corner
    /// 
    /// `(0, 0)` is the bottom left corner
    /// `(1, 0)` is one square to the right
    /// `(0, 1)` is one square up
    pub fn cells_occupied(&self) -> &'static [(usize, usize)] {
        match self {
            Self::Q => &[(0, 0), (0, 1), (1, 0), (1, 1)],
            Self::Z => &[(0, 1), (1, 0), (1, 1), (2, 0)],
            Self::S => &[(0, 0), (1, 0), (1, 1), (2, 1)],
            Self::T => &[(0, 1), (1, 0), (1, 1), (2, 1)],
            Self::I => &[(0, 0), (1, 0), (2, 0), (3, 0)],
            Self::L => &[(0, 0), (0, 1), (0, 2), (1, 0)],
            Self::J => &[(0, 0), (1, 0), (1, 1), (1, 2)],
        }
    }

    /// The height of each column in the shape from the bottom
    pub fn column_heights(&self) -> &'static [usize] {
        // TODO(raina): calculate from .cells_occupied()
        match self {
            Self::Q => &[2, 2],
            Self::Z => &[1, 2, 2],
            Self::S => &[2, 2, 1],
            Self::T => &[1, 2, 1],
            Self::I => &[1, 1, 1, 1],
            Self::L => &[3, 3],
            Self::J => &[3, 3],
        }
    }
    
    /// The height of the tallest column
    pub fn max_height(&self) -> usize {
        *self.column_heights().iter().max().expect("all shapes have at least 1 column")
    }
}