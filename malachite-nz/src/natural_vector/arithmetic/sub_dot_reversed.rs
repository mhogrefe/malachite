// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{DotReversed, SubDotReversed, SubDotReversedAssign};

// `x` minus the reversed dot product of `v` and `w`; `Natural` subtraction panics if the dot
// product is greater than `x`.
fn sub_dot_reversed(x: &Natural, v: &NaturalVector, w: &NaturalVector) -> Natural {
    x - v.dot_reversed(w)
}

impl SubDotReversed<NaturalVector, NaturalVector> for Natural {
    type Output = Self;

    /// Subtracts the dot product of a [`NaturalVector`] and the reverse of another from a
    /// [`Natural`], taking the [`Natural`] by value, the first vector by value, and the second by
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
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of `x` and of any element.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if the dot product is greater than `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubDotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// assert_eq!(
    ///     Natural::from(100u32).sub_dot_reversed(v.clone(), w.clone()),
    ///     72u32
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with
    /// `initial` set to `x`, `subtract` set to 1, and `reverse` set to 1.
    #[inline]
    fn sub_dot_reversed(self, v: NaturalVector, w: NaturalVector) -> Self {
        sub_dot_reversed(&self, &v, &w)
    }
}

impl SubDotReversed<NaturalVector, &NaturalVector> for Natural {
    type Output = Self;

    /// Subtracts the dot product of a [`NaturalVector`] and the reverse of another from a
    /// [`Natural`], taking the [`Natural`] by value, the first vector by value, and the second by
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
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of `x` and of any element.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if the dot product is greater than `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubDotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// assert_eq!(Natural::from(100u32).sub_dot_reversed(v.clone(), &w), 72u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with
    /// `initial` set to `x`, `subtract` set to 1, and `reverse` set to 1.
    #[inline]
    fn sub_dot_reversed(self, v: NaturalVector, w: &NaturalVector) -> Self {
        sub_dot_reversed(&self, &v, w)
    }
}

impl SubDotReversed<&NaturalVector, NaturalVector> for Natural {
    type Output = Self;

    /// Subtracts the dot product of a [`NaturalVector`] and the reverse of another from a
    /// [`Natural`], taking the [`Natural`] by value, the first vector by reference, and the second
    /// by value.
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
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of `x` and of any element.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if the dot product is greater than `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubDotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// assert_eq!(Natural::from(100u32).sub_dot_reversed(&v, w.clone()), 72u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with
    /// `initial` set to `x`, `subtract` set to 1, and `reverse` set to 1.
    #[inline]
    fn sub_dot_reversed(self, v: &NaturalVector, w: NaturalVector) -> Self {
        sub_dot_reversed(&self, v, &w)
    }
}

impl SubDotReversed<&NaturalVector, &NaturalVector> for Natural {
    type Output = Self;

    /// Subtracts the dot product of a [`NaturalVector`] and the reverse of another from a
    /// [`Natural`], taking the [`Natural`] by value, the first vector by reference, and the second
    /// by reference.
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
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of `x` and of any element.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if the dot product is greater than `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubDotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// assert_eq!(Natural::from(100u32).sub_dot_reversed(&v, &w), 72u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with
    /// `initial` set to `x`, `subtract` set to 1, and `reverse` set to 1.
    #[inline]
    fn sub_dot_reversed(self, v: &NaturalVector, w: &NaturalVector) -> Self {
        sub_dot_reversed(&self, v, w)
    }
}

impl SubDotReversed<&NaturalVector, &NaturalVector> for &Natural {
    type Output = Natural;

    /// Subtracts the dot product of a [`NaturalVector`] and the reverse of another from a
    /// [`Natural`], taking the [`Natural`] by reference, the first vector by reference, and the
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
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of `x` and of any element.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if the dot product is greater than `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubDotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// assert_eq!((&Natural::from(100u32)).sub_dot_reversed(&v, &w), 72u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with
    /// `initial` set to `x`, `subtract` set to 1, and `reverse` set to 1.
    #[inline]
    fn sub_dot_reversed(self, v: &NaturalVector, w: &NaturalVector) -> Natural {
        sub_dot_reversed(self, v, w)
    }
}

impl SubDotReversedAssign<NaturalVector, NaturalVector> for Natural {
    /// Subtracts the dot product of a [`NaturalVector`] and the reverse of another from a
    /// [`Natural`] in place, taking the first vector by value and the second by value.
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
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of `x` and of any element.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if the dot product is greater than `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubDotReversedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// let mut x = Natural::from(100u32);
    /// x.sub_dot_reversed_assign(v.clone(), w.clone());
    /// assert_eq!(x, 72u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with
    /// `initial` set to `x`, `subtract` set to 1, and `reverse` set to 1.
    #[inline]
    fn sub_dot_reversed_assign(&mut self, v: NaturalVector, w: NaturalVector) {
        *self = sub_dot_reversed(self, &v, &w);
    }
}

impl SubDotReversedAssign<NaturalVector, &NaturalVector> for Natural {
    /// Subtracts the dot product of a [`NaturalVector`] and the reverse of another from a
    /// [`Natural`] in place, taking the first vector by value and the second by reference.
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
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of `x` and of any element.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if the dot product is greater than `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubDotReversedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// let mut x = Natural::from(100u32);
    /// x.sub_dot_reversed_assign(v.clone(), &w);
    /// assert_eq!(x, 72u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with
    /// `initial` set to `x`, `subtract` set to 1, and `reverse` set to 1.
    #[inline]
    fn sub_dot_reversed_assign(&mut self, v: NaturalVector, w: &NaturalVector) {
        *self = sub_dot_reversed(self, &v, w);
    }
}

impl SubDotReversedAssign<&NaturalVector, NaturalVector> for Natural {
    /// Subtracts the dot product of a [`NaturalVector`] and the reverse of another from a
    /// [`Natural`] in place, taking the first vector by reference and the second by value.
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
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of `x` and of any element.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if the dot product is greater than `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubDotReversedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// let mut x = Natural::from(100u32);
    /// x.sub_dot_reversed_assign(&v, w.clone());
    /// assert_eq!(x, 72u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with
    /// `initial` set to `x`, `subtract` set to 1, and `reverse` set to 1.
    #[inline]
    fn sub_dot_reversed_assign(&mut self, v: &NaturalVector, w: NaturalVector) {
        *self = sub_dot_reversed(self, v, &w);
    }
}

impl SubDotReversedAssign<&NaturalVector, &NaturalVector> for Natural {
    /// Subtracts the dot product of a [`NaturalVector`] and the reverse of another from a
    /// [`Natural`] in place, taking the first vector by reference and the second by reference.
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
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of `x` and of any element.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if the dot product is greater than `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SubDotReversedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// let mut x = Natural::from(100u32);
    /// x.sub_dot_reversed_assign(&v, &w);
    /// assert_eq!(x, 72u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with
    /// `initial` set to `x`, `subtract` set to 1, and `reverse` set to 1.
    #[inline]
    fn sub_dot_reversed_assign(&mut self, v: &NaturalVector, w: &NaturalVector) {
        *self = sub_dot_reversed(self, v, w);
    }
}
