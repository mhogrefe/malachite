// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{EqMod, Mod};
use crate::num::basic::traits::Zero;

fn eq_mod<U: Eq, S: Copy + Eq + Mod<S, Output = U> + Zero>(x: S, other: S, m: S) -> bool {
    x == other || m != S::ZERO && x.mod_op(m) == other.mod_op(m)
}

macro_rules! impl_eq_mod {
    ($t:ident) => {
        impl EqMod<$t> for $t {
            /// Returns whether a number is equivalent to another number modulo a third; that is,
            /// whether the difference between the first two is a multiple of the third.
            ///
            /// Two numbers are equal to each other modulo 0 iff they are equal.
            ///
            /// $f(x, y, m) = (x \equiv y \mod m)$.
            ///
            /// $f(x, y, m) = (\exists k \in \Z : x - y = km)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::eq_mod#eq_mod).
            #[inline]
            fn eq_mod(self, other: $t, m: $t) -> bool {
                eq_mod(self, other, m)
            }
        }
    };
}
apply_to_primitive_ints!(impl_eq_mod);
