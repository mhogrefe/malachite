use std::cmp::Ordering;

use num::arithmetic::traits::UnsignedAbs;
use num::comparison::traits::OrdAbs;

macro_rules! impl_comparison_traits {
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
                self.unsigned_abs().cmp(&other.unsigned_abs())
            }
        }
    };
}

impl_comparison_traits!(i8);
impl_comparison_traits!(i16);
impl_comparison_traits!(i32);
impl_comparison_traits!(i64);
impl_comparison_traits!(i128);
impl_comparison_traits!(isize);
