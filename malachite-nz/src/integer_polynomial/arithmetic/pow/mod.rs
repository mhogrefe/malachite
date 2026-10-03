// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use crate::integer_polynomial::arithmetic::pow::addchains::{
    addition_chain, addition_chain_is_shorter, pow_to_out_addchains,
};
use crate::integer_polynomial::arithmetic::pow::binexp::pow_to_out_binexp;
use crate::integer_polynomial::arithmetic::pow::binomial::pow_to_out_binomial;
use crate::integer_polynomial::arithmetic::pow::multinomial::pow_to_out_multinomial;
use crate::integer_polynomial::arithmetic::pow::small::pow_to_out_small;
use crate::integer_polynomial::arithmetic::square::square_to_out;
use crate::integer_polynomial::arithmetic::vec::max_bits::vec_max_bits;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{Pow, PowAssign};
use malachite_base::num::conversion::traits::ExactFrom;

pub mod addchains;
pub mod binexp;
pub mod binomial;
pub mod multinomial;
pub mod small;

// Whether the multinomial recurrence is expected to beat repeated multiplication for a polynomial
// of length `len`, at least 3, whose largest coefficient has `bits` significant bits, raised to the
// power `e`. The recurrence costs about $e\,\ell^2$ multiplications by coefficients of the power,
// which repeated multiplication avoids by working on whole polynomials, so it pays off only when
// `e` is large relative to the length and the coefficients are small.
//
// The constants were measured on an Apple M-series machine with the tuner level `poly_pow_grid`.
// FLINT's criterion, that the number of limbs is less than $(3e/2 + 150)/\ell$, is far from the
// measured crossover here: it chooses the recurrence for every one-limb polynomial of length less
// than 100, where repeated multiplication is often many times faster.
pub(crate) fn multinomial_preferred(len: u64, bits: u64, e: u64) -> bool {
    len <= 16
        && len * bits <= 3072
        && e >= (len << 2) * (bits >> 8).max(1)
        && e.saturating_mul(bits) >= (len << 6).max(512)
}

// Whether the multinomial recurrence is expected to beat the binomial kernel for a polynomial of
// length 2 whose largest coefficient has `bits` significant bits, raised to the power `e`. Each
// step of the recurrence multiplies a coefficient of the power by a coefficient of the polynomial,
// while the binomial kernel multiplies coefficients of the power by powers of the polynomial's
// coefficients, which are as large; with small coefficients and large `e`, the recurrence wins.
pub(crate) fn multinomial_preferred_for_binomial(bits: u64, e: u64) -> bool {
    e >= 64 && (32..=1024).contains(&bits)
}

// Sets `out` to the coefficients of the `e`th power of the polynomial with coefficients `xs`, which
// has length at least 2 and nonzero first and last elements, where `e` is at least 3. `out` must
// have length `e * (xs.len() - 1) + 1`.
//
// Whether an addition chain is expected to beat binary exponentiation for a polynomial of length
// `len`, at least 3, whose largest coefficient has `bits` significant bits, raised to the power
// `e`, which is at least 5. A chain wins when it is shorter, and for $e \leq 7$ with coefficients
// of at least 32 bits, where it is as long but its multiplications are more balanced. Binary
// exponentiation's multiplications are all by the polynomial itself, which is cheap when the
// polynomial is short and its coefficients small, so short polynomials need larger coefficients for
// a chain to pay off.
pub(crate) fn addition_chain_preferred(len: u64, bits: u64, e: u64) -> bool {
    e <= 148
        && ((e <= 7 && bits >= 32) || addition_chain_is_shorter(e))
        && (len >= 8 || (len >= 4 && (bits >= 1024 || e <= 7)) || bits >= 4096)
}

