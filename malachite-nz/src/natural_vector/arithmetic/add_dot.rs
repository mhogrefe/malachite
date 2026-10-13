// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_vector::arithmetic::dot_general::vec_dot_general;
use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{AddDot, AddDotAssign};

// `x` plus the dot product of `v` and `w`.
fn add_dot(x: &Natural, v: &NaturalVector, w: &NaturalVector) -> Natural {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "the vectors must have the same dimension"
    );
    vec_dot_general(Some(x), false, &v.elements, &w.elements, false)
}

impl AddDot<NaturalVector, NaturalVector> for Natural {
    type Output = Self;

    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`], taking the [`Natural`] by
    /// value, the first vector by value, and the second by value.
    ///
    /// $$
    /// f(x, v, w) = x + \sum_{i=0}^{n-1} v_iw_i,
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
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::AddDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// assert_eq!(Natural::from(100u32).add_dot(v.clone(), w.clone()), 132u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with
    /// `initial` set to `x`, `subtract` set to 0, and `reverse` set to 0.
    #[inline]
    fn add_dot(self, v: NaturalVector, w: NaturalVector) -> Self {
        add_dot(&self, &v, &w)
    }
}

impl AddDot<NaturalVector, &NaturalVector> for Natural {
    type Output = Self;

    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`], taking the [`Natural`] by
    /// value, the first vector by value, and the second by reference.
    ///
    /// $$
    /// f(x, v, w) = x + \sum_{i=0}^{n-1} v_iw_i,
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
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::AddDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// assert_eq!(Natural::from(100u32).add_dot(v.clone(), &w), 132u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with
    /// `initial` set to `x`, `subtract` set to 0, and `reverse` set to 0.
    #[inline]
    fn add_dot(self, v: NaturalVector, w: &NaturalVector) -> Self {
        add_dot(&self, &v, w)
    }
}

impl AddDot<&NaturalVector, NaturalVector> for Natural {
    type Output = Self;

    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`], taking the [`Natural`] by
    /// value, the first vector by reference, and the second by value.
    ///
    /// $$
    /// f(x, v, w) = x + \sum_{i=0}^{n-1} v_iw_i,
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
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::AddDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// assert_eq!(Natural::from(100u32).add_dot(&v, w.clone()), 132u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with
    /// `initial` set to `x`, `subtract` set to 0, and `reverse` set to 0.
    #[inline]
    fn add_dot(self, v: &NaturalVector, w: NaturalVector) -> Self {
        add_dot(&self, v, &w)
    }
}

impl AddDot<&NaturalVector, &NaturalVector> for Natural {
    type Output = Self;

    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`], taking the [`Natural`] by
    /// value, the first vector by reference, and the second by reference.
    ///
    /// $$
    /// f(x, v, w) = x + \sum_{i=0}^{n-1} v_iw_i,
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
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::AddDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// assert_eq!(Natural::from(100u32).add_dot(&v, &w), 132u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with
    /// `initial` set to `x`, `subtract` set to 0, and `reverse` set to 0.
    #[inline]
    fn add_dot(self, v: &NaturalVector, w: &NaturalVector) -> Self {
        add_dot(&self, v, w)
    }
}

impl AddDot<&NaturalVector, &NaturalVector> for &Natural {
    type Output = Natural;

    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`], taking the [`Natural`] by
    /// reference, the first vector by reference, and the second by reference.
    ///
    /// $$
    /// f(x, v, w) = x + \sum_{i=0}^{n-1} v_iw_i,
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
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::AddDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// assert_eq!((&Natural::from(100u32)).add_dot(&v, &w), 132u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with
    /// `initial` set to `x`, `subtract` set to 0, and `reverse` set to 0.
    #[inline]
    fn add_dot(self, v: &NaturalVector, w: &NaturalVector) -> Natural {
        add_dot(self, v, w)
    }
}

impl AddDotAssign<NaturalVector, NaturalVector> for Natural {
    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] in place, taking the first
    /// vector by value and the second by value.
    ///
    /// $$
    /// x \gets x + \sum_{i=0}^{n-1} v_iw_i,
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
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::AddDotAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// let mut x = Natural::from(100u32);
    /// x.add_dot_assign(v.clone(), w.clone());
    /// assert_eq!(x, 132u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with
    /// `initial` set to `x`, `subtract` set to 0, and `reverse` set to 0.
    #[inline]
    fn add_dot_assign(&mut self, v: NaturalVector, w: NaturalVector) {
        *self = add_dot(self, &v, &w);
    }
}

impl AddDotAssign<NaturalVector, &NaturalVector> for Natural {
    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] in place, taking the first
    /// vector by value and the second by reference.
    ///
    /// $$
    /// x \gets x + \sum_{i=0}^{n-1} v_iw_i,
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
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::AddDotAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// let mut x = Natural::from(100u32);
    /// x.add_dot_assign(v.clone(), &w);
    /// assert_eq!(x, 132u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with
    /// `initial` set to `x`, `subtract` set to 0, and `reverse` set to 0.
    #[inline]
    fn add_dot_assign(&mut self, v: NaturalVector, w: &NaturalVector) {
        *self = add_dot(self, &v, w);
    }
}

impl AddDotAssign<&NaturalVector, NaturalVector> for Natural {
    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] in place, taking the first
    /// vector by reference and the second by value.
    ///
    /// $$
    /// x \gets x + \sum_{i=0}^{n-1} v_iw_i,
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
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::AddDotAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// let mut x = Natural::from(100u32);
    /// x.add_dot_assign(&v, w.clone());
    /// assert_eq!(x, 132u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with
    /// `initial` set to `x`, `subtract` set to 0, and `reverse` set to 0.
    #[inline]
    fn add_dot_assign(&mut self, v: &NaturalVector, w: NaturalVector) {
        *self = add_dot(self, v, &w);
    }
}

impl AddDotAssign<&NaturalVector, &NaturalVector> for Natural {
    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] in place, taking the first
    /// vector by reference and the second by reference.
    ///
    /// $$
    /// x \gets x + \sum_{i=0}^{n-1} v_iw_i,
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
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::AddDotAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 6)").unwrap();
    /// let mut x = Natural::from(100u32);
    /// x.add_dot_assign(&v, &w);
    /// assert_eq!(x, 132u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot_general` from `fmpz_vec/dot.c`, FLINT 3.6.0, with
    /// `initial` set to `x`, `subtract` set to 0, and `reverse` set to 0.
    #[inline]
    fn add_dot_assign(&mut self, v: &NaturalVector, w: &NaturalVector) {
        *self = add_dot(self, v, w);
    }
}
