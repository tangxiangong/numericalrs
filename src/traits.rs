use num::Complex;
use num::Num;
// use num::traits::real::Real;

pub trait ComplexNum: Num {}

impl ComplexNum for Complex<i32> {}
