// Copyright © 2024 Mikhail Hogrefe
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

use crate::num::arithmetic::traits::{ModNeg, ModNegAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;

fn mod_neg<T: PrimitiveUnsigned>(x: T, m: T) -> T {
    assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
    if x == T::ZERO {
        T::ZERO
    } else {
        m - x
    }
}

fn mod_neg_assign<T: PrimitiveUnsigned>(x: &mut T, m: T) {
    assert!(*x < m, "x must be reduced mod m, but {x} >= {m}");
    if *x != T::ZERO {
        *x = m - *x;
    }
}

macro_rules! impl_mod_neg {
    ($t:ident) => {
        impl ModNeg for $t {
            type Output = $t;

            /// Negates a number modulo another number $m$, in place. The input must be already
            /// reduced modulo $m$.
            ///
            /// $f(x, m) = y$, where $x, y < m$ and $-x \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mod_neg#mod_neg).
            ///
            /// This is equivalent to `nmod_neg` from `nmod.h`, FLINT 2.7.1.
            #[inline]
            fn mod_neg(self, m: $t) -> $t {
                mod_neg(self, m)
            }
        }

        impl ModNegAssign for $t {
            /// Negates a number modulo another number $m$. The input must be already reduced modulo
            /// $m$.
            ///
            /// $x \gets y$, where $x, y < m$ and $-x \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mod_neg#mod_neg_assign).
            ///
            /// This is equivalent to `nmod_neg` from `nmod.h`, FLINT 2.7.1, where the output is
            /// assigned to `a`.
            #[inline]
            fn mod_neg_assign(&mut self, m: $t) {
                mod_neg_assign(self, m)
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_neg);
