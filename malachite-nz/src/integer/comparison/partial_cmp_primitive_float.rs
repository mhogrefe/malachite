use integer::Integer;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use std::cmp::Ordering;

macro_rules! impl_float {
    ($t: ident) => {
        impl PartialOrd<$t> for Integer {
            /// Compares an `Integer` to a value of primitive float type.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See the documentation of the `integer::comparison::partial_cmp_primitive_float`
            /// module.
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                if self.sign {
                    self.unsigned_abs().partial_cmp(other)
                } else {
                    self.unsigned_abs()
                        .partial_cmp(&-other)
                        .map(Ordering::reverse)
                }
            }
        }

        impl PartialOrd<Integer> for $t {
            /// Compares a value of primitive float type to an `Integer`.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// # Examples
            /// See the documentation of the `integer::comparison::partial_cmp_primitive_float`
            /// module.
            #[inline]
            fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_primitive_floats!(impl_float);
