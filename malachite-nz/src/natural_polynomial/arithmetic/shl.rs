// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use core::ops::{Shl, ShlAssign};

// Shifts every coefficient left by `bits`. A nonzero coefficient stays nonzero, so the result is
// normalized.
//
// This is equivalent to `fmpz_poly_scalar_mul_2exp` from `fmpz_poly/scalar.c`, FLINT 3.6.0.
fn shl_ref<T: Copy>(p: &NaturalPolynomial, bits: T) -> NaturalPolynomial
where
    for<'a> &'a Natural: Shl<T, Output = Natural>,
{
    NaturalPolynomial {
        coefficients: p.coefficients.iter().map(|c| c << bits).collect(),
    }
}

// Shifts every coefficient left by `bits`, in place.
//
// This is equivalent to `fmpz_poly_scalar_mul_2exp` from `fmpz_poly/scalar.c`, FLINT 3.6.0.
fn shl_assign<T: Copy>(p: &mut NaturalPolynomial, bits: T)
where
    Natural: ShlAssign<T>,
{
    for c in &mut p.coefficients {
        *c <<= bits;
    }
}

macro_rules! impl_natural_polynomial_shl_unsigned {
    ($t:ident) => {
        impl Shl<$t> for NaturalPolynomial {
            type Output = NaturalPolynomial;

            /// Left-shifts a [`NaturalPolynomial`] (multiplies it by a power of 2), taking it by
            /// value. Every coefficient is shifted.
            ///
            /// $f(p, k) = 2^kp$.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// coefficients, $m$ is `bits`, and $k$ is `self.len()`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(mut self, bits: $t) -> NaturalPolynomial {
                self <<= bits;
                self
            }
        }

        impl Shl<$t> for &NaturalPolynomial {
            type Output = NaturalPolynomial;

            /// Left-shifts a [`NaturalPolynomial`] (multiplies it by a power of 2), taking it by
            /// reference. Every coefficient is shifted.
            ///
            /// $f(p, k) = 2^kp$.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// coefficients, $m$ is `bits`, and $k$ is `self.len()`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(self, bits: $t) -> NaturalPolynomial {
                shl_ref(self, bits)
            }
        }

        impl ShlAssign<$t> for NaturalPolynomial {
            /// Left-shifts a [`NaturalPolynomial`] (multiplies it by a power of 2), in place. Every
            /// coefficient is shifted.
            ///
            /// $p \gets 2^kp$.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// coefficients, $m$ is `bits`, and $k$ is `self.len()`.
            ///
            /// # Examples
            /// See [here](super::shl#shl_assign).
            #[inline]
            fn shl_assign(&mut self, bits: $t) {
                shl_assign(self, bits);
            }
        }
    };
}
apply_to_unsigneds!(impl_natural_polynomial_shl_unsigned);
