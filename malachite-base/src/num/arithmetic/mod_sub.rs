// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 William Hart
//
//      Copyright © 2021 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModSub, ModSubAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;

fn mod_sub<T: PrimitiveUnsigned>(x: T, y: T, m: T) -> T {
    assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
    assert!(y < m, "y must be reduced mod m, but {y} >= {m}");
    let diff = x.wrapping_sub(y);
    if x < y {
        m.wrapping_add(diff)
    } else {
        diff
    }
}

macro_rules! impl_mod_sub {
    ($t:ident) => {
        impl ModSub<$t> for $t {
            type Output = $t;

            /// Subtracts two numbers modulo a third number $m$. The inputs must be already reduced
            /// modulo $m$.
            ///
            /// $f(x, y, m) = z$, where $x, y, z < m$ and $x - y \equiv z \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` or `other` are greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_sub#mod_sub).
            ///
            /// This is equivalent to `nmod_sub` from `nmod.h`, FLINT 2.7.1.
            #[inline]
            fn mod_sub(self, other: $t, m: $t) -> $t {
                mod_sub(self, other, m)
            }
        }

        impl ModSubAssign<$t> for $t {
            /// Subtracts two numbers modulo a third number $m$, in place. The inputs must be
            /// already reduced modulo $m$.
            ///
            /// $x \gets z$, where $x, y, z < m$ and $x - y \equiv z \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` or `other` are greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_sub#mod_sub_assign).
            ///
            /// This is equivalent to `nmod_sub` from `nmod.h`, FLINT 2.7.1, where the result is
            /// assigned to `a`.
            #[inline]
            fn mod_sub_assign(&mut self, other: $t, m: $t) {
                *self = self.mod_sub(other, m);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_sub);
