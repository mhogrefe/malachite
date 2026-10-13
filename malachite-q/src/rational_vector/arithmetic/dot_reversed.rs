// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use malachite_base::num::arithmetic::traits::DotReversed;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;

// The reversed dot product of two vectors, each written as integers over a common denominator: one
// integer dot product and one reduction replace a reduction per pair.
fn dot_reversed_over_denominators(
    (xs, d): (IntegerVector, Natural),
    (ys, e): (IntegerVector, Natural),
) -> Rational {
    Rational::from_integers(xs.dot_reversed(ys), Integer::from(d * e))
}

fn assert_same_dimension(v: &RationalVector, w: &RationalVector) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "the vectors must have the same dimension"
    );
}

impl DotReversed<Self> for RationalVector {
    type Output = Rational;

    /// Computes the dot product of a [`RationalVector`] and the reverse of another, taking the
    /// first by value and the second by value.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product. The reversed dot
    /// product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w) = \sum_{i=0}^{n-1} v_iw_{n-1-i},
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
    /// use malachite_base::num::arithmetic::traits::DotReversed;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3)").unwrap();
    /// let w = RationalVector::from_str("(4/5, 3/2)").unwrap();
    /// assert_eq!(v.clone().dot_reversed(w.clone()).to_string(), "13/60");
    /// ```
    fn dot_reversed(self, other: Self) -> Rational {
        assert_same_dimension(&self, &other);
        dot_reversed_over_denominators(
            self.into_numerators_and_denominator(),
            other.into_numerators_and_denominator(),
        )
    }
}

impl DotReversed<&Self> for RationalVector {
    type Output = Rational;

    /// Computes the dot product of a [`RationalVector`] and the reverse of another, taking the
    /// first by value and the second by reference.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product. The reversed dot
    /// product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w) = \sum_{i=0}^{n-1} v_iw_{n-1-i},
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
    /// use malachite_base::num::arithmetic::traits::DotReversed;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3)").unwrap();
    /// let w = RationalVector::from_str("(4/5, 3/2)").unwrap();
    /// assert_eq!(v.clone().dot_reversed(&w).to_string(), "13/60");
    /// ```
    fn dot_reversed(self, other: &Self) -> Rational {
        assert_same_dimension(&self, other);
        dot_reversed_over_denominators(
            self.into_numerators_and_denominator(),
            other.to_numerators_and_denominator(),
        )
    }
}

impl DotReversed<RationalVector> for &RationalVector {
    type Output = Rational;

    /// Computes the dot product of a [`RationalVector`] and the reverse of another, taking the
    /// first by reference and the second by value.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product. The reversed dot
    /// product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w) = \sum_{i=0}^{n-1} v_iw_{n-1-i},
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
    /// use malachite_base::num::arithmetic::traits::DotReversed;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3)").unwrap();
    /// let w = RationalVector::from_str("(4/5, 3/2)").unwrap();
    /// assert_eq!((&v).dot_reversed(w.clone()).to_string(), "13/60");
    /// ```
    fn dot_reversed(self, other: RationalVector) -> Rational {
        assert_same_dimension(self, &other);
        dot_reversed_over_denominators(
            self.to_numerators_and_denominator(),
            other.into_numerators_and_denominator(),
        )
    }
}

impl DotReversed<&RationalVector> for &RationalVector {
    type Output = Rational;

    /// Computes the dot product of a [`RationalVector`] and the reverse of another, taking the
    /// first by reference and the second by reference.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product. The reversed dot
    /// product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w) = \sum_{i=0}^{n-1} v_iw_{n-1-i},
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
    /// use malachite_base::num::arithmetic::traits::DotReversed;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3)").unwrap();
    /// let w = RationalVector::from_str("(4/5, 3/2)").unwrap();
    /// assert_eq!((&v).dot_reversed(&w).to_string(), "13/60");
    /// ```
    fn dot_reversed(self, other: &RationalVector) -> Rational {
        assert_same_dimension(self, other);
        dot_reversed_over_denominators(
            self.to_numerators_and_denominator(),
            other.to_numerators_and_denominator(),
        )
    }
}
