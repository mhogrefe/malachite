// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use malachite_base::num::comparison::traits::EqAbs;

macro_rules! impl_primitive_float {
    ($t: ident) => {
        impl EqAbs<$t> for BigExponentFloat {
            /// Determines whether the absolute values of a [`BigExponentFloat`] and a primitive
            /// float are equal.
            ///
            /// This implementation follows the IEEE 754 standard: NaN is not equal to anything.
            /// Both primitive float zeros are equal to the [`BigExponentFloat`] zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
            ///
            /// # Examples
            /// See [here](super::eq_abs_primitive_float#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &$t) -> bool {
                self.eq_abs(&Self::from(*other))
            }
        }

        impl EqAbs<BigExponentFloat> for $t {
            /// Determines whether the absolute values of a primitive float and a
            /// [`BigExponentFloat`] are equal.
            ///
            /// This implementation follows the IEEE 754 standard: NaN is not equal to anything.
            /// Both primitive float zeros are equal to the [`BigExponentFloat`] zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.complexity()`.
            ///
            /// # Examples
            /// See [here](super::eq_abs_primitive_float#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &BigExponentFloat) -> bool {
                other.eq_abs(self)
            }
        }
    };
}
apply_to_primitive_floats!(impl_primitive_float);
