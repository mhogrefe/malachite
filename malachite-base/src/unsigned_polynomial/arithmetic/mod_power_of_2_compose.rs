// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{ModPowerOf2Compose, ModPowerOf2ComposeAssign, ModPowerOf2Evaluate};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_add::assert_reduced;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::{
    add_wrapping_assign, from_coefficients_trimmed, mask_coefficients, mul_wrapping,
};
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_square::square_wrapping;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_taylor_shift::*;
use alloc::vec;
use alloc::vec::Vec;
use core::mem::{swap, take};

// Whether a polynomial with `xs_len` coefficients is composed with a polynomial with `ys_len`
// coefficients by Horner's rule rather than divide and conquer, as measured on this machine. In
// wrapping word arithmetic every product is cheap, and short ones are schoolbook products, so
// divide and conquer only pays when it saves many of them: from about 24 coefficients when the
// inner polynomial is short (more for 128-bit words), and from about 12 when it is longer.
const fn compose_horner_preferred<T: PrimitiveUnsigned>(xs_len: usize, ys_len: usize) -> bool {
    let max_len = if ys_len > 4 {
        11
    } else if T::WIDTH == 128 {
        31
    } else {
        23
    };
    xs_len <= max_len
}

// The coefficients of the composition of the polynomials with coefficients `xs` and `ys`, both
// nonempty, correct modulo $2^k$, where $k$ is `pow`, by Horner's rule over whole polynomials:
// $((x_{n-1} y + x_{n-2}) y + \cdots) y + x_0$. The result is neither reduced nor trimmed.
crate_test_fn! {mod_power_of_2_compose_horner<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    pow: u64,
) -> Vec<T> {
    let (&leading, rest) = xs.split_last().unwrap();
    let Some((&second, rest)) = rest.split_last() else {
        return vec![leading];
    };
    let step = ys.len() - 1;
    let mut acc: Vec<T> = ys.iter().map(|&y| y.wrapping_mul(leading)).collect();
    acc[0].wrapping_add_assign(second);
    let mut scratch = Vec::with_capacity((xs.len() - 1) * step + 1);
    for &x in rest.iter().rev() {
        // The product overwrites every element, so stale ones may stay.
        scratch.resize(acc.len() + step, T::ZERO);
        mul_wrapping(&mut scratch, &acc, ys, pow);
        scratch[0].wrapping_add_assign(x);
        swap(&mut acc, &mut scratch);
    }
    acc
}}

// Adds `ys` into `xs`, wrapping and reusing whichever of the two is longer.
fn add_wrapping_assign_val<T: PrimitiveUnsigned>(xs: &mut Vec<T>, mut ys: Vec<T>) {
    if ys.len() > xs.len() {
        swap(xs, &mut ys);
    }
    add_wrapping_assign(xs, &ys);
}

// The coefficients of the composition of the polynomials with coefficients `xs` and `ys`, both
// nonempty, correct modulo $2^k$, where $k$ is `pow`, by divide and conquer: each coefficient of
// `xs` starts as a constant group; at every level, neighboring groups $g_{2i}$ and $g_{2i+1}$ merge
// into $g_{2i} + y^{2^j} g_{2i+1}$, and the power of `ys` is squared, until one group is left. Zero
// groups are kept empty. The result is neither reduced nor trimmed.
crate_test_fn! {mod_power_of_2_compose_divide_and_conquer<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    pow: u64,
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
                mul_wrapping(&mut product, &power, &hi, pow);
                add_wrapping_assign_val(&mut lo, product);
            }
            merged.push(lo);
        }
        groups = merged;
        if groups.len() == 1 {
            break;
        }
        let mut square = vec![T::ZERO; (power.len() << 1) - 1];
        square_wrapping(&mut square, &power, pow);
        power = square;
    }
    groups.pop().unwrap()
}}

// Composes the polynomial with coefficients `xs`, of length at least 2, with $bx + a$ modulo $2^k$,
// where `ys` is `[a, b]`, in place: a Taylor shift by $a$, after which the coefficient of $x^i$ is
// multiplied by $b^i$. The result is reduced but not trimmed.
fn mod_power_of_2_compose_linear_in_place<T: PrimitiveUnsigned>(xs: &mut [T], ys: &[T], pow: u64) {
    let (a, b) = (ys[0], ys[1]);
    if a != T::ZERO {
        mod_power_of_2_taylor_shift_divide_and_conquer(xs, a, pow);
    }
    if b != T::ONE {
        let mut power = b;
        for x in &mut xs[1..] {
            x.wrapping_mul_assign(power);
            power.wrapping_mul_assign(b);
        }
        mask_coefficients(xs, pow);
    }
}

