use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::traits::Zero;

use integer::Integer;
use natural::Natural;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialEq<$t> for Integer {
            /// Determines whether an `Integer` is equal to a value of unsigned primitive integer
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
            #[inline]
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
apply_to_unsigneds!(impl_unsigned);

fn _eq_signed<U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(x: &Integer, other: &S) -> bool
where
    Natural: PartialEq<U>,
{
    x.sign == (*other >= S::ZERO) && x.abs == other.unsigned_abs()
}

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialEq<$t> for Integer {
            /// Determines whether an `Integer` is equal to a value of signed primitive integer
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
                _eq_signed(self, other)
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
apply_to_signeds!(impl_signed);
