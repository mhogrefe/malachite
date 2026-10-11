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
    ModAddMulAssign, ModAddMulShl, ModAddMulShlAssign, ModShl,
};

// Since $cw2^b \equiv w(c2^b \bmod m) \pmod m$, the shift is applied to the scalar once, and the
// result is `mod_add_mul_assign` by the shifted scalar.
fn shifted_scalar(c: &Natural, bits: u64, m: &Natural) -> Natural {
    assert!(c < m, "c must be reduced mod m, but {c} >= {m}");
    c.mod_shl(bits, m)
}

impl ModAddMulShl<Self, Natural, Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`] modulo $m$, taking both vectors, the scalar, and the modulus by value. The
    /// elements and the scalar must already be reduced modulo $m$.
    ///
    /// The operation is taken element by element, so the result has the same dimension as the
    /// vectors.
    ///
    /// $$
    /// f(v, w, c, b, m) = (v + cw2^b) \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n + n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()` times
    /// `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMulShl;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul_shl(w, Natural::from(3u32), 2, Natural::from(7u32))
    ///         .to_string(),
    ///     "(1, 3, 2)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul_shl(
    ///         w,
    ///         Natural::from_str("18446744073709551617").unwrap(),
    ///         3,
    ///         Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(367507707706996859298976932346, 461168601842738790424)"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl(mut self, w: Self, c: Natural, bits: u64, m: Natural) -> Self {
        self.mod_add_mul_shl_assign(w, c, bits, m);
        self
    }
}

impl ModAddMulShl<Self, Natural, &Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`] modulo $m$, taking the first vector by value, the second by value, the
    /// scalar by value, and the modulus by reference. The elements and the scalar must already be
    /// reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n + n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()` times
    /// `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMulShl;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul_shl(w, Natural::from(3u32), 2, &Natural::from(7u32))
    ///         .to_string(),
    ///     "(1, 3, 2)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul_shl(
    ///         w,
    ///         Natural::from_str("18446744073709551617").unwrap(),
    ///         3,
    ///         &Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(367507707706996859298976932346, 461168601842738790424)"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl(mut self, w: Self, c: Natural, bits: u64, m: &Natural) -> Self {
        self.mod_add_mul_shl_assign(w, c, bits, m);
        self
    }
}

impl ModAddMulShl<Self, &Natural, Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`] modulo $m$, taking the first vector by value, the second by value, the
    /// scalar by reference, and the modulus by value. The elements and the scalar must already be
    /// reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n + n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()` times
    /// `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMulShl;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul_shl(w, &Natural::from(3u32), 2, Natural::from(7u32))
    ///         .to_string(),
    ///     "(1, 3, 2)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul_shl(
    ///         w,
    ///         &Natural::from_str("18446744073709551617").unwrap(),
    ///         3,
    ///         Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(367507707706996859298976932346, 461168601842738790424)"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl(mut self, w: Self, c: &Natural, bits: u64, m: Natural) -> Self {
        self.mod_add_mul_shl_assign(w, c, bits, m);
        self
    }
}

impl ModAddMulShl<Self, &Natural, &Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`] modulo $m$, taking the first vector by value, the second by value, the
    /// scalar by reference, and the modulus by reference. The elements and the scalar must already
    /// be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n + n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()` times
    /// `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMulShl;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul_shl(w, &Natural::from(3u32), 2, &Natural::from(7u32))
    ///         .to_string(),
    ///     "(1, 3, 2)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul_shl(
    ///         w,
    ///         &Natural::from_str("18446744073709551617").unwrap(),
    ///         3,
    ///         &Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(367507707706996859298976932346, 461168601842738790424)"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl(mut self, w: Self, c: &Natural, bits: u64, m: &Natural) -> Self {
        self.mod_add_mul_shl_assign(w, c, bits, m);
        self
    }
}

impl ModAddMulShl<&Self, Natural, Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`] modulo $m$, taking the first vector by value, the second by reference, the
    /// scalar by value, and the modulus by value. The elements and the scalar must already be
    /// reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n + n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()` times
    /// `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMulShl;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul_shl(&w, Natural::from(3u32), 2, Natural::from(7u32))
    ///         .to_string(),
    ///     "(1, 3, 2)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul_shl(
    ///         &w,
    ///         Natural::from_str("18446744073709551617").unwrap(),
    ///         3,
    ///         Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(367507707706996859298976932346, 461168601842738790424)"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl(mut self, w: &Self, c: Natural, bits: u64, m: Natural) -> Self {
        self.mod_add_mul_shl_assign(w, c, bits, m);
        self
    }
}

