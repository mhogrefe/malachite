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
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::{
    Dot, ModPowerOf2, ModPowerOf2Add, ModPowerOf2Dot, ModPowerOf2IsReduced, ModPowerOf2Sub,
    WrappingAddAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::WrappingFrom;

// Checks the arguments of a dot product modulo $2^k$: the vectors have the same dimension, and
// their elements are reduced. `v_name` and `w_name` name the vectors in the panic messages.
fn assert_mod_power_of_2_dot_arguments(
    v: &NaturalVector,
    w: &NaturalVector,
    pow: u64,
    v_name: &str,
    w_name: &str,
) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "the vectors must have the same dimension"
    );
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "{v_name} must be reduced mod 2^{pow}, but {v} has an element >= 2^{pow}"
    );
    assert!(
        w.mod_power_of_2_is_reduced(pow),
        "{w_name} must be reduced mod 2^{pow}, but {w} has an element >= 2^{pow}"
    );
}

// The dot product of two vectors modulo $2^k$, with `w` read backwards if `reverse`; the arguments
// have already been checked. When $k$ is at most the width of a limb, every element fits in a limb,
// and the products are summed in wrapping limb arithmetic, since $2^k$ divides the limb modulus;
// otherwise the exact dot product is reduced once.
fn mod_power_of_2_dot_checked(
    v: &NaturalVector,
    w: &NaturalVector,
    reverse: bool,
    pow: u64,
) -> Natural {
    if pow <= Limb::WIDTH {
        let mut sum: Limb = 0;
        let mut add = |x: &Natural, y: &Natural| {
            sum.wrapping_add_assign(Limb::wrapping_from(x).wrapping_mul(Limb::wrapping_from(y)));
        };
        if reverse {
            for (x, y) in v.elements.iter().zip(w.elements.iter().rev()) {
                add(x, y);
            }
        } else {
            for (x, y) in v.elements.iter().zip(&w.elements) {
                add(x, y);
            }
        }
        Natural::from(sum.mod_power_of_2(pow))
    } else if reverse {
        vec_dot_general(None, false, &v.elements, &w.elements, true).mod_power_of_2(pow)
    } else {
        v.dot(w).mod_power_of_2(pow)
    }
}

// The dot product of two vectors modulo $2^k$, after checking the arguments.
fn mod_power_of_2_dot(v: &NaturalVector, w: &NaturalVector, pow: u64) -> Natural {
    assert_mod_power_of_2_dot_arguments(v, w, pow, "self", "other");
    mod_power_of_2_dot_checked(v, w, false, pow)
}

// The dot product of `v` and the reverse of `w` modulo $2^k$, after checking the arguments.
pub(crate) fn mod_power_of_2_dot_reversed_vectors(
    v: &NaturalVector,
    w: &NaturalVector,
    pow: u64,
) -> Natural {
    assert_mod_power_of_2_dot_arguments(v, w, pow, "self", "other");
    mod_power_of_2_dot_checked(v, w, true, pow)
}

// `x` plus, or if `subtract` minus, the dot product of `v` and `w` modulo $2^k$, with `w` read
// backwards if `reverse`, after checking the arguments.
pub(crate) fn mod_power_of_2_dot_general(
    x: &Natural,
    subtract: bool,
    v: &NaturalVector,
    w: &NaturalVector,
    reverse: bool,
    pow: u64,
) -> Natural {
    assert_mod_power_of_2_dot_arguments(v, w, pow, "v", "w");
    assert!(
        x.mod_power_of_2_is_reduced(pow),
        "x must be reduced mod 2^{pow}, but {x} >= 2^{pow}"
    );
    let d = mod_power_of_2_dot_checked(v, w, reverse, pow);
    if subtract {
        x.mod_power_of_2_sub(d, pow)
    } else {
        x.mod_power_of_2_add(d, pow)
    }
}

impl ModPowerOf2Dot<Self> for NaturalVector {
    type Output = Natural;

    /// Computes the dot product of two [`NaturalVector`]s modulo $2^k$, taking the first by value
    /// and the second by value. The elements of both must already be reduced.
    ///
    /// The dot product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w, k) = \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $k$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if any element is greater than or equal
    /// to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Dot;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(v.clone().mod_power_of_2_dot(w.clone(), 4), 3);
    /// ```
    #[inline]
    fn mod_power_of_2_dot(self, other: Self, pow: u64) -> Natural {
        mod_power_of_2_dot(&self, &other, pow)
    }
}

impl ModPowerOf2Dot<&Self> for NaturalVector {
    type Output = Natural;

    /// Computes the dot product of two [`NaturalVector`]s modulo $2^k$, taking the first by value
    /// and the second by reference. The elements of both must already be reduced.
    ///
    /// The dot product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w, k) = \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $k$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if any element is greater than or equal
    /// to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Dot;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(v.clone().mod_power_of_2_dot(&w, 4), 3);
    /// ```
    #[inline]
    fn mod_power_of_2_dot(self, other: &Self, pow: u64) -> Natural {
        mod_power_of_2_dot(&self, other, pow)
    }
}

impl ModPowerOf2Dot<NaturalVector> for &NaturalVector {
    type Output = Natural;

    /// Computes the dot product of two [`NaturalVector`]s modulo $2^k$, taking the first by
    /// reference and the second by value. The elements of both must already be reduced.
    ///
    /// The dot product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w, k) = \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $k$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if any element is greater than or equal
    /// to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Dot;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!((&v).mod_power_of_2_dot(w.clone(), 4), 3);
    /// ```
    #[inline]
    fn mod_power_of_2_dot(self, other: NaturalVector, pow: u64) -> Natural {
        mod_power_of_2_dot(self, &other, pow)
    }
}

impl ModPowerOf2Dot<&NaturalVector> for &NaturalVector {
    type Output = Natural;

    /// Computes the dot product of two [`NaturalVector`]s modulo $2^k$, taking the first by
    /// reference and the second by reference. The elements of both must already be reduced.
    ///
    /// The dot product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w, k) = \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $k$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if any element is greater than or equal
    /// to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Dot;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!((&v).mod_power_of_2_dot(&w, 4), 3);
    /// ```
    #[inline]
    fn mod_power_of_2_dot(self, other: &NaturalVector, pow: u64) -> Natural {
        mod_power_of_2_dot(self, other, pow)
    }
}
