// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::pow::binexp::pow_binexp_trimmed;
use crate::integer_polynomial::arithmetic::pow_truncated::pow_truncated_ref;
use crate::integer_polynomial::arithmetic::vec::max_bits::vec_max_bits;
use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_mul_truncated::mod_mul_truncated_ref_ref;
use crate::natural_polynomial::arithmetic::mod_pow::power_is_below;
use crate::natural_polynomial::arithmetic::mod_square::assert_reduced;
use crate::natural_polynomial::arithmetic::mod_square_truncated::mod_square_truncated_ref;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{ModPow, ModPowAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, SaturatingFrom};
use malachite_base::polynomial::{ModPowTruncated, ModPowTruncatedAssign, Polynomial};

// The coefficients, without zeros at the end, of the `e`th power modulo `m` of the polynomial with
// coefficients `xs`, which has length at least 2, a nonzero first element, and coefficients reduced
// modulo `m`, keeping only the coefficients of $x^i$ for $i$ less than `len`, which is at least 2,
// where `e` is at least 3, by binary exponentiation with truncated squaring and multiplication,
// each reduced and trimmed.
//
// This is equivalent to `_fmpz_mod_poly_pow_trunc_binexp` from `fmpz_mod_poly/pow_trunc_binexp.c`,
// FLINT 3.6.0, except that the polynomial is not padded to length `len` and the intermediate powers
// are trimmed.
crate_test_fn! {mod_pow_truncated_binexp(
    xs: &[Natural],
    e: u64,
    len: u64,
    m: &Natural,
) -> Vec<Natural> {
    pow_binexp_trimmed(
        xs,
        e,
        |r| mod_square_truncated_ref(r, len, m).into_coefficients_asc(),
        |r, xs| mod_mul_truncated_ref_ref(r, xs, len, m).into_coefficients_asc(),
    )
}}

// The `e`th power modulo `m` of the polynomial with coefficients `xs`, which has no zeros at the
// end and coefficients reduced modulo `m`, keeping only the coefficients of $x^i$ for $i$ less than
// `len`.
//
// Writing the polynomial modulo $x^n$ as $x^\ell q$, with $q_0 \neq 0$, its power is $x^{e\ell}
// (q^e \bmod x^{n - e\ell})$, or 0 if $e\ell \geq n$. When the power of $q$ over the integers
// already has every coefficient less than `m`, it is computed with `pow_truncated`; otherwise by
// binary exponentiation modulo `m`.
//
// This is equivalent to `fmpz_mod_poly_pow_trunc` from `fmpz_mod_poly/pow_trunc.c`, FLINT 3.6.0,
// except for the removal of the factor of $x^\ell$ and the integer power, and except that the
// zeroth power of the zero polynomial is 1, where FLINT gives 0.
pub(crate) fn mod_pow_truncated_ref(
    xs: &[Natural],
    e: u64,
    len: u64,
    m: &Natural,
) -> NaturalPolynomial {
    if *m == 1u32 || len == 0 {
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
            let c = (&q[0]).mod_pow(Natural::from(e), m);
            if c == 0u32 { Vec::new() } else { vec![c] }
        }
        (k, 1) => {
            let mut power = q[..k].to_vec();
            while power.last() == Some(&Natural::ZERO) {
                power.pop();
            }
            power
        }
        _ if power_is_below(q.len(), vec_max_bits(q).0, e, m) => pow_truncated_ref(q, e, q_len_u64),
        (_, 2) => mod_square_truncated_ref(q, q_len_u64, m).into_coefficients_asc(),
        _ => mod_pow_truncated_binexp(q, e, q_len_u64, m),
    };
    if power.is_empty() {
        return NaturalPolynomial::ZERO;
    }
    power.splice(0..0, core::iter::repeat_n(Natural::ZERO, shift));
    NaturalPolynomial {
        coefficients: power,
    }
}

// Replaces the coefficients of `p`, which has coefficients reduced modulo `m`, with those of its
// `e`th power modulo `m` truncated to length `len`, reusing them when the power of a constant is
// computed or the power is the polynomial itself.
fn mod_pow_truncated_assign_helper(p: &mut NaturalPolynomial, e: u64, len: u64, m: &Natural) {
    if *m == 1u32 || len == 0 {
        p.coefficients.clear();
        return;
    }
    let xs = &mut p.coefficients;
    match (xs.len(), e) {
        (0, 0) => xs.push(Natural::ONE),
        (_, 0) => {
            xs.truncate(1);
            xs[0] = Natural::ONE;
        }
        (0, _) => {}
        (_, 1) => p.truncate_assign(len),
        (1, _) => {
            xs[0].mod_pow_assign(Natural::from(e), m);
            if xs[0] == 0u32 {
                xs.clear();
            }
        }
        _ => *p = mod_pow_truncated_ref(xs, e, len, m),
    }
}

impl ModPowTruncated<Natural> for NaturalPolynomial {
    type Output = Self;

