use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::traits::Zero;
use std::cmp::Ordering;

fn partial_cmp_unsigned<T>(x: &Integer, other: &T) -> Option<Ordering>
where
    Natural: PartialOrd<T>,
{
    if x.sign {
        x.abs.partial_cmp(other)
    } else {
        Some(Ordering::Less)
    }
}

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialOrd<$t> for Integer {
            /// Compares an [`Integer`] to an unsigned primitive integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_int#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                partial_cmp_unsigned(self, other)
            }
        }

        impl PartialOrd<Integer> for $t {
            /// Compares an unsigned primitive integer to an [`Integer`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_int#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

fn partial_cmp_signed<U: PartialOrd<Natural>, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &Integer,
    other: &S,
) -> Option<Ordering>
where
    Natural: PartialOrd<U>,
{
    if x.sign {
        if *other >= S::ZERO {
            x.abs.partial_cmp(&other.unsigned_abs())
        } else {
            Some(Ordering::Greater)
        }
    } else if *other >= S::ZERO {
        Some(Ordering::Less)
    } else {
        other.unsigned_abs().partial_cmp(&x.abs)
    }
}

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialOrd<$t> for Integer {
            /// Compares an [`Integer`] to a signed primitive integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_int#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                partial_cmp_signed(self, other)
            }
        }

        impl PartialOrd<Integer> for $t {
            /// Compares a signed primitive integer to an [`Integer`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_int#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_signeds!(impl_signed);
