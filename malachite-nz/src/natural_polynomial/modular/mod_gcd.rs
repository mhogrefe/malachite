// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::modular::mod_div_rem::{
    assert_reduced, from_words, mod_quotient_with, remainder, to_words, trimmed,
};
use crate::natural_polynomial::modular::mod_power_of_2_div_rem::{
    mod_power_of_2_quotient_with, remainder as mod_power_of_2_remainder,
};
use crate::natural_polynomial::modular::subproduct_tree::{NaturalModulus, NaturalPowerOf2Modulus};
use crate::natural_polynomial::series::mod_div_truncated::mod_div_truncated_dispatch;
use crate::natural_polynomial::series::mod_power_of_2_div_truncated::*;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::*;
use malachite_base::polynomial::modular::traits::{CheckedModGcd, ModGcd, ModGcdAssign};
use malachite_base::unsigned_polynomial::modular::mod_gcd::{
    GcdCutoffs, GcdModulus, checked_monic_gcd,
};
use malachite_base::unsigned_polynomial::modular::subproduct_tree::{Modulus, PowerOf2Modulus};

// The cutoffs for moduli of more than two limbs. The half-GCD's switch to its iterative step is
// `FMPZ_MOD_POLY_HGCD_CUTOFF` from `fmpz_mod_poly.h`, FLINT 3.6.0; the GCD and extended GCD
// crossovers, which FLINT sets to `FMPZ_MOD_POLY_GCD_CUTOFF`, 256, were measured on an Apple
// M-series machine, 2026-11, with prime moduli of 193 to 301 bits and random coefficients.
const NATURAL_GCD_CUTOFFS: GcdCutoffs = GcdCutoffs {
    hgcd_iter: 128,
    hgcd_outer: 170,
    gcd_hgcd: 170,
    xgcd_hgcd: 40,
};

impl GcdModulus<Natural> for NaturalModulus<'_> {
    #[inline]
    fn is_zero(&self, x: &Natural) -> bool {
        *x == 0u32
    }

    #[inline]
    fn inverse(&self, x: &Natural) -> Option<Natural> {
        if *x == 0u32 {
            None
        } else {
            x.mod_inverse(self.m)
        }
    }

    fn rem(&self, xs: &[Natural], ys: &[Natural], inverse: &Natural) -> Vec<Natural> {
        let qs = mod_quotient_with(xs, ys, self.m, inverse, mod_div_truncated_dispatch);
        remainder(xs, ys, &qs, self.m)
    }

    fn div_rem(
        &self,
        xs: &[Natural],
        ys: &[Natural],
        inverse: &Natural,
    ) -> (Vec<Natural>, Vec<Natural>) {
        let qs = mod_quotient_with(xs, ys, self.m, inverse, mod_div_truncated_dispatch);
        let rs = remainder(xs, ys, &qs, self.m);
        (qs, rs)
    }

    #[inline]
    fn gcd_cutoffs(&self) -> GcdCutoffs {
        NATURAL_GCD_CUTOFFS
    }
}

impl GcdModulus<Natural> for NaturalPowerOf2Modulus {
    #[inline]
    fn is_zero(&self, x: &Natural) -> bool {
        *x == 0u32
    }

    #[inline]
    fn inverse(&self, x: &Natural) -> Option<Natural> {
        if x.even() {
            None
        } else {
            x.mod_power_of_2_inverse(self.0)
        }
    }

    fn rem(&self, xs: &[Natural], ys: &[Natural], inverse: &Natural) -> Vec<Natural> {
        let qs = mod_power_of_2_quotient_with(
            xs,
            ys,
            self.0,
            inverse,
            mod_power_of_2_div_truncated_dispatch,
        );
        mod_power_of_2_remainder(xs, ys, &qs, self.0)
    }

    fn div_rem(
        &self,
        xs: &[Natural],
        ys: &[Natural],
        inverse: &Natural,
    ) -> (Vec<Natural>, Vec<Natural>) {
        let qs = mod_power_of_2_quotient_with(
            xs,
            ys,
            self.0,
            inverse,
            mod_power_of_2_div_truncated_dispatch,
        );
        let rs = mod_power_of_2_remainder(xs, ys, &qs, self.0);
        (qs, rs)
    }

