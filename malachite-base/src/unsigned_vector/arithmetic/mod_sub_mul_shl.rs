// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModShl, ModSubMulAssign, ModSubMulShl, ModSubMulShlAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

// Since $cw2^b \equiv w(c2^b \bmod m) \pmod m$, the shift is applied to the scalar once, and the
// result is `mod_sub_mul_assign` by the shifted scalar.
fn shifted_scalar<T: PrimitiveUnsigned + ModShl<u64, T, Output = T>>(c: T, bits: u64, m: T) -> T {
    assert!(c < m, "c must be reduced mod m, but {c} >= {m}");
    c.mod_shl(bits, m)
}

impl<T: PrimitiveUnsigned + ModShl<u64, T, Output = T>> ModSubMulShl<Self, T, T>
    for UnsignedVector<T>
{
    type Output = Self;

    /// Subtracts a scalar multiple of an [`UnsignedVector`], shifted left by `bits`, from an
    /// [`UnsignedVector`] modulo $m$, taking both vectors by value. The elements and the scalar
    /// must already be reduced modulo $m$.
    ///
    /// The operation is taken element by element, so the result has the same dimension as the
    /// vectors.
    ///
    /// $$
    /// f(v, w, c, b, m) = (v - cw2^b) \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n + \log b)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMulShl;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(v.mod_sub_mul_shl(w, 3, 2, 7).to_string(), "(2, 6, 4)");
    /// ```
    #[inline]
    fn mod_sub_mul_shl(mut self, w: Self, c: T, bits: u64, m: T) -> Self {
        self.mod_sub_mul_shl_assign(&w, c, bits, m);
        self
    }
}

impl<T: PrimitiveUnsigned + ModShl<u64, T, Output = T>> ModSubMulShl<&Self, T, T>
    for UnsignedVector<T>
{
    type Output = Self;

    /// Subtracts a scalar multiple of an [`UnsignedVector`], shifted left by `bits`, from an
    /// [`UnsignedVector`] modulo $m$, taking the first vector by value and the second by reference.
    /// The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMulShl`] implementation that takes both vectors by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n + \log b)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMulShl;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(v.mod_sub_mul_shl(&w, 3, 2, 7).to_string(), "(2, 6, 4)");
    /// ```
    #[inline]
    fn mod_sub_mul_shl(mut self, w: &Self, c: T, bits: u64, m: T) -> Self {
        self.mod_sub_mul_shl_assign(w, c, bits, m);
        self
    }
}

impl<T: PrimitiveUnsigned + ModShl<u64, T, Output = T>> ModSubMulShl<&UnsignedVector<T>, T, T>
    for &UnsignedVector<T>
{
    type Output = UnsignedVector<T>;

    /// Subtracts a scalar multiple of an [`UnsignedVector`], shifted left by `bits`, from an
    /// [`UnsignedVector`] modulo $m$, taking both vectors by reference. The elements and the scalar
    /// must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMulShl`] implementation that takes both vectors by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n + \log b)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMulShl;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!((&v).mod_sub_mul_shl(&w, 3, 2, 7).to_string(), "(2, 6, 4)");
    /// ```
    fn mod_sub_mul_shl(self, w: &UnsignedVector<T>, c: T, bits: u64, m: T) -> UnsignedVector<T> {
        let mut v = self.clone();
        v.mod_sub_mul_shl_assign(w, c, bits, m);
        v
    }
}

impl<T: PrimitiveUnsigned + ModShl<u64, T, Output = T>> ModSubMulShlAssign<Self, T, T>
    for UnsignedVector<T>
{
    /// Subtracts a scalar multiple of an [`UnsignedVector`], shifted left by `bits`, from an
    /// [`UnsignedVector`] modulo $m$, in place, taking the vector on the right-hand side by value.
    /// The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMulShl`] implementation that takes both vectors by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n + \log b)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMulShlAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(2, 6, 4)").unwrap();
    /// v.mod_sub_mul_shl_assign(w, 3, 2, 7);
    /// assert_eq!(v.to_string(), "(2, 6, 4)");
    /// ```
    #[inline]
    fn mod_sub_mul_shl_assign(&mut self, w: Self, c: T, bits: u64, m: T) {
        self.mod_sub_mul_shl_assign(&w, c, bits, m);
    }
}

impl<T: PrimitiveUnsigned + ModShl<u64, T, Output = T>> ModSubMulShlAssign<&Self, T, T>
    for UnsignedVector<T>
{
    /// Subtracts a scalar multiple of an [`UnsignedVector`], shifted left by `bits`, from an
    /// [`UnsignedVector`] modulo $m$, in place, taking the vector on the right-hand side by
    /// reference. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMulShl`] implementation that takes both vectors by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n + \log b)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMulShlAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(2, 6, 4)").unwrap();
    /// v.mod_sub_mul_shl_assign(&w, 3, 2, 7);
    /// assert_eq!(v.to_string(), "(2, 6, 4)");
    /// ```
    #[inline]
    fn mod_sub_mul_shl_assign(&mut self, w: &Self, c: T, bits: u64, m: T) {
        self.mod_sub_mul_assign(w, shifted_scalar(c, bits, m), m);
    }
}
