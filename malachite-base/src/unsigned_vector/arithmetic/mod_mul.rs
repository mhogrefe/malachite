// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModIsReduced, ModMul, ModMulAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use alloc::vec::Vec;

fn assert_reduced<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, c: T, m: T) {
    assert!(
        v.mod_is_reduced(&m),
        "self must be reduced mod m, but {v} has an element >= {m}"
    );
    assert!(
        c.mod_is_reduced(&m),
        "c must be reduced mod m, but {c} >= {m}"
    );
}

// Multiplies every element by `c` modulo `m`, which the elements and `c` are already reduced
// modulo. The data for multiplying modulo `m` is computed once and shared by all the elements.
pub(crate) fn mod_mul_assign_unchecked<T: PrimitiveUnsigned>(xs: &mut [T], c: T, m: T) {
    let data = T::precompute_mod_mul_data(&m);
    for x in xs {
        x.mod_mul_precomputed_assign(c, m, &data);
    }
}

// The elements multiplied by `c` modulo `m`, as `mod_mul_assign_unchecked` computes them.
pub(crate) fn mod_mul_unchecked<T: PrimitiveUnsigned>(xs: &[T], c: T, m: T) -> Vec<T> {
    let data = T::precompute_mod_mul_data(&m);
    xs.iter()
        .map(|&x| x.mod_mul_precomputed(c, m, &data))
        .collect()
}

impl<T: PrimitiveUnsigned> ModMul<T, T> for UnsignedVector<T> {
    type Output = Self;

    /// Multiplies every element of an [`UnsignedVector`] by a scalar modulo $m$, taking the vector
    /// by value. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// Each product is reduced modulo $m$, and the result has the same dimension as the vector. The
    /// data for multiplying modulo $m$ is precomputed once and shared by all the elements.
    ///
    /// $$
    /// f(v, c, m) = cv \bmod m.
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
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// assert_eq!(v.mod_mul(3, 7).to_string(), "(1, 3, 2)");
    /// ```
    #[inline]
    fn mod_mul(mut self, c: T, m: T) -> Self {
        self.mod_mul_assign(c, m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModMul<T, T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Multiplies every element of an [`UnsignedVector`] by a scalar modulo $m$, taking the vector
    /// by reference. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes the vector by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// assert_eq!((&v).mod_mul(3, 7).to_string(), "(1, 3, 2)");
    /// ```
    fn mod_mul(self, c: T, m: T) -> UnsignedVector<T> {
        assert_reduced(self, c, m);
        UnsignedVector {
            elements: mod_mul_unchecked(&self.elements, c, m),
        }
    }
}

impl<T: PrimitiveUnsigned> ModMulAssign<T, T> for UnsignedVector<T> {
    /// Multiplies every element of an [`UnsignedVector`] by a scalar modulo $m$, in place. The
    /// elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes the vector by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// v.mod_mul_assign(3, 7);
    /// assert_eq!(v.to_string(), "(1, 3, 2)");
    /// ```
    fn mod_mul_assign(&mut self, c: T, m: T) {
        assert_reduced(self, c, m);
        mod_mul_assign_unchecked(&mut self.elements, c, m);
    }
}
