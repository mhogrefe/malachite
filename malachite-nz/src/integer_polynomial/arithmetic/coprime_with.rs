// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::gcd::gcd_helper;
use malachite_base::num::arithmetic::traits::Content;
use malachite_base::num::number_theory::traits::CoprimeWith;

// Whether the GCD of `p` and `q` is 1. The contents are compared first, since that is cheap and
// often settles the question.
fn coprime_with_helper(p: &IntegerPolynomial, q: &IntegerPolynomial) -> bool {
    p.content().coprime_with(q.content()) && gcd_helper(p, q).coefficients.len() == 1
}

impl CoprimeWith<Self> for IntegerPolynomial {
    /// Determines whether two [`IntegerPolynomial`]s are coprime, taking both by value.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \text{true} & \text{if } \gcd(a, b) = 1, \\\\
    ///     \text{false} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// Coprime polynomials have no common factor other than $\pm 1$; this does not mean that they
    /// generate the whole ring, as 2 and $x$ show. A polynomial is coprime with 0 exactly when it
    /// is $\pm 1$.
    ///
    /// The contents are compared first, and then the GCD is found as by
    /// [`Gcd`](malachite_base::num::number_theory::traits::Gcd).
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
    /// use malachite_base::num::number_theory::traits::CoprimeWith;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("2").unwrap();
    /// let q = IntegerPolynomial::from_str("x").unwrap();
    /// assert!(p.coprime_with(q));
    ///
    /// let p = IntegerPolynomial::from_str("6*x+6").unwrap();
    /// let q = IntegerPolynomial::from_str("4*x^2-4").unwrap();
    /// assert!(!p.coprime_with(q));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn coprime_with(self, other: Self) -> bool {
        coprime_with_helper(&self, &other)
    }
}

impl CoprimeWith<&Self> for IntegerPolynomial {
    /// Determines whether two [`IntegerPolynomial`]s are coprime, taking the first by value and the
    /// second by reference.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \text{true} & \text{if } \gcd(a, b) = 1, \\\\
    ///     \text{false} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// Coprime polynomials have no common factor other than $\pm 1$; this does not mean that they
    /// generate the whole ring, as 2 and $x$ show. A polynomial is coprime with 0 exactly when it
    /// is $\pm 1$.
    ///
    /// The contents are compared first, and then the GCD is found as by
    /// [`Gcd`](malachite_base::num::number_theory::traits::Gcd).
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
    /// use malachite_base::num::number_theory::traits::CoprimeWith;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("2").unwrap();
    /// let q = IntegerPolynomial::from_str("x").unwrap();
    /// assert!(p.coprime_with(&q));
    ///
    /// let p = IntegerPolynomial::from_str("6*x+6").unwrap();
    /// let q = IntegerPolynomial::from_str("4*x^2-4").unwrap();
    /// assert!(!p.coprime_with(&q));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn coprime_with(self, other: &Self) -> bool {
        coprime_with_helper(&self, other)
    }
}

impl CoprimeWith<IntegerPolynomial> for &IntegerPolynomial {
    /// Determines whether two [`IntegerPolynomial`]s are coprime, taking the first by reference and
    /// the second by value.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \text{true} & \text{if } \gcd(a, b) = 1, \\\\
    ///     \text{false} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// Coprime polynomials have no common factor other than $\pm 1$; this does not mean that they
    /// generate the whole ring, as 2 and $x$ show. A polynomial is coprime with 0 exactly when it
    /// is $\pm 1$.
    ///
    /// The contents are compared first, and then the GCD is found as by
    /// [`Gcd`](malachite_base::num::number_theory::traits::Gcd).
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
    /// use malachite_base::num::number_theory::traits::CoprimeWith;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("2").unwrap();
    /// let q = IntegerPolynomial::from_str("x").unwrap();
    /// assert!((&p).coprime_with(q));
    ///
    /// let p = IntegerPolynomial::from_str("6*x+6").unwrap();
    /// let q = IntegerPolynomial::from_str("4*x^2-4").unwrap();
    /// assert!(!(&p).coprime_with(q));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn coprime_with(self, other: IntegerPolynomial) -> bool {
        coprime_with_helper(self, &other)
    }
}

impl CoprimeWith<&IntegerPolynomial> for &IntegerPolynomial {
    /// Determines whether two [`IntegerPolynomial`]s are coprime, taking both by reference.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \text{true} & \text{if } \gcd(a, b) = 1, \\\\
    ///     \text{false} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// Coprime polynomials have no common factor other than $\pm 1$; this does not mean that they
    /// generate the whole ring, as 2 and $x$ show. A polynomial is coprime with 0 exactly when it
    /// is $\pm 1$.
    ///
    /// The contents are compared first, and then the GCD is found as by
    /// [`Gcd`](malachite_base::num::number_theory::traits::Gcd).
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
    /// use malachite_base::num::number_theory::traits::CoprimeWith;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("2").unwrap();
    /// let q = IntegerPolynomial::from_str("x").unwrap();
    /// assert!((&p).coprime_with(&q));
    ///
    /// let p = IntegerPolynomial::from_str("6*x+6").unwrap();
    /// let q = IntegerPolynomial::from_str("4*x^2-4").unwrap();
    /// assert!(!(&p).coprime_with(&q));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn coprime_with(self, other: &IntegerPolynomial) -> bool {
        coprime_with_helper(self, other)
    }
}
