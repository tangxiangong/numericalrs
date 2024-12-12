use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub enum MathError {
    DimensionErr1D(usize, usize),
    EmptyArrayErr,
}

impl std::error::Error for MathError {}

impl Display for MathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self, f)
        // match self {
        //     MathError::DimensionErr1D(lhs, rhs) => write!(
        //         f,
        //         "纬度不匹配! 左边的维度为 {}, 而右边的维度为 {}.",
        //         lhs, rhs
        //     ),
        //     MathError::EmptyArrayErr => write!(
        //         f,
        //         "该操作对空向量非法！"
        //     )
        // }
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
        }
    }
}

// pub struct DimensionError1D {
//     lhs: usize,
//     rhs: usize,
// }

// impl DimensionError1D {
//     pub fn new(lhs: usize, rhs: usize) -> DimensionError1D {
//         DimensionError1D { lhs, rhs }
//     }
// }

// impl Display for DimensionError1D {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "纬度不匹配! 左边的维度为 {}, 而右边的维度为 {}.",
//             self.lhs, self.rhs
//         )
//     }
// }

// impl Debug for DimensionError1D {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "纬度不匹配! 左边的维度为 {}, 而右边的维度为 {}.",
//             self.lhs, self.rhs
//         )
//     }
// }
//
// impl std::error::Error for DimensionError1D {}
//
//
//
// pub struct EmptyArrayError {}

// impl Display for EmptyArrayError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "该操作对空向量非法！"
//         )
//     }
// }
//
// impl Debug for EmptyArrayError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "该操作对空向量非法！"
//         )
//     }
// }
//
// impl std::error::Error for EmptyArrayError {}
