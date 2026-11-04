// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2007, 2008 William Hart
//
//      Copyright © 2011 Sebastian Pancratz
//
//      Copyright © 2023 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::num::modular::traits::ModIsReduced;
use crate::polynomial::Polynomial;
use crate::polynomial::modular::traits::{CheckedModGcd, ModGcd, ModGcdAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::hgcd::gcd_hgcd;
use crate::unsigned_polynomial::modular::mod_div_rem::mod_div_rem_dispatch;
use crate::unsigned_polynomial::modular::mod_power_of_2_div_rem::mod_power_of_2_div_rem_dispatch;
use crate::unsigned_polynomial::modular::mod_power_of_2_mul::mask_coefficients;
use crate::unsigned_polynomial::modular::subproduct_tree::{Modulus, PowerOf2Modulus, TreeModulus};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;

/// The arithmetic on coefficients of type `C`, modulo $m$ or modulo $2^k$, that the GCD algorithms
/// need beyond that of [`TreeModulus`]. Every input is reduced, and so is every output.
///
/// This is not part of the public API; it is public so that `malachite-nz` can compute GCDs of
/// `NaturalPolynomial`s.
#[doc(hidden)]
pub trait GcdModulus<C: Clone>: TreeModulus<C> {
    fn is_zero(&self, x: &C) -> bool;

    /// The inverse of `x`, or `None` if it has none (as when it is 0).
    fn inverse(&self, x: &C) -> Option<C>;

    /// The remainder of `xs` by `ys`, where `xs` is at least as long as `ys`, `ys` is nonempty, and
    /// `inverse` is the inverse of the last coefficient of `ys`; it has `ys.len() - 1`
    /// coefficients, possibly with zeros at the end.
    fn rem(&self, xs: &[C], ys: &[C], inverse: &C) -> Vec<C>;

    /// The quotient and remainder of `xs` by `ys`, as for [`rem`](Self::rem); the quotient has
    /// `xs.len() - ys.len() + 1` coefficients.
    fn div_rem(&self, xs: &[C], ys: &[C], inverse: &C) -> (Vec<C>, Vec<C>);

    /// The lengths at which the GCD algorithms switch strategies.
    fn gcd_cutoffs(&self) -> GcdCutoffs;

    /// The product of the polynomials with coefficients `xs` and `ys`, both nonempty.
    fn mul_polynomials(&self, xs: &[C], ys: &[C]) -> Vec<C> {
        let mut out = vec![self.zero(); xs.len() + ys.len() - 1];
        self.mul_to_out(&mut out, xs, ys);
        out
    }
}

/// The lengths at which the GCD algorithms switch strategies.
///
/// This is not part of the public API.
#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub struct GcdCutoffs {
    /// The length below which the half-GCD switches to its iterative step.
    pub hgcd_iter: usize,
    /// The length below which the half-GCD GCD switches to Euclid's algorithm.
    pub hgcd_outer: usize,
    /// The length below which the GCD uses Euclid's algorithm instead of the half-GCD GCD.
    pub gcd_hgcd: usize,
    /// The length below which the extended GCD uses Euclid's algorithm instead of the half-GCD
    /// extended GCD, and below which the half-GCD extended GCD switches to Euclid's algorithm.
    pub xgcd_hgcd: usize,
}

// The lengths below which the half-GCD switches to its iterative step, indexed by the number of
// bits of the modulus, minus 1. These are `nmod_poly_hgcd_iter_recursive_cutoff_tab` from
// `nmod_poly/gcd.c`, FLINT 3.6.0; measured here, the time depends little on this cutoff between
// about 40 and 200.
const HGCD_ITER_RECURSIVE_CUTOFFS: [u16; 64] = [
    68, 88, 60, 76, 64, 64, 76, 68, 72, 68, 80, 88, 96, 108, 68, 140, 116, 108, 96, 92, 104, 132,
    140, 136, 88, 104, 120, 112, 136, 200, 188, 176, 172, 112, 144, 156, 132, 172, 156, 148, 180,
    200, 188, 164, 160, 168, 168, 168, 168, 244, 180, 180, 200, 204, 192, 228, 212, 208, 168, 196,
    188, 216, 208, 76,
];