    /// Raises a [`NaturalPolynomial`] to a power modulo `m`, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking the polynomial by value and the modulus by value. Its
    /// coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, e, n, m) = (p^e \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomial need not already be truncated: only its first `len` coefficients are read.
    /// The zeroth power of every polynomial is 1, truncated to 0 when `len` is 0 and reduced to 0
    /// when `m` is 1. The power is computed by repeated truncated squaring modulo `m`, unless no
    /// coefficient of the power over the integers reaches `m`, in which case it is computed as in
    /// [`PowTruncated`](malachite_base::polynomial::PowTruncated).
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, $m$ is `m.significant_bits()`,
    /// and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x+1").unwrap())
    ///         .mod_pow_truncated(5, 3, Natural::from(7u32))
    ///         .to_string(),
    ///     "3*x^2+5*x+1"
    /// );
    /// // The power is a multiple of x^4.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+x").unwrap())
    ///         .mod_pow_truncated(4, 4, Natural::from(7u32))
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_pow_trunc` from `fmpz_mod_poly/pow_trunc.c`, FLINT
    /// 3.6.0, except that a factor of $x^\ell$ is removed before powering, that the intermediate
    /// powers are trimmed rather than padded to `len`, that a power needing no reduction is
    /// computed over the integers, and that the zeroth power of the zero polynomial is 1 (modulo
    /// `m`), where FLINT gives 0.
    #[inline]
    fn mod_pow_truncated(mut self, exp: u64, len: u64, m: Natural) -> Self {
        self.mod_pow_truncated_assign(exp, len, m);
        self
    }
}

impl ModPowTruncated<&Natural> for NaturalPolynomial {
    type Output = Self;

    /// Raises a [`NaturalPolynomial`] to a power modulo `m`, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking the polynomial by value and the modulus by reference. Its
    /// coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, e, n, m) = (p^e \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomial need not already be truncated: only its first `len` coefficients are read.
    /// The zeroth power of every polynomial is 1, truncated to 0 when `len` is 0 and reduced to 0
    /// when `m` is 1. The power is computed by repeated truncated squaring modulo `m`, unless no
    /// coefficient of the power over the integers reaches `m`, in which case it is computed as in
    /// [`PowTruncated`](malachite_base::polynomial::PowTruncated).
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, $m$ is `m.significant_bits()`,
    /// and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x+1").unwrap())
    ///         .mod_pow_truncated(5, 3, &Natural::from(7u32))
    ///         .to_string(),
    ///     "3*x^2+5*x+1"
    /// );
    /// // The power is a multiple of x^4.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+x").unwrap())
    ///         .mod_pow_truncated(4, 4, &Natural::from(7u32))
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_pow_trunc` from `fmpz_mod_poly/pow_trunc.c`, FLINT
    /// 3.6.0, except that a factor of $x^\ell$ is removed before powering, that the intermediate
    /// powers are trimmed rather than padded to `len`, that a power needing no reduction is
    /// computed over the integers, and that the zeroth power of the zero polynomial is 1 (modulo
    /// `m`), where FLINT gives 0.
    #[inline]
    fn mod_pow_truncated(mut self, exp: u64, len: u64, m: &Natural) -> Self {
        self.mod_pow_truncated_assign(exp, len, m);
        self
    }
}

impl ModPowTruncated<Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Raises a [`NaturalPolynomial`] to a power modulo `m`, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking the polynomial by reference and the modulus by value. Its
    /// coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, e, n, m) = (p^e \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomial need not already be truncated: only its first `len` coefficients are read.
    /// The zeroth power of every polynomial is 1, truncated to 0 when `len` is 0 and reduced to 0
    /// when `m` is 1. The power is computed by repeated truncated squaring modulo `m`, unless no
    /// coefficient of the power over the integers reaches `m`, in which case it is computed as in
    /// [`PowTruncated`](malachite_base::polynomial::PowTruncated).
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, $m$ is `m.significant_bits()`,
    /// and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+1").unwrap())
    ///         .mod_pow_truncated(5, 3, Natural::from(7u32))
    ///         .to_string(),
    ///     "3*x^2+5*x+1"
    /// );
    /// // The power is a multiple of x^4.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+x").unwrap())
    ///         .mod_pow_truncated(4, 4, Natural::from(7u32))
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_pow_trunc` from `fmpz_mod_poly/pow_trunc.c`, FLINT
    /// 3.6.0, except that a factor of $x^\ell$ is removed before powering, that the intermediate
    /// powers are trimmed rather than padded to `len`, that a power needing no reduction is
    /// computed over the integers, and that the zeroth power of the zero polynomial is 1 (modulo
    /// `m`), where FLINT gives 0.
    fn mod_pow_truncated(self, exp: u64, len: u64, m: Natural) -> NaturalPolynomial {
        assert_reduced(self, &m);
        mod_pow_truncated_ref(&self.coefficients, exp, len, &m)
    }
}

impl ModPowTruncated<&Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Raises a [`NaturalPolynomial`] to a power modulo `m`, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking the polynomial by reference and the modulus by reference.
    /// Its coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, e, n, m) = (p^e \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomial need not already be truncated: only its first `len` coefficients are read.
    /// The zeroth power of every polynomial is 1, truncated to 0 when `len` is 0 and reduced to 0
    /// when `m` is 1. The power is computed by repeated truncated squaring modulo `m`, unless no
    /// coefficient of the power over the integers reaches `m`, in which case it is computed as in
    /// [`PowTruncated`](malachite_base::polynomial::PowTruncated).
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, $m$ is `m.significant_bits()`,
    /// and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+1").unwrap())
    ///         .mod_pow_truncated(5, 3, &Natural::from(7u32))
    ///         .to_string(),
    ///     "3*x^2+5*x+1"
    /// );
    /// // The power is a multiple of x^4.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+x").unwrap())
    ///         .mod_pow_truncated(4, 4, &Natural::from(7u32))
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_pow_trunc` from `fmpz_mod_poly/pow_trunc.c`, FLINT
    /// 3.6.0, except that a factor of $x^\ell$ is removed before powering, that the intermediate
    /// powers are trimmed rather than padded to `len`, that a power needing no reduction is
    /// computed over the integers, and that the zeroth power of the zero polynomial is 1 (modulo
    /// `m`), where FLINT gives 0.
    fn mod_pow_truncated(self, exp: u64, len: u64, m: &Natural) -> NaturalPolynomial {
        assert_reduced(self, m);
        mod_pow_truncated_ref(&self.coefficients, exp, len, m)
    }
}

