// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModPowerOf2Product;
use crate::num::basic::unsigneds::PrimitiveUnsigned;

// Since $2^k$ divides $2^W$, where $W$ is the width of `T`, wrapping multiplication followed by
// reduction gives the product modulo $2^k$. The partial product is reduced after every step so that
// a zero residue is seen at once: from then on the product stays zero, so the factors after the one
// that made it zero are not examined. Starting from 1 reduced modulo $2^k$ makes the product of no
// numbers correct even when $k = 0$.
fn mod_power_of_2_product<T: PrimitiveUnsigned, I: Iterator<Item = T>>(xs: I, pow: u64) -> T {
    assert!(pow <= T::WIDTH);
    let mut product = T::ONE.mod_power_of_2(pow);
    for x in xs {
        assert!(
            x.significant_bits() <= pow,
            "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
        );
        product = product.wrapping_mul(x).mod_power_of_2(pow);
        if product == T::ZERO {
            break;
        }
    }
    product
}

macro_rules! impl_mod_power_of_2_product {
    ($t:ident) => {
        impl ModPowerOf2Product for $t {
            /// Multiplies together all the numbers in an iterator, modulo $2^k$. Every number must
            /// be already reduced modulo $2^k$.
            ///
            /// $f((x_i)_ {i=0}^{n-1}, k) = y$, where $y < 2^k$ and $\prod_ {i=0}^{n-1} x_i \equiv y
            /// \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.count()`.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH` or if any number is greater than or
            /// equal to $2^k$. As soon as the partial product is zero, the remaining numbers are
            /// not examined, so an unreduced number after that point is not detected.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_product#mod_power_of_2_product).
            #[inline]
            fn mod_power_of_2_product<I: Iterator<Item = $t>>(xs: I, pow: u64) -> $t {
                mod_power_of_2_product(xs, pow)
            }
        }

        impl<'a> ModPowerOf2Product<&'a $t> for $t {
            /// Multiplies together all the numbers in an iterator of references, modulo $2^k$.
            /// Every number must be already reduced modulo $2^k$.
            ///
            /// $f((x_i)_ {i=0}^{n-1}, k) = y$, where $y < 2^k$ and $\prod_ {i=0}^{n-1} x_i \equiv y
            /// \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.count()`.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH` or if any number is greater than or
            /// equal to $2^k$. As soon as the partial product is zero, the remaining numbers are
            /// not examined, so an unreduced number after that point is not detected.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_product#mod_power_of_2_product).
            #[inline]
            fn mod_power_of_2_product<I: Iterator<Item = &'a $t>>(xs: I, pow: u64) -> $t {
                mod_power_of_2_product(xs.copied(), pow)
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_product);