    #[inline]
    fn gcd_cutoffs(&self) -> GcdCutoffs {
        NATURAL_GCD_CUTOFFS
    }
}

// The shape of a GCD-like operation's result, with each polynomial in it represented as a `P`.
pub(crate) trait GcdShape {
    type Of<P>;

    fn map<P, Q>(x: Self::Of<P>, f: impl FnMut(P) -> Q) -> Self::Of<Q>;

    // The result whose polynomials are all `x`.
    fn fill<P: Clone>(x: P) -> Self::Of<P>;
}

// A result that is one polynomial: a GCD or an LCM.
pub(crate) struct OnePolynomial;

impl GcdShape for OnePolynomial {
    type Of<P> = P;

    #[inline]
    fn map<P, Q>(x: P, mut f: impl FnMut(P) -> Q) -> Q {
        f(x)
    }

    #[inline]
    fn fill<P: Clone>(x: P) -> P {
        x
    }
}

// A result that is three polynomials: a GCD and its cofactors.
pub(crate) struct ThreePolynomials;

impl GcdShape for ThreePolynomials {
    type Of<P> = (P, P, P);

    #[inline]
    fn map<P, Q>((g, s, t): (P, P, P), mut f: impl FnMut(P) -> Q) -> (Q, Q, Q) {
        (f(g), f(s), f(t))
    }

    #[inline]
    fn fill<P: Clone>(x: P) -> (P, P, P) {
        (x.clone(), x.clone(), x)
    }
}

// The result of a GCD-like operation `O` on polynomials represented as `P`s.
pub(crate) type GcdResult<O, P> = <<O as GcdOperation>::Shape as GcdShape>::Of<P>;

// A GCD-like operation on two polynomials, generic over the coefficient arithmetic.
pub(crate) trait GcdOperation {
    type Shape: GcdShape;

    fn run<C: Clone, M: GcdModulus<C>>(
        xs: &[C],
        ys: &[C],
        m: &M,
    ) -> Option<GcdResult<Self, Vec<C>>>;
}

fn run_word<
    T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>,
    O: GcdOperation,
    M: GcdModulus<T>,
>(
    xs: &[Natural],
    ys: &[Natural],
    m: &M,
) -> Option<GcdResult<O, NaturalPolynomial>>
where
    Natural: From<T>,
{
    Some(O::Shape::map(
        O::run(&to_words::<T>(xs), &to_words::<T>(ys), m)?,
        from_words,
    ))
}

fn run_natural<O: GcdOperation, M: GcdModulus<Natural>>(
    xs: &[Natural],
    ys: &[Natural],
    m: &M,
) -> Option<GcdResult<O, NaturalPolynomial>> {
    Some(O::Shape::map(O::run(xs, ys, m)?, trimmed))
}

