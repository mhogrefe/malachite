use malachite_base::num::comparison::traits::PartialOrdAbs;
use natural::Natural;
use std::cmp::Ordering;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for Natural {
            /// Compares a `Natural` to a value of unsigned primitive integer type. Since both
            /// values are non-negative, this is the same as ordinary `partial_cmp`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `natural::comparison::partial_cmp_abs_primitive_int`
            /// module.
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                self.partial_cmp(other)
            }
        }

        impl PartialOrdAbs<Natural> for $t {
            /// Compares a value of unsigned primitive integer type to a `Natural`. Since both
            /// values are non-negative, this is the same as ordinary `partial_cmp`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `natural::comparison::partial_cmp_abs_primitive_int`
            /// module.
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
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `natural::comparison::partial_cmp_abs_primitive_int`
            /// module.
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                self.partial_cmp(&other.unsigned_abs())
            }
        }

        impl PartialOrdAbs<Natural> for $t {
            /// Compares the absolute value of a value of signed primitive integer type to a
            /// `Natural`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `natural::comparison::partial_cmp_abs_primitive_int`
            /// module.
            #[inline]
            fn partial_cmp_abs(&self, other: &Natural) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_signeds!(impl_signed);