/// The coefficients of the composition, modulo $2^k$, of the polynomials with coefficients `xs` and
/// `ys`, both reduced modulo $2^k$ and with no zeros at the end, trimmed. `pow` must be no greater
/// than `T::WIDTH`.
///
/// This is not part of the public API; it is public so that `malachite-nz` can compose
/// `NaturalPolynomial`s with word-sized coefficients modulo $2^k$.
#[doc(hidden)]
pub fn mod_power_of_2_compose_vec<T: PrimitiveUnsigned>(
    mut xs: Vec<T>,
    ys: &[T],
    pow: u64,
) -> Vec<T> {
    let coefficients = match (xs.as_slice(), ys) {
        ([] | [_], _) => return xs,
        ([x, ..], []) => vec![*x],
        (_, [y]) => vec![UnsignedPolynomial { coefficients: xs }.mod_power_of_2_evaluate(*y, pow)],
        (_, [_, _]) => {
            mod_power_of_2_compose_linear_in_place(&mut xs, ys, pow);
            xs
        }
        _ => {
            let mut out = if compose_horner_preferred::<T>(xs.len(), ys.len()) {
                mod_power_of_2_compose_horner(&xs, ys, pow)
            } else {
                mod_power_of_2_compose_divide_and_conquer(&xs, ys, pow)
            };
            mask_coefficients(&mut out, pow);
            out
        }
    };
    from_coefficients_trimmed(coefficients).coefficients
}

