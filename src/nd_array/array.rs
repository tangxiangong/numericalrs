use crate::nd_array::errors::*;
use num::Num;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
pub struct Array<T: Num + Copy + Clone> {
    data: Vec<T>,
}

impl<T> Array<T>
where
    T: Num + Copy + Clone,
{
    pub fn new() -> Array<T> {
        let data = Vec::<T>::new();
        Array { data }
    }

    pub fn zero(size: usize) -> Array<T> {
        Array::from(vec![T::zero(); size])
    }

    pub fn one(size: usize) -> Array<T> {
        let data = vec![T::one(); size];
        Array { data }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value)
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl<T> Display for Array<T>
where
    T: Num + Copy + Clone + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "1-d Array {{{:?}, size={}}}", self.data, self.size())
    }
}

impl<T> From<Vec<T>> for Array<T>
where
    T: Num + Copy + Clone,
{
    fn from(data: Vec<T>) -> Array<T> {
        Array { data }
    }
}

impl<T, const N: usize> From<[T; N]> for Array<T>
where
    T: Num + Copy + Clone,
{
    fn from(arr: [T; N]) -> Array<T> {
        let data = Vec::from(arr);
        Array { data }
    }
}

impl<T> Deref for Array<T>
where
    T: Num + Copy + Clone,
{
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &(self.data)
    }
}

impl<T> DerefMut for Array<T>
where
    T: Num + Copy + Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut (self.data)
    }
}

impl<T> Index<usize> for Array<T>
where
    T: Num + Copy + Clone,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &(self.data[index])
    }
}

impl<T> IndexMut<usize> for Array<T>
where
    T: Num + Copy + Clone,
{
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

impl<T> Add for &Array<T>
where
    T: Num + Copy + Clone,
{
    type Output = Result<Array<T>, DimensionError1D>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.size() != rhs.size() {
            Err(DimensionError1D::new(self.size(), rhs.size()))
        } else {
            let n = self.size();
            let mut result = Array::<T>::zero(n);
            for k in 0..n {
                result[k] = self[k] + rhs[k];
            }
            Ok(result)
        }
    }
}
