// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_power_of_2_add::assert_reduced;
use crate::natural_polynomial::arithmetic::mod_power_of_2_taylor_shift::*;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Add, ModPowerOf2AddAssign, ModPowerOf2Mul, ModPowerOf2MulAssign,
    ModPowerOf2SquareAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::polynomial::{
    ModPowerOf2Compose, ModPowerOf2ComposeAssign, ModPowerOf2Evaluate,
};
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_compose::*;

// Whether a polynomial with `xs_len` coefficients is composed modulo $2^k$, where $k$ is `pow`,
// which is larger than a limb, with Horner's rule rather than divide and conquer, as measured on
// this machine: divide and conquer pays from about 12 coefficients up to a thousand bits or so, and
// from about 6 beyond.
const fn compose_horner_preferred(xs_len: usize, pow: u64) -> bool {
    xs_len <= if pow <= 1024 { 10 } else { 5 }
}

// The composition, modulo $2^k$, of the polynomial with coefficients `xs`, reduced and with at
// least two coefficients, with `q`, which is not constant, by Horner's rule over whole polynomials:
// $((x_{n-1} q + x_{n-2}) q + \cdots) q + x_0$.
crate_test_fn! {mod_power_of_2_compose_horner(
    xs: &[Natural],
    q: &NaturalPolynomial,
    pow: u64,
) -> NaturalPolynomial {
    let mut acc = NaturalPolynomial::ZERO;
    for x in xs.iter().rev() {
        acc = (&acc)
            .mod_power_of_2_mul(q, pow)
            .mod_power_of_2_add(NaturalPolynomial::from(x.clone()), pow);
    }
    acc
}}

// The composition, modulo $2^k$, of the polynomial with coefficients `xs`, reduced, with `q`, by
// divide and conquer: each coefficient of `xs` starts as a constant group; at every level,
// neighboring groups $g_{2i}$ and $g_{2i+1}$ merge into $g_{2i} + q^{2^j} g_{2i+1}$, and the power
// of `q` is squared, until one group is left.
crate_test_fn! {mod_power_of_2_compose_divide_and_conquer(
    xs: &[Natural],
    q: &NaturalPolynomial,
    pow: u64,
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
                lo.mod_power_of_2_add_assign((&power).mod_power_of_2_mul(hi, pow), pow);
            }
            merged.push(lo);
        }
        groups = merged;
        if groups.len() == 1 {
            break;
        }
        power.mod_power_of_2_square_assign(pow);
    }
    groups.pop().unwrap()
}}

// Composes the polynomial with coefficients `xs`, of length at least 2, with $bx + a$ modulo $2^k$,
// where `ys` is `[a, b]`, in place: a Taylor shift by $a$, after which the coefficient of $x^i$ is
// multiplied by $b^i$. The result is reduced but not trimmed.
fn mod_power_of_2_compose_linear_in_place(xs: &mut [Natural], ys: &[Natural], pow: u64) {
    let (a, b) = (&ys[0], &ys[1]);
    if *a != 0u32 {
        mod_power_of_2_taylor_shift_divide_and_conquer(xs, a, pow);
    }
    if *b != 1u32 {
        let mut power = b.clone();
        for x in &mut xs[1..] {
            x.mod_power_of_2_mul_assign(&power, pow);
            power.mod_power_of_2_mul_assign(b, pow);
        }
    }
}

// Composes the polynomial with coefficients `xs` with `q` modulo $2^k$; both are reduced.
fn mod_power_of_2_compose_helper(
    mut xs: Vec<Natural>,
    q: &NaturalPolynomial,
    pow: u64,
) -> NaturalPolynomial {
    if pow <= Limb::WIDTH {
        // Every coefficient fits in a limb, and arithmetic modulo $2^k$ is word arithmetic.
        let xs: Vec<Limb> = xs.iter().map(Limb::wrapping_from).collect();
        let ys: Vec<Limb> = q.coefficients.iter().map(Limb::wrapping_from).collect();
        return NaturalPolynomial {
            coefficients: mod_power_of_2_compose_vec(xs, &ys, pow)
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
                NaturalPolynomial { coefficients: xs }.mod_power_of_2_evaluate(y, pow),
            );
        }
        (_, [_, _]) => {
            mod_power_of_2_compose_linear_in_place(&mut xs, &q.coefficients, pow);
            NaturalPolynomial { coefficients: xs }
        }
        _ => {
            if compose_horner_preferred(xs.len(), pow) {
                mod_power_of_2_compose_horner(&xs, q, pow)
            } else {
                mod_power_of_2_compose_divide_and_conquer(&xs, q, pow)
            }
        }
    };
    p.trim();
    p
}