// The outputs of the operation `O` on two polynomials modulo `m`, or `None` if a leading
// coefficient met along the way is not invertible. When `m` fits in one or two limbs, this is done
// in word arithmetic.
pub(crate) fn checked_mod_gcd_operation<O: GcdOperation>(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> Option<GcdResult<O, NaturalPolynomial>> {
    assert_reduced(p, q, m);
    if *m == 1u32 {
        return Some(O::Shape::fill(NaturalPolynomial::ZERO));
    }
    let (xs, ys) = (&p.coefficients, &q.coefficients);
    let bits = m.significant_bits();
    if bits <= Limb::WIDTH {
        run_word::<Limb, O, _>(xs, ys, &Modulus(Limb::exact_from(m)))
    } else if bits <= DoubleLimb::WIDTH {
        run_word::<DoubleLimb, O, _>(xs, ys, &Modulus(DoubleLimb::exact_from(m)))
    } else {
        run_natural::<O, _>(xs, ys, &NaturalModulus::new(m))
    }
}

// The outputs of the operation `O` on two polynomials modulo $2^k$, where $k$ is `pow`, or `None`
// if a leading coefficient met along the way is even. When $2^k$ fits in one or two limbs, this is
// done in wrapping word arithmetic.
pub(crate) fn checked_mod_power_of_2_gcd_operation<O: GcdOperation>(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> Option<GcdResult<O, NaturalPolynomial>> {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    assert!(
        q.mod_power_of_2_is_reduced(pow),
        "other must be reduced mod 2^pow, but {q} has a coefficient >= 2^{pow}"
    );
    if pow == 0 {
        return Some(O::Shape::fill(NaturalPolynomial::ZERO));
    }
    let (xs, ys) = (&p.coefficients, &q.coefficients);
    if pow <= Limb::WIDTH {
        run_word::<Limb, O, _>(xs, ys, &PowerOf2Modulus(pow))
    } else if pow <= DoubleLimb::WIDTH {
        run_word::<DoubleLimb, O, _>(xs, ys, &PowerOf2Modulus(pow))
    } else {
        run_natural::<O, _>(xs, ys, &NaturalPowerOf2Modulus(pow))
    }
}

pub(crate) struct GcdOp;

impl GcdOperation for GcdOp {
    type Shape = OnePolynomial;

    #[inline]
    fn run<C: Clone, M: GcdModulus<C>>(xs: &[C], ys: &[C], m: &M) -> Option<Vec<C>> {
        checked_monic_gcd(xs, ys, m)
    }
}

fn checked_mod_gcd_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> Option<NaturalPolynomial> {
    checked_mod_gcd_operation::<GcdOp>(p, q, m)
}

fn mod_gcd_helper(p: &NaturalPolynomial, q: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
    checked_mod_gcd_helper(p, q, m)
        .expect("a leading coefficient met by Euclid's algorithm is not invertible modulo m")
}

impl ModGcd<Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, taking both polynomials by
    /// value and the modulus by value. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = g,
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and every
    /// common divisor of $a$ and $b$ divides it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then this function panics. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(p.mod_gcd(q, Natural::from(7u32)).to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_gcd` from `fmpz_mod_poly/gcd.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_gcd(self, other: Self, m: Natural) -> Self {
        mod_gcd_helper(&self, &other, &m)
    }
}

impl ModGcd<Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, taking both polynomials by
    /// value and the modulus by reference. The coefficients of both must already be reduced modulo
    /// $m$.
    ///
    /// $$
    /// f(a, b, m) = g,
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and every
    /// common divisor of $a$ and $b$ divides it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then this function panics. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(p.mod_gcd(q, &Natural::from(7u32)).to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_gcd` from `fmpz_mod_poly/gcd.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_gcd(self, other: Self, m: &Natural) -> Self {
        mod_gcd_helper(&self, &other, m)
    }
}

impl ModGcd<&Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, taking the first polynomial
    /// by value, the second by reference, and the modulus by value. The coefficients of both must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = g,
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and every
    /// common divisor of $a$ and $b$ divides it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then this function panics. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(p.mod_gcd(&q, Natural::from(7u32)).to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_gcd` from `fmpz_mod_poly/gcd.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_gcd(self, other: &Self, m: Natural) -> Self {
        mod_gcd_helper(&self, other, &m)
    }
}

impl ModGcd<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, taking the first polynomial
    /// by value, the second by reference, and the modulus by reference. The coefficients of both
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = g,
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and every
    /// common divisor of $a$ and $b$ divides it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then this function panics. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(p.mod_gcd(&q, &Natural::from(7u32)).to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_gcd` from `fmpz_mod_poly/gcd.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_gcd(self, other: &Self, m: &Natural) -> Self {
        mod_gcd_helper(&self, other, m)
    }
}

impl ModGcd<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, taking the first polynomial
    /// by reference, the second by value, and the modulus by value. The coefficients of both must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = g,
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and every
    /// common divisor of $a$ and $b$ divides it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then this function panics. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!((&p).mod_gcd(q, Natural::from(7u32)).to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_gcd` from `fmpz_mod_poly/gcd.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_gcd(self, other: NaturalPolynomial, m: Natural) -> NaturalPolynomial {
        mod_gcd_helper(self, &other, &m)
    }
}

