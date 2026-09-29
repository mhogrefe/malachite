// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::{
    ModIsReduced, ModMul, ModMulAssign, ModShl, ModShlAssign,
};
use malachite_base::num::basic::traits::One;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

fn assert_reduced(p: &NaturalPolynomial, m: &Natural) {
    assert!(
        p.mod_is_reduced(m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

// 2^bits mod m, for a nonzero polynomial reduced modulo m; its leading coefficient is at least 1
// and less than m, so m is at least 2 and 1 is reduced modulo m.
fn power_of_2_mod<T: PrimitiveUnsigned>(bits: T, m: &Natural) -> Natural
where
    for<'a> Natural: ModShl<T, &'a Natural, Output = Natural>,
{
    Natural::ONE.mod_shl(bits, m)
}

// Multiplies every coefficient by 2^bits mod m. Since m need not be odd, a product can be zero, so
// the result is trimmed.
fn mod_shl_ref<T: PrimitiveUnsigned>(
    p: &NaturalPolynomial,
    bits: T,
    m: &Natural,
) -> NaturalPolynomial
where
    for<'a> Natural: ModShl<T, &'a Natural, Output = Natural>,
{
    assert_reduced(p, m);
    if bits == T::ZERO || p.coefficients.is_empty() {
        return p.clone();
    }
    let factor = power_of_2_mod(bits, m);
    let mut q = NaturalPolynomial {
        coefficients: p
            .coefficients
            .iter()
            .map(|c| c.mod_mul(&factor, m))
            .collect(),
    };
    q.trim();
    q
}

fn mod_shl_assign<T: PrimitiveUnsigned>(p: &mut NaturalPolynomial, bits: T, m: &Natural)
where
    for<'a> Natural: ModShl<T, &'a Natural, Output = Natural>,
{
    assert_reduced(p, m);
    if bits == T::ZERO || p.coefficients.is_empty() {
        return;
    }
    let factor = power_of_2_mod(bits, m);
    for c in &mut p.coefficients {
        c.mod_mul_assign(&factor, m);
    }
    p.trim();
}

macro_rules! impl_mod_shl_unsigned {
    ($t:ident) => {
        impl ModShl<$t, Natural> for NaturalPolynomial {
            type Output = NaturalPolynomial;

            /// Left-shifts a [`NaturalPolynomial`] (multiplies it by a power of 2) modulo `m`,
            /// taking the polynomial by value and the modulus by value. The coefficients must
            /// already be reduced modulo `m`.
            ///
            /// $2^k \bmod m$ is computed once, and every coefficient is multiplied by it. Since `m`
            /// need not be odd, coefficients can become zero, so the degree can drop.
            ///
            /// $$
            /// f(p, k, m) = 2^kp \bmod m.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O((m + k)n \log n \log\log n)$
            ///
            /// $M(n, k) = O(n \log n + kn)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, $m$ is
            /// `bits`, and $k$ is `self.len()`.
            ///
            /// # Panics
            /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl).
            #[inline]
            fn mod_shl(mut self, bits: $t, m: Natural) -> NaturalPolynomial {
                mod_shl_assign(&mut self, bits, &m);
                self
            }
        }

        impl ModShl<$t, &Natural> for NaturalPolynomial {
            type Output = NaturalPolynomial;

            /// Left-shifts a [`NaturalPolynomial`] (multiplies it by a power of 2) modulo `m`,
            /// taking the polynomial by value and the modulus by reference. The coefficients must
            /// already be reduced modulo `m`.
            ///
            /// $2^k \bmod m$ is computed once, and every coefficient is multiplied by it. Since `m`
            /// need not be odd, coefficients can become zero, so the degree can drop.
            ///
            /// $$
            /// f(p, k, m) = 2^kp \bmod m.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O((m + k)n \log n \log\log n)$
            ///
            /// $M(n, k) = O(n \log n + kn)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, $m$ is
            /// `bits`, and $k$ is `self.len()`.
            ///
            /// # Panics
            /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl).
            #[inline]
            fn mod_shl(mut self, bits: $t, m: &Natural) -> NaturalPolynomial {
                mod_shl_assign(&mut self, bits, m);
                self
            }
        }

        impl ModShl<$t, Natural> for &NaturalPolynomial {
            type Output = NaturalPolynomial;

            /// Left-shifts a [`NaturalPolynomial`] (multiplies it by a power of 2) modulo `m`,
            /// taking the polynomial by reference and the modulus by value. The coefficients must
            /// already be reduced modulo `m`.
            ///
            /// $2^k \bmod m$ is computed once, and every coefficient is multiplied by it. Since `m`
            /// need not be odd, coefficients can become zero, so the degree can drop.
            ///
            /// $$
            /// f(p, k, m) = 2^kp \bmod m.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O((m + k)n \log n \log\log n)$
            ///
            /// $M(n, k) = O(n \log n + kn)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, $m$ is
            /// `bits`, and $k$ is `self.len()`.
            ///
            /// # Panics
            /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl).
            #[inline]
            fn mod_shl(self, bits: $t, m: Natural) -> NaturalPolynomial {
                mod_shl_ref(self, bits, &m)
            }
        }

        impl ModShl<$t, &Natural> for &NaturalPolynomial {
            type Output = NaturalPolynomial;

            /// Left-shifts a [`NaturalPolynomial`] (multiplies it by a power of 2) modulo `m`,
            /// taking the polynomial by reference and the modulus by reference. The coefficients
            /// must already be reduced modulo `m`.
            ///
            /// $2^k \bmod m$ is computed once, and every coefficient is multiplied by it. Since `m`
            /// need not be odd, coefficients can become zero, so the degree can drop.
            ///
            /// $$
            /// f(p, k, m) = 2^kp \bmod m.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O((m + k)n \log n \log\log n)$
            ///
            /// $M(n, k) = O(n \log n + kn)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, $m$ is
            /// `bits`, and $k$ is `self.len()`.
            ///
            /// # Panics
            /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl).
            #[inline]
            fn mod_shl(self, bits: $t, m: &Natural) -> NaturalPolynomial {
                mod_shl_ref(self, bits, m)
            }
        }

        impl ModShlAssign<$t, Natural> for NaturalPolynomial {
            /// Left-shifts a [`NaturalPolynomial`] (multiplies it by a power of 2) modulo `m`, in
            /// place, taking the modulus by value. The coefficients must already be reduced modulo
            /// `m`.
            ///
            /// $2^k \bmod m$ is computed once, and every coefficient is multiplied by it. Since `m`
            /// need not be odd, coefficients can become zero, so the degree can drop.
            ///
            /// $$
            /// p \gets 2^kp \bmod m.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O((m + k)n \log n \log\log n)$
            ///
            /// $M(n, k) = O(n \log n + kn)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, $m$ is
            /// `bits`, and $k$ is `self.len()`.
            ///
            /// # Panics
            /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl_assign).
            #[inline]
            fn mod_shl_assign(&mut self, bits: $t, m: Natural) {
                mod_shl_assign(self, bits, &m);
            }
        }

        impl ModShlAssign<$t, &Natural> for NaturalPolynomial {
            /// Left-shifts a [`NaturalPolynomial`] (multiplies it by a power of 2) modulo `m`, in
            /// place, taking the modulus by reference. The coefficients must already be reduced
            /// modulo `m`.
            ///
            /// $2^k \bmod m$ is computed once, and every coefficient is multiplied by it. Since `m`
            /// need not be odd, coefficients can become zero, so the degree can drop.
            ///
            /// $$
            /// p \gets 2^kp \bmod m.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O((m + k)n \log n \log\log n)$
            ///
            /// $M(n, k) = O(n \log n + kn)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, $m$ is
            /// `bits`, and $k$ is `self.len()`.
            ///
            /// # Panics
            /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl_assign).
            #[inline]
            fn mod_shl_assign(&mut self, bits: $t, m: &Natural) {
                mod_shl_assign(self, bits, m);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_shl_unsigned);
