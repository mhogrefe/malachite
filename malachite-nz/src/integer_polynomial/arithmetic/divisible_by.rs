// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::div_exact::checked_div_exact_helper;
use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::basic::traits::Zero;

// Whether `q` divides `p`. 0 is divisible by every polynomial, and only 0 is divisible by 0.
//
// This is equivalent to the result of `fmpz_poly_divides` from `fmpz_poly/divides.c`, FLINT 3.6.0,
// except that 0 is a valid divisor.
fn divisible_by_helper(p: &IntegerPolynomial, q: &IntegerPolynomial) -> bool {
    *p == IntegerPolynomial::ZERO || checked_div_exact_helper(p, q).is_some()
}

impl DivisibleBy<Self> for IntegerPolynomial {
    /// Determines whether an [`IntegerPolynomial`] is divisible by another, taking the first by
    /// value and the second by value.
    ///
    /// $$
    /// f(a, b) = (\exists q : a = bq).
    /// $$
    ///
    /// 0 is divisible by every polynomial, and only 0 is divisible by 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+13*x+5").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x+5").unwrap();
    /// assert!(p.divisible_by(q));
    ///
    /// let p = IntegerPolynomial::from_str("x^2+1").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// assert!(!p.divisible_by(q));
    /// ```
    ///
    /// This is equivalent to the result of `fmpz_poly_divides` from `fmpz_poly/divides.c`, FLINT
    /// 3.6.0, except that 0 is a valid divisor.
    #[inline]
    fn divisible_by(self, other: Self) -> bool {
        divisible_by_helper(&self, &other)
    }
}

impl DivisibleBy<&Self> for IntegerPolynomial {
    /// Determines whether an [`IntegerPolynomial`] is divisible by another, taking the first by
    /// value and the second by reference.
    ///
    /// $$
    /// f(a, b) = (\exists q : a = bq).
    /// $$
    ///
    /// 0 is divisible by every polynomial, and only 0 is divisible by 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+13*x+5").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x+5").unwrap();
    /// assert!(p.divisible_by(&q));
    ///
    /// let p = IntegerPolynomial::from_str("x^2+1").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// assert!(!p.divisible_by(&q));
    /// ```
    ///
    /// This is equivalent to the result of `fmpz_poly_divides` from `fmpz_poly/divides.c`, FLINT
    /// 3.6.0, except that 0 is a valid divisor.
    #[inline]
    fn divisible_by(self, other: &Self) -> bool {
        divisible_by_helper(&self, other)
    }
}

impl DivisibleBy<IntegerPolynomial> for &IntegerPolynomial {
    /// Determines whether an [`IntegerPolynomial`] is divisible by another, taking the first by
    /// reference and the second by value.
    ///
    /// $$
    /// f(a, b) = (\exists q : a = bq).
    /// $$
    ///
    /// 0 is divisible by every polynomial, and only 0 is divisible by 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+13*x+5").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x+5").unwrap();
    /// assert!((&p).divisible_by(q));
    ///
    /// let p = IntegerPolynomial::from_str("x^2+1").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// assert!(!(&p).divisible_by(q));
    /// ```
    ///
    /// This is equivalent to the result of `fmpz_poly_divides` from `fmpz_poly/divides.c`, FLINT
    /// 3.6.0, except that 0 is a valid divisor.
    #[inline]
    fn divisible_by(self, other: IntegerPolynomial) -> bool {
        divisible_by_helper(self, &other)
    }
}

impl DivisibleBy<&IntegerPolynomial> for &IntegerPolynomial {
    /// Determines whether an [`IntegerPolynomial`] is divisible by another, taking the first by
    /// reference and the second by reference.
    ///
    /// $$
    /// f(a, b) = (\exists q : a = bq).
    /// $$
    ///
    /// 0 is divisible by every polynomial, and only 0 is divisible by 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+13*x+5").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x+5").unwrap();
    /// assert!((&p).divisible_by(&q));
    ///
    /// let p = IntegerPolynomial::from_str("x^2+1").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// assert!(!(&p).divisible_by(&q));
    /// ```
    ///
    /// This is equivalent to the result of `fmpz_poly_divides` from `fmpz_poly/divides.c`, FLINT
    /// 3.6.0, except that 0 is a valid divisor.
    #[inline]
    fn divisible_by(self, other: &IntegerPolynomial) -> bool {
        divisible_by_helper(self, other)
    }
}
