// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::arithmetic::traits::{ModIsReduced, ModMul, ModMulAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_vector::arithmetic::mod_mul::{mod_mul_assign_unchecked, mod_mul_unchecked};

fn assert_reduced<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, c: T, m: T) {
    assert!(
        a.mod_is_reduced(&m),
        "self must be reduced mod m, but {a} has an element >= {m}"
    );
    assert!(
        c.mod_is_reduced(&m),
        "c must be reduced mod m, but {c} >= {m}"
    );
}

impl<T: PrimitiveUnsigned> ModMul<T, T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Multiplies every element of an [`UnsignedMatrix`] by a scalar modulo $m$, taking the matrix
    /// by value. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// Each product is reduced modulo $m$, and the result has the same shape as the matrix. The
    /// data for multiplying modulo $m$ is precomputed once and shared by all the elements.
    ///
    /// $$
    /// f(A, c, m) = cA \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// assert_eq!(a.mod_mul(3, 7).to_string(), "[1, 3; 2, 6]");
    /// ```
    #[inline]
    fn mod_mul(mut self, c: T, m: T) -> Self {
        self.mod_mul_assign(c, m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModMul<T, T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Multiplies every element of an [`UnsignedMatrix`] by a scalar modulo $m$, taking the matrix
    /// by reference. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes the matrix by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// assert_eq!((&a).mod_mul(3, 7).to_string(), "[1, 3; 2, 6]");
    /// ```
    fn mod_mul(self, c: T, m: T) -> UnsignedMatrix<T> {
        assert_reduced(self, c, m);
        UnsignedMatrix::from_owned_elements(
            self.height(),
            self.width(),
            mod_mul_unchecked(&self.elements, c, m),
        )
    }
}

impl<T: PrimitiveUnsigned> ModMulAssign<T, T> for UnsignedMatrix<T> {
    /// Multiplies every element of an [`UnsignedMatrix`] by a scalar modulo $m$, in place. The
    /// elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes the matrix by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// a.mod_mul_assign(3, 7);
    /// assert_eq!(a.to_string(), "[1, 3; 2, 6]");
    /// ```
    fn mod_mul_assign(&mut self, c: T, m: T) {
        assert_reduced(self, c, m);
        mod_mul_assign_unchecked(&mut self.elements, c, m);
    }
}
