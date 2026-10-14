// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::{RationalPolynomial, reduce};
use core::mem::take;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::traits::One;
use malachite_base::polynomial::{Polynomial, TaylorShift, TaylorShiftAssign};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;

// Shifts the polynomial `numerator / denominator`, which is in lowest terms, by `c`.
//
// With `c = a / b`, the numerator N of degree n is scaled to M(x) = b^n N(x/b), M is shifted by the
// integer a, and x is scaled back by b: p(x + a/b) = M(bx + a) / (b^n d). The powers of b are kept
// one at a time, so that no table of them is needed.
fn taylor_shift_helper(
    mut numerator: IntegerPolynomial,
    denominator: Natural,
    c: &Rational,
) -> RationalPolynomial {
    if *c == 0u32 || numerator.len() <= 1 {
        return RationalPolynomial {
            numerator,
            denominator,
        };
    }
    let a = Integer::from_sign_and_abs_ref(*c > 0u32, c.numerator_ref());
    let b = c.denominator_ref();
    if *b == 1u32 {
        // A shift by an integer is invertible over the integers, so it does not change the GCD of
        // the numerator's coefficients, and the result is in lowest terms.
        numerator.taylor_shift_assign(&a);
        return RationalPolynomial {
            numerator,
            denominator,
        };
    }
    let b = Integer::from(b);
    // The coefficient of x^i becomes N_i b^(n - i).
    let mut xs = numerator.into_coefficients_asc();
    let mut power = Integer::ONE;
    for x in xs.iter_mut().rev().skip(1) {
        power *= &b;
        *x *= &power;
    }
    let mut scaled = IntegerPolynomial::from_coefficients_asc(xs);
    scaled.taylor_shift_assign(&a);
    // The coefficient of x^i becomes T_i b^i, over the denominator d b^n.
    let mut xs = scaled.into_coefficients_asc();
    power = Integer::ONE;
    for x in xs.iter_mut().skip(1) {
        power *= &b;
        *x *= &power;
    }
    reduce(
        IntegerPolynomial::from_coefficients_asc(xs),
        denominator * power.unsigned_abs(),
    )
}

impl TaylorShift<&Rational> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Shifts a [`RationalPolynomial`] by a [`Rational`] $c$, replacing $x$ with $x + c$, taking
    /// both by reference.
    ///
    /// $$
    /// f(p, c) = p(x + c).
    /// $$
    ///
    /// When $c$ is an integer only the numerator is shifted, and the result is already in lowest
    /// terms, since shifting by an integer does not change the GCD of the numerator's coefficients.
    /// Otherwise, with $p = N/d$ and $c = a/b$ in lowest terms, the numerator is scaled to $M(x) =
    /// b^nN(x/b)$, where $n$ is the degree, which has integer coefficients; then $p(x + c) = M(bx +
    /// a)/(b^nd)$, and $M(bx + a)$ is the shift of $M$ by the integer $a$ with $x$ scaled by $b$.
    /// So the work is one shift of an integer polynomial, followed by putting the result in lowest
    /// terms.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + nm (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any coefficient of the numerator, plus the number of bits of
    /// the denominator, plus $n$ times `c.significant_bits()`, a bound on the size of the numerator
    /// of the result: the first term is the shift of an integer polynomial and the second the GCDs
    /// that put the result in lowest terms.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, OneHalf};
    /// use malachite_base::polynomial::TaylorShift;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("x^2-3*x+2").unwrap();
    /// assert_eq!((&p).taylor_shift(&Rational::ONE).to_string(), "x^2-x");
    /// assert_eq!(
    ///     (&p).taylor_shift(&Rational::ONE_HALF).to_string(),
    ///     "x^2-2*x+3/4"
    /// );
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2").unwrap();
    /// assert_eq!(
    ///     (&p).taylor_shift(&Rational::from_signeds(-1, 3))
    ///         .to_string(),
    ///     "1/2*x^2-1/3*x+1/18"
    /// );
    /// ```
    #[inline]
    fn taylor_shift(self, c: &Rational) -> RationalPolynomial {
        taylor_shift_helper(self.numerator.clone(), self.denominator.clone(), c)
    }
}

