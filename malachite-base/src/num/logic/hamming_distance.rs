use std::ops::BitXor;

use num::basic::traits::Zero;
use num::logic::traits::{CheckedHammingDistance, CountOnes, HammingDistance};

#[inline]
pub fn _hamming_distance_unsigned<T: CountOnes>(x: T, y: T) -> u64
where
    T: BitXor<Output = T>,
{
    CountOnes::count_ones(x ^ y)
}

macro_rules! impl_hamming_distance_unsigned {
    ($t:ident) => {
        impl HammingDistance<$t> for $t {
            /// Returns the Hamming distance between `self` and `other`, or the number of bit flips
            /// needed to turn `self` into `other`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::HammingDistance;
            ///
            /// assert_eq!(123u32.hamming_distance(456), 6);
            /// assert_eq!(0u8.hamming_distance(255), 8);
            /// ```
            #[inline]
            fn hamming_distance(self, other: $t) -> u64 {
                _hamming_distance_unsigned(self, other)
            }
        }
    };
}
apply_to_unsigneds!(impl_hamming_distance_unsigned);

#[inline]
pub fn _checked_hamming_distance_signed<T: Copy + CountOnes + Ord + Zero>(x: T, y: T) -> Option<u64>
where
    T: BitXor<Output = T>,
{
    if (x >= T::ZERO) == (y >= T::ZERO) {
        Some(CountOnes::count_ones(x ^ y))
    } else {
        None
    }
}

macro_rules! impl_checked_hamming_distance_signed {
    ($t:ident) => {
        impl CheckedHammingDistance<$t> for $t {
            /// Returns the Hamming distance between `self` and `other`, or the number of bit flips
            /// needed to turn `self` into `other`. If `self` and `other` have opposite signs, then
            /// the number of flips would be infinite, so the result is `None`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::CheckedHammingDistance;
            ///
            /// assert_eq!(123i32.checked_hamming_distance(456), Some(6));
            /// assert_eq!(0i8.checked_hamming_distance(127), Some(7));
            /// assert_eq!(0i8.checked_hamming_distance(-1), None);
            /// ```
            #[inline]
            fn checked_hamming_distance(self, other: $t) -> Option<u64> {
                _checked_hamming_distance_signed(self, other)
            }
        }
    };
}
apply_to_signeds!(impl_checked_hamming_distance_signed);