// The lengths from which the GCD uses the half-GCD rather than Euclid's algorithm, and below which
// the half-GCD GCD switches to Euclid's algorithm, indexed by the number of bits of the modulus,
// minus 1. Measured on an Apple M-series machine, 2026-11, with prime moduli and random
// coefficients; the resultant's crossover was found to be the same.
const GCD_HGCD_CUTOFFS: [u16; 64] = [
    1866, 1866, 2160, 2269, 2269, 2501, 2758, 2758, 2501, 2758, 2382, 2501, 2501, 2501, 2501, 2501,
    2501, 2501, 2501, 2501, 2501, 2501, 2758, 2501, 3192, 3880, 4074, 4717, 6321, 6321, 6321, 1326,
    1392, 1326, 1326, 1326, 1326, 1392, 1392, 1326, 1462, 1535, 1535, 1326, 1326, 1392, 1392, 1392,
    1392, 1612, 1462, 1462, 1535, 1612, 1535, 1263, 1326, 1535, 1326, 1777, 3040, 3192, 3192, 2626,
];

// The lengths from which the extended GCD uses the half-GCD rather than Euclid's algorithm, and
// below which the half-GCD extended GCD switches to Euclid's algorithm, indexed by the number of
// bits of the modulus, minus 1. Measured as for `GCD_HGCD_CUTOFFS`.
const XGCD_HGCD_CUTOFFS: [u16; 64] = [
    229, 229, 179, 171, 162, 162, 134, 155, 134, 155, 140, 140, 100, 134, 155, 134, 100, 140, 134,
    140, 90, 90, 134, 100, 127, 140, 134, 140, 95, 525, 525, 115, 82, 78, 78, 82, 78, 82, 82, 78,
    82, 78, 78, 105, 110, 105, 100, 127, 105, 105, 100, 105, 105, 105, 115, 82, 110, 100, 100, 95,
    188, 292, 292, 100,
];

// The cutoffs for a modulus with `bits` bits, as in `nmod_poly_hgcd_iter_recursive_cutoff`,
// `nmod_poly_hgcd_outer_cutoff`, and `nmod_poly_gcd_hgcd_cutoff` from `nmod_poly/gcd.c`, and
// `nmod_poly_xgcd_hgcd_cutoff` from `nmod_poly/xgcd.c`, FLINT 3.6.0, but with the GCD and extended
// GCD cutoffs measured anew and the half-GCD GCD's switch to Euclid's algorithm at the GCD's
// crossover. Moduli of 65 to 128 bits, which FLINT does not support, have cutoffs measured as for
// `GCD_HGCD_CUTOFFS`: the double-word arithmetic makes the half-GCD pay off much sooner, except
// just below the top bit.
fn word_gcd_cutoffs(bits: u64) -> GcdCutoffs {
    if bits > 64 {
        let (gcd_hgcd, xgcd_hgcd) = match bits {
            65..=125 => (170, 50),
            126..=127 => (440, 100),
            _ => (280, 50),
        };
        return GcdCutoffs {
            hgcd_iter: usize::from(HGCD_ITER_RECURSIVE_CUTOFFS[63]),
            hgcd_outer: gcd_hgcd,
            gcd_hgcd,
            xgcd_hgcd,
        };
    }
    let i = usize::exact_from(bits - 1);
    let gcd_hgcd = usize::from(GCD_HGCD_CUTOFFS[i]);
    GcdCutoffs {
        hgcd_iter: usize::from(HGCD_ITER_RECURSIVE_CUTOFFS[i]),
        hgcd_outer: gcd_hgcd,
        gcd_hgcd,
        xgcd_hgcd: usize::from(XGCD_HGCD_CUTOFFS[i]),
    }
}

impl<T: PrimitiveUnsigned> GcdModulus<T> for Modulus<T> {
    #[inline]
    fn is_zero(&self, x: &T) -> bool {
        *x == T::ZERO
    }

