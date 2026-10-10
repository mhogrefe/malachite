// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModAddMul, ModAddMulAssign, ModIsReduced};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

fn assert_reduced<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, w: &UnsignedVector<T>, c: T, m: T) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot add-multiply vectors of different dimensions"
    );
    assert!(
        v.mod_is_reduced(&m),
        "self must be reduced mod m, but {v} has an element >= {m}"
    );
    assert!(
        w.mod_is_reduced(&m),
        "w must be reduced mod m, but {w} has an element >= {m}"
    );
    assert!(
        c.mod_is_reduced(&m),
        "c must be reduced mod m, but {c} >= {m}"
    );
}

// Adds `ys` times `c` to `xs` modulo `m`, in place, for inputs already known to be reduced. Like
// FLINT, this handles `c` equal to 0, 1, and `m - 1` directly, and otherwise computes the data for
// multiplying modulo `m` once and reuses it for every element.
fn mod_add_mul_assign_unchecked<T: PrimitiveUnsigned>(xs: &mut [T], ys: &[T], c: T, m: T) {
    if c == T::ZERO {
        return;
    }
    if c == T::ONE {
        for (x, &y) in xs.iter_mut().zip(ys) {
            x.mod_add_assign(y, m);
        }
    } else if c == m - T::ONE {
        for (x, &y) in xs.iter_mut().zip(ys) {
            x.mod_sub_assign(y, m);
        }
    } else {
        let data = T::precompute_mod_mul_data(&m);
        for (x, &y) in xs.iter_mut().zip(ys) {
            x.mod_add_assign(y.mod_mul_precomputed(c, m, &data), m);
        }
    }
}

impl<T: PrimitiveUnsigned> ModAddMul<Self, T, T> for UnsignedVector<T> {
    type Output = Self;

    /// Adds a scalar multiple of an [`UnsignedVector`] to an [`UnsignedVector`] modulo $m$, taking
    /// both vectors by value. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// The operation is taken element by element, so the result has the same dimension as the
    /// vectors.
    ///
    /// $$
    /// f(v, w, c, m) = (v + cw) \bmod m.
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
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMul;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(v.mod_add_mul(w, 3, 7).to_string(), "(4, 5, 1)");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_scalar_addmul_nmod` from `nmod_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add_mul(mut self, w: Self, c: T, m: T) -> Self {
        self.mod_add_mul_assign(&w, c, m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModAddMul<&Self, T, T> for UnsignedVector<T> {
    type Output = Self;

    /// Adds a scalar multiple of an [`UnsignedVector`] to an [`UnsignedVector`] modulo $m$, taking
    /// the first vector by value and the second by reference. The elements and the scalar must
    /// already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMul`] implementation that takes both vectors by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMul;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(v.mod_add_mul(&w, 3, 7).to_string(), "(4, 5, 1)");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_scalar_addmul_nmod` from `nmod_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add_mul(mut self, w: &Self, c: T, m: T) -> Self {
        self.mod_add_mul_assign(w, c, m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModAddMul<&UnsignedVector<T>, T, T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Adds a scalar multiple of an [`UnsignedVector`] to an [`UnsignedVector`] modulo $m$, taking
    /// both vectors by reference. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMul`] implementation that takes both vectors by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMul;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!((&v).mod_add_mul(&w, 3, 7).to_string(), "(4, 5, 1)");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_scalar_addmul_nmod` from `nmod_vec/scalar.c`, FLINT 3.6.0.
    fn mod_add_mul(self, w: &UnsignedVector<T>, c: T, m: T) -> UnsignedVector<T> {
        assert_reduced(self, w, c, m);
        let mut elements = self.elements.clone();
        mod_add_mul_assign_unchecked(&mut elements, &w.elements, c, m);
        UnsignedVector { elements }
    }
}

impl<T: PrimitiveUnsigned> ModAddMulAssign<Self, T, T> for UnsignedVector<T> {
    /// Adds a scalar multiple of an [`UnsignedVector`] to an [`UnsignedVector`] modulo $m$, in
    /// place, taking the vector on the right-hand side by value. The elements and the scalar must
    /// already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMul`] implementation that takes both vectors by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMulAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(2, 6, 4)").unwrap();
    /// v.mod_add_mul_assign(w, 3, 7);
    /// assert_eq!(v.to_string(), "(4, 5, 1)");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_scalar_addmul_nmod` from `nmod_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add_mul_assign(&mut self, w: Self, c: T, m: T) {
        self.mod_add_mul_assign(&w, c, m);
    }
}

impl<T: PrimitiveUnsigned> ModAddMulAssign<&Self, T, T> for UnsignedVector<T> {
    /// Adds a scalar multiple of an [`UnsignedVector`] to an [`UnsignedVector`] modulo $m$, in
    /// place, taking the vector on the right-hand side by reference. The elements and the scalar
    /// must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMul`] implementation that takes both vectors by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMulAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(2, 6, 4)").unwrap();
    /// v.mod_add_mul_assign(&w, 3, 7);
    /// assert_eq!(v.to_string(), "(4, 5, 1)");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_scalar_addmul_nmod` from `nmod_vec/scalar.c`, FLINT 3.6.0.
    fn mod_add_mul_assign(&mut self, w: &Self, c: T, m: T) {
        assert_reduced(self, w, c, m);
        mod_add_mul_assign_unchecked(&mut self.elements, &w.elements, c, m);
    }
}
