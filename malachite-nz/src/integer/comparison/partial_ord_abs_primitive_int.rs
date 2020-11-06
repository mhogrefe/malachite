use std::cmp::Ordering;

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::comparison::traits::PartialOrdAbs;

use integer::Integer;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for Integer {
            /// Compares the absolute value of an `Integer` to a value of unsigned primitive integer
            /// type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            ///
            /// use malachite_base::num::comparison::traits::PartialOrdAbs;
            /// use malachite_nz::integer::Integer;
            ///
            /// assert!(Integer::from(-122).lt_abs(&123u64));
            /// assert!(Integer::from(-122).le_abs(&123u64));
            /// assert!(Integer::from(-123).lt_abs(&124u64));
            /// assert!(Integer::from(-123).le_abs(&124u64));
            /// assert!(Integer::trillion().gt_abs(&123u64));
            /// assert!(Integer::trillion().ge_abs(&123u64));
            /// assert!((-Integer::trillion()).gt_abs(&123u64));
            /// assert!((-Integer::trillion()).ge_abs(&123u64));
            /// ```
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                self.abs.partial_cmp(other)
            }
        }

        impl PartialOrdAbs<Integer> for $t {
            /// Compares a value of unsigned primitive integer type to the absolute value of an
            /// `Integer`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            ///
            /// use malachite_base::num::comparison::traits::PartialOrdAbs;
            /// use malachite_nz::integer::Integer;
            ///
            /// assert!(123u64.gt_abs(&Integer::from(-122)));
            /// assert!(123u64.ge_abs(&Integer::from(-122)));
            /// assert!(124u64.gt_abs(&Integer::from(-123)));
            /// assert!(124u64.ge_abs(&Integer::from(-123)));
            /// assert!(123u64.lt_abs(&Integer::trillion()));
            /// assert!(123u64.le_abs(&Integer::trillion()));
            /// assert!(123u64.lt_abs(&-Integer::trillion()));
            /// assert!(123u64.le_abs(&-Integer::trillion()));
            /// ```
            #[inline]
            fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for Integer {
            /// Compares the absolute value of an `Integer` to the absolute value of a value of
            /// signed primitive integer type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            ///
            /// use malachite_base::num::comparison::traits::PartialOrdAbs;
            /// use malachite_nz::integer::Integer;
            ///
            /// assert!(Integer::from(-122).lt_abs(&-123i64));
            /// assert!(Integer::from(-122).le_abs(&-123i64));
            /// assert!(Integer::from(-124).gt_abs(&-123i64));
            /// assert!(Integer::from(-124).ge_abs(&-123i64));
            /// assert!(Integer::trillion().gt_abs(&123i64));
            /// assert!(Integer::trillion().ge_abs(&123i64));
            /// assert!((-Integer::trillion()).gt_abs(&123i64));
            /// assert!((-Integer::trillion()).ge_abs(&123i64));
            /// ```
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                self.abs.partial_cmp(&other.unsigned_abs())
            }
        }

        impl PartialOrdAbs<Integer> for $t {
            /// Compares the absolute value of a value of signed primitive integer type to the
            /// absolute value of an `Integer`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            ///
            /// use malachite_base::num::comparison::traits::PartialOrdAbs;
            /// use malachite_nz::integer::Integer;
            ///
            /// assert!((-123i64).gt_abs(&Integer::from(-122)));
            /// assert!((-123i64).ge_abs(&Integer::from(-122)));
            /// assert!((-123i64).lt_abs(&Integer::from(-124)));
            /// assert!((-123i64).le_abs(&Integer::from(-124)));
            /// assert!(123i64.lt_abs(&Integer::trillion()));
            /// assert!(123i64.le_abs(&Integer::trillion()));
            /// assert!(123i64.lt_abs(&-Integer::trillion()));
            /// assert!(123i64.le_abs(&-Integer::trillion()));
            /// ```
            #[inline]
            fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_signeds!(impl_signed);
