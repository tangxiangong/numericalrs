mod macros;
pub mod nd_array;

pub use nd_array::array::Array;

pub use nd_array::ops;

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
