// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::div_exact::checked_div_exact_helper;
use crate::integer_polynomial::arithmetic::gcd::gcd_helper;
use malachite_base::num::arithmetic::traits::{Lcm, LcmAssign};
use malachite_base::num::basic::traits::Zero;

// The LCM of `p` and `q`: their product divided by their GCD, with a positive leading coefficient.
// The LCM of 0 and any polynomial is 0.
//
// This is equivalent to `fmpz_poly_lcm` from `fmpz_poly/lcm.c`, FLINT 3.6.0.
fn lcm_helper(p: &IntegerPolynomial, q: &IntegerPolynomial) -> IntegerPolynomial {
    if p.coefficients.is_empty() || q.coefficients.is_empty() {
        return IntegerPolynomial::ZERO;
    }
    let product = p * q;
    let g = gcd_helper(p, q);
    let l = checked_div_exact_helper(&product, &g).unwrap();
    if *l.coefficients.last().unwrap() > 0u32 {
        l
    } else {
        -l
    }
}

impl Lcm<Self> for IntegerPolynomial {
    type Output = Self;

    /// Computes the LCM of two [`IntegerPolynomial`]s, taking both by value.
    ///
    /// $$
    /// f(a, b) = \pm\frac{ab}{\gcd(a, b)},
    /// $$
    ///
    /// with the sign chosen to make the leading coefficient positive; the LCM is 0 if either
    /// polynomial is 0. Every common multiple of $a$ and $b$ is a multiple of it.
    ///
    /// The GCD is found as by [`Gcd`](malachite_base::num::arithmetic::traits::Gcd), and the
    /// product is divided by it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Lcm;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x+6").unwrap();
    /// let q = IntegerPolynomial::from_str("4*x^2-4").unwrap();
    /// assert_eq!(p.lcm(q).to_string(), "12*x^2-12");
    ///
    /// let p = IntegerPolynomial::from_str("-x^2+1").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(p.lcm(q).to_string(), "x^2-1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_lcm` from `fmpz_poly/lcm.c`, FLINT 3.6.0.
    #[inline]
    fn lcm(self, other: Self) -> Self {
        lcm_helper(&self, &other)
    }
}

impl Lcm<&Self> for IntegerPolynomial {
    type Output = Self;

    /// Computes the LCM of two [`IntegerPolynomial`]s, taking the first by value and the second by
    /// reference.
    ///
    /// $$
    /// f(a, b) = \pm\frac{ab}{\gcd(a, b)},
    /// $$
    ///
    /// with the sign chosen to make the leading coefficient positive; the LCM is 0 if either
    /// polynomial is 0. Every common multiple of $a$ and $b$ is a multiple of it.
    ///
    /// The GCD is found as by [`Gcd`](malachite_base::num::arithmetic::traits::Gcd), and the
    /// product is divided by it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Lcm;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x+6").unwrap();
    /// let q = IntegerPolynomial::from_str("4*x^2-4").unwrap();
    /// assert_eq!(p.lcm(&q).to_string(), "12*x^2-12");
    ///
    /// let p = IntegerPolynomial::from_str("-x^2+1").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(p.lcm(&q).to_string(), "x^2-1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_lcm` from `fmpz_poly/lcm.c`, FLINT 3.6.0.
    #[inline]
    fn lcm(self, other: &Self) -> Self {
        lcm_helper(&self, other)
    }
}

impl Lcm<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Computes the LCM of two [`IntegerPolynomial`]s, taking the first by reference and the second
    /// by value.
    ///
    /// $$
    /// f(a, b) = \pm\frac{ab}{\gcd(a, b)},
    /// $$
    ///
    /// with the sign chosen to make the leading coefficient positive; the LCM is 0 if either
    /// polynomial is 0. Every common multiple of $a$ and $b$ is a multiple of it.
    ///
    /// The GCD is found as by [`Gcd`](malachite_base::num::arithmetic::traits::Gcd), and the
    /// product is divided by it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Lcm;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x+6").unwrap();
    /// let q = IntegerPolynomial::from_str("4*x^2-4").unwrap();
    /// assert_eq!((&p).lcm(q).to_string(), "12*x^2-12");
    ///
    /// let p = IntegerPolynomial::from_str("-x^2+1").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// assert_eq!((&p).lcm(q).to_string(), "x^2-1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_lcm` from `fmpz_poly/lcm.c`, FLINT 3.6.0.
    #[inline]
    fn lcm(self, other: IntegerPolynomial) -> IntegerPolynomial {
        lcm_helper(self, &other)
    }
}

