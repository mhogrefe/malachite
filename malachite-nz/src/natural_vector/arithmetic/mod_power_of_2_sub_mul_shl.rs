// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2Shl, ModPowerOf2SubMulAssign, ModPowerOf2SubMulShl,
    ModPowerOf2SubMulShlAssign,
};

// Since $cw2^b \equiv w(c2^b \bmod 2^k) \pmod {2^k}$, the shift is applied to the scalar once, and
// the result is `mod_power_of_2_sub_mul_assign` by the shifted scalar.
fn shifted_scalar(c: &Natural, bits: u64, pow: u64) -> Natural {
    assert!(
        c.mod_power_of_2_is_reduced(pow),
        "c must be reduced mod 2^pow, but {c} >= 2^{pow}"
    );
    c.mod_power_of_2_shl(bits, pow)
}

impl ModPowerOf2SubMulShl<Self, Natural> for NaturalVector {
    type Output = Self;

    /// Subtracts a scalar multiple of a [`NaturalVector`], shifted left by `bits`, from a
    /// [`NaturalVector`] modulo $2^k$, taking both vectors and the scalar by value. The elements
    /// and the scalar must already be reduced modulo $2^k$.
    ///
    /// The operation is taken element by element, so the result has the same dimension as the
    /// vectors.
    ///
    /// $$
    /// f(v, w, c, b, k) = (v - cw2^b) \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow` times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubMulShl;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 7, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_sub_mul_shl(w, Natural::from(3u32), 1, 4)
    ///         .to_string(),
    ///     "(9, 7, 11)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_sub_mul_shl(
    ///         w,
    ///         Natural::from_str("18446744073709551617").unwrap(),
    ///         3,
    ///         70
    ///     )
    ///     .to_string(),
    ///     "(9, 756316507022091616232)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_sub_mul_shl(mut self, w: Self, c: Natural, bits: u64, pow: u64) -> Self {
        self.mod_power_of_2_sub_mul_shl_assign(w, c, bits, pow);
        self
    }
}

impl ModPowerOf2SubMulShl<Self, &Natural> for NaturalVector {
    type Output = Self;

    /// Subtracts a scalar multiple of a [`NaturalVector`], shifted left by `bits`, from a
    /// [`NaturalVector`] modulo $2^k$, taking the first vector by value, the second by value, and
    /// the scalar by reference. The elements and the scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2SubMulShl`] implementation that takes everything
    /// by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow` times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubMulShl;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 7, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_sub_mul_shl(w, &Natural::from(3u32), 1, 4)
    ///         .to_string(),
    ///     "(9, 7, 11)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_sub_mul_shl(
    ///         w,
    ///         &Natural::from_str("18446744073709551617").unwrap(),
    ///         3,
    ///         70
    ///     )
    ///     .to_string(),
    ///     "(9, 756316507022091616232)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_sub_mul_shl(mut self, w: Self, c: &Natural, bits: u64, pow: u64) -> Self {
        self.mod_power_of_2_sub_mul_shl_assign(w, c, bits, pow);
        self
    }
}

impl ModPowerOf2SubMulShl<&Self, Natural> for NaturalVector {
    type Output = Self;

    /// Subtracts a scalar multiple of a [`NaturalVector`], shifted left by `bits`, from a
    /// [`NaturalVector`] modulo $2^k$, taking the first vector by value, the second by reference,
    /// and the scalar by value. The elements and the scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2SubMulShl`] implementation that takes everything
    /// by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow` times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubMulShl;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 7, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_sub_mul_shl(&w, Natural::from(3u32), 1, 4)
    ///         .to_string(),
    ///     "(9, 7, 11)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_sub_mul_shl(
    ///         &w,
    ///         Natural::from_str("18446744073709551617").unwrap(),
    ///         3,
    ///         70
    ///     )
    ///     .to_string(),
    ///     "(9, 756316507022091616232)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_sub_mul_shl(mut self, w: &Self, c: Natural, bits: u64, pow: u64) -> Self {
        self.mod_power_of_2_sub_mul_shl_assign(w, c, bits, pow);
        self
    }
}

impl ModPowerOf2SubMulShl<&Self, &Natural> for NaturalVector {
    type Output = Self;

    /// Subtracts a scalar multiple of a [`NaturalVector`], shifted left by `bits`, from a
    /// [`NaturalVector`] modulo $2^k$, taking the first vector by value, the second by reference,
    /// and the scalar by reference. The elements and the scalar must already be reduced modulo
    /// $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2SubMulShl`] implementation that takes everything
    /// by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow` times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubMulShl;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 7, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_sub_mul_shl(&w, &Natural::from(3u32), 1, 4)
    ///         .to_string(),
    ///     "(9, 7, 11)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_sub_mul_shl(
    ///         &w,
    ///         &Natural::from_str("18446744073709551617").unwrap(),
    ///         3,
    ///         70
    ///     )
    ///     .to_string(),
    ///     "(9, 756316507022091616232)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_sub_mul_shl(mut self, w: &Self, c: &Natural, bits: u64, pow: u64) -> Self {
        self.mod_power_of_2_sub_mul_shl_assign(w, c, bits, pow);
        self
    }
}

