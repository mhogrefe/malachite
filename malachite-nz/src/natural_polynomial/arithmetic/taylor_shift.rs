// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::taylor_shift::taylor_shift_in_place;
use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::polynomial::{TaylorShift, TaylorShiftAssign};

// Shifts the coefficients `xs` by `c` with the `Integer` kernel. A `Natural` becomes an `Integer`,
// and a non-negative `Integer` becomes a `Natural` again, without copying its limbs, and a shift by
// a non-negative `c` leaves every coefficient non-negative.
fn taylor_shift_naturals(xs: Vec<Natural>, c: &Natural) -> Vec<Natural> {
    let mut ys: Vec<Integer> = xs.into_iter().map(Integer::from).collect();
    taylor_shift_in_place(&mut ys, &Integer::from(c));
    ys.into_iter().map(UnsignedAbs::unsigned_abs).collect()
}

impl TaylorShift<&Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$, replacing $x$ with $x + c$, taking both
    /// by reference.
    ///
    /// $$
    /// f(p, c) = p(x + c).
    /// $$
    ///
    /// The coefficient of $x^k$ in the result is $\sum_{i \geq k} \binom{i}{k} p_i c^{i-k}$, where
    /// $p_i$ is the coefficient of $x^i$ in $p$; since $c$ is non-negative, so is every coefficient
    /// of the result. The shift is done as for an
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial): Horner's rule for short
    /// polynomials, and divide and conquer for long ones.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any coefficient plus $n$ times `c.significant_bits()`, a bound
    /// on the size of the coefficients of the result.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::TaylorShift;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!((&p).taylor_shift(&Natural::ONE).to_string(), "x^2+5*x+6");
    /// assert_eq!(
    ///     (&p).taylor_shift(&Natural::from(10u32)).to_string(),
    ///     "x^2+23*x+132"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3").unwrap();
    /// assert_eq!(
    ///     (&p).taylor_shift(&Natural::ONE).to_string(),
    ///     "x^3+3*x^2+3*x+1"
    /// );
    /// ```
    fn taylor_shift(self, c: &Natural) -> NaturalPolynomial {
        let mut ys: Vec<Integer> = self.coefficients.iter().map(Integer::from).collect();
        taylor_shift_in_place(&mut ys, &Integer::from(c));
        NaturalPolynomial {
            coefficients: ys.into_iter().map(UnsignedAbs::unsigned_abs).collect(),
        }
    }
}

impl TaylorShift<Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$, replacing $x$ with $x + c$, taking the
    /// polynomial by reference and $c$ by value.
    ///
    /// $$
    /// f(p, c) = p(x + c).
    /// $$
    ///
    /// The coefficient of $x^k$ in the result is $\sum_{i \geq k} \binom{i}{k} p_i c^{i-k}$, where
    /// $p_i$ is the coefficient of $x^i$ in $p$; since $c$ is non-negative, so is every coefficient
    /// of the result. The shift is done as for an
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial): Horner's rule for short
    /// polynomials, and divide and conquer for long ones.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any coefficient plus $n$ times `c.significant_bits()`, a bound
    /// on the size of the coefficients of the result.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::TaylorShift;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!((&p).taylor_shift(Natural::ONE).to_string(), "x^2+5*x+6");
    /// assert_eq!(
    ///     (&p).taylor_shift(Natural::from(10u32)).to_string(),
    ///     "x^2+23*x+132"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3").unwrap();
    /// assert_eq!(
    ///     (&p).taylor_shift(Natural::ONE).to_string(),
    ///     "x^3+3*x^2+3*x+1"
    /// );
    /// ```
    #[inline]
    fn taylor_shift(self, c: Natural) -> NaturalPolynomial {
        self.taylor_shift(&c)
    }
}

impl TaylorShift<&Natural> for NaturalPolynomial {
    type Output = Self;

    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$, replacing $x$ with $x + c$, taking the
    /// polynomial by value and $c$ by reference.
    ///
    /// $$
    /// f(p, c) = p(x + c).
    /// $$
    ///
    /// The coefficient of $x^k$ in the result is $\sum_{i \geq k} \binom{i}{k} p_i c^{i-k}$, where
    /// $p_i$ is the coefficient of $x^i$ in $p$; since $c$ is non-negative, so is every coefficient
    /// of the result. The shift is done as for an
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial): Horner's rule for short
    /// polynomials, and divide and conquer for long ones.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any coefficient plus $n$ times `c.significant_bits()`, a bound
    /// on the size of the coefficients of the result.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::TaylorShift;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     p.clone().taylor_shift(&Natural::ONE).to_string(),
    ///     "x^2+5*x+6"
    /// );
    /// assert_eq!(
    ///     p.clone().taylor_shift(&Natural::from(10u32)).to_string(),
    ///     "x^2+23*x+132"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3").unwrap();
    /// assert_eq!(
    ///     p.clone().taylor_shift(&Natural::ONE).to_string(),
    ///     "x^3+3*x^2+3*x+1"
    /// );
    /// ```
    #[inline]
    fn taylor_shift(mut self, c: &Natural) -> Self {
        self.taylor_shift_assign(c);
        self
    }
}

