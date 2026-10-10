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
    ModPowerOf2AddMul, ModPowerOf2AddMulAssign, ModPowerOf2IsReduced,
};

fn assert_reduced(v: &NaturalVector, w: &NaturalVector, c: &Natural, pow: u64) {
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

impl ModPowerOf2AddMul<Self, Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $2^k$, taking
    /// both vectors and the scalar by value. The elements and the scalar must already be reduced
    /// modulo $2^k$.
    ///
    /// The operation is taken element by element, so the result has the same dimension as the
    /// vectors.
    ///
    /// $$
    /// f(v, w, c, k) = (v + cw) \bmod 2^k.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 7, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_add_mul(w, Natural::from(3u32), 3)
    ///         .to_string(),
    ///     "(3, 6, 7)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_add_mul(w, Natural::from_str("18446744073709551617").unwrap(), 70)
    ///         .to_string(),
    ///     "(0, 73786976294838206467)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add_mul(mut self, w: Self, c: Natural, pow: u64) -> Self {
        self.mod_power_of_2_add_mul_assign(w, c, pow);
        self
    }
}

impl ModPowerOf2AddMul<Self, &Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $2^k$, taking
    /// the first vector by value, the second by value, and the scalar by reference. The elements
    /// and the scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2AddMul`] implementation that takes everything by
    /// value for details.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 7, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_add_mul(w, &Natural::from(3u32), 3)
    ///         .to_string(),
    ///     "(3, 6, 7)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_add_mul(w, &Natural::from_str("18446744073709551617").unwrap(), 70)
    ///         .to_string(),
    ///     "(0, 73786976294838206467)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add_mul(mut self, w: Self, c: &Natural, pow: u64) -> Self {
        self.mod_power_of_2_add_mul_assign(w, c, pow);
        self
    }
}

impl ModPowerOf2AddMul<&Self, Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $2^k$, taking
    /// the first vector by value, the second by reference, and the scalar by value. The elements
    /// and the scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2AddMul`] implementation that takes everything by
    /// value for details.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 7, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_add_mul(&w, Natural::from(3u32), 3)
    ///         .to_string(),
    ///     "(3, 6, 7)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_add_mul(&w, Natural::from_str("18446744073709551617").unwrap(), 70)
    ///         .to_string(),
    ///     "(0, 73786976294838206467)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add_mul(mut self, w: &Self, c: Natural, pow: u64) -> Self {
        self.mod_power_of_2_add_mul_assign(w, c, pow);
        self
    }
}

impl ModPowerOf2AddMul<&Self, &Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $2^k$, taking
    /// the first vector by value, the second by reference, and the scalar by reference. The
    /// elements and the scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2AddMul`] implementation that takes everything by
    /// value for details.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 7, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_add_mul(&w, &Natural::from(3u32), 3)
    ///         .to_string(),
    ///     "(3, 6, 7)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_add_mul(&w, &Natural::from_str("18446744073709551617").unwrap(), 70)
    ///         .to_string(),
    ///     "(0, 73786976294838206467)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add_mul(mut self, w: &Self, c: &Natural, pow: u64) -> Self {
        self.mod_power_of_2_add_mul_assign(w, c, pow);
        self
    }
}

impl ModPowerOf2AddMul<&NaturalVector, &Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $2^k$, taking
    /// both vectors and the scalar by reference. The elements and the scalar must already be
    /// reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2AddMul`] implementation that takes everything by
    /// value for details.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 7, 4)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_power_of_2_add_mul(&w, &Natural::from(3u32), 3)
    ///         .to_string(),
    ///     "(3, 6, 7)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_power_of_2_add_mul(
    ///         &w,
    ///         &Natural::from_str("18446744073709551617").unwrap(),
    ///         70
    ///     )
    ///     .to_string(),
    ///     "(0, 73786976294838206467)"
    /// );
    /// ```
    fn mod_power_of_2_add_mul(self, w: &NaturalVector, c: &Natural, pow: u64) -> NaturalVector {
        assert_reduced(self, w, c, pow);
        NaturalVector {
            elements: self
                .elements
                .iter()
                .zip(&w.elements)
                .map(|(x, y)| x.mod_power_of_2_add_mul(y, c, pow))
                .collect(),
        }
    }
}

