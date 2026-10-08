// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_vector::NaturalVector;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2Sub, ModPowerOf2SubAssign,
};

fn assert_reduced(v: &NaturalVector, w: &NaturalVector, pow: u64) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot subtract vectors of different dimensions"
    );
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {v} has an element >= 2^{pow}"
    );
    assert!(
        w.mod_power_of_2_is_reduced(pow),
        "other must be reduced mod 2^pow, but {w} has an element >= 2^{pow}"
    );
}

impl ModPowerOf2Sub<Self> for NaturalVector {
    type Output = Self;

    /// Subtracts two [`NaturalVector`]s modulo $2^k$, taking both by value. The elements of both
    /// must already be reduced modulo $2^k$.
    ///
    /// The difference is taken element by element, each reduced modulo $2^k$, so the result has the
    /// same dimension as the operands.
    ///
    /// $$
    /// f(v, w, k) = v - w \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times `pow`.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different dimensions, or if any element of either is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sub;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 0)").unwrap();
    /// assert_eq!(v.mod_power_of_2_sub(w, 3).to_string(), "(1, 2, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    #[inline]
    fn mod_power_of_2_sub(mut self, other: Self, pow: u64) -> Self {
        self.mod_power_of_2_sub_assign(other, pow);
        self
    }
}

impl ModPowerOf2Sub<&Self> for NaturalVector {
    type Output = Self;

    /// Subtracts two [`NaturalVector`]s modulo $2^k$, taking the first by value and the second by
    /// reference. The elements of both must already be reduced modulo $2^k$.
    ///
    /// The difference is taken element by element, each reduced modulo $2^k$, so the result has the
    /// same dimension as the operands.
    ///
    /// $$
    /// f(v, w, k) = v - w \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times `pow`.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different dimensions, or if any element of either is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sub;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 0)").unwrap();
    /// assert_eq!(v.mod_power_of_2_sub(&w, 3).to_string(), "(1, 2, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    #[inline]
    fn mod_power_of_2_sub(mut self, other: &Self, pow: u64) -> Self {
        self.mod_power_of_2_sub_assign(other, pow);
        self
    }
}

impl ModPowerOf2Sub<NaturalVector> for &NaturalVector {
    type Output = NaturalVector;

    /// Subtracts two [`NaturalVector`]s modulo $2^k$, taking the first by reference and the second
    /// by value. The elements of both must already be reduced modulo $2^k$.
    ///
    /// The difference is taken element by element, each reduced modulo $2^k$, so the result has the
    /// same dimension as the operands.
    ///
    /// $$
    /// f(v, w, k) = v - w \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times `pow`.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different dimensions, or if any element of either is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sub;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 0)").unwrap();
    /// assert_eq!((&v).mod_power_of_2_sub(w, 3).to_string(), "(1, 2, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_sub(self, mut other: NaturalVector, pow: u64) -> NaturalVector {
        assert_reduced(self, &other, pow);
        for (x, y) in other.elements.iter_mut().zip(&self.elements) {
            *x = y.mod_power_of_2_sub(take(x), pow);
        }
        other
    }
}

impl ModPowerOf2Sub<&NaturalVector> for &NaturalVector {
    type Output = NaturalVector;

    /// Subtracts two [`NaturalVector`]s modulo $2^k$, taking both by reference. The elements of
    /// both must already be reduced modulo $2^k$.
    ///
    /// The difference is taken element by element, each reduced modulo $2^k$, so the result has the
    /// same dimension as the operands.
    ///
    /// $$
    /// f(v, w, k) = v - w \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times `pow`.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different dimensions, or if any element of either is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sub;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 0)").unwrap();
    /// assert_eq!((&v).mod_power_of_2_sub(&w, 3).to_string(), "(1, 2, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_sub(self, other: &NaturalVector, pow: u64) -> NaturalVector {
        assert_reduced(self, other, pow);
        NaturalVector {
            elements: self
                .elements
                .iter()
                .zip(&other.elements)
                .map(|(x, y)| x.mod_power_of_2_sub(y, pow))
                .collect(),
        }
    }
}

impl ModPowerOf2SubAssign<Self> for NaturalVector {
    /// Subtracts a [`NaturalVector`] from a [`NaturalVector`] modulo $2^k$, in place, taking the
    /// [`NaturalVector`] on the right-hand side by value. The elements of both must already be
    /// reduced modulo $2^k$.
    ///
    /// The difference is taken element by element, each reduced modulo $2^k$, so the result has the
    /// same dimension as the operands.
    ///
    /// $$
    /// f(v, w, k) = v - w \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times `pow`.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different dimensions, or if any element of either is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubAssign;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// v.mod_power_of_2_sub_assign(NaturalVector::from_str("(4, 7, 0)").unwrap(), 3);
    /// assert_eq!(v.to_string(), "(1, 2, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_sub_assign(&mut self, other: Self, pow: u64) {
        assert_reduced(self, &other, pow);
        for (x, y) in self.elements.iter_mut().zip(other.elements) {
            x.mod_power_of_2_sub_assign(y, pow);
        }
    }
}

impl ModPowerOf2SubAssign<&Self> for NaturalVector {
    /// Subtracts a [`NaturalVector`] from a [`NaturalVector`] modulo $2^k$, in place, taking the
    /// [`NaturalVector`] on the right-hand side by reference. The elements of both must already be
    /// reduced modulo $2^k$.
    ///
    /// The difference is taken element by element, each reduced modulo $2^k$, so the result has the
    /// same dimension as the operands.
    ///
    /// $$
    /// f(v, w, k) = v - w \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times `pow`.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different dimensions, or if any element of either is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubAssign;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// v.mod_power_of_2_sub_assign(&NaturalVector::from_str("(4, 7, 0)").unwrap(), 3);
    /// assert_eq!(v.to_string(), "(1, 2, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_sub_assign(&mut self, other: &Self, pow: u64) {
        assert_reduced(self, other, pow);
        for (x, y) in self.elements.iter_mut().zip(&other.elements) {
            x.mod_power_of_2_sub_assign(y, pow);
        }
    }
}
