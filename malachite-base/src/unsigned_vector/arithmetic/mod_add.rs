// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModAdd, ModAddAssign, ModIsReduced};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

fn assert_reduced<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, w: &UnsignedVector<T>, m: T) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot add vectors of different dimensions"
    );
    assert!(
        v.mod_is_reduced(&m),
        "self must be reduced mod m, but {v} has an element >= {m}"
    );
    assert!(
        w.mod_is_reduced(&m),
        "other must be reduced mod m, but {w} has an element >= {m}"
    );
}

fn mod_add_assign<T: PrimitiveUnsigned>(v: &mut UnsignedVector<T>, w: &UnsignedVector<T>, m: T) {
    assert_reduced(v, w, m);
    for (x, &y) in v.elements.iter_mut().zip(&w.elements) {
        x.mod_add_assign(y, m);
    }
}

impl<T: PrimitiveUnsigned> ModAdd<Self, T> for UnsignedVector<T> {
    type Output = Self;

    /// Adds two [`UnsignedVector`]s modulo $m$, taking the first by value and the second by value.
    /// The elements of both must already be reduced modulo $m$.
    ///
    /// The sum is taken element by element, each reduced modulo $m$, so the result has the same
    /// dimension as the summands.
    ///
    /// $$
    /// f(v, w, m) = v + w \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 7, 0)").unwrap();
    /// assert_eq!(v.mod_add(w, 8).to_string(), "(1, 0, 3)");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_add` from `nmod_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add(mut self, other: Self, m: T) -> Self {
        mod_add_assign(&mut self, &other, m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModAdd<&Self, T> for UnsignedVector<T> {
    type Output = Self;

    /// Adds two [`UnsignedVector`]s modulo $m$, taking the first by value and the second by
    /// reference. The elements of both must already be reduced modulo $m$.
    ///
    /// The sum is taken element by element, each reduced modulo $m$, so the result has the same
    /// dimension as the summands.
    ///
    /// $$
    /// f(v, w, m) = v + w \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 7, 0)").unwrap();
    /// assert_eq!(v.mod_add(&w, 8).to_string(), "(1, 0, 3)");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_add` from `nmod_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add(mut self, other: &Self, m: T) -> Self {
        mod_add_assign(&mut self, other, m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModAdd<UnsignedVector<T>, T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Adds two [`UnsignedVector`]s modulo $m$, taking the first by reference and the second by
    /// value. The elements of both must already be reduced modulo $m$.
    ///
    /// The sum is taken element by element, each reduced modulo $m$, so the result has the same
    /// dimension as the summands.
    ///
    /// $$
    /// f(v, w, m) = v + w \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 7, 0)").unwrap();
    /// assert_eq!((&v).mod_add(w, 8).to_string(), "(1, 0, 3)");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_add` from `nmod_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add(self, mut other: UnsignedVector<T>, m: T) -> UnsignedVector<T> {
        mod_add_assign(&mut other, self, m);
        other
    }
}

impl<T: PrimitiveUnsigned> ModAdd<&UnsignedVector<T>, T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Adds two [`UnsignedVector`]s modulo $m$, taking the first by reference and the second by
    /// reference. The elements of both must already be reduced modulo $m$.
    ///
    /// The sum is taken element by element, each reduced modulo $m$, so the result has the same
    /// dimension as the summands.
    ///
    /// $$
    /// f(v, w, m) = v + w \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 7, 0)").unwrap();
    /// assert_eq!((&v).mod_add(&w, 8).to_string(), "(1, 0, 3)");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_add` from `nmod_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add(self, other: &UnsignedVector<T>, m: T) -> UnsignedVector<T> {
        let mut v = self.clone();
        mod_add_assign(&mut v, other, m);
        v
    }
}

impl<T: PrimitiveUnsigned> ModAddAssign<Self, T> for UnsignedVector<T> {
    /// Adds an [`UnsignedVector`] to an [`UnsignedVector`] modulo $m$, in place, taking the
    /// [`UnsignedVector`] on the right-hand side by value. The elements of both must already be
    /// reduced modulo $m$.
    ///
    /// The sum is taken element by element, each reduced modulo $m$, so the result has the same
    /// dimension as the summands.
    ///
    /// $$
    /// f(v, w, m) = v + w \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 7, 0)").unwrap();
    /// v.mod_add_assign(w, 8);
    /// assert_eq!(v.to_string(), "(1, 0, 3)");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_add` from `nmod_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add_assign(&mut self, other: Self, m: T) {
        mod_add_assign(self, &other, m);
    }
}

impl<T: PrimitiveUnsigned> ModAddAssign<&Self, T> for UnsignedVector<T> {
    /// Adds an [`UnsignedVector`] to an [`UnsignedVector`] modulo $m$, in place, taking the
    /// [`UnsignedVector`] on the right-hand side by reference. The elements of both must already be
    /// reduced modulo $m$.
    ///
    /// The sum is taken element by element, each reduced modulo $m$, so the result has the same
    /// dimension as the summands.
    ///
    /// $$
    /// f(v, w, m) = v + w \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 7, 0)").unwrap();
    /// v.mod_add_assign(&w, 8);
    /// assert_eq!(v.to_string(), "(1, 0, 3)");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_add` from `nmod_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add_assign(&mut self, other: &Self, m: T) {
        mod_add_assign(self, other, m);
    }
}
