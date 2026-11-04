// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::test_util::unsigned_polynomial::modular::mod_div_rem::mod_div_rem_naive;
use crate::test_util::unsigned_polynomial::modular::mod_mul::mod_mul_polynomial_naive;
use crate::test_util::unsigned_polynomial::modular::mod_power_of_2_div_rem::*;
use crate::test_util::unsigned_polynomial::modular::mod_power_of_2_mul::*;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::hgcd::gcd_hgcd;
use crate::unsigned_polynomial::modular::mod_gcd::{GcdModulus, gcd_euclidean};
use crate::unsigned_polynomial::modular::subproduct_tree::Modulus;

// The polynomial arithmetic, modulo $m$ or modulo $2^k$, that the naive GCD algorithms use, built
// only from schoolbook division and multiplication and scalar modular arithmetic.
pub trait NaiveGcdArithmetic<T: PrimitiveUnsigned> {
    // Whether every polynomial is 0: the modulus is 1.
    fn trivial(&self) -> bool;

    fn inverse(&self, x: T) -> Option<T>;

    fn div_rem(
        &self,
        p: &UnsignedPolynomial<T>,
        q: &UnsignedPolynomial<T>,
    ) -> Option<(UnsignedPolynomial<T>, UnsignedPolynomial<T>)>;

    fn mul(&self, p: &UnsignedPolynomial<T>, q: &UnsignedPolynomial<T>) -> UnsignedPolynomial<T>;

    fn sub(&self, p: &UnsignedPolynomial<T>, q: &UnsignedPolynomial<T>) -> UnsignedPolynomial<T>;

    fn scale(&self, p: &UnsignedPolynomial<T>, c: T) -> UnsignedPolynomial<T>;
}

pub struct NaiveModulus<T>(pub T);

impl<T: PrimitiveUnsigned> NaiveGcdArithmetic<T> for NaiveModulus<T> {
    fn trivial(&self) -> bool {
        self.0 == T::ONE
    }

    fn inverse(&self, x: T) -> Option<T> {
        if x == T::ZERO {
            None
        } else {
            x.mod_inverse(self.0)
        }
    }

    fn div_rem(
        &self,
        p: &UnsignedPolynomial<T>,
        q: &UnsignedPolynomial<T>,
    ) -> Option<(UnsignedPolynomial<T>, UnsignedPolynomial<T>)> {
        mod_div_rem_naive(p, q, self.0)
    }

    fn mul(&self, p: &UnsignedPolynomial<T>, q: &UnsignedPolynomial<T>) -> UnsignedPolynomial<T> {
        mod_mul_polynomial_naive(p, q, self.0)
    }

    fn sub(&self, p: &UnsignedPolynomial<T>, q: &UnsignedPolynomial<T>) -> UnsignedPolynomial<T> {
        let (xs, ys) = (p.coefficients_asc(), q.coefficients_asc());
        UnsignedPolynomial::from_coefficients_asc(
            (0..xs.len().max(ys.len()))
                .map(|i| {
                    xs.get(i)
                        .copied()
                        .unwrap_or(T::ZERO)
                        .mod_sub(ys.get(i).copied().unwrap_or(T::ZERO), self.0)
                })
                .collect(),
        )
    }

    fn scale(&self, p: &UnsignedPolynomial<T>, c: T) -> UnsignedPolynomial<T> {
        UnsignedPolynomial::from_coefficients_asc(
            p.coefficients_asc()
                .iter()
                .map(|&x| x.mod_mul(c, self.0))
                .collect(),
        )
    }
}

pub struct NaivePowerOf2Modulus(pub u64);

impl<T: PrimitiveUnsigned> NaiveGcdArithmetic<T> for NaivePowerOf2Modulus {
    fn trivial(&self) -> bool {
        self.0 == 0
    }

    fn inverse(&self, x: T) -> Option<T> {
        x.mod_power_of_2_inverse(self.0)
    }

    fn div_rem(
        &self,
        p: &UnsignedPolynomial<T>,
        q: &UnsignedPolynomial<T>,
    ) -> Option<(UnsignedPolynomial<T>, UnsignedPolynomial<T>)> {
        mod_power_of_2_div_rem_naive(p, q, self.0)
    }