impl ModPowTruncatedAssign<Natural> for NaturalPolynomial {
    /// Raises a [`NaturalPolynomial`] to a power modulo `m` in place, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the modulus by value. Its coefficients must already
    /// be reduced modulo `m`.
    ///
    /// $$
    /// p \gets (p^e \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomial need not already be truncated: only its first `len` coefficients are read.
    /// The zeroth power of every polynomial is 1, truncated to 0 when `len` is 0 and reduced to 0
    /// when `m` is 1. The power is computed by repeated truncated squaring modulo `m`, unless no
    /// coefficient of the power over the integers reaches `m`, in which case it is computed as in
    /// [`PowTruncated`](malachite_base::polynomial::PowTruncated).
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, $m$ is `m.significant_bits()`,
    /// and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowTruncatedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x+1").unwrap();
    /// p.mod_pow_truncated_assign(5, 3, Natural::from(7u32));
    /// assert_eq!(p.to_string(), "3*x^2+5*x+1");
    ///
    /// // The power is a multiple of x^4.
    /// let mut p = NaturalPolynomial::from_str("x^2+x").unwrap();
    /// p.mod_pow_truncated_assign(4, 4, Natural::from(7u32));
    /// assert_eq!(p.to_string(), "0");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_pow_trunc` from `fmpz_mod_poly/pow_trunc.c`, FLINT
    /// 3.6.0, except that a factor of $x^\ell$ is removed before powering, that the intermediate
    /// powers are trimmed rather than padded to `len`, that a power needing no reduction is
    /// computed over the integers, and that the zeroth power of the zero polynomial is 1 (modulo
    /// `m`), where FLINT gives 0.
    fn mod_pow_truncated_assign(&mut self, exp: u64, len: u64, m: Natural) {
        assert_reduced(self, &m);
        mod_pow_truncated_assign_helper(self, exp, len, &m);
    }
}

impl ModPowTruncatedAssign<&Natural> for NaturalPolynomial {
    /// Raises a [`NaturalPolynomial`] to a power modulo `m` in place, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the modulus by reference. Its coefficients must
    /// already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets (p^e \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomial need not already be truncated: only its first `len` coefficients are read.
    /// The zeroth power of every polynomial is 1, truncated to 0 when `len` is 0 and reduced to 0
    /// when `m` is 1. The power is computed by repeated truncated squaring modulo `m`, unless no
    /// coefficient of the power over the integers reaches `m`, in which case it is computed as in
    /// [`PowTruncated`](malachite_base::polynomial::PowTruncated).
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, $m$ is `m.significant_bits()`,
    /// and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowTruncatedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x+1").unwrap();
    /// p.mod_pow_truncated_assign(5, 3, &Natural::from(7u32));
    /// assert_eq!(p.to_string(), "3*x^2+5*x+1");
    ///
    /// // The power is a multiple of x^4.
    /// let mut p = NaturalPolynomial::from_str("x^2+x").unwrap();
    /// p.mod_pow_truncated_assign(4, 4, &Natural::from(7u32));
    /// assert_eq!(p.to_string(), "0");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_pow_trunc` from `fmpz_mod_poly/pow_trunc.c`, FLINT
    /// 3.6.0, except that a factor of $x^\ell$ is removed before powering, that the intermediate
    /// powers are trimmed rather than padded to `len`, that a power needing no reduction is
    /// computed over the integers, and that the zeroth power of the zero polynomial is 1 (modulo
    /// `m`), where FLINT gives 0.
    fn mod_pow_truncated_assign(&mut self, exp: u64, len: u64, m: &Natural) {
        assert_reduced(self, m);
        mod_pow_truncated_assign_helper(self, exp, len, m);
    }
}