impl ModAddMulShl<&Self, Natural, &Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`] modulo $m$, taking the first vector by value, the second by reference, the
    /// scalar by value, and the modulus by reference. The elements and the scalar must already be
    /// reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n + n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()` times
    /// `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMulShl;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul_shl(&w, Natural::from(3u32), 2, &Natural::from(7u32))
    ///         .to_string(),
    ///     "(1, 3, 2)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul_shl(
    ///         &w,
    ///         Natural::from_str("18446744073709551617").unwrap(),
    ///         3,
    ///         &Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(367507707706996859298976932346, 461168601842738790424)"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl(mut self, w: &Self, c: Natural, bits: u64, m: &Natural) -> Self {
        self.mod_add_mul_shl_assign(w, c, bits, m);
        self
    }
}

impl ModAddMulShl<&Self, &Natural, Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`] modulo $m$, taking the first vector by value, the second by reference, the
    /// scalar by reference, and the modulus by value. The elements and the scalar must already be
    /// reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n + n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()` times
    /// `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMulShl;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul_shl(&w, &Natural::from(3u32), 2, Natural::from(7u32))
    ///         .to_string(),
    ///     "(1, 3, 2)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul_shl(
    ///         &w,
    ///         &Natural::from_str("18446744073709551617").unwrap(),
    ///         3,
    ///         Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(367507707706996859298976932346, 461168601842738790424)"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl(mut self, w: &Self, c: &Natural, bits: u64, m: Natural) -> Self {
        self.mod_add_mul_shl_assign(w, c, bits, m);
        self
    }
}

impl ModAddMulShl<&Self, &Natural, &Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`] modulo $m$, taking the first vector by value, the second by reference, the
    /// scalar by reference, and the modulus by reference. The elements and the scalar must already
    /// be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n + n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()` times
    /// `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMulShl;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul_shl(&w, &Natural::from(3u32), 2, &Natural::from(7u32))
    ///         .to_string(),
    ///     "(1, 3, 2)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul_shl(
    ///         &w,
    ///         &Natural::from_str("18446744073709551617").unwrap(),
    ///         3,
    ///         &Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(367507707706996859298976932346, 461168601842738790424)"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl(mut self, w: &Self, c: &Natural, bits: u64, m: &Natural) -> Self {
        self.mod_add_mul_shl_assign(w, c, bits, m);
        self
    }
}

impl ModAddMulShl<&NaturalVector, &Natural, &Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`] modulo $m$, taking both vectors, the scalar, and the modulus by reference.
    /// The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n + n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()` times
    /// `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMulShl;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_add_mul_shl(&w, &Natural::from(3u32), 2, &Natural::from(7u32))
    ///         .to_string(),
    ///     "(1, 3, 2)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_add_mul_shl(
    ///         &w,
    ///         &Natural::from_str("18446744073709551617").unwrap(),
    ///         3,
    ///         &Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(367507707706996859298976932346, 461168601842738790424)"
    /// );
    /// ```
    fn mod_add_mul_shl(
        self,
        w: &NaturalVector,
        c: &Natural,
        bits: u64,
        m: &Natural,
    ) -> NaturalVector {
        let mut v = self.clone();
        v.mod_add_mul_shl_assign(w, c, bits, m);
        v
    }
}

impl ModAddMulShlAssign<Self, Natural, Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`] modulo $m$, in place, taking the vector on the right-hand side by value,
    /// the scalar by value, and the modulus by value. The elements and the scalar must already be
    /// reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n + n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()` times
    /// `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMulShlAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_add_mul_shl_assign(w, Natural::from(3u32), 2, Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(1, 3, 2)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_add_mul_shl_assign(
    ///     w,
    ///     Natural::from_str("18446744073709551617").unwrap(),
    ///     3,
    ///     Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(367507707706996859298976932346, 461168601842738790424)"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl_assign(&mut self, w: Self, c: Natural, bits: u64, m: Natural) {
        let c = shifted_scalar(&c, bits, &m);
        self.mod_add_mul_assign(w, c, m);
    }
}

impl ModAddMulShlAssign<Self, Natural, &Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`] modulo $m$, in place, taking the vector on the right-hand side by value,
    /// the scalar by value, and the modulus by reference. The elements and the scalar must already
    /// be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n + n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()` times
    /// `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMulShlAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_add_mul_shl_assign(w, Natural::from(3u32), 2, &Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(1, 3, 2)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_add_mul_shl_assign(
    ///     w,
    ///     Natural::from_str("18446744073709551617").unwrap(),
    ///     3,
    ///     &Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(367507707706996859298976932346, 461168601842738790424)"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl_assign(&mut self, w: Self, c: Natural, bits: u64, m: &Natural) {
        let c = shifted_scalar(&c, bits, m);
        self.mod_add_mul_assign(w, c, m);
    }
}

