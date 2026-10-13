// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use malachite_base::num::arithmetic::traits::{DotReversed, SubDotReversed, SubDotReversedAssign};

impl SubDotReversed<RationalVector, RationalVector> for Rational {
    type Output = Self;

    /// Subtracts the dot product of a [`RationalVector`] and the reverse of another from a
    /// [`Rational`], taking the [`Rational`] by value, the first vector by value, and the second by
    /// value.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(x, v, w) = x - \sum_{i=0}^{n-1} v_iw_{n-1-i},
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
    /// the largest number of significant bits of `x` and of the numerators the elements take over
    /// each vector's common denominator.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubDotReversed;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3)").unwrap();
    /// let w = RationalVector::from_str("(4/5, 3/2)").unwrap();
    /// assert_eq!(
    ///     Rational::from_str("1/3")
    ///         .unwrap()
    ///         .sub_dot_reversed(v.clone(), w.clone())
    ///         .to_string(),
    ///     "7/60"
    /// );
    /// ```
    #[inline]
    fn sub_dot_reversed(self, v: RationalVector, w: RationalVector) -> Self {
        self - v.dot_reversed(w)
    }
}

impl SubDotReversed<RationalVector, &RationalVector> for Rational {
    type Output = Self;

    /// Subtracts the dot product of a [`RationalVector`] and the reverse of another from a
    /// [`Rational`], taking the [`Rational`] by value, the first vector by value, and the second by
    /// reference.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(x, v, w) = x - \sum_{i=0}^{n-1} v_iw_{n-1-i},
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
    /// the largest number of significant bits of `x` and of the numerators the elements take over
    /// each vector's common denominator.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubDotReversed;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3)").unwrap();
    /// let w = RationalVector::from_str("(4/5, 3/2)").unwrap();
    /// assert_eq!(
    ///     Rational::from_str("1/3")
    ///         .unwrap()
    ///         .sub_dot_reversed(v.clone(), &w)
    ///         .to_string(),
    ///     "7/60"
    /// );
    /// ```
    #[inline]
    fn sub_dot_reversed(self, v: RationalVector, w: &RationalVector) -> Self {
        self - v.dot_reversed(w)
    }
}

impl SubDotReversed<&RationalVector, RationalVector> for Rational {
    type Output = Self;

    /// Subtracts the dot product of a [`RationalVector`] and the reverse of another from a
    /// [`Rational`], taking the [`Rational`] by value, the first vector by reference, and the
    /// second by value.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(x, v, w) = x - \sum_{i=0}^{n-1} v_iw_{n-1-i},
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
    /// the largest number of significant bits of `x` and of the numerators the elements take over
    /// each vector's common denominator.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubDotReversed;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3)").unwrap();
    /// let w = RationalVector::from_str("(4/5, 3/2)").unwrap();
    /// assert_eq!(
    ///     Rational::from_str("1/3")
    ///         .unwrap()
    ///         .sub_dot_reversed(&v, w.clone())
    ///         .to_string(),
    ///     "7/60"
    /// );
    /// ```
    #[inline]
    fn sub_dot_reversed(self, v: &RationalVector, w: RationalVector) -> Self {
        self - v.dot_reversed(w)
    }
}

impl SubDotReversed<&RationalVector, &RationalVector> for Rational {
    type Output = Self;

    /// Subtracts the dot product of a [`RationalVector`] and the reverse of another from a
    /// [`Rational`], taking the [`Rational`] by value, the first vector by reference, and the
    /// second by reference.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(x, v, w) = x - \sum_{i=0}^{n-1} v_iw_{n-1-i},
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
    /// the largest number of significant bits of `x` and of the numerators the elements take over
    /// each vector's common denominator.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubDotReversed;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3)").unwrap();
    /// let w = RationalVector::from_str("(4/5, 3/2)").unwrap();
    /// assert_eq!(
    ///     Rational::from_str("1/3")
    ///         .unwrap()
    ///         .sub_dot_reversed(&v, &w)
    ///         .to_string(),
    ///     "7/60"
    /// );
    /// ```
    #[inline]
    fn sub_dot_reversed(self, v: &RationalVector, w: &RationalVector) -> Self {
        self - v.dot_reversed(w)
    }
}

impl SubDotReversed<&RationalVector, &RationalVector> for &Rational {
    type Output = Rational;

