// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use malachite_base::num::arithmetic::traits::{Content, ContentChained, Gcd};

impl ContentChained<Rational> for RationalVector {
    type Output = Rational;

    /// Computes the GCD of `x` and the elements of a [`RationalVector`], taking the vector by value
    /// and `x` by value.
    ///
    /// The GCD of `x` and the elements of the 0-dimensional vector, or of a vector of zeros, is
    /// `x`. The computation stops as soon as the GCD reaches 1.
    ///
    /// $$
    /// f(v, x) = \gcd(x, v_0, v_1, \ldots, v_{n-1}),
    /// $$
    ///
    /// where $n$ is the dimension of $v$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentChained;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -3/4)").unwrap();
    /// assert_eq!(
    ///     v.clone()
    ///         .content_chained(Rational::from_str("1/6").unwrap())
    ///         .to_string(),
    ///     "1/12"
    /// );
    /// assert_eq!(
    ///     v.content_chained(Rational::from_str("0").unwrap())
    ///         .to_string(),
    ///     "1/4"
    /// );
    /// let v = RationalVector::from_str("()").unwrap();
    /// assert_eq!(
    ///     v.content_chained(Rational::from_str("1/3").unwrap())
    ///         .to_string(),
    ///     "1/3"
    /// );
    /// ```
    #[inline]
    fn content_chained(self, x: Rational) -> Rational {
        self.content().gcd(x)
    }
}

impl ContentChained<&Rational> for RationalVector {
    type Output = Rational;

    /// Computes the GCD of `x` and the elements of a [`RationalVector`], taking the vector by value
    /// and `x` by reference.
    ///
    /// The GCD of `x` and the elements of the 0-dimensional vector, or of a vector of zeros, is
    /// `x`. The computation stops as soon as the GCD reaches 1.
    ///
    /// $$
    /// f(v, x) = \gcd(x, v_0, v_1, \ldots, v_{n-1}),
    /// $$
    ///
    /// where $n$ is the dimension of $v$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentChained;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -3/4)").unwrap();
    /// assert_eq!(
    ///     v.clone()
    ///         .content_chained(&Rational::from_str("1/6").unwrap())
    ///         .to_string(),
    ///     "1/12"
    /// );
    /// assert_eq!(
    ///     v.content_chained(&Rational::from_str("0").unwrap())
    ///         .to_string(),
    ///     "1/4"
    /// );
    /// let v = RationalVector::from_str("()").unwrap();
    /// assert_eq!(
    ///     v.content_chained(&Rational::from_str("1/3").unwrap())
    ///         .to_string(),
    ///     "1/3"
    /// );
    /// ```
    #[inline]
    fn content_chained(self, x: &Rational) -> Rational {
        self.content().gcd(x)
    }
}

impl ContentChained<Rational> for &RationalVector {
    type Output = Rational;

    /// Computes the GCD of `x` and the elements of a [`RationalVector`], taking the vector by
    /// reference and `x` by value.
    ///
    /// The GCD of `x` and the elements of the 0-dimensional vector, or of a vector of zeros, is
    /// `x`. The computation stops as soon as the GCD reaches 1.
    ///
    /// $$
    /// f(v, x) = \gcd(x, v_0, v_1, \ldots, v_{n-1}),
    /// $$
    ///
    /// where $n$ is the dimension of $v$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentChained;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -3/4)").unwrap();
    /// assert_eq!(
    ///     (&v).content_chained(Rational::from_str("1/6").unwrap())
    ///         .to_string(),
    ///     "1/12"
    /// );
    /// assert_eq!(
    ///     (&v).content_chained(Rational::from_str("0").unwrap())
    ///         .to_string(),
    ///     "1/4"
    /// );
    /// let v = RationalVector::from_str("()").unwrap();
    /// assert_eq!(
    ///     (&v).content_chained(Rational::from_str("1/3").unwrap())
    ///         .to_string(),
    ///     "1/3"
    /// );
    /// ```
    #[inline]
    fn content_chained(self, x: Rational) -> Rational {
        self.content().gcd(x)
    }
}

impl ContentChained<&Rational> for &RationalVector {
    type Output = Rational;

    /// Computes the GCD of `x` and the elements of a [`RationalVector`], taking the vector by
    /// reference and `x` by reference.
    ///
    /// The GCD of `x` and the elements of the 0-dimensional vector, or of a vector of zeros, is
    /// `x`. The computation stops as soon as the GCD reaches 1.
    ///
    /// $$
    /// f(v, x) = \gcd(x, v_0, v_1, \ldots, v_{n-1}),
    /// $$
    ///
    /// where $n$ is the dimension of $v$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentChained;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -3/4)").unwrap();
    /// assert_eq!(
    ///     (&v).content_chained(&Rational::from_str("1/6").unwrap())
    ///         .to_string(),
    ///     "1/12"
    /// );
    /// assert_eq!(
    ///     (&v).content_chained(&Rational::from_str("0").unwrap())
    ///         .to_string(),
    ///     "1/4"
    /// );
    /// let v = RationalVector::from_str("()").unwrap();
    /// assert_eq!(
    ///     (&v).content_chained(&Rational::from_str("1/3").unwrap())
    ///         .to_string(),
    ///     "1/3"
    /// );
    /// ```
    #[inline]
    fn content_chained(self, x: &Rational) -> Rational {
        self.content().gcd(x)
    }
}
