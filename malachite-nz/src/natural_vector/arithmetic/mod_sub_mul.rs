// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    ModAddAssign, ModIsReduced, ModMulPrecomputed, ModMulPrecomputedAssign, ModSubAssign,
    ModSubMul, ModSubMulAssign,
};
use malachite_base::num::basic::traits::{One, Zero};

fn assert_reduced(v: &NaturalVector, w: &NaturalVector, c: &Natural, m: &Natural) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot sub-multiply vectors of different dimensions"
    );
    assert!(
        v.mod_is_reduced(m),
        "self must be reduced mod m, but {v} has an element >= {m}"
    );
    assert!(
        w.mod_is_reduced(m),
        "w must be reduced mod m, but {w} has an element >= {m}"
    );
    assert!(
        c.mod_is_reduced(m),
        "c must be reduced mod m, but {c} >= {m}"
    );
}

// Subtracts `ys` times `c` from `xs` modulo `m`, in place, for inputs already known to be reduced.
// Like FLINT, this handles `c` equal to 0, 1, and `m - 1` directly, and otherwise computes the data
// for multiplying modulo `m` once and reuses it for every element.
fn mod_sub_mul_assign_ref(xs: &mut [Natural], ys: &[Natural], c: &Natural, m: &Natural) {
    match *c {
        Natural::ZERO => {}
        Natural::ONE => {
            for (x, y) in xs.iter_mut().zip(ys) {
                x.mod_sub_assign(y, m);
            }
        }
        _ if *c == m - Natural::ONE => {
            for (x, y) in xs.iter_mut().zip(ys) {
                x.mod_add_assign(y, m);
            }
        }
        _ => {
            let data =
                <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
            for (x, y) in xs.iter_mut().zip(ys) {
                x.mod_sub_assign(y.mod_mul_precomputed(c, m, &data), m);
            }
        }
    }
}

// As `mod_sub_mul_assign_ref`, consuming `ys`, whose elements hold the products.
fn mod_sub_mul_assign_val(xs: &mut [Natural], ys: Vec<Natural>, c: &Natural, m: &Natural) {
    match *c {
        Natural::ZERO => {}
        Natural::ONE => {
            for (x, y) in xs.iter_mut().zip(ys) {
                x.mod_sub_assign(y, m);
            }
        }
        _ if *c == m - Natural::ONE => {
            for (x, y) in xs.iter_mut().zip(ys) {
                x.mod_add_assign(y, m);
            }
        }
        _ => {
            let data =
                <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
            for (x, mut y) in xs.iter_mut().zip(ys) {
                y.mod_mul_precomputed_assign(c, m, &data);
                x.mod_sub_assign(y, m);
            }
        }
    }
}

impl ModSubMul<Self, Natural, Natural> for NaturalVector {
    type Output = Self;

    /// Subtracts a scalar multiple of a [`NaturalVector`] from a [`NaturalVector`] modulo $m$,
    /// taking both vectors, the scalar, and the modulus by value. The elements and the scalar must
    /// already be reduced modulo $m$.
    ///
    /// The operation is taken element by element, so the result has the same dimension as the
    /// vectors.
    ///
    /// $$
    /// f(v, w, c, m) = (v - cw) \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()` times
    /// `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_sub_mul(w, Natural::from(3u32), Natural::from(7u32))
    ///         .to_string(),
    ///     "(6, 4, 5)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_sub_mul(
    ///         w,
    ///         Natural::from_str("18446744073709551617").unwrap(),
    ///         Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(79061536536625392587627883465, 999999999963106511852580896822)"
    /// );
    /// ```
    #[inline]
    fn mod_sub_mul(mut self, w: Self, c: Natural, m: Natural) -> Self {
        self.mod_sub_mul_assign(w, c, m);
        self
    }
}

impl ModSubMul<Self, Natural, &Natural> for NaturalVector {
    type Output = Self;

    /// Subtracts a scalar multiple of a [`NaturalVector`] from a [`NaturalVector`] modulo $m$,
    /// taking the first vector by value, the second by value, the scalar by value, and the modulus
    /// by reference. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMul`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()` times
    /// `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_sub_mul(w, Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "(6, 4, 5)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_sub_mul(
    ///         w,
    ///         Natural::from_str("18446744073709551617").unwrap(),
    ///         &Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(79061536536625392587627883465, 999999999963106511852580896822)"
    /// );
    /// ```
    #[inline]
    fn mod_sub_mul(mut self, w: Self, c: Natural, m: &Natural) -> Self {
        self.mod_sub_mul_assign(w, c, m);
        self
    }
}

