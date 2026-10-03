// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModIsReduced;
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{ExactFrom, SaturatingFrom};
use crate::polynomial::{ModPowTruncated, ModPowTruncatedAssign, Polynomial, pow_binexp_trimmed};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_mul_truncated::mod_mul_truncated_helper;
use crate::unsigned_polynomial::arithmetic::mod_square_truncated::mod_square_truncated_helper;
use alloc::vec;
use alloc::vec::Vec;

fn assert_reduced<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, m: T) {
    assert!(
        p.mod_is_reduced(&m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

// The coefficients, without zeros at the end, of the `e`th power modulo `m` of the polynomial with
// coefficients `xs`, which has length at least 2, a nonzero first element, and coefficients reduced
// modulo $m$, keeping only the coefficients of $x^i$ for $i$ less than `len`, which is at least 2,
// where `e` is at least 3, by binary exponentiation with truncated squaring and multiplication,
// each reduced and trimmed.
//
// This is equivalent to `_nmod_poly_pow_trunc_binexp` from `nmod_poly/pow_trunc.c`, FLINT 3.6.0, ,
// except that the polynomial is not padded to length `len` and the intermediate powers are trimmed.
crate_test_fn! {mod_pow_truncated_binexp<T: PrimitiveUnsigned>(
    xs: &[T],
    e: u64,
    len: u64,
    m: T,
) -> Vec<T> {
    pow_binexp_trimmed(
        xs,
        e,
        |r| mod_square_truncated_helper(r, len, m).into_coefficients_asc(),
        |r, xs| mod_mul_truncated_helper(r, xs, len, m).into_coefficients_asc(),
    )
}}

// The `e`th power modulo `m` of the polynomial with coefficients `xs`, which has no zeros at the
// end and coefficients reduced modulo $m$, keeping only the coefficients of $x^i$ for $i$ less than
// `len`.
//
// Writing the polynomial modulo $x^n$ as $x^\ell q$, with $q_0 \neq 0$, its power is $x^{e\ell}
// (q^e \bmod x^{n - e\ell})$, or 0 if $e\ell \geq n$.
//
// This is equivalent to `nmod_poly_pow_trunc` from `nmod_poly/pow_trunc.c`, FLINT 3.6.0, with the
// modulus $m$, except for the removal of the factor of $x^\ell$, and except that the zeroth power
// of the zero polynomial is 1, where FLINT gives 0.
fn mod_pow_truncated_helper<T: PrimitiveUnsigned>(
    xs: &[T],
    e: u64,
    len: u64,
    m: T,
) -> UnsignedPolynomial<T> {
    if m == T::ONE || len == 0 {
        return UnsignedPolynomial::ZERO;
    }
    if e == 0 {
        return UnsignedPolynomial::one();
    }
    let n = usize::saturating_from(len);
    let xs = &xs[..xs.len().min(n)];
    let Some(low) = xs.iter().position(|&x| x != T::ZERO) else {
        return UnsignedPolynomial::ZERO;
    };
    let shift = usize::saturating_from(e).saturating_mul(low);
    if shift >= n {
        return UnsignedPolynomial::ZERO;
    }
    let q = &xs[low..];
    let q_len = n - shift;
    let q_len_u64 = u64::exact_from(q_len);
    let mut power = match (q.len().min(q_len), e) {
        (1, _) => {
            let c = q[0].mod_pow(e, m);
            if c == T::ZERO { Vec::new() } else { vec![c] }
        }
        (m, 1) => {
            let mut power = q[..m].to_vec();
            while power.last() == Some(&T::ZERO) {
                power.pop();
            }
            power
        }
        (_, 2) => mod_square_truncated_helper(q, q_len_u64, m).into_coefficients_asc(),
        _ => mod_pow_truncated_binexp(q, e, q_len_u64, m),
    };
    if power.is_empty() {
        return UnsignedPolynomial::ZERO;
    }
    power.splice(0..0, core::iter::repeat_n(T::ZERO, shift));
    UnsignedPolynomial {
        coefficients: power,
    }
}

impl<T: PrimitiveUnsigned> ModPowTruncated<T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Raises an [`UnsignedPolynomial`] to a power modulo $m$, keeping only the coefficients of
    /// $x^i$ for $i$ less than `len`, taking it by value. Its coefficients must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(p, e, n, k) = (p^e \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomial need not already be truncated: only its first `len` coefficients are read.
    /// The zeroth power of every polynomial is 1, truncated to 0 when `len` is 0 and reduced to 0
    /// when $k$ is 0. The power is computed by repeated truncated squaring modulo $m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3} \log e)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u8>::from_str("x+1").unwrap())
    ///         .mod_pow_truncated(5, 3, 7)
    ///         .to_string(),
    ///     "3*x^2+5*x+1"
    /// );
    /// // The power is a multiple of x^4.
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u8>::from_str("x^2+x").unwrap())
    ///         .mod_pow_truncated(4, 4, 7)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_pow_trunc` from `nmod_poly/pow_trunc.c`, FLINT 3.6.0,
    /// except that a factor of $x^\ell$ is removed before powering, that the intermediate powers
    /// are trimmed rather than padded to `len`, and that the zeroth power of the zero polynomial is
    /// 1 (modulo $m$), where FLINT gives 0.
    #[inline]
    fn mod_pow_truncated(mut self, exp: u64, len: u64, m: T) -> Self {
        self.mod_pow_truncated_assign(exp, len, m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowTruncated<T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Raises an [`UnsignedPolynomial`] to a power modulo $m$, keeping only the coefficients of
    /// $x^i$ for $i$ less than `len`, taking it by reference. Its coefficients must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// f(p, e, n, k) = (p^e \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomial need not already be truncated: only its first `len` coefficients are read.
    /// The zeroth power of every polynomial is 1, truncated to 0 when `len` is 0 and reduced to 0
    /// when $k$ is 0. The power is computed by repeated truncated squaring modulo $m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3} \log e)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x+1").unwrap())
    ///         .mod_pow_truncated(5, 3, 7)
    ///         .to_string(),
    ///     "3*x^2+5*x+1"
    /// );
    /// // The power is a multiple of x^4.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+x").unwrap())
    ///         .mod_pow_truncated(4, 4, 7)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_pow_trunc` from `nmod_poly/pow_trunc.c`, FLINT 3.6.0,
    /// except that a factor of $x^\ell$ is removed before powering, that the intermediate powers
    /// are trimmed rather than padded to `len`, and that the zeroth power of the zero polynomial is
    /// 1 (modulo $m$), where FLINT gives 0.
    fn mod_pow_truncated(self, exp: u64, len: u64, m: T) -> UnsignedPolynomial<T> {
        assert_reduced(self, m);
        mod_pow_truncated_helper(&self.coefficients, exp, len, m)
    }
}

impl<T: PrimitiveUnsigned> ModPowTruncatedAssign<T> for UnsignedPolynomial<T> {
    /// Raises an [`UnsignedPolynomial`] to a power modulo $m$ in place, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`. Its coefficients must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// p \gets (p^e \bmod x^n) \bmod m.
    /// $$
    ///
    /// The polynomial need not already be truncated: only its first `len` coefficients are read.
    /// The zeroth power of every polynomial is 1, truncated to 0 when `len` is 0 and reduced to 0
    /// when $k$ is 0. The power is computed by repeated truncated squaring modulo $m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3} \log e)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowTruncatedAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// p.mod_pow_truncated_assign(5, 3, 7);
    /// assert_eq!(p.to_string(), "3*x^2+5*x+1");
    ///
    /// // The power is a multiple of x^4.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+x").unwrap();
    /// p.mod_pow_truncated_assign(4, 4, 7);
    /// assert_eq!(p.to_string(), "0");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_pow_trunc` from `nmod_poly/pow_trunc.c`, FLINT 3.6.0,
    /// except that a factor of $x^\ell$ is removed before powering, that the intermediate powers
    /// are trimmed rather than padded to `len`, and that the zeroth power of the zero polynomial is
    /// 1 (modulo $m$), where FLINT gives 0.
    fn mod_pow_truncated_assign(&mut self, exp: u64, len: u64, m: T) {
        assert_reduced(self, m);
        let xs = &mut self.coefficients;
        match (xs.len(), exp, len, m == T::ONE) {
            (_, _, 0, _) | (_, _, _, true) => xs.clear(),
            (0, 0, _, _) => xs.push(T::ONE),
            (_, 0, _, _) => {
                xs.truncate(1);
                xs[0] = T::ONE;
            }
            (0, _, _, _) => {}
            (_, 1, _, _) => self.truncate_assign(len),
            (1, _, _, _) => {
                xs[0].mod_pow_assign(exp, m);
                if xs[0] == T::ZERO {
                    xs.clear();
                }
            }
            _ => *self = mod_pow_truncated_helper(xs, exp, len, m),
        }
    }
}
