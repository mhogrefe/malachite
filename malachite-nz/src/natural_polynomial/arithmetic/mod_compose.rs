// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural::arithmetic::mod_mul::ModMulData;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_add::assert_reduced;
use crate::natural_polynomial::arithmetic::mod_taylor_shift::*;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{
    ModAdd, ModAddAssign, ModMul, ModMulPrecomputed, ModMulPrecomputedAssign, ModSquareAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::{ModCompose, ModComposeAssign, ModEvaluate};
use malachite_base::unsigned_polynomial::arithmetic::mod_compose::*;

// Whether a polynomial with `xs_len` coefficients is composed modulo a number larger than a limb
// with Horner's rule rather than divide and conquer, as measured on this machine: divide and
// conquer pays from about 7 coefficients, whatever the size of the modulus.
const fn compose_horner_preferred(xs_len: usize) -> bool {
    xs_len <= 6
}

// The composition, modulo `m`, of the polynomial with coefficients `xs`, reduced and nonempty, with
// `q`, by Horner's rule over whole polynomials: $((x_{n-1} q + x_{n-2}) q + \cdots) q + x_0$.
crate_test_fn! {mod_compose_horner(
    xs: &[Natural],
    q: &NaturalPolynomial,
    m: &Natural,
) -> NaturalPolynomial {
    let mut acc = NaturalPolynomial::ZERO;
    for x in xs.iter().rev() {
        acc = (&acc)
            .mod_mul(q, m)
            .mod_add(NaturalPolynomial::from(x.clone()), m);
    }
    acc
}}

// The composition, modulo `m`, of the polynomial with coefficients `xs`, reduced and nonempty, with
// `q`, by divide and conquer: each coefficient of `xs` starts as a constant group; at every level,
// neighboring groups $g_{2i}$ and $g_{2i+1}$ merge into $g_{2i} + q^{2^j} g_{2i+1}$, and the power
// of `q` is squared, until one group is left.
crate_test_fn! {mod_compose_divide_and_conquer(
    xs: &[Natural],
    q: &NaturalPolynomial,
    m: &Natural,
) -> NaturalPolynomial {
    let mut groups: Vec<NaturalPolynomial> =
        xs.iter().map(|x| NaturalPolynomial::from(x.clone())).collect();
    let mut power = q.clone();
    loop {
        let mut merged = Vec::with_capacity(groups.len().div_ceil(2));
        let mut groups_iter = groups.into_iter();
        while let Some(mut lo) = groups_iter.next() {
            if let Some(hi) = groups_iter.next()
                && hi != 0u32
            {
                lo.mod_add_assign((&power).mod_mul(hi, m), m);
            }
            merged.push(lo);
        }
        groups = merged;
        if groups.len() == 1 {
            break;
        }
        power.mod_square_assign(m);
    }
    groups.pop().unwrap()
}}

// Composes the polynomial with coefficients `xs`, of length at least 2, with $bx + a$ modulo `m`,
// where `ys` is `[a, b]`, in place: a Taylor shift by $a$, after which the coefficient of $x^i$ is
// multiplied by $b^i$. The result is reduced but not trimmed.
fn mod_compose_linear_in_place(xs: &mut [Natural], ys: &[Natural], m: &Natural) {
    let (a, b) = (&ys[0], &ys[1]);
    if *a != 0u32 {
        mod_taylor_shift_divide_and_conquer(xs, a, m);
    }
    if *b != 1u32 {
        let data: ModMulData =
            <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
        let mut power = b.clone();
        for x in &mut xs[1..] {
            x.mod_mul_precomputed_assign(&power, m, &data);
            power.mod_mul_precomputed_assign(b, m, &data);
        }
    }
}

// Composes the polynomial with coefficients `xs` with `q` modulo `m`; both are reduced.
fn mod_compose_helper(
    mut xs: Vec<Natural>,
    q: &NaturalPolynomial,
    m: &Natural,
) -> NaturalPolynomial {
    if m.significant_bits() <= Limb::WIDTH {
        // Every coefficient fits in a limb.
        let xs: Vec<Limb> = xs.iter().map(Limb::wrapping_from).collect();
        let ys: Vec<Limb> = q.coefficients.iter().map(Limb::wrapping_from).collect();
        return NaturalPolynomial {
            coefficients: mod_compose_vec(xs, &ys, Limb::wrapping_from(m))
                .into_iter()
                .map(Natural::from)
                .collect(),
        };
    }
    let mut p = match (xs.as_slice(), q.coefficients.as_slice()) {
        ([] | [_], _) => return NaturalPolynomial { coefficients: xs },
        ([x, ..], []) => return NaturalPolynomial::from(x.clone()),
        (_, [y]) => {
            return NaturalPolynomial::from(
                NaturalPolynomial { coefficients: xs }.mod_evaluate(y, m),
            );
        }
        (_, [_, _]) => {
            mod_compose_linear_in_place(&mut xs, &q.coefficients, m);
            NaturalPolynomial { coefficients: xs }
        }
        _ => {
            if compose_horner_preferred(xs.len()) {
                mod_compose_horner(&xs, q, m)
            } else {
                mod_compose_divide_and_conquer(&xs, q, m)
            }
        }
    };
    p.trim();
    p
}

impl ModCompose<Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo $m$, giving $p(q(x)) \\bmod m$, taking all three
    /// by value. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, m) = p(q(x)) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// when $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer. Nothing is divided modulo $m$, so any modulus works, prime or not. The leading
    /// coefficient of the result can vanish modulo $m$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O((nk)^{\log_2 3} b \log b \log\log b)$
    ///
    /// $M(n, k, b) = O(nk(b + \log (nk)) \log (nkb))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $b$ is `m.significant_bits()`, the size to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModCompose;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+5").unwrap();
    /// let m = Natural::from(7u32);
    /// assert_eq!(p.mod_compose(q, m).to_string(), "4*x^2+5*x");
    ///
    /// let p = NaturalPolynomial::from_str("x^3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+x").unwrap();
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// assert_eq!(p.mod_compose(q, m).to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Modulo 12, the leading coefficient 12 vanishes.
    /// let p = NaturalPolynomial::from_str("3*x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x").unwrap();
    /// let m = Natural::from(12u32);
    /// assert_eq!(p.mod_compose(q, m).to_string(), "1");
    /// ```
    #[inline]
    fn mod_compose(self, other: Self, m: Natural) -> Self {
        assert_reduced(&self, &other, &m);
        mod_compose_helper(self.coefficients, &other, &m)
    }
}

