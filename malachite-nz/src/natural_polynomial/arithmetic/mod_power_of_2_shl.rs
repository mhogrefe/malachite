// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Assign, ModPowerOf2IsReduced, ModPowerOf2Shl, ModPowerOf2ShlAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;

fn assert_reduced(p: &NaturalPolynomial, pow: u64) {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
}

// Shifts every coefficient left by `bits` modulo 2^pow: only the low `pow - bits` bits of each
// survive, so a coefficient can become zero, and the result is trimmed.
fn mod_power_of_2_shl_ref(p: &NaturalPolynomial, bits: u64, pow: u64) -> NaturalPolynomial {
    assert_reduced(p, pow);
    if bits >= pow {
        return NaturalPolynomial::ZERO;
    }
    let mut q = NaturalPolynomial {
        coefficients: p
            .coefficients
            .iter()
            .map(|c| c.mod_power_of_2(pow - bits) << bits)
            .collect(),
    };
    q.trim();
    q
}

fn mod_power_of_2_shl_assign(p: &mut NaturalPolynomial, bits: u64, pow: u64) {
    assert_reduced(p, pow);
    if bits >= pow {
        *p = NaturalPolynomial::ZERO;
    } else if bits != 0 {
        for c in &mut p.coefficients {
            c.mod_power_of_2_assign(pow - bits);
            *c <<= bits;
        }
        p.trim();
    }
}

macro_rules! impl_mod_power_of_2_shl_unsigned {
    ($t:ident) => {
        impl ModPowerOf2Shl<$t> for NaturalPolynomial {
            type Output = NaturalPolynomial;

            /// Left-shifts a [`NaturalPolynomial`] (multiplies it by a power of 2) modulo $2^k$,
            /// taking the polynomial by value. The coefficients must already be reduced modulo
            /// $2^k$.
            ///
            /// Every coefficient is shifted and reduced. Coefficients can become zero, so the
            /// degree can drop; if `bits` is at least `pow`, the result is zero.
            ///
            /// $$
            /// f(p, m, k) = 2^mp \bmod 2^k.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients
            /// times `pow`.
            ///
            /// # Panics
            /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl).
            #[inline]
            fn mod_power_of_2_shl(mut self, bits: $t, pow: u64) -> NaturalPolynomial {
                self.mod_power_of_2_shl_assign(bits, pow);
                self
            }
        }

        impl ModPowerOf2Shl<$t> for &NaturalPolynomial {
            type Output = NaturalPolynomial;

            /// Left-shifts a [`NaturalPolynomial`] (multiplies it by a power of 2) modulo $2^k$,
            /// taking the polynomial by reference. The coefficients must already be reduced modulo
            /// $2^k$.
            ///
            /// Every coefficient is shifted and reduced. Coefficients can become zero, so the
            /// degree can drop; if `bits` is at least `pow`, the result is zero.
            ///
            /// $$
            /// f(p, m, k) = 2^mp \bmod 2^k.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients
            /// times `pow`.
            ///
            /// # Panics
            /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl).
            #[inline]
            fn mod_power_of_2_shl(self, bits: $t, pow: u64) -> NaturalPolynomial {
                mod_power_of_2_shl_ref(self, u64::exact_from(bits), pow)
            }
        }

        impl ModPowerOf2ShlAssign<$t> for NaturalPolynomial {
            /// Left-shifts a [`NaturalPolynomial`] (multiplies it by a power of 2) modulo $2^k$, in
            /// place. The coefficients must already be reduced modulo $2^k$.
            ///
            /// Every coefficient is shifted and reduced. Coefficients can become zero, so the
            /// degree can drop; if `bits` is at least `pow`, the result is zero.
            ///
            /// $$
            /// p \gets 2^mp \bmod 2^k.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients
            /// times `pow`.
            ///
            /// # Panics
            /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl_assign).
            #[inline]
            fn mod_power_of_2_shl_assign(&mut self, bits: $t, pow: u64) {
                mod_power_of_2_shl_assign(self, u64::exact_from(bits), pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_shl_unsigned);