// Below the fifth power, a square and a multiplication or two suffice. Otherwise the choice is
// between the binomial kernel, for length 2; the multinomial recurrence, for large exponents and
// small coefficients; addition chains; and binary exponentiation.
//
// This is equivalent to `_fmpz_poly_pow` from `fmpz_poly/pow.c`, FLINT 3.6.0, except for the
// criteria, which were measured, and except that FLINT never uses addition chains.
crate_test_fn! {pow_to_out<C: PolynomialCoefficient>(out: &mut [C], xs: &[C], e: u64) {
    let len = u64::exact_from(xs.len());
    if e < 5 {
        pow_to_out_small(out, xs, e);
        return;
    }
    let bits = vec_max_bits(xs).0;
    if len == 2 {
        if multinomial_preferred_for_binomial(bits, e) {
            pow_to_out_multinomial(out, xs, e);
        } else {
            pow_to_out_binomial(out, xs, e);
        }
    } else if multinomial_preferred(len, bits, e) {
        pow_to_out_multinomial(out, xs, e);
    } else if addition_chain_preferred(len, bits, e) {
        pow_to_out_addchains_e(out, xs, e);
    } else {
        pow_to_out_binexp(out, xs, e);
    }
}}

// Sets `out` to the coefficients of the `e`th power of the polynomial with coefficients `xs`, as
// `pow_to_out` does, using an addition chain for `e`, which must be at most 148.
//
// This is equivalent to the main case of `fmpz_poly_pow_addchains` from
// `fmpz_poly/pow_addchains.c`, FLINT 3.6.0.
crate_test_fn! {pow_to_out_addchains_e<C: PolynomialCoefficient>(out: &mut [C], xs: &[C], e: u64) {
    let (a, start) = addition_chain(e);
    pow_to_out_addchains(out, xs, &a[start..]);
}}

// The length of the coefficient vector of the `e`th power of a polynomial with nonzero constant
// term and length `len`, after a factor of $x^{e \ell}$, where $\ell$ is `low`.
fn power_len(len: usize, low: usize, e: u64) -> usize {
    let e = usize::exact_from(e);
    e.checked_mul(len - 1 + low)
        .and_then(|n| n.checked_add(1))
        .expect("the power has too many coefficients to represent")
}

// Returns the coefficients of the `e`th power of the polynomial with coefficients `xs`, which has
// no zeros at the end, using `pow_to_out_kernel` when the polynomial, once its factor of $x^\ell$
// is removed, has length at least 2, and when `e` is at least 3.
//
// Writing the polynomial as $x^\ell q$, with $q_0 \neq 0$, its power is $x^{e\ell} q^e$, so only
// $q$ is raised to the power `e` and the result is shifted. A kernel therefore always sees a
// nonzero constant term, and a monomial $c x^\ell$ reduces to the power of a single coefficient.
//
// This is equivalent to `fmpz_poly_pow` from `fmpz_poly/pow.c`, FLINT 3.6.0, and, with the
// corresponding kernels, to `fmpz_poly_pow_multinomial`, `fmpz_poly_pow_binomial`,
// `fmpz_poly_pow_binexp`, and `fmpz_poly_pow_addchains`, which share its special cases, except that
// FLINT does not remove the factor of $x^\ell$, apart from in the multinomial kernel, which
// requires a nonzero constant term.
crate_test_fn! {pow_ref_with_kernel<C: PolynomialCoefficient>(
    xs: &[C],
    e: u64,
    pow_to_out_kernel: fn(&mut [C], &[C], u64),
) -> Vec<C> {
    if e == 0 {
        return vec![C::ONE];
    }
    let Some(low) = xs.iter().position(|x| !x.is_zero()) else {
        return Vec::new();
    };
    let q = &xs[low..];
    let mut out = vec![C::ZERO; power_len(q.len(), low, e)];
    let out_q = &mut out[usize::exact_from(e) * low..];
    match (q.len(), e) {
        (1, _) => out_q[0] = q[0].pow_ref(e),
        (_, 1) => out_q.clone_from_slice(q),
        (_, 2) => square_to_out(out_q, q),
        _ => pow_to_out_kernel(out_q, q, e),
    }
    out
}}

// This is equivalent to `fmpz_poly_pow` from `fmpz_poly/pow.c`, FLINT 3.6.0, except as described
// for `pow_ref_with_kernel`.
#[inline]
pub(crate) fn pow_ref<C: PolynomialCoefficient>(xs: &[C], e: u64) -> Vec<C> {
    pow_ref_with_kernel(xs, e, pow_to_out)
}