    #[inline]
    fn inverse(&self, x: &T) -> Option<T> {
        if *x == T::ZERO {
            None
        } else {
            x.mod_inverse(self.0)
        }
    }

    #[inline]
    fn rem(&self, xs: &[T], ys: &[T], inverse: &T) -> Vec<T> {
        mod_div_rem_dispatch(xs, ys, self.0, *inverse).1
    }

    #[inline]
    fn div_rem(&self, xs: &[T], ys: &[T], inverse: &T) -> (Vec<T>, Vec<T>) {
        mod_div_rem_dispatch(xs, ys, self.0, *inverse)
    }

    #[inline]
    fn gcd_cutoffs(&self) -> GcdCutoffs {
        word_gcd_cutoffs(self.0.significant_bits())
    }
}

impl<T: PrimitiveUnsigned> GcdModulus<T> for PowerOf2Modulus {
    #[inline]
    fn is_zero(&self, x: &T) -> bool {
        *x == T::ZERO
    }

    #[inline]
    fn inverse(&self, x: &T) -> Option<T> {
        if x.even() {
            None
        } else {
            x.mod_power_of_2_inverse(self.0)
        }
    }

    fn rem(&self, xs: &[T], ys: &[T], inverse: &T) -> Vec<T> {
        let mut rs = mod_power_of_2_div_rem_dispatch(xs, ys, self.0, *inverse).1;
        mask_coefficients(&mut rs, self.0);
        rs
    }
    fn div_rem(&self, xs: &[T], ys: &[T], inverse: &T) -> (Vec<T>, Vec<T>) {
        let (mut qs, mut rs) = mod_power_of_2_div_rem_dispatch(xs, ys, self.0, *inverse);
        mask_coefficients(&mut qs, self.0);
        mask_coefficients(&mut rs, self.0);
        (qs, rs)
    }

    #[inline]
    fn gcd_cutoffs(&self) -> GcdCutoffs {
        // A modulus $2^k$ has $k + 1$ bits, but its arithmetic is that of a `T`.
        word_gcd_cutoffs(min(self.0 + 1, T::WIDTH))
    }
}

// Removes the zeros at the end of `xs`.
pub(crate) fn normalize<C: Clone, M: GcdModulus<C>>(xs: &mut Vec<C>, m: &M) {
    while xs.last().is_some_and(|x| m.is_zero(x)) {
        xs.pop();
    }
}

// The remainder of `xs` by `ys`, without zeros at the end, or `None` if the last coefficient of
// `ys` has no inverse.
fn checked_rem<C: Clone, M: GcdModulus<C>>(xs: &[C], ys: &[C], m: &M) -> Option<Vec<C>> {
    let inverse = m.inverse(ys.last().unwrap())?;
    let mut rs = m.rem(xs, ys, &inverse);
    normalize(&mut rs, m);
    Some(rs)
}

/// A GCD, not normalized, of the polynomials with coefficients `xs` and `ys`, where `xs` is at
/// least as long as `ys` and neither is empty or ends with a zero, by Euclid's algorithm; or `None`
/// if a leading coefficient met along the way has no inverse. Every remainder's leading coefficient
/// is checked for an inverse before it is divided by, so the result's leading coefficient has one.
///
/// This is not part of the public API; it is public so that `malachite-nz` can compute GCDs of
/// `NaturalPolynomial`s.
///
/// This is equivalent to `_gr_poly_gcd_euclidean` from `gr_poly/gcd_euclidean.c`, FLINT 3.6.0,
/// except that a constant `ys` must have an inverse.
#[doc(hidden)]
pub fn gcd_euclidean<C: Clone, M: GcdModulus<C>>(xs: &[C], ys: &[C], m: &M) -> Option<Vec<C>> {
    if ys.len() == 1 {
        m.inverse(&ys[0])?;
        return Some(vec![m.one()]);
    }
    let mut r3 = checked_rem(xs, ys, m)?;
    if r3.is_empty() {
        return Some(ys.to_vec());
    }
    let mut r2 = ys.to_vec();
    loop {
        r2 = checked_rem(&r2, &r3, m)?;
        core::mem::swap(&mut r2, &mut r3);
        if r3.is_empty() {
            return Some(r2);
        }
    }
}

