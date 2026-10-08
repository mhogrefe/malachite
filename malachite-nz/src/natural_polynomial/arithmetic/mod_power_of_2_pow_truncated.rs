// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::pow_truncated::pow_truncated_ref;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_power_of_2_mul_truncated::*;
use crate::natural_polynomial::arithmetic::mod_power_of_2_pow::power_needs_no_reduction;
use crate::natural_polynomial::arithmetic::mod_power_of_2_square::assert_reduced;
use crate::natural_polynomial::arithmetic::mod_power_of_2_square_truncated::*;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{ModPowerOf2Pow, ModPowerOf2PowAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, SaturatingFrom};
use malachite_base::polynomial::{
    ModPowerOf2PowTruncated, ModPowerOf2PowTruncatedAssign, Polynomial, pow_binexp_trimmed,
};

// The coefficients, without zeros at the end, of the `e`th power modulo $2^k$, where $k$ is `pow`,
// of the polynomial with coefficients `xs`, which has length at least 2, a nonzero first element,
// and coefficients reduced modulo $2^k$, keeping only the coefficients of $x^i$ for $i$ less than
// `len`, which is at least 2, where `e` is at least 3, by binary exponentiation with truncated
// squaring and multiplication, each reduced and trimmed.
//
// This is equivalent to `_fmpz_mod_poly_pow_trunc_binexp` from `fmpz_mod_poly/pow_trunc_binexp.c`,
// FLINT 3.6.0, with the modulus $2^k$, except that the polynomial is not padded to length `len` and
// the intermediate powers are trimmed.
crate_test_fn! {mod_power_of_2_pow_truncated_binexp(
    xs: &[Natural],
    e: u64,
    len: u64,
    pow: u64,
) -> Vec<Natural> {
    pow_binexp_trimmed(
        xs,
        e,
        |r| mod_power_of_2_square_truncated_ref(r, len, pow).into_coefficients_asc(),
        |r, xs| mod_power_of_2_mul_truncated_ref_ref(r, xs, len, pow).into_coefficients_asc(),
    )
}}

// The `e`th power modulo $2^k$, where $k$ is `pow`, of the polynomial with coefficients `xs`, which
// has no zeros at the end and coefficients reduced modulo $2^k$, keeping only the coefficients of
// $x^i$ for $i$ less than `len`.
//
// Writing the polynomial modulo $x^n$ as $x^\ell q$, with $q_0 \neq 0$, its power is $x^{e\ell}
// (q^e \bmod x^{n - e\ell})$, or 0 if $e\ell \geq n$. When the power of $q$ over the integers
// already has every coefficient less than $2^k$, it is computed with `pow_truncated`; otherwise by
// binary exponentiation modulo $2^k$.
//
// This is equivalent to `fmpz_mod_poly_pow_trunc` from `fmpz_mod_poly/pow_trunc.c`, FLINT 3.6.0,
// with the modulus $2^k$, except for the removal of the factor of $x^\ell$ and the integer power,
// and except that the zeroth power of the zero polynomial is 1, where FLINT gives 0.
pub(crate) fn mod_power_of_2_pow_truncated_ref(
    xs: &[Natural],
    e: u64,
    len: u64,
    pow: u64,
) -> NaturalPolynomial {
    if pow == 0 || len == 0 {
        return NaturalPolynomial::ZERO;
    }
    if e == 0 {
        return NaturalPolynomial::one();
    }
    let n = usize::saturating_from(len);
    let xs = &xs[..xs.len().min(n)];
    let Some(low) = xs.iter().position(|x| *x != 0u32) else {
        return NaturalPolynomial::ZERO;
    };
    let shift = usize::saturating_from(e).saturating_mul(low);
    if shift >= n {
        return NaturalPolynomial::ZERO;
    }
    let q = &xs[low..];
    let q_len = n - shift;
    let q_len_u64 = u64::exact_from(q_len);
    let mut power = match (q.len().min(q_len), e) {
        (1, _) => {
            let c = (&q[0]).mod_power_of_2_pow(Natural::from(e), pow);
            if c == 0u32 { Vec::new() } else { vec![c] }
        }
        (m, 1) => {
            let mut power = q[..m].to_vec();
            while power.last() == Some(&Natural::ZERO) {
                power.pop();
            }
            power
        }
        _ if power_needs_no_reduction(q.len(), vec_max_bits(q).0, e, pow) => {
            pow_truncated_ref(q, e, q_len_u64)
        }
        (_, 2) => mod_power_of_2_square_truncated_ref(q, q_len_u64, pow).into_coefficients_asc(),
        _ => mod_power_of_2_pow_truncated_binexp(q, e, q_len_u64, pow),
    };
    if power.is_empty() {
        return NaturalPolynomial::ZERO;
    }
    power.splice(0..0, core::iter::repeat_n(Natural::ZERO, shift));
    NaturalPolynomial {
        coefficients: power,
    }
}

impl ModPowerOf2PowTruncated for NaturalPolynomial {
    type Output = Self;

