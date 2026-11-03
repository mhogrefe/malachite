// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::modular::mod_mul::{mod_mul_precompute_shoup, mod_mul_shoup, mod_mul_shoup_lazy};
use crate::num::modular::traits::ModIsReduced;
use crate::polynomial::modular::traits::ModEvaluate;
#[cfg(doc)]
use crate::polynomial::modular::traits::ModEvaluateMany;
use crate::unsigned_polynomial::UnsignedPolynomial;

// The shortest polynomial evaluated with Shoup's method, when `T` is wider than 32 bits: Shoup's
// precomputation is a two-by-one division, which is only paid back from this length on. For
// narrower types every polynomial of length 2 or more uses it. Tuned on Apple M-series for `u64`
// and `u128`; FLINT's `FLINT_MULMOD_SHOUP_THRESHOLD` is 10.
pub(crate) const MOD_EVALUATE_SHOUP_THRESHOLD: usize = 3;

// Evaluates a polynomial at `x` modulo `m` with Horner's rule, reducing after every step.
// `coefficients` must be nonempty.
//
// This is equivalent to `_nmod_poly_evaluate_nmod_horner` from `nmod_poly/evaluate_nmod.c`, FLINT
// 3.6.0.
crate_test_fn! {mod_evaluate_horner<T: PrimitiveUnsigned>(coefficients: &[T], x: T, m: T) -> T {
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
crate_test_fn! {mod_evaluate_shoup<T: PrimitiveUnsigned>(
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

// Evaluates a polynomial at `x` modulo `m` like `mod_evaluate_shoup`, but reduces only partially:
// the result is congruent to the polynomial's value and less than $3m - 1$. `coefficients` must be
// nonempty, `x_precomp` must be `mod_mul_precompute_shoup(x, m)`, and `m` must be at most `T::MAX /
// 3`, so that $3m - 1$ values fit.
//
// This is equivalent to `_nmod_poly_evaluate_nmod_precomp_lazy` from `nmod_poly/evaluate_nmod.c`,
// FLINT 3.6.0.
crate_test_fn! {mod_evaluate_shoup_lazy<T: PrimitiveUnsigned>(
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
pub fn mod_evaluate_slice<T: PrimitiveUnsigned>(coefficients: &[T], x: T, m: T) -> T {
    let len = coefficients.len();
    if len == 0 {
        return T::ZERO;
    }
    if len == 1 || x == T::ZERO {
        return coefficients[0];
    }
    // Shoup's method needs the top bit of m clear
    if m.get_highest_bit() || (T::WIDTH > u32::WIDTH && len < MOD_EVALUATE_SHOUP_THRESHOLD) {
        return mod_evaluate_horner(coefficients, x, m);
    }
    let x_precomp = mod_mul_precompute_shoup(x, m);
    // The lazy loop's values are less than 3m - 1, so it is used when those fit: when m <= (2^W +
    // 1) / 3, which is T::MAX / 3 since every width W is even. FLINT calls this bound LAZY_MAX.
    if m <= T::MAX / T::from(3u8) {
        let mut value = mod_evaluate_shoup_lazy(coefficients, x, x_precomp, m);
        // correct the excess
        let two_m = m << 1;
        if value >= two_m {
            value -= two_m;
        } else if value >= m {
            value -= m;
        }
        value
    } else {
        mod_evaluate_shoup(coefficients, x, x_precomp, m)
    }
}

fn mod_evaluate<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, x: T, m: T) -> T {
    assert!(
        p.mod_is_reduced(&m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
    mod_evaluate_slice(&p.coefficients, x, m)
}

impl<T: PrimitiveUnsigned> ModEvaluate<T> for &UnsignedPolynomial<T> {
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
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 10 mod 13.
    /// assert_eq!((&p).mod_evaluate(6, 13), 10);
    /// assert_eq!((&p).mod_evaluate(0, 13), 7);
    /// // 205 itself, modulo a larger modulus.
    /// assert_eq!((&p).mod_evaluate(6, 211), 205);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_evaluate_nmod` from `nmod_poly/evaluate_nmod.c`, FLINT
    /// 3.6.0, except that the value must be reduced.
    #[inline]
    fn mod_evaluate(self, x: T, m: T) -> T {
        mod_evaluate(self, x, m)
    }
}

impl<T: PrimitiveUnsigned> ModEvaluate<T> for UnsignedPolynomial<T> {
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
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 10 mod 13.
    /// assert_eq!(p.clone().mod_evaluate(6, 13), 10);
    /// assert_eq!(p.clone().mod_evaluate(0, 13), 7);
    /// // 205 itself, modulo a larger modulus.
    /// assert_eq!(p.mod_evaluate(6, 211), 205);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_evaluate_nmod` from `nmod_poly/evaluate_nmod.c`, FLINT
    /// 3.6.0, except that the value must be reduced.
    #[inline]
    fn mod_evaluate(self, x: T, m: T) -> T {
        mod_evaluate(&self, x, m)
    }
}
