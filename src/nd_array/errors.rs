use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub struct DimensionError1D {
    lhs: usize,
    rhs: usize,
}

impl DimensionError1D {
    pub fn new(lhs: usize, rhs: usize) -> DimensionError1D {
        DimensionError1D { lhs, rhs }
    }
}


impl Display for DimensionError1D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "纬度不匹配! 左边的维度为 {}, 而右边的维度为 {}.", self.lhs, self.rhs)
    }
}

impl Debug for DimensionError1D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "纬度不匹配! 左边的维度为 {}, 而右边的维度为 {}.", self.lhs, self.rhs)
    }
}


impl std::error::Error for DimensionError1D {}
