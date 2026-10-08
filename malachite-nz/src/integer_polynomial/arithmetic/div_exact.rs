// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign, NegAssign};
use malachite_base::num::basic::traits::One;
use malachite_base::polynomial::Polynomial;

impl DivExact<Integer> for IntegerPolynomial {
    type Output = Self;

    /// Divides an [`IntegerPolynomial`] by an [`Integer`], taking both by value. Every coefficient
    /// of the polynomial must be exactly divisible by the [`Integer`]. If one isn't, this function
    /// may panic or return a meaningless result.
    ///
    /// $$
    /// f(p, c) = \frac{p}{c}.
    /// $$
    ///
    /// A polynomial is divisible by $c$ exactly when $|c|$ divides its
    /// [`content`](malachite_base::num::arithmetic::traits::Content::content), so that is how to
    /// check beforehand.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `c` is zero. May panic if a coefficient of the polynomial is not divisible by `c`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2-3*x+9").unwrap();
    /// assert_eq!(
    ///     p.clone().div_exact(Integer::from(3)).to_string(),
    ///     "2*x^2-x+3"
    /// );
    /// assert_eq!(
    ///     p.clone().div_exact(Integer::from(-3)).to_string(),
    ///     "-2*x^2+x-3"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::ZERO.div_exact(Integer::from(5)),
    ///     IntegerPolynomial::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_scalar_divexact_fmpz` from
    /// `fmpz_poly/scalar_divexact_fmpz.c`, FLINT 3.6.0.
    #[inline]
    fn div_exact(mut self, c: Integer) -> Self {
        self.div_exact_assign(&c);
        self
    }
}

impl<'a> DivExact<&'a Integer> for IntegerPolynomial {
    type Output = Self;

    /// Divides an [`IntegerPolynomial`] by an [`Integer`], taking the polynomial by value and the
    /// [`Integer`] by reference. Every coefficient of the polynomial must be exactly divisible by
    /// the [`Integer`]. If one isn't, this function may panic or return a meaningless result.
    ///
    /// See the documentation for the [`DivExact`] implementation on [`IntegerPolynomial`] that
    /// takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `c` is zero. May panic if a coefficient of the polynomial is not divisible by `c`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2-3*x+9").unwrap();
    /// assert_eq!(
    ///     p.clone().div_exact(&Integer::from(3)).to_string(),
    ///     "2*x^2-x+3"
    /// );
    /// assert_eq!(
    ///     p.clone().div_exact(&Integer::from(-3)).to_string(),
    ///     "-2*x^2+x-3"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::ZERO.div_exact(&Integer::from(5)),
    ///     IntegerPolynomial::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_scalar_divexact_fmpz` from
    /// `fmpz_poly/scalar_divexact_fmpz.c`, FLINT 3.6.0.
    #[inline]
    fn div_exact(mut self, c: &'a Integer) -> Self {
        self.div_exact_assign(c);
        self
    }
}

impl DivExact<Integer> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Divides an [`IntegerPolynomial`] by an [`Integer`], taking the polynomial by reference and
    /// the [`Integer`] by value. Every coefficient of the polynomial must be exactly divisible by
    /// the [`Integer`]. If one isn't, this function may panic or return a meaningless result.
    ///
    /// See the documentation for the [`DivExact`] implementation on [`IntegerPolynomial`] that
    /// takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `c` is zero. May panic if a coefficient of the polynomial is not divisible by `c`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2-3*x+9").unwrap();
    /// assert_eq!((&p).div_exact(Integer::from(3)).to_string(), "2*x^2-x+3");
    /// assert_eq!((&p).div_exact(Integer::from(-3)).to_string(), "-2*x^2+x-3");
    /// assert_eq!(
    ///     IntegerPolynomial::ZERO.div_exact(Integer::from(5)),
    ///     IntegerPolynomial::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_scalar_divexact_fmpz` from
    /// `fmpz_poly/scalar_divexact_fmpz.c`, FLINT 3.6.0.
    #[inline]
    fn div_exact(self, c: Integer) -> IntegerPolynomial {
        self.div_exact(&c)
    }
}

