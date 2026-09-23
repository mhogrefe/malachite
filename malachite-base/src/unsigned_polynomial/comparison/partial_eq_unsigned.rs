// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::UnsignedPolynomial;

impl<T: PrimitiveUnsigned> PartialEq<T> for UnsignedPolynomial<T> {
    /// Determines whether an [`UnsignedPolynomial`] is equal to a value of its coefficient type.
    ///
    /// The polynomial is equal to the value when it is the constant polynomial with that value, so
    /// the zero polynomial is equal to 0 and nothing else, and no polynomial of positive degree is
    /// equal to any value. In particular, `p == 0` and `p == 1` test whether `p` is the zero
    /// polynomial or the polynomial 1, without building either.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::partial_eq_unsigned#partial_eq).
    fn eq(&self, other: &T) -> bool {
        match self.coefficients.as_slice() {
            [] => *other == T::ZERO,
            [c] => c == other,
            _ => false,
        }
    }
}

macro_rules! impl_partial_eq_unsigned {
    ($t: ident) => {
        impl PartialEq<UnsignedPolynomial<$t>> for $t {
            /// Determines whether a value is equal to an [`UnsignedPolynomial`] with coefficients
            /// of the value's type.
            ///
            /// The value is equal to the polynomial when the polynomial is the constant polynomial
            /// with that value, so 0 is equal to the zero polynomial.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_eq_unsigned#partial_eq).
            #[inline]
            fn eq(&self, other: &UnsignedPolynomial<$t>) -> bool {
                other == self
            }
        }
    };
}
apply_to_unsigneds!(impl_partial_eq_unsigned);
