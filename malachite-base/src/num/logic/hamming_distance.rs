use num::logic::traits::{CheckedHammingDistance, CountOnes, HammingDistance};

macro_rules! impl_hamming_distance_unsigned {
    ($t:ident) => {
        impl HammingDistance<$t> for $t {
            /// Returns the Hamming distance between `self` and `rhs`, or the number of bit flips
            /// needed to turn `self` into `rhs`.
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
                CountOnes::count_ones(self ^ other)
            }
        }
    };
}

impl_hamming_distance_unsigned!(u8);
impl_hamming_distance_unsigned!(u16);
impl_hamming_distance_unsigned!(u32);
impl_hamming_distance_unsigned!(u64);
impl_hamming_distance_unsigned!(u128);
impl_hamming_distance_unsigned!(usize);

macro_rules! impl_checked_hamming_distance_signed {
    ($t:ident) => {
        impl CheckedHammingDistance<$t> for $t {
            /// Returns the Hamming distance between `self` and `rhs`, or the number of bit flips
            /// needed to turn `self` into `rhs`. If `self` and `rhs` have opposite signs, then the
            /// number of flips would be infinite, so the result is `None`.
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
                if (self >= 0) == (other >= 0) {
                    Some(CountOnes::count_ones(self ^ other))
                } else {
                    None
                }
            }
        }
    };
}

impl_checked_hamming_distance_signed!(i8);
impl_checked_hamming_distance_signed!(i16);
impl_checked_hamming_distance_signed!(i32);
impl_checked_hamming_distance_signed!(i64);
impl_checked_hamming_distance_signed!(i128);
impl_checked_hamming_distance_signed!(isize);
