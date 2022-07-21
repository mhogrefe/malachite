use crate::Rational;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialEq<$t> for Rational {
            /// Determines whether a [`Rational`] is equal to an unsigned primitive integer.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                self.sign && self.denominator == 1 && self.numerator == *other
            }
        }

        impl PartialEq<Rational> for $t {
            /// Determines whether an unsigned primitive integer is equal to a [`Rational`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq(&self, other: &Rational) -> bool {
                other == self
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialEq<$t> for Rational {
            /// Determines whether a [`Rational`] is equal to a signed primitive integer.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            fn eq(&self, other: &$t) -> bool {
                self.sign == (*other >= 0)
                    && self.denominator == 1
                    && self.numerator == other.unsigned_abs()
            }
        }

        impl PartialEq<Rational> for $t {
            /// Determines whether a signed primitive integer is equal to a [`Rational`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq(&self, other: &Rational) -> bool {
                other == self
            }
        }
    };
}
apply_to_signeds!(impl_signed);
