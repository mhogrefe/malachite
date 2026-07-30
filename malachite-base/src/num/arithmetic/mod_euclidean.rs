// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModEuclidean, ModEuclideanAssign};

macro_rules! impl_mod_euclidean {
    ($t:ident) => {
        impl ModEuclidean<$t> for $t {
            type Output = $t;

            /// Divides a number by another number, returning just the remainder. The remainder is
            /// always nonnegative.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$
            /// and $0 \leq r < |y|$.
            ///
            /// $$
            /// f(x, y) = x - y \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor.
            /// $$
            ///
            /// For unsigned integers, `mod_euclidean` is equivalent to
            /// [`mod_op`](super::traits::Mod::mod_op).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1 (when `$t` is
            /// signed).
            ///
            /// # Examples
            /// See [here](super::mod_euclidean#mod_euclidean).
            #[inline]
            fn mod_euclidean(self, other: $t) -> $t {
                self.rem_euclid(other)
            }
        }

        impl ModEuclideanAssign<$t> for $t {
            /// Divides a number by another number, replacing the first number by the remainder. The
            /// remainder is always nonnegative.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$
            /// and $0 \leq r < |y|$.
            ///
            /// $$
            /// x \gets x - y \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor.
            /// $$
            ///
            /// For unsigned integers, `mod_euclidean_assign` is equivalent to
            /// [`mod_assign`](super::traits::ModAssign::mod_assign).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1 (when `$t` is
            /// signed).
            ///
            /// # Examples
            /// See [here](super::mod_euclidean#mod_euclidean_assign).
            #[inline]
            fn mod_euclidean_assign(&mut self, other: $t) {
                *self = self.rem_euclid(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_mod_euclidean);
