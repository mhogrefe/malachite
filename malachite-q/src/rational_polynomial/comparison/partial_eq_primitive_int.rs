// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;

macro_rules! impl_partial_eq_primitive_int {
    ($t: ident) => {
        impl PartialEq<$t> for RationalPolynomial {
            /// Determines whether a [`RationalPolynomial`] is equal to a value of a primitive
            /// integer type.
            ///
            /// The polynomial is equal to the value when it is the constant polynomial with that
            /// value, so the zero polynomial is equal to 0 and nothing else, no polynomial with a
            /// non-integer coefficient is equal to any value, and no polynomial of positive degree
            /// is equal to any value. In particular, `p == 0` and `p == 1` test whether `p` is the
            /// zero polynomial or the polynomial 1, without building either.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            fn eq(&self, other: &$t) -> bool {
                self.denominator == 1u32 && self.numerator == *other
            }
        }

        impl PartialEq<RationalPolynomial> for $t {
            /// Determines whether a value of a primitive integer type is equal to a
            /// [`RationalPolynomial`].
            ///
            /// The value is equal to the polynomial when the polynomial is the constant polynomial
            /// with that value, so 0 is equal to the zero polynomial.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq(&self, other: &RationalPolynomial) -> bool {
                other == self
            }
        }
    };
}
apply_to_primitive_ints!(impl_partial_eq_primitive_int);
