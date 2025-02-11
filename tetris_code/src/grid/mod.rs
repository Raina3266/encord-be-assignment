use crate::shape::Shape;

pub struct Grid {

}

impl Grid {
    pub fn new(width: usize) -> Self {
        todo!()
    }

    pub fn max_height(&self) -> usize {
        todo!()
    }

    pub fn add_shape(&mut self, shape: Shape, position: u8) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_example() {
        let mut grid = Grid::new(10);

        grid.add_shape(Shape::Q, 0);
        grid.add_shape(Shape::Q, 0);

        assert_eq!(grid.max_height(), 4);
    }
}