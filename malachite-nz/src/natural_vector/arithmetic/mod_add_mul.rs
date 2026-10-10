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
    ModAddAssign, ModAddMul, ModAddMulAssign, ModIsReduced, ModMulPrecomputed,
    ModMulPrecomputedAssign, ModSubAssign,
};
use malachite_base::num::basic::traits::{One, Zero};

fn assert_reduced(v: &NaturalVector, w: &NaturalVector, c: &Natural, m: &Natural) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot add-multiply vectors of different dimensions"
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

// Adds `ys` times `c` to `xs` modulo `m`, in place, for inputs already known to be reduced. Like
// FLINT, this handles `c` equal to 0, 1, and `m - 1` directly, and otherwise computes the data for
// multiplying modulo `m` once and reuses it for every element.
fn mod_add_mul_assign_ref(xs: &mut [Natural], ys: &[Natural], c: &Natural, m: &Natural) {
    match *c {
        Natural::ZERO => {}
        Natural::ONE => {
            for (x, y) in xs.iter_mut().zip(ys) {
                x.mod_add_assign(y, m);
            }
        }
        _ if *c == m - Natural::ONE => {
            for (x, y) in xs.iter_mut().zip(ys) {
                x.mod_sub_assign(y, m);
            }
        }
        _ => {
            let data =
                <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
            for (x, y) in xs.iter_mut().zip(ys) {
                x.mod_add_assign(y.mod_mul_precomputed(c, m, &data), m);
            }
        }
    }
}

// As `mod_add_mul_assign_ref`, consuming `ys`, whose elements hold the products.
fn mod_add_mul_assign_val(xs: &mut [Natural], ys: Vec<Natural>, c: &Natural, m: &Natural) {
    match *c {
        Natural::ZERO => {}
        Natural::ONE => {
            for (x, y) in xs.iter_mut().zip(ys) {
                x.mod_add_assign(y, m);
            }
        }
        _ if *c == m - Natural::ONE => {
            for (x, y) in xs.iter_mut().zip(ys) {
                x.mod_sub_assign(y, m);
            }
        }
        _ => {
            let data =
                <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
            for (x, mut y) in xs.iter_mut().zip(ys) {
                y.mod_mul_precomputed_assign(c, m, &data);
                x.mod_add_assign(y, m);
            }
        }
    }
}

impl ModAddMul<Self, Natural, Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $m$, taking both
    /// vectors, the scalar, and the modulus by value. The elements and the scalar must already be
    /// reduced modulo $m$.
    ///
    /// The operation is taken element by element, so the result has the same dimension as the
    /// vectors.
    ///
    /// $$
    /// f(v, w, c, m) = (v + cw) \bmod m.
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
    /// use malachite_base::num::arithmetic::traits::ModAddMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul(w, Natural::from(3u32), Natural::from(7u32))
    ///         .to_string(),
    ///     "(4, 5, 1)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul(
    ///         w,
    ///         Natural::from_str("18446744073709551617").unwrap(),
    ///         Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(920938463463374607412372116594, 73786976294838206467)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_addmul_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_add_mul(mut self, w: Self, c: Natural, m: Natural) -> Self {
        self.mod_add_mul_assign(w, c, m);
        self
    }
}

impl ModAddMul<Self, Natural, &Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $m$, taking the
    /// first vector by value, the second by value, the scalar by value, and the modulus by
    /// reference. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMul`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::ModAddMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul(w, Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "(4, 5, 1)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul(
    ///         w,
    ///         Natural::from_str("18446744073709551617").unwrap(),
    ///         &Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(920938463463374607412372116594, 73786976294838206467)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_addmul_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_add_mul(mut self, w: Self, c: Natural, m: &Natural) -> Self {
        self.mod_add_mul_assign(w, c, m);
        self
    }
}

impl ModAddMul<Self, &Natural, Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $m$, taking the
    /// first vector by value, the second by value, the scalar by reference, and the modulus by
    /// value. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMul`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::ModAddMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul(w, &Natural::from(3u32), Natural::from(7u32))
    ///         .to_string(),
    ///     "(4, 5, 1)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul(
    ///         w,
    ///         &Natural::from_str("18446744073709551617").unwrap(),
    ///         Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(920938463463374607412372116594, 73786976294838206467)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_addmul_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_add_mul(mut self, w: Self, c: &Natural, m: Natural) -> Self {
        self.mod_add_mul_assign(w, c, m);
        self
    }
}

impl ModAddMul<Self, &Natural, &Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $m$, taking the
    /// first vector by value, the second by value, the scalar by reference, and the modulus by
    /// reference. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMul`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::ModAddMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul(w, &Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "(4, 5, 1)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul(
    ///         w,
    ///         &Natural::from_str("18446744073709551617").unwrap(),
    ///         &Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(920938463463374607412372116594, 73786976294838206467)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_addmul_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_add_mul(mut self, w: Self, c: &Natural, m: &Natural) -> Self {
        self.mod_add_mul_assign(w, c, m);
        self
    }
}