impl ModCompose<Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo $m$, giving $p(q(x)) \\bmod m$, taking the first
    /// polynomial by value, the second by value, and $m$ by reference. The coefficients of both
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, m) = p(q(x)) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// when $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer. Nothing is divided modulo $m$, so any modulus works, prime or not. The leading
    /// coefficient of the result can vanish modulo $m$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O((nk)^{\log_2 3} b \log b \log\log b)$
    ///
    /// $M(n, k, b) = O(nk(b + \log (nk)) \log (nkb))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $b$ is `m.significant_bits()`, the size to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModCompose;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+5").unwrap();
    /// let m = Natural::from(7u32);
    /// assert_eq!(p.mod_compose(q, &m).to_string(), "4*x^2+5*x");
    ///
    /// let p = NaturalPolynomial::from_str("x^3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+x").unwrap();
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// assert_eq!(p.mod_compose(q, &m).to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Modulo 12, the leading coefficient 12 vanishes.
    /// let p = NaturalPolynomial::from_str("3*x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x").unwrap();
    /// let m = Natural::from(12u32);
    /// assert_eq!(p.mod_compose(q, &m).to_string(), "1");
    /// ```
    #[inline]
    fn mod_compose(self, other: Self, m: &Natural) -> Self {
        assert_reduced(&self, &other, m);
        mod_compose_helper(self.coefficients, &other, m)
    }
}

impl ModCompose<&Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo $m$, giving $p(q(x)) \\bmod m$, taking the first
    /// polynomial by value, the second by reference, and $m$ by value. The coefficients of both
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, m) = p(q(x)) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// when $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer. Nothing is divided modulo $m$, so any modulus works, prime or not. The leading
    /// coefficient of the result can vanish modulo $m$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O((nk)^{\log_2 3} b \log b \log\log b)$
    ///
    /// $M(n, k, b) = O(nk(b + \log (nk)) \log (nkb))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $b$ is `m.significant_bits()`, the size to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModCompose;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+5").unwrap();
    /// let m = Natural::from(7u32);
    /// assert_eq!(p.mod_compose(&q, m).to_string(), "4*x^2+5*x");
    ///
    /// let p = NaturalPolynomial::from_str("x^3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+x").unwrap();
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// assert_eq!(p.mod_compose(&q, m).to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Modulo 12, the leading coefficient 12 vanishes.
    /// let p = NaturalPolynomial::from_str("3*x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x").unwrap();
    /// let m = Natural::from(12u32);
    /// assert_eq!(p.mod_compose(&q, m).to_string(), "1");
    /// ```
    #[inline]
    fn mod_compose(self, other: &Self, m: Natural) -> Self {
        assert_reduced(&self, other, &m);
        mod_compose_helper(self.coefficients, other, &m)
    }
}

