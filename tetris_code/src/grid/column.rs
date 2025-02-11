#[derive(Debug, Clone)]
pub struct Column {
    cells: Vec<bool>,
}

impl Column {
    pub fn new() -> Self {
        Self { cells: vec![] }
    }

    pub fn max_height(&self) -> usize {
        self.cells
            .iter()
            .enumerate()
            .rev() // iterate from end, more efficient in tall columns
            .filter_map(|(index, value)| match value {
                true => Some(index),
                false => None,
            })
            .map(|index| index + 1)
            .next()
            .unwrap_or(0)
    }

    pub fn get(&self, height: usize) -> bool {
        self.cells.get(height).copied().unwrap_or(false)
    }

    pub fn set(&mut self, height: usize, value: bool) {
        if height >= self.cells.len() {
            self.cells.resize_with(height + 1, || false);
        }

        self.cells[height] = value;
    }

    pub fn remove(&mut self, height: usize) {
        self.cells.remove(height);
    }
}

#[cfg(test)]
mod tests {
    use std::usize;

    use super::*;

    #[test]
    fn simple_get_set() {
        let mut col = Column::new();

        // can get columns before setting
        assert_eq!(col.get(5), false);
        assert_eq!(col.get(usize::MAX), false);

        col.set(5, true);
        assert_eq!(col.get(5), true);
    }

    #[test]
    fn max_height_works() {
        let mut col = Column::new();

        assert_eq!(col.max_height(), 0);

        col.set(4, true);
        assert_eq!(col.max_height(), 5);

        col.set(4, false);
        assert_eq!(col.max_height(), 0);

        col.set(9, true);
        assert_eq!(col.max_height(), 10);
    }
}
