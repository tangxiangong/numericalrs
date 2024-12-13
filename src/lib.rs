mod macros;
pub mod nd_array;

pub use nd_array::array::Array;

pub use nd_array::ops;

pub use nd_array::array::Norm;

pub use nd_array::matrix::Matrix;

pub mod polynomial;

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