impl ModCompose<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo $m$, giving $p(q(x)) \\bmod m$, taking the first
    /// polynomial by value, the second by reference, and $m$ by reference. The coefficients of both
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, m) = p(q(x)) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// when $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer. Nothing is divided modulo $m$, so any modulus works, prime or not. The leading
    /// coefficient of the result can vanish modulo $m$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O((nk)^{\log_2 3} b \log b \log\log b)$
    ///
    /// $M(n, k, b) = O(nk(b + \log (nk)) \log (nkb))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $b$ is `m.significant_bits()`, the size to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModCompose;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+5").unwrap();
    /// let m = Natural::from(7u32);
    /// assert_eq!(p.mod_compose(&q, &m).to_string(), "4*x^2+5*x");
    ///
    /// let p = NaturalPolynomial::from_str("x^3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+x").unwrap();
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// assert_eq!(p.mod_compose(&q, &m).to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Modulo 12, the leading coefficient 12 vanishes.
    /// let p = NaturalPolynomial::from_str("3*x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x").unwrap();
    /// let m = Natural::from(12u32);
    /// assert_eq!(p.mod_compose(&q, &m).to_string(), "1");
    /// ```
    #[inline]
    fn mod_compose(self, other: &Self, m: &Natural) -> Self {
        assert_reduced(&self, other, m);
        mod_compose_helper(self.coefficients, other, m)
    }
}

impl ModCompose<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo $m$, giving $p(q(x)) \\bmod m$, taking the first
    /// polynomial by reference, the second by value, and $m$ by value. The coefficients of both
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, m) = p(q(x)) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// when $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer. Nothing is divided modulo $m$, so any modulus works, prime or not. The leading
    /// coefficient of the result can vanish modulo $m$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O((nk)^{\log_2 3} b \log b \log\log b)$
    ///
    /// $M(n, k, b) = O(nk(b + \log (nk)) \log (nkb))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $b$ is `m.significant_bits()`, the size to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModCompose;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+5").unwrap();
    /// let m = Natural::from(7u32);
    /// assert_eq!((&p).mod_compose(q, m).to_string(), "4*x^2+5*x");
    ///
    /// let p = NaturalPolynomial::from_str("x^3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+x").unwrap();
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// assert_eq!((&p).mod_compose(q, m).to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Modulo 12, the leading coefficient 12 vanishes.
    /// let p = NaturalPolynomial::from_str("3*x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x").unwrap();
    /// let m = Natural::from(12u32);
    /// assert_eq!((&p).mod_compose(q, m).to_string(), "1");
    /// ```
    #[inline]
    fn mod_compose(self, other: NaturalPolynomial, m: Natural) -> NaturalPolynomial {
        assert_reduced(self, &other, &m);
        mod_compose_helper(self.coefficients.clone(), &other, &m)
    }
}

impl ModCompose<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo $m$, giving $p(q(x)) \\bmod m$, taking the first
    /// polynomial by reference, the second by value, and $m$ by reference. The coefficients of both
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, m) = p(q(x)) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// when $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer. Nothing is divided modulo $m$, so any modulus works, prime or not. The leading
    /// coefficient of the result can vanish modulo $m$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O((nk)^{\log_2 3} b \log b \log\log b)$
    ///
    /// $M(n, k, b) = O(nk(b + \log (nk)) \log (nkb))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $b$ is `m.significant_bits()`, the size to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModCompose;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+5").unwrap();
    /// let m = Natural::from(7u32);
    /// assert_eq!((&p).mod_compose(q, &m).to_string(), "4*x^2+5*x");
    ///
    /// let p = NaturalPolynomial::from_str("x^3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+x").unwrap();
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// assert_eq!((&p).mod_compose(q, &m).to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Modulo 12, the leading coefficient 12 vanishes.
    /// let p = NaturalPolynomial::from_str("3*x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x").unwrap();
    /// let m = Natural::from(12u32);
    /// assert_eq!((&p).mod_compose(q, &m).to_string(), "1");
    /// ```
    #[inline]
    fn mod_compose(self, other: NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
        assert_reduced(self, &other, m);
        mod_compose_helper(self.coefficients.clone(), &other, m)
    }
}

