use integer::Integer;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use std::cmp::Ordering;

macro_rules! impl_float {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for Integer {
            /// Compares the absolute values of an [`Integer`] and a primitive float.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_abs_primitive_float#partial_cmp_abs).
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                self.unsigned_abs().partial_cmp(&other.abs())
            }
        }

        impl PartialOrdAbs<Integer> for $t {
            /// Compares the absolute values of a primitive float and an [`Integer`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// See [here](super::partial_cmp_abs_primitive_float#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_primitive_floats!(impl_float);