impl ModGcd<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, taking the first polynomial
    /// by reference, the second by value, and the modulus by reference. The coefficients of both
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = g,
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and every
    /// common divisor of $a$ and $b$ divides it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then this function panics. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!((&p).mod_gcd(q, &Natural::from(7u32)).to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_gcd` from `fmpz_mod_poly/gcd.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_gcd(self, other: NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
        mod_gcd_helper(self, &other, m)
    }
}

impl ModGcd<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, taking both polynomials by
    /// reference and the modulus by value. The coefficients of both must already be reduced modulo
    /// $m$.
    ///
    /// $$
    /// f(a, b, m) = g,
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and every
    /// common divisor of $a$ and $b$ divides it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then this function panics. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!((&p).mod_gcd(&q, Natural::from(7u32)).to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_gcd` from `fmpz_mod_poly/gcd.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_gcd(self, other: &NaturalPolynomial, m: Natural) -> NaturalPolynomial {
        mod_gcd_helper(self, other, &m)
    }
}

impl ModGcd<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, taking both polynomials by
    /// reference and the modulus by reference. The coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = g,
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and every
    /// common divisor of $a$ and $b$ divides it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then this function panics. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!((&p).mod_gcd(&q, &Natural::from(7u32)).to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_gcd` from `fmpz_mod_poly/gcd.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_gcd(self, other: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
        mod_gcd_helper(self, other, m)
    }
}

impl ModGcdAssign<Self, Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its monic GCD with another modulo $m$, taking the
    /// other polynomial by value and the modulus by value. The coefficients of both must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// a \gets g,
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and every
    /// common divisor of $a$ and $b$ divides it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then this function panics. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModGcdAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// p.mod_gcd_assign(q, Natural::from(7u32));
    /// assert_eq!(p.to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_gcd` from `fmpz_mod_poly/gcd.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_gcd_assign(&mut self, other: Self, m: Natural) {
        *self = mod_gcd_helper(self, &other, &m);
    }
}

impl ModGcdAssign<Self, &Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its monic GCD with another modulo $m$, taking the
    /// other polynomial by value and the modulus by reference. The coefficients of both must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// a \gets g,
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and every
    /// common divisor of $a$ and $b$ divides it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then this function panics. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModGcdAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// p.mod_gcd_assign(q, &Natural::from(7u32));
    /// assert_eq!(p.to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_gcd` from `fmpz_mod_poly/gcd.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_gcd_assign(&mut self, other: Self, m: &Natural) {
        *self = mod_gcd_helper(self, &other, m);
    }
}

impl ModGcdAssign<&Self, Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its monic GCD with another modulo $m$, taking the
    /// other polynomial by reference and the modulus by value. The coefficients of both must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// a \gets g,
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and every
    /// common divisor of $a$ and $b$ divides it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then this function panics. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModGcdAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// p.mod_gcd_assign(&q, Natural::from(7u32));
    /// assert_eq!(p.to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_gcd` from `fmpz_mod_poly/gcd.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_gcd_assign(&mut self, other: &Self, m: Natural) {
        *self = mod_gcd_helper(self, other, &m);
    }
}

impl ModGcdAssign<&Self, &Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its monic GCD with another modulo $m$, taking the
    /// other polynomial by reference and the modulus by reference. The coefficients of both must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// a \gets g,
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and every
    /// common divisor of $a$ and $b$ divides it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then this function panics. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModGcdAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// p.mod_gcd_assign(&q, &Natural::from(7u32));
    /// assert_eq!(p.to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_gcd` from `fmpz_mod_poly/gcd.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_gcd_assign(&mut self, other: &Self, m: &Natural) {
        *self = mod_gcd_helper(self, other, m);
    }
}

