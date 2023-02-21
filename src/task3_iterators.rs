use std::iter::Sum;
use std::ops::Rem;

/// # Useful references
/// - https://doc.rust-lang.org/book/ch13-02-iterators.html
/// - https://doc.rust-lang.org/std/iter/trait.Iterator.html

/// Task 3.1:
/// This function should increment every element in a list.
///
/// Hint: The vec can be converted to an iterator with `into_vec()`.
/// Then use `map()` to modify it, then `collect()` to turn it into a vec again.
fn plus_one(list: Vec<i32>) -> Vec<i32> {
    todo!()
}


/// Task 3.2:
/// This function should return the sum of the even numbers in the iterator.
/// Odd numbers should be ignored.
///
/// Usually functions that do iterator operations don't take a vec as an argument. Instead
/// they take the generic `IntoIterator` trait, so the caller can pass any compatible collection type,
/// or even an existing iterator.
///
/// Hint: `IntoIter` only has one method, `into_iter`. Use this to get an useful iterator.
/// Look up the documentation of the `Iterator` trait (link at line 3) for useful iterator adaptors.
fn sum_even_i32<I>(iter: I) -> i32 {
    todo!()
}

/// Task 3.3: Advanced, feel free to skip
/// The previous function only accepted iterators of i32, but there are many other integer types.
///
/// This function should take an iterator of any type that supports the modulo (%) operation.
/// This operation is implemented through the `Rem` trait.
///
/// Hint: You might need `TryFrom` to cast the constant numbers to the right type.
/// You probably need `PartialEq` and `Sum`.
fn sum_even<I, E>(iter: I) -> E {
    todo!()
}

/// Task 3.4:
/// This function should find the longest subsequence of increasing numbers in an iterator.
/// The numbers do not have to be consecutive.
///
/// This function uses generics to accept any iterator of elements that can be ordered.
///
/// # Examples
/// ```
/// assert_eq!(
///     longest_increasing_subsequence(vec![1, 2, 3]),
///     3,
/// );
///
/// assert_eq!(
///     longest_increasing_subsequence(vec![1]),
///     1,
/// );
///
/// assert_eq!(
///     longest_increasing_subsequence(vec![1.4, 5.1, 2.284, 13.1857, 17.17599, 10.0]),
///     3,
/// );
/// ```
fn longest_increasing_subsequence<I>(iter: I) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::task3_iterators::*;

    #[test]
    fn test_task3_1_plus_one() {
        //assert_eq!(
        //    plus_one(vec![1, 2, 3, 4]),
        //    vec![2, 3, 4, 5],
        //)
    }

    #[test]
    fn test_task3_2_sum_even_i32() {
        //assert_eq!(
        //    sum_even_i32(vec![1, 2, 3, 4, 5, 6, 7]),
        //    12,
        //);

        //assert_eq!(
        //    sum_even_i32(vec![1, 5, 3, 9, -1, 23, 41]),
        //    0,
        //);
    }

    #[test]
    fn test_task3_3_sum_even() {
        //let vec_i32: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
        //assert_eq!(sum_even(vec_i32), 12);

        //let vec_u64: Vec<u64> = vec![1, 5, 3, 9, 1, 23, 41];
        //assert_eq!(sum_even(vec_u64), 0);
    }

    #[test]
    fn test_task3_4_longest_increasing_subsequence() {
        //assert_eq!(
        //    longest_increasing_subsequence(
        //        vec![1, 3, 6, 2, 5, 1, 4, 5, 6, 8, 3]
        //    ),
        //    5,
        //);

        //assert_eq!(
        //    longest_increasing_subsequence(
        //        vec![8, 7, 6, 5, 4, 3, 2, 1]
        //    ),
        //    1,
        //);
    }
}