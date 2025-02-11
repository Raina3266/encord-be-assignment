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
        let height = self.calculate_insert_height(shape, position);
        self.fill_in_cells(shape, position, height);
        self.remove_full_rows();
    }

    fn calculate_insert_height(&self, shape: Shape, position: u8) -> usize {
        // iterator of "highest point in column + height of shape"
        let total_heights =
            shape
                .column_heights()
                .iter()
                .enumerate()
                .map(|(shape_column, shape_height)| {
                    let column_index = shape_column + position as usize;
                    let column = &self.columns[column_index];
                    column.max_height() + shape_height
                });

        let column_plus_shape_height = total_heights
            .max()
            .expect("all shapes have at least 1 column");

        column_plus_shape_height - shape.max_height()
    }

    fn fill_in_cells(&mut self, shape: Shape, position: u8, height: usize) {
        for (x, y) in shape.cells_occupied() {
            let x = x + position as usize;
            let y = y + height;
            self.columns[x].set(y, true);
        }
    }

    fn remove_full_rows(&mut self) {
        // go backwards to avoid skipping rows
        for i in (0..self.max_height()).rev() {
            if self.row_is_filled(i) {
                self.remove_row(i);
            }
        }
    }

    fn remove_row(&mut self, height: usize) {
        for col in &mut self.columns {
            col.remove(height);
        }
    }

    fn row_is_filled(&self, height: usize) -> bool {
        for col in &self.columns {
            if !col.get(height) {
                return false;
            }
        }
        
        true
    }

    #[cfg(test)]
    pub fn debug_print(&self) {
        let height = self.max_height() + 1;
        for y in (0..height).rev() {
            print!("|");
            for col in &self.columns {
                let char = if col.get(y) { '#' } else { ' ' };
                print!("{char}");
            }
            println!("|");
        }
        println!("{}", "-".repeat(self.columns.len() + 2))
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
        grid.debug_print();

        assert_eq!(grid.max_height(), 4);
    }
}
