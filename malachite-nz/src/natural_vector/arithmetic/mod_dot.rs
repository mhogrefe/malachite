// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{Dot, Mod, ModDot, ModIsReduced};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

// The elements of a vector whose elements all fit in a limb, as limbs.
fn to_limb_vector(v: &NaturalVector) -> UnsignedVector<Limb> {
    UnsignedVector::from_owned_elements(
        v.elements
            .iter()
            .map(Limb::wrapping_from)
            .collect::<Vec<_>>(),
    )
}

// The dot product of two vectors modulo `m`. When `m` fits in a limb, so does every element, and
// the products are summed exactly in one to three limbs and reduced once, by the `UnsignedVector`
// implementation; otherwise the exact dot product is reduced once.
fn mod_dot(v: &NaturalVector, w: &NaturalVector, m: &Natural) -> Natural {
    assert_ne!(*m, 0u32, "m cannot be zero");
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "the vectors must have the same dimension"
    );
    assert!(
        v.mod_is_reduced(m),
        "self must be reduced mod m, but {v} has an element >= {m}"
    );
    assert!(
        w.mod_is_reduced(m),
        "other must be reduced mod m, but {w} has an element >= {m}"
    );
    if let Natural(Small(small_m)) = *m {
        Natural::from(to_limb_vector(v).mod_dot(to_limb_vector(w), small_m))
    } else {
        v.dot(w).mod_op(m)
    }
}

impl ModDot<Self, Natural> for NaturalVector {
    type Output = Natural;

    /// Computes the dot product of two [`NaturalVector`]s modulo $m$, taking the first by value and
    /// the second by value, and the modulus by value. The elements of both must already be reduced.
    ///
    /// The dot product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w, m) = \sum_{i=0}^{n-1} v_iw_i \bmod m,
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
    /// use malachite_base::num::arithmetic::traits::ModDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(v.clone().mod_dot(w.clone(), Natural::from(10u32)), 5);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_dot` from `fmpz_mod_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn mod_dot(self, other: Self, m: Natural) -> Natural {
        mod_dot(&self, &other, &m)
    }
}

impl ModDot<Self, &Natural> for NaturalVector {
    type Output = Natural;

    /// Computes the dot product of two [`NaturalVector`]s modulo $m$, taking the first by value and
    /// the second by value, and the modulus by reference. The elements of both must already be
    /// reduced.
    ///
    /// The dot product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w, m) = \sum_{i=0}^{n-1} v_iw_i \bmod m,
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
    /// use malachite_base::num::arithmetic::traits::ModDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(v.clone().mod_dot(w.clone(), &Natural::from(10u32)), 5);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_dot` from `fmpz_mod_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn mod_dot(self, other: Self, m: &Natural) -> Natural {
        mod_dot(&self, &other, m)
    }
}

impl ModDot<&Self, Natural> for NaturalVector {
    type Output = Natural;

    /// Computes the dot product of two [`NaturalVector`]s modulo $m$, taking the first by value and
    /// the second by reference, and the modulus by value. The elements of both must already be
    /// reduced.
    ///
    /// The dot product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w, m) = \sum_{i=0}^{n-1} v_iw_i \bmod m,
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
    /// use malachite_base::num::arithmetic::traits::ModDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(v.clone().mod_dot(&w, Natural::from(10u32)), 5);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_dot` from `fmpz_mod_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn mod_dot(self, other: &Self, m: Natural) -> Natural {
        mod_dot(&self, other, &m)
    }
}

impl ModDot<&Self, &Natural> for NaturalVector {
    type Output = Natural;

    /// Computes the dot product of two [`NaturalVector`]s modulo $m$, taking the first by value and
    /// the second by reference, and the modulus by reference. The elements of both must already be
    /// reduced.
    ///
    /// The dot product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w, m) = \sum_{i=0}^{n-1} v_iw_i \bmod m,
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
    /// use malachite_base::num::arithmetic::traits::ModDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(v.clone().mod_dot(&w, &Natural::from(10u32)), 5);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_dot` from `fmpz_mod_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn mod_dot(self, other: &Self, m: &Natural) -> Natural {
        mod_dot(&self, other, m)
    }
}

impl ModDot<NaturalVector, Natural> for &NaturalVector {
    type Output = Natural;

    /// Computes the dot product of two [`NaturalVector`]s modulo $m$, taking the first by reference
    /// and the second by value, and the modulus by value. The elements of both must already be
    /// reduced.
    ///
    /// The dot product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w, m) = \sum_{i=0}^{n-1} v_iw_i \bmod m,
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
    /// use malachite_base::num::arithmetic::traits::ModDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!((&v).mod_dot(w.clone(), Natural::from(10u32)), 5);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_dot` from `fmpz_mod_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn mod_dot(self, other: NaturalVector, m: Natural) -> Natural {
        mod_dot(self, &other, &m)
    }
}

impl ModDot<NaturalVector, &Natural> for &NaturalVector {
    type Output = Natural;

    /// Computes the dot product of two [`NaturalVector`]s modulo $m$, taking the first by reference
    /// and the second by value, and the modulus by reference. The elements of both must already be
    /// reduced.
    ///
    /// The dot product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w, m) = \sum_{i=0}^{n-1} v_iw_i \bmod m,
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
    /// use malachite_base::num::arithmetic::traits::ModDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!((&v).mod_dot(w.clone(), &Natural::from(10u32)), 5);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_dot` from `fmpz_mod_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn mod_dot(self, other: NaturalVector, m: &Natural) -> Natural {
        mod_dot(self, &other, m)
    }
}

impl ModDot<&NaturalVector, Natural> for &NaturalVector {
    type Output = Natural;

    /// Computes the dot product of two [`NaturalVector`]s modulo $m$, taking the first by reference
    /// and the second by reference, and the modulus by value. The elements of both must already be
    /// reduced.
    ///
    /// The dot product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w, m) = \sum_{i=0}^{n-1} v_iw_i \bmod m,
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
    /// use malachite_base::num::arithmetic::traits::ModDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!((&v).mod_dot(&w, Natural::from(10u32)), 5);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_dot` from `fmpz_mod_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn mod_dot(self, other: &NaturalVector, m: Natural) -> Natural {
        mod_dot(self, other, &m)
    }
}

impl ModDot<&NaturalVector, &Natural> for &NaturalVector {
    type Output = Natural;

    /// Computes the dot product of two [`NaturalVector`]s modulo $m$, taking the first by reference
    /// and the second by reference, and the modulus by reference. The elements of both must already
    /// be reduced.
    ///
    /// The dot product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w, m) = \sum_{i=0}^{n-1} v_iw_i \bmod m,
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
    /// use malachite_base::num::arithmetic::traits::ModDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!((&v).mod_dot(&w, &Natural::from(10u32)), 5);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_dot` from `fmpz_mod_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn mod_dot(self, other: &NaturalVector, m: &Natural) -> Natural {
        mod_dot(self, other, m)
    }
}
