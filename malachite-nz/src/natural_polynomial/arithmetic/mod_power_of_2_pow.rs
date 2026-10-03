// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::pow::binexp::pow_binexp_trimmed;
use crate::integer_polynomial::arithmetic::pow::pow_ref;
use crate::integer_polynomial::arithmetic::vec::max_bits::vec_max_bits;
use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_power_of_2_mul::mod_power_of_2_mul_ref_ref;
use crate::natural_polynomial::arithmetic::mod_power_of_2_square::{
    assert_reduced, mod_power_of_2_square_ref,
};
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, ModPowerOf2Assign, ModPowerOf2Pow, ModPowerOf2PowAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;

// Whether the `e`th power of a polynomial of length `len`, whose coefficients have at most `bits`
// significant bits, has every coefficient less than $2^k$, where $k$ is `pow`, so that it needs no
// reduction. Each coefficient of the power is at most the `e`th power of the sum of the
// coefficients, which is less than $\ell 2^b$.
pub(crate) fn power_needs_no_reduction(len: usize, bits: u64, e: u64, pow: u64) -> bool {
    (bits + u64::exact_from(len).ceiling_log_base_2())
        .checked_mul(e)
        .is_some_and(|b| b <= pow)
}

// The coefficients, without zeros at the end, of the `e`th power modulo $2^k$, where $k$ is `pow`,
// of the polynomial with coefficients `xs`, which has length at least 2, nonzero first and last
// elements, and coefficients reduced modulo $2^k$, where `e` is at least 3, by binary
// exponentiation: each square and product is reduced and trimmed, so the intermediate powers shrink
// when leading coefficients vanish modulo $2^k$.
//
// This is equivalent to `_fmpz_mod_poly_pow` from `fmpz_mod_poly/pow.c`, FLINT 3.6.0, with the
// modulus $2^k$, except that the intermediate powers are trimmed.
crate_test_fn! {mod_power_of_2_pow_binexp(xs: &[Natural], e: u64, pow: u64) -> Vec<Natural> {
    pow_binexp_trimmed(
        xs,
        e,
        |r| mod_power_of_2_square_ref(r, pow).into_coefficients_asc(),
        |r, xs| mod_power_of_2_mul_ref_ref(r, xs, pow).into_coefficients_asc(),
    )
}}

// The coefficients, without zeros at the end, of the `e`th power modulo $2^k$, where $k$ is `pow`,
// of the polynomial with coefficients `xs`, as `mod_power_of_2_pow_binexp` requires, computed as
// the power over the integers, with its coefficients reduced afterwards.
crate_test_fn! {mod_power_of_2_pow_exact(xs: &[Natural], e: u64, pow: u64) -> Vec<Natural> {
    let mut out = pow_ref(xs, e);
    for x in &mut out {
        x.mod_power_of_2_assign(pow);
    }
    while out.last() == Some(&Natural::ZERO) {
        out.pop();
    }
    out
}}

