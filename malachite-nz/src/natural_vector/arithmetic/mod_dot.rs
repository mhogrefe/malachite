// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_vector::arithmetic::dot_general::vec_dot_general;
use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    Dot, Mod, ModAdd, ModDot, ModDotReversed, ModIsReduced, ModSub,
};
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

// Checks the arguments of a modular dot product: `m` is positive, the vectors have the same
// dimension, and their elements are reduced. `v_name` and `w_name` name the vectors in the panic
// messages.
fn assert_mod_dot_arguments(
    v: &NaturalVector,
    w: &NaturalVector,
    m: &Natural,
    v_name: &str,
    w_name: &str,
) {
    assert_ne!(*m, 0u32, "m cannot be zero");
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "the vectors must have the same dimension"
    );
    assert!(
        v.mod_is_reduced(m),
        "{v_name} must be reduced mod m, but {v} has an element >= {m}"
    );
    assert!(
        w.mod_is_reduced(m),
        "{w_name} must be reduced mod m, but {w} has an element >= {m}"
    );
}

// The dot product of two vectors modulo `m`, with `w` read backwards if `reverse`; the arguments
// have already been checked. When `m` fits in a limb, so does every element, and the products are
// summed exactly in one to three limbs and reduced once, by the `UnsignedVector` implementation;
// otherwise the exact dot product is reduced once.
fn mod_dot_checked(v: &NaturalVector, w: &NaturalVector, reverse: bool, m: &Natural) -> Natural {
    if let Natural(Small(small_m)) = *m {
        let (v, w) = (to_limb_vector(v), to_limb_vector(w));
        Natural::from(if reverse {
            v.mod_dot_reversed(w, small_m)
        } else {
            v.mod_dot(w, small_m)
        })
    } else if reverse {
        vec_dot_general(None, false, &v.elements, &w.elements, true).mod_op(m)
    } else {
        v.dot(w).mod_op(m)
    }
}

// The dot product of two vectors modulo `m`, after checking the arguments.
fn mod_dot(v: &NaturalVector, w: &NaturalVector, m: &Natural) -> Natural {
    assert_mod_dot_arguments(v, w, m, "self", "other");
    mod_dot_checked(v, w, false, m)
}

// The dot product of `v` and the reverse of `w` modulo `m`, after checking the arguments.
pub(crate) fn mod_dot_reversed_vectors(
    v: &NaturalVector,
    w: &NaturalVector,
    m: &Natural,
) -> Natural {
    assert_mod_dot_arguments(v, w, m, "self", "other");
    mod_dot_checked(v, w, true, m)
}

// `x` plus, or if `subtract` minus, the dot product of `v` and `w` modulo `m`, with `w` read
// backwards if `reverse`, after checking the arguments.
pub(crate) fn mod_dot_general(
    x: &Natural,
    subtract: bool,
    v: &NaturalVector,
    w: &NaturalVector,
    reverse: bool,
    m: &Natural,
) -> Natural {
    assert_mod_dot_arguments(v, w, m, "v", "w");
    assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
    let d = mod_dot_checked(v, w, reverse, m);
    if subtract {
        x.mod_sub(d, m)
    } else {
        x.mod_add(d, m)
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
