use crate::nd_array::errors::MathError;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::slice::Iter;

#[inline]
pub fn approx_equal(a: f64, b: f64) -> bool {
    (a - b).abs() <= f64::EPSILON
}

pub enum Norm {
    L1,
    L2,
    LInf,
}

// TODO line space 等方法

#[derive(Debug)]
pub struct Array {
    data: Vec<f64>,
}

impl Array {
    pub fn new() -> Array {
        let data = Vec::<f64>::new();
        Array { data }
    }

    pub fn zero(size: usize) -> Array {
        Array::from(vec![0.0; size])
    }

    pub fn one(size: usize) -> Array {
        let data = vec![1.0; size];
        Array { data }
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn is_zero(&self) -> bool {
        for &v in self.iter() {
            if approx_equal(v, 0.0) {
                return true;
            }
        }
        false
    }

    pub fn push(&mut self, value: f64) {
        self.data.push(value)
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> Iter<'_, f64> {
        self.data.iter()
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
                let mut result = self[0];
                for &v in self.iter() {
                    if v > result {
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

impl From<Vec<i32>> for Array {
    fn from(vec: Vec<i32>) -> Array {
        let data = vec.iter().map(|&v| v as f64).collect();
        Array { data }
    }
}

impl From<Vec<f64>> for Array {
    fn from(data: Vec<f64>) -> Array {
        Array { data }
    }
}

impl<const N: usize> From<[f64; N]> for Array {
    fn from(arr: [f64; N]) -> Array {
        let data = Vec::from(arr);
        Array { data }
    }
}

impl<const N: usize> From<[i32; N]> for Array {
    fn from(arr: [i32; N]) -> Array {
        let data = Vec::from(arr);
        Array::from(data)
    }
}

impl Deref for Array {
    type Target = [f64];
    fn deref(&self) -> &Self::Target {
        &(self.data)
    }
}

impl DerefMut for Array {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut (self.data)
    }
}

impl Index<usize> for Array {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &(self.data[index])
    }
}

impl IndexMut<usize> for Array {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// TODO 切片操作
// impl<T> ops::Index<Range<usize>> for Array<T>
// where T: Num + Copy + Clone {
//     type Output = Array<T>;
//     fn index(&self, index: Range<usize>) -> &Self::Output {
//         let data = self.data[index];
//     }
// }

impl FromIterator<f64> for Array {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        let v = Vec::from_iter(iter);
        Array::from(v)
    }
}

// use crate::nd_array::errors::*;
// use num::Num;
// use std::fmt;
// use std::fmt::{Debug, Display, Formatter};
// use std::ops::{Add, Deref, DerefMut, Index, IndexMut};
// use std::slice::Iter;
//
// #[derive(Debug)]
// pub struct Array<T: Num + Copy + Clone> {
//     data: Vec<T>,
// }
//
// impl<T> Array<T>
// where
//     T: Num + Copy + Clone,
// {
//     pub fn new() -> Array<T> {
//         let data = Vec::<T>::new();
//         Array { data }
//     }
//
//     pub fn zero(size: usize) -> Array<T> {
//         Array::from(vec![T::zero(); size])
//     }
//
//     pub fn one(size: usize) -> Array<T> {
//         let data = vec![T::one(); size];
//         Array { data }
//     }
//
//     pub fn push(&mut self, value: T) {
//         self.data.push(value)
//     }
//
//     pub fn size(&self) -> usize {
//         self.data.len()
//     }
//
//     pub fn iter(&self) -> Iter<'_, T> {
//         self.data.iter()
//     }
//
//     pub fn sin(&self) -> Array<T> {
//         self.iter().map(|&v| v).collect()
//     }
// }
//
// impl<T> Display for Array<T>
// where
//     T: Num + Copy + Clone + Debug,
// {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "1-d Array {{{:?}, size={}}}", self.data, self.size())
//     }
// }
//
// impl<T> From<Vec<T>> for Array<T>
// where
//     T: Num + Copy + Clone,
// {
//     fn from(data: Vec<T>) -> Array<T> {
//         Array { data }
//     }
// }
//
// impl<T, const N: usize> From<[T; N]> for Array<T>
// where
//     T: Num + Copy + Clone,
// {
//     fn from(arr: [T; N]) -> Array<T> {
//         let data = Vec::from(arr);
//         Array { data }
//     }
// }
//
// impl<T> Deref for Array<T>
// where
//     T: Num + Copy + Clone,
// {
//     type Target = [T];
//     fn deref(&self) -> &Self::Target {
//         &(self.data)
//     }
// }
//
// impl<T> DerefMut for Array<T>
// where
//     T: Num + Copy + Clone,
// {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut (self.data)
//     }
// }
//
// impl<T> Index<usize> for Array<T>
// where
//     T: Num + Copy + Clone,
// {
//     type Output = T;
//     fn index(&self, index: usize) -> &Self::Output {
//         &(self.data[index])
//     }
// }
//
// impl<T> IndexMut<usize> for Array<T>
// where
//     T: Num + Copy + Clone,
// {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         &mut self.data[index]
//     }
// }
//
// // TODO 切片操作
// // impl<T> ops::Index<Range<usize>> for Array<T>
// // where T: Num + Copy + Clone {
// //     type Output = Array<T>;
// //     fn index(&self, index: Range<usize>) -> &Self::Output {
// //         let data = self.data[index];
// //     }
// // }
//
// impl<T> Add for &Array<T>
// where
//     T: Num + Copy + Clone,
// {
//     type Output = Result<Array<T>, DimensionError1D>;
//     fn add(self, rhs: Self) -> Self::Output {
//         if self.size() != rhs.size() {
//             Err(DimensionError1D::new(self.size(), rhs.size()))
//         } else {
//             let n = self.size();
//             let mut result = Array::<T>::zero(n);
//             for k in 0..n {
//                 result[k] = self[k] + rhs[k];
//             }
//             Ok(result)
//         }
//     }
// }
//
// impl<T> FromIterator<T> for Array<T>
// where T: Num + Copy + Clone
// {
//     fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
//         let v = Vec::from_iter(iter);
//         Array::from(v)
//     }
// }
