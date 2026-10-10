// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    ModPowerOf2AddMul, ModPowerOf2AddMulAssign, ModPowerOf2IsReduced,
};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

fn assert_reduced<T: PrimitiveUnsigned>(
    v: &UnsignedVector<T>,
    w: &UnsignedVector<T>,
    c: T,
    pow: u64,
) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most T::WIDTH, but it is {pow}"
    );
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot add-multiply vectors of different dimensions"
    );
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {v} has an element >= 2^{pow}"
    );
    assert!(
        w.mod_power_of_2_is_reduced(pow),
        "w must be reduced mod 2^pow, but {w} has an element >= 2^{pow}"
    );
    assert!(
        c.mod_power_of_2_is_reduced(pow),
        "c must be reduced mod 2^pow, but {c} >= 2^{pow}"
    );
}

impl<T: PrimitiveUnsigned> ModPowerOf2AddMul<Self, T> for UnsignedVector<T> {
    type Output = Self;

    /// Adds a scalar multiple of an [`UnsignedVector`] to an [`UnsignedVector`] modulo $2^k$,
    /// taking both vectors by value. The elements and the scalar must already be reduced modulo
    /// $2^k$.
    ///
    /// The operation is taken element by element, so the result has the same dimension as the
    /// vectors.
    ///
    /// $$
    /// f(v, w, c, k) = (v + cw) \bmod 2^k.
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
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `w` have different dimensions, or
    /// if any element of `self` or `w`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddMul;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(2, 7, 4)").unwrap();
    /// assert_eq!(v.mod_power_of_2_add_mul(w, 3, 3).to_string(), "(3, 6, 7)");
    /// ```
    #[inline]
    fn mod_power_of_2_add_mul(mut self, w: Self, c: T, pow: u64) -> Self {
        self.mod_power_of_2_add_mul_assign(&w, c, pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2AddMul<&Self, T> for UnsignedVector<T> {
    type Output = Self;

    /// Adds a scalar multiple of an [`UnsignedVector`] to an [`UnsignedVector`] modulo $2^k$,
    /// taking the first vector by value and the second by reference. The elements and the scalar
    /// must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2AddMul`] implementation that takes both vectors
    /// by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `w` have different dimensions, or
    /// if any element of `self` or `w`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddMul;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(2, 7, 4)").unwrap();
    /// assert_eq!(v.mod_power_of_2_add_mul(&w, 3, 3).to_string(), "(3, 6, 7)");
    /// ```
    #[inline]
    fn mod_power_of_2_add_mul(mut self, w: &Self, c: T, pow: u64) -> Self {
        self.mod_power_of_2_add_mul_assign(w, c, pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2AddMul<&UnsignedVector<T>, T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Adds a scalar multiple of an [`UnsignedVector`] to an [`UnsignedVector`] modulo $2^k$,
    /// taking both vectors by reference. The elements and the scalar must already be reduced modulo
    /// $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2AddMul`] implementation that takes both vectors
    /// by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `w` have different dimensions, or
    /// if any element of `self` or `w`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddMul;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(2, 7, 4)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_power_of_2_add_mul(&w, 3, 3).to_string(),
    ///     "(3, 6, 7)"
    /// );
    /// ```
    fn mod_power_of_2_add_mul(self, w: &UnsignedVector<T>, c: T, pow: u64) -> UnsignedVector<T> {
        assert_reduced(self, w, c, pow);
        UnsignedVector {
            elements: self
                .elements
                .iter()
                .zip(&w.elements)
                .map(|(&x, &y)| x.mod_power_of_2_add_mul(y, c, pow))
                .collect(),
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2AddMulAssign<Self, T> for UnsignedVector<T> {
    /// Adds a scalar multiple of an [`UnsignedVector`] to an [`UnsignedVector`] modulo $2^k$, in
    /// place, taking the vector on the right-hand side by value. The elements and the scalar must
    /// already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2AddMul`] implementation that takes both vectors
    /// by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `w` have different dimensions, or
    /// if any element of `self` or `w`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddMulAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(2, 7, 4)").unwrap();
    /// v.mod_power_of_2_add_mul_assign(w, 3, 3);
    /// assert_eq!(v.to_string(), "(3, 6, 7)");
    /// ```
    #[inline]
    fn mod_power_of_2_add_mul_assign(&mut self, w: Self, c: T, pow: u64) {
        self.mod_power_of_2_add_mul_assign(&w, c, pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2AddMulAssign<&Self, T> for UnsignedVector<T> {
    /// Adds a scalar multiple of an [`UnsignedVector`] to an [`UnsignedVector`] modulo $2^k$, in
    /// place, taking the vector on the right-hand side by reference. The elements and the scalar
    /// must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2AddMul`] implementation that takes both vectors
    /// by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `w` have different dimensions, or
    /// if any element of `self` or `w`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddMulAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(2, 7, 4)").unwrap();
    /// v.mod_power_of_2_add_mul_assign(&w, 3, 3);
    /// assert_eq!(v.to_string(), "(3, 6, 7)");
    /// ```
    fn mod_power_of_2_add_mul_assign(&mut self, w: &Self, c: T, pow: u64) {
        assert_reduced(self, w, c, pow);
        if c == T::ZERO {
            return;
        }
        for (x, &y) in self.elements.iter_mut().zip(&w.elements) {
            x.mod_power_of_2_add_mul_assign(y, c, pow);
        }
    }
}
