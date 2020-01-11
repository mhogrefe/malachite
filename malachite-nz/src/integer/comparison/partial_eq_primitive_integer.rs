use malachite_base::num::arithmetic::traits::UnsignedAbs;

use integer::Integer;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialEq<$t> for Integer {
            /// Determines whether an `Integer` is equal to a a value of unsigned primitive integer
            /// type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::integer::Integer;
            ///
            /// assert!(Integer::from(123) == 123u64);
            /// assert!(Integer::from(-123) != 123u64);
            /// ```
            fn eq(&self, other: &$t) -> bool {
                self.sign && self.abs == *other
            }
        }

        impl PartialEq<Integer> for $t {
            /// Determines whether a value of unsigned primitive integer type is equal to an
            /// `Integer`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::integer::Integer;
            ///
            /// assert!(123u64 == Integer::from(123));
            /// assert!(123u64 != Integer::from(-123));
            /// ```
            #[inline]
            fn eq(&self, other: &Integer) -> bool {
                other == self
            }
        }
    };
}

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialEq<$t> for Integer {
            /// Determines whether an `Integer` is equal to a a value of signed primitive integer
            /// type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::integer::Integer;
            ///
            /// assert!(Integer::from(123) != 23i64);
            /// assert!(Integer::from(-123) == -123i64);
            /// ```
            fn eq(&self, other: &$t) -> bool {
                self.sign == (*other >= 0) && self.abs == other.unsigned_abs()
            }
        }

        impl PartialEq<Integer> for $t {
            /// Determines whether a value of signed primitive integer type is equal to an
            /// `Integer`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::integer::Integer;
            ///
            /// assert!(23i64 != Integer::from(123));
            /// assert!(-123i64 == Integer::from(-123));
            /// ```
            #[inline]
            fn eq(&self, other: &Integer) -> bool {
                other == self
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
