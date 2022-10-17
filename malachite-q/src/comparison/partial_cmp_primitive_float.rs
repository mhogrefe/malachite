use crate::Rational;
use malachite_base::num::arithmetic::traits::FloorLogBase2;
use std::cmp::Ordering;

macro_rules! impl_float {
    ($t: ident) => {
        impl PartialOrd<$t> for Rational {
            /// Compares a [`Rational`] to a primitive float.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(self.significant_bits(), other.sci_exponent())`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_float#partial_cmp).
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                if other.is_nan() {
                    None
                } else if self.sign != (*other >= 0.0) {
                    Some(if self.sign {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    })
                } else if !other.is_finite() {
                    Some(if self.sign {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    })
                } else if *other == 0.0 {
                    self.partial_cmp(&0u32)
                } else if *self == 0u32 {
                    0.0.partial_cmp(other)
                } else {
                    let ord_cmp = self
                        .floor_log_base_2_of_abs()
                        .cmp(&other.abs().floor_log_base_2());
                    Some(if ord_cmp != Ordering::Equal {
                        if self.sign {
                            ord_cmp
                        } else {
                            ord_cmp.reverse()
                        }
                    } else {
                        self.cmp(&Rational::try_from(*other).unwrap())
                    })
                }
            }
        }

        impl PartialOrd<Rational> for $t {
            /// Compares a primitive float to a [`Rational`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(other.sci_exponent(), self.significant_bits())`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_float#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_primitive_floats!(impl_float);