    /// Raises a [`NaturalPolynomial`] to a power modulo $2^k$, keeping only the coefficients of
    /// $x^i$ for $i$ less than `len`, taking it by value. Its coefficients must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// f(p, e, n, k) = (p^e \bmod x^n) \bmod 2^k.
    /// $$
    ///
    /// The polynomial need not already be truncated: only its first `len` coefficients are read.
    /// The zeroth power of every polynomial is 1, truncated to 0 when `len` is 0 and reduced to 0
    /// when $k$ is 0. The power is computed by repeated truncated squaring modulo $2^k$, unless no
    /// coefficient of the power over the integers reaches $2^k$, in which case it is computed as in
    /// [`PowTruncated`](malachite_base::polynomial::PowTruncated).
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, $m$ is `pow`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `self` is not reduced modulo $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2PowTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x+1").unwrap())
    ///         .mod_power_of_2_pow_truncated(5, 3, 3)
    ///         .to_string(),
    ///     "2*x^2+5*x+1"
    /// );
    /// // The power is a multiple of x^4.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+x").unwrap())
    ///         .mod_power_of_2_pow_truncated(4, 4, 3)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_pow_trunc` from `fmpz_mod_poly/pow_trunc.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that a factor of $x^\ell$ is removed before powering,
    /// that the intermediate powers are trimmed rather than padded to `len`, that a power needing
    /// no reduction is computed over the integers, and that the zeroth power of the zero polynomial
    /// is 1 (modulo $2^k$), where FLINT gives 0.
    #[inline]
    fn mod_power_of_2_pow_truncated(mut self, exp: u64, len: u64, pow: u64) -> Self {
        self.mod_power_of_2_pow_truncated_assign(exp, len, pow);
        self
    }
}

impl ModPowerOf2PowTruncated for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Raises a [`NaturalPolynomial`] to a power modulo $2^k$, keeping only the coefficients of
    /// $x^i$ for $i$ less than `len`, taking it by reference. Its coefficients must already be
    /// reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, e, n, k) = (p^e \bmod x^n) \bmod 2^k.
    /// $$
    ///
    /// The polynomial need not already be truncated: only its first `len` coefficients are read.
    /// The zeroth power of every polynomial is 1, truncated to 0 when `len` is 0 and reduced to 0
    /// when $k$ is 0. The power is computed by repeated truncated squaring modulo $2^k$, unless no
    /// coefficient of the power over the integers reaches $2^k$, in which case it is computed as in
    /// [`PowTruncated`](malachite_base::polynomial::PowTruncated).
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, $m$ is `pow`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `self` is not reduced modulo $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2PowTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+1").unwrap())
    ///         .mod_power_of_2_pow_truncated(5, 3, 3)
    ///         .to_string(),
    ///     "2*x^2+5*x+1"
    /// );
    /// // The power is a multiple of x^4.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+x").unwrap())
    ///         .mod_power_of_2_pow_truncated(4, 4, 3)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_pow_trunc` from `fmpz_mod_poly/pow_trunc.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that a factor of $x^\ell$ is removed before powering,
    /// that the intermediate powers are trimmed rather than padded to `len`, that a power needing
    /// no reduction is computed over the integers, and that the zeroth power of the zero polynomial
    /// is 1 (modulo $2^k$), where FLINT gives 0.
    fn mod_power_of_2_pow_truncated(self, exp: u64, len: u64, pow: u64) -> NaturalPolynomial {
        assert_reduced(self, pow);
        mod_power_of_2_pow_truncated_ref(&self.coefficients, exp, len, pow)
    }
}

impl ModPowerOf2PowTruncatedAssign for NaturalPolynomial {
    /// Raises a [`NaturalPolynomial`] to a power modulo $2^k$ in place, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`. Its coefficients must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// p \gets (p^e \bmod x^n) \bmod 2^k.
    /// $$
    ///
    /// The polynomial need not already be truncated: only its first `len` coefficients are read.
    /// The zeroth power of every polynomial is 1, truncated to 0 when `len` is 0 and reduced to 0
    /// when $k$ is 0. The power is computed by repeated truncated squaring modulo $2^k$, unless no
    /// coefficient of the power over the integers reaches $2^k$, in which case it is computed as in
    /// [`PowTruncated`](malachite_base::polynomial::PowTruncated).
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, $m$ is `pow`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `self` is not reduced modulo $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2PowTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x+1").unwrap();
    /// p.mod_power_of_2_pow_truncated_assign(5, 3, 3);
    /// assert_eq!(p.to_string(), "2*x^2+5*x+1");
    ///
    /// // The power is a multiple of x^4.
    /// let mut p = NaturalPolynomial::from_str("x^2+x").unwrap();
    /// p.mod_power_of_2_pow_truncated_assign(4, 4, 3);
    /// assert_eq!(p.to_string(), "0");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_pow_trunc` from `fmpz_mod_poly/pow_trunc.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that a factor of $x^\ell$ is removed before powering,
    /// that the intermediate powers are trimmed rather than padded to `len`, that a power needing
    /// no reduction is computed over the integers, and that the zeroth power of the zero polynomial
    /// is 1 (modulo $2^k$), where FLINT gives 0.
    fn mod_power_of_2_pow_truncated_assign(&mut self, exp: u64, len: u64, pow: u64) {
        assert_reduced(self, pow);
        let xs = &mut self.coefficients;
        match (xs.len(), exp, len, pow) {
            (_, _, 0, _) | (_, _, _, 0) => xs.clear(),
            (0, 0, _, _) => xs.push(Natural::ONE),
            (_, 0, _, _) => {
                xs.truncate(1);
                xs[0] = Natural::ONE;
            }
            (0, _, _, _) => {}
            (_, 1, _, _) => self.truncate_assign(len),
            (1, _, _, _) => {
                xs[0].mod_power_of_2_pow_assign(Natural::from(exp), pow);
                if xs[0] == 0u32 {
                    xs.clear();
                }
            }
            _ => *self = mod_power_of_2_pow_truncated_ref(xs, exp, len, pow),
        }
    }
}
