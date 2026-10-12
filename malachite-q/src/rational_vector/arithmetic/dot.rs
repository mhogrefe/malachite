// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use malachite_base::num::arithmetic::traits::Dot;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;

// The dot product of two vectors, each written as integers over a common denominator: one integer
// dot product and one reduction replace a reduction per pair.
fn dot_over_denominators(
    (xs, d): (IntegerVector, Natural),
    (ys, e): (IntegerVector, Natural),
) -> Rational {
    Rational::from_integers(xs.dot(ys), Integer::from(d * e))
}

impl Dot<Self> for RationalVector {
    type Output = Rational;

    /// Computes the dot product of two [`RationalVector`]s, taking the first by value and the
    /// second by value.
    ///
    /// The dot product is the sum of the products of corresponding elements, and is 0 for two
    /// 0-dimensional vectors.
    ///
    /// $$
    /// f(v, w) = \sum_{i=0}^{n-1} v_iw_i,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of the numerators the elements take over each
    /// vector's common denominator.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Dot;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3)").unwrap();
    /// let w = RationalVector::from_str("(4/5, 3/2)").unwrap();
    /// assert_eq!(v.clone().dot(w.clone()).to_string(), "-3/5");
    /// assert_eq!(
    ///     RationalVector::from_str("()")
    ///         .unwrap()
    ///         .dot(RationalVector::from_str("()").unwrap()),
    ///     0
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpq_vec_dot` from `fmpq_vec/dot.c`, FLINT 3.6.0.
    fn dot(self, other: Self) -> Rational {
        assert_eq!(
            self.elements.len(),
            other.elements.len(),
            "the vectors must have the same dimension"
        );
        dot_over_denominators(
            self.into_numerators_and_denominator(),
            other.into_numerators_and_denominator(),
        )
    }
}

impl Dot<&Self> for RationalVector {
    type Output = Rational;

    /// Computes the dot product of two [`RationalVector`]s, taking the first by value and the
    /// second by reference.
    ///
    /// The dot product is the sum of the products of corresponding elements, and is 0 for two
    /// 0-dimensional vectors.
    ///
    /// $$
    /// f(v, w) = \sum_{i=0}^{n-1} v_iw_i,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of the numerators the elements take over each
    /// vector's common denominator.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Dot;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3)").unwrap();
    /// let w = RationalVector::from_str("(4/5, 3/2)").unwrap();
    /// assert_eq!(v.clone().dot(&w).to_string(), "-3/5");
    /// assert_eq!(
    ///     RationalVector::from_str("()")
    ///         .unwrap()
    ///         .dot(RationalVector::from_str("()").unwrap()),
    ///     0
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpq_vec_dot` from `fmpq_vec/dot.c`, FLINT 3.6.0.
    fn dot(self, other: &Self) -> Rational {
        assert_eq!(
            self.elements.len(),
            other.elements.len(),
            "the vectors must have the same dimension"
        );
        dot_over_denominators(
            self.into_numerators_and_denominator(),
            other.to_numerators_and_denominator(),
        )
    }
}

impl Dot<RationalVector> for &RationalVector {
    type Output = Rational;

    /// Computes the dot product of two [`RationalVector`]s, taking the first by reference and the
    /// second by value.
    ///
    /// The dot product is the sum of the products of corresponding elements, and is 0 for two
    /// 0-dimensional vectors.
    ///
    /// $$
    /// f(v, w) = \sum_{i=0}^{n-1} v_iw_i,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of the numerators the elements take over each
    /// vector's common denominator.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Dot;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3)").unwrap();
    /// let w = RationalVector::from_str("(4/5, 3/2)").unwrap();
    /// assert_eq!((&v).dot(w.clone()).to_string(), "-3/5");
    /// assert_eq!(
    ///     RationalVector::from_str("()")
    ///         .unwrap()
    ///         .dot(RationalVector::from_str("()").unwrap()),
    ///     0
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpq_vec_dot` from `fmpq_vec/dot.c`, FLINT 3.6.0.
    fn dot(self, other: RationalVector) -> Rational {
        assert_eq!(
            self.elements.len(),
            other.elements.len(),
            "the vectors must have the same dimension"
        );
        dot_over_denominators(
            self.to_numerators_and_denominator(),
            other.into_numerators_and_denominator(),
        )
    }
}

impl Dot<&RationalVector> for &RationalVector {
    type Output = Rational;

    /// Computes the dot product of two [`RationalVector`]s, taking the first by reference and the
    /// second by reference.
    ///
    /// The dot product is the sum of the products of corresponding elements, and is 0 for two
    /// 0-dimensional vectors.
    ///
    /// $$
    /// f(v, w) = \sum_{i=0}^{n-1} v_iw_i,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of the numerators the elements take over each
    /// vector's common denominator.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Dot;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3)").unwrap();
    /// let w = RationalVector::from_str("(4/5, 3/2)").unwrap();
    /// assert_eq!((&v).dot(&w).to_string(), "-3/5");
    /// assert_eq!(
    ///     RationalVector::from_str("()")
    ///         .unwrap()
    ///         .dot(RationalVector::from_str("()").unwrap()),
    ///     0
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpq_vec_dot` from `fmpq_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn dot(self, other: &RationalVector) -> Rational {
        assert_eq!(
            self.elements.len(),
            other.elements.len(),
            "the vectors must have the same dimension"
        );
        dot_over_denominators(
            self.to_numerators_and_denominator(),
            other.to_numerators_and_denominator(),
        )
    }
}
