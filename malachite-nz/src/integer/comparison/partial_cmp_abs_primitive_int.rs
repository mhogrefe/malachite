use integer::Integer;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use std::cmp::Ordering;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for Integer {
            /// Compares the absolute value of an `Integer` to a value of unsigned primitive integer
            /// type.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `integer::comparison::partial_cmp_abs_primitive_int`
            /// module.
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                self.abs.partial_cmp(other)
            }
        }

        impl PartialOrdAbs<Integer> for $t {
            /// Compares a value of unsigned primitive integer type to the absolute value of an
            /// `Integer`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `integer::comparison::partial_cmp_abs_primitive_int`
            /// module.
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
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `integer::comparison::partial_cmp_abs_primitive_int`
            /// module.
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                self.abs.partial_cmp(&other.unsigned_abs())
            }
        }

        impl PartialOrdAbs<Integer> for $t {
            /// Compares the absolute value of a value of signed primitive integer type to the
            /// absolute value of an `Integer`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `integer::comparison::partial_cmp_abs_primitive_int`
            /// module.
            #[inline]
            fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_signeds!(impl_signed);
