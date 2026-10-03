// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::pow::pow_ref;
use crate::integer_polynomial::arithmetic::vec::max_bits::vec_max_bits;
use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_mul::mod_mul_ref_ref;
use crate::natural_polynomial::arithmetic::mod_square::{assert_reduced, mod_square_ref};
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, ModAssign, ModPow, ModPowAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::{Polynomial, pow_binexp_trimmed};

// Whether the `e`th power of a polynomial of length `len`, whose coefficients have at most `bits`
// significant bits, has every coefficient less than `m`, so that it needs no reduction. Each
// coefficient of the power is less than $(\ell 2^b)^e \leq 2^{e(b + \lceil \log_2 \ell \rceil)}$,
// which is at most $2^{\lfloor \log_2 m \rfloor} \leq m$ when the exponent is less than the number
// of significant bits of `m`.
pub(crate) fn power_is_below(len: usize, bits: u64, e: u64, m: &Natural) -> bool {
    (bits + u64::exact_from(len).ceiling_log_base_2())
        .checked_mul(e)
        .is_some_and(|b| b < m.significant_bits())
}

// The coefficients, without zeros at the end, of the `e`th power modulo `m` of the polynomial with
// coefficients `xs`, which has length at least 2, nonzero first and last elements, and coefficients
// reduced modulo `m`, where `e` is at least 3, by binary exponentiation: each square and product is
// reduced and trimmed, so the intermediate powers shrink when leading coefficients vanish modulo
// `m`.
//
// This is equivalent to `_fmpz_mod_poly_pow` from `fmpz_mod_poly/pow.c`, FLINT 3.6.0, except that
// the intermediate powers are trimmed.
crate_test_fn! {mod_pow_binexp(xs: &[Natural], e: u64, m: &Natural) -> Vec<Natural> {
    pow_binexp_trimmed(
        xs,
        e,
        |r| mod_square_ref(r, m).into_coefficients_asc(),
        |r, xs| mod_mul_ref_ref(r, xs, m).into_coefficients_asc(),
    )
}}

// The coefficients, without zeros at the end, of the `e`th power modulo `m` of the polynomial with
// coefficients `xs`, as `mod_pow_binexp` requires, computed as the power over the integers, with
// its coefficients reduced afterwards.
crate_test_fn! {mod_pow_exact(xs: &[Natural], e: u64, m: &Natural) -> Vec<Natural> {
    let mut out = pow_ref(xs, e);
    for x in &mut out {
        x.mod_assign(m);
    }
    while out.last() == Some(&Natural::ZERO) {
        out.pop();
    }
    out
}}

// The `e`th power modulo `m` of the polynomial with coefficients `xs`, which has no zeros at the
// end and coefficients reduced modulo `m`.
//
// Writing the polynomial as $x^\ell q$, with $q_0 \neq 0$, its power is $x^{e\ell} q^e$. When the
// power of $q$ over the integers already has every coefficient less than `m`, it is computed with
// `pow`, by whichever algorithm suits it; otherwise by binary exponentiation modulo `m`.
//
// This is equivalent to `fmpz_mod_poly_pow` from `fmpz_mod_poly/pow.c`, FLINT 3.6.0, except for the
// removal of the factor of $x^\ell$ and the integer power.
pub(crate) fn mod_pow_ref(xs: &[Natural], e: u64, m: &Natural) -> NaturalPolynomial {
    if *m == 1u32 {
        return NaturalPolynomial::ZERO;
    }
    if e == 0 {
        return NaturalPolynomial::one();
    }
    let Some(low) = xs.iter().position(|x| *x != 0u32) else {
        return NaturalPolynomial::ZERO;
    };
    let q = &xs[low..];
    let mut power = match (q.len(), e) {
        (1, _) => {
            let c = (&q[0]).mod_pow(Natural::from(e), m);
            if c == 0u32 { Vec::new() } else { vec![c] }
        }
        (_, 1) => q.to_vec(),
        _ if power_is_below(q.len(), vec_max_bits(q).0, e, m) => pow_ref(q, e),
        (_, 2) => mod_square_ref(q, m).into_coefficients_asc(),
        _ => mod_pow_binexp(q, e, m),
    };
    if power.is_empty() {
        return NaturalPolynomial::ZERO;
    }
    if low != 0 {
        let shift = usize::exact_from(e)
            .checked_mul(low)
            .expect("the power has too many coefficients to represent");
        power.splice(0..0, core::iter::repeat_n(Natural::ZERO, shift));
    }
    NaturalPolynomial {
        coefficients: power,
    }
}

