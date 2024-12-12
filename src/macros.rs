#[macro_export]
macro_rules! array {
    () => (
        $crate::nd_array::Array::new()
    );
    ($elem:expr; $n:expr) => (
        $crate::Array::from([$elem as f64; $n])
    );
    ($($x:expr),+ $(,)?) => (
        $crate::Array::from(vec![$($x as f64),+])
    );
}
