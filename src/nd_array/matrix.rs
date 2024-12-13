#[derive(Debug)]
pub struct Matrix {
    data: Vec<f64>,
    size: (usize, usize), // row, column
}

impl Matrix {
    pub fn new(size: (usize, usize)) -> Matrix {
        let (row, column) = size;
        if row == 0 || column == 0 {
            let data = Vec::<f64>::new();
            Matrix { data, size: (0, 0) }
        } else {
            let data = vec![0.0; row * column];
            Matrix { data, size }
        }
    }
}