impl ModAddMul<&Self, Natural, Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $m$, taking the
    /// first vector by value, the second by reference, the scalar by value, and the modulus by
    /// value. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMul`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::ModAddMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul(&w, Natural::from(3u32), Natural::from(7u32))
    ///         .to_string(),
    ///     "(4, 5, 1)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul(
    ///         &w,
    ///         Natural::from_str("18446744073709551617").unwrap(),
    ///         Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(920938463463374607412372116594, 73786976294838206467)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_addmul_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_add_mul(mut self, w: &Self, c: Natural, m: Natural) -> Self {
        self.mod_add_mul_assign(w, c, m);
        self
    }
}

impl ModAddMul<&Self, Natural, &Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $m$, taking the
    /// first vector by value, the second by reference, the scalar by value, and the modulus by
    /// reference. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMul`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::ModAddMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul(&w, Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "(4, 5, 1)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul(
    ///         &w,
    ///         Natural::from_str("18446744073709551617").unwrap(),
    ///         &Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(920938463463374607412372116594, 73786976294838206467)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_addmul_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_add_mul(mut self, w: &Self, c: Natural, m: &Natural) -> Self {
        self.mod_add_mul_assign(w, c, m);
        self
    }
}

impl ModAddMul<&Self, &Natural, Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $m$, taking the
    /// first vector by value, the second by reference, the scalar by reference, and the modulus by
    /// value. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMul`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::ModAddMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul(&w, &Natural::from(3u32), Natural::from(7u32))
    ///         .to_string(),
    ///     "(4, 5, 1)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul(
    ///         &w,
    ///         &Natural::from_str("18446744073709551617").unwrap(),
    ///         Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(920938463463374607412372116594, 73786976294838206467)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_addmul_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_add_mul(mut self, w: &Self, c: &Natural, m: Natural) -> Self {
        self.mod_add_mul_assign(w, c, m);
        self
    }
}

impl ModAddMul<&Self, &Natural, &Natural> for NaturalVector {
    type Output = Self;

    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $m$, taking the
    /// first vector by value, the second by reference, the scalar by reference, and the modulus by
    /// reference. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMul`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::ModAddMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul(&w, &Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "(4, 5, 1)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_add_mul(
    ///         &w,
    ///         &Natural::from_str("18446744073709551617").unwrap(),
    ///         &Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(920938463463374607412372116594, 73786976294838206467)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_addmul_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_add_mul(mut self, w: &Self, c: &Natural, m: &Natural) -> Self {
        self.mod_add_mul_assign(w, c, m);
        self
    }
}

impl ModAddMul<&NaturalVector, &Natural, &Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $m$, taking both
    /// vectors, the scalar, and the modulus by reference. The elements and the scalar must already
    /// be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMul`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::ModAddMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_add_mul(&w, &Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "(4, 5, 1)"
    /// );
    ///
    /// let v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_add_mul(
    ///         &w,
    ///         &Natural::from_str("18446744073709551617").unwrap(),
    ///         &Natural::from_str("1000000000000000000000000000057").unwrap()
    ///     )
    ///     .to_string(),
    ///     "(920938463463374607412372116594, 73786976294838206467)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_addmul_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0.
    fn mod_add_mul(self, w: &NaturalVector, c: &Natural, m: &Natural) -> NaturalVector {
        assert_reduced(self, w, c, m);
        let mut elements = self.elements.clone();
        mod_add_mul_assign_ref(&mut elements, &w.elements, c, m);
        NaturalVector { elements }
    }
}

impl ModAddMulAssign<Self, Natural, Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $m$, in place,
    /// taking the vector on the right-hand side by value, the scalar by value, and the modulus by
    /// value. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMul`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::ModAddMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_add_mul_assign(w, Natural::from(3u32), Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(4, 5, 1)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_add_mul_assign(
    ///     w,
    ///     Natural::from_str("18446744073709551617").unwrap(),
    ///     Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(920938463463374607412372116594, 73786976294838206467)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_addmul_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0.
    fn mod_add_mul_assign(&mut self, w: Self, c: Natural, m: Natural) {
        assert_reduced(self, &w, &c, &m);
        mod_add_mul_assign_val(&mut self.elements, w.elements, &c, &m);
    }
}

impl ModAddMulAssign<Self, Natural, &Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $m$, in place,
    /// taking the vector on the right-hand side by value, the scalar by value, and the modulus by
    /// reference. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMul`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::ModAddMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_add_mul_assign(w, Natural::from(3u32), &Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(4, 5, 1)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_add_mul_assign(
    ///     w,
    ///     Natural::from_str("18446744073709551617").unwrap(),
    ///     &Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(920938463463374607412372116594, 73786976294838206467)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_addmul_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0.
    fn mod_add_mul_assign(&mut self, w: Self, c: Natural, m: &Natural) {
        assert_reduced(self, &w, &c, m);
        mod_add_mul_assign_val(&mut self.elements, w.elements, &c, m);
    }
}