impl ModPowerOf2AddMulAssign<Self, Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $2^k$, in place,
    /// taking the vector and the scalar on the right-hand side by value. The elements and the
    /// scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2AddMul`] implementation that takes everything by
    /// value for details.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 7, 4)").unwrap();
    /// v.mod_power_of_2_add_mul_assign(w, Natural::from(3u32), 3);
    /// assert_eq!(v.to_string(), "(3, 6, 7)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_power_of_2_add_mul_assign(w, Natural::from_str("18446744073709551617").unwrap(), 70);
    /// assert_eq!(v.to_string(), "(0, 73786976294838206467)");
    /// ```
    #[inline]
    fn mod_power_of_2_add_mul_assign(&mut self, w: Self, c: Natural, pow: u64) {
        self.mod_power_of_2_add_mul_assign(w, &c, pow);
    }
}

impl ModPowerOf2AddMulAssign<Self, &Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $2^k$, in place,
    /// taking the vector on the right-hand side by value and the scalar by reference. The elements
    /// and the scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2AddMul`] implementation that takes everything by
    /// value for details.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 7, 4)").unwrap();
    /// v.mod_power_of_2_add_mul_assign(w, &Natural::from(3u32), 3);
    /// assert_eq!(v.to_string(), "(3, 6, 7)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_power_of_2_add_mul_assign(w, &Natural::from_str("18446744073709551617").unwrap(), 70);
    /// assert_eq!(v.to_string(), "(0, 73786976294838206467)");
    /// ```
    fn mod_power_of_2_add_mul_assign(&mut self, w: Self, c: &Natural, pow: u64) {
        assert_reduced(self, &w, c, pow);
        if *c == 0u32 {
            return;
        }
        for (x, y) in self.elements.iter_mut().zip(w.elements) {
            x.mod_power_of_2_add_mul_assign(y, c, pow);
        }
    }
}

impl ModPowerOf2AddMulAssign<&Self, Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $2^k$, in place,
    /// taking the vector on the right-hand side by reference and the scalar by value. The elements
    /// and the scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2AddMul`] implementation that takes everything by
    /// value for details.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 7, 4)").unwrap();
    /// v.mod_power_of_2_add_mul_assign(&w, Natural::from(3u32), 3);
    /// assert_eq!(v.to_string(), "(3, 6, 7)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_power_of_2_add_mul_assign(&w, Natural::from_str("18446744073709551617").unwrap(), 70);
    /// assert_eq!(v.to_string(), "(0, 73786976294838206467)");
    /// ```
    #[inline]
    fn mod_power_of_2_add_mul_assign(&mut self, w: &Self, c: Natural, pow: u64) {
        self.mod_power_of_2_add_mul_assign(w, &c, pow);
    }
}

impl ModPowerOf2AddMulAssign<&Self, &Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $2^k$, in place,
    /// taking the vector and the scalar on the right-hand side by reference. The elements and the
    /// scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2AddMul`] implementation that takes everything by
    /// value for details.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 7, 4)").unwrap();
    /// v.mod_power_of_2_add_mul_assign(&w, &Natural::from(3u32), 3);
    /// assert_eq!(v.to_string(), "(3, 6, 7)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_power_of_2_add_mul_assign(
    ///     &w,
    ///     &Natural::from_str("18446744073709551617").unwrap(),
    ///     70,
    /// );
    /// assert_eq!(v.to_string(), "(0, 73786976294838206467)");
    /// ```
    fn mod_power_of_2_add_mul_assign(&mut self, w: &Self, c: &Natural, pow: u64) {
        assert_reduced(self, w, c, pow);
        if *c == 0u32 {
            return;
        }
        for (x, y) in self.elements.iter_mut().zip(&w.elements) {
            x.mod_power_of_2_add_mul_assign(y, c, pow);
        }
    }
}