impl ModCompose<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo $m$, giving $p(q(x)) \\bmod m$, taking the first
    /// polynomial by reference, the second by reference, and $m$ by value. The coefficients of both
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, m) = p(q(x)) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// when $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer. Nothing is divided modulo $m$, so any modulus works, prime or not. The leading
    /// coefficient of the result can vanish modulo $m$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O((nk)^{\log_2 3} b \log b \log\log b)$
    ///
    /// $M(n, k, b) = O(nk(b + \log (nk)) \log (nkb))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $b$ is `m.significant_bits()`, the size to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModCompose;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+5").unwrap();
    /// let m = Natural::from(7u32);
    /// assert_eq!((&p).mod_compose(&q, m).to_string(), "4*x^2+5*x");
    ///
    /// let p = NaturalPolynomial::from_str("x^3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+x").unwrap();
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// assert_eq!((&p).mod_compose(&q, m).to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Modulo 12, the leading coefficient 12 vanishes.
    /// let p = NaturalPolynomial::from_str("3*x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x").unwrap();
    /// let m = Natural::from(12u32);
    /// assert_eq!((&p).mod_compose(&q, m).to_string(), "1");
    /// ```
    #[inline]
    fn mod_compose(self, other: &NaturalPolynomial, m: Natural) -> NaturalPolynomial {
        assert_reduced(self, other, &m);
        mod_compose_helper(self.coefficients.clone(), other, &m)
    }
}

impl ModCompose<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo $m$, giving $p(q(x)) \\bmod m$, taking all three
    /// by reference. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, m) = p(q(x)) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// when $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer. Nothing is divided modulo $m$, so any modulus works, prime or not. The leading
    /// coefficient of the result can vanish modulo $m$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O((nk)^{\log_2 3} b \log b \log\log b)$
    ///
    /// $M(n, k, b) = O(nk(b + \log (nk)) \log (nkb))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $b$ is `m.significant_bits()`, the size to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModCompose;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+5").unwrap();
    /// let m = Natural::from(7u32);
    /// assert_eq!((&p).mod_compose(&q, &m).to_string(), "4*x^2+5*x");
    ///
    /// let p = NaturalPolynomial::from_str("x^3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+x").unwrap();
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// assert_eq!((&p).mod_compose(&q, &m).to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Modulo 12, the leading coefficient 12 vanishes.
    /// let p = NaturalPolynomial::from_str("3*x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x").unwrap();
    /// let m = Natural::from(12u32);
    /// assert_eq!((&p).mod_compose(&q, &m).to_string(), "1");
    /// ```
    #[inline]
    fn mod_compose(self, other: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
        assert_reduced(self, other, m);
        mod_compose_helper(self.coefficients.clone(), other, m)
    }
}

impl ModComposeAssign<Self, Natural> for NaturalPolynomial {
    /// Composes a [`NaturalPolynomial`] with another modulo $m$ in place, replacing $p$ with
    /// $p(q(x)) \\bmod m$, taking the other by value and $m$ by value. The coefficients of both
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// p \\gets p(q(x)) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// when $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer. Nothing is divided modulo $m$, so any modulus works, prime or not. The leading
    /// coefficient of the result can vanish modulo $m$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O((nk)^{\log_2 3} b \log b \log\log b)$
    ///
    /// $M(n, k, b) = O(nk(b + \log (nk)) \log (nkb))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $b$ is `m.significant_bits()`, the size to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+5").unwrap();
    /// let m = Natural::from(7u32);
    /// p.mod_compose_assign(q, m);
    /// assert_eq!(p.to_string(), "4*x^2+5*x");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+x").unwrap();
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// p.mod_compose_assign(q, m);
    /// assert_eq!(p.to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Modulo 12, the leading coefficient 12 vanishes.
    /// let mut p = NaturalPolynomial::from_str("3*x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x").unwrap();
    /// let m = Natural::from(12u32);
    /// p.mod_compose_assign(q, m);
    /// assert_eq!(p.to_string(), "1");
    /// ```
    #[inline]
    fn mod_compose_assign(&mut self, other: Self, m: Natural) {
        assert_reduced(self, &other, &m);
        *self = mod_compose_helper(take(&mut self.coefficients), &other, &m);
    }
}