    /// Subtracts the dot product of a [`RationalVector`] and the reverse of another from a
    /// [`Rational`], taking the [`Rational`] by reference, the first vector by reference, and the
    /// second by reference.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(x, v, w) = x - \sum_{i=0}^{n-1} v_iw_{n-1-i},
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
    /// the largest number of significant bits of `x` and of the numerators the elements take over
    /// each vector's common denominator.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubDotReversed;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3)").unwrap();
    /// let w = RationalVector::from_str("(4/5, 3/2)").unwrap();
    /// assert_eq!(
    ///     (&Rational::from_str("1/3").unwrap())
    ///         .sub_dot_reversed(&v, &w)
    ///         .to_string(),
    ///     "7/60"
    /// );
    /// ```
    #[inline]
    fn sub_dot_reversed(self, v: &RationalVector, w: &RationalVector) -> Rational {
        self - v.dot_reversed(w)
    }
}

impl SubDotReversedAssign<RationalVector, RationalVector> for Rational {
    /// Subtracts the dot product of a [`RationalVector`] and the reverse of another from a
    /// [`Rational`] in place, taking the first vector by value and the second by value.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// x \gets x - \sum_{i=0}^{n-1} v_iw_{n-1-i},
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
    /// the largest number of significant bits of `x` and of the numerators the elements take over
    /// each vector's common denominator.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubDotReversedAssign;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3)").unwrap();
    /// let w = RationalVector::from_str("(4/5, 3/2)").unwrap();
    /// let mut x = Rational::from_str("1/3").unwrap();
    /// x.sub_dot_reversed_assign(v.clone(), w.clone());
    /// assert_eq!(x.to_string(), "7/60");
    /// ```
    #[inline]
    fn sub_dot_reversed_assign(&mut self, v: RationalVector, w: RationalVector) {
        *self -= v.dot_reversed(w);
    }
}

impl SubDotReversedAssign<RationalVector, &RationalVector> for Rational {
    /// Subtracts the dot product of a [`RationalVector`] and the reverse of another from a
    /// [`Rational`] in place, taking the first vector by value and the second by reference.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// x \gets x - \sum_{i=0}^{n-1} v_iw_{n-1-i},
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
    /// the largest number of significant bits of `x` and of the numerators the elements take over
    /// each vector's common denominator.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubDotReversedAssign;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3)").unwrap();
    /// let w = RationalVector::from_str("(4/5, 3/2)").unwrap();
    /// let mut x = Rational::from_str("1/3").unwrap();
    /// x.sub_dot_reversed_assign(v.clone(), &w);
    /// assert_eq!(x.to_string(), "7/60");
    /// ```
    #[inline]
    fn sub_dot_reversed_assign(&mut self, v: RationalVector, w: &RationalVector) {
        *self -= v.dot_reversed(w);
    }
}

impl SubDotReversedAssign<&RationalVector, RationalVector> for Rational {
    /// Subtracts the dot product of a [`RationalVector`] and the reverse of another from a
    /// [`Rational`] in place, taking the first vector by reference and the second by value.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// x \gets x - \sum_{i=0}^{n-1} v_iw_{n-1-i},
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
    /// the largest number of significant bits of `x` and of the numerators the elements take over
    /// each vector's common denominator.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubDotReversedAssign;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3)").unwrap();
    /// let w = RationalVector::from_str("(4/5, 3/2)").unwrap();
    /// let mut x = Rational::from_str("1/3").unwrap();
    /// x.sub_dot_reversed_assign(&v, w.clone());
    /// assert_eq!(x.to_string(), "7/60");
    /// ```
    #[inline]
    fn sub_dot_reversed_assign(&mut self, v: &RationalVector, w: RationalVector) {
        *self -= v.dot_reversed(w);
    }
}

impl SubDotReversedAssign<&RationalVector, &RationalVector> for Rational {
    /// Subtracts the dot product of a [`RationalVector`] and the reverse of another from a
    /// [`Rational`] in place, taking the first vector by reference and the second by reference.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// x \gets x - \sum_{i=0}^{n-1} v_iw_{n-1-i},
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
    /// the largest number of significant bits of `x` and of the numerators the elements take over
    /// each vector's common denominator.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubDotReversedAssign;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3)").unwrap();
    /// let w = RationalVector::from_str("(4/5, 3/2)").unwrap();
    /// let mut x = Rational::from_str("1/3").unwrap();
    /// x.sub_dot_reversed_assign(&v, &w);
    /// assert_eq!(x.to_string(), "7/60");
    /// ```
    #[inline]
    fn sub_dot_reversed_assign(&mut self, v: &RationalVector, w: &RationalVector) {
        *self -= v.dot_reversed(w);
    }
}
