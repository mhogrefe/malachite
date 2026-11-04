// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::arithmetic::traits::PrimitivePart;
use malachite_base::num::number_theory::traits::{CoprimeWith, Gcd};
use malachite_base::polynomial::Polynomial;

// Whether the GCD of `p` and `q` is 1: when neither is 0, whether the GCD of the primitive parts of
// their numerators is constant.
fn coprime_with_helper(p: &RationalPolynomial, q: &RationalPolynomial) -> bool {
    let (p, q) = if p.len() < q.len() { (q, p) } else { (p, q) };
    match q.len() {
        0 => p.len() == 1,
        1 => true,
        _ => {
            (&p.numerator)
                .primitive_part()
                .gcd((&q.numerator).primitive_part())
                .len()
                == 1
        }
    }
}

impl CoprimeWith<Self> for RationalPolynomial {
    /// Determines whether two [`RationalPolynomial`]s are coprime, taking both by value.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \text{true} & \text{if } \gcd(a, b) = 1, \\\\
    ///     \text{false} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// Coprime polynomials have no common root. A polynomial is coprime with 0 exactly when it is a
    /// nonzero constant.
    ///
    /// Nonzero constants are coprime with everything; otherwise, the GCD of the primitive parts of
    /// the numerators is found as by [`Gcd`] on
    /// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial)s.
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
    /// use malachite_base::num::number_theory::traits::CoprimeWith;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2+1").unwrap();
    /// let q = RationalPolynomial::from_str("x+1/3").unwrap();
    /// assert!(p.coprime_with(q));
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2-1/2").unwrap();
    /// let q = RationalPolynomial::from_str("3*x^2+6*x+3").unwrap();
    /// assert!(!p.coprime_with(q));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn coprime_with(self, other: Self) -> bool {
        coprime_with_helper(&self, &other)
    }
}

impl CoprimeWith<&Self> for RationalPolynomial {
    /// Determines whether two [`RationalPolynomial`]s are coprime, taking the first by value and
    /// the second by reference.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \text{true} & \text{if } \gcd(a, b) = 1, \\\\
    ///     \text{false} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// Coprime polynomials have no common root. A polynomial is coprime with 0 exactly when it is a
    /// nonzero constant.
    ///
    /// Nonzero constants are coprime with everything; otherwise, the GCD of the primitive parts of
    /// the numerators is found as by [`Gcd`] on
    /// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial)s.
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
    /// use malachite_base::num::number_theory::traits::CoprimeWith;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2+1").unwrap();
    /// let q = RationalPolynomial::from_str("x+1/3").unwrap();
    /// assert!(p.coprime_with(&q));
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2-1/2").unwrap();
    /// let q = RationalPolynomial::from_str("3*x^2+6*x+3").unwrap();
    /// assert!(!p.coprime_with(&q));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn coprime_with(self, other: &Self) -> bool {
        coprime_with_helper(&self, other)
    }
}

impl CoprimeWith<RationalPolynomial> for &RationalPolynomial {
    /// Determines whether two [`RationalPolynomial`]s are coprime, taking the first by reference
    /// and the second by value.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \text{true} & \text{if } \gcd(a, b) = 1, \\\\
    ///     \text{false} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// Coprime polynomials have no common root. A polynomial is coprime with 0 exactly when it is a
    /// nonzero constant.
    ///
    /// Nonzero constants are coprime with everything; otherwise, the GCD of the primitive parts of
    /// the numerators is found as by [`Gcd`] on
    /// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial)s.
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
    /// use malachite_base::num::number_theory::traits::CoprimeWith;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2+1").unwrap();
    /// let q = RationalPolynomial::from_str("x+1/3").unwrap();
    /// assert!((&p).coprime_with(q));
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2-1/2").unwrap();
    /// let q = RationalPolynomial::from_str("3*x^2+6*x+3").unwrap();
    /// assert!(!(&p).coprime_with(q));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn coprime_with(self, other: RationalPolynomial) -> bool {
        coprime_with_helper(self, &other)
    }
}

impl CoprimeWith<&RationalPolynomial> for &RationalPolynomial {
    /// Determines whether two [`RationalPolynomial`]s are coprime, taking both by reference.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \text{true} & \text{if } \gcd(a, b) = 1, \\\\
    ///     \text{false} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// Coprime polynomials have no common root. A polynomial is coprime with 0 exactly when it is a
    /// nonzero constant.
    ///
    /// Nonzero constants are coprime with everything; otherwise, the GCD of the primitive parts of
    /// the numerators is found as by [`Gcd`] on
    /// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial)s.
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
    /// use malachite_base::num::number_theory::traits::CoprimeWith;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2+1").unwrap();
    /// let q = RationalPolynomial::from_str("x+1/3").unwrap();
    /// assert!((&p).coprime_with(&q));
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2-1/2").unwrap();
    /// let q = RationalPolynomial::from_str("3*x^2+6*x+3").unwrap();
    /// assert!(!(&p).coprime_with(&q));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn coprime_with(self, other: &RationalPolynomial) -> bool {
        coprime_with_helper(self, other)
    }
}
