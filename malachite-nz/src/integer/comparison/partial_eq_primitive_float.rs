use integer::Integer;
use malachite_base::num::arithmetic::traits::UnsignedAbs;

macro_rules! impl_float {
    ($t: ident) => {
        impl PartialEq<$t> for Integer {
            /// Determines whether an `Integer` is equal to a value of primitive float type.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See the documentation of the `integer::comparison::partial_eq_primitive_float`
            /// module.
            fn eq(&self, other: &$t) -> bool {
                if self.sign {
                    self.unsigned_abs() == *other
                } else {
                    self.unsigned_abs() == -other
                }
            }
        }

        impl PartialEq<Integer> for $t {
            /// Determines whether a value of primitive float type is equal to an `Integer`.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// # Examples
            /// See the documentation of the `integer::comparison::partial_eq_primitive_float`
            /// module.
            #[inline]
            fn eq(&self, other: &Integer) -> bool {
                other == self
            }
        }
    };
}
apply_to_primitive_floats!(impl_float);
