// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use malachite_base::polynomial::{
    DeflatePowerOfX, DeflatePowerOfXAssign, slice_deflate_power_of_x, vec_deflate_power_of_x,
};

impl DeflatePowerOfX for IntegerPolynomial {
    type Output = Self;

    /// Deflates an [`IntegerPolynomial`] by $n$, taking it by value, giving the polynomial $q$ with
    /// $q(x^n) = p(x)$. The coefficient of $x^{in}$ moves to $x^i$.
    ///
    /// $$
    /// f(p, n) = q, \quad \text{where} \quad q(x^n) = p(x).
    /// $$
    ///
    /// A constant polynomial deflates to itself, and deflating by 1 changes nothing.
    ///
    /// # Worst-case complexity
    /// $T(m) = O(m)$
    ///
    /// $M(m) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $m$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `n` is 0, or if the polynomial has a nonzero coefficient at an exponent that is
    /// not a multiple of `n`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DeflatePowerOfX;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^4-3*x^2+2").unwrap();
    /// assert_eq!(p.deflate_power_of_x(2).to_string(), "x^2-3*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_deflate` from `fmpz_poly/deflate.c`, FLINT 3.6.0, except
    /// that it panics rather than dropping the coefficients at other exponents.
    #[inline]
    fn deflate_power_of_x(mut self, n: u64) -> Self {
        self.deflate_power_of_x_assign(n);
        self
    }
}

impl DeflatePowerOfX for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Deflates an [`IntegerPolynomial`] by $n$, taking it by reference, giving the polynomial $q$
    /// with $q(x^n) = p(x)$. The coefficient of $x^{in}$ moves to $x^i$.
    ///
    /// $$
    /// f(p, n) = q, \quad \text{where} \quad q(x^n) = p(x).
    /// $$
    ///
    /// A constant polynomial deflates to itself, and deflating by 1 changes nothing.
    ///
    /// # Worst-case complexity
    /// $T(m) = O(m)$
    ///
    /// $M(m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $m$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Panics
    /// Panics if `n` is 0, or if the polynomial has a nonzero coefficient at an exponent that is
    /// not a multiple of `n`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DeflatePowerOfX;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^4-3*x^2+2").unwrap();
    /// assert_eq!((&p).deflate_power_of_x(2).to_string(), "x^2-3*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_deflate` from `fmpz_poly/deflate.c`, FLINT 3.6.0, except
    /// that it panics rather than dropping the coefficients at other exponents.
    #[inline]
    fn deflate_power_of_x(self, n: u64) -> IntegerPolynomial {
        IntegerPolynomial {
            coefficients: slice_deflate_power_of_x(&self.coefficients, n, |c| *c == 0u32),
        }
    }
}

impl DeflatePowerOfXAssign for IntegerPolynomial {
    /// Deflates an [`IntegerPolynomial`] by $n$ in place, replacing $p$ with the polynomial $q$
    /// such that $q(x^n) = p(x)$. The coefficient of $x^{in}$ moves to $x^i$.
    ///
    /// $$
    /// p \gets q, \quad \text{where} \quad q(x^n) = p(x).
    /// $$
    ///
    /// A constant polynomial deflates to itself, and deflating by 1 changes nothing.
    ///
    /// # Worst-case complexity
    /// $T(m) = O(m)$
    ///
    /// $M(m) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $m$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `n` is 0, or if the polynomial has a nonzero coefficient at an exponent that is
    /// not a multiple of `n`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DeflatePowerOfXAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^4-3*x^2+2").unwrap();
    /// p.deflate_power_of_x_assign(2);
    /// assert_eq!(p.to_string(), "x^2-3*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_deflate` from `fmpz_poly/deflate.c`, FLINT 3.6.0, except
    /// that it panics rather than dropping the coefficients at other exponents.
    #[inline]
    fn deflate_power_of_x_assign(&mut self, n: u64) {
        vec_deflate_power_of_x(&mut self.coefficients, n, |c| *c == 0u32);
    }
}