impl ModSubMul<Self, &Natural, Natural> for NaturalVector {
    type Output = Self;

    /// Subtracts a scalar multiple of a [`NaturalVector`] from a [`NaturalVector`] modulo $m$,
    /// taking the first vector by value, the second by value, the scalar by reference, and the
    /// modulus by value. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMul`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()` times
    /// `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_sub_mul(w, &Natural::from(3u32), Natural::from(7u32))
    ///         .to_string(),
    ///     "(6, 4, 5)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_sub_mul(
    ///         w,
    ///         &Natural::from_str("18446744073709551617").unwrap(),
    ///         Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(79061536536625392587627883465, 999999999963106511852580896822)"
    /// );
    /// ```
    #[inline]
    fn mod_sub_mul(mut self, w: Self, c: &Natural, m: Natural) -> Self {
        self.mod_sub_mul_assign(w, c, m);
        self
    }
}

impl ModSubMul<Self, &Natural, &Natural> for NaturalVector {
    type Output = Self;

    /// Subtracts a scalar multiple of a [`NaturalVector`] from a [`NaturalVector`] modulo $m$,
    /// taking the first vector by value, the second by value, the scalar by reference, and the
    /// modulus by reference. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMul`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()` times
    /// `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_sub_mul(w, &Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "(6, 4, 5)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_sub_mul(
    ///         w,
    ///         &Natural::from_str("18446744073709551617").unwrap(),
    ///         &Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(79061536536625392587627883465, 999999999963106511852580896822)"
    /// );
    /// ```
    #[inline]
    fn mod_sub_mul(mut self, w: Self, c: &Natural, m: &Natural) -> Self {
        self.mod_sub_mul_assign(w, c, m);
        self
    }
}

impl ModSubMul<&Self, Natural, Natural> for NaturalVector {
    type Output = Self;

    /// Subtracts a scalar multiple of a [`NaturalVector`] from a [`NaturalVector`] modulo $m$,
    /// taking the first vector by value, the second by reference, the scalar by value, and the
    /// modulus by value. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMul`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()` times
    /// `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_sub_mul(&w, Natural::from(3u32), Natural::from(7u32))
    ///         .to_string(),
    ///     "(6, 4, 5)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_sub_mul(
    ///         &w,
    ///         Natural::from_str("18446744073709551617").unwrap(),
    ///         Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(79061536536625392587627883465, 999999999963106511852580896822)"
    /// );
    /// ```
    #[inline]
    fn mod_sub_mul(mut self, w: &Self, c: Natural, m: Natural) -> Self {
        self.mod_sub_mul_assign(w, c, m);
        self
    }
}

impl ModSubMul<&Self, Natural, &Natural> for NaturalVector {
    type Output = Self;

    /// Subtracts a scalar multiple of a [`NaturalVector`] from a [`NaturalVector`] modulo $m$,
    /// taking the first vector by value, the second by reference, the scalar by value, and the
    /// modulus by reference. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMul`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()` times
    /// `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_sub_mul(&w, Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "(6, 4, 5)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_sub_mul(
    ///         &w,
    ///         Natural::from_str("18446744073709551617").unwrap(),
    ///         &Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(79061536536625392587627883465, 999999999963106511852580896822)"
    /// );
    /// ```
    #[inline]
    fn mod_sub_mul(mut self, w: &Self, c: Natural, m: &Natural) -> Self {
        self.mod_sub_mul_assign(w, c, m);
        self
    }
}

impl ModSubMul<&Self, &Natural, Natural> for NaturalVector {
    type Output = Self;

    /// Subtracts a scalar multiple of a [`NaturalVector`] from a [`NaturalVector`] modulo $m$,
    /// taking the first vector by value, the second by reference, the scalar by reference, and the
    /// modulus by value. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMul`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()` times
    /// `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_sub_mul(&w, &Natural::from(3u32), Natural::from(7u32))
    ///         .to_string(),
    ///     "(6, 4, 5)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_sub_mul(
    ///         &w,
    ///         &Natural::from_str("18446744073709551617").unwrap(),
    ///         Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(79061536536625392587627883465, 999999999963106511852580896822)"
    /// );
    /// ```
    #[inline]
    fn mod_sub_mul(mut self, w: &Self, c: &Natural, m: Natural) -> Self {
        self.mod_sub_mul_assign(w, c, m);
        self
    }
}

impl ModSubMul<&Self, &Natural, &Natural> for NaturalVector {
    type Output = Self;

