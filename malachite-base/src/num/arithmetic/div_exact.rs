// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{DivExact, DivExactAssign};

macro_rules! impl_div_exact {
    ($t:ident) => {
        impl DivExact<$t> for $t {
            type Output = $t;

            /// Divides a value by another value. The first value must be exactly divisible by the
            /// second.
            ///
            /// If `self` is not exactly divisible by `other`, this function may panic or return a
            /// meaningless result.
            ///
            /// $$
            /// f(x, y) = \frac{x}{y}.
            /// $$
            ///
            /// If you are unsure whether the division will be exact, use `self / other` instead. If
            /// you're unsure and you want to know, use `self.div_mod(other)` and check whether the
            /// remainder is zero. If you want a function that panics if the division is not exact,
            /// use `self.div_round(other, Exact)`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is zero or if `self` is `Self::MIN` and other is -1.
            ///
            /// # Examples
            /// See [here](super::div_exact#div_exact).
            #[inline]
            fn div_exact(self, other: $t) -> $t {
                self / other
            }
        }

        impl DivExactAssign<$t> for $t {
            /// Divides a value by another value in place. The value being assigned to must be
            /// exactly divisible by the value on the right-hand side.
            ///
            /// If `self` is not exactly divisible by `other`, this function may panic or return a
            /// meaningless result.
            ///
            /// $$
            /// x \gets \frac{x}{y}.
            /// $$
            ///
            /// If you are unsure whether the division will be exact, use `self /= other` instead.
            /// If you're unsure and you want to know, use `self.div_assign_mod(other)` and check
            /// whether the remainder is zero. If you want a function that panics if the division is
            /// not exact, use `self.div_round_assign(other, Exact)`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is zero or if `self` is `Self::MIN` and other is -1.
            ///
            /// # Examples
            /// See [here](super::div_exact#div_exact_assign).
            #[inline]
            fn div_exact_assign(&mut self, other: $t) {
                *self /= other;
            }
        }
    };
}
apply_to_primitive_ints!(impl_div_exact);
