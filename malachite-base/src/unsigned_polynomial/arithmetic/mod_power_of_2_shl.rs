// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2IsReduced, ModPowerOf2Shl, ModPowerOf2ShlAssign};
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::UnsignedPolynomial;

fn assert_reduced<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, pow: u64) {
    assert!(pow <= T::WIDTH);
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
}

// The shift amount as a `u64`, or `None` if it is at least `pow`, in which case every coefficient
// becomes zero. `pow` is at most `T::WIDTH`, which is at most 128, so it fits in every unsigned
// type.
fn shift_below_pow<U: PrimitiveUnsigned>(bits: U, pow: u64) -> Option<u64> {
    if bits >= U::exact_from(pow) {
        None
    } else {
        Some(bits.exact_into())
    }
}

// Shifts every coefficient left by `bits` modulo 2^pow: only the low `pow - bits` bits of each
// survive, so the shift cannot overflow, a coefficient can become zero, and the result is trimmed.
fn mod_power_of_2_shl_ref<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    bits: U,
    pow: u64,
) -> UnsignedPolynomial<T> {
    assert_reduced(p, pow);
    let Some(bits) = shift_below_pow(bits, pow) else {
        return UnsignedPolynomial::ZERO;
    };
    let mut q = UnsignedPolynomial {
        coefficients: p
            .coefficients
            .iter()
            .map(|&c| c.mod_power_of_2(pow - bits) << bits)
            .collect(),
    };
    q.trim();
    q
}

fn mod_power_of_2_shl_assign<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    p: &mut UnsignedPolynomial<T>,
    bits: U,
    pow: u64,
) {
    assert_reduced(p, pow);
    let Some(bits) = shift_below_pow(bits, pow) else {
        p.coefficients.clear();
        return;
    };
    if bits != 0 {
        for c in &mut p.coefficients {
            *c = c.mod_power_of_2(pow - bits) << bits;
        }
        p.trim();
    }
}

macro_rules! impl_mod_power_of_2_shl_unsigned {
    ($t:ident) => {
        impl<T: PrimitiveUnsigned> ModPowerOf2Shl<$t> for UnsignedPolynomial<T> {
            type Output = UnsignedPolynomial<T>;

            /// Left-shifts an [`UnsignedPolynomial`] (multiplies it by a power of 2) modulo $2^k$,
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
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is
            /// greater than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl).
            #[inline]
            fn mod_power_of_2_shl(mut self, bits: $t, pow: u64) -> UnsignedPolynomial<T> {
                mod_power_of_2_shl_assign(&mut self, bits, pow);
                self
            }
        }

        impl<T: PrimitiveUnsigned> ModPowerOf2Shl<$t> for &UnsignedPolynomial<T> {
            type Output = UnsignedPolynomial<T>;

            /// Left-shifts an [`UnsignedPolynomial`] (multiplies it by a power of 2) modulo $2^k$,
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
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is
            /// greater than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl).
            #[inline]
            fn mod_power_of_2_shl(self, bits: $t, pow: u64) -> UnsignedPolynomial<T> {
                mod_power_of_2_shl_ref(self, bits, pow)
            }
        }

        impl<T: PrimitiveUnsigned> ModPowerOf2ShlAssign<$t> for UnsignedPolynomial<T> {
            /// Left-shifts an [`UnsignedPolynomial`] (multiplies it by a power of 2) modulo $2^k$,
            /// in place. The coefficients must already be reduced modulo $2^k$.
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
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is
            /// greater than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl_assign).
            #[inline]
            fn mod_power_of_2_shl_assign(&mut self, bits: $t, pow: u64) {
                mod_power_of_2_shl_assign(self, bits, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_shl_unsigned);