impl Lcm<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Computes the LCM of two [`IntegerPolynomial`]s, taking both by reference.
    ///
    /// $$
    /// f(a, b) = \pm\frac{ab}{\gcd(a, b)},
    /// $$
    ///
    /// with the sign chosen to make the leading coefficient positive; the LCM is 0 if either
    /// polynomial is 0. Every common multiple of $a$ and $b$ is a multiple of it.
    ///
    /// The GCD is found as by [`Gcd`](malachite_base::num::arithmetic::traits::Gcd), and the
    /// product is divided by it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Lcm;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x+6").unwrap();
    /// let q = IntegerPolynomial::from_str("4*x^2-4").unwrap();
    /// assert_eq!((&p).lcm(&q).to_string(), "12*x^2-12");
    ///
    /// let p = IntegerPolynomial::from_str("-x^2+1").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// assert_eq!((&p).lcm(&q).to_string(), "x^2-1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_lcm` from `fmpz_poly/lcm.c`, FLINT 3.6.0.
    #[inline]
    fn lcm(self, other: &IntegerPolynomial) -> IntegerPolynomial {
        lcm_helper(self, other)
    }
}

impl LcmAssign<Self> for IntegerPolynomial {
    /// Replaces an [`IntegerPolynomial`] with its LCM with another, taking the other polynomial by
    /// value.
    ///
    /// $$
    /// a \gets \pm\frac{ab}{\gcd(a, b)},
    /// $$
    ///
    /// with the sign chosen to make the leading coefficient positive; the LCM is 0 if either
    /// polynomial is 0. Every common multiple of $a$ and $b$ is a multiple of it.
    ///
    /// The GCD is found as by [`Gcd`](malachite_base::num::arithmetic::traits::Gcd), and the
    /// product is divided by it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::LcmAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x+6").unwrap();
    /// let q = IntegerPolynomial::from_str("4*x^2-4").unwrap();
    /// p.lcm_assign(q);
    /// assert_eq!(p.to_string(), "12*x^2-12");
    ///
    /// let mut p = IntegerPolynomial::from_str("-x^2+1").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// p.lcm_assign(q);
    /// assert_eq!(p.to_string(), "x^2-1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_lcm` from `fmpz_poly/lcm.c`, FLINT 3.6.0.
    #[inline]
    fn lcm_assign(&mut self, other: Self) {
        *self = lcm_helper(self, &other);
    }
}

impl LcmAssign<&Self> for IntegerPolynomial {
    /// Replaces an [`IntegerPolynomial`] with its LCM with another, taking the other polynomial by
    /// reference.
    ///
    /// $$
    /// a \gets \pm\frac{ab}{\gcd(a, b)},
    /// $$
    ///
    /// with the sign chosen to make the leading coefficient positive; the LCM is 0 if either
    /// polynomial is 0. Every common multiple of $a$ and $b$ is a multiple of it.
    ///
    /// The GCD is found as by [`Gcd`](malachite_base::num::arithmetic::traits::Gcd), and the
    /// product is divided by it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::LcmAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x+6").unwrap();
    /// let q = IntegerPolynomial::from_str("4*x^2-4").unwrap();
    /// p.lcm_assign(&q);
    /// assert_eq!(p.to_string(), "12*x^2-12");
    ///
    /// let mut p = IntegerPolynomial::from_str("-x^2+1").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// p.lcm_assign(&q);
    /// assert_eq!(p.to_string(), "x^2-1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_lcm` from `fmpz_poly/lcm.c`, FLINT 3.6.0.
    #[inline]
    fn lcm_assign(&mut self, other: &Self) {
        *self = lcm_helper(self, other);
    }
}
