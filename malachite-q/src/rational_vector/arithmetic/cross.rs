// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{Cross, CrossAssign, MulSubMul};

// Panics unless both vectors have dimension 3.
fn assert_3d(xs: &[Rational], ys: &[Rational]) {
    assert!(
        xs.len() == 3 && ys.len() == 3,
        "the cross product is only defined for 3-dimensional vectors"
    );
}

// The cross product of the 3-dimensional vectors with elements `xs` and `ys`. Each element is a
// 2-by-2 determinant, $ab - cd$.
fn rationals_cross(xs: &[Rational], ys: &[Rational]) -> Vec<Rational> {
    assert_3d(xs, ys);
    let [x0, x1, x2] = xs else { unreachable!() };
    let [y0, y1, y2] = ys else { unreachable!() };
    vec![x1.mul_sub_mul(y2, x2, y1), x2.mul_sub_mul(y0, x0, y2), x0.mul_sub_mul(y1, x1, y0)]
}

impl Cross<Self> for RationalVector {
    type Output = Self;

    /// Computes the cross product of two 3-dimensional [`RationalVector`]s, taking both by value.
    ///
    /// $$
    /// f(v, w) = v \\times w = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0).
    /// $$
    ///
    /// The result is perpendicular to both vectors, its length is the area of the parallelogram
    /// they span, and swapping the vectors negates it.
    ///
    /// # Worst-case complexity
    /// $T(m) = O(m \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $m$ is the largest number of significant
    /// bits of the numerator or denominator of any element.
    ///
    /// # Panics
    /// Panics if either vector does not have dimension 3.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Cross;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     RationalVector::from_str("(1/2, 0, 1)")
    ///         .unwrap()
    ///         .cross(RationalVector::from_str("(0, 2/3, -1)").unwrap())
    ///         .to_string(),
    ///     "(-2/3, 1/2, 1/3)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     RationalVector::from_str("(1, 0, 0)")
    ///         .unwrap()
    ///         .cross(RationalVector::from_str("(0, 1, 0)").unwrap())
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// ```
    #[inline]
    fn cross(self, other: Self) -> Self {
        Self {
            elements: rationals_cross(&self.elements, &other.elements),
        }
    }
}

impl Cross<&Self> for RationalVector {
    type Output = Self;

    /// Computes the cross product of two 3-dimensional [`RationalVector`]s, taking the first by
    /// value and the second by reference.
    ///
    /// $$
    /// f(v, w) = v \\times w = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0).
    /// $$
    ///
    /// The result is perpendicular to both vectors, its length is the area of the parallelogram
    /// they span, and swapping the vectors negates it.
    ///
    /// # Worst-case complexity
    /// $T(m) = O(m \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $m$ is the largest number of significant
    /// bits of the numerator or denominator of any element.
    ///
    /// # Panics
    /// Panics if either vector does not have dimension 3.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Cross;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     RationalVector::from_str("(1/2, 0, 1)")
    ///         .unwrap()
    ///         .cross(&RationalVector::from_str("(0, 2/3, -1)").unwrap())
    ///         .to_string(),
    ///     "(-2/3, 1/2, 1/3)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     RationalVector::from_str("(1, 0, 0)")
    ///         .unwrap()
    ///         .cross(&RationalVector::from_str("(0, 1, 0)").unwrap())
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// ```
    #[inline]
    fn cross(self, other: &Self) -> Self {
        Self {
            elements: rationals_cross(&self.elements, &other.elements),
        }
    }
}

impl Cross<RationalVector> for &RationalVector {
    type Output = RationalVector;

    /// Computes the cross product of two 3-dimensional [`RationalVector`]s, taking the first by
    /// reference and the second by value.
    ///
    /// $$
    /// f(v, w) = v \\times w = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0).
    /// $$
    ///
    /// The result is perpendicular to both vectors, its length is the area of the parallelogram
    /// they span, and swapping the vectors negates it.
    ///
    /// # Worst-case complexity
    /// $T(m) = O(m \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $m$ is the largest number of significant
    /// bits of the numerator or denominator of any element.
    ///
    /// # Panics
    /// Panics if either vector does not have dimension 3.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Cross;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     (&RationalVector::from_str("(1/2, 0, 1)").unwrap())
    ///         .cross(RationalVector::from_str("(0, 2/3, -1)").unwrap())
    ///         .to_string(),
    ///     "(-2/3, 1/2, 1/3)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     (&RationalVector::from_str("(1, 0, 0)").unwrap())
    ///         .cross(RationalVector::from_str("(0, 1, 0)").unwrap())
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// ```
    #[inline]
    fn cross(self, other: RationalVector) -> RationalVector {
        RationalVector {
            elements: rationals_cross(&self.elements, &other.elements),
        }
    }
}