impl<T: PrimitiveUnsigned> ModPowerOf2Compose<Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Composes two [`UnsignedPolynomial`]s modulo $2^k$, giving $p(q(x)) \\bmod 2^k$, taking both
    /// by value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = p(q(x)) \\bmod 2^k.
    /// $$
    ///
    /// The arithmetic is done in wrapping word arithmetic and reduced modulo $2^k$ at the end,
    /// which gives the right result because $2^k$ divides $2^W$. When $q$ is a constant $c$, the
    /// result is the constant $p(c)$; when $q$ is linear, $bx + a$, it is a Taylor shift of $p$ by
    /// $a$, after which the coefficient of $x^i$ is multiplied by $b^i$; otherwise short
    /// polynomials are composed with Horner's rule and longer ones by divide and conquer, which
    /// merges neighboring groups of coefficients with ever higher powers $q^{2^j}$. The leading
    /// coefficient of the result can vanish modulo $2^k$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log^2 (nm))$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is `other.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2Compose;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_compose(UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 4)
    ///         .to_string(),
    ///     "4*x^2+10*x+10"
    /// );
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^3")
    ///         .unwrap()
    ///         .mod_power_of_2_compose(UnsignedPolynomial::<u8>::from_str("x^2+x").unwrap(), 3)
    ///         .to_string(),
    ///     "x^6+3*x^5+3*x^4+x^3"
    /// );
    /// // Modulo 8, the leading coefficient 8 vanishes.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("2*x^2+1")
    ///         .unwrap()
    ///         .mod_power_of_2_compose(UnsignedPolynomial::<u8>::from_str("2*x").unwrap(), 3)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_compose(self, other: Self, pow: u64) -> Self {
        assert_reduced(&self, &other, pow);
        Self {
            coefficients: mod_power_of_2_compose_vec(self.coefficients, &other.coefficients, pow),
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Compose<&Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Composes two [`UnsignedPolynomial`]s modulo $2^k$, giving $p(q(x)) \\bmod 2^k$, taking the
    /// first by value and the second by reference. The coefficients of both must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = p(q(x)) \\bmod 2^k.
    /// $$
    ///
    /// The arithmetic is done in wrapping word arithmetic and reduced modulo $2^k$ at the end,
    /// which gives the right result because $2^k$ divides $2^W$. When $q$ is a constant $c$, the
    /// result is the constant $p(c)$; when $q$ is linear, $bx + a$, it is a Taylor shift of $p$ by
    /// $a$, after which the coefficient of $x^i$ is multiplied by $b^i$; otherwise short
    /// polynomials are composed with Horner's rule and longer ones by divide and conquer, which
    /// merges neighboring groups of coefficients with ever higher powers $q^{2^j}$. The leading
    /// coefficient of the result can vanish modulo $2^k$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log^2 (nm))$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is `other.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2Compose;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_compose(&UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 4)
    ///         .to_string(),
    ///     "4*x^2+10*x+10"
    /// );
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^3")
    ///         .unwrap()
    ///         .mod_power_of_2_compose(&UnsignedPolynomial::<u8>::from_str("x^2+x").unwrap(), 3)
    ///         .to_string(),
    ///     "x^6+3*x^5+3*x^4+x^3"
    /// );
    /// // Modulo 8, the leading coefficient 8 vanishes.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("2*x^2+1")
    ///         .unwrap()
    ///         .mod_power_of_2_compose(&UnsignedPolynomial::<u8>::from_str("2*x").unwrap(), 3)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_compose(self, other: &Self, pow: u64) -> Self {
        assert_reduced(&self, other, pow);
        Self {
            coefficients: mod_power_of_2_compose_vec(self.coefficients, &other.coefficients, pow),
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Compose<UnsignedPolynomial<T>> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo $2^k$, giving $p(q(x)) \\bmod 2^k$, taking the
    /// first by reference and the second by value. The coefficients of both must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = p(q(x)) \\bmod 2^k.
    /// $$
    ///
    /// The arithmetic is done in wrapping word arithmetic and reduced modulo $2^k$ at the end,
    /// which gives the right result because $2^k$ divides $2^W$. When $q$ is a constant $c$, the
    /// result is the constant $p(c)$; when $q$ is linear, $bx + a$, it is a Taylor shift of $p$ by
    /// $a$, after which the coefficient of $x^i$ is multiplied by $b^i$; otherwise short
    /// polynomials are composed with Horner's rule and longer ones by divide and conquer, which
    /// merges neighboring groups of coefficients with ever higher powers $q^{2^j}$. The leading
    /// coefficient of the result can vanish modulo $2^k$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log^2 (nm))$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is `other.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2Compose;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_power_of_2_compose(UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 4)
    ///         .to_string(),
    ///     "4*x^2+10*x+10"
    /// );
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^3").unwrap())
    ///         .mod_power_of_2_compose(UnsignedPolynomial::<u8>::from_str("x^2+x").unwrap(), 3)
    ///         .to_string(),
    ///     "x^6+3*x^5+3*x^4+x^3"
    /// );
    /// // Modulo 8, the leading coefficient 8 vanishes.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("2*x^2+1").unwrap())
    ///         .mod_power_of_2_compose(UnsignedPolynomial::<u8>::from_str("2*x").unwrap(), 3)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_compose(
        self,
        other: UnsignedPolynomial<T>,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        assert_reduced(self, &other, pow);
        UnsignedPolynomial {
            coefficients: mod_power_of_2_compose_vec(
                self.coefficients.clone(),
                &other.coefficients,
                pow,
            ),
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Compose<&UnsignedPolynomial<T>> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo $2^k$, giving $p(q(x)) \\bmod 2^k$, taking both
    /// by reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = p(q(x)) \\bmod 2^k.
    /// $$
    ///
    /// The arithmetic is done in wrapping word arithmetic and reduced modulo $2^k$ at the end,
    /// which gives the right result because $2^k$ divides $2^W$. When $q$ is a constant $c$, the
    /// result is the constant $p(c)$; when $q$ is linear, $bx + a$, it is a Taylor shift of $p$ by
    /// $a$, after which the coefficient of $x^i$ is multiplied by $b^i$; otherwise short
    /// polynomials are composed with Horner's rule and longer ones by divide and conquer, which
    /// merges neighboring groups of coefficients with ever higher powers $q^{2^j}$. The leading
    /// coefficient of the result can vanish modulo $2^k$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log^2 (nm))$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is `other.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2Compose;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_power_of_2_compose(&UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 4)
    ///         .to_string(),
    ///     "4*x^2+10*x+10"
    /// );
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^3").unwrap())
    ///         .mod_power_of_2_compose(&UnsignedPolynomial::<u8>::from_str("x^2+x").unwrap(), 3)
    ///         .to_string(),
    ///     "x^6+3*x^5+3*x^4+x^3"
    /// );
    /// // Modulo 8, the leading coefficient 8 vanishes.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("2*x^2+1").unwrap())
    ///         .mod_power_of_2_compose(&UnsignedPolynomial::<u8>::from_str("2*x").unwrap(), 3)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_compose(
        self,
        other: &UnsignedPolynomial<T>,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        assert_reduced(self, other, pow);
        UnsignedPolynomial {
            coefficients: mod_power_of_2_compose_vec(
                self.coefficients.clone(),
                &other.coefficients,
                pow,
            ),
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeAssign<Self> for UnsignedPolynomial<T> {
    /// Composes an [`UnsignedPolynomial`] with another modulo $2^k$ in place, replacing $p$ with
    /// $p(q(x)) \\bmod 2^k$, taking the other by value. The coefficients of both must already be
    /// reduced modulo $2^k$.
    ///
    /// $$
    /// p \\gets p(q(x)) \\bmod 2^k.
    /// $$
    ///
    /// The arithmetic is done in wrapping word arithmetic and reduced modulo $2^k$ at the end,
    /// which gives the right result because $2^k$ divides $2^W$. When $q$ is a constant $c$, the
    /// result is the constant $p(c)$; when $q$ is linear, $bx + a$, it is a Taylor shift of $p$ by
    /// $a$, after which the coefficient of $x^i$ is multiplied by $b^i$; otherwise short
    /// polynomials are composed with Horner's rule and longer ones by divide and conquer, which
    /// merges neighboring groups of coefficients with ever higher powers $q^{2^j}$. The leading
    /// coefficient of the result can vanish modulo $2^k$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log^2 (nm))$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is `other.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2ComposeAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_compose_assign(UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 4);
    /// assert_eq!(p.to_string(), "4*x^2+10*x+10");
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3").unwrap();
    /// p.mod_power_of_2_compose_assign(UnsignedPolynomial::<u8>::from_str("x^2+x").unwrap(), 3);
    /// assert_eq!(p.to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Modulo 8, the leading coefficient 8 vanishes.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("2*x^2+1").unwrap();
    /// p.mod_power_of_2_compose_assign(UnsignedPolynomial::<u8>::from_str("2*x").unwrap(), 3);
    /// assert_eq!(p.to_string(), "1");
    /// ```
    #[inline]
    fn mod_power_of_2_compose_assign(&mut self, other: Self, pow: u64) {
        assert_reduced(self, &other, pow);
        self.coefficients =
            mod_power_of_2_compose_vec(take(&mut self.coefficients), &other.coefficients, pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeAssign<&Self> for UnsignedPolynomial<T> {
    /// Composes an [`UnsignedPolynomial`] with another modulo $2^k$ in place, replacing $p$ with
    /// $p(q(x)) \\bmod 2^k$, taking the other by reference. The coefficients of both must already
    /// be reduced modulo $2^k$.
    ///
    /// $$
    /// p \\gets p(q(x)) \\bmod 2^k.
    /// $$
    ///
    /// The arithmetic is done in wrapping word arithmetic and reduced modulo $2^k$ at the end,
    /// which gives the right result because $2^k$ divides $2^W$. When $q$ is a constant $c$, the
    /// result is the constant $p(c)$; when $q$ is linear, $bx + a$, it is a Taylor shift of $p$ by
    /// $a$, after which the coefficient of $x^i$ is multiplied by $b^i$; otherwise short
    /// polynomials are composed with Horner's rule and longer ones by divide and conquer, which
    /// merges neighboring groups of coefficients with ever higher powers $q^{2^j}$. The leading
    /// coefficient of the result can vanish modulo $2^k$, and then its degree is lower than the
    /// product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log^2 (nm))$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is `other.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2ComposeAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_compose_assign(&UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 4);
    /// assert_eq!(p.to_string(), "4*x^2+10*x+10");
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3").unwrap();
    /// p.mod_power_of_2_compose_assign(&UnsignedPolynomial::<u8>::from_str("x^2+x").unwrap(), 3);
    /// assert_eq!(p.to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Modulo 8, the leading coefficient 8 vanishes.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("2*x^2+1").unwrap();
    /// p.mod_power_of_2_compose_assign(&UnsignedPolynomial::<u8>::from_str("2*x").unwrap(), 3);
    /// assert_eq!(p.to_string(), "1");
    /// ```
    #[inline]
    fn mod_power_of_2_compose_assign(&mut self, other: &Self, pow: u64) {
        assert_reduced(self, other, pow);
        self.coefficients =
            mod_power_of_2_compose_vec(take(&mut self.coefficients), &other.coefficients, pow);
    }
}
