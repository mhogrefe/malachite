// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{ModCompose, ModComposeAssign, ModEvaluate};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_add::assert_reduced;
use crate::unsigned_polynomial::arithmetic::mod_mul::mod_mul_to_out;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::from_coefficients_trimmed;
use crate::unsigned_polynomial::arithmetic::mod_square::mod_square_to_out;
use crate::unsigned_polynomial::arithmetic::mod_taylor_shift::*;
use alloc::vec;
use alloc::vec::Vec;
use core::mem::{swap, take};

// Whether a polynomial with `xs_len` coefficients is composed with a polynomial with `ys_len`
// coefficients by Horner's rule rather than divide and conquer, as measured on this machine: divide
// and conquer pays from about 16 coefficients when the inner polynomial is short, and from about 10
// when it is longer, for every word type and modulus size.
const fn compose_horner_preferred(xs_len: usize, ys_len: usize) -> bool {
    xs_len <= if ys_len > 4 { 9 } else { 15 }
}

// The coefficients of the composition of the polynomials with coefficients `xs` and `ys`, both
// nonempty and reduced modulo `m`, modulo `m`, by Horner's rule over whole polynomials: $((x_{n-1}
// y + x_{n-2}) y + \cdots) y + x_0$. The result is not trimmed.
crate_test_fn! {mod_compose_horner<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], m: T) -> Vec<T> {
    let (&leading, rest) = xs.split_last().unwrap();
    let mut acc = vec![leading];
    let mut scratch = Vec::new();
    for &x in rest.iter().rev() {
        scratch.resize(acc.len() + ys.len() - 1, T::ZERO);
        mod_mul_to_out(&mut scratch, &acc, ys, m);
        scratch[0].mod_add_assign(x, m);
        swap(&mut acc, &mut scratch);
    }
    acc
}}

// Adds `ys` into `xs` modulo `m`, reusing whichever of the two is longer.
fn mod_add_assign_val<T: PrimitiveUnsigned>(xs: &mut Vec<T>, mut ys: Vec<T>, m: T) {
    if ys.len() > xs.len() {
        swap(xs, &mut ys);
    }
    for (x, y) in xs.iter_mut().zip(ys) {
        x.mod_add_assign(y, m);
    }
}

// The coefficients of the composition of the polynomials with coefficients `xs` and `ys`, both
// nonempty and reduced modulo `m`, modulo `m`, by divide and conquer: each coefficient of `xs`
// starts as a constant group; at every level, neighboring groups $g_{2i}$ and $g_{2i+1}$ merge into
// $g_{2i} + y^{2^j} g_{2i+1}$, and the power of `ys` is squared, until one group is left. Zero
// groups are kept empty. The result is not trimmed.
crate_test_fn! {mod_compose_divide_and_conquer<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    m: T,
) -> Vec<T> {
    let mut groups: Vec<Vec<T>> = xs
        .iter()
        .map(|&x| if x == T::ZERO { Vec::new() } else { vec![x] })
        .collect();
    let mut power: Vec<T> = ys.to_vec();
    loop {
        let mut merged = Vec::with_capacity(groups.len().div_ceil(2));
        let mut groups_iter = groups.into_iter();
        while let Some(mut lo) = groups_iter.next() {
            if let Some(hi) = groups_iter.next()
                && !hi.is_empty()
            {
                let mut product = vec![T::ZERO; power.len() + hi.len() - 1];
                mod_mul_to_out(&mut product, &power, &hi, m);
                mod_add_assign_val(&mut lo, product, m);
            }
            merged.push(lo);
        }
        groups = merged;
        if groups.len() == 1 {
            break;
        }
        let mut square = vec![T::ZERO; (power.len() << 1) - 1];
        mod_square_to_out(&mut square, &power, m);
        power = square;
    }
    groups.pop().unwrap()
}}

// Composes the polynomial with coefficients `xs`, of length at least 2, with $bx + a$ modulo `m`,
// where `ys` is `[a, b]`, in place: a Taylor shift by $a$, after which the coefficient of $x^i$ is
// multiplied by $b^i$. The result is not trimmed.
fn mod_compose_linear_in_place<T: PrimitiveUnsigned>(xs: &mut [T], ys: &[T], m: T) {
    let (a, b) = (ys[0], ys[1]);
    if a != T::ZERO {
        mod_taylor_shift_divide_and_conquer(xs, a, m);
    }
    if b != T::ONE {
        let data = T::precompute_mod_mul_data(&m);
        let mut power = b;
        for x in &mut xs[1..] {
            x.mod_mul_precomputed_assign(power, m, &data);
            power.mod_mul_precomputed_assign(b, m, &data);
        }
    }
}

