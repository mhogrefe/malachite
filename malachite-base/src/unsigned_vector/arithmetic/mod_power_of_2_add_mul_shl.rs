// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    ModPowerOf2AddMulAssign, ModPowerOf2AddMulShl, ModPowerOf2AddMulShlAssign,
};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

// Since $cw2^b \equiv w(c2^b \bmod 2^k) \pmod {2^k}$, the shift is applied to the scalar once, and
// the result is `mod_power_of_2_add_mul_assign` by the shifted scalar.
fn shifted_scalar<T: PrimitiveUnsigned>(c: T, bits: u64, pow: u64) -> T {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most T::WIDTH, but it is {pow}"
    );
    assert!(
        c.mod_power_of_2_is_reduced(pow),
        "c must be reduced mod 2^pow, but {c} >= 2^{pow}"
    );
    c.mod_power_of_2_shl(bits, pow)
}

impl<T: PrimitiveUnsigned> ModPowerOf2AddMulShl<Self, T> for UnsignedVector<T> {
    type Output = Self;

    /// Adds a scalar multiple of an [`UnsignedVector`], shifted left by `bits`, to an
    /// [`UnsignedVector`] modulo $2^k$, taking both vectors by value. The elements and the scalar
    /// must already be reduced modulo $2^k$.
    ///
    /// The operation is taken element by element, so the result has the same dimension as the
    /// vectors.
    ///
    /// $$
    /// f(v, w, c, b, k) = (v + cw2^b) \bmod 2^k.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddMulShl;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(2, 7, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_add_mul_shl(w, 3, 1, 4).to_string(),
    ///     "(1, 11, 11)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add_mul_shl(mut self, w: Self, c: T, bits: u64, pow: u64) -> Self {
        self.mod_power_of_2_add_mul_shl_assign(&w, c, bits, pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2AddMulShl<&Self, T> for UnsignedVector<T> {
    type Output = Self;

    /// Adds a scalar multiple of an [`UnsignedVector`], shifted left by `bits`, to an
    /// [`UnsignedVector`] modulo $2^k$, taking the first vector by value and the second by
    /// reference. The elements and the scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2AddMulShl`] implementation that takes both
    /// vectors by value for details.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddMulShl;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(2, 7, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_add_mul_shl(&w, 3, 1, 4).to_string(),
    ///     "(1, 11, 11)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add_mul_shl(mut self, w: &Self, c: T, bits: u64, pow: u64) -> Self {
        self.mod_power_of_2_add_mul_shl_assign(w, c, bits, pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2AddMulShl<&UnsignedVector<T>, T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Adds a scalar multiple of an [`UnsignedVector`], shifted left by `bits`, to an
    /// [`UnsignedVector`] modulo $2^k$, taking both vectors by reference. The elements and the
    /// scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2AddMulShl`] implementation that takes both
    /// vectors by value for details.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddMulShl;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(2, 7, 4)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_power_of_2_add_mul_shl(&w, 3, 1, 4).to_string(),
    ///     "(1, 11, 11)"
    /// );
    /// ```
    fn mod_power_of_2_add_mul_shl(
        self,
        w: &UnsignedVector<T>,
        c: T,
        bits: u64,
        pow: u64,
    ) -> UnsignedVector<T> {
        let mut v = self.clone();
        v.mod_power_of_2_add_mul_shl_assign(w, c, bits, pow);
        v
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2AddMulShlAssign<Self, T> for UnsignedVector<T> {
    /// Adds a scalar multiple of an [`UnsignedVector`], shifted left by `bits`, to an
    /// [`UnsignedVector`] modulo $2^k$, in place, taking the vector on the right-hand side by
    /// value. The elements and the scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2AddMulShl`] implementation that takes both
    /// vectors by value for details.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddMulShlAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(2, 7, 4)").unwrap();
    /// v.mod_power_of_2_add_mul_shl_assign(w, 3, 1, 4);
    /// assert_eq!(v.to_string(), "(1, 11, 11)");
    /// ```
    #[inline]
    fn mod_power_of_2_add_mul_shl_assign(&mut self, w: Self, c: T, bits: u64, pow: u64) {
        self.mod_power_of_2_add_mul_shl_assign(&w, c, bits, pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2AddMulShlAssign<&Self, T> for UnsignedVector<T> {
    /// Adds a scalar multiple of an [`UnsignedVector`], shifted left by `bits`, to an
    /// [`UnsignedVector`] modulo $2^k$, in place, taking the vector on the right-hand side by
    /// reference. The elements and the scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2AddMulShl`] implementation that takes both
    /// vectors by value for details.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddMulShlAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(2, 7, 4)").unwrap();
    /// v.mod_power_of_2_add_mul_shl_assign(&w, 3, 1, 4);
    /// assert_eq!(v.to_string(), "(1, 11, 11)");
    /// ```
    #[inline]
    fn mod_power_of_2_add_mul_shl_assign(&mut self, w: &Self, c: T, bits: u64, pow: u64) {
        self.mod_power_of_2_add_mul_assign(w, shifted_scalar(c, bits, pow), pow);
    }
}
