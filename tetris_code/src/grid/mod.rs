use column::Column;

use crate::shape::Shape;

mod column;

pub struct Grid {
    columns: Vec<Column>,
}

impl Grid {
    pub fn new(width: usize) -> Self {
        let columns = vec![Column::new(); width];
        Self { columns }
    }

    pub fn max_height(&self) -> usize {
        self.columns
            .iter()
            .map(Column::max_height)
            .max()
            .expect("at least one column")
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
