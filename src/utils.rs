use num;

#[inline]
pub fn approx_equal<T: num::Float>(a: T, b: T) -> bool {
    (a - b).abs() <= T::epsilon()
}