impl ModPowerOf2SubMulShl<&NaturalVector, &Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Subtracts a scalar multiple of a [`NaturalVector`], shifted left by `bits`, from a
    /// [`NaturalVector`] modulo $2^k$, taking both vectors and the scalar by reference. The
    /// elements and the scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2SubMulShl`] implementation that takes everything
    /// by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow` times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubMulShl;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 7, 4)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_power_of_2_sub_mul_shl(&w, &Natural::from(3u32), 1, 4)
    ///         .to_string(),
    ///     "(9, 7, 11)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_power_of_2_sub_mul_shl(
    ///         &w,
    ///         &Natural::from_str("18446744073709551617").unwrap(),
    ///         3,
    ///         70
    ///     )
    ///     .to_string(),
    ///     "(9, 756316507022091616232)"
    /// );
    /// ```
    fn mod_power_of_2_sub_mul_shl(
        self,
        w: &NaturalVector,
        c: &Natural,
        bits: u64,
        pow: u64,
    ) -> NaturalVector {
        let mut v = self.clone();
        v.mod_power_of_2_sub_mul_shl_assign(w, c, bits, pow);
        v
    }
}

impl ModPowerOf2SubMulShlAssign<Self, Natural> for NaturalVector {
    /// Subtracts a scalar multiple of a [`NaturalVector`], shifted left by `bits`, from a
    /// [`NaturalVector`] modulo $2^k$, in place, taking the vector and the scalar on the right-hand
    /// side by value. The elements and the scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2SubMulShl`] implementation that takes everything
    /// by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow` times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubMulShlAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 7, 4)").unwrap();
    /// v.mod_power_of_2_sub_mul_shl_assign(w, Natural::from(3u32), 1, 4);
    /// assert_eq!(v.to_string(), "(9, 7, 11)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_power_of_2_sub_mul_shl_assign(
    ///     w,
    ///     Natural::from_str("18446744073709551617").unwrap(),
    ///     3,
    ///     70,
    /// );
    /// assert_eq!(v.to_string(), "(9, 756316507022091616232)");
    /// ```
    #[inline]
    fn mod_power_of_2_sub_mul_shl_assign(&mut self, w: Self, c: Natural, bits: u64, pow: u64) {
        self.mod_power_of_2_sub_mul_assign(w, shifted_scalar(&c, bits, pow), pow);
    }
}

impl ModPowerOf2SubMulShlAssign<Self, &Natural> for NaturalVector {
    /// Subtracts a scalar multiple of a [`NaturalVector`], shifted left by `bits`, from a
    /// [`NaturalVector`] modulo $2^k$, in place, taking the vector on the right-hand side by value
    /// and the scalar by reference. The elements and the scalar must already be reduced modulo
    /// $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2SubMulShl`] implementation that takes everything
    /// by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow` times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubMulShlAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 7, 4)").unwrap();
    /// v.mod_power_of_2_sub_mul_shl_assign(w, &Natural::from(3u32), 1, 4);
    /// assert_eq!(v.to_string(), "(9, 7, 11)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_power_of_2_sub_mul_shl_assign(
    ///     w,
    ///     &Natural::from_str("18446744073709551617").unwrap(),
    ///     3,
    ///     70,
    /// );
    /// assert_eq!(v.to_string(), "(9, 756316507022091616232)");
    /// ```
    #[inline]
    fn mod_power_of_2_sub_mul_shl_assign(&mut self, w: Self, c: &Natural, bits: u64, pow: u64) {
        self.mod_power_of_2_sub_mul_assign(w, shifted_scalar(c, bits, pow), pow);
    }
}

impl ModPowerOf2SubMulShlAssign<&Self, Natural> for NaturalVector {
    /// Subtracts a scalar multiple of a [`NaturalVector`], shifted left by `bits`, from a
    /// [`NaturalVector`] modulo $2^k$, in place, taking the vector on the right-hand side by
    /// reference and the scalar by value. The elements and the scalar must already be reduced
    /// modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2SubMulShl`] implementation that takes everything
    /// by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow` times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubMulShlAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 7, 4)").unwrap();
    /// v.mod_power_of_2_sub_mul_shl_assign(&w, Natural::from(3u32), 1, 4);
    /// assert_eq!(v.to_string(), "(9, 7, 11)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_power_of_2_sub_mul_shl_assign(
    ///     &w,
    ///     Natural::from_str("18446744073709551617").unwrap(),
    ///     3,
    ///     70,
    /// );
    /// assert_eq!(v.to_string(), "(9, 756316507022091616232)");
    /// ```
    #[inline]
    fn mod_power_of_2_sub_mul_shl_assign(&mut self, w: &Self, c: Natural, bits: u64, pow: u64) {
        self.mod_power_of_2_sub_mul_assign(w, shifted_scalar(&c, bits, pow), pow);
    }
}

impl ModPowerOf2SubMulShlAssign<&Self, &Natural> for NaturalVector {
    /// Subtracts a scalar multiple of a [`NaturalVector`], shifted left by `bits`, from a
    /// [`NaturalVector`] modulo $2^k$, in place, taking the vector and the scalar on the right-hand
    /// side by reference. The elements and the scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2SubMulShl`] implementation that takes everything
    /// by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow` times `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubMulShlAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 7, 4)").unwrap();
    /// v.mod_power_of_2_sub_mul_shl_assign(&w, &Natural::from(3u32), 1, 4);
    /// assert_eq!(v.to_string(), "(9, 7, 11)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_power_of_2_sub_mul_shl_assign(
    ///     &w,
    ///     &Natural::from_str("18446744073709551617").unwrap(),
    ///     3,
    ///     70,
    /// );
    /// assert_eq!(v.to_string(), "(9, 756316507022091616232)");
    /// ```
    #[inline]
    fn mod_power_of_2_sub_mul_shl_assign(&mut self, w: &Self, c: &Natural, bits: u64, pow: u64) {
        self.mod_power_of_2_sub_mul_assign(w, shifted_scalar(c, bits, pow), pow);
    }
}
