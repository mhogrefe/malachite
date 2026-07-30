// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{DivEuclidean, DivEuclideanAssign};

macro_rules! impl_div_euclidean {
    ($t:ident) => {
        impl DivEuclidean<$t> for $t {
            type Output = $t;

            /// Divides a number by another number, returning just the quotient. The quotient is
            /// rounded so that the remainder would be nonnegative.
            ///
            /// If the remainder were computed, the quotient and remainder would satisfy $x = qy +
            /// r$ and $0 \leq r < |y|$.
            ///
            /// $$
            /// f(x, y) = \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor.
            /// $$
            ///
            /// For unsigned integers, `div_euclidean` is equivalent to division.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1 (when `$t` is
            /// signed).
            ///
            /// # Examples
            /// See [here](super::div_euclidean#div_euclidean).
            #[inline]
            fn div_euclidean(self, other: $t) -> $t {
                self.div_euclid(other)
            }
        }

        impl DivEuclideanAssign<$t> for $t {
            /// Divides a number by another number in place, keeping just the quotient. The quotient
            /// is rounded so that the remainder would be nonnegative.
            ///
            /// If the remainder were computed, the quotient and remainder would satisfy $x = qy +
            /// r$ and $0 \leq r < |y|$.
            ///
            /// $$
            /// x \gets \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor.
            /// $$
            ///
            /// For unsigned integers, `div_euclidean_assign` is equivalent to `/=`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1 (when `$t` is
            /// signed).
            ///
            /// # Examples
            /// See [here](super::div_euclidean#div_euclidean_assign).
            #[inline]
            fn div_euclidean_assign(&mut self, other: $t) {
                *self = self.div_euclid(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_div_euclidean);
