use std::ops::Range;

use column::Column;

use crate::shape::Shape;

mod column;

/// A tetris grid
pub struct Grid {
    columns: Vec<Column>,
}

impl Grid {
    pub fn new(width: usize) -> Self {
        let columns = vec![Column::new(); width];
        Self { columns }
    }

    /// The highest filled square in any column
    pub fn max_height(&self) -> usize {
        self.columns
            .iter()
            .map(Column::max_height)
            .max()
            .expect("at least one column")
    }

    /// Insert a shape at a particular position
    /// 
    /// For example, calling `grid.add_shape(Shape::Q, 5)` will put the "Q" shape in column 5
    pub fn add_shape(&mut self, shape: Shape, position: u8) {
        let height = self.calculate_insert_height(shape, position);
        let modified_rows = self.fill_in_cells(shape, position, height);
        self.remove_full_rows(modified_rows);
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

    /// This function returns a range of rows that were modified, which is given to `remove_full_rows`
    /// This means `remove_full_rows` only has to check a small number of rows, instead of checking the entire grid
    /// 
    /// The `insert_and_clear` benchmark took 20ms before this change, and took 300us after, so its a lot faster
    fn fill_in_cells(&mut self, shape: Shape, position: u8, height: usize) -> Range<usize> {
        let mut min_y = usize::MAX;
        let mut max_y = 0;

        for (x, y) in shape.cells_occupied() {
            let x = x + position as usize;
            let y = y + height;
            self.columns[x].set(y, true);

            min_y = std::cmp::min(min_y, y);
            max_y = std::cmp::max(max_y, y);
        }

        min_y..(max_y + 1)
    }

    fn remove_full_rows(&mut self, range: Range<usize>) {
        // go backwards to avoid skipping rows
        for i in range.rev() {
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
