use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul};

/// Task 2.1:
/// Define the C32 struct. This struct represents a complex number with 32 bits (per field).
/// The struct should have two fields, `real` and `imag`.
///
/// Task 2.2:
/// Implement the `Debug` trait for debug printing, and the `PartialEq` trait
/// for comparisons (both are required to use the `assert_eq` macro in tests).
/// These can be derived automatically with the `#[derive()]` directive, as long as all the fields
/// implement the traits.






/// Task 2.3:
/// Implement the `Display` trait for C32. This is used to get a string representation of the struct.
/// Hint:
///   `f.write_fmt(format_args!("{}", arg));
/// Examples:
///   4+2i
///   7-9i
///   -2+6i
///   -3-4i







/// Task 2.4:
/// Implement the `Add` trait for C32.
/// Example:
///   (2+5i) + (1-5i) = 2+1 + 5i-5i = 3+0i





/// Task 2.5:
/// Implement the `Mul` trait for C32.
/// Hint:
///   i*i = -1
/// Example:
///   (5+2i)(-2+4i) = 5*-2 + 5*4i + 2i*-2 + 2i*4i = -10 + 20i - 4i + 8i² = -10 - 8 + 20i - 4i = -18 + 16i







/// Task 2.6:
/// Implement the `Mul` trait for multiplying C32 with i32 (both directions)
/// Hint:
///   Mul takes a generic argument
///   You need two `impl` blocks








#[cfg(test)]
mod tests {
    use crate::task2_complex_numbers::*;

    #[test]
    fn test_task2_1_exists() {
        //C32 { real: 5, imag: 2 };
    }

    #[test]
    fn test_task2_2_compare() {
        //assert_eq!(
        //    C32 { real: 9, imag: 15 },
        //    C32 { real: 9, imag: 15 },
        //)
    }

    #[test]
    fn test_task2_3_display() {
        //assert_eq!(
        //    format!("{}", C32 { real: 6, imag: 2 }),
        //    "6+2i",
        //);
        //assert_eq!(
        //    format!("{}", C32 { real: -2, imag: 0 }),
        //    "-2+0i",
        //);
        //assert_eq!(
        //    format!("{}", C32 { real: 27, imag: -9 }),
        //    "27-9i",
        //);
        //assert_eq!(
        //    format!("{}", C32 { real: -17, imag: -42 }),
        //    "-17-42i",
        //);
    }

    #[test]
    fn test_task2_4_add() {
        //assert_eq!(
        //    C32 { real: 2, imag: 8 } + C32 { real: -5, imag: 18 },
        //    C32 { real: -3, imag: 26 },
        //)
    }

    #[test]
    fn test_task2_5_mul() {
        //assert_eq!(
        //    C32 { real: 6, imag: -4 } * C32 { real: -9, imag: -1 },
        //    C32 { real: -58, imag: 30 },
        //)
    }

    #[test]
    fn test_task2_6_mul_i32() {
        //assert_eq!(
        //    C32 { real: 6, imag: -4 } * 3,
        //    C32 { real: 18, imag: -12 },
        //);

        //assert_eq!(
        //    3 * C32 { real: 6, imag: -4 },
        //    C32 { real: 18, imag: -12 },
        //);
    }
}
