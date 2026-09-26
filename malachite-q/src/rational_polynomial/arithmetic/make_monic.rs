// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{MakeMonic, MakeMonicAssign, Polynomial, PrimitivePart};
use malachite_nz::integer_polynomial::IntegerPolynomial;

// For p = A/d, the monic multiple of p is A/lc(A). Dividing numerator and denominator by cont(A),
// which divides lc(A), gives pp(A)/lc(pp(A)) up to sign, and that pair is already canonical: pp(A)
// has content 1. So the denominator of p plays no part, and no GCD is needed beyond the content.
fn monic_from_primitive_part(primitive_part: IntegerPolynomial) -> RationalPolynomial {
    if primitive_part == IntegerPolynomial::ZERO {
        return RationalPolynomial::ZERO;
    }
    let denominator = primitive_part.leading_coefficient().unsigned_abs();
    RationalPolynomial {
        numerator: primitive_part,
        denominator,
    }
}

impl MakeMonic for RationalPolynomial {
    type Output = Self;

    /// Makes a [`RationalPolynomial`] monic, by dividing it by its leading coefficient, taking the
    /// polynomial by value.
    ///
    /// The zero polynomial has no leading coefficient, and is returned unchanged. For $p = A/d$ the
    /// result is $\operatorname{pp}(A)/\operatorname{lc}(\operatorname{pp}(A))$, which is already
    /// in lowest terms, so the denominator of $p$ plays no part.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerator's coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::MakeMonic;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("6*x+4").unwrap();
    /// assert_eq!(p.clone().make_monic().to_string(), "x+2/3");
    /// let p = RationalPolynomial::from_str("-1/2*x^2+1/3").unwrap();
    /// assert_eq!(p.clone().make_monic().to_string(), "x^2-2/3");
    /// assert_eq!(RationalPolynomial::ZERO.make_monic(), 0);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_make_monic` from `fmpq_poly/make_monic.c`, FLINT 3.6.0.
    #[inline]
    fn make_monic(self) -> Self {
        monic_from_primitive_part(self.numerator.primitive_part())
    }
}

impl MakeMonic for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Makes a [`RationalPolynomial`] monic, by dividing it by its leading coefficient, taking the
    /// polynomial by reference.
    ///
    /// The zero polynomial has no leading coefficient, and is returned unchanged. For $p = A/d$ the
    /// result is $\operatorname{pp}(A)/\operatorname{lc}(\operatorname{pp}(A))$, which is already
    /// in lowest terms, so the denominator of $p$ plays no part.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerator's coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::MakeMonic;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("6*x+4").unwrap();
    /// assert_eq!((&p).make_monic().to_string(), "x+2/3");
    /// let p = RationalPolynomial::from_str("-1/2*x^2+1/3").unwrap();
    /// assert_eq!((&p).make_monic().to_string(), "x^2-2/3");
    /// assert_eq!((&RationalPolynomial::ZERO).make_monic(), 0);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_make_monic` from `fmpq_poly/make_monic.c`, FLINT 3.6.0.
    #[inline]
    fn make_monic(self) -> RationalPolynomial {
        monic_from_primitive_part((&self.numerator).primitive_part())
    }
}

impl MakeMonicAssign for RationalPolynomial {
    /// Makes a [`RationalPolynomial`] monic in place, by dividing it by its leading coefficient.
    ///
    /// See [`make_monic`](MakeMonic::make_monic). The zero polynomial is left unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerator's coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MakeMonicAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("6*x+4").unwrap();
    /// p.make_monic_assign();
    /// assert_eq!(p.to_string(), "x+2/3");
    /// ```
    #[inline]
    fn make_monic_assign(&mut self) {
        let numerator = core::mem::take(&mut self.numerator);
        *self = monic_from_primitive_part(numerator.primitive_part());
    }
}
