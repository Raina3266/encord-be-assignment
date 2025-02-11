#[derive(Debug, Clone)]
pub struct Column {

}

impl Column {
    pub fn new() -> Self {
        todo!()
    }

    pub fn max_height(&self) -> usize {
        todo!()
    }
    
    pub fn get(&self, height: usize) -> bool {
        todo!()
    }

    pub fn set(&mut self, height: usize, value: bool) {
        todo!()
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
}

