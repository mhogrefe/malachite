// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use crate::natural_vector::arithmetic::mod_dot::mod_dot_reversed_vectors;
use malachite_base::num::arithmetic::traits::ModDotReversed;

impl ModDotReversed<Self, Natural> for NaturalVector {
    type Output = Natural;

    /// Computes the dot product of a [`NaturalVector`] and the reverse of another modulo $m$,
    /// taking the first by value and the second by value, and the modulus by value. The elements
    /// must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(v, w, m) = \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod m,
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
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     v.clone().mod_dot_reversed(w.clone(), Natural::from(11u32)),
    ///     7u32
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_dot_rev` from `fmpz_mod_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn mod_dot_reversed(self, other: Self, m: Natural) -> Natural {
        mod_dot_reversed_vectors(&self, &other, &m)
    }
}

impl ModDotReversed<Self, &Natural> for NaturalVector {
    type Output = Natural;

    /// Computes the dot product of a [`NaturalVector`] and the reverse of another modulo $m$,
    /// taking the first by value and the second by value, and the modulus by reference. The
    /// elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(v, w, m) = \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod m,
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
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     v.clone().mod_dot_reversed(w.clone(), &Natural::from(11u32)),
    ///     7u32
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_dot_rev` from `fmpz_mod_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn mod_dot_reversed(self, other: Self, m: &Natural) -> Natural {
        mod_dot_reversed_vectors(&self, &other, m)
    }
}

impl ModDotReversed<&Self, Natural> for NaturalVector {
    type Output = Natural;

    /// Computes the dot product of a [`NaturalVector`] and the reverse of another modulo $m$,
    /// taking the first by value and the second by reference, and the modulus by value. The
    /// elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(v, w, m) = \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod m,
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
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(v.clone().mod_dot_reversed(&w, Natural::from(11u32)), 7u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_dot_rev` from `fmpz_mod_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn mod_dot_reversed(self, other: &Self, m: Natural) -> Natural {
        mod_dot_reversed_vectors(&self, other, &m)
    }
}

impl ModDotReversed<&Self, &Natural> for NaturalVector {
    type Output = Natural;

    /// Computes the dot product of a [`NaturalVector`] and the reverse of another modulo $m$,
    /// taking the first by value and the second by reference, and the modulus by reference. The
    /// elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(v, w, m) = \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod m,
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
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(v.clone().mod_dot_reversed(&w, &Natural::from(11u32)), 7u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_dot_rev` from `fmpz_mod_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn mod_dot_reversed(self, other: &Self, m: &Natural) -> Natural {
        mod_dot_reversed_vectors(&self, other, m)
    }
}

impl ModDotReversed<NaturalVector, Natural> for &NaturalVector {
    type Output = Natural;

    /// Computes the dot product of a [`NaturalVector`] and the reverse of another modulo $m$,
    /// taking the first by reference and the second by value, and the modulus by value. The
    /// elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(v, w, m) = \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod m,
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
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!((&v).mod_dot_reversed(w.clone(), Natural::from(11u32)), 7u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_dot_rev` from `fmpz_mod_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn mod_dot_reversed(self, other: NaturalVector, m: Natural) -> Natural {
        mod_dot_reversed_vectors(self, &other, &m)
    }
}

impl ModDotReversed<NaturalVector, &Natural> for &NaturalVector {
    type Output = Natural;

    /// Computes the dot product of a [`NaturalVector`] and the reverse of another modulo $m$,
    /// taking the first by reference and the second by value, and the modulus by reference. The
    /// elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(v, w, m) = \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod m,
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
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_dot_reversed(w.clone(), &Natural::from(11u32)),
    ///     7u32
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_dot_rev` from `fmpz_mod_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn mod_dot_reversed(self, other: NaturalVector, m: &Natural) -> Natural {
        mod_dot_reversed_vectors(self, &other, m)
    }
}

impl ModDotReversed<&NaturalVector, Natural> for &NaturalVector {
    type Output = Natural;

    /// Computes the dot product of a [`NaturalVector`] and the reverse of another modulo $m$,
    /// taking the first by reference and the second by reference, and the modulus by value. The
    /// elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(v, w, m) = \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod m,
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
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!((&v).mod_dot_reversed(&w, Natural::from(11u32)), 7u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_dot_rev` from `fmpz_mod_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn mod_dot_reversed(self, other: &NaturalVector, m: Natural) -> Natural {
        mod_dot_reversed_vectors(self, other, &m)
    }
}

impl ModDotReversed<&NaturalVector, &Natural> for &NaturalVector {
    type Output = Natural;

    /// Computes the dot product of a [`NaturalVector`] and the reverse of another modulo $m$,
    /// taking the first by reference and the second by reference, and the modulus by reference. The
    /// elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(v, w, m) = \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod m,
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
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!((&v).mod_dot_reversed(&w, &Natural::from(11u32)), 7u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_dot_rev` from `fmpz_mod_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn mod_dot_reversed(self, other: &NaturalVector, m: &Natural) -> Natural {
        mod_dot_reversed_vectors(self, other, m)
    }
}
