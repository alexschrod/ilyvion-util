//! Functionality for generating permutations

use std::ops::{Add, Mul};

use num_traits::{NumCast, ToPrimitive, Zero};

fn _heap_permutation<
    T: Copy + ToPrimitive,
    R: Mul<R, Output = R> + Add<R, Output = R> + Zero + NumCast,
>(
    digits: &mut [T],
    remaining_operations: usize,
    digit_length: usize,
    result: &mut Vec<R>,
) {
    if remaining_operations == 1 {
        result.push(digits.iter().fold(R::zero(), |sum, d| {
            sum * R::from(10).unwrap() + R::from(*d).unwrap()
        }));
        return;
    }

    for i in 0..remaining_operations {
        _heap_permutation(digits, remaining_operations - 1, digit_length, result);

        if remaining_operations % 2 == 1 {
            digits.swap(0, remaining_operations - 1);
        } else {
            digits.swap(i, remaining_operations - 1);
        }
    }
}

/// Creates every permutation of all digits in `digits`. Shuffles `digits` during use; no guarantees
/// are made about its elements' order upon completion.
pub fn heap_permutation<
    T: Copy + ToPrimitive,
    R: Mul<R, Output = R> + Add<R, Output = R> + Zero + NumCast,
>(
    digits: &mut [T],
) -> Vec<R> {
    let digits_len = digits.len();
    let mut result = vec![];
    _heap_permutation(digits, digits_len, digits_len, &mut result);

    result
}

#[cfg(test)]
mod tests {
    use super::heap_permutation;

    #[test]
    fn two() {
        let mut digits: Vec<f64> = vec![1., 2.];
        let sut = heap_permutation(&mut digits);
        assert_eq!(vec![12., 21.], sut);
    }

    #[test]
    fn three() {
        let mut digits = vec![1, 2, 3];
        let sut = heap_permutation(&mut digits);
        assert_eq!(vec![123, 213, 312, 132, 231, 321], sut);
    }
}