impl ModAddMulAssign<Self, &Natural, Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $m$, in place,
    /// taking the vector on the right-hand side by value, the scalar by reference, and the modulus
    /// by value. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMul`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::ModAddMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_add_mul_assign(w, &Natural::from(3u32), Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(4, 5, 1)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_add_mul_assign(
    ///     w,
    ///     &Natural::from_str("18446744073709551617").unwrap(),
    ///     Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(920938463463374607412372116594, 73786976294838206467)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_addmul_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0.
    fn mod_add_mul_assign(&mut self, w: Self, c: &Natural, m: Natural) {
        assert_reduced(self, &w, c, &m);
        mod_add_mul_assign_val(&mut self.elements, w.elements, c, &m);
    }
}

impl ModAddMulAssign<Self, &Natural, &Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $m$, in place,
    /// taking the vector on the right-hand side by value, the scalar by reference, and the modulus
    /// by reference. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMul`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::ModAddMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_add_mul_assign(w, &Natural::from(3u32), &Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(4, 5, 1)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_add_mul_assign(
    ///     w,
    ///     &Natural::from_str("18446744073709551617").unwrap(),
    ///     &Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(920938463463374607412372116594, 73786976294838206467)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_addmul_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0.
    fn mod_add_mul_assign(&mut self, w: Self, c: &Natural, m: &Natural) {
        assert_reduced(self, &w, c, m);
        mod_add_mul_assign_val(&mut self.elements, w.elements, c, m);
    }
}

impl ModAddMulAssign<&Self, Natural, Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $m$, in place,
    /// taking the vector on the right-hand side by reference, the scalar by value, and the modulus
    /// by value. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMul`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::ModAddMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_add_mul_assign(&w, Natural::from(3u32), Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(4, 5, 1)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_add_mul_assign(
    ///     &w,
    ///     Natural::from_str("18446744073709551617").unwrap(),
    ///     Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(920938463463374607412372116594, 73786976294838206467)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_addmul_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0.
    fn mod_add_mul_assign(&mut self, w: &Self, c: Natural, m: Natural) {
        assert_reduced(self, w, &c, &m);
        mod_add_mul_assign_ref(&mut self.elements, &w.elements, &c, &m);
    }
}

impl ModAddMulAssign<&Self, Natural, &Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $m$, in place,
    /// taking the vector on the right-hand side by reference, the scalar by value, and the modulus
    /// by reference. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMul`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::ModAddMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_add_mul_assign(&w, Natural::from(3u32), &Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(4, 5, 1)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_add_mul_assign(
    ///     &w,
    ///     Natural::from_str("18446744073709551617").unwrap(),
    ///     &Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(920938463463374607412372116594, 73786976294838206467)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_addmul_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0.
    fn mod_add_mul_assign(&mut self, w: &Self, c: Natural, m: &Natural) {
        assert_reduced(self, w, &c, m);
        mod_add_mul_assign_ref(&mut self.elements, &w.elements, &c, m);
    }
}

impl ModAddMulAssign<&Self, &Natural, Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $m$, in place,
    /// taking the vector on the right-hand side by reference, the scalar by reference, and the
    /// modulus by value. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMul`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::ModAddMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_add_mul_assign(&w, &Natural::from(3u32), Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(4, 5, 1)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_add_mul_assign(
    ///     &w,
    ///     &Natural::from_str("18446744073709551617").unwrap(),
    ///     Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(920938463463374607412372116594, 73786976294838206467)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_addmul_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0.
    fn mod_add_mul_assign(&mut self, w: &Self, c: &Natural, m: Natural) {
        assert_reduced(self, w, c, &m);
        mod_add_mul_assign_ref(&mut self.elements, &w.elements, c, &m);
    }
}

impl ModAddMulAssign<&Self, &Natural, &Natural> for NaturalVector {
    /// Adds a scalar multiple of a [`NaturalVector`] to a [`NaturalVector`] modulo $m$, in place,
    /// taking the vector on the right-hand side by reference, the scalar by reference, and the
    /// modulus by reference. The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModAddMul`] implementation that takes everything by value
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
    /// use malachite_base::num::arithmetic::traits::ModAddMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(2, 6, 4)").unwrap();
    /// v.mod_add_mul_assign(&w, &Natural::from(3u32), &Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(4, 5, 1)");
    ///
    /// let mut v = NaturalVector::from_str("(1, 18446744073709551616)").unwrap();
    /// let w = NaturalVector::from_str("(18446744073709551615, 3)").unwrap();
    /// v.mod_add_mul_assign(
    ///     &w,
    ///     &Natural::from_str("18446744073709551617").unwrap(),
    ///     &Natural::from_str("1000000000000000000000000000057").unwrap(),
    /// );
    /// assert_eq!(
    ///     v.to_string(),
    ///     "(920938463463374607412372116594, 73786976294838206467)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_addmul_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0.
    fn mod_add_mul_assign(&mut self, w: &Self, c: &Natural, m: &Natural) {
        assert_reduced(self, w, c, m);
        mod_add_mul_assign_ref(&mut self.elements, &w.elements, c, m);
    }
}
