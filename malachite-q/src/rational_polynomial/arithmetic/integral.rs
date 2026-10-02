// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use alloc::vec;
use alloc::vec::Vec;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign, Gcd};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{Integral, IntegralAssign, Polynomial};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;

// The GCD of $|x|$ and $k$.
fn gcd_with_small(x: &Natural, k: u64) -> u64 {
    u64::exact_from(&(x % Natural::from(k))).gcd(k)
}

// Integrates in place the polynomial whose numerator coefficients follow a zero in `coefficients`,
// so that `coefficients[k]` holds the coefficient of $x^{k-1}$ of the integrand for $k \geq 1$,
// over `denominator`, which together are canonical. Each coefficient moved to $x^k$ must be divided
// by $k$: it is divided by its GCD with $k$, and what is left of $k$ goes into $t$, the least
// common multiple of the leftover divisors. Then the denominator is multiplied by $t$ and each
// coefficient by $t$ divided by its leftover divisor. The result is canonical without a further
// GCD: for a prime dividing $t$, the coefficient whose leftover divisor has the highest power of it
// is not divisible by it, and a prime dividing only the original denominator already failed to
// divide some coefficient.
//
// This is equivalent to `_fmpq_poly_integral` from `fmpq_poly/integral.c`, FLINT 3.6.0.
fn integral_in_place(coefficients: &mut [Integer], denominator: &mut Natural) {
    let len = coefficients.len();
    let mut divisors = vec![1u64; len];
    let mut t = Natural::ONE;
    for (k, c) in coefficients.iter_mut().enumerate().skip(2).rev() {
        if *c == 0u32 {
            continue;
        }
        let k = u64::exact_from(k);
        let g = gcd_with_small(c.unsigned_abs_ref(), k);
        if g == k {
            c.div_exact_assign(Integer::from(k));
        } else {
            if g != 1 {
                c.div_exact_assign(Integer::from(g));
            }
            let divisor = k / g;
            divisors[usize::exact_from(k)] = divisor;
            let d = gcd_with_small(&t, divisor);
            if d != divisor {
                t *= Natural::from(divisor / d);
            }
        }
    }
    if t != 1u32 {
        for (c, &divisor) in coefficients.iter_mut().zip(&divisors).skip(2) {
            if *c != 0u32 {
                *c *= Integer::from((&t).div_exact(Natural::from(divisor)));
            }
        }
        coefficients[1] *= Integer::from(&t);
        *denominator *= t;
    }
}

// The numerator coefficients of the integrand behind a zero, ready for `integral_in_place`.
fn shifted(mut coefficients: Vec<Integer>) -> Vec<Integer> {
    coefficients.insert(0, Integer::ZERO);
    coefficients
}

impl Integral for RationalPolynomial {
    type Output = Self;

    /// Computes the integral of a [`RationalPolynomial`] whose constant term is zero, taking it by
    /// value.
    ///
    /// $$
    /// f(p) = \int_0^x p(t)\,dt = \sum_{i=0}^{n-1} \frac{a_i}{i+1}x^{i+1}.
    /// $$
    ///
    /// The constant of integration is zero. The result is built in lowest terms coefficient by
    /// coefficient: each numerator coefficient $c$ that is divided by $k$ first cancels $\gcd(c,
    /// k)$, and the denominator is multiplied by the least common multiple of what remains of the
    /// divisors, so no GCD of the whole polynomial is needed. The integral of zero is zero.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + n) \log (m + n) \log\log (m + n))$
    ///
    /// $M(n, m) = O(n (m + n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any numerator coefficient or of the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::Integral;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("3*x^2+x+1/2").unwrap();
    /// assert_eq!(p.integral().to_string(), "x^3+1/2*x^2+1/2*x");
    ///
    /// // Each divisor shares what it can with its coefficient; the rest goes to the denominator.
    /// let p = RationalPolynomial::from_str("1/3*x^2+2*x+1").unwrap();
    /// assert_eq!(p.integral().to_string(), "1/9*x^3+x^2+x");
    ///
    /// let p = RationalPolynomial::from_str("0").unwrap();
    /// assert_eq!(p.integral(), RationalPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_integral` from `fmpq_poly/integral.c`, FLINT 3.6.0.
    #[inline]
    fn integral(mut self) -> Self {
        self.integral_assign();
        self
    }
}

