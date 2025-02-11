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
}