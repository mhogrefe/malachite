use num::arithmetic::traits::UnsignedAbs;
use num::comparison::traits::{OrdAbs, PartialOrdAbs};
use std::cmp::Ordering;

/// This macro defines trait implementations that are the same for unsigned and signed types.
macro_rules! impl_partial_ord_abs {
    ($t:ident) => {
        impl PartialOrdAbs<$t> for $t {
            /// Compares the absolute values of `self` and `other`, taking both by reference.
            ///
            /// The `PartialOrdAbs` interface allows for pairs of incomparable elements, but for
            /// primitive integers these never occur.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::comparison::ord_abs` module.
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                Some(self.cmp_abs(other))
            }
        }
    };
}
apply_to_primitive_ints!(impl_partial_ord_abs);

macro_rules! impl_ord_abs_unsigned {
    ($t:ident) => {
        impl OrdAbs for $t {
            /// Compares the absolute values of `self` and `other`, taking both by reference.
            ///
            /// For unsigned values, this is the same as ordinary comparison.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::comparison::ord_abs` module.
            #[inline]
            fn cmp_abs(&self, other: &Self) -> Ordering {
                self.cmp(other)
            }
        }
    };
}
apply_to_unsigneds!(impl_ord_abs_unsigned);

fn _cmp_abs_signed<U: Ord, S: Copy + UnsignedAbs<Output = U>>(x: &S, y: &S) -> Ordering {
    x.unsigned_abs().cmp(&y.unsigned_abs())
}

macro_rules! impl_ord_abs_signed {
    ($t:ident) => {
        impl OrdAbs for $t {
            /// Compares the absolute values of `self` and `other`, taking both by reference.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::comparison::ord_abs` module.
            #[inline]
            fn cmp_abs(&self, other: &Self) -> Ordering {
                _cmp_abs_signed(self, other)
            }
        }
    };
}
apply_to_signeds!(impl_ord_abs_signed);
