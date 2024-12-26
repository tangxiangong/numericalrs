// use crate::MathError;
// use num::traits::real::Real;
use num::Num;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Array<T: Num = f64> {
    data: Vec<T>,
}

impl<T: Num> Deref for Array<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Num> DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: Num> Default for Array<T> {
    fn default() -> Self {
        Self {
            data: Vec::<T>::new(),
        }
    }
}

impl<T: Num + Clone> Array<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::<T>::with_capacity(capacity),
        }
    }
    pub fn zeros(size: usize) -> Self {
        Self {
            data: vec![T::zero(); size],
        }
    }

    pub fn ones(size: usize) -> Self {
        Self {
            data: vec![T::one(); size],
        }
    }

    // pub fn linspace(start: f64, end: f64, nums: usize) -> Result<Self, MathError> {
    //     if start >= end {
    //         Err(MathError::ArgsErr("第一个参数要小于第二个参数"))
    //     } else {
    //         let h = (end - start) / nums as f64;
    //         let arr: Array = (0..nums).map(|idx| idx as f64 * h).collect();
    //         Ok(arr)
    //     }
    // }

    pub fn is_zeros(&self) -> bool {
        !self.iter().any(|v| !v.is_zero())
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

//TODO 范数

// impl<T: Real> Array<T> {
//     pub fn l1_norm(&self) -> Result<T, MathError> {
//         if self.is_empty() {
//             Err(MathError::EmptyArrayErr)
//         } else {
//             let result = self.iter().fold(T::zero(), |acc, &x| acc + x.abs());
//             Ok(result)
//         }
//     }
// }

// impl<T: ComplexNum> Array<T> {
//     pub fn l1_norm(&self) -> Result<T, MathError> {
//         if self.is_empty() {
//             Err(MathError::EmptyArrayErr)
//         } else {
//             let result = self.iter().fold(T::zero(), |acc, &x| acc + x.norm());
//             Ok(result)
//         }
//     }
// }

// pub fn norm(&self, space: Norm) -> Result<f64, MathError> {
//     if self.is_empty() {
//         return Err(MathError::EmptyArrayErr);
//     }
//     match space {
//         Norm::L1 => {
//
//         }
//         Norm::L2 => {
//             let result = self
//                 .iter()
//                 .fold(T::zero(), |acc, &x| acc + x * x)
//                 .to_f64()
//                 .unwrap()
//                 .sqrt();
//             Ok(result)
//         }
//         Norm::LInf => {
//             let mut iter = self.iter().map(|&v| v.abs());
//             let mut result = iter.next().unwrap();
//             for v in iter {
//                 if v >= result {
//                     result = v;
//                 }
//             }
//             Ok(result.to_f64().unwrap())
//         }
//     }
// }

impl<T: Num + Clone> Clone for Array<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<T: Num + Debug + Clone> Display for Array<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "1-d Array {{{:?}, size={}}}", self.data, self.size())
    }
}

impl<T: Num, V> From<V> for Array<T>
where
    V: Into<Vec<T>>,
{
    fn from(value: V) -> Self {
        Self { data: value.into() }
    }
}

impl<T: Num> FromIterator<T> for Array<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let v = Vec::from_iter(iter);
        Self { data: v }
    }
}

impl<T: Num + Clone> PartialEq for Array<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() {
            false
        } else if self.is_zeros() {
            true
        } else {
            !self.iter().zip(other.iter()).any(|(a, b)| !a.eq(b))
        }
    }
}

impl<T: Num + Copy + Clone> Eq for Array<T> {}
