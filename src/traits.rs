use num::Complex;
use num::Num;

pub trait ComplexNum: Num {
    fn calc_norm(&self) -> f64;
    fn calc_square(&self) -> f64;
}

impl ComplexNum for Complex<i32> {
    fn calc_norm(&self) -> f64 {
        (self.norm_sqr() as f64).sqrt()
    }
    fn calc_square(&self) -> f64 {
        self.norm_sqr() as f64
    }
}

impl ComplexNum for Complex<f64> {
    fn calc_norm(&self) -> f64 {
        Complex::norm(*self)
    }
    fn calc_square(&self) -> f64 {
        self.norm_sqr()
    }
}

pub trait Norm {
    fn norm(&self) -> f64;
    fn square(&self) -> f64;
}

impl<T: ComplexNum> Norm for T {
    fn norm(&self) -> f64 {
        self.calc_norm()
    }

    fn square(&self) -> f64 {
        self.calc_square()
    }
}

impl Norm for i32 {
    fn norm(&self) -> f64 {
        self.abs() as f64
    }

    fn square(&self) -> f64 {
        (self * self) as f64
    }
}

impl Norm for f64 {
    fn norm(&self) -> f64 {
        self.abs()
    }

    fn square(&self) -> f64 {
        self * self
    }
}
