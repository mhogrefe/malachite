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
use malachite_base::num::arithmetic::traits::{Lcm, LcmAssign, PrimitivePart};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;

// The monic LCM of `p` and `q`, the monic multiple of the LCM of the primitive parts of their
// numerators. The LCM of 0 and any polynomial is 0.
//
// This is equivalent to `fmpq_poly_lcm` from `fmpq_poly/lcm.c`, FLINT 3.6.0.
fn lcm_helper(p: &RationalPolynomial, q: &RationalPolynomial) -> RationalPolynomial {
    if *p == RationalPolynomial::ZERO || *q == RationalPolynomial::ZERO {
        RationalPolynomial::ZERO
    } else if p.len() == 1 && q.len() == 1 {
        RationalPolynomial::one()
    } else {
        monic_from_primitive_part(
            (&p.numerator)
                .primitive_part()
                .lcm((&q.numerator).primitive_part()),
        )
    }
}

impl Lcm<Self> for RationalPolynomial {
    type Output = Self;

    /// Computes the monic LCM of two [`RationalPolynomial`]s, taking both by value.
    ///
    /// $$
    /// f(a, b) = \operatorname{lcm}(a, b),
    /// $$
    ///
    /// where the LCM is monic, or 0 if either polynomial is 0. Every common multiple of $a$ and $b$
    /// is a multiple of it.
    ///
    /// The product of the primitive parts of the numerators is divided by their GCD, found as by
    /// [`Gcd`](malachite_base::num::arithmetic::traits::Gcd) on
    /// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial)s, and made monic.
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
    /// use malachite_base::num::arithmetic::traits::Lcm;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2-1/2").unwrap();
    /// let q = RationalPolynomial::from_str("3*x^2+6*x+3").unwrap();
    /// assert_eq!(p.lcm(q).to_string(), "x^3+x^2-x-1");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_lcm` from `fmpq_poly/lcm.c`, FLINT 3.6.0.
    #[inline]
    fn lcm(self, other: Self) -> Self {
        lcm_helper(&self, &other)
    }
}

impl Lcm<&Self> for RationalPolynomial {
    type Output = Self;

    /// Computes the monic LCM of two [`RationalPolynomial`]s, taking the first by value and the
    /// second by reference.
    ///
    /// $$
    /// f(a, b) = \operatorname{lcm}(a, b),
    /// $$
    ///
    /// where the LCM is monic, or 0 if either polynomial is 0. Every common multiple of $a$ and $b$
    /// is a multiple of it.
    ///
    /// The product of the primitive parts of the numerators is divided by their GCD, found as by
    /// [`Gcd`](malachite_base::num::arithmetic::traits::Gcd) on
    /// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial)s, and made monic.
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
    /// use malachite_base::num::arithmetic::traits::Lcm;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2-1/2").unwrap();
    /// let q = RationalPolynomial::from_str("3*x^2+6*x+3").unwrap();
    /// assert_eq!(p.lcm(&q).to_string(), "x^3+x^2-x-1");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_lcm` from `fmpq_poly/lcm.c`, FLINT 3.6.0.
    #[inline]
    fn lcm(self, other: &Self) -> Self {
        lcm_helper(&self, other)
    }
}

impl Lcm<RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Computes the monic LCM of two [`RationalPolynomial`]s, taking the first by reference and the
    /// second by value.
    ///
    /// $$
    /// f(a, b) = \operatorname{lcm}(a, b),
    /// $$
    ///
    /// where the LCM is monic, or 0 if either polynomial is 0. Every common multiple of $a$ and $b$
    /// is a multiple of it.
    ///
    /// The product of the primitive parts of the numerators is divided by their GCD, found as by
    /// [`Gcd`](malachite_base::num::arithmetic::traits::Gcd) on
    /// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial)s, and made monic.
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
    /// use malachite_base::num::arithmetic::traits::Lcm;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2-1/2").unwrap();
    /// let q = RationalPolynomial::from_str("3*x^2+6*x+3").unwrap();
    /// assert_eq!((&p).lcm(q).to_string(), "x^3+x^2-x-1");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_lcm` from `fmpq_poly/lcm.c`, FLINT 3.6.0.
    #[inline]
    fn lcm(self, other: RationalPolynomial) -> RationalPolynomial {
        lcm_helper(self, &other)
    }
}

impl Lcm<&RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Computes the monic LCM of two [`RationalPolynomial`]s, taking both by reference.
    ///
    /// $$
    /// f(a, b) = \operatorname{lcm}(a, b),
    /// $$
    ///
    /// where the LCM is monic, or 0 if either polynomial is 0. Every common multiple of $a$ and $b$
    /// is a multiple of it.
    ///
    /// The product of the primitive parts of the numerators is divided by their GCD, found as by
    /// [`Gcd`](malachite_base::num::arithmetic::traits::Gcd) on
    /// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial)s, and made monic.
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
    /// use malachite_base::num::arithmetic::traits::Lcm;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2-1/2").unwrap();
    /// let q = RationalPolynomial::from_str("3*x^2+6*x+3").unwrap();
    /// assert_eq!((&p).lcm(&q).to_string(), "x^3+x^2-x-1");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_lcm` from `fmpq_poly/lcm.c`, FLINT 3.6.0.
    #[inline]
    fn lcm(self, other: &RationalPolynomial) -> RationalPolynomial {
        lcm_helper(self, other)
    }
}

impl LcmAssign<Self> for RationalPolynomial {
    /// Replaces a [`RationalPolynomial`] with its monic LCM with another, taking the other
    /// polynomial by value.
    ///
    /// $$
    /// a \gets \operatorname{lcm}(a, b),
    /// $$
    ///
    /// where the LCM is monic, or 0 if either polynomial is 0. Every common multiple of $a$ and $b$
    /// is a multiple of it.
    ///
    /// The product of the primitive parts of the numerators is divided by their GCD, found as by
    /// [`Gcd`](malachite_base::num::arithmetic::traits::Gcd) on
    /// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial)s, and made monic.
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
    /// use malachite_base::num::arithmetic::traits::LcmAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^2-1/2").unwrap();
    /// let q = RationalPolynomial::from_str("3*x^2+6*x+3").unwrap();
    /// p.lcm_assign(q);
    /// assert_eq!(p.to_string(), "x^3+x^2-x-1");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_lcm` from `fmpq_poly/lcm.c`, FLINT 3.6.0.
    #[inline]
    fn lcm_assign(&mut self, other: Self) {
        *self = lcm_helper(self, &other);
    }
}

impl LcmAssign<&Self> for RationalPolynomial {
    /// Replaces a [`RationalPolynomial`] with its monic LCM with another, taking the other
    /// polynomial by reference.
    ///
    /// $$
    /// a \gets \operatorname{lcm}(a, b),
    /// $$
    ///
    /// where the LCM is monic, or 0 if either polynomial is 0. Every common multiple of $a$ and $b$
    /// is a multiple of it.
    ///
    /// The product of the primitive parts of the numerators is divided by their GCD, found as by
    /// [`Gcd`](malachite_base::num::arithmetic::traits::Gcd) on
    /// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial)s, and made monic.
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
    /// use malachite_base::num::arithmetic::traits::LcmAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^2-1/2").unwrap();
    /// let q = RationalPolynomial::from_str("3*x^2+6*x+3").unwrap();
    /// p.lcm_assign(&q);
    /// assert_eq!(p.to_string(), "x^3+x^2-x-1");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_lcm` from `fmpq_poly/lcm.c`, FLINT 3.6.0.
    #[inline]
    fn lcm_assign(&mut self, other: &Self) {
        *self = lcm_helper(self, other);
    }
}
