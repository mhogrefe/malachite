use malachite_base::num::conversion::traits::{CheckedFrom, IntegerMantissaAndExponent};
use malachite_base::num::logic::traits::SignificantBits;
use natural::Natural;
use std::cmp::Ordering;

macro_rules! impl_float {
    ($t: ident) => {
        impl PartialEq<$t> for Natural {
            /// Determines whether a `Natural` is equal to a value of primitive float type.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See the documentation of the `natural::comparison::partial_eq_primitive_float`
            /// module.
            fn eq(&self, other: &$t) -> bool {
                if !other.is_finite() {
                    false
                } else if *other == 0.0 {
                    *self == 0u32
                } else if *other < 1.0 || *self == 0u32 {
                    false
                } else {
                    let (m, e) = other.integer_mantissa_and_exponent();
                    if let Some(e) = u64::checked_from(e) {
                        self.significant_bits() == m.significant_bits() + e
                            && self.cmp_normalized(&Natural::from(m)) == Ordering::Equal
                    } else {
                        false
                    }
                }
            }
        }

        impl PartialEq<Natural> for $t {
            /// Determines whether a value of primitive float type is equal to a `Natural`.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// # Examples
            /// See the documentation of the `natural::comparison::partial_eq_primitive_float`
            /// module.
            #[inline]
            fn eq(&self, other: &Natural) -> bool {
                other == self
            }
        }
    };
}
apply_to_primitive_floats!(impl_float);
