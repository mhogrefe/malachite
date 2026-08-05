// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    ModMulPrecomputed, ModPow, ModPowAssign, ModSquare, ModSquareAssign, ModSquarePrecomputed,
    ModSquarePrecomputedAssign,
};

// `$mul_data` projects the modular-exponentiation data onto the modular-multiplication data it
// contains: the pow data is `(mul_inverse, shift)` for every type except `u128`, whose pow and mul
// data are both `()`.
macro_rules! impl_mod_square {
    ($t:ident, $mul_data:path) => {
        impl ModSquare for $t {
            type Output = $t;

            /// Squares a number modulo another number $m$. The input must be already reduced modulo
            /// $m$.
            ///
            /// $f(x, m) = y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_square#mod_square).
            #[inline]
            fn mod_square(self, m: $t) -> $t {
                self.mod_pow(2, m)
            }
        }

        impl ModSquareAssign for $t {
            /// Squares a number modulo another number $m$, in place. The input must be already
            /// reduced modulo $m$.
            ///
            /// $x \gets y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_square#mod_square_assign).
            #[inline]
            fn mod_square_assign(&mut self, m: $t) {
                self.mod_pow_assign(2, m);
            }
        }

        impl ModSquarePrecomputed<u64, $t> for $t {
            /// Squares a number modulo another number $m$. The input must be already reduced modulo
            /// $m$.
            ///
            /// Some precomputed data is provided; this speeds up computations involving several
            /// modular squarings with the same modulus. The precomputed data should be obtained
            /// using [`precompute_mod_pow_data`](super::traits::ModPowPrecomputed).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_square#mod_square_precomputed).
            #[inline]
            fn mod_square_precomputed(self, m: $t, data: &Self::Data) -> Self::Output {
                self.mod_mul_precomputed(self, m, $mul_data(data))
            }
        }

        impl ModSquarePrecomputedAssign<u64, $t> for $t {
            /// Squares a number modulo another number $m$, in place. The input must be already
            /// reduced modulo $m$.
            ///
            /// Some precomputed data is provided; this speeds up computations involving several
            /// modular squarings with the same modulus. The precomputed data should be obtained
            /// using [`precompute_mod_pow_data`](super::traits::ModPowPrecomputed).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_square#mod_square_precomputed_assign).
            #[inline]
            fn mod_square_precomputed_assign(&mut self, m: $t, data: &Self::Data) {
                *self = self.mod_mul_precomputed(*self, m, $mul_data(data));
            }
        }
    };
}
const fn pair_mul_data<A, B>(data: &(A, B)) -> &A {
    &data.0
}

const fn unit_mul_data(data: &()) -> &() {
    data
}

impl_mod_square!(u8, pair_mul_data);
impl_mod_square!(u16, pair_mul_data);
impl_mod_square!(u32, pair_mul_data);
impl_mod_square!(u64, pair_mul_data);
impl_mod_square!(u128, unit_mul_data);
impl_mod_square!(usize, pair_mul_data);