    /// Subtracts a scalar multiple of a [`NaturalVector`] from a [`NaturalVector`] modulo $m$,
    /// taking the first vector by value, the second by reference, the scalar by reference, and the
    /// modulus by reference. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMul`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()` times
    /// `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_sub_mul(&w, &Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "(6, 4, 5)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_sub_mul(
    ///         &w,
    ///         &Natural::from_str("18446744073709551617").unwrap(),
    ///         &Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(79061536536625392587627883465, 999999999963106511852580896822)"
    /// );
    /// ```
    #[inline]
    fn mod_sub_mul(mut self, w: &Self, c: &Natural, m: &Natural) -> Self {
        self.mod_sub_mul_assign(w, c, m);
        self
    }
}

impl ModSubMul<&NaturalVector, &Natural, &Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Subtracts a scalar multiple of a [`NaturalVector`] from a [`NaturalVector`] modulo $m$,
    /// taking both vectors, the scalar, and the modulus by reference. The elements and the scalar
    /// must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMul`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()` times
    /// `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_sub_mul(&w, &Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "(6, 4, 5)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_sub_mul(
    ///         &w,
    ///         &Natural::from_str("18446744073709551617").unwrap(),
    ///         &Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(79061536536625392587627883465, 999999999963106511852580896822)"
    /// );
    /// ```
    fn mod_sub_mul(self, w: &NaturalVector, c: &Natural, m: &Natural) -> NaturalVector {
        assert_reduced(self, w, c, m);
        let mut elements = self.elements.clone();
        mod_sub_mul_assign_ref(&mut elements, &w.elements, c, m);
        NaturalVector { elements }
    }
}

impl ModSubMulAssign<Self, Natural, Natural> for NaturalVector {
    /// Subtracts a scalar multiple of a [`NaturalVector`] from a [`NaturalVector`] modulo $m$, in
    /// place, taking the vector on the right-hand side by value, the scalar by value, and the
    /// modulus by value. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMul`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()` times
    /// `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_sub_mul_assign(w, Natural::from(3u32), Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(6, 4, 5)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_sub_mul_assign(
    ///     w,
    ///     Natural::from_str("18446744073709551617").unwrap(),
    ///     Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(79061536536625392587627883465, 999999999963106511852580896822)"
    /// );
    /// ```
    fn mod_sub_mul_assign(&mut self, w: Self, c: Natural, m: Natural) {
        assert_reduced(self, &w, &c, &m);
        mod_sub_mul_assign_val(&mut self.elements, w.elements, &c, &m);
    }
}

impl ModSubMulAssign<Self, Natural, &Natural> for NaturalVector {
    /// Subtracts a scalar multiple of a [`NaturalVector`] from a [`NaturalVector`] modulo $m$, in
    /// place, taking the vector on the right-hand side by value, the scalar by value, and the
    /// modulus by reference. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMul`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()` times
    /// `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_sub_mul_assign(w, Natural::from(3u32), &Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(6, 4, 5)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_sub_mul_assign(
    ///     w,
    ///     Natural::from_str("18446744073709551617").unwrap(),
    ///     &Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(79061536536625392587627883465, 999999999963106511852580896822)"
    /// );
    /// ```
    fn mod_sub_mul_assign(&mut self, w: Self, c: Natural, m: &Natural) {
        assert_reduced(self, &w, &c, m);
        mod_sub_mul_assign_val(&mut self.elements, w.elements, &c, m);
    }
}

impl ModSubMulAssign<Self, &Natural, Natural> for NaturalVector {
    /// Subtracts a scalar multiple of a [`NaturalVector`] from a [`NaturalVector`] modulo $m$, in
    /// place, taking the vector on the right-hand side by value, the scalar by reference, and the
    /// modulus by value. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMul`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()` times
    /// `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_sub_mul_assign(w, &Natural::from(3u32), Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(6, 4, 5)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_sub_mul_assign(
    ///     w,
    ///     &Natural::from_str("18446744073709551617").unwrap(),
    ///     Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(79061536536625392587627883465, 999999999963106511852580896822)"
    /// );
    /// ```
    fn mod_sub_mul_assign(&mut self, w: Self, c: &Natural, m: Natural) {
        assert_reduced(self, &w, c, &m);
        mod_sub_mul_assign_val(&mut self.elements, w.elements, c, &m);
    }
}

