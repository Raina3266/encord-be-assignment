#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub enum Shape {
    Q,
    Z,
    S,
    T,
    I,
    L,
    J,
}

// pub enum Direction {
//     S,
//     W,
//     E,
//     N,
// }

// impl Direction {
//     pub fn from_byte(letter: u8) -> Option<Direction> {
//         match letter {
//             b'S' => Some(Direction::S),
//             b'W' => Some(Direction::W),
//             b'E' => Some(Direction::E),
//             b'N' => Some(Direction::N),
//             _ => None,
//         }
//     }

//     fn rotate_90(point: (isize, isize)) -> (isize, isize) {
//         let (x, y) = point;
//         (-y, x)
//     }

//     fn rotate(&self, point: (isize, isize)) -> (isize, isize) {
//         match self {
//             Self::N => point,
//             Self::W => self.rotate(point),
//             Self::S => self.rotate(self.rotate(point)),
//             Self::E => self.rotate(self.rotate(self.rotate(point))),
//         }
//     }
// }

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
    pub fn column_heights(&self) -> impl Iterator<Item = usize> + '_ {
        // TODO(raina): calculate from .cells_occupied()

        (0..10).filter_map(|current_x| {
            let y_value = self
                .cells_occupied()
                .iter()
                .filter(|(x, _y)| *x == current_x)
                .map(|(_x, y)| y);

            let max_y = y_value.clone().max();
            let min_y = y_value.clone().min();

            match (max_y, min_y) {
                (Some(max_y), Some(min_y)) => Some(max_y - min_y + 1),
                _ => None,
            }
        })
    }

    /// The height of the tallest column
    pub fn max_height(&self) -> usize {
        self.column_heights()
            .max()
            .expect("all shapes have at least 1 column")
    }
}

#[cfg(test)]
pub mod proptest_helper {
    use proptest::prelude::*;

    use super::Shape;

    /// strategy that gives random shapes and valid positions for them to be
    pub fn valid_shape() -> impl Strategy<Value = (Shape, u8)> {
        any::<Shape>().prop_flat_map(|shape| {
            let width: u8 = match shape {
                Shape::Q => 2,
                Shape::Z => 3,
                Shape::S => 3,
                Shape::T => 3,
                Shape::I => 4,
                Shape::L => 2,
                Shape::J => 2,
            };

            let valid_position = 0..=(10 - width);

            (Just(shape), valid_position)
        })
    }
}
