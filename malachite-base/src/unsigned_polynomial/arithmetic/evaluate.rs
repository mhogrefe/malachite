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
use crate::polynomial::{EvaluateMod, EvaluateModPowerOf2};
use crate::unsigned_polynomial::UnsignedPolynomial;

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

// This is equivalent to `_nmod_poly_evaluate_nmod` from `nmod_poly/evaluate_nmod.c`, FLINT 3.6.0,
// except that rectangular splitting is not used.
crate_test_fn! {evaluate_mod_slice<T: PrimitiveUnsigned>(coefficients: &[T], x: T, m: T) -> T {
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
}}

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
