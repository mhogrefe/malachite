// Copyright © 2026 Mikhail Hogrefe
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

use crate::num::arithmetic::traits::ModSum;
use crate::num::basic::unsigneds::PrimitiveUnsigned;

fn mod_sum<T: PrimitiveUnsigned, I: Iterator<Item = T>>(xs: I, m: T) -> T {
    assert_ne!(m, T::ZERO, "m cannot be zero");
    xs.fold(T::ZERO, |s, x| {
        assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
        s.mod_add(x, m)
    })
}

macro_rules! impl_mod_sum {
    ($t:ident) => {
        impl ModSum for $t {
            /// Adds up all the numbers in an iterator, modulo another number $m$. Every number must
            /// be already reduced modulo $m$.
            ///
            /// $f((x_i)_ {i=0}^{n-1}, m) = y$, where $y < m$ and $\sum_ {i=0}^{n-1} x_i \equiv y
            /// \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.count()`.
            ///
            /// # Panics
            /// Panics if `m` is zero or if any number is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_sum#mod_sum).
            #[inline]
            fn mod_sum<I: Iterator<Item = $t>>(xs: I, m: $t) -> $t {
                mod_sum(xs, m)
            }
        }

        impl<'a> ModSum<&'a $t, $t> for $t {
            /// Adds up all the numbers in an iterator of references, modulo another number $m$.
            /// Every number must be already reduced modulo $m$.
            ///
            /// $f((x_i)_ {i=0}^{n-1}, m) = y$, where $y < m$ and $\sum_ {i=0}^{n-1} x_i \equiv y
            /// \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.count()`.
            ///
            /// # Panics
            /// Panics if `m` is zero or if any number is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_sum#mod_sum).
            #[inline]
            fn mod_sum<I: Iterator<Item = &'a $t>>(xs: I, m: $t) -> $t {
                mod_sum(xs.copied(), m)
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_sum);
