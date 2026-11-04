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
use crate::test_util::unsigned_polynomial::modular::mod_gcd::{NaiveGcdArithmetic, NaiveModulus};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::hgcd::xgcd_hgcd;
use crate::unsigned_polynomial::modular::mod_extended_gcd::xgcd_euclidean;
use crate::unsigned_polynomial::modular::mod_gcd::GcdModulus;
use crate::unsigned_polynomial::modular::subproduct_tree::Modulus;

// The monic GCD with cofactors by the extended Euclidean algorithm with schoolbook division.
#[allow(clippy::type_complexity)]
pub fn extended_gcd_naive<T: PrimitiveUnsigned, A: NaiveGcdArithmetic<T>>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    a: &A,
) -> Option<(
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
)> {
    if a.trivial() {
        return Some((
            UnsignedPolynomial::ZERO,
            UnsignedPolynomial::ZERO,
            UnsignedPolynomial::ZERO,
        ));
    }
    if p.len() < q.len() {
        let (g, t, s) = extended_gcd_naive(q, p, a)?;
        return Some((g, s, t));
    }
    let one = UnsignedPolynomial::from_coefficients_asc(vec![T::ONE]);
    let (mut r0, mut s0, mut t0) = (p.clone(), one.clone(), UnsignedPolynomial::ZERO);
    let (mut r1, mut s1, mut t1) = (q.clone(), UnsignedPolynomial::ZERO, one);
    while r1 != UnsignedPolynomial::ZERO {
        let (quotient, r) = a.div_rem(&r0, &r1)?;
        let s = a.sub(&s0, &a.mul(&quotient, &s1));
        let t = a.sub(&t0, &a.mul(&quotient, &t1));
        r0 = core::mem::replace(&mut r1, r);
        s0 = core::mem::replace(&mut s1, s);
        t0 = core::mem::replace(&mut t1, t);
    }
    if r0 == UnsignedPolynomial::ZERO {
        return Some((r0, UnsignedPolynomial::ZERO, UnsignedPolynomial::ZERO));
    }
    let inverse = a.inverse(*r0.coefficients_asc().last().unwrap())?;
    Some((
        a.scale(&r0, inverse),
        a.scale(&s0, inverse),
        a.scale(&t0, inverse),
    ))
}

#[allow(clippy::type_complexity)]
pub fn mod_extended_gcd_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> Option<(
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
)> {
    extended_gcd_naive(p, q, &NaiveModulus(m))
}

// The monic extended GCD of the polynomials with coefficients `xs` and `ys`, neither ending with a
// zero, by the given library algorithm, which takes polynomials of length at least 2 with the first
// at least as long; the other cases are done as the library does them. The modulus must not be 1.
#[allow(clippy::type_complexity)]
pub fn monic_extended_gcd_slices_with<C: Clone, M: GcdModulus<C>>(
    xs: &[C],
    ys: &[C],
    m: &M,
    algorithm: &dyn Fn(&[C], &[C], &M) -> Option<(Vec<C>, Vec<C>, Vec<C>)>,
) -> Option<(Vec<C>, Vec<C>, Vec<C>)> {
    if xs.len() < ys.len() {
        let (g, t, s) = monic_extended_gcd_slices_with(ys, xs, m, algorithm)?;
        return Some((g, s, t));
    }
    let (mut g, mut s, mut t) = if xs.is_empty() {
        return Some((Vec::new(), Vec::new(), Vec::new()));
    } else if ys.is_empty() {
        (xs.to_vec(), vec![m.one()], Vec::new())
    } else if ys.len() == 1 {
        let inverse = m.inverse(&ys[0])?;
        return Some((vec![m.one()], Vec::new(), vec![inverse]));
    } else {
        algorithm(xs, ys, m)?
    };
    let inverse = m.inverse(g.last().unwrap())?;
    for c in g.iter_mut().chain(s.iter_mut()).chain(t.iter_mut()) {
        *c = m.mul(c, &inverse);
    }
    Some((g, s, t))
}

#[allow(clippy::type_complexity)]
pub fn monic_extended_gcd_with<T: PrimitiveUnsigned, M: GcdModulus<T>>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: &M,
    trivial: bool,
    algorithm: &dyn Fn(&[T], &[T], &M) -> Option<(Vec<T>, Vec<T>, Vec<T>)>,
) -> Option<(
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
)> {
    if trivial {
        return Some((
            UnsignedPolynomial::ZERO,
            UnsignedPolynomial::ZERO,
            UnsignedPolynomial::ZERO,
        ));
    }
    let (g, s, t) =
        monic_extended_gcd_slices_with(p.coefficients_asc(), q.coefficients_asc(), m, algorithm)?;
    Some((
        UnsignedPolynomial::from_coefficients_asc(g),
        UnsignedPolynomial::from_coefficients_asc(s),
        UnsignedPolynomial::from_coefficients_asc(t),
    ))
}

#[allow(clippy::type_complexity)]
pub fn mod_extended_gcd_euclidean<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> Option<(
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
)> {
    monic_extended_gcd_with(p, q, &Modulus(m), m == T::ONE, &xgcd_euclidean)
}

#[allow(clippy::type_complexity)]
pub fn mod_extended_gcd_hgcd<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> Option<(
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
)> {
    let modulus = Modulus(m);
    let cutoffs = modulus.gcd_cutoffs();
    monic_extended_gcd_with(p, q, &modulus, m == T::ONE, &|xs, ys, m| {
        xgcd_hgcd(xs, ys, cutoffs.hgcd_iter, cutoffs.xgcd_hgcd, m)
    })
}
