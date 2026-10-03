// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModIsReduced, ModPow, ModPowAssign};
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::polynomial::{Polynomial, pow_binexp_trimmed};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_mul::mod_mul_helper;
use crate::unsigned_polynomial::arithmetic::mod_square::mod_square_helper;
use alloc::vec;
use alloc::vec::Vec;

fn assert_reduced<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, m: T) {
    assert!(
        p.mod_is_reduced(&m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

// The coefficients, without zeros at the end, of the `e`th power modulo `m` of the polynomial with
// coefficients `xs`, which has length at least 2, nonzero first and last elements, and coefficients
// reduced modulo $m$, where `e` is at least 3, by binary exponentiation: each square and product is
// reduced and trimmed, so the intermediate powers shrink when leading coefficients vanish modulo
// $m$.
//
// This is equivalent to `_nmod_poly_pow_binexp` from `nmod_poly/pow_binexp.c`, FLINT 3.6.0, except
// that the intermediate powers are trimmed.
crate_test_fn! {mod_pow_binexp<T: PrimitiveUnsigned>(
    xs: &[T],
    e: u64,
    m: T,
) -> Vec<T> {
    pow_binexp_trimmed(
        xs,
        e,
        |r| mod_square_helper(r, m).into_coefficients_asc(),
        |r, xs| mod_mul_helper(r, xs, m).into_coefficients_asc(),
    )
}}

// The `e`th power modulo `m` of the polynomial with coefficients `xs`, which has no zeros at the
// end and coefficients reduced modulo $m$.
//
// Writing the polynomial as $x^\ell q$, with $q_0 \neq 0$, its power is $x^{e\ell} q^e$.
//
// This is equivalent to `nmod_poly_pow` from `nmod_poly/pow.c`, FLINT 3.6.0, except for the removal
// of the factor of $x^\ell$.
fn mod_pow_helper<T: PrimitiveUnsigned>(xs: &[T], e: u64, m: T) -> UnsignedPolynomial<T> {
    if m == T::ONE {
        return UnsignedPolynomial::ZERO;
    }
    if e == 0 {
        return UnsignedPolynomial::one();
    }
    let Some(low) = xs.iter().position(|&x| x != T::ZERO) else {
        return UnsignedPolynomial::ZERO;
    };
    let q = &xs[low..];
    let mut power = match (q.len(), e) {
        (1, _) => {
            let c = q[0].mod_pow(e, m);
            if c == T::ZERO { Vec::new() } else { vec![c] }
        }
        (_, 1) => q.to_vec(),
        (_, 2) => mod_square_helper(q, m).into_coefficients_asc(),
        _ => mod_pow_binexp(q, e, m),
    };
    if power.is_empty() {
        return UnsignedPolynomial::ZERO;
    }
    if low != 0 {
        let shift = usize::exact_from(e)
            .checked_mul(low)
            .expect("the power has too many coefficients to represent");
        power.splice(0..0, core::iter::repeat_n(T::ZERO, shift));
    }
    UnsignedPolynomial {
        coefficients: power,
    }
}

impl<T: PrimitiveUnsigned> ModPow<u64, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Raises an [`UnsignedPolynomial`] to a power modulo $m$, taking it by value. Its coefficients
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, e, k) = p^e \bmod m.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1, which is 0 modulo 1. When `m` is
    /// not prime, the leading coefficients of a power can vanish modulo `m`, and then its degree is
    /// lower than $e$ times the degree of the polynomial. The power is computed by repeated
    /// squaring modulo $m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3} \log e)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u8>::from_str("x+1").unwrap())
    ///         .mod_pow(5, 7)
    ///         .to_string(),
    ///     "x^5+5*x^4+3*x^3+3*x^2+5*x+1"
    /// );
    /// // The square of 2*x+1 is 4*x^2+4*x+1, which is 1 modulo 4.
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap())
    ///         .mod_pow(2, 4)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_pow` from `nmod_poly/pow.c`, FLINT 3.6.0, except that a
    /// factor of $x^\ell$ is removed before powering and that the intermediate powers are trimmed.
    #[inline]
    fn mod_pow(mut self, exp: u64, m: T) -> Self {
        self.mod_pow_assign(exp, m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPow<u64, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Raises an [`UnsignedPolynomial`] to a power modulo $m$, taking it by reference. Its
    /// coefficients must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, e, k) = p^e \bmod m.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1, which is 0 modulo 1. When `m` is
    /// not prime, the leading coefficients of a power can vanish modulo `m`, and then its degree is
    /// lower than $e$ times the degree of the polynomial. The power is computed by repeated
    /// squaring modulo $m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3} \log e)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x+1").unwrap())
    ///         .mod_pow(5, 7)
    ///         .to_string(),
    ///     "x^5+5*x^4+3*x^3+3*x^2+5*x+1"
    /// );
    /// // The square of 2*x+1 is 4*x^2+4*x+1, which is 1 modulo 4.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap())
    ///         .mod_pow(2, 4)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_pow` from `nmod_poly/pow.c`, FLINT 3.6.0, except that a
    /// factor of $x^\ell$ is removed before powering and that the intermediate powers are trimmed.
    fn mod_pow(self, exp: u64, m: T) -> UnsignedPolynomial<T> {
        assert_reduced(self, m);
        mod_pow_helper(&self.coefficients, exp, m)
    }
}

impl<T: PrimitiveUnsigned> ModPowAssign<u64, T> for UnsignedPolynomial<T> {
    /// Raises an [`UnsignedPolynomial`] to a power modulo $m$ in place. Its coefficients must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// p \gets p^e \bmod m.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1, which is 0 modulo 1. When `m` is
    /// not prime, the leading coefficients of a power can vanish modulo `m`, and then its degree is
    /// lower than $e$ times the degree of the polynomial. The power is computed by repeated
    /// squaring modulo $m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3} \log e)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// p.mod_pow_assign(5, 7);
    /// assert_eq!(p.to_string(), "x^5+5*x^4+3*x^3+3*x^2+5*x+1");
    ///
    /// // The square of 2*x+1 is 4*x^2+4*x+1, which is 1 modulo 4.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// p.mod_pow_assign(2, 4);
    /// assert_eq!(p.to_string(), "1");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_pow` from `nmod_poly/pow.c`, FLINT 3.6.0, except that a
    /// factor of $x^\ell$ is removed before powering and that the intermediate powers are trimmed.
    fn mod_pow_assign(&mut self, exp: u64, m: T) {
        assert_reduced(self, m);
        let xs = &mut self.coefficients;
        match (xs.len(), exp, m == T::ONE) {
            (_, _, true) => xs.clear(),
            (0, 0, _) => xs.push(T::ONE),
            (_, 0, _) => {
                xs.truncate(1);
                xs[0] = T::ONE;
            }
            (0, _, _) | (_, 1, _) => {}
            (1, _, _) => {
                xs[0].mod_pow_assign(exp, m);
                if xs[0] == T::ZERO {
                    xs.clear();
                }
            }
            _ => *self = mod_pow_helper(xs, exp, m),
        }
    }
}
