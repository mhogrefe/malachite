// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2IsReduced, ModPowerOf2Pow, ModPowerOf2PowAssign};
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::polynomial::{Polynomial, pow_binexp_trimmed};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::mod_power_of_2_mul_helper;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_square::mod_power_of_2_square_helper;
use alloc::vec;
use alloc::vec::Vec;

fn assert_reduced<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, pow: u64) {
    assert!(pow <= T::WIDTH);
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
}

// The coefficients, without zeros at the end, of the `e`th power modulo $2^k$, where $k$ is `pow`,
// of the polynomial with coefficients `xs`, which has length at least 2, nonzero first and last
// elements, and coefficients reduced modulo $2^k$, where `e` is at least 3, by binary
// exponentiation: each square and product is reduced and trimmed, so the intermediate powers shrink
// when leading coefficients vanish modulo $2^k$.
//
// This is equivalent to `_nmod_poly_pow_binexp` from `nmod_poly/pow_binexp.c`, FLINT 3.6.0, with
// the modulus $2^k$, except that the intermediate powers are trimmed.
crate_test_fn! {mod_power_of_2_pow_binexp<T: PrimitiveUnsigned>(
    xs: &[T],
    e: u64,
    pow: u64,
) -> Vec<T> {
    pow_binexp_trimmed(
        xs,
        e,
        |r| mod_power_of_2_square_helper(r, pow).into_coefficients_asc(),
        |r, xs| mod_power_of_2_mul_helper(r, xs, pow).into_coefficients_asc(),
    )
}}

// The `e`th power modulo $2^k$, where $k$ is `pow`, of the polynomial with coefficients `xs`, which
// has no zeros at the end and coefficients reduced modulo $2^k$.
//
// Writing the polynomial as $x^\ell q$, with $q_0 \neq 0$, its power is $x^{e\ell} q^e$.
//
// This is equivalent to `nmod_poly_pow` from `nmod_poly/pow.c`, FLINT 3.6.0, with the modulus
// $2^k$, except for the removal of the factor of $x^\ell$.
fn mod_power_of_2_pow_helper<T: PrimitiveUnsigned>(
    xs: &[T],
    e: u64,
    pow: u64,
) -> UnsignedPolynomial<T> {
    if pow == 0 {
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
            let c = q[0].mod_power_of_2_pow(e, pow);
            if c == T::ZERO { Vec::new() } else { vec![c] }
        }
        (_, 1) => q.to_vec(),
        (_, 2) => mod_power_of_2_square_helper(q, pow).into_coefficients_asc(),
        _ => mod_power_of_2_pow_binexp(q, e, pow),
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

impl<T: PrimitiveUnsigned> ModPowerOf2Pow<u64> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Raises an [`UnsignedPolynomial`] to a power modulo $2^k$, taking it by value. Its
    /// coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, e, k) = p^e \bmod 2^k.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1, which is 0 modulo $2^0$. The
    /// leading coefficients of a power can vanish modulo $2^k$, and then its degree is lower than
    /// $e$ times the degree of the polynomial. The power is computed by repeated squaring modulo
    /// $2^k$.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Pow;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u8>::from_str("x+1").unwrap())
    ///         .mod_power_of_2_pow(5, 3)
    ///         .to_string(),
    ///     "x^5+5*x^4+2*x^3+2*x^2+5*x+1"
    /// );
    /// // The square of 2*x+1 is 4*x^2+4*x+1, which is 1 modulo 4.
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap())
    ///         .mod_power_of_2_pow(2, 2)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_pow` from `nmod_poly/pow.c`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that a factor of $x^\ell$ is removed before powering and that the intermediate
    /// powers are trimmed.
    #[inline]
    fn mod_power_of_2_pow(mut self, exp: u64, pow: u64) -> Self {
        self.mod_power_of_2_pow_assign(exp, pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Pow<u64> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Raises an [`UnsignedPolynomial`] to a power modulo $2^k$, taking it by reference. Its
    /// coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, e, k) = p^e \bmod 2^k.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1, which is 0 modulo $2^0$. The
    /// leading coefficients of a power can vanish modulo $2^k$, and then its degree is lower than
    /// $e$ times the degree of the polynomial. The power is computed by repeated squaring modulo
    /// $2^k$.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Pow;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x+1").unwrap())
    ///         .mod_power_of_2_pow(5, 3)
    ///         .to_string(),
    ///     "x^5+5*x^4+2*x^3+2*x^2+5*x+1"
    /// );
    /// // The square of 2*x+1 is 4*x^2+4*x+1, which is 1 modulo 4.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap())
    ///         .mod_power_of_2_pow(2, 2)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_pow` from `nmod_poly/pow.c`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that a factor of $x^\ell$ is removed before powering and that the intermediate
    /// powers are trimmed.
    fn mod_power_of_2_pow(self, exp: u64, pow: u64) -> UnsignedPolynomial<T> {
        assert_reduced(self, pow);
        mod_power_of_2_pow_helper(&self.coefficients, exp, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2PowAssign<u64> for UnsignedPolynomial<T> {
    /// Raises an [`UnsignedPolynomial`] to a power modulo $2^k$ in place. Its coefficients must
    /// already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \gets p^e \bmod 2^k.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1, which is 0 modulo $2^0$. The
    /// leading coefficients of a power can vanish modulo $2^k$, and then its degree is lower than
    /// $e$ times the degree of the polynomial. The power is computed by repeated squaring modulo
    /// $2^k$.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2PowAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// p.mod_power_of_2_pow_assign(5, 3);
    /// assert_eq!(p.to_string(), "x^5+5*x^4+2*x^3+2*x^2+5*x+1");
    ///
    /// // The square of 2*x+1 is 4*x^2+4*x+1, which is 1 modulo 4.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// p.mod_power_of_2_pow_assign(2, 2);
    /// assert_eq!(p.to_string(), "1");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_pow` from `nmod_poly/pow.c`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that a factor of $x^\ell$ is removed before powering and that the intermediate
    /// powers are trimmed.
    fn mod_power_of_2_pow_assign(&mut self, exp: u64, pow: u64) {
        assert_reduced(self, pow);
        let xs = &mut self.coefficients;
        match (xs.len(), exp, pow) {
            (_, _, 0) => xs.clear(),
            (0, 0, _) => xs.push(T::ONE),
            (_, 0, _) => {
                xs.truncate(1);
                xs[0] = T::ONE;
            }
            (0, _, _) | (_, 1, _) => {}
            (1, _, _) => {
                xs[0].mod_power_of_2_pow_assign(exp, pow);
                if xs[0] == T::ZERO {
                    xs.clear();
                }
            }
            _ => *self = mod_power_of_2_pow_helper(xs, exp, pow),
        }
    }
}