impl<'a> DivExact<&'a Integer> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Divides an [`IntegerPolynomial`] by an [`Integer`], taking both by reference. Every
    /// coefficient of the polynomial must be exactly divisible by the [`Integer`]. If one isn't,
    /// this function may panic or return a meaningless result.
    ///
    /// See the documentation for the [`DivExact`] implementation on [`IntegerPolynomial`] that
    /// takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `c` is zero. May panic if a coefficient of the polynomial is not divisible by `c`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2-3*x+9").unwrap();
    /// assert_eq!((&p).div_exact(&Integer::from(3)).to_string(), "2*x^2-x+3");
    /// assert_eq!((&p).div_exact(&Integer::from(-3)).to_string(), "-2*x^2+x-3");
    /// assert_eq!(
    ///     IntegerPolynomial::ZERO.div_exact(&Integer::from(5)),
    ///     IntegerPolynomial::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_scalar_divexact_fmpz` from
    /// `fmpz_poly/scalar_divexact_fmpz.c`, FLINT 3.6.0.
    fn div_exact(self, c: &'a Integer) -> IntegerPolynomial {
        assert_ne!(*c, 0u32, "division by zero");
        // Only a coefficient that is not divisible by `c` can come out as zero, so trimming is what
        // keeps even a meaningless result a valid polynomial.
        IntegerPolynomial::from_coefficients_asc(match *c {
            integer_one!() => self.coefficients.clone(),
            integer_negative_one!() => self.coefficients.iter().map(|x| -x).collect(),
            _ => self.coefficients.iter().map(|x| x.div_exact(c)).collect(),
        })
    }
}

impl DivExactAssign<Integer> for IntegerPolynomial {
    /// Divides an [`IntegerPolynomial`] by an [`Integer`] in place, taking the [`Integer`] by
    /// value. Every coefficient of the polynomial must be exactly divisible by the [`Integer`]. If
    /// one isn't, this function may panic or leave a meaningless result.
    ///
    /// $$
    /// p \gets \frac{p}{c}.
    /// $$
    ///
    /// See the documentation for the [`DivExact`] implementation on [`IntegerPolynomial`] that
    /// takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `c` is zero. May panic if a coefficient of the polynomial is not divisible by `c`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExactAssign;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x^2-3*x+9").unwrap();
    /// p.div_exact_assign(Integer::from(3));
    /// assert_eq!(p.to_string(), "2*x^2-x+3");
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x^2-3*x+9").unwrap();
    /// p.div_exact_assign(Integer::from(-3));
    /// assert_eq!(p.to_string(), "-2*x^2+x-3");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_scalar_divexact_fmpz` from
    /// `fmpz_poly/scalar_divexact_fmpz.c`, FLINT 3.6.0.
    #[inline]
    fn div_exact_assign(&mut self, c: Integer) {
        self.div_exact_assign(&c);
    }
}

impl<'a> DivExactAssign<&'a Integer> for IntegerPolynomial {
    /// Divides an [`IntegerPolynomial`] by an [`Integer`] in place, taking the [`Integer`] by
    /// reference. Every coefficient of the polynomial must be exactly divisible by the [`Integer`].
    /// If one isn't, this function may panic or leave a meaningless result.
    ///
    /// $$
    /// p \gets \frac{p}{c}.
    /// $$
    ///
    /// See the documentation for the [`DivExact`] implementation on [`IntegerPolynomial`] that
    /// takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `c` is zero. May panic if a coefficient of the polynomial is not divisible by `c`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExactAssign;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x^2-3*x+9").unwrap();
    /// p.div_exact_assign(&Integer::from(3));
    /// assert_eq!(p.to_string(), "2*x^2-x+3");
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x^2-3*x+9").unwrap();
    /// p.div_exact_assign(&Integer::from(-3));
    /// assert_eq!(p.to_string(), "-2*x^2+x-3");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_scalar_divexact_fmpz` from
    /// `fmpz_poly/scalar_divexact_fmpz.c`, FLINT 3.6.0.
    fn div_exact_assign(&mut self, c: &'a Integer) {
        assert_ne!(*c, 0u32, "division by zero");
        match *c {
            integer_one!() => {}
            integer_negative_one!() => self.neg_assign(),
            _ => {
                for x in &mut self.coefficients {
                    x.div_exact_assign(c);
                }
                // Only a coefficient that is not divisible by `c` can come out as zero, so trimming
                // is what keeps even a meaningless result a valid polynomial.
                self.trim();
            }
        }
    }
}