// Replaces the coefficients of `p`, which has coefficients reduced modulo `m`, with those of its
// `e`th power modulo `m`, reusing them when the power of a constant is computed or nothing changes.
fn mod_pow_assign_helper(p: &mut NaturalPolynomial, e: u64, m: &Natural) {
    let xs = &mut p.coefficients;
    if *m == 1u32 {
        xs.clear();
        return;
    }
    match (xs.len(), e) {
        (0, 0) => xs.push(Natural::ONE),
        (_, 0) => {
            xs.truncate(1);
            xs[0] = Natural::ONE;
        }
        (0, _) | (_, 1) => {}
        (1, _) => {
            xs[0].mod_pow_assign(Natural::from(e), m);
            if xs[0] == 0u32 {
                xs.clear();
            }
        }
        _ => *p = mod_pow_ref(xs, e, m),
    }
}

impl ModPow<u64, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Raises a [`NaturalPolynomial`] to a power modulo `m`, taking the polynomial by value and the
    /// modulus by value. Its coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, e, m) = p^e \bmod m.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1, which is 0 modulo 1. When `m` is
    /// not prime, the leading coefficients of a power can vanish modulo `m`, and then its degree is
    /// lower than $e$ times the degree of the polynomial. The power is computed by repeated
    /// squaring modulo `m`, unless no coefficient of the power over the integers reaches `m`, in
    /// which case it is computed as in [`Pow`](malachite_base::num::arithmetic::traits::Pow).
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, $m$ is `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x+1").unwrap())
    ///         .mod_pow(5, Natural::from(7u32))
    ///         .to_string(),
    ///     "x^5+5*x^4+3*x^3+3*x^2+5*x+1"
    /// );
    /// // Modulo 4, which has zero divisors, the square of 2*x+1 is 1.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("2*x+1").unwrap())
    ///         .mod_pow(2, Natural::from(4u32))
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_pow` from `fmpz_mod_poly/pow.c`, FLINT 3.6.0, except
    /// that a factor of $x^\ell$ is removed before powering, that the intermediate powers are
    /// trimmed, and that a power needing no reduction is computed over the integers.
    #[inline]
    fn mod_pow(mut self, exp: u64, m: Natural) -> Self {
        self.mod_pow_assign(exp, m);
        self
    }
}

impl ModPow<u64, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Raises a [`NaturalPolynomial`] to a power modulo `m`, taking the polynomial by value and the
    /// modulus by reference. Its coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, e, m) = p^e \bmod m.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1, which is 0 modulo 1. When `m` is
    /// not prime, the leading coefficients of a power can vanish modulo `m`, and then its degree is
    /// lower than $e$ times the degree of the polynomial. The power is computed by repeated
    /// squaring modulo `m`, unless no coefficient of the power over the integers reaches `m`, in
    /// which case it is computed as in [`Pow`](malachite_base::num::arithmetic::traits::Pow).
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, $m$ is `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x+1").unwrap())
    ///         .mod_pow(5, &Natural::from(7u32))
    ///         .to_string(),
    ///     "x^5+5*x^4+3*x^3+3*x^2+5*x+1"
    /// );
    /// // Modulo 4, which has zero divisors, the square of 2*x+1 is 1.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("2*x+1").unwrap())
    ///         .mod_pow(2, &Natural::from(4u32))
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_pow` from `fmpz_mod_poly/pow.c`, FLINT 3.6.0, except
    /// that a factor of $x^\ell$ is removed before powering, that the intermediate powers are
    /// trimmed, and that a power needing no reduction is computed over the integers.
    #[inline]
    fn mod_pow(mut self, exp: u64, m: &Natural) -> Self {
        self.mod_pow_assign(exp, m);
        self
    }
}

impl ModPow<u64, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Raises a [`NaturalPolynomial`] to a power modulo `m`, taking the polynomial by reference and
    /// the modulus by value. Its coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, e, m) = p^e \bmod m.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1, which is 0 modulo 1. When `m` is
    /// not prime, the leading coefficients of a power can vanish modulo `m`, and then its degree is
    /// lower than $e$ times the degree of the polynomial. The power is computed by repeated
    /// squaring modulo `m`, unless no coefficient of the power over the integers reaches `m`, in
    /// which case it is computed as in [`Pow`](malachite_base::num::arithmetic::traits::Pow).
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, $m$ is `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+1").unwrap())
    ///         .mod_pow(5, Natural::from(7u32))
    ///         .to_string(),
    ///     "x^5+5*x^4+3*x^3+3*x^2+5*x+1"
    /// );
    /// // Modulo 4, which has zero divisors, the square of 2*x+1 is 1.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("2*x+1").unwrap())
    ///         .mod_pow(2, Natural::from(4u32))
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_pow` from `fmpz_mod_poly/pow.c`, FLINT 3.6.0, except
    /// that a factor of $x^\ell$ is removed before powering, that the intermediate powers are
    /// trimmed, and that a power needing no reduction is computed over the integers.
    fn mod_pow(self, exp: u64, m: Natural) -> NaturalPolynomial {
        assert_reduced(self, &m);
        mod_pow_ref(&self.coefficients, exp, &m)
    }
}

