use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub enum MathError {
    DimensionErr1D(usize, usize),
    EmptyArrayErr,
    ArgsErr(&'static str),
}

impl std::error::Error for MathError {}

impl Display for MathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DimensionErr1D(lhs, rhs) => write!(
                f,
                "Error: 纬度不匹配! 左边的维度为 {}, 而右边的维度为 {}.",
                lhs, rhs
            ),
            MathError::EmptyArrayErr => write!(f, "Error: 该操作对空向量非法！"),
            MathError::ArgsErr(s) => write!(f, "Error: 参数错误, {s}."),
        }
    }
}

impl Debug for MathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DimensionErr1D(lhs, rhs) => write!(
                f,
                "纬度不匹配! 左边的维度为 {}, 而右边的维度为 {}.",
                lhs, rhs
            ),
            MathError::EmptyArrayErr => write!(f, "该操作对空向量非法！"),
            MathError::ArgsErr(s) => write!(f, "参数错误, {s}."),
        }
    }
}
