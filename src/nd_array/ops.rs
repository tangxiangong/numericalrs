use super::array::Array;
use crate::MathError;
use num::traits::real::Real;
use std::ops::{Add, Mul, Neg, Sub};

//TODO broadcast

impl<T: Real> Add for &Array<T> {
    type Output = Result<Array<T>, MathError>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.size() != rhs.size() {
            Err(MathError::DimensionErr1D(self.size(), rhs.size()))
        } else if self.is_empty() {
            Err(MathError::EmptyArrayErr)
        } else {
            let result: Array<_> = self.iter().zip(rhs.iter()).map(|(&a, &b)| a + b).collect();
            Ok(result)
        }
    }
}

impl<T: Real> Add<T> for &Array<T> {
    type Output = Result<Array<T>, MathError>;
    fn add(self, rhs: T) -> Self::Output {
        if self.is_empty() {
            Err(MathError::EmptyArrayErr)
        } else {
            let result: Array<_> = self.iter().map(|&v| v + rhs).collect();
            Ok(result)
        }
    }
}

// impl<T: num::traits::int::PrimInt> Add<&Array<T>> for i32 {
//     type Output = Result<Array<T>, MathError>;
//     fn add(self, rhs: &Array<T>) -> Self::Output {
//         rhs + self
//     }
// }

impl<T: Real> Neg for &Array<T> {
    type Output = Result<Array<T>, MathError>;
    fn neg(self) -> Self::Output {
        if self.is_empty() {
            Err(MathError::EmptyArrayErr)
        } else if self.is_zeros() {
            Ok(Array::zeros(self.size()))
        } else {
            Ok(self.iter().map(|&x| -x).collect())
        }
    }
}

impl<T: Real> Sub for &Array<T> {
    type Output = Result<Array<T>, MathError>;
    fn sub(self, rhs: Self) -> Self::Output {
        let neg_rhs = (-rhs)?;
        self + &neg_rhs
    }
}

// impl Sub<f64> for &Array {
//     type Output = Result<Array, MathError>;
//     fn sub(self, rhs: f64) -> Self::Output {
//         self + (-rhs)
//     }
// }

// impl Sub<i32> for &Array {
//     type Output = Result<Array, MathError>;
//     fn sub(self, rhs: i32) -> Self::Output {
//         self + (-rhs)
//     }
// }

// impl Sub<&Array> for f64 {
//     type Output = Result<Array, MathError>;
//     fn sub(self, rhs: &Array) -> Self::Output {
//         let neg_rhs = (-rhs)?;
//         &neg_rhs + self
//     }
// }

// impl Sub<&Array> for i32 {
//     type Output = Result<Array, MathError>;
//     fn sub(self, rhs: &Array) -> Self::Output {
//         let neg_rhs = (-rhs)?;
//         &neg_rhs + self
//     }
// }

impl<T: Real> Mul for &Array<T> {
    type Output = Result<T, MathError>;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.size() != rhs.size() {
            Err(MathError::DimensionErr1D(self.size(), rhs.size()))
        } else if self.is_empty() {
            Err(MathError::EmptyArrayErr)
        } else {
            let result = self
                .iter()
                .zip(rhs.iter())
                .fold(T::zero(), |acc, (&a, &b)| acc.add(a * b));
            Ok(result)
        }
    }
}

// impl Mul<f64> for &Array {
//     type Output = Result<Array, MathError>;
//     fn mul(self, rhs: f64) -> Self::Output {
//         if self.is_empty() {
//             Err(MathError::EmptyArrayErr)
//         } else if self.is_zeros() || approx_equal(rhs, 0.0) {
//             Ok(Array::zeros(self.size()))
//         } else {
//             let result: Array = self.iter().map(|&v| v * rhs).collect();
//             Ok(result)
//         }
//     }
// }

// impl Mul<i32> for &Array {
//     type Output = Result<Array, MathError>;
//     fn mul(self, rhs: i32) -> Self::Output {
//         self * (rhs as f64)
//     }
// }
//
// impl Mul<&Array> for f64 {
//     type Output = Result<Array, MathError>;
//     fn mul(self, rhs: &Array) -> Self::Output {
//         rhs * self
//     }
// }
//
// impl Mul<&Array> for i32 {
//     type Output = Result<Array, MathError>;
//     fn mul(self, rhs: &Array) -> Self::Output {
//         rhs * self
//     }
// }
