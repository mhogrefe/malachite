// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModIsReduced, ModSubMul, ModSubMulAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;

fn assert_reduced<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, b: &UnsignedMatrix<T>, c: T, m: T) {
    assert_eq!(
        (a.height, a.width),
        (b.height, b.width),
        "cannot sub-multiply matrices of different shapes"
    );
    assert!(
        a.mod_is_reduced(&m),
        "self must be reduced mod m, but {a} has an element >= {m}"
    );
    assert!(
        b.mod_is_reduced(&m),
        "b must be reduced mod m, but {b} has an element >= {m}"
    );
    assert!(
        c.mod_is_reduced(&m),
        "c must be reduced mod m, but {c} >= {m}"
    );
}

// Subtracts `ys` times `c` from `xs` modulo `m`, in place, for inputs already known to be reduced.
// Like FLINT's matrix functions, this handles `c` equal to 0, 1, and `m - 1` directly, and
// otherwise computes the data for multiplying modulo `m` once and reuses it for every element.
fn mod_sub_mul_assign_unchecked<T: PrimitiveUnsigned>(xs: &mut [T], ys: &[T], c: T, m: T) {
    if c == T::ZERO {
        return;
    }
    if c == T::ONE {
        for (x, &y) in xs.iter_mut().zip(ys) {
            x.mod_sub_assign(y, m);
        }
    } else if c == m - T::ONE {
        for (x, &y) in xs.iter_mut().zip(ys) {
            x.mod_add_assign(y, m);
        }
    } else {
        let data = T::precompute_mod_mul_data(&m);
        for (x, &y) in xs.iter_mut().zip(ys) {
            x.mod_sub_assign(y.mod_mul_precomputed(c, m, &data), m);
        }
    }
}

impl<T: PrimitiveUnsigned> ModSubMul<Self, T, T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Subtracts a scalar multiple of an [`UnsignedMatrix`] from an [`UnsignedMatrix`] modulo $m$,
    /// taking both matrices by value. The elements and the scalar must already be reduced modulo
    /// $m$.
    ///
    /// The operation is taken element by element, so the result has the same shape as the matrices.
    ///
    /// $$
    /// f(A, B, c, m) = (A - cB) \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes, or if any element of `self` or `b`, or `c`,
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 1, 3]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[2, 6, 4]").unwrap();
    /// assert_eq!(a.mod_sub_mul(b, 3, 7).to_string(), "[6, 4, 5]");
    /// ```
    #[inline]
    fn mod_sub_mul(mut self, b: Self, c: T, m: T) -> Self {
        self.mod_sub_mul_assign(&b, c, m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModSubMul<&Self, T, T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Subtracts a scalar multiple of an [`UnsignedMatrix`] from an [`UnsignedMatrix`] modulo $m$,
    /// taking the first matrix by value and the second by reference. The elements and the scalar
    /// must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMul`] implementation that takes both matrices by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes, or if any element of `self` or `b`, or `c`,
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 1, 3]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[2, 6, 4]").unwrap();
    /// assert_eq!(a.mod_sub_mul(&b, 3, 7).to_string(), "[6, 4, 5]");
    /// ```
    #[inline]
    fn mod_sub_mul(mut self, b: &Self, c: T, m: T) -> Self {
        self.mod_sub_mul_assign(b, c, m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModSubMul<&UnsignedMatrix<T>, T, T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Subtracts a scalar multiple of an [`UnsignedMatrix`] from an [`UnsignedMatrix`] modulo $m$,
    /// taking both matrices by reference. The elements and the scalar must already be reduced
    /// modulo $m$.
    ///
    /// See the documentation for the [`ModSubMul`] implementation that takes both matrices by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes, or if any element of `self` or `b`, or `c`,
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 1, 3]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[2, 6, 4]").unwrap();
    /// assert_eq!((&a).mod_sub_mul(&b, 3, 7).to_string(), "[6, 4, 5]");
    /// ```
    fn mod_sub_mul(self, b: &UnsignedMatrix<T>, c: T, m: T) -> UnsignedMatrix<T> {
        assert_reduced(self, b, c, m);
        let mut elements = self.elements.clone();
        mod_sub_mul_assign_unchecked(&mut elements, &b.elements, c, m);
        UnsignedMatrix {
            elements,
            height: self.height,
            width: self.width,
        }
    }
}

impl<T: PrimitiveUnsigned> ModSubMulAssign<Self, T, T> for UnsignedMatrix<T> {
    /// Subtracts a scalar multiple of an [`UnsignedMatrix`] from an [`UnsignedMatrix`] modulo $m$,
    /// in place, taking the matrix on the right-hand side by value. The elements and the scalar
    /// must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMul`] implementation that takes both matrices by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes, or if any element of `self` or `b`, or `c`,
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMulAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[5, 1, 3]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[2, 6, 4]").unwrap();
    /// a.mod_sub_mul_assign(b, 3, 7);
    /// assert_eq!(a.to_string(), "[6, 4, 5]");
    /// ```
    #[inline]
    fn mod_sub_mul_assign(&mut self, b: Self, c: T, m: T) {
        self.mod_sub_mul_assign(&b, c, m);
    }
}

impl<T: PrimitiveUnsigned> ModSubMulAssign<&Self, T, T> for UnsignedMatrix<T> {
    /// Subtracts a scalar multiple of an [`UnsignedMatrix`] from an [`UnsignedMatrix`] modulo $m$,
    /// in place, taking the matrix on the right-hand side by reference. The elements and the scalar
    /// must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMul`] implementation that takes both matrices by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes, or if any element of `self` or `b`, or `c`,
    /// is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMulAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[5, 1, 3]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[2, 6, 4]").unwrap();
    /// a.mod_sub_mul_assign(&b, 3, 7);
    /// assert_eq!(a.to_string(), "[6, 4, 5]");
    /// ```
    fn mod_sub_mul_assign(&mut self, b: &Self, c: T, m: T) {
        assert_reduced(self, b, c, m);
        mod_sub_mul_assign_unchecked(&mut self.elements, &b.elements, c, m);
    }
}
