use Rational;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialEq<$t> for Rational {
            /// Determines whether a `Rational` is equal to a value of unsigned primitive integer
            /// type.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `comparison::partial_eq_primitive_int` module.
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                self.sign && self.denominator == 1 && self.numerator == *other
            }
        }

        impl PartialEq<Rational> for $t {
            /// Determines whether a value of unsigned primitive integer type is equal to a
            /// `Rational`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `comparison::partial_eq_primitive_int` module.
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
            /// Determines whether a `Rational` is equal to a value of signed primitive integer
            /// type.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `comparison::partial_eq_primitive_int` module.
            fn eq(&self, other: &$t) -> bool {
                self.sign == (*other >= 0)
                    && self.denominator == 1
                    && self.numerator == other.unsigned_abs()
            }
        }

        impl PartialEq<Rational> for $t {
            /// Determines whether a value of signed primitive integer type is equal to a
            /// `Rational`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `comparison::partial_eq_primitive_int` module.
            #[inline]
            fn eq(&self, other: &Rational) -> bool {
                other == self
            }
        }
    };
}
apply_to_signeds!(impl_signed);