impl ModSubMulAssign<Self, &Natural, &Natural> for NaturalVector {
    /// Subtracts a scalar multiple of a [`NaturalVector`] from a [`NaturalVector`] modulo $m$, in
    /// place, taking the vector on the right-hand side by value, the scalar by reference, and the
    /// modulus by reference. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMul`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()` times
    /// `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_sub_mul_assign(w, &Natural::from(3u32), &Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(6, 4, 5)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_sub_mul_assign(
    ///     w,
    ///     &Natural::from_str("18446744073709551617").unwrap(),
    ///     &Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(79061536536625392587627883465, 999999999963106511852580896822)"
    /// );
    /// ```
    fn mod_sub_mul_assign(&mut self, w: Self, c: &Natural, m: &Natural) {
        assert_reduced(self, &w, c, m);
        mod_sub_mul_assign_val(&mut self.elements, w.elements, c, m);
    }
}

impl ModSubMulAssign<&Self, Natural, Natural> for NaturalVector {
    /// Subtracts a scalar multiple of a [`NaturalVector`] from a [`NaturalVector`] modulo $m$, in
    /// place, taking the vector on the right-hand side by reference, the scalar by value, and the
    /// modulus by value. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMul`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()` times
    /// `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_sub_mul_assign(&w, Natural::from(3u32), Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(6, 4, 5)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_sub_mul_assign(
    ///     &w,
    ///     Natural::from_str("18446744073709551617").unwrap(),
    ///     Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(79061536536625392587627883465, 999999999963106511852580896822)"
    /// );
    /// ```
    fn mod_sub_mul_assign(&mut self, w: &Self, c: Natural, m: Natural) {
        assert_reduced(self, w, &c, &m);
        mod_sub_mul_assign_ref(&mut self.elements, &w.elements, &c, &m);
    }
}

impl ModSubMulAssign<&Self, Natural, &Natural> for NaturalVector {
    /// Subtracts a scalar multiple of a [`NaturalVector`] from a [`NaturalVector`] modulo $m$, in
    /// place, taking the vector on the right-hand side by reference, the scalar by value, and the
    /// modulus by reference. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMul`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()` times
    /// `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_sub_mul_assign(&w, Natural::from(3u32), &Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(6, 4, 5)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_sub_mul_assign(
    ///     &w,
    ///     Natural::from_str("18446744073709551617").unwrap(),
    ///     &Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(79061536536625392587627883465, 999999999963106511852580896822)"
    /// );
    /// ```
    fn mod_sub_mul_assign(&mut self, w: &Self, c: Natural, m: &Natural) {
        assert_reduced(self, w, &c, m);
        mod_sub_mul_assign_ref(&mut self.elements, &w.elements, &c, m);
    }
}

impl ModSubMulAssign<&Self, &Natural, Natural> for NaturalVector {
    /// Subtracts a scalar multiple of a [`NaturalVector`] from a [`NaturalVector`] modulo $m$, in
    /// place, taking the vector on the right-hand side by reference, the scalar by reference, and
    /// the modulus by value. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMul`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()` times
    /// `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_sub_mul_assign(&w, &Natural::from(3u32), Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(6, 4, 5)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_sub_mul_assign(
    ///     &w,
    ///     &Natural::from_str("18446744073709551617").unwrap(),
    ///     Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(79061536536625392587627883465, 999999999963106511852580896822)"
    /// );
    /// ```
    fn mod_sub_mul_assign(&mut self, w: &Self, c: &Natural, m: Natural) {
        assert_reduced(self, w, c, &m);
        mod_sub_mul_assign_ref(&mut self.elements, &w.elements, c, &m);
    }
}

impl ModSubMulAssign<&Self, &Natural, &Natural> for NaturalVector {
    /// Subtracts a scalar multiple of a [`NaturalVector`] from a [`NaturalVector`] modulo $m$, in
    /// place, taking the vector on the right-hand side by reference, the scalar by reference, and
    /// the modulus by reference. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModSubMul`] implementation that takes everything by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()` times
    /// `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions, or if any element of `self` or `w`, or
    /// `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_sub_mul_assign(&w, &Natural::from(3u32), &Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(6, 4, 5)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_sub_mul_assign(
    ///     &w,
    ///     &Natural::from_str("18446744073709551617").unwrap(),
    ///     &Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(79061536536625392587627883465, 999999999963106511852580896822)"
    /// );
    /// ```
    fn mod_sub_mul_assign(&mut self, w: &Self, c: &Natural, m: &Natural) {
        assert_reduced(self, w, c, m);
        mod_sub_mul_assign_ref(&mut self.elements, &w.elements, c, m);
    }
}
