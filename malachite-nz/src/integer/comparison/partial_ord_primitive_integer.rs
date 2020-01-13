use std::cmp::Ordering;

use malachite_base::num::arithmetic::traits::UnsignedAbs;

use integer::Integer;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialOrd<$t> for Integer {
            /// Compares an `Integer` to a value of unsigned primitive integer type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::integer::Integer;
            ///
            /// assert!(Integer::from(-123) < 122u64);
            /// assert!(Integer::from(-123) <= 122u64);
            /// assert!(Integer::from(-123) < 124u64);
            /// assert!(Integer::from(-123) <= 124u64);
            /// assert!(Integer::trillion() > 123u64);
            /// assert!(Integer::trillion() >= 123u64);
            /// assert!(-Integer::trillion() < 123u64);
            /// assert!(-Integer::trillion() <= 123u64);
            /// ```
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                if self.sign {
                    self.abs.partial_cmp(other)
                } else {
                    Some(Ordering::Less)
                }
            }
        }

        impl PartialOrd<Integer> for $t {
            /// Compares a value of unsigned primitive integer type to an `Integer`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::integer::Integer;
            ///
            /// assert!(122u64 > Integer::from(-123));
            /// assert!(122u64 >= Integer::from(-123));
            /// assert!(124u64 > Integer::from(-123));
            /// assert!(124u64 >= Integer::from(-123));
            /// assert!(123u64 < Integer::trillion());
            /// assert!(123u64 <= Integer::trillion());
            /// assert!(123u64 > -Integer::trillion());
            /// assert!(123u64 >= -Integer::trillion());
            /// ```
            #[inline]
            fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialOrd<$t> for Integer {
            /// Compares an `Integer` to a value of signed primitive integer type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::integer::Integer;
            ///
            /// assert!(Integer::from(-123) < -122i64);
            /// assert!(Integer::from(-123) <= -122i64);
            /// assert!(Integer::from(-123) > -124i64);
            /// assert!(Integer::from(-123) >= -124i64);
            /// assert!(Integer::trillion() > 123i64);
            /// assert!(Integer::trillion() >= 123i64);
            /// assert!(-Integer::trillion() < 123i64);
            /// assert!(-Integer::trillion() <= 123i64);
            /// ```
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                if self.sign {
                    if *other >= 0 {
                        self.abs.partial_cmp(&other.unsigned_abs())
                    } else {
                        Some(Ordering::Greater)
                    }
                } else if *other >= 0 {
                    Some(Ordering::Less)
                } else {
                    other.unsigned_abs().partial_cmp(&self.abs)
                }
            }
        }

        impl PartialOrd<Integer> for $t {
            /// Compares a value of signed primitive integer type to an `Integer`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::integer::Integer;
            ///
            /// assert!(-122i64 > Integer::from(-123));
            /// assert!(-122i64 >= Integer::from(-123));
            /// assert!(-124i64 < Integer::from(-123));
            /// assert!(-124i64 <= Integer::from(-123));
            /// assert!(123i64 < Integer::trillion());
            /// assert!(123i64 <= Integer::trillion());
            /// assert!(123i64 > -Integer::trillion());
            /// assert!(123i64 >= -Integer::trillion());
            /// ```
            #[inline]
            fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}

impl_unsigned!(u8);
impl_unsigned!(u16);
impl_unsigned!(u32);
impl_unsigned!(u64);
impl_unsigned!(usize);

impl_signed!(i8);
impl_signed!(i16);
impl_signed!(i32);
impl_signed!(i64);
impl_signed!(isize);
