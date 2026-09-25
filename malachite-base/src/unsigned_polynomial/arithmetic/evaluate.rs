// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::mod_mul::{
    mod_mul_precompute_shoup, mod_mul_shoup, mod_mul_shoup_lazy,
};
use crate::num::arithmetic::traits::{ModIsReduced, ModPowerOf2IsReduced};
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::polynomial::{EvaluateGeometricMod, EvaluateManyMod, EvaluateMod, EvaluateModPowerOf2};
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec::Vec;

// Evaluates a polynomial at x modulo 2^pow, after checking that pow fits in `T` and that the
// coefficients and x are reduced.
fn evaluate_mod_power_of_2<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, x: T, pow: u64) -> T {
    assert!(pow <= T::WIDTH);
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    assert!(
        x.significant_bits() <= pow,
        "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
    let mut value = T::ZERO;
    for &c in p.coefficients.iter().rev() {
        value = value.wrapping_mul(x).wrapping_add(c);
    }
    value.mod_power_of_2(pow)
}

impl<T: PrimitiveUnsigned> EvaluateModPowerOf2<T> for &UnsignedPolynomial<T> {
    type Output = T;

    /// Evaluates an [`UnsignedPolynomial`] at a value of its coefficient type, modulo $2^k$, taking
    /// the polynomial by reference. The coefficients and the value must already be reduced modulo
    /// $2^k$, and $k$ may be at most the width of the type.
    ///
    /// $$
    /// f(p, x, k) = \sum_{i=0}^{n-1} c_i x^i \bmod 2^k,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Reducing modulo $2^k$ commutes with wrapping arithmetic, which works modulo $2^w$ for the
    /// type's width $w \geq k$, so Horner's rule is carried out with wrapping multiplications and
    /// additions and the value is reduced once, at the end.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `x` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::EvaluateModPowerOf2;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 13 mod 16.
    /// assert_eq!((&p).evaluate_mod_power_of_2(6, 4), 13);
    /// assert_eq!((&p).evaluate_mod_power_of_2(0, 4), 7);
    /// // All 8 bits of a u8: 205 itself.
    /// assert_eq!((&p).evaluate_mod_power_of_2(6, 8), 205);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_evaluate_nmod` from `nmod_poly/evaluate_nmod.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that the value must be reduced.
    #[inline]
    fn evaluate_mod_power_of_2(self, x: T, pow: u64) -> T {
        evaluate_mod_power_of_2(self, x, pow)
    }
}

impl<T: PrimitiveUnsigned> EvaluateModPowerOf2<T> for UnsignedPolynomial<T> {
    type Output = T;

    /// Evaluates an [`UnsignedPolynomial`] at a value of its coefficient type, modulo $2^k$, taking
    /// the polynomial by value. The coefficients and the value must already be reduced modulo
    /// $2^k$, and $k$ may be at most the width of the type.
    ///
    /// $$
    /// f(p, x, k) = \sum_{i=0}^{n-1} c_i x^i \bmod 2^k,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Reducing modulo $2^k$ commutes with wrapping arithmetic, which works modulo $2^w$ for the
    /// type's width $w \geq k$, so Horner's rule is carried out with wrapping multiplications and
    /// additions and the value is reduced once, at the end.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `x` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::EvaluateModPowerOf2;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 13 mod 16.
    /// assert_eq!(p.clone().evaluate_mod_power_of_2(6, 4), 13);
    /// assert_eq!(p.clone().evaluate_mod_power_of_2(0, 4), 7);
    /// // All 8 bits of a u8: 205 itself.
    /// assert_eq!(p.evaluate_mod_power_of_2(6, 8), 205);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_evaluate_nmod` from `nmod_poly/evaluate_nmod.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that the value must be reduced.
    #[inline]
    fn evaluate_mod_power_of_2(self, x: T, pow: u64) -> T {
        evaluate_mod_power_of_2(&self, x, pow)
    }
}

// The shortest polynomial evaluated with Shoup's method, when `T` is wider than 32 bits: Shoup's
// precomputation is a two-by-one division, which is only paid back from this length on. For
// narrower types every polynomial of length 2 or more uses it. Tuned on Apple M-series for `u64`
// and `u128`; FLINT's `FLINT_MULMOD_SHOUP_THRESHOLD` is 10.
const EVALUATE_MOD_SHOUP_THRESHOLD: usize = 3;

// Evaluates a polynomial at `x` modulo `m` with Horner's rule, reducing after every step.
// `coefficients` must be nonempty.
//
// This is equivalent to `_nmod_poly_evaluate_nmod_horner` from `nmod_poly/evaluate_nmod.c`, FLINT
// 3.6.0.
crate_test_fn! {evaluate_mod_horner<T: PrimitiveUnsigned>(coefficients: &[T], x: T, m: T) -> T {
    let data = T::precompute_mod_mul_data(&m);
    let (&last, rest) = coefficients.split_last().unwrap();
    let mut value = last;
    for &c in rest.iter().rev() {
        value.mod_mul_precomputed_assign(x, m, &data);
        value.mod_add_assign(c, m);
    }
    value
}}

// Evaluates a polynomial at `x` modulo `m` with Horner's rule, multiplying by `x` with Shoup's
// method. `coefficients` must be nonempty, `x_precomp` must be `mod_mul_precompute_shoup(x, m)`,
// and the top bit of `m` must be clear.
//
// This is equivalent to `_nmod_poly_evaluate_nmod_precomp` from `nmod_poly/evaluate_nmod.c`, FLINT
// 3.6.0.
crate_test_fn! {evaluate_mod_shoup<T: PrimitiveUnsigned>(
    coefficients: &[T],
    x: T,
    x_precomp: T,
    m: T,
) -> T {
    let (&last, rest) = coefficients.split_last().unwrap();
    let mut value = last;
    for &c in rest.iter().rev() {
        value = mod_mul_shoup(x, value, x_precomp, m);
        value.mod_add_assign(c, m);
    }
    value
}}

// Evaluates a polynomial at `x` modulo `m` like `evaluate_mod_shoup`, but reduces only partially:
// the result is congruent to the polynomial's value and less than $3m - 1$. `coefficients` must be
// nonempty, `x_precomp` must be `mod_mul_precompute_shoup(x, m)`, and `m` must be at most `T::MAX /
// 3`, so that $3m - 1$ values fit.
//
// This is equivalent to `_nmod_poly_evaluate_nmod_precomp_lazy` from `nmod_poly/evaluate_nmod.c`,
// FLINT 3.6.0.
crate_test_fn! {evaluate_mod_shoup_lazy<T: PrimitiveUnsigned>(
    coefficients: &[T],
    x: T,
    x_precomp: T,
    m: T,
) -> T {
    let (&last, rest) = coefficients.split_last().unwrap();
    let mut value = last;
    for &c in rest.iter().rev() {
        // value is x * value mod m, or that plus m
        value = mod_mul_shoup_lazy(x, value, x_precomp, m);
        // value is now less than 3m - 1, since c < m
        value.wrapping_add_assign(c);
    }
    value
}}

// Evaluates the polynomial with the given coefficients at `x` modulo `m`. The coefficients and `x`
// must be less than `m`; this is not checked. It is public, but hidden, because malachite-nz
// evaluates an `IntegerPolynomial` modulo a word with it, after reducing the coefficients.
//
// This is equivalent to `_nmod_poly_evaluate_nmod` from `nmod_poly/evaluate_nmod.c`, FLINT 3.6.0,
// except that rectangular splitting is not used.
#[doc(hidden)]
pub fn evaluate_mod_slice<T: PrimitiveUnsigned>(coefficients: &[T], x: T, m: T) -> T {
    let len = coefficients.len();
    if len == 0 {
        return T::ZERO;
    }
    if len == 1 || x == T::ZERO {
        return coefficients[0];
    }
    // Shoup's method needs the top bit of m clear
    if m.get_highest_bit() || (T::WIDTH > u32::WIDTH && len < EVALUATE_MOD_SHOUP_THRESHOLD) {
        return evaluate_mod_horner(coefficients, x, m);
    }
    let x_precomp = mod_mul_precompute_shoup(x, m);
    // The lazy loop's values are less than 3m - 1, so it is used when those fit: when m <= (2^W +
    // 1) / 3, which is T::MAX / 3 since every width W is even. FLINT calls this bound LAZY_MAX.
    if m <= T::MAX / T::from(3u8) {
        let mut value = evaluate_mod_shoup_lazy(coefficients, x, x_precomp, m);
        // correct the excess
        let two_m = m << 1;
        if value >= two_m {
            value -= two_m;
        } else if value >= m {
            value -= m;
        }
        value
    } else {
        evaluate_mod_shoup(coefficients, x, x_precomp, m)
    }
}

fn evaluate_mod<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, x: T, m: T) -> T {
    assert!(
        p.mod_is_reduced(&m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
    evaluate_mod_slice(&p.coefficients, x, m)
}

impl<T: PrimitiveUnsigned> EvaluateMod<T> for &UnsignedPolynomial<T> {
    type Output = T;

    /// Evaluates an [`UnsignedPolynomial`] at a value of its coefficient type, modulo a value of
    /// that type, taking the polynomial by reference. The coefficients and the value must already
    /// be reduced modulo `m`.
    ///
    /// $$
    /// f(p, x, m) = \sum_{i=0}^{n-1} c_i x^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// The value is found with Horner's rule, reducing after every step. When the polynomial is
    /// long enough and the top bit of `m` is clear, every multiplication is by the same `x`, so
    /// Shoup's method is used: $\lfloor x 2^W / m \rfloor$, where $W$ is the width of `T`, is
    /// computed once, and each product is then reduced with a multiplication in place of a
    /// division. When $m \leq (2^W - 1) / 3$ the reductions are also lazy, leaving the value below
    /// $3m - 1$ until the end.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `x` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::EvaluateMod;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 10 mod 13.
    /// assert_eq!((&p).evaluate_mod(6, 13), 10);
    /// assert_eq!((&p).evaluate_mod(0, 13), 7);
    /// // 205 itself, modulo a larger modulus.
    /// assert_eq!((&p).evaluate_mod(6, 211), 205);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_evaluate_nmod` from `nmod_poly/evaluate_nmod.c`, FLINT
    /// 3.6.0, except that the value must be reduced.
    #[inline]
    fn evaluate_mod(self, x: T, m: T) -> T {
        evaluate_mod(self, x, m)
    }
}

impl<T: PrimitiveUnsigned> EvaluateMod<T> for UnsignedPolynomial<T> {
    type Output = T;

    /// Evaluates an [`UnsignedPolynomial`] at a value of its coefficient type, modulo a value of
    /// that type, taking the polynomial by value. The coefficients and the value must already be
    /// reduced modulo `m`.
    ///
    /// $$
    /// f(p, x, m) = \sum_{i=0}^{n-1} c_i x^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// The value is found with Horner's rule, reducing after every step. When the polynomial is
    /// long enough and the top bit of `m` is clear, every multiplication is by the same `x`, so
    /// Shoup's method is used: $\lfloor x 2^W / m \rfloor$, where $W$ is the width of `T`, is
    /// computed once, and each product is then reduced with a multiplication in place of a
    /// division. When $m \leq (2^W - 1) / 3$ the reductions are also lazy, leaving the value below
    /// $3m - 1$ until the end.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `x` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::EvaluateMod;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 10 mod 13.
    /// assert_eq!(p.clone().evaluate_mod(6, 13), 10);
    /// assert_eq!(p.clone().evaluate_mod(0, 13), 7);
    /// // 205 itself, modulo a larger modulus.
    /// assert_eq!(p.evaluate_mod(6, 211), 205);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_evaluate_nmod` from `nmod_poly/evaluate_nmod.c`, FLINT
    /// 3.6.0, except that the value must be reduced.
    #[inline]
    fn evaluate_mod(self, x: T, m: T) -> T {
        evaluate_mod(&self, x, m)
    }
}

// The numbers of points evaluated together by `evaluate_many_mod_in_place`, for types at most 32
// bits wide and for wider types. Horner's rule is a chain of dependent multiplications, so running
// several points through one pass over the coefficients keeps more multiplications in flight. Tuned
// on Apple M-series: for `u64` a block of 8 is 3.4 to 4.8 times as fast as one point at a time at
// length 256, and for `u32` a block of 4 is best.
const EVALUATE_MANY_MOD_NARROW_BLOCK: usize = 4;
const EVALUATE_MANY_MOD_WIDE_BLOCK: usize = 8;

// Replaces each of the `N` points in `xs` with the polynomial's value there modulo `m`, with
// Horner's rule for all of them in one pass over the coefficients. `coefficients` must be nonempty,
// and the points and coefficients reduced.
crate_test_fn! {evaluate_mod_horner_block<T: PrimitiveUnsigned, const N: usize>(
    coefficients: &[T],
    xs: &mut [T; N],
    m: T,
) {
    let data = T::precompute_mod_mul_data(&m);
    let (&last, rest) = coefficients.split_last().unwrap();
    let points = *xs;
    let mut values = [last; N];
    for &c in rest.iter().rev() {
        for (value, &x) in values.iter_mut().zip(points.iter()) {
            value.mod_mul_precomputed_assign(x, m, &data);
            value.mod_add_assign(c, m);
        }
    }
    *xs = values;
}}

// Like `evaluate_mod_horner_block`, multiplying by each point with Shoup's method. The top bit of
// `m` must be clear.
crate_test_fn! {evaluate_mod_shoup_block<T: PrimitiveUnsigned, const N: usize>(
    coefficients: &[T],
    xs: &mut [T; N],
    m: T,
) {
    let points = *xs;
    let precomps = points.map(|x| mod_mul_precompute_shoup(x, m));
    let (&last, rest) = coefficients.split_last().unwrap();
    let mut values = [last; N];
    for &c in rest.iter().rev() {
        for ((value, &x), &x_precomp) in values.iter_mut().zip(points.iter()).zip(precomps.iter()) {
            *value = mod_mul_shoup(x, *value, x_precomp, m);
            value.mod_add_assign(c, m);
        }
    }
    *xs = values;
}}

// Like `evaluate_mod_shoup_block`, with lazy reduction as in `evaluate_mod_shoup_lazy`. `m` must be
// at most `T::MAX / 3`. The values are fully reduced at the end.
crate_test_fn! {evaluate_mod_shoup_lazy_block<T: PrimitiveUnsigned, const N: usize>(
    coefficients: &[T],
    xs: &mut [T; N],
    m: T,
) {
    let points = *xs;
    let precomps = points.map(|x| mod_mul_precompute_shoup(x, m));
    let (&last, rest) = coefficients.split_last().unwrap();
    let mut values = [last; N];
    for &c in rest.iter().rev() {
        for ((value, &x), &x_precomp) in values.iter_mut().zip(points.iter()).zip(precomps.iter()) {
            // value is x * value mod m, or that plus m, and then less than 3m - 1
            *value = mod_mul_shoup_lazy(x, *value, x_precomp, m);
            value.wrapping_add_assign(c);
        }
    }
    let two_m = m << 1;
    for value in &mut values {
        if *value >= two_m {
            *value -= two_m;
        } else if *value >= m {
            *value -= m;
        }
    }
    *xs = values;
}}

// Replaces each point in `xs` with the polynomial's value there modulo `m`. The points and the
// coefficients must be reduced; this is not checked. Blocks of points are evaluated together, with
// the method `evaluate_mod_slice` would choose for one point. Evaluates the points in `xs` in
// blocks of `N`, with the method chosen by `evaluate_many_mod_in_place`, and returns the leftover
// points, fewer than `N` of them.
fn evaluate_many_mod_blocks<'a, T: PrimitiveUnsigned, const N: usize>(
    coefficients: &[T],
    xs: &'a mut [T],
    m: T,
    shoup: bool,
    lazy: bool,
) -> &'a mut [T] {
    let (blocks, remainder) = xs.as_chunks_mut::<N>();
    for block in blocks {
        if !shoup {
            evaluate_mod_horner_block(coefficients, block, m);
        } else if lazy {
            evaluate_mod_shoup_lazy_block(coefficients, block, m);
        } else {
            evaluate_mod_shoup_block(coefficients, block, m);
        }
    }
    remainder
}

