use std::cmp::Ordering;

use num::comparison::traits::OrdAbs;

macro_rules! impl_comparison_traits {
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

impl_comparison_traits!(u8);
impl_comparison_traits!(u16);
impl_comparison_traits!(u32);
impl_comparison_traits!(u64);
impl_comparison_traits!(u128);
impl_comparison_traits!(usize);