/// The monic GCD of the polynomials with coefficients `xs` and `ys`, neither ending with a zero, by
/// Euclid's algorithm or the half-GCD algorithm, or `None` if a leading coefficient met along the
/// way has no inverse. The GCD of two empty polynomials is empty.
///
/// This is not part of the public API; it is public so that `malachite-nz` can compute GCDs of
/// `NaturalPolynomial`s.
///
/// This is equivalent to `nmod_poly_gcd` from `nmod_poly/gcd.c`, FLINT 3.6.0.
#[doc(hidden)]
pub fn checked_monic_gcd<C: Clone, M: GcdModulus<C>>(xs: &[C], ys: &[C], m: &M) -> Option<Vec<C>> {
    let (xs, ys) = if xs.len() < ys.len() {
        (ys, xs)
    } else {
        (xs, ys)
    };
    let mut g = if xs.is_empty() {
        return Some(Vec::new());
    } else if ys.is_empty() {
        xs.to_vec()
    } else {
        let cutoffs = m.gcd_cutoffs();
        if ys.len() < cutoffs.gcd_hgcd {
            gcd_euclidean(xs, ys, m)?
        } else {
            gcd_hgcd(xs, ys, cutoffs.hgcd_iter, cutoffs.hgcd_outer, m)?
        }
    };
    let inverse = m.inverse(g.last().unwrap())?;
    for c in &mut g {
        *c = m.mul(c, &inverse);
    }
    Some(g)
}

pub(crate) fn assert_reduced<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) {
    assert!(
        p.mod_is_reduced(&m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    assert!(
        q.mod_is_reduced(&m),
        "other must be reduced mod m, but {q} has a coefficient >= {m}"
    );
}

pub(crate) fn checked_mod_gcd_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    assert_reduced(p, q, m);
    if m == T::ONE {
        return Some(UnsignedPolynomial::ZERO);
    }
    checked_monic_gcd(&p.coefficients, &q.coefficients, &Modulus(m))
        .map(UnsignedPolynomial::from_coefficients_asc)
}

fn mod_gcd_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    checked_mod_gcd_helper(p, q, m)
        .expect("a leading coefficient met by Euclid's algorithm is not invertible modulo m")
}