/// The coefficients of the composition, modulo `m`, of the polynomials with coefficients `xs` and
/// `ys`, both reduced modulo `m` and with no zeros at the end, trimmed. `m` must be positive.
///
/// This is not part of the public API; it is public so that `malachite-nz` can compose
/// `NaturalPolynomial`s modulo a word.
#[doc(hidden)]
pub fn mod_compose_vec<T: PrimitiveUnsigned>(mut xs: Vec<T>, ys: &[T], m: T) -> Vec<T> {
    let coefficients = match (xs.as_slice(), ys) {
        ([] | [_], _) => return xs,
        ([x, ..], []) => vec![*x],
        (_, [y]) => vec![UnsignedPolynomial { coefficients: xs }.mod_evaluate(*y, m)],
        (_, [_, _]) => {
            mod_compose_linear_in_place(&mut xs, ys, m);
            xs
        }
        _ => {
            if compose_horner_preferred(xs.len(), ys.len()) {
                mod_compose_horner(&xs, ys, m)
            } else {
                mod_compose_divide_and_conquer(&xs, ys, m)
            }
        }
    };
    from_coefficients_trimmed(coefficients).coefficients
}

impl<T: PrimitiveUnsigned> ModCompose<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Composes two [`UnsignedPolynomial`]s modulo $m$, giving $p(q(x)) \\bmod m$, taking both by
    /// value. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, m) = p(q(x)) \\bmod m.
    /// $$
    ///
    /// When $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer, which merges neighboring groups of coefficients with ever higher powers
    /// $q^{2^j}$. Nothing is divided modulo $m$, so any modulus works, prime or not. The leading
    /// coefficient of the result can vanish modulo $m$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O((nk)^{\log_2 3})$
    ///
    /// $M(n, k) = O(nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `other.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModCompose;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_compose(UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 7)
    ///         .to_string(),
    ///     "4*x^2+5*x"
    /// );
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^3")
    ///         .unwrap()
    ///         .mod_compose(UnsignedPolynomial::<u8>::from_str("x^2+x").unwrap(), 7)
    ///         .to_string(),
    ///     "x^6+3*x^5+3*x^4+x^3"
    /// );
    /// // Modulo 12, the leading coefficient 12 vanishes.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("3*x^2+1")
    ///         .unwrap()
    ///         .mod_compose(UnsignedPolynomial::<u8>::from_str("2*x").unwrap(), 12)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    #[inline]
    fn mod_compose(self, other: Self, m: T) -> Self {
        assert_reduced(&self, &other, m);
        Self {
            coefficients: mod_compose_vec(self.coefficients, &other.coefficients, m),
        }
    }
}

impl<T: PrimitiveUnsigned> ModCompose<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Composes two [`UnsignedPolynomial`]s modulo $m$, giving $p(q(x)) \\bmod m$, taking the first
    /// by value and the second by reference. The coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(p, q, m) = p(q(x)) \\bmod m.
    /// $$
    ///
    /// When $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer, which merges neighboring groups of coefficients with ever higher powers
    /// $q^{2^j}$. Nothing is divided modulo $m$, so any modulus works, prime or not. The leading
    /// coefficient of the result can vanish modulo $m$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O((nk)^{\log_2 3})$
    ///
    /// $M(n, k) = O(nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `other.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModCompose;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_compose(&UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 7)
    ///         .to_string(),
    ///     "4*x^2+5*x"
    /// );
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^3")
    ///         .unwrap()
    ///         .mod_compose(&UnsignedPolynomial::<u8>::from_str("x^2+x").unwrap(), 7)
    ///         .to_string(),
    ///     "x^6+3*x^5+3*x^4+x^3"
    /// );
    /// // Modulo 12, the leading coefficient 12 vanishes.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("3*x^2+1")
    ///         .unwrap()
    ///         .mod_compose(&UnsignedPolynomial::<u8>::from_str("2*x").unwrap(), 12)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    #[inline]
    fn mod_compose(self, other: &Self, m: T) -> Self {
        assert_reduced(&self, other, m);
        Self {
            coefficients: mod_compose_vec(self.coefficients, &other.coefficients, m),
        }
    }
}

impl<T: PrimitiveUnsigned> ModCompose<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo $m$, giving $p(q(x)) \\bmod m$, taking the first
    /// by reference and the second by value. The coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(p, q, m) = p(q(x)) \\bmod m.
    /// $$
    ///
    /// When $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer, which merges neighboring groups of coefficients with ever higher powers
    /// $q^{2^j}$. Nothing is divided modulo $m$, so any modulus works, prime or not. The leading
    /// coefficient of the result can vanish modulo $m$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O((nk)^{\log_2 3})$
    ///
    /// $M(n, k) = O(nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `other.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModCompose;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_compose(UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 7)
    ///         .to_string(),
    ///     "4*x^2+5*x"
    /// );
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^3").unwrap())
    ///         .mod_compose(UnsignedPolynomial::<u8>::from_str("x^2+x").unwrap(), 7)
    ///         .to_string(),
    ///     "x^6+3*x^5+3*x^4+x^3"
    /// );
    /// // Modulo 12, the leading coefficient 12 vanishes.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("3*x^2+1").unwrap())
    ///         .mod_compose(UnsignedPolynomial::<u8>::from_str("2*x").unwrap(), 12)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    #[inline]
    fn mod_compose(self, other: UnsignedPolynomial<T>, m: T) -> UnsignedPolynomial<T> {
        assert_reduced(self, &other, m);
        UnsignedPolynomial {
            coefficients: mod_compose_vec(self.coefficients.clone(), &other.coefficients, m),
        }
    }
}

