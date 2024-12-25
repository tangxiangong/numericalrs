use crate::approx_equal;
use super::array::Array;
use super::errors::MathError;
use std::ops::{Add, Mul, Neg, Sub};

impl Add for &Array {
    type Output = Result<Array, MathError>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.size() != rhs.size() {
            Err(MathError::DimensionErr1D(self.size(), rhs.size()))
        } else if self.is_empty() {
            Err(MathError::EmptyArrayErr)
        } else {
            let result: Array = self.iter().zip(rhs.iter()).map(|(&a, &b)| a + b).collect();
            Ok(result)
        }
    }
}

impl Add<f64> for &Array {
    type Output = Result<Array, MathError>;
    fn add(self, rhs: f64) -> Self::Output {
        if self.is_empty() {
            Err(MathError::EmptyArrayErr)
        } else {
            let result: Array = self.iter().map(|&v| v + rhs).collect();
            Ok(result)
        }
    }
}

impl Add<i32> for &Array {
    type Output = Result<Array, MathError>;
    fn add(self, rhs: i32) -> Self::Output {
        self + (rhs as f64)
    }
}

impl Add<&Array> for f64 {
    type Output = Result<Array, MathError>;
    fn add(self, rhs: &Array) -> Self::Output {
        rhs + self
    }
}

impl Add<&Array> for i32 {
    type Output = Result<Array, MathError>;
    fn add(self, rhs: &Array) -> Self::Output {
        rhs + self
    }
}

impl Neg for &Array {
    type Output = Result<Array, MathError>;
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

impl Sub for &Array {
    type Output = Result<Array, MathError>;
    fn sub(self, rhs: Self) -> Self::Output {
        let neg_rhs = (-rhs)?;
        self + &neg_rhs
    }
}

impl Sub<f64> for &Array {
    type Output = Result<Array, MathError>;
    fn sub(self, rhs: f64) -> Self::Output {
        self + (-rhs)
    }
}

impl Sub<i32> for &Array {
    type Output = Result<Array, MathError>;
    fn sub(self, rhs: i32) -> Self::Output {
        self + (-rhs)
    }
}

impl Sub<&Array> for f64 {
    type Output = Result<Array, MathError>;
    fn sub(self, rhs: &Array) -> Self::Output {
        let neg_rhs = (-rhs)?;
        &neg_rhs + self
    }
}

impl Sub<&Array> for i32 {
    type Output = Result<Array, MathError>;
    fn sub(self, rhs: &Array) -> Self::Output {
        let neg_rhs = (-rhs)?;
        &neg_rhs + self
    }
}

impl Mul for &Array {
    type Output = Result<f64, MathError>;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.size() != rhs.size() {
            Err(MathError::DimensionErr1D(self.size(), rhs.size()))
        } else if self.is_empty() {
            Err(MathError::EmptyArrayErr)
        } else {
            let result: f64 = self
                .iter()
                .zip(rhs.iter())
                .fold(0.0, |acc, (&a, &b)| acc + a * b);
            Ok(result)
        }
    }
}

impl Mul<f64> for &Array {
    type Output = Result<Array, MathError>;
    fn mul(self, rhs: f64) -> Self::Output {
        if self.is_empty() {
            Err(MathError::EmptyArrayErr)
        } else if self.is_zeros() || approx_equal(rhs, 0.0) {
            Ok(Array::zeros(self.size()))
        } else {
            let result: Array = self.iter().map(|&v| v * rhs).collect();
            Ok(result)
        }
    }
}

impl Mul<i32> for &Array {
    type Output = Result<Array, MathError>;
    fn mul(self, rhs: i32) -> Self::Output {
        self * (rhs as f64)
    }
}

impl Mul<&Array> for f64 {
    type Output = Result<Array, MathError>;
    fn mul(self, rhs: &Array) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Array> for i32 {
    type Output = Result<Array, MathError>;
    fn mul(self, rhs: &Array) -> Self::Output {
        rhs * self
    }
}
