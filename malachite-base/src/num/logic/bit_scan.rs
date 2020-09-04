use num::basic::integers::PrimitiveInt;
use num::logic::traits::{BitScan, TrailingZeros};

fn _index_of_next_false_bit_unsigned<T: PrimitiveInt>(x: T, start: u64) -> Option<u64> {
    Some(if start >= T::WIDTH {
        start
    } else {
        TrailingZeros::trailing_zeros(!(x | T::low_mask(start)))
    })
}

fn _index_of_next_true_bit_unsigned<T: PrimitiveInt>(x: T, start: u64) -> Option<u64> {
    if start >= T::WIDTH {
        None
    } else {
        let index = TrailingZeros::trailing_zeros(x & !T::low_mask(start));
        if index == T::WIDTH {
            None
        } else {
            Some(index)
        }
    }
}

macro_rules! impl_bit_scan_unsigned {
    ($t:ident) => {
        impl BitScan for $t {
            /// Finds the smallest index of a `false` bit that is greater than or equal to
            /// `starting_index`. Since `$t` is unsigned and therefore has an implicit prefix of
            /// infinitely-many zeros, this function always returns a value.
            ///
            /// Starting beyond the type's width is allowed; the result will be the starting index.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::logic::traits::BitScan;
            ///
            /// assert_eq!(0xb_0000_0000u64.index_of_next_false_bit(0), Some(0));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_false_bit(20), Some(20));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_false_bit(31), Some(31));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_false_bit(32), Some(34));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_false_bit(33), Some(34));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_false_bit(34), Some(34));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_false_bit(35), Some(36));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_false_bit(100), Some(100));
            /// ```
            #[inline]
            fn index_of_next_false_bit(self, start: u64) -> Option<u64> {
                _index_of_next_false_bit_unsigned(self, start)
            }

            /// Finds the smallest index of a `true` bit that is greater than or equal to
            /// `starting_index`.
            ///
            /// If the starting index is greater than or equal to the type's width, the result will
            /// be `None` since there are no `true` bits past that point.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::logic::traits::BitScan;
            ///
            /// assert_eq!(0xb_0000_0000u64.index_of_next_true_bit(0), Some(32));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_true_bit(20), Some(32));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_true_bit(31), Some(32));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_true_bit(32), Some(32));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_true_bit(33), Some(33));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_true_bit(34), Some(35));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_true_bit(35), Some(35));
            /// assert_eq!(0xb_0000_0000u64.index_of_next_true_bit(36), None);
            /// assert_eq!(0xb_0000_0000u64.index_of_next_true_bit(100), None);
            /// ```
            #[inline]
            fn index_of_next_true_bit(self, start: u64) -> Option<u64> {
                _index_of_next_true_bit_unsigned(self, start)
            }
        }
    };
}
apply_to_unsigneds!(impl_bit_scan_unsigned);

fn _index_of_next_false_bit_signed<T: PrimitiveInt>(x: T, start: u64) -> Option<u64> {
    if start >= T::WIDTH - 1 {
        if x >= T::ZERO {
            Some(start)
        } else {
            None
        }
    } else {
        let index = TrailingZeros::trailing_zeros(!(x | T::low_mask(start)));
        if index == T::WIDTH {
            None
        } else {
            Some(index)
        }
    }
}

fn _index_of_next_true_bit_signed<T: PrimitiveInt>(x: T, start: u64) -> Option<u64> {
    if start >= T::WIDTH - 1 {
        if x >= T::ZERO {
            None
        } else {
            Some(start)
        }
    } else {
        let index = TrailingZeros::trailing_zeros(x & !T::low_mask(start));
        if index == T::WIDTH {
            None
        } else {
            Some(index)
        }
    }
}

macro_rules! impl_bit_scan_signed {
    ($t:ident) => {
        impl BitScan for $t {
            /// Finds the smallest index of a `false` bit that is greater than or equal to
            /// `starting_index`.
            ///
            /// If the input is negative and starting index is greater than or equal to the type's
            /// width, the result will be `None` since there are no `false` bits past that point. If
            /// the input is non-negative, the result will be the starting index.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::logic::traits::BitScan;
            ///
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_false_bit(0), Some(0));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_false_bit(20), Some(20));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_false_bit(31), Some(31));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_false_bit(32), Some(34));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_false_bit(33), Some(34));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_false_bit(34), Some(34));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_false_bit(35), None);
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_false_bit(100), None);
            /// ```
            #[inline]
            fn index_of_next_false_bit(self, start: u64) -> Option<u64> {
                _index_of_next_false_bit_signed(self, start)
            }

            /// Finds the smallest index of a `true` bit that is greater than or equal to
            /// `starting_index`.
            ///
            /// If the input is non-negative and starting index is greater than or equal to the
            /// type's width, the result will be `None` since there are no `true` bits past that
            /// point. If the input is negative, the result will be the starting index.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::logic::traits::BitScan;
            ///
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_true_bit(0), Some(32));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_true_bit(20), Some(32));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_true_bit(31), Some(32));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_true_bit(32), Some(32));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_true_bit(33), Some(33));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_true_bit(34), Some(35));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_true_bit(35), Some(35));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_true_bit(36), Some(36));
            /// assert_eq!((-0x5_0000_0000i64).index_of_next_true_bit(100), Some(100));
            /// ```
            #[inline]
            fn index_of_next_true_bit(self, start: u64) -> Option<u64> {
                _index_of_next_true_bit_signed(self, start)
            }
        }
    };
}
apply_to_signeds!(impl_bit_scan_signed);