impl<T: PrimitiveUnsigned> ModCompose<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo $m$, giving $p(q(x)) \\bmod m$, taking both by
    /// reference. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, m) = p(q(x)) \\bmod m.
    /// $$
    ///
    /// When $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer, which merges neighboring groups of coefficients with ever higher powers
    /// $q^{2^j}$. Nothing is divided modulo $m$, so any modulus works, prime or not. The leading
    /// coefficient of the result can vanish modulo $m$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O((nk)^{\log_2 3})$
    ///
    /// $M(n, k) = O(nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `other.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModCompose;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_compose(&UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 7)
    ///         .to_string(),
    ///     "4*x^2+5*x"
    /// );
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^3").unwrap())
    ///         .mod_compose(&UnsignedPolynomial::<u8>::from_str("x^2+x").unwrap(), 7)
    ///         .to_string(),
    ///     "x^6+3*x^5+3*x^4+x^3"
    /// );
    /// // Modulo 12, the leading coefficient 12 vanishes.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("3*x^2+1").unwrap())
    ///         .mod_compose(&UnsignedPolynomial::<u8>::from_str("2*x").unwrap(), 12)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    #[inline]
    fn mod_compose(self, other: &UnsignedPolynomial<T>, m: T) -> UnsignedPolynomial<T> {
        assert_reduced(self, other, m);
        UnsignedPolynomial {
            coefficients: mod_compose_vec(self.coefficients.clone(), &other.coefficients, m),
        }
    }
}

impl<T: PrimitiveUnsigned> ModComposeAssign<Self, T> for UnsignedPolynomial<T> {
    /// Composes an [`UnsignedPolynomial`] with another modulo $m$ in place, replacing $p$ with
    /// $p(q(x)) \\bmod m$, taking the other by value. The coefficients of both must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// p \\gets p(q(x)) \\bmod m.
    /// $$
    ///
    /// When $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer, which merges neighboring groups of coefficients with ever higher powers
    /// $q^{2^j}$. Nothing is divided modulo $m$, so any modulus works, prime or not. The leading
    /// coefficient of the result can vanish modulo $m$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O((nk)^{\log_2 3})$
    ///
    /// $M(n, k) = O(nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `other.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_compose_assign(UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 7);
    /// assert_eq!(p.to_string(), "4*x^2+5*x");
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3").unwrap();
    /// p.mod_compose_assign(UnsignedPolynomial::<u8>::from_str("x^2+x").unwrap(), 7);
    /// assert_eq!(p.to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Modulo 12, the leading coefficient 12 vanishes.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("3*x^2+1").unwrap();
    /// p.mod_compose_assign(UnsignedPolynomial::<u8>::from_str("2*x").unwrap(), 12);
    /// assert_eq!(p.to_string(), "1");
    /// ```
    #[inline]
    fn mod_compose_assign(&mut self, other: Self, m: T) {
        assert_reduced(self, &other, m);
        self.coefficients = mod_compose_vec(take(&mut self.coefficients), &other.coefficients, m);
    }
}

impl<T: PrimitiveUnsigned> ModComposeAssign<&Self, T> for UnsignedPolynomial<T> {
    /// Composes an [`UnsignedPolynomial`] with another modulo $m$ in place, replacing $p$ with
    /// $p(q(x)) \\bmod m$, taking the other by reference. The coefficients of both must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// p \\gets p(q(x)) \\bmod m.
    /// $$
    ///
    /// When $q$ is a constant $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$,
    /// it is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is multiplied by
    /// $b^i$; otherwise short polynomials are composed with Horner's rule and longer ones by divide
    /// and conquer, which merges neighboring groups of coefficients with ever higher powers
    /// $q^{2^j}$. Nothing is divided modulo $m$, so any modulus works, prime or not. The leading
    /// coefficient of the result can vanish modulo $m$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O((nk)^{\log_2 3})$
    ///
    /// $M(n, k) = O(nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `other.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_compose_assign(&UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 7);
    /// assert_eq!(p.to_string(), "4*x^2+5*x");
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3").unwrap();
    /// p.mod_compose_assign(&UnsignedPolynomial::<u8>::from_str("x^2+x").unwrap(), 7);
    /// assert_eq!(p.to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Modulo 12, the leading coefficient 12 vanishes.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("3*x^2+1").unwrap();
    /// p.mod_compose_assign(&UnsignedPolynomial::<u8>::from_str("2*x").unwrap(), 12);
    /// assert_eq!(p.to_string(), "1");
    /// ```
    #[inline]
    fn mod_compose_assign(&mut self, other: &Self, m: T) {
        assert_reduced(self, other, m);
        self.coefficients = mod_compose_vec(take(&mut self.coefficients), &other.coefficients, m);
    }
}