impl TaylorShift<Rational> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Shifts a [`RationalPolynomial`] by a [`Rational`] $c$, replacing $x$ with $x + c$, taking
    /// the polynomial by reference and $c$ by value.
    ///
    /// $$
    /// f(p, c) = p(x + c).
    /// $$
    ///
    /// When $c$ is an integer only the numerator is shifted, and the result is already in lowest
    /// terms, since shifting by an integer does not change the GCD of the numerator's coefficients.
    /// Otherwise, with $p = N/d$ and $c = a/b$ in lowest terms, the numerator is scaled to $M(x) =
    /// b^nN(x/b)$, where $n$ is the degree, which has integer coefficients; then $p(x + c) = M(bx +
    /// a)/(b^nd)$, and $M(bx + a)$ is the shift of $M$ by the integer $a$ with $x$ scaled by $b$.
    /// So the work is one shift of an integer polynomial, followed by putting the result in lowest
    /// terms.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + nm (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any coefficient of the numerator, plus the number of bits of
    /// the denominator, plus $n$ times `c.significant_bits()`, a bound on the size of the numerator
    /// of the result: the first term is the shift of an integer polynomial and the second the GCDs
    /// that put the result in lowest terms.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, OneHalf};
    /// use malachite_base::polynomial::TaylorShift;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("x^2-3*x+2").unwrap();
    /// assert_eq!((&p).taylor_shift(Rational::ONE).to_string(), "x^2-x");
    /// assert_eq!(
    ///     (&p).taylor_shift(Rational::ONE_HALF).to_string(),
    ///     "x^2-2*x+3/4"
    /// );
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2").unwrap();
    /// assert_eq!(
    ///     (&p).taylor_shift(Rational::from_signeds(-1, 3)).to_string(),
    ///     "1/2*x^2-1/3*x+1/18"
    /// );
    /// ```
    #[inline]
    fn taylor_shift(self, c: Rational) -> RationalPolynomial {
        taylor_shift_helper(self.numerator.clone(), self.denominator.clone(), &c)
    }
}

impl TaylorShift<&Rational> for RationalPolynomial {
    type Output = Self;

    /// Shifts a [`RationalPolynomial`] by a [`Rational`] $c$, replacing $x$ with $x + c$, taking
    /// the polynomial by value and $c$ by reference.
    ///
    /// $$
    /// f(p, c) = p(x + c).
    /// $$
    ///
    /// When $c$ is an integer only the numerator is shifted, and the result is already in lowest
    /// terms, since shifting by an integer does not change the GCD of the numerator's coefficients.
    /// Otherwise, with $p = N/d$ and $c = a/b$ in lowest terms, the numerator is scaled to $M(x) =
    /// b^nN(x/b)$, where $n$ is the degree, which has integer coefficients; then $p(x + c) = M(bx +
    /// a)/(b^nd)$, and $M(bx + a)$ is the shift of $M$ by the integer $a$ with $x$ scaled by $b$.
    /// So the work is one shift of an integer polynomial, followed by putting the result in lowest
    /// terms.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + nm (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any coefficient of the numerator, plus the number of bits of
    /// the denominator, plus $n$ times `c.significant_bits()`, a bound on the size of the numerator
    /// of the result: the first term is the shift of an integer polynomial and the second the GCDs
    /// that put the result in lowest terms.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, OneHalf};
    /// use malachite_base::polynomial::TaylorShift;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("x^2-3*x+2").unwrap();
    /// assert_eq!(p.clone().taylor_shift(&Rational::ONE).to_string(), "x^2-x");
    /// assert_eq!(
    ///     p.clone().taylor_shift(&Rational::ONE_HALF).to_string(),
    ///     "x^2-2*x+3/4"
    /// );
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2").unwrap();
    /// assert_eq!(
    ///     p.clone()
    ///         .taylor_shift(&Rational::from_signeds(-1, 3))
    ///         .to_string(),
    ///     "1/2*x^2-1/3*x+1/18"
    /// );
    /// ```
    #[inline]
    fn taylor_shift(self, c: &Rational) -> Self {
        taylor_shift_helper(self.numerator, self.denominator, c)
    }
}

impl TaylorShift<Rational> for RationalPolynomial {
    type Output = Self;

    /// Shifts a [`RationalPolynomial`] by a [`Rational`] $c$, replacing $x$ with $x + c$, taking
    /// both by value.
    ///
    /// $$
    /// f(p, c) = p(x + c).
    /// $$
    ///
    /// When $c$ is an integer only the numerator is shifted, and the result is already in lowest
    /// terms, since shifting by an integer does not change the GCD of the numerator's coefficients.
    /// Otherwise, with $p = N/d$ and $c = a/b$ in lowest terms, the numerator is scaled to $M(x) =
    /// b^nN(x/b)$, where $n$ is the degree, which has integer coefficients; then $p(x + c) = M(bx +
    /// a)/(b^nd)$, and $M(bx + a)$ is the shift of $M$ by the integer $a$ with $x$ scaled by $b$.
    /// So the work is one shift of an integer polynomial, followed by putting the result in lowest
    /// terms.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + nm (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any coefficient of the numerator, plus the number of bits of
    /// the denominator, plus $n$ times `c.significant_bits()`, a bound on the size of the numerator
    /// of the result: the first term is the shift of an integer polynomial and the second the GCDs
    /// that put the result in lowest terms.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, OneHalf};
    /// use malachite_base::polynomial::TaylorShift;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("x^2-3*x+2").unwrap();
    /// assert_eq!(p.clone().taylor_shift(Rational::ONE).to_string(), "x^2-x");
    /// assert_eq!(
    ///     p.clone().taylor_shift(Rational::ONE_HALF).to_string(),
    ///     "x^2-2*x+3/4"
    /// );
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2").unwrap();
    /// assert_eq!(
    ///     p.clone()
    ///         .taylor_shift(Rational::from_signeds(-1, 3))
    ///         .to_string(),
    ///     "1/2*x^2-1/3*x+1/18"
    /// );
    /// ```
    #[inline]
    fn taylor_shift(self, c: Rational) -> Self {
        taylor_shift_helper(self.numerator, self.denominator, &c)
    }
}