impl Integral for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Computes the integral of a [`RationalPolynomial`] whose constant term is zero, taking it by
    /// reference.
    ///
    /// $$
    /// f(p) = \int_0^x p(t)\,dt = \sum_{i=0}^{n-1} \frac{a_i}{i+1}x^{i+1}.
    /// $$
    ///
    /// The constant of integration is zero. The result is built in lowest terms coefficient by
    /// coefficient: each numerator coefficient $c$ that is divided by $k$ first cancels $\gcd(c,
    /// k)$, and the denominator is multiplied by the least common multiple of what remains of the
    /// divisors, so no GCD of the whole polynomial is needed. The integral of zero is zero.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + n) \log (m + n) \log\log (m + n))$
    ///
    /// $M(n, m) = O(n (m + n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any numerator coefficient or of the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::Integral;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("3*x^2+x+1/2").unwrap();
    /// assert_eq!((&p).integral().to_string(), "x^3+1/2*x^2+1/2*x");
    ///
    /// // Each divisor shares what it can with its coefficient; the rest goes to the denominator.
    /// let p = RationalPolynomial::from_str("1/3*x^2+2*x+1").unwrap();
    /// assert_eq!((&p).integral().to_string(), "1/9*x^3+x^2+x");
    ///
    /// let p = RationalPolynomial::from_str("0").unwrap();
    /// assert_eq!((&p).integral(), RationalPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_integral` from `fmpq_poly/integral.c`, FLINT 3.6.0.
    fn integral(self) -> RationalPolynomial {
        if *self == RationalPolynomial::ZERO {
            return RationalPolynomial::ZERO;
        }
        let mut coefficients = shifted(self.numerator.coefficients_asc().to_vec());
        let mut denominator = self.denominator.clone();
        integral_in_place(&mut coefficients, &mut denominator);
        RationalPolynomial {
            numerator: IntegerPolynomial::from_coefficients_asc(coefficients),
            denominator,
        }
    }
}

impl IntegralAssign for RationalPolynomial {
    /// Replaces a [`RationalPolynomial`] with its integral whose constant term is zero.
    ///
    /// $$
    /// p \gets \int_0^x p(t)\,dt = \sum_{i=0}^{n-1} \frac{a_i}{i+1}x^{i+1}.
    /// $$
    ///
    /// The constant of integration is zero. The result is built in lowest terms coefficient by
    /// coefficient: each numerator coefficient $c$ that is divided by $k$ first cancels $\gcd(c,
    /// k)$, and the denominator is multiplied by the least common multiple of what remains of the
    /// divisors, so no GCD of the whole polynomial is needed. The integral of zero is zero.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + n) \log (m + n) \log\log (m + n))$
    ///
    /// $M(n, m) = O(n (m + n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any numerator coefficient or of the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::IntegralAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("3*x^2+x+1/2").unwrap();
    /// p.integral_assign();
    /// assert_eq!(p.to_string(), "x^3+1/2*x^2+1/2*x");
    ///
    /// // Each divisor shares what it can with its coefficient; the rest goes to the denominator.
    /// let mut p = RationalPolynomial::from_str("1/3*x^2+2*x+1").unwrap();
    /// p.integral_assign();
    /// assert_eq!(p.to_string(), "1/9*x^3+x^2+x");
    ///
    /// let mut p = RationalPolynomial::ZERO;
    /// p.integral_assign();
    /// assert_eq!(p, RationalPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_integral` from `fmpq_poly/integral.c`, FLINT 3.6.0.
    fn integral_assign(&mut self) {
        if *self == Self::ZERO {
            return;
        }
        let mut coefficients = shifted(take(&mut self.numerator).into_coefficients_asc());
        integral_in_place(&mut coefficients, &mut self.denominator);
        self.numerator = IntegerPolynomial::from_coefficients_asc(coefficients);
    }
}
