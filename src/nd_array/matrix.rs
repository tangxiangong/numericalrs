use num::traits::real::Real;

#[derive(Debug)]
pub struct Matrix<T: Real> {
    data: Vec<T>,
    size: (usize, usize), // row, column
}

impl<T: Real> Matrix<T> {
    pub fn new(size: (usize, usize)) -> Self {
        let (row, column) = size;
        if row == 0 || column == 0 {
            let data = Vec::<T>::new();
            Matrix { data, size: (0, 0) }
        } else {
            let data = vec![T::zero(); row * column];
            Matrix { data, size }
        }
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    pub fn get(&self, row: usize, col: usize) -> Option<T> {
        if (row >= self.size.0) | (col >= self.size.1) {
            None
        } else {
            Some(self.data[self.size.1 * row + col])
        }
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if (row >= self.size.0) | (col >= self.size.1) {
            None
        } else {
            Some(&mut self.data[self.size.1 * row + col])
        }
    }
}