impl ModComposeAssign<Self, &Natural> for NaturalPolynomial {
    /// Composes a [`NaturalPolynomial`] with another modulo $m$ in place, replacing $p$ with
    /// $p(q(x)) \\bmod m$, taking the other by value and $m$ by reference. The coefficients of both
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// p \\gets p(q(x)) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// when $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer. Nothing is divided modulo $m$, so any modulus works, prime or not. The leading
    /// coefficient of the result can vanish modulo $m$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O((nk)^{\log_2 3} b \log b \log\log b)$
    ///
    /// $M(n, k, b) = O(nk(b + \log (nk)) \log (nkb))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $b$ is `m.significant_bits()`, the size to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+5").unwrap();
    /// let m = Natural::from(7u32);
    /// p.mod_compose_assign(q, &m);
    /// assert_eq!(p.to_string(), "4*x^2+5*x");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+x").unwrap();
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// p.mod_compose_assign(q, &m);
    /// assert_eq!(p.to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Modulo 12, the leading coefficient 12 vanishes.
    /// let mut p = NaturalPolynomial::from_str("3*x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x").unwrap();
    /// let m = Natural::from(12u32);
    /// p.mod_compose_assign(q, &m);
    /// assert_eq!(p.to_string(), "1");
    /// ```
    #[inline]
    fn mod_compose_assign(&mut self, other: Self, m: &Natural) {
        assert_reduced(self, &other, m);
        *self = mod_compose_helper(take(&mut self.coefficients), &other, m);
    }
}

impl ModComposeAssign<&Self, Natural> for NaturalPolynomial {
    /// Composes a [`NaturalPolynomial`] with another modulo $m$ in place, replacing $p$ with
    /// $p(q(x)) \\bmod m$, taking the other by reference and $m$ by value. The coefficients of both
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// p \\gets p(q(x)) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// when $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer. Nothing is divided modulo $m$, so any modulus works, prime or not. The leading
    /// coefficient of the result can vanish modulo $m$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O((nk)^{\log_2 3} b \log b \log\log b)$
    ///
    /// $M(n, k, b) = O(nk(b + \log (nk)) \log (nkb))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $b$ is `m.significant_bits()`, the size to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+5").unwrap();
    /// let m = Natural::from(7u32);
    /// p.mod_compose_assign(&q, m);
    /// assert_eq!(p.to_string(), "4*x^2+5*x");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+x").unwrap();
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// p.mod_compose_assign(&q, m);
    /// assert_eq!(p.to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Modulo 12, the leading coefficient 12 vanishes.
    /// let mut p = NaturalPolynomial::from_str("3*x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x").unwrap();
    /// let m = Natural::from(12u32);
    /// p.mod_compose_assign(&q, m);
    /// assert_eq!(p.to_string(), "1");
    /// ```
    #[inline]
    fn mod_compose_assign(&mut self, other: &Self, m: Natural) {
        assert_reduced(self, other, &m);
        *self = mod_compose_helper(take(&mut self.coefficients), other, &m);
    }
}

impl ModComposeAssign<&Self, &Natural> for NaturalPolynomial {
    /// Composes a [`NaturalPolynomial`] with another modulo $m$ in place, replacing $p$ with
    /// $p(q(x)) \\bmod m$, taking the other by reference and $m$ by reference. The coefficients of
    /// both must already be reduced modulo $m$.
    ///
    /// $$
    /// p \\gets p(q(x)) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// when $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer. Nothing is divided modulo $m$, so any modulus works, prime or not. The leading
    /// coefficient of the result can vanish modulo $m$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O((nk)^{\log_2 3} b \log b \log\log b)$
    ///
    /// $M(n, k, b) = O(nk(b + \log (nk)) \log (nkb))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $b$ is `m.significant_bits()`, the size to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+5").unwrap();
    /// let m = Natural::from(7u32);
    /// p.mod_compose_assign(&q, &m);
    /// assert_eq!(p.to_string(), "4*x^2+5*x");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+x").unwrap();
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// p.mod_compose_assign(&q, &m);
    /// assert_eq!(p.to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Modulo 12, the leading coefficient 12 vanishes.
    /// let mut p = NaturalPolynomial::from_str("3*x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x").unwrap();
    /// let m = Natural::from(12u32);
    /// p.mod_compose_assign(&q, &m);
    /// assert_eq!(p.to_string(), "1");
    /// ```
    #[inline]
    fn mod_compose_assign(&mut self, other: &Self, m: &Natural) {
        assert_reduced(self, other, m);
        *self = mod_compose_helper(take(&mut self.coefficients), other, m);
    }
}