    fn mul(&self, p: &UnsignedPolynomial<T>, q: &UnsignedPolynomial<T>) -> UnsignedPolynomial<T> {
        mod_power_of_2_mul_polynomial_naive(p, q, self.0)
    }

    fn sub(&self, p: &UnsignedPolynomial<T>, q: &UnsignedPolynomial<T>) -> UnsignedPolynomial<T> {
        let (xs, ys) = (p.coefficients_asc(), q.coefficients_asc());
        UnsignedPolynomial::from_coefficients_asc(
            (0..xs.len().max(ys.len()))
                .map(|i| {
                    xs.get(i)
                        .copied()
                        .unwrap_or(T::ZERO)
                        .mod_power_of_2_sub(ys.get(i).copied().unwrap_or(T::ZERO), self.0)
                })
                .collect(),
        )
    }

    fn scale(&self, p: &UnsignedPolynomial<T>, c: T) -> UnsignedPolynomial<T> {
        UnsignedPolynomial::from_coefficients_asc(
            p.coefficients_asc()
                .iter()
                .map(|&x| x.mod_power_of_2_mul(c, self.0))
                .collect(),
        )
    }
}

// The monic multiple of `p`, or `None` if its leading coefficient has no inverse.
pub fn naive_make_monic<T: PrimitiveUnsigned, A: NaiveGcdArithmetic<T>>(
    p: &UnsignedPolynomial<T>,
    a: &A,
) -> Option<UnsignedPolynomial<T>> {
    let inverse = a.inverse(*p.coefficients_asc().last()?)?;
    Some(a.scale(p, inverse))
}

// The monic GCD by Euclid's algorithm with schoolbook division, the longer polynomial divided
// first; `None` if a divisor's leading coefficient has no inverse.
pub fn gcd_naive<T: PrimitiveUnsigned, A: NaiveGcdArithmetic<T>>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    a: &A,
) -> Option<UnsignedPolynomial<T>> {
    if a.trivial() {
        return Some(UnsignedPolynomial::ZERO);
    }
    let (mut x, mut y) = if p.len() < q.len() {
        (q.clone(), p.clone())
    } else {
        (p.clone(), q.clone())
    };
    while y != UnsignedPolynomial::ZERO {
        let r = a.div_rem(&x, &y)?.1;
        x = core::mem::replace(&mut y, r);
    }
    if x == UnsignedPolynomial::ZERO {
        Some(x)
    } else {
        naive_make_monic(&x, a)
    }
}

pub fn mod_gcd_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    gcd_naive(p, q, &NaiveModulus(m))
}

// The monic GCD of the polynomials with coefficients `xs` and `ys`, neither ending with a zero, by
// the given library algorithm, which takes nonempty polynomials with the first at least as long;
// the cases it does not handle are done as the library does them. The modulus must not be 1.
pub fn monic_gcd_slices_with<C: Clone, M: GcdModulus<C>>(
    xs: &[C],
    ys: &[C],
    m: &M,
    algorithm: &dyn Fn(&[C], &[C], &M) -> Option<Vec<C>>,
) -> Option<Vec<C>> {
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
        algorithm(xs, ys, m)?
    };
    let inverse = m.inverse(g.last().unwrap())?;
    for c in &mut g {
        *c = m.mul(c, &inverse);
    }
    Some(g)
}

pub fn monic_gcd_with<T: PrimitiveUnsigned, M: GcdModulus<T>>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: &M,
    trivial: bool,
    algorithm: &dyn Fn(&[T], &[T], &M) -> Option<Vec<T>>,
) -> Option<UnsignedPolynomial<T>> {
    if trivial {
        return Some(UnsignedPolynomial::ZERO);
    }
    monic_gcd_slices_with(p.coefficients_asc(), q.coefficients_asc(), m, algorithm)
        .map(UnsignedPolynomial::from_coefficients_asc)
}

pub fn mod_gcd_euclidean<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    monic_gcd_with(p, q, &Modulus(m), m == T::ONE, &gcd_euclidean)
}

pub fn mod_gcd_hgcd<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    let modulus = Modulus(m);
    let cutoffs = modulus.gcd_cutoffs();
    monic_gcd_with(p, q, &modulus, m == T::ONE, &|xs, ys, m| {
        gcd_hgcd(xs, ys, cutoffs.hgcd_iter, cutoffs.hgcd_outer, m)
    })
}
