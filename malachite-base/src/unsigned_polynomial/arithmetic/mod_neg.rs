// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModIsReduced, ModNeg, ModNegAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::UnsignedPolynomial;

fn assert_reduced<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, m: T) {
    assert!(
        p.mod_is_reduced(&m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

// Negates every coefficient modulo m. The coefficients are reduced, so a nonzero one stays nonzero
// and nothing needs trimming.
fn negate<T: PrimitiveUnsigned>(coefficients: &mut [T], m: T) {
    for c in coefficients {
        if *c != T::ZERO {
            *c = m - *c;
        }
    }
}

impl<T: PrimitiveUnsigned> ModNeg<T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Negates an [`UnsignedPolynomial`] modulo `m`, taking the polynomial by value. The
    /// coefficients must already be reduced modulo `m`.
    ///
    /// Each nonzero coefficient $c$ becomes $m - c$, which is also nonzero, so the degree is
    /// unchanged. The zero polynomial is its own negation.
    ///
    /// $$
    /// f(p, m) = -p \bmod m.
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
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModNeg;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+x+3").unwrap();
    /// assert_eq!(p.clone().mod_neg(7).to_string(), "2*x^2+6*x+4");
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::ZERO.mod_neg(7),
    ///     UnsignedPolynomial::<u8>::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_neg` from `nmod_poly/neg.c`, FLINT 3.6.0.
    #[inline]
    fn mod_neg(mut self, m: T) -> Self {
        self.mod_neg_assign(m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModNeg<T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Negates an [`UnsignedPolynomial`] modulo `m`, taking the polynomial by reference. The
    /// coefficients must already be reduced modulo `m`.
    ///
    /// Each nonzero coefficient $c$ becomes $m - c$, which is also nonzero, so the degree is
    /// unchanged. The zero polynomial is its own negation.
    ///
    /// $$
    /// f(p, m) = -p \bmod m.
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
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModNeg;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+x+3").unwrap();
    /// assert_eq!((&p).mod_neg(7).to_string(), "2*x^2+6*x+4");
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::ZERO).mod_neg(7),
    ///     UnsignedPolynomial::<u8>::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_neg` from `nmod_poly/neg.c`, FLINT 3.6.0.
    fn mod_neg(self, m: T) -> UnsignedPolynomial<T> {
        assert_reduced(self, m);
        let mut coefficients = self.coefficients.clone();
        negate(&mut coefficients, m);
        UnsignedPolynomial { coefficients }
    }
}

impl<T: PrimitiveUnsigned> ModNegAssign<T> for UnsignedPolynomial<T> {
    /// Negates an [`UnsignedPolynomial`] modulo `m`, in place. The coefficients must already be
    /// reduced modulo `m`.
    ///
    /// See [`mod_neg`](ModNeg::mod_neg).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModNegAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("5*x^2+x+3").unwrap();
    /// p.mod_neg_assign(7);
    /// assert_eq!(p.to_string(), "2*x^2+6*x+4");
    /// ```
    fn mod_neg_assign(&mut self, m: T) {
        assert_reduced(self, m);
        negate(&mut self.coefficients, m);
    }
}
