use crate::Array;

pub struct Polynomial {
    coefficients: Array,
    roots: Option<Array>,
}

impl Polynomial {
    pub fn new(coefficients: Array) -> Self {
        assert!(!coefficients.is_empty());
        Polynomial {
            coefficients,
            roots: None,
        }
    }
}
