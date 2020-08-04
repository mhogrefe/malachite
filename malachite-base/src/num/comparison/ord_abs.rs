use std::cmp::Ordering;

use num::arithmetic::traits::UnsignedAbs;
use num::comparison::traits::{OrdAbs, PartialOrdAbs};

fn _partial_cmp_abs<T: OrdAbs>(x: &T, y: &T) -> Option<Ordering> {
    Some(x.cmp_abs(y))
}

/// This macro defines trait implementations that are the same for unsigned and signed types.
macro_rules! impl_partial_ord_abs {
    ($t:ident) => {
        impl PartialOrdAbs<$t> for $t {
            /// Compare the absolute values of `self` and `other`, taking both by reference. The
            /// `PartialOrdAbs` interface allows for pairs of incomparable elements, but for
            /// primitive integers these never occur.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::comparison::traits::PartialOrdAbs;
            /// use std::cmp::Ordering;
            ///
            /// assert_eq!(123i32.partial_cmp_abs(&-456), Some(Ordering::Less));
            /// assert_eq!(123i32.partial_cmp_abs(&-123), Some(Ordering::Equal));
            /// ```
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                _partial_cmp_abs(self, other)
            }
        }
    };
}
apply_to_primitive_ints!(impl_partial_ord_abs);

macro_rules! impl_ord_abs_unsigned {
    ($t:ident) => {
        impl OrdAbs for $t {
            /// Compare the absolute values of `self` and `other`, taking both by reference. For
            /// unsigned values, this is the same as ordinary comparison.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::comparison::traits::OrdAbs;
            /// use std::cmp::Ordering;
            ///
            /// assert_eq!(123u32.cmp_abs(&456), Ordering::Less);
            /// assert_eq!(123u32.cmp_abs(&123), Ordering::Equal);
            /// ```
            #[inline]
            fn cmp_abs(&self, other: &Self) -> Ordering {
                self.cmp(other)
            }
        }
    };
}
apply_to_unsigneds!(impl_ord_abs_unsigned);

fn _cmp_abs_signed<T: Copy + UnsignedAbs>(x: &T, y: &T) -> Ordering
where
    <T as UnsignedAbs>::Output: Ord,
{
    x.unsigned_abs().cmp(&y.unsigned_abs())
}

macro_rules! impl_ord_abs_signed {
    ($t:ident) => {
        impl OrdAbs for $t {
            /// Compare the absolute values of `self` and `other`, taking both by reference.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::comparison::traits::OrdAbs;
            /// use std::cmp::Ordering;
            ///
            /// assert_eq!(123i32.cmp_abs(&-456), Ordering::Less);
            /// assert_eq!(123i32.cmp_abs(&-123), Ordering::Equal);
            /// ```
            #[inline]
            fn cmp_abs(&self, other: &Self) -> Ordering {
                _cmp_abs_signed(self, other)
            }
        }
    };
}
apply_to_signeds!(impl_ord_abs_signed);
