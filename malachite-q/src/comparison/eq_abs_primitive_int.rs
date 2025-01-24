// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::comparison::traits::EqAbs;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl EqAbs<$t> for Rational {
            /// Determines whether the absolute values of a [`Rational`] and a primitive unsigned
            /// integer are equal.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::eq_abs_primitive_int#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &$t) -> bool {
                self.denominator == 1u32 && self.numerator == *other
            }
        }

        impl EqAbs<Rational> for $t {
            /// Determines whether the absolute values of a primitive unsigned integer and a
            /// [`Rational`] are equal.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::eq_abs_primitive_int#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &Rational) -> bool {
                other.denominator == 1u32 && other.numerator == *self
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

macro_rules! impl_signed {
    ($t: ident) => {
        impl EqAbs<$t> for Rational {
            /// Determines whether the absolute values of a [`Rational`] and a primitive signed
            /// integer are equal.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::eq_abs_primitive_int#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &$t) -> bool {
                self.denominator == 1 && self.numerator == other.unsigned_abs()
            }
        }

        impl EqAbs<Rational> for $t {
            /// Determines whether the absolute values of a primitive signed integer and an
            /// [`Rational`] are equal.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::eq_abs_primitive_int#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &Rational) -> bool {
                other.denominator == 1 && other.numerator == self.unsigned_abs()
            }
        }
    };
}
apply_to_signeds!(impl_signed);