// Replaces each point in `xs` with the polynomial's value there modulo `m`. The points and the
// coefficients must be reduced; this is not checked. Blocks of points are evaluated together, with
// the method `evaluate_mod_slice` would choose for one point.
crate_test_fn! {evaluate_many_mod_in_place<T: PrimitiveUnsigned>(
    coefficients: &[T],
    xs: &mut [T],
    m: T,
) {
    let len = coefficients.len();
    match len {
        0 => xs.fill(T::ZERO),
        1 => xs.fill(coefficients[0]),
        _ => {
            // As in evaluate_mod_slice: Shoup's method needs the top bit of m clear, and is used
            // for short polynomials only when T is at most 32 bits wide
            let shoup = !m.get_highest_bit()
                && (T::WIDTH <= u32::WIDTH || len >= EVALUATE_MOD_SHOUP_THRESHOLD);
            let lazy = m <= T::MAX / T::from(3u8);
            // Wider types take blocks of the wide size first; the leftover points go through blocks
            // of the narrow size, and then one at a time
            let xs = if T::WIDTH <= u32::WIDTH {
                xs
            } else {
                evaluate_many_mod_blocks::<T, EVALUATE_MANY_MOD_WIDE_BLOCK>(
                    coefficients,
                    xs,
                    m,
                    shoup,
                    lazy,
                )
            };
            let remainder = evaluate_many_mod_blocks::<T, EVALUATE_MANY_MOD_NARROW_BLOCK>(
                coefficients,
                xs,
                m,
                shoup,
                lazy,
            );
            for x in remainder {
                *x = evaluate_mod_slice(coefficients, *x, m);
            }
        }
    }
}}