impl ModPowerOf2Compose<Self> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo $2^k$, giving $p(q(x)) \\bmod 2^k$, taking both
    /// by value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = p(q(x)) \\bmod 2^k.
    /// $$
    ///
    /// The coefficients are kept reduced throughout. When $2^k$ fits in a limb, the composition is
    /// done in wrapping limb arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// when $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer. The leading coefficient of the result can vanish modulo $2^k$, and then its
    /// degree is lower than the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m, k) = O((nm)^{\log_2 3} k \log k \log\log k)$
    ///
    /// $M(n, m, k) = O(nm(k + \log (nm)) \log (nmk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $m$ is `other.len()`, and
    /// $k$ is `pow`, the size to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2Compose;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_compose(NaturalPolynomial::from_str("2*x+5").unwrap(), 4)
    ///         .to_string(),
    ///     "4*x^2+10*x+10"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3")
    ///         .unwrap()
    ///         .mod_power_of_2_compose(NaturalPolynomial::from_str("x^2+x").unwrap(), 100)
    ///         .to_string(),
    ///     "x^6+3*x^5+3*x^4+x^3"
    /// );
    /// // Modulo 8, the leading coefficient 8 vanishes.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("2*x^2+1")
    ///         .unwrap()
    ///         .mod_power_of_2_compose(NaturalPolynomial::from_str("2*x").unwrap(), 3)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_compose(self, other: Self, pow: u64) -> Self {
        assert_reduced(&self, &other, pow);
        mod_power_of_2_compose_helper(self.coefficients, &other, pow)
    }
}

impl ModPowerOf2Compose<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo $2^k$, giving $p(q(x)) \\bmod 2^k$, taking the
    /// first by value and the second by reference. The coefficients of both must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = p(q(x)) \\bmod 2^k.
    /// $$
    ///
    /// The coefficients are kept reduced throughout. When $2^k$ fits in a limb, the composition is
    /// done in wrapping limb arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// when $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer. The leading coefficient of the result can vanish modulo $2^k$, and then its
    /// degree is lower than the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m, k) = O((nm)^{\log_2 3} k \log k \log\log k)$
    ///
    /// $M(n, m, k) = O(nm(k + \log (nm)) \log (nmk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $m$ is `other.len()`, and
    /// $k$ is `pow`, the size to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2Compose;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_compose(&NaturalPolynomial::from_str("2*x+5").unwrap(), 4)
    ///         .to_string(),
    ///     "4*x^2+10*x+10"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3")
    ///         .unwrap()
    ///         .mod_power_of_2_compose(&NaturalPolynomial::from_str("x^2+x").unwrap(), 100)
    ///         .to_string(),
    ///     "x^6+3*x^5+3*x^4+x^3"
    /// );
    /// // Modulo 8, the leading coefficient 8 vanishes.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("2*x^2+1")
    ///         .unwrap()
    ///         .mod_power_of_2_compose(&NaturalPolynomial::from_str("2*x").unwrap(), 3)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_compose(self, other: &Self, pow: u64) -> Self {
        assert_reduced(&self, other, pow);
        mod_power_of_2_compose_helper(self.coefficients, other, pow)
    }
}

impl ModPowerOf2Compose<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo $2^k$, giving $p(q(x)) \\bmod 2^k$, taking the
    /// first by reference and the second by value. The coefficients of both must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = p(q(x)) \\bmod 2^k.
    /// $$
    ///
    /// The coefficients are kept reduced throughout. When $2^k$ fits in a limb, the composition is
    /// done in wrapping limb arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// when $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer. The leading coefficient of the result can vanish modulo $2^k$, and then its
    /// degree is lower than the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m, k) = O((nm)^{\log_2 3} k \log k \log\log k)$
    ///
    /// $M(n, m, k) = O(nm(k + \log (nm)) \log (nmk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $m$ is `other.len()`, and
    /// $k$ is `pow`, the size to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2Compose;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_power_of_2_compose(NaturalPolynomial::from_str("2*x+5").unwrap(), 4)
    ///         .to_string(),
    ///     "4*x^2+10*x+10"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3").unwrap())
    ///         .mod_power_of_2_compose(NaturalPolynomial::from_str("x^2+x").unwrap(), 100)
    ///         .to_string(),
    ///     "x^6+3*x^5+3*x^4+x^3"
    /// );
    /// // Modulo 8, the leading coefficient 8 vanishes.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("2*x^2+1").unwrap())
    ///         .mod_power_of_2_compose(NaturalPolynomial::from_str("2*x").unwrap(), 3)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_compose(self, other: NaturalPolynomial, pow: u64) -> NaturalPolynomial {
        assert_reduced(self, &other, pow);
        mod_power_of_2_compose_helper(self.coefficients.clone(), &other, pow)
    }
}

