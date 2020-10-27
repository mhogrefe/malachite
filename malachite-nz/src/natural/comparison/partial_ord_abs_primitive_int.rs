use std::cmp::Ordering;

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::comparison::traits::PartialOrdAbs;

use natural::Natural;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for Natural {
            /// Compares a `Natural` to a value of unsigned primitive integer type. Since both
            /// values are non-negative, this is the same as ordinary `partial_cmp`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// extern crate malachite_base;
            ///
            /// use malachite_base::num::comparison::traits::PartialOrdAbs;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert!(Natural::from(122u32).lt_abs(&123u64));
            /// assert!(Natural::from(122u32).le_abs(&123u64));
            /// assert!(Natural::from(123u32).lt_abs(&124u64));
            /// assert!(Natural::from(123u32).le_abs(&124u64));
            /// assert!(Natural::trillion().gt_abs(&123u64));
            /// assert!(Natural::trillion().ge_abs(&123u64));
            /// ```
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                self.partial_cmp(other)
            }
        }

        impl PartialOrdAbs<Natural> for $t {
            /// Compares a value of unsigned primitive integer type to a `Natural`. Since both
            /// values are non-negative, this is the same as ordinary `partial_cmp`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// extern crate malachite_base;
            ///
            /// use malachite_base::num::comparison::traits::PartialOrdAbs;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert!(123u64.gt_abs(&Natural::from(122u32)));
            /// assert!(123u64.ge_abs(&Natural::from(122u32)));
            /// assert!(124u64.gt_abs(&Natural::from(123u32)));
            /// assert!(124u64.ge_abs(&Natural::from(123u32)));
            /// assert!(123u64.lt_abs(&Natural::trillion()));
            /// assert!(123u64.le_abs(&Natural::trillion()));
            /// ```
            #[inline]
            fn partial_cmp_abs(&self, other: &Natural) -> Option<Ordering> {
                self.partial_cmp(other)
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for Natural {
            /// Compares a `Natural` to the absolute value of a value of signed primitive integer
            /// type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// extern crate malachite_base;
            ///
            /// use malachite_base::num::comparison::traits::PartialOrdAbs;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert!(Natural::from(122u32).lt_abs(&-123i64));
            /// assert!(Natural::from(122u32).le_abs(&-123i64));
            /// assert!(Natural::from(124u32).gt_abs(&-123i64));
            /// assert!(Natural::from(124u32).ge_abs(&-123i64));
            /// assert!(Natural::trillion().gt_abs(&123i64));
            /// assert!(Natural::trillion().ge_abs(&123i64));
            /// ```
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                self.partial_cmp(&other.unsigned_abs())
            }
        }

        impl PartialOrdAbs<Natural> for $t {
            /// Compares the absolute value of a value of signed primitive integer type to a
            /// `Natural`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// extern crate malachite_base;
            ///
            /// use malachite_base::num::comparison::traits::PartialOrdAbs;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert!((-123i64).gt_abs(&Natural::from(122u32)));
            /// assert!((-123i64).ge_abs(&Natural::from(122u32)));
            /// assert!((-123i64).lt_abs(&Natural::from(124u32)));
            /// assert!((-123i64).le_abs(&Natural::from(124u32)));
            /// assert!(123i64.lt_abs(&Natural::trillion()));
            /// assert!(123i64.le_abs(&Natural::trillion()));
            /// ```
            #[inline]
            fn partial_cmp_abs(&self, other: &Natural) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_signeds!(impl_signed);