// The `e`th power modulo $2^k$, where $k$ is `pow`, of the polynomial with coefficients `xs`, which
// has no zeros at the end and coefficients reduced modulo $2^k$.
//
// Writing the polynomial as $x^\ell q$, with $q_0 \neq 0$, its power is $x^{e\ell} q^e$. When the
// power of $q$ over the integers already has every coefficient less than $2^k$, it is computed with
// `pow`, by whichever algorithm suits it; otherwise by binary exponentiation modulo $2^k$.
//
// This is equivalent to `fmpz_mod_poly_pow` from `fmpz_mod_poly/pow.c`, FLINT 3.6.0, with the
// modulus $2^k$, except for the removal of the factor of $x^\ell$ and the integer power.
pub(crate) fn mod_power_of_2_pow_ref(xs: &[Natural], e: u64, pow: u64) -> NaturalPolynomial {
    if pow == 0 {
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
            let c = (&q[0]).mod_power_of_2_pow(Natural::from(e), pow);
            if c == 0u32 { Vec::new() } else { vec![c] }
        }
        (_, 1) => q.to_vec(),
        _ if power_needs_no_reduction(q.len(), vec_max_bits(q).0, e, pow) => pow_ref(q, e),
        (_, 2) => mod_power_of_2_square_ref(q, pow).into_coefficients_asc(),
        _ => mod_power_of_2_pow_binexp(q, e, pow),
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

impl ModPowerOf2Pow<u64> for NaturalPolynomial {
    type Output = Self;

    /// Raises a [`NaturalPolynomial`] to a power modulo $2^k$, taking it by value. Its coefficients
    /// must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, e, k) = p^e \bmod 2^k.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1, which is 0 modulo $2^0$. Since a
    /// power of a polynomial with even coefficients can vanish modulo $2^k$, the degree of the
    /// power may be less than $e$ times the degree of the polynomial. The power is computed by
    /// repeated squaring modulo $2^k$, unless no coefficient of the power over the integers reaches
    /// $2^k$, in which case it is computed as in
    /// [`Pow`](malachite_base::num::arithmetic::traits::Pow).
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, $m$ is `pow`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `self` is not reduced modulo $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Pow;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x+1").unwrap())
    ///         .mod_power_of_2_pow(5, 3)
    ///         .to_string(),
    ///     "x^5+5*x^4+2*x^3+2*x^2+5*x+1"
    /// );
    /// // The square of 2*x+1 modulo 4 is 1.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("2*x+1").unwrap())
    ///         .mod_power_of_2_pow(2, 2)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_pow` from `fmpz_mod_poly/pow.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that a factor of $x^\ell$ is removed before powering, that the
    /// intermediate powers are trimmed, and that a power needing no reduction is computed over the
    /// integers.
    #[inline]
    fn mod_power_of_2_pow(mut self, exp: u64, pow: u64) -> Self {
        self.mod_power_of_2_pow_assign(exp, pow);
        self
    }
}

impl ModPowerOf2Pow<u64> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Raises a [`NaturalPolynomial`] to a power modulo $2^k$, taking it by reference. Its
    /// coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, e, k) = p^e \bmod 2^k.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1, which is 0 modulo $2^0$. Since a
    /// power of a polynomial with even coefficients can vanish modulo $2^k$, the degree of the
    /// power may be less than $e$ times the degree of the polynomial. The power is computed by
    /// repeated squaring modulo $2^k$, unless no coefficient of the power over the integers reaches
    /// $2^k$, in which case it is computed as in
    /// [`Pow`](malachite_base::num::arithmetic::traits::Pow).
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, $m$ is `pow`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `self` is not reduced modulo $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Pow;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+1").unwrap())
    ///         .mod_power_of_2_pow(5, 3)
    ///         .to_string(),
    ///     "x^5+5*x^4+2*x^3+2*x^2+5*x+1"
    /// );
    /// // The square of 2*x+1 modulo 4 is 1.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("2*x+1").unwrap())
    ///         .mod_power_of_2_pow(2, 2)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_pow` from `fmpz_mod_poly/pow.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that a factor of $x^\ell$ is removed before powering, that the
    /// intermediate powers are trimmed, and that a power needing no reduction is computed over the
    /// integers.
    fn mod_power_of_2_pow(self, exp: u64, pow: u64) -> NaturalPolynomial {
        assert_reduced(self, pow);
        mod_power_of_2_pow_ref(&self.coefficients, exp, pow)
    }
}

impl ModPowerOf2PowAssign<u64> for NaturalPolynomial {
    /// Raises a [`NaturalPolynomial`] to a power modulo $2^k$ in place. Its coefficients must
    /// already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \gets p^e \bmod 2^k.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1, which is 0 modulo $2^0$. Since a
    /// power of a polynomial with even coefficients can vanish modulo $2^k$, the degree of the
    /// power may be less than $e$ times the degree of the polynomial. The power is computed by
    /// repeated squaring modulo $2^k$, unless no coefficient of the power over the integers reaches
    /// $2^k$, in which case it is computed as in
    /// [`Pow`](malachite_base::num::arithmetic::traits::Pow).
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, $m$ is `pow`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `self` is not reduced modulo $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2PowAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x+1").unwrap();
    /// p.mod_power_of_2_pow_assign(5, 3);
    /// assert_eq!(p.to_string(), "x^5+5*x^4+2*x^3+2*x^2+5*x+1");
    ///
    /// // The square of 2*x+1 modulo 4 is 1.
    /// let mut p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// p.mod_power_of_2_pow_assign(2, 2);
    /// assert_eq!(p.to_string(), "1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_pow` from `fmpz_mod_poly/pow.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that a factor of $x^\ell$ is removed before powering, that the
    /// intermediate powers are trimmed, and that a power needing no reduction is computed over the
    /// integers.
    fn mod_power_of_2_pow_assign(&mut self, exp: u64, pow: u64) {
        assert_reduced(self, pow);
        let xs = &mut self.coefficients;
        match (xs.len(), exp, pow) {
            (_, _, 0) => xs.clear(),
            (0, 0, _) => xs.push(Natural::ONE),
            (_, 0, _) => {
                xs.truncate(1);
                xs[0] = Natural::ONE;
            }
            (0, _, _) | (_, 1, _) => {}
            (1, _, _) => {
                xs[0].mod_power_of_2_pow_assign(Natural::from(exp), pow);
                if xs[0] == 0u32 {
                    xs.clear();
                }
            }
            _ => *self = mod_power_of_2_pow_ref(xs, exp, pow),
        }
    }
}
