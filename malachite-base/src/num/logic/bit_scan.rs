use num::basic::integers::PrimitiveInteger;
use num::logic::traits::{BitScan, LowMask, TrailingZeros};

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
                Some(if start >= Self::WIDTH {
                    start
                } else {
                    TrailingZeros::trailing_zeros(!(self | $t::low_mask(start)))
                })
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
                if start >= Self::WIDTH {
                    None
                } else {
                    let index = TrailingZeros::trailing_zeros(self & !$t::low_mask(start));
                    if index == Self::WIDTH {
                        None
                    } else {
                        Some(index)
                    }
                }
            }
        }
    };
}

impl_bit_scan_unsigned!(u8);
impl_bit_scan_unsigned!(u16);
impl_bit_scan_unsigned!(u32);
impl_bit_scan_unsigned!(u64);
impl_bit_scan_unsigned!(u128);
impl_bit_scan_unsigned!(usize);

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
                if start >= Self::WIDTH - 1 {
                    if self >= 0 {
                        Some(start)
                    } else {
                        None
                    }
                } else {
                    let index = u64::from((!(self | $t::low_mask(start))).trailing_zeros());
                    if index == $t::WIDTH {
                        None
                    } else {
                        Some(index)
                    }
                }
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
                if start >= Self::WIDTH - 1 {
                    if self >= 0 {
                        None
                    } else {
                        Some(start)
                    }
                } else {
                    let index = TrailingZeros::trailing_zeros(self & !$t::low_mask(start));
                    if index == $t::WIDTH {
                        None
                    } else {
                        Some(index)
                    }
                }
            }
        }
    };
}

impl_bit_scan_signed!(i8);
impl_bit_scan_signed!(i16);
impl_bit_scan_signed!(i32);
impl_bit_scan_signed!(i64);
impl_bit_scan_signed!(i128);
impl_bit_scan_signed!(isize);
