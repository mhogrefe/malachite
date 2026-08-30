// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::AbsSquared;
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};

macro_rules! impl_float {
    ($t: ident) => {
        impl EqAbs<$t> for GaussianInteger {
            /// Determines whether the absolute values of a [`GaussianInteger`] and a primitive
            /// float are equal.
            ///
            /// No [`GaussianInteger`] is equal in absolute value to an infinity or NaN. If the
            /// float is not an integer, its square is not an integer either (its odd mantissa
            /// contributes an odd square), so it cannot equal the absolute value of any
            /// [`GaussianInteger`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of
            /// significant bits of the real and imaginary parts of `self`.
            ///
            /// # Examples
            /// See [here](super::eq_abs_primitive_float#eq_abs).
            fn eq_abs(&self, other: &$t) -> bool {
                if self.imaginary == 0u32 {
                    self.real.eq_abs(other)
                } else if self.real == 0u32 {
                    self.imaginary.eq_abs(other)
                } else if !self.real.lt_abs(other) || !self.imaginary.lt_abs(other) {
                    false
                } else if let Ok(y) = Integer::try_from(*other) {
                    self.abs_squared() == y.abs_squared()
                } else {
                    false
                }
            }
        }

        impl EqAbs<GaussianInteger> for $t {
            /// Determines whether the absolute values of a primitive float and a
            /// [`GaussianInteger`] are equal.
            ///
            /// No infinity or NaN is equal in absolute value to a [`GaussianInteger`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of
            /// significant bits of the real and imaginary parts of `other`.
            ///
            /// # Examples
            /// See [here](super::eq_abs_primitive_float#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &GaussianInteger) -> bool {
                other.eq_abs(self)
            }
        }
    };
}
apply_to_primitive_floats!(impl_float);