impl ModPowerOf2Compose<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo $2^k$, giving $p(q(x)) \\bmod 2^k$, taking both
    /// by reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = p(q(x)) \\bmod 2^k.
    /// $$
    ///
    /// The coefficients are kept reduced throughout. When $2^k$ fits in a limb, the composition is
    /// done in wrapping limb arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// when $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer. The leading coefficient of the result can vanish modulo $2^k$, and then its
    /// degree is lower than the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m, k) = O((nm)^{\log_2 3} k \log k \log\log k)$
    ///
    /// $M(n, m, k) = O(nm(k + \log (nm)) \log (nmk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $m$ is `other.len()`, and
    /// $k$ is `pow`, the size to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2Compose;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_power_of_2_compose(&NaturalPolynomial::from_str("2*x+5").unwrap(), 4)
    ///         .to_string(),
    ///     "4*x^2+10*x+10"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3").unwrap())
    ///         .mod_power_of_2_compose(&NaturalPolynomial::from_str("x^2+x").unwrap(), 100)
    ///         .to_string(),
    ///     "x^6+3*x^5+3*x^4+x^3"
    /// );
    /// // Modulo 8, the leading coefficient 8 vanishes.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("2*x^2+1").unwrap())
    ///         .mod_power_of_2_compose(&NaturalPolynomial::from_str("2*x").unwrap(), 3)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_compose(self, other: &NaturalPolynomial, pow: u64) -> NaturalPolynomial {
        assert_reduced(self, other, pow);
        mod_power_of_2_compose_helper(self.coefficients.clone(), other, pow)
    }
}

impl ModPowerOf2ComposeAssign<Self> for NaturalPolynomial {
    /// Composes a [`NaturalPolynomial`] with another modulo $2^k$ in place, replacing $p$ with
    /// $p(q(x)) \\bmod 2^k$, taking the other by value. The coefficients of both must already be
    /// reduced modulo $2^k$.
    ///
    /// $$
    /// p \\gets p(q(x)) \\bmod 2^k.
    /// $$
    ///
    /// The coefficients are kept reduced throughout. When $2^k$ fits in a limb, the composition is
    /// done in wrapping limb arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// when $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer. The leading coefficient of the result can vanish modulo $2^k$, and then its
    /// degree is lower than the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m, k) = O((nm)^{\log_2 3} k \log k \log\log k)$
    ///
    /// $M(n, m, k) = O(nm(k + \log (nm)) \log (nmk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $m$ is `other.len()`, and
    /// $k$ is `pow`, the size to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2ComposeAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_compose_assign(NaturalPolynomial::from_str("2*x+5").unwrap(), 4);
    /// assert_eq!(p.to_string(), "4*x^2+10*x+10");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3").unwrap();
    /// p.mod_power_of_2_compose_assign(NaturalPolynomial::from_str("x^2+x").unwrap(), 100);
    /// assert_eq!(p.to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Modulo 8, the leading coefficient 8 vanishes.
    /// let mut p = NaturalPolynomial::from_str("2*x^2+1").unwrap();
    /// p.mod_power_of_2_compose_assign(NaturalPolynomial::from_str("2*x").unwrap(), 3);
    /// assert_eq!(p.to_string(), "1");
    /// ```
    #[inline]
    fn mod_power_of_2_compose_assign(&mut self, other: Self, pow: u64) {
        assert_reduced(self, &other, pow);
        *self = mod_power_of_2_compose_helper(take(&mut self.coefficients), &other, pow);
    }
}

impl ModPowerOf2ComposeAssign<&Self> for NaturalPolynomial {
    /// Composes a [`NaturalPolynomial`] with another modulo $2^k$ in place, replacing $p$ with
    /// $p(q(x)) \\bmod 2^k$, taking the other by reference. The coefficients of both must already
    /// be reduced modulo $2^k$.
    ///
    /// $$
    /// p \\gets p(q(x)) \\bmod 2^k.
    /// $$
    ///
    /// The coefficients are kept reduced throughout. When $2^k$ fits in a limb, the composition is
    /// done in wrapping limb arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// when $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer. The leading coefficient of the result can vanish modulo $2^k$, and then its
    /// degree is lower than the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m, k) = O((nm)^{\log_2 3} k \log k \log\log k)$
    ///
    /// $M(n, m, k) = O(nm(k + \log (nm)) \log (nmk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $m$ is `other.len()`, and
    /// $k$ is `pow`, the size to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2ComposeAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_compose_assign(&NaturalPolynomial::from_str("2*x+5").unwrap(), 4);
    /// assert_eq!(p.to_string(), "4*x^2+10*x+10");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3").unwrap();
    /// p.mod_power_of_2_compose_assign(&NaturalPolynomial::from_str("x^2+x").unwrap(), 100);
    /// assert_eq!(p.to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Modulo 8, the leading coefficient 8 vanishes.
    /// let mut p = NaturalPolynomial::from_str("2*x^2+1").unwrap();
    /// p.mod_power_of_2_compose_assign(&NaturalPolynomial::from_str("2*x").unwrap(), 3);
    /// assert_eq!(p.to_string(), "1");
    /// ```
    #[inline]
    fn mod_power_of_2_compose_assign(&mut self, other: &Self, pow: u64) {
        assert_reduced(self, other, pow);
        *self = mod_power_of_2_compose_helper(take(&mut self.coefficients), other, pow);
    }
}
