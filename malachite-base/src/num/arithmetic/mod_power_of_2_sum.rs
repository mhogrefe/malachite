// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModPowerOf2Sum;
use crate::num::basic::unsigneds::PrimitiveUnsigned;

// Since $2^k$ divides $2^W$, where $W$ is the width of `T`, wrapping addition followed by a single
// reduction gives the sum modulo $2^k$.
fn mod_power_of_2_sum<T: PrimitiveUnsigned, I: Iterator<Item = T>>(xs: I, pow: u64) -> T {
    assert!(pow <= T::WIDTH);
    xs.fold(T::ZERO, |s, x| {
        assert!(
            x.significant_bits() <= pow,
            "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
        );
        s.wrapping_add(x)
    })
    .mod_power_of_2(pow)
}

macro_rules! impl_mod_power_of_2_sum {
    ($t:ident) => {
        impl ModPowerOf2Sum for $t {
            /// Adds up all the numbers in an iterator, modulo $2^k$. Every number must be already
            /// reduced modulo $2^k$.
            ///
            /// $f((x_i)_ {i=0}^{n-1}, k) = y$, where $y < 2^k$ and $\sum_ {i=0}^{n-1} x_i \equiv y
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
            /// equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_sum#mod_power_of_2_sum).
            #[inline]
            fn mod_power_of_2_sum<I: Iterator<Item = $t>>(xs: I, pow: u64) -> $t {
                mod_power_of_2_sum(xs, pow)
            }
        }

        impl<'a> ModPowerOf2Sum<&'a $t> for $t {
            /// Adds up all the numbers in an iterator of references, modulo $2^k$. Every number
            /// must be already reduced modulo $2^k$.
            ///
            /// $f((x_i)_ {i=0}^{n-1}, k) = y$, where $y < 2^k$ and $\sum_ {i=0}^{n-1} x_i \equiv y
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
            /// equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_sum#mod_power_of_2_sum).
            #[inline]
            fn mod_power_of_2_sum<I: Iterator<Item = &'a $t>>(xs: I, pow: u64) -> $t {
                mod_power_of_2_sum(xs.copied(), pow)
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_sum);
