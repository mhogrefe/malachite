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

use crate::num::arithmetic::traits::ModProduct;
use crate::num::basic::unsigneds::PrimitiveUnsigned;

// The data for multiplying modulo `m` is computed once and reused for every factor. Once the
// partial product is zero it stays zero, so the factors after the one that made it zero are not
// examined. Starting from 1 reduced modulo `m` makes the product of no numbers correct even when
// `m` is 1.
fn mod_product<T: PrimitiveUnsigned, I: Iterator<Item = T>>(xs: I, m: T) -> T {
    assert_ne!(m, T::ZERO, "m cannot be zero");
    let data = T::precompute_mod_mul_data(&m);
    let mut product = if m == T::ONE { T::ZERO } else { T::ONE };
    for x in xs {
        assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
        product.mod_mul_precomputed_assign(x, m, &data);
        if product == T::ZERO {
            break;
        }
    }
    product
}

macro_rules! impl_mod_product {
    ($t:ident) => {
        impl ModProduct for $t {
            /// Multiplies together all the numbers in an iterator, modulo another number $m$. Every
            /// number must be already reduced modulo $m$.
            ///
            /// The product of no numbers is 1 reduced modulo $m$, which is 0 when $m = 1$.
            ///
            /// $f((x_i)_ {i=0}^{n-1}, m) = y$, where $y < m$ and $\prod_ {i=0}^{n-1} x_i \equiv y
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
            /// Panics if `m` is zero or if any number is greater than or equal to `m`. As soon as
            /// the partial product is zero, the remaining numbers are not examined, so an unreduced
            /// number after that point is not detected.
            ///
            /// # Examples
            /// See [here](super::mod_product#mod_product).
            #[inline]
            fn mod_product<I: Iterator<Item = $t>>(xs: I, m: $t) -> $t {
                mod_product(xs, m)
            }
        }

        impl<'a> ModProduct<&'a $t, $t> for $t {
            /// Multiplies together all the numbers in an iterator of references, modulo another
            /// number $m$. Every number must be already reduced modulo $m$.
            ///
            /// The product of no numbers is 1 reduced modulo $m$, which is 0 when $m = 1$.
            ///
            /// $f((x_i)_ {i=0}^{n-1}, m) = y$, where $y < m$ and $\prod_ {i=0}^{n-1} x_i \equiv y
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
            /// Panics if `m` is zero or if any number is greater than or equal to `m`. As soon as
            /// the partial product is zero, the remaining numbers are not examined, so an unreduced
            /// number after that point is not detected.
            ///
            /// # Examples
            /// See [here](super::mod_product#mod_product).
            #[inline]
            fn mod_product<I: Iterator<Item = &'a $t>>(xs: I, m: $t) -> $t {
                mod_product(xs.copied(), m)
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_product);
