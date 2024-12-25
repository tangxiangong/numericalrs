mod macros;
pub mod utils;
pub use utils::*;

pub mod nd_array;

pub use nd_array::array::Array;

pub use nd_array::ops;

pub use nd_array::array::Norm;

pub use nd_array::matrix::Matrix;

pub mod errors;
pub mod polynomial;
pub use errors::*;

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
