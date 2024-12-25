use crate::approx_equal;
use crate::MathError;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};

pub enum Norm {
    L1,
    L2,
    LInf,
}

#[derive(Debug, Default)]
pub struct Array {
    data: Vec<f64>,
}

impl Deref for Array {
    type Target = Vec<f64>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Array {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Array {
    pub fn new() -> Array {
        Array::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Array {
            data: Vec::with_capacity(capacity),
        }
    }
    pub fn zeros(size: usize) -> Array {
        Array::from(vec![0.0; size])
    }

    pub fn ones(size: usize) -> Array {
        let data = vec![1.0; size];
        Array { data }
    }

    pub fn linspace(start: f64, end: f64, nums: usize) -> Result<Array, MathError> {
        if start >= end {
            Err(MathError::ArgsErr("第一个参数要小于第二个参数"))
        } else {
            let h = (end - start) / nums as f64;
            let arr: Array = (0..nums).map(|idx| idx as f64 * h).collect();
            Ok(arr)
        }
    }

    pub fn is_zeros(&self) -> bool {
        !self.iter().any(|&v| !approx_equal(v, 0.0))
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn norm(&self, space: Norm) -> Result<f64, MathError> {
        if self.is_empty() {
            return Err(MathError::EmptyArrayErr);
        }
        match space {
            Norm::L1 => {
                let result = self.iter().fold(0.0, |acc, &x| acc + x.abs()).sqrt();
                Ok(result)
            }
            Norm::L2 => {
                let result = self.iter().fold(0.0, |acc, &x| acc + x * x).sqrt();
                Ok(result)
            }
            Norm::LInf => {
                let mut iter = self.iter().map(|&v| v.abs());
                let mut result = iter.next().unwrap();
                for v in iter {
                    if v >= result {
                        result = v;
                    }
                }
                Ok(result)
            }
        }
    }
}

impl Clone for Array {
    fn clone(&self) -> Self {
        Array::from(self.data.clone())
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "1-d Array {{{:?}, size={}}}", self.data, self.size())
    }
}

impl<T> From<T> for Array
where
    T: Into<Vec<f64>>,
{
    fn from(value: T) -> Self {
        Self { data: value.into() }
    }
}

impl FromIterator<f64> for Array {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        let v = Vec::from_iter(iter);
        Array::from(v)
    }
}

impl PartialEq for Array {
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() {
            false
        } else if self.is_zeros() {
            true
        } else {
            !self
                .iter()
                .zip(other.iter())
                .any(|(&a, &b)| !approx_equal(a, b))
        }
    }
}

impl Eq for Array {}