impl CheckedModGcd<Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking both polynomials
    /// by value and the modulus by value. The coefficients of both must already be reduced modulo
    /// $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(g) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and every
    /// common divisor of $a$ and $b$ divides it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then the result is `None`. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_gcd(q, Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "x+1"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_gcd(q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_gcd` from `fmpz_mod_poly/gcd.c`, FLINT 3.6.0, except
    /// that Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_gcd(self, other: Self, m: Natural) -> Option<Self> {
        checked_mod_gcd_helper(&self, &other, &m)
    }
}

impl CheckedModGcd<Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking both polynomials
    /// by value and the modulus by reference. The coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(g) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and every
    /// common divisor of $a$ and $b$ divides it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then the result is `None`. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_gcd(q, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "x+1"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_gcd(q, &Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_gcd` from `fmpz_mod_poly/gcd.c`, FLINT 3.6.0, except
    /// that Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_gcd(self, other: Self, m: &Natural) -> Option<Self> {
        checked_mod_gcd_helper(&self, &other, m)
    }
}

impl CheckedModGcd<&Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking the first
    /// polynomial by value, the second by reference, and the modulus by value. The coefficients of
    /// both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(g) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and every
    /// common divisor of $a$ and $b$ divides it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then the result is `None`. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_gcd(&q, Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "x+1"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_gcd(&q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_gcd` from `fmpz_mod_poly/gcd.c`, FLINT 3.6.0, except
    /// that Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_gcd(self, other: &Self, m: Natural) -> Option<Self> {
        checked_mod_gcd_helper(&self, other, &m)
    }
}

impl CheckedModGcd<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking the first
    /// polynomial by value, the second by reference, and the modulus by reference. The coefficients
    /// of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(g) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and every
    /// common divisor of $a$ and $b$ divides it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then the result is `None`. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_gcd(&q, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "x+1"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_gcd(&q, &Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_gcd` from `fmpz_mod_poly/gcd.c`, FLINT 3.6.0, except
    /// that Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_gcd(self, other: &Self, m: &Natural) -> Option<Self> {
        checked_mod_gcd_helper(&self, other, m)
    }
}

impl CheckedModGcd<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking the first
    /// polynomial by reference, the second by value, and the modulus by value. The coefficients of
    /// both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(g) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and every
    /// common divisor of $a$ and $b$ divides it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then the result is `None`. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_gcd(q, Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "x+1"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_gcd(q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_gcd` from `fmpz_mod_poly/gcd.c`, FLINT 3.6.0, except
    /// that Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_gcd(self, other: NaturalPolynomial, m: Natural) -> Option<NaturalPolynomial> {
        checked_mod_gcd_helper(self, &other, &m)
    }
}

impl CheckedModGcd<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking the first
    /// polynomial by reference, the second by value, and the modulus by reference. The coefficients
    /// of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(g) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and every
    /// common divisor of $a$ and $b$ divides it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then the result is `None`. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_gcd(q, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "x+1"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_gcd(q, &Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_gcd` from `fmpz_mod_poly/gcd.c`, FLINT 3.6.0, except
    /// that Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_gcd(self, other: NaturalPolynomial, m: &Natural) -> Option<NaturalPolynomial> {
        checked_mod_gcd_helper(self, &other, m)
    }
}

impl CheckedModGcd<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking both polynomials
    /// by reference and the modulus by value. The coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(g) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and every
    /// common divisor of $a$ and $b$ divides it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then the result is `None`. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_gcd(&q, Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "x+1"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_gcd(&q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_gcd` from `fmpz_mod_poly/gcd.c`, FLINT 3.6.0, except
    /// that Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_gcd(self, other: &NaturalPolynomial, m: Natural) -> Option<NaturalPolynomial> {
        checked_mod_gcd_helper(self, other, &m)
    }
}

impl CheckedModGcd<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking both polynomials
    /// by reference and the modulus by reference. The coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(g) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and every
    /// common divisor of $a$ and $b$ divides it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then the result is `None`. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_gcd(&q, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "x+1"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_gcd(&q, &Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_gcd` from `fmpz_mod_poly/gcd.c`, FLINT 3.6.0, except
    /// that Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_gcd(self, other: &NaturalPolynomial, m: &Natural) -> Option<NaturalPolynomial> {
        checked_mod_gcd_helper(self, other, m)
    }
}
