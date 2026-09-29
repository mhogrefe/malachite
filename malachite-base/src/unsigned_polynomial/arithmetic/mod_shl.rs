// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModIsReduced, ModShl, ModShlAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::UnsignedPolynomial;

fn assert_reduced<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, m: T) {
    assert!(
        p.mod_is_reduced(&m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

// Multiplies every coefficient by 2^bits mod m, which is computed once. A nonzero polynomial
// reduced modulo m has a leading coefficient that is at least 1 and less than m, so m is at least 2
// and 1 is reduced modulo m. Since m need not be odd, a product can be zero, so the result is
// trimmed.
fn mod_shl_assign<T: PrimitiveUnsigned + ModShl<U, T, Output = T>, U: PrimitiveUnsigned>(
    p: &mut UnsignedPolynomial<T>,
    bits: U,
    m: T,
) {
    assert_reduced(p, m);
    if bits == U::ZERO || p.coefficients.is_empty() {
        return;
    }
    let factor = T::ONE.mod_shl(bits, m);
    for c in &mut p.coefficients {
        *c = c.mod_mul(factor, m);
    }
    p.trim();
}

fn mod_shl_ref<T: PrimitiveUnsigned + ModShl<U, T, Output = T>, U: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    bits: U,
    m: T,
) -> UnsignedPolynomial<T> {
    assert_reduced(p, m);
    if bits == U::ZERO || p.coefficients.is_empty() {
        return p.clone();
    }
    let factor = T::ONE.mod_shl(bits, m);
    let mut q = UnsignedPolynomial {
        coefficients: p
            .coefficients
            .iter()
            .map(|&c| c.mod_mul(factor, m))
            .collect(),
    };
    q.trim();
    q
}

macro_rules! impl_mod_shl_unsigned {
    ($t:ident) => {
        impl<T: PrimitiveUnsigned + ModShl<$t, T, Output = T>> ModShl<$t, T>
            for UnsignedPolynomial<T>
        {
            type Output = UnsignedPolynomial<T>;

            /// Left-shifts an [`UnsignedPolynomial`] (multiplies it by a power of 2) modulo `m`,
            /// taking the polynomial by value. The coefficients must already be reduced modulo `m`.
            ///
            /// $2^k \bmod m$ is computed once, and every coefficient is multiplied by it. Since `m`
            /// need not be odd, coefficients can become zero, so the degree can drop.
            ///
            /// $$
            /// f(p, k, m) = 2^kp \bmod m.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
            /// `bits.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl).
            #[inline]
            fn mod_shl(mut self, bits: $t, m: T) -> UnsignedPolynomial<T> {
                mod_shl_assign(&mut self, bits, m);
                self
            }
        }

        impl<T: PrimitiveUnsigned + ModShl<$t, T, Output = T>> ModShl<$t, T>
            for &UnsignedPolynomial<T>
        {
            type Output = UnsignedPolynomial<T>;

            /// Left-shifts an [`UnsignedPolynomial`] (multiplies it by a power of 2) modulo `m`,
            /// taking the polynomial by reference. The coefficients must already be reduced modulo
            /// `m`.
            ///
            /// $2^k \bmod m$ is computed once, and every coefficient is multiplied by it. Since `m`
            /// need not be odd, coefficients can become zero, so the degree can drop.
            ///
            /// $$
            /// f(p, k, m) = 2^kp \bmod m.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
            /// `bits.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl).
            #[inline]
            fn mod_shl(self, bits: $t, m: T) -> UnsignedPolynomial<T> {
                mod_shl_ref(self, bits, m)
            }
        }

        impl<T: PrimitiveUnsigned + ModShl<$t, T, Output = T>> ModShlAssign<$t, T>
            for UnsignedPolynomial<T>
        {
            /// Left-shifts an [`UnsignedPolynomial`] (multiplies it by a power of 2) modulo `m`, in
            /// place. The coefficients must already be reduced modulo `m`.
            ///
            /// $2^k \bmod m$ is computed once, and every coefficient is multiplied by it. Since `m`
            /// need not be odd, coefficients can become zero, so the degree can drop.
            ///
            /// $$
            /// p \gets 2^kp \bmod m.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
            /// `bits.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl_assign).
            #[inline]
            fn mod_shl_assign(&mut self, bits: $t, m: T) {
                mod_shl_assign(self, bits, m);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_shl_unsigned);