impl Cross<&RationalVector> for &RationalVector {
    type Output = RationalVector;

    /// Computes the cross product of two 3-dimensional [`RationalVector`]s, taking both by
    /// reference.
    ///
    /// $$
    /// f(v, w) = v \\times w = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0).
    /// $$
    ///
    /// The result is perpendicular to both vectors, its length is the area of the parallelogram
    /// they span, and swapping the vectors negates it.
    ///
    /// # Worst-case complexity
    /// $T(m) = O(m \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $m$ is the largest number of significant
    /// bits of the numerator or denominator of any element.
    ///
    /// # Panics
    /// Panics if either vector does not have dimension 3.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Cross;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     (&RationalVector::from_str("(1/2, 0, 1)").unwrap())
    ///         .cross(&RationalVector::from_str("(0, 2/3, -1)").unwrap())
    ///         .to_string(),
    ///     "(-2/3, 1/2, 1/3)"
    /// );
    /// // The cross product of the first two standard basis vectors is the third.
    /// assert_eq!(
    ///     (&RationalVector::from_str("(1, 0, 0)").unwrap())
    ///         .cross(&RationalVector::from_str("(0, 1, 0)").unwrap())
    ///         .to_string(),
    ///     "(0, 0, 1)"
    /// );
    /// ```
    #[inline]
    fn cross(self, other: &RationalVector) -> RationalVector {
        RationalVector {
            elements: rationals_cross(&self.elements, &other.elements),
        }
    }
}

impl CrossAssign<Self> for RationalVector {
    /// Replaces a 3-dimensional [`RationalVector`] with its cross product with another, taking the
    /// other by value.
    ///
    /// $$
    /// v \\gets v \\times w = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0).
    /// $$
    ///
    /// The result is perpendicular to both vectors, its length is the area of the parallelogram
    /// they span, and swapping the vectors negates it.
    ///
    /// # Worst-case complexity
    /// $T(m) = O(m \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $m$ is the largest number of significant
    /// bits of the numerator or denominator of any element.
    ///
    /// # Panics
    /// Panics if either vector does not have dimension 3.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CrossAssign;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1/2, 0, 1)").unwrap();
    /// v.cross_assign(RationalVector::from_str("(0, 2/3, -1)").unwrap());
    /// assert_eq!(v.to_string(), "(-2/3, 1/2, 1/3)");
    ///
    /// // The cross product of the first two standard basis vectors is the third.
    /// let mut v = RationalVector::from_str("(1, 0, 0)").unwrap();
    /// v.cross_assign(RationalVector::from_str("(0, 1, 0)").unwrap());
    /// assert_eq!(v.to_string(), "(0, 0, 1)");
    /// ```
    #[inline]
    fn cross_assign(&mut self, other: Self) {
        self.elements = rationals_cross(&self.elements, &other.elements);
    }
}

impl CrossAssign<&Self> for RationalVector {
    /// Replaces a 3-dimensional [`RationalVector`] with its cross product with another, taking the
    /// other by reference.
    ///
    /// $$
    /// v \\gets v \\times w = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0).
    /// $$
    ///
    /// The result is perpendicular to both vectors, its length is the area of the parallelogram
    /// they span, and swapping the vectors negates it.
    ///
    /// # Worst-case complexity
    /// $T(m) = O(m \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $m$ is the largest number of significant
    /// bits of the numerator or denominator of any element.
    ///
    /// # Panics
    /// Panics if either vector does not have dimension 3.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CrossAssign;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1/2, 0, 1)").unwrap();
    /// v.cross_assign(&RationalVector::from_str("(0, 2/3, -1)").unwrap());
    /// assert_eq!(v.to_string(), "(-2/3, 1/2, 1/3)");
    ///
    /// // The cross product of the first two standard basis vectors is the third.
    /// let mut v = RationalVector::from_str("(1, 0, 0)").unwrap();
    /// v.cross_assign(&RationalVector::from_str("(0, 1, 0)").unwrap());
    /// assert_eq!(v.to_string(), "(0, 0, 1)");
    /// ```
    #[inline]
    fn cross_assign(&mut self, other: &Self) {
        self.elements = rationals_cross(&self.elements, &other.elements);
    }
}