impl TaylorShift<Natural> for NaturalPolynomial {
    type Output = Self;

    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$, replacing $x$ with $x + c$, taking both
    /// by value.
    ///
    /// $$
    /// f(p, c) = p(x + c).
    /// $$
    ///
    /// The coefficient of $x^k$ in the result is $\sum_{i \geq k} \binom{i}{k} p_i c^{i-k}$, where
    /// $p_i$ is the coefficient of $x^i$ in $p$; since $c$ is non-negative, so is every coefficient
    /// of the result. The shift is done as for an
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial): Horner's rule for short
    /// polynomials, and divide and conquer for long ones.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any coefficient plus $n$ times `c.significant_bits()`, a bound
    /// on the size of the coefficients of the result.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::TaylorShift;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     p.clone().taylor_shift(Natural::ONE).to_string(),
    ///     "x^2+5*x+6"
    /// );
    /// assert_eq!(
    ///     p.clone().taylor_shift(Natural::from(10u32)).to_string(),
    ///     "x^2+23*x+132"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3").unwrap();
    /// assert_eq!(
    ///     p.clone().taylor_shift(Natural::ONE).to_string(),
    ///     "x^3+3*x^2+3*x+1"
    /// );
    /// ```
    #[inline]
    fn taylor_shift(mut self, c: Natural) -> Self {
        self.taylor_shift_assign(&c);
        self
    }
}

impl TaylorShiftAssign<&Natural> for NaturalPolynomial {
    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$ in place, replacing $x$ with $x + c$,
    /// taking $c$ by reference.
    ///
    /// $$
    /// p \\gets p(x + c).
    /// $$
    ///
    /// The coefficient of $x^k$ in the result is $\sum_{i \geq k} \binom{i}{k} p_i c^{i-k}$, where
    /// $p_i$ is the coefficient of $x^i$ in $p$; since $c$ is non-negative, so is every coefficient
    /// of the result. The shift is done as for an
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial): Horner's rule for short
    /// polynomials, and divide and conquer for long ones.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any coefficient plus $n$ times `c.significant_bits()`, a bound
    /// on the size of the coefficients of the result.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::TaylorShiftAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.taylor_shift_assign(&Natural::ONE);
    /// assert_eq!(p.to_string(), "x^2+5*x+6");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3").unwrap();
    /// p.taylor_shift_assign(&Natural::ONE);
    /// assert_eq!(p.to_string(), "x^3+3*x^2+3*x+1");
    /// ```
    fn taylor_shift_assign(&mut self, c: &Natural) {
        self.coefficients = taylor_shift_naturals(core::mem::take(&mut self.coefficients), c);
    }
}

impl TaylorShiftAssign<Natural> for NaturalPolynomial {
    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$ in place, replacing $x$ with $x + c$,
    /// taking $c$ by value.
    ///
    /// $$
    /// p \\gets p(x + c).
    /// $$
    ///
    /// The coefficient of $x^k$ in the result is $\sum_{i \geq k} \binom{i}{k} p_i c^{i-k}$, where
    /// $p_i$ is the coefficient of $x^i$ in $p$; since $c$ is non-negative, so is every coefficient
    /// of the result. The shift is done as for an
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial): Horner's rule for short
    /// polynomials, and divide and conquer for long ones.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any coefficient plus $n$ times `c.significant_bits()`, a bound
    /// on the size of the coefficients of the result.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::TaylorShiftAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.taylor_shift_assign(Natural::ONE);
    /// assert_eq!(p.to_string(), "x^2+5*x+6");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3").unwrap();
    /// p.taylor_shift_assign(Natural::ONE);
    /// assert_eq!(p.to_string(), "x^3+3*x^2+3*x+1");
    /// ```
    #[inline]
    fn taylor_shift_assign(&mut self, c: Natural) {
        self.taylor_shift_assign(&c);
    }
}