impl ModPow<u64, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Raises a [`NaturalPolynomial`] to a power modulo `m`, taking the polynomial by reference and
    /// the modulus by reference. Its coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, e, m) = p^e \bmod m.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1, which is 0 modulo 1. When `m` is
    /// not prime, the leading coefficients of a power can vanish modulo `m`, and then its degree is
    /// lower than $e$ times the degree of the polynomial. The power is computed by repeated
    /// squaring modulo `m`, unless no coefficient of the power over the integers reaches `m`, in
    /// which case it is computed as in [`Pow`](malachite_base::num::arithmetic::traits::Pow).
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, $m$ is `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+1").unwrap())
    ///         .mod_pow(5, &Natural::from(7u32))
    ///         .to_string(),
    ///     "x^5+5*x^4+3*x^3+3*x^2+5*x+1"
    /// );
    /// // Modulo 4, which has zero divisors, the square of 2*x+1 is 1.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("2*x+1").unwrap())
    ///         .mod_pow(2, &Natural::from(4u32))
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_pow` from `fmpz_mod_poly/pow.c`, FLINT 3.6.0, except
    /// that a factor of $x^\ell$ is removed before powering, that the intermediate powers are
    /// trimmed, and that a power needing no reduction is computed over the integers.
    fn mod_pow(self, exp: u64, m: &Natural) -> NaturalPolynomial {
        assert_reduced(self, m);
        mod_pow_ref(&self.coefficients, exp, m)
    }
}

impl ModPowAssign<u64, Natural> for NaturalPolynomial {
    /// Raises a [`NaturalPolynomial`] to a power modulo `m` in place, taking the modulus by value.
    /// Its coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets p^e \bmod m.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1, which is 0 modulo 1. When `m` is
    /// not prime, the leading coefficients of a power can vanish modulo `m`, and then its degree is
    /// lower than $e$ times the degree of the polynomial. The power is computed by repeated
    /// squaring modulo `m`, unless no coefficient of the power over the integers reaches `m`, in
    /// which case it is computed as in [`Pow`](malachite_base::num::arithmetic::traits::Pow).
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, $m$ is `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x+1").unwrap();
    /// p.mod_pow_assign(5, Natural::from(7u32));
    /// assert_eq!(p.to_string(), "x^5+5*x^4+3*x^3+3*x^2+5*x+1");
    ///
    /// // Modulo 4, which has zero divisors, the square of 2*x+1 is 1.
    /// let mut p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// p.mod_pow_assign(2, Natural::from(4u32));
    /// assert_eq!(p.to_string(), "1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_pow` from `fmpz_mod_poly/pow.c`, FLINT 3.6.0, except
    /// that a factor of $x^\ell$ is removed before powering, that the intermediate powers are
    /// trimmed, and that a power needing no reduction is computed over the integers.
    fn mod_pow_assign(&mut self, exp: u64, m: Natural) {
        assert_reduced(self, &m);
        mod_pow_assign_helper(self, exp, &m);
    }
}

impl ModPowAssign<u64, &Natural> for NaturalPolynomial {
    /// Raises a [`NaturalPolynomial`] to a power modulo `m` in place, taking the modulus by
    /// reference. Its coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets p^e \bmod m.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1, which is 0 modulo 1. When `m` is
    /// not prime, the leading coefficients of a power can vanish modulo `m`, and then its degree is
    /// lower than $e$ times the degree of the polynomial. The power is computed by repeated
    /// squaring modulo `m`, unless no coefficient of the power over the integers reaches `m`, in
    /// which case it is computed as in [`Pow`](malachite_base::num::arithmetic::traits::Pow).
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, $m$ is `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x+1").unwrap();
    /// p.mod_pow_assign(5, &Natural::from(7u32));
    /// assert_eq!(p.to_string(), "x^5+5*x^4+3*x^3+3*x^2+5*x+1");
    ///
    /// // Modulo 4, which has zero divisors, the square of 2*x+1 is 1.
    /// let mut p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// p.mod_pow_assign(2, &Natural::from(4u32));
    /// assert_eq!(p.to_string(), "1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_pow` from `fmpz_mod_poly/pow.c`, FLINT 3.6.0, except
    /// that a factor of $x^\ell$ is removed before powering, that the intermediate powers are
    /// trimmed, and that a power needing no reduction is computed over the integers.
    fn mod_pow_assign(&mut self, exp: u64, m: &Natural) {
        assert_reduced(self, m);
        mod_pow_assign_helper(self, exp, m);
    }
}
