use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MathError {
    #[error("Error: 纬度不匹配! 左边的维度为 {0}, 而右边的维度为 {1}.")]
    DimensionErr1D(usize, usize),
    #[error("Error: 该操作对空向量非法！")]
    EmptyArrayErr,
    #[error("Error: 参数错误, {0}.")]
    ArgsErr(&'static str),
}