impl TaylorShiftAssign<&Rational> for RationalPolynomial {
    /// Shifts a [`RationalPolynomial`] by a [`Rational`] $c$ in place, replacing $x$ with $x + c$,
    /// taking $c$ by reference.
    ///
    /// $$
    /// p \\gets p(x + c).
    /// $$
    ///
    /// When $c$ is an integer only the numerator is shifted, and the result is already in lowest
    /// terms, since shifting by an integer does not change the GCD of the numerator's coefficients.
    /// Otherwise, with $p = N/d$ and $c = a/b$ in lowest terms, the numerator is scaled to $M(x) =
    /// b^nN(x/b)$, where $n$ is the degree, which has integer coefficients; then $p(x + c) = M(bx +
    /// a)/(b^nd)$, and $M(bx + a)$ is the shift of $M$ by the integer $a$ with $x$ scaled by $b$.
    /// So the work is one shift of an integer polynomial, followed by putting the result in lowest
    /// terms.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + nm (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any coefficient of the numerator, plus the number of bits of
    /// the denominator, plus $n$ times `c.significant_bits()`, a bound on the size of the numerator
    /// of the result: the first term is the shift of an integer polynomial and the second the GCDs
    /// that put the result in lowest terms.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::polynomial::TaylorShiftAssign;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("x^2-3*x+2").unwrap();
    /// p.taylor_shift_assign(&Rational::ONE_HALF);
    /// assert_eq!(p.to_string(), "x^2-2*x+3/4");
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^2").unwrap();
    /// p.taylor_shift_assign(&Rational::from_signeds(-1, 3));
    /// assert_eq!(p.to_string(), "1/2*x^2-1/3*x+1/18");
    /// ```
    fn taylor_shift_assign(&mut self, c: &Rational) {
        *self = taylor_shift_helper(take(&mut self.numerator), take(&mut self.denominator), c);
    }
}

impl TaylorShiftAssign<Rational> for RationalPolynomial {
    /// Shifts a [`RationalPolynomial`] by a [`Rational`] $c$ in place, replacing $x$ with $x + c$,
    /// taking $c$ by value.
    ///
    /// $$
    /// p \\gets p(x + c).
    /// $$
    ///
    /// When $c$ is an integer only the numerator is shifted, and the result is already in lowest
    /// terms, since shifting by an integer does not change the GCD of the numerator's coefficients.
    /// Otherwise, with $p = N/d$ and $c = a/b$ in lowest terms, the numerator is scaled to $M(x) =
    /// b^nN(x/b)$, where $n$ is the degree, which has integer coefficients; then $p(x + c) = M(bx +
    /// a)/(b^nd)$, and $M(bx + a)$ is the shift of $M$ by the integer $a$ with $x$ scaled by $b$.
    /// So the work is one shift of an integer polynomial, followed by putting the result in lowest
    /// terms.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + nm (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any coefficient of the numerator, plus the number of bits of
    /// the denominator, plus $n$ times `c.significant_bits()`, a bound on the size of the numerator
    /// of the result: the first term is the shift of an integer polynomial and the second the GCDs
    /// that put the result in lowest terms.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::polynomial::TaylorShiftAssign;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("x^2-3*x+2").unwrap();
    /// p.taylor_shift_assign(Rational::ONE_HALF);
    /// assert_eq!(p.to_string(), "x^2-2*x+3/4");
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^2").unwrap();
    /// p.taylor_shift_assign(Rational::from_signeds(-1, 3));
    /// assert_eq!(p.to_string(), "1/2*x^2-1/3*x+1/18");
    /// ```
    #[inline]
    fn taylor_shift_assign(&mut self, c: Rational) {
        self.taylor_shift_assign(&c);
    }
}