impl ModAddMulShlAssign<Self, &Natural, Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`] modulo $m$, in place, taking the vector on the right-hand side by value,
    /// the scalar by reference, and the modulus by value. The elements and the scalar must already
    /// be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n + n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()` times
    /// `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMulShlAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_add_mul_shl_assign(w, &Natural::from(3u32), 2, Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(1, 3, 2)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_add_mul_shl_assign(
    ///     w,
    ///     &Natural::from_str("18446744073709551617").unwrap(),
    ///     3,
    ///     Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(367507707706996859298976932346, 461168601842738790424)"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl_assign(&mut self, w: Self, c: &Natural, bits: u64, m: Natural) {
        let c = shifted_scalar(c, bits, &m);
        self.mod_add_mul_assign(w, c, m);
    }
}

impl ModAddMulShlAssign<Self, &Natural, &Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`] modulo $m$, in place, taking the vector on the right-hand side by value,
    /// the scalar by reference, and the modulus by reference. The elements and the scalar must
    /// already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n + n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()` times
    /// `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMulShlAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_add_mul_shl_assign(w, &Natural::from(3u32), 2, &Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(1, 3, 2)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_add_mul_shl_assign(
    ///     w,
    ///     &Natural::from_str("18446744073709551617").unwrap(),
    ///     3,
    ///     &Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(367507707706996859298976932346, 461168601842738790424)"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl_assign(&mut self, w: Self, c: &Natural, bits: u64, m: &Natural) {
        let c = shifted_scalar(c, bits, m);
        self.mod_add_mul_assign(w, c, m);
    }
}

impl ModAddMulShlAssign<&Self, Natural, Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`] modulo $m$, in place, taking the vector on the right-hand side by
    /// reference, the scalar by value, and the modulus by value. The elements and the scalar must
    /// already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n + n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()` times
    /// `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMulShlAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_add_mul_shl_assign(&w, Natural::from(3u32), 2, Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(1, 3, 2)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_add_mul_shl_assign(
    ///     &w,
    ///     Natural::from_str("18446744073709551617").unwrap(),
    ///     3,
    ///     Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(367507707706996859298976932346, 461168601842738790424)"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl_assign(&mut self, w: &Self, c: Natural, bits: u64, m: Natural) {
        let c = shifted_scalar(&c, bits, &m);
        self.mod_add_mul_assign(w, c, m);
    }
}

impl ModAddMulShlAssign<&Self, Natural, &Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`] modulo $m$, in place, taking the vector on the right-hand side by
    /// reference, the scalar by value, and the modulus by reference. The elements and the scalar
    /// must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n + n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()` times
    /// `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMulShlAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_add_mul_shl_assign(&w, Natural::from(3u32), 2, &Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(1, 3, 2)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_add_mul_shl_assign(
    ///     &w,
    ///     Natural::from_str("18446744073709551617").unwrap(),
    ///     3,
    ///     &Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(367507707706996859298976932346, 461168601842738790424)"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl_assign(&mut self, w: &Self, c: Natural, bits: u64, m: &Natural) {
        let c = shifted_scalar(&c, bits, m);
        self.mod_add_mul_assign(w, c, m);
    }
}

impl ModAddMulShlAssign<&Self, &Natural, Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`] modulo $m$, in place, taking the vector on the right-hand side by
    /// reference, the scalar by reference, and the modulus by value. The elements and the scalar
    /// must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n + n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()` times
    /// `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMulShlAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_add_mul_shl_assign(&w, &Natural::from(3u32), 2, Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(1, 3, 2)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_add_mul_shl_assign(
    ///     &w,
    ///     &Natural::from_str("18446744073709551617").unwrap(),
    ///     3,
    ///     Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(367507707706996859298976932346, 461168601842738790424)"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl_assign(&mut self, w: &Self, c: &Natural, bits: u64, m: Natural) {
        let c = shifted_scalar(c, bits, &m);
        self.mod_add_mul_assign(w, c, m);
    }
}

impl ModAddMulShlAssign<&Self, &Natural, &Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`], shifted left by `bits`, to a
    /// [`NaturalVector`] modulo $m$, in place, taking the vector on the right-hand side by
    /// reference, the scalar by reference, and the modulus by reference. The elements and the
    /// scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMulShl`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n + n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()` times
    /// `self.dimension()`, and $b$ is `bits`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddMulShlAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_add_mul_shl_assign(&w, &Natural::from(3u32), 2, &Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(1, 3, 2)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_add_mul_shl_assign(
    ///     &w,
    ///     &Natural::from_str("18446744073709551617").unwrap(),
    ///     3,
    ///     &Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(367507707706996859298976932346, 461168601842738790424)"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl_assign(&mut self, w: &Self, c: &Natural, bits: u64, m: &Natural) {
        let c = shifted_scalar(c, bits, m);
        self.mod_add_mul_assign(w, c, m);
    }
}