impl<T: PrimitiveUnsigned> ModGcd<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $m$, taking both by value. The
    /// coefficients of both must already be reduced modulo $m$.
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
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $m$; when $m$ is composite, a
    /// leading coefficient met along the way may not be, and then this function panics. When $m$ is
    /// prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModGcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(p.mod_gcd(q, 7).to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_gcd` from `nmod_poly/gcd.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_gcd(self, other: Self, m: T) -> Self {
        mod_gcd_helper(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModGcd<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $m$, taking the first by value
    /// and the second by reference. The coefficients of both must already be reduced modulo $m$.
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
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $m$; when $m$ is composite, a
    /// leading coefficient met along the way may not be, and then this function panics. When $m$ is
    /// prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModGcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(p.mod_gcd(&q, 7).to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_gcd` from `nmod_poly/gcd.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_gcd(self, other: &Self, m: T) -> Self {
        mod_gcd_helper(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModGcd<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $m$, taking the first by
    /// reference and the second by value. The coefficients of both must already be reduced modulo
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
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $m$; when $m$ is composite, a
    /// leading coefficient met along the way may not be, and then this function panics. When $m$ is
    /// prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModGcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!((&p).mod_gcd(q, 7).to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_gcd` from `nmod_poly/gcd.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_gcd(self, other: UnsignedPolynomial<T>, m: T) -> UnsignedPolynomial<T> {
        mod_gcd_helper(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModGcd<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $m$, taking both by reference.
    /// The coefficients of both must already be reduced modulo $m$.
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
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $m$; when $m$ is composite, a
    /// leading coefficient met along the way may not be, and then this function panics. When $m$ is
    /// prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModGcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!((&p).mod_gcd(&q, 7).to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_gcd` from `nmod_poly/gcd.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_gcd(self, other: &UnsignedPolynomial<T>, m: T) -> UnsignedPolynomial<T> {
        mod_gcd_helper(self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModGcdAssign<Self, T> for UnsignedPolynomial<T> {
    /// Replaces an [`UnsignedPolynomial`] with its monic GCD with another modulo $m$, taking the
    /// other polynomial by value. The coefficients of both must already be reduced modulo $m$.
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
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $m$; when $m$ is composite, a
    /// leading coefficient met along the way may not be, and then this function panics. When $m$ is
    /// prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModGcdAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+4*x+3").unwrap();
    /// p.mod_gcd_assign(q, 7);
    /// assert_eq!(p.to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_gcd` from `nmod_poly/gcd.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_gcd_assign(&mut self, other: Self, m: T) {
        *self = mod_gcd_helper(self, &other, m);
    }
}

impl<T: PrimitiveUnsigned> ModGcdAssign<&Self, T> for UnsignedPolynomial<T> {
    /// Replaces an [`UnsignedPolynomial`] with its monic GCD with another modulo $m$, taking the
    /// other polynomial by reference. The coefficients of both must already be reduced modulo $m$.
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
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $m$; when $m$ is composite, a
    /// leading coefficient met along the way may not be, and then this function panics. When $m$ is
    /// prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModGcdAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+4*x+3").unwrap();
    /// p.mod_gcd_assign(&q, 7);
    /// assert_eq!(p.to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_gcd` from `nmod_poly/gcd.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_gcd_assign(&mut self, other: &Self, m: T) {
        *self = mod_gcd_helper(self, other, m);
    }
}

impl<T: PrimitiveUnsigned> CheckedModGcd<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking both by value.
    /// The coefficients of both must already be reduced modulo $m$.
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
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $m$; when $m$ is composite, a
    /// leading coefficient met along the way may not be, and then the result is `None`. When $m$ is
    /// prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModGcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(p.checked_mod_gcd(q, 7).unwrap().to_string(), "x+1");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_gcd(q, 6).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_gcd` from `nmod_poly/gcd.c`, FLINT 3.6.0, except that
    /// Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_gcd(self, other: Self, m: T) -> Option<Self> {
        checked_mod_gcd_helper(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModGcd<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking the first by
    /// value and the second by reference. The coefficients of both must already be reduced modulo
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
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $m$; when $m$ is composite, a
    /// leading coefficient met along the way may not be, and then the result is `None`. When $m$ is
    /// prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModGcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(p.checked_mod_gcd(&q, 7).unwrap().to_string(), "x+1");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_gcd(&q, 6).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_gcd` from `nmod_poly/gcd.c`, FLINT 3.6.0, except that
    /// Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_gcd(self, other: &Self, m: T) -> Option<Self> {
        checked_mod_gcd_helper(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModGcd<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking the first by
    /// reference and the second by value. The coefficients of both must already be reduced modulo
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
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $m$; when $m$ is composite, a
    /// leading coefficient met along the way may not be, and then the result is `None`. When $m$ is
    /// prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModGcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!((&p).checked_mod_gcd(q, 7).unwrap().to_string(), "x+1");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_gcd(q, 6).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_gcd` from `nmod_poly/gcd.c`, FLINT 3.6.0, except that
    /// Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_gcd(self, other: UnsignedPolynomial<T>, m: T) -> Option<UnsignedPolynomial<T>> {
        checked_mod_gcd_helper(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModGcd<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking both by
    /// reference. The coefficients of both must already be reduced modulo $m$.
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
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $m$; when $m$ is composite, a
    /// leading coefficient met along the way may not be, and then the result is `None`. When $m$ is
    /// prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModGcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!((&p).checked_mod_gcd(&q, 7).unwrap().to_string(), "x+1");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_gcd(&q, 6).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_gcd` from `nmod_poly/gcd.c`, FLINT 3.6.0, except that
    /// Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_gcd(self, other: &UnsignedPolynomial<T>, m: T) -> Option<UnsignedPolynomial<T>> {
        checked_mod_gcd_helper(self, other, m)
    }
}