impl<T: PrimitiveUnsigned> EvaluateManyMod<T> for &UnsignedPolynomial<T> {
    type Output = T;

    /// Evaluates an [`UnsignedPolynomial`] at each of several values of its coefficient type,
    /// modulo a value of that type. The coefficients and the values must already be reduced modulo
    /// `m`.
    ///
    /// $$
    /// f(p, (x_j)_{j=0}^{k-1}, m) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \bmod m
    /// \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    ///
    /// The result is the same as calling
    /// [`evaluate_mod`](crate::polynomial::EvaluateMod::evaluate_mod) at each value, with the same
    /// choice between Horner's rule and Shoup's method, but the polynomial is checked once, and
    /// several values are evaluated together in each pass over the coefficients. Horner's rule is a
    /// chain of dependent multiplications, so interleaving independent chains keeps the processor's
    /// multipliers busy.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk)$
    ///
    /// $M(k) = O(k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `xs.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or any value in `xs` is greater than or
    /// equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::EvaluateManyMod;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // 7, 15, 33, 61, 99, and 147, mod 13
    /// assert_eq!(
    ///     (&p).evaluate_many_mod(&[0, 1, 2, 3, 4, 5], 13),
    ///     &[7, 2, 7, 9, 8, 4]
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_evaluate_nmod_vec_iter` from
    /// `nmod_poly/evaluate_nmod_vec.c`, FLINT 3.6.0, except that the values must be reduced.
    fn evaluate_many_mod(self, xs: &[T], m: T) -> Vec<T> {
        assert!(
            self.mod_is_reduced(&m),
            "self must be reduced mod m, but {self} has a coefficient >= {m}"
        );
        for &x in xs {
            assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
        }
        let mut values = xs.to_vec();
        evaluate_many_mod_in_place(&self.coefficients, &mut values, m);
        values
    }
}

impl<T: PrimitiveUnsigned> EvaluateGeometricMod<T> for &UnsignedPolynomial<T> {
    type Output = T;

    /// Evaluates an [`UnsignedPolynomial`] at $1, q, q^2, \ldots, q^{k-1}$, modulo a value of its
    /// coefficient type. The coefficients and `q` must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, k, m) = \left ( \sum_{i=0}^{n-1} c_i q^{ij} \bmod m \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    ///
    /// The powers of `q` are computed with Shoup's method when the top bit of `m` is clear, since
    /// every multiplication is by `q`, and are then evaluated as by
    /// [`evaluate_many_mod`](crate::polynomial::EvaluateManyMod::evaluate_many_mod), in place.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk)$
    ///
    /// $M(k) = O(k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `k`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `q` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::EvaluateGeometricMod;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // At 1, 2, 4, and 8: 15, 33, 99, and 351, mod 13
    /// assert_eq!((&p).evaluate_geometric_mod(2, 4, 13), &[2, 7, 8, 0]);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_evaluate_geometric_nmod_vec_iter` from
    /// `nmod_poly/evaluate_geometric_nmod_vec.c`, FLINT 3.6.0, with `q` in place of FLINT's $r^2$:
    /// FLINT evaluates at the powers of the square of its argument.
    fn evaluate_geometric_mod(self, q: T, k: u64, m: T) -> Vec<T> {
        assert!(
            self.mod_is_reduced(&m),
            "self must be reduced mod m, but {self} has a coefficient >= {m}"
        );
        assert!(q < m, "q must be reduced mod m, but {q} >= {m}");
        let k = usize::exact_from(k);
        let mut values = Vec::with_capacity(k);
        if k != 0 {
            let mut power = T::ONE % m;
            values.push(power);
            if m.get_highest_bit() {
                let data = T::precompute_mod_mul_data(&m);
                for _ in 1..k {
                    power.mod_mul_precomputed_assign(q, m, &data);
                    values.push(power);
                }
            } else {
                let q_precomp = mod_mul_precompute_shoup(q, m);
                for _ in 1..k {
                    power = mod_mul_shoup(q, power, q_precomp, m);
                    values.push(power);
                }
            }
        }
        evaluate_many_mod_in_place(&self.coefficients, &mut values, m);
        values
    }
}
