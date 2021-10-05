use num::basic::traits::Zero;
use num::logic::traits::{CheckedHammingDistance, CountOnes, HammingDistance};
use std::ops::BitXor;

fn hamming_distance_unsigned<T: BitXor<Output = T> + CountOnes>(x: T, y: T) -> u64 {
    CountOnes::count_ones(x ^ y)
}

macro_rules! impl_hamming_distance_unsigned {
    ($t:ident) => {
        impl HammingDistance<$t> for $t {
            /// Returns the Hamming distance between `self` and `other`, or the number of bit flips
            /// needed to turn `self` into `other`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::logic::hamming_distance` module.
            #[inline]
            fn hamming_distance(self, other: $t) -> u64 {
                hamming_distance_unsigned(self, other)
            }
        }
    };
}
apply_to_unsigneds!(impl_hamming_distance_unsigned);

fn checked_hamming_distance_signed<T: BitXor<Output = T> + Copy + CountOnes + Ord + Zero>(
    x: T,
    y: T,
) -> Option<u64> {
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
            /// needed to turn `self` into `other`.
            ///
            /// If `self` and `other` have opposite signs, then the number of flips would be
            /// infinite, so the result is `None`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::logic::hamming_distance` module.
            #[inline]
            fn checked_hamming_distance(self, other: $t) -> Option<u64> {
                checked_hamming_distance_signed(self, other)
            }
        }
    };
}
apply_to_signeds!(impl_checked_hamming_distance_signed);
