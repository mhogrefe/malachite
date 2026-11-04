// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use crate::rational_polynomial::arithmetic::make_monic::monic_from_primitive_part;
use malachite_base::num::arithmetic::traits::{Gcd, GcdAssign, PrimitivePart};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::arithmetic::traits::MakeMonic;

// The monic GCD of `p` and `q`, the monic multiple of the GCD of the primitive parts of their
// numerators. The GCD of 0 and 0 is 0.
//
// This is equivalent to `fmpq_poly_gcd` from `fmpq_poly/gcd.c`, FLINT 3.6.0.
pub(crate) fn gcd_helper(p: &RationalPolynomial, q: &RationalPolynomial) -> RationalPolynomial {
    let (p, q) = if p.len() < q.len() { (q, p) } else { (p, q) };
    if *p == RationalPolynomial::ZERO {
        RationalPolynomial::ZERO
    } else if *q == RationalPolynomial::ZERO {
        p.make_monic()
    } else if p.len() == 1 {
        RationalPolynomial::one()
    } else {
        monic_from_primitive_part(
            (&p.numerator)
                .primitive_part()
                .gcd((&q.numerator).primitive_part()),
        )
    }
}

impl Gcd<Self> for RationalPolynomial {
    type Output = Self;

    /// Computes the monic GCD of two [`RationalPolynomial`]s, taking both by value.
    ///
    /// $$
    /// f(a, b) = \gcd(a, b),
    /// $$
    ///
    /// where the GCD is monic, or 0 if both polynomials are 0.
    ///
    /// It is the GCD of the primitive parts of the numerators, found as by [`Gcd`] on
    /// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial)s, made monic.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of both polynomials' coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2-1/2").unwrap();
    /// let q = RationalPolynomial::from_str("3*x^2+6*x+3").unwrap();
    /// assert_eq!(p.gcd(q).to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_gcd` from `fmpq_poly/gcd.c`, FLINT 3.6.0.
    #[inline]
    fn gcd(self, other: Self) -> Self {
        gcd_helper(&self, &other)
    }
}

impl Gcd<&Self> for RationalPolynomial {
    type Output = Self;

    /// Computes the monic GCD of two [`RationalPolynomial`]s, taking the first by value and the
    /// second by reference.
    ///
    /// $$
    /// f(a, b) = \gcd(a, b),
    /// $$
    ///
    /// where the GCD is monic, or 0 if both polynomials are 0.
    ///
    /// It is the GCD of the primitive parts of the numerators, found as by [`Gcd`] on
    /// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial)s, made monic.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of both polynomials' coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2-1/2").unwrap();
    /// let q = RationalPolynomial::from_str("3*x^2+6*x+3").unwrap();
    /// assert_eq!(p.gcd(&q).to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_gcd` from `fmpq_poly/gcd.c`, FLINT 3.6.0.
    #[inline]
    fn gcd(self, other: &Self) -> Self {
        gcd_helper(&self, other)
    }
}

impl Gcd<RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Computes the monic GCD of two [`RationalPolynomial`]s, taking the first by reference and the
    /// second by value.
    ///
    /// $$
    /// f(a, b) = \gcd(a, b),
    /// $$
    ///
    /// where the GCD is monic, or 0 if both polynomials are 0.
    ///
    /// It is the GCD of the primitive parts of the numerators, found as by [`Gcd`] on
    /// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial)s, made monic.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of both polynomials' coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2-1/2").unwrap();
    /// let q = RationalPolynomial::from_str("3*x^2+6*x+3").unwrap();
    /// assert_eq!((&p).gcd(q).to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_gcd` from `fmpq_poly/gcd.c`, FLINT 3.6.0.
    #[inline]
    fn gcd(self, other: RationalPolynomial) -> RationalPolynomial {
        gcd_helper(self, &other)
    }
}

impl Gcd<&RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Computes the monic GCD of two [`RationalPolynomial`]s, taking both by reference.
    ///
    /// $$
    /// f(a, b) = \gcd(a, b),
    /// $$
    ///
    /// where the GCD is monic, or 0 if both polynomials are 0.
    ///
    /// It is the GCD of the primitive parts of the numerators, found as by [`Gcd`] on
    /// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial)s, made monic.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of both polynomials' coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2-1/2").unwrap();
    /// let q = RationalPolynomial::from_str("3*x^2+6*x+3").unwrap();
    /// assert_eq!((&p).gcd(&q).to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_gcd` from `fmpq_poly/gcd.c`, FLINT 3.6.0.
    #[inline]
    fn gcd(self, other: &RationalPolynomial) -> RationalPolynomial {
        gcd_helper(self, other)
    }
}

impl GcdAssign<Self> for RationalPolynomial {
    /// Replaces a [`RationalPolynomial`] with its monic GCD with another, taking the other
    /// polynomial by value.
    ///
    /// $$
    /// a \gets \gcd(a, b),
    /// $$
    ///
    /// where the GCD is monic, or 0 if both polynomials are 0.
    ///
    /// It is the GCD of the primitive parts of the numerators, found as by [`Gcd`] on
    /// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial)s, made monic.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of both polynomials' coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::GcdAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^2-1/2").unwrap();
    /// let q = RationalPolynomial::from_str("3*x^2+6*x+3").unwrap();
    /// p.gcd_assign(q);
    /// assert_eq!(p.to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_gcd` from `fmpq_poly/gcd.c`, FLINT 3.6.0.
    #[inline]
    fn gcd_assign(&mut self, other: Self) {
        *self = gcd_helper(self, &other);
    }
}

impl GcdAssign<&Self> for RationalPolynomial {
    /// Replaces a [`RationalPolynomial`] with its monic GCD with another, taking the other
    /// polynomial by reference.
    ///
    /// $$
    /// a \gets \gcd(a, b),
    /// $$
    ///
    /// where the GCD is monic, or 0 if both polynomials are 0.
    ///
    /// It is the GCD of the primitive parts of the numerators, found as by [`Gcd`] on
    /// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial)s, made monic.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of both polynomials' coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::GcdAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^2-1/2").unwrap();
    /// let q = RationalPolynomial::from_str("3*x^2+6*x+3").unwrap();
    /// p.gcd_assign(&q);
    /// assert_eq!(p.to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_gcd` from `fmpq_poly/gcd.c`, FLINT 3.6.0.
    #[inline]
    fn gcd_assign(&mut self, other: &Self) {
        *self = gcd_helper(self, other);
    }
}
