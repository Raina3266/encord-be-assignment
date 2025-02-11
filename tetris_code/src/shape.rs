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

    pub fn column_heights(&self) -> &'static [usize] {
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
    
    pub fn max_height(&self) -> usize {
        *self.column_heights().iter().max().expect("all shapes have at least 1 column")
    }
}