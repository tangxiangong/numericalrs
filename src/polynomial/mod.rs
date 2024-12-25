use crate::Array;
use num::traits::real::Real;

pub struct Polynomial<T: Real> {
    coefficient: Array<T>,
    root: Option<Array<T>>,
}

impl<T: Real> Polynomial<T> {
    pub fn new(coefficient: Array<T>) -> Self {
        assert!(!coefficient.is_empty());
        Polynomial {
            coefficient,
            root: None,
        }
    }

    pub fn coe(&self) -> &Array<T> {
        &self.coefficient
    }

    pub fn root(&self) -> Option<&Array<T>> {
        self.root.as_ref()
    }
}
