// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{DivAssignEuclidean, DivEuclidean};

macro_rules! impl_div_euclidean {
    ($t:ident) => {
        impl DivEuclidean<$t> for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            /// Divides a number by another number, returning the quotient and remainder. The
            /// quotient is rounded so that the remainder is nonnegative.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < |y|$.
            ///
            /// $$
            /// f(x, y) = \left ( \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor,
            /// \space x - y \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor \right ).
            /// $$
            ///
            /// For unsigned integers, `div_euclidean` is equivalent to
            /// [`div_mod`](super::traits::DivMod::div_mod).
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
            fn div_euclidean(self, other: $t) -> ($t, $t) {
                (self.div_euclid(other), self.rem_euclid(other))
            }
        }

        impl DivAssignEuclidean<$t> for $t {
            type ModOutput = $t;

            /// Divides a number by another number in place, returning the remainder. The quotient
            /// is rounded so that the remainder is nonnegative.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < |y|$.
            ///
            /// $$
            /// f(x, y) = x - y \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor,
            /// $$
            /// $$
            /// x \gets \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor.
            /// $$
            ///
            /// For unsigned integers, `div_assign_euclidean` is equivalent to
            /// [`div_assign_mod`](super::traits::DivAssignMod::div_assign_mod).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1 (when `$t` is
            /// signed).
            ///
            /// # Examples
            /// See [here](super::div_euclidean#div_assign_euclidean).
            #[inline]
            fn div_assign_euclidean(&mut self, other: $t) -> $t {
                let q = self.div_euclid(other);
                let r = self.rem_euclid(other);
                *self = q;
                r
            }
        }
    };
}
apply_to_primitive_ints!(impl_div_euclidean);
