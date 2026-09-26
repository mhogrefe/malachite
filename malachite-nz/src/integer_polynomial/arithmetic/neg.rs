// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use core::ops::Neg;
use malachite_base::num::arithmetic::traits::NegAssign;

impl Neg for IntegerPolynomial {
    type Output = Self;

    /// Negates an [`IntegerPolynomial`], taking it by value.
    ///
    /// $$
    /// f(p) = -p.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// assert_eq!((-p).to_string(), "-x^2+3*x-2");
    /// assert_eq!(-IntegerPolynomial::ZERO, IntegerPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_neg` from `fmpz_poly/neg.c`, FLINT 3.6.0.
    fn neg(mut self) -> Self {
        self.neg_assign();
        self
    }
}

impl Neg for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Negates an [`IntegerPolynomial`], taking it by reference.
    ///
    /// $$
    /// f(p) = -p.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// assert_eq!((-&p).to_string(), "-x^2+3*x-2");
    /// assert_eq!(-&IntegerPolynomial::ZERO, IntegerPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_neg` from `fmpz_poly/neg.c`, FLINT 3.6.0.
    fn neg(self) -> IntegerPolynomial {
        IntegerPolynomial {
            coefficients: self.coefficients.iter().map(|c| -c).collect(),
        }
    }
}

impl NegAssign for IntegerPolynomial {
    /// Negates an [`IntegerPolynomial`] in place.
    ///
    /// $$
    /// p \gets -p.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::NegAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// p.neg_assign();
    /// assert_eq!(p.to_string(), "-x^2+3*x-2");
    /// ```
    fn neg_assign(&mut self) {
        for c in &mut self.coefficients {
            c.neg_assign();
        }
    }
}