// Replaces the coefficients `xs` of a polynomial with those of its `e`th power, reusing `xs` when
// the power of a constant is computed or nothing changes.
pub(crate) fn pow_assign_vec<C: PolynomialCoefficient + PowAssign<u64>>(xs: &mut Vec<C>, e: u64) {
    match (xs.len(), e) {
        (0, 0) => xs.push(C::ONE),
        (_, 0) => {
            xs.truncate(1);
            xs[0] = C::ONE;
        }
        (0, _) | (_, 1) => {}
        (1, _) => xs[0].pow_assign(e),
        _ => *xs = pow_ref(xs, e),
    }
}

impl Pow<u64> for IntegerPolynomial {
    type Output = Self;

    /// Raises an [`IntegerPolynomial`] to a power, taking it by value.
    ///
    /// $$
    /// f(p, e) = p^e.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1. Depending on the length of the
    /// polynomial, the size of its coefficients, and the exponent, the power is computed by the
    /// binomial theorem, by J. C. P. Miller's recurrence for the coefficients of a power, by an
    /// addition chain, or by repeated squaring.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, and $m$ is `exp` times the largest number of significant bits of any of its
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x+1")
    ///         .unwrap()
    ///         .pow(3)
    ///         .to_string(),
    ///     "x^3+3*x^2+3*x+1"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("2*x-1")
    ///         .unwrap()
    ///         .pow(4)
    ///         .to_string(),
    ///     "16*x^4-32*x^3+24*x^2-8*x+1"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^2-x")
    ///         .unwrap()
    ///         .pow(0)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pow` from `fmpz_poly/pow.c`, FLINT 3.6.0, except that a
    /// factor of $x^k$ is removed before powering, and that the algorithm is chosen by measured
    /// criteria, which include addition chains.
    #[inline]
    fn pow(mut self, exp: u64) -> Self {
        self.pow_assign(exp);
        self
    }
}

impl Pow<u64> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Raises an [`IntegerPolynomial`] to a power, taking it by reference.
    ///
    /// $$
    /// f(p, e) = p^e.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1. Depending on the length of the
    /// polynomial, the size of its coefficients, and the exponent, the power is computed by the
    /// binomial theorem, by J. C. P. Miller's recurrence for the coefficients of a power, by an
    /// addition chain, or by repeated squaring.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, and $m$ is `exp` times the largest number of significant bits of any of its
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x+1").unwrap())
    ///         .pow(3)
    ///         .to_string(),
    ///     "x^3+3*x^2+3*x+1"
    /// );
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("2*x-1").unwrap())
    ///         .pow(4)
    ///         .to_string(),
    ///     "16*x^4-32*x^3+24*x^2-8*x+1"
    /// );
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2-x").unwrap())
    ///         .pow(0)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pow` from `fmpz_poly/pow.c`, FLINT 3.6.0, except that a
    /// factor of $x^k$ is removed before powering, and that the algorithm is chosen by measured
    /// criteria, which include addition chains.
    #[inline]
    fn pow(self, exp: u64) -> IntegerPolynomial {
        IntegerPolynomial {
            coefficients: pow_ref(&self.coefficients, exp),
        }
    }
}

impl PowAssign<u64> for IntegerPolynomial {
    /// Raises an [`IntegerPolynomial`] to a power in place.
    ///
    /// $$
    /// p \gets p^e.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1. Depending on the length of the
    /// polynomial, the size of its coefficients, and the exponent, the power is computed by the
    /// binomial theorem, by J. C. P. Miller's recurrence for the coefficients of a power, by an
    /// addition chain, or by repeated squaring.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, and $m$ is `exp` times the largest number of significant bits of any of its
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::PowAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x+1").unwrap();
    /// p.pow_assign(3);
    /// assert_eq!(p.to_string(), "x^3+3*x^2+3*x+1");
    ///
    /// let mut p = IntegerPolynomial::from_str("2*x-1").unwrap();
    /// p.pow_assign(4);
    /// assert_eq!(p.to_string(), "16*x^4-32*x^3+24*x^2-8*x+1");
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2-x").unwrap();
    /// p.pow_assign(0);
    /// assert_eq!(p.to_string(), "1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pow` from `fmpz_poly/pow.c`, FLINT 3.6.0, except that a
    /// factor of $x^k$ is removed before powering, and that the algorithm is chosen by measured
    /// criteria, which include addition chains.
    #[inline]
    fn pow_assign(&mut self, exp: u64) {
        pow_assign_vec(&mut self.coefficients, exp);
    }
}
