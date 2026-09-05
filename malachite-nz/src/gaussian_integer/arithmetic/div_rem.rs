// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2022 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::gaussian_integer::arithmetic::div_exact::{
    DOUBLE_QUOTIENT_BITS, nearest_quotient_double,
};
use crate::gaussian_integer::arithmetic::mul::mul_val_ref;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{
    AbsSquared, Conjugate, DivAssignRem, DivRem, DivRound,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode::Floor;

// A dividend with this many fewer bits than the divisor (both measured as the larger of the two
// parts' bit counts) has norm less than N(y) / 8, so the nearest quotient is 0.
const SMALL_DIVIDEND_BITS: u64 = 2;

// The nearest quotient, computed as floor((2 x conj(y) + N(y)(1 + i)) / (2 N(y))), part by part.
fn nearest_quotient(x: &GaussianInteger, y: &GaussianInteger) -> GaussianInteger {
    let mut t = mul_val_ref(y.conjugate(), x);
    let mut norm = y.abs_squared();
    t.real <<= 1u32;
    t.imaginary <<= 1u32;
    t.real += &norm;
    t.imaginary += &norm;
    norm <<= 1u32;
    GaussianInteger {
        real: t.real.div_round(&norm, Floor).0,
        imaginary: t.imaginary.div_round(norm, Floor).0,
    }
}
// The nearest quotient, or `None` when it is zero because the dividend is zero or much smaller than
// the divisor.
pub(super) fn quotient_or_zero(
    x: &GaussianInteger,
    y: &GaussianInteger,
) -> Option<GaussianInteger> {
    let y_bits = y.max_significant_bits();
    assert!(y_bits != 0, "division by zero");
    let x_bits = x.max_significant_bits();
    if x_bits == 0 || x_bits + SMALL_DIVIDEND_BITS < y_bits {
        None
    } else {
        Some(nearest_quotient(x, y))
    }
}

// A port of `fmpzi_divrem_approx`: like `div_rem`, but when the operands are within 45 bits of each
// other the quotient is computed in double precision, so it may miss the nearest quotient by one in
// a part. The remainder is still small enough for a Euclidean step, which is all this is used for.
pub(super) fn div_rem_approx(
    x: &GaussianInteger,
    y: &GaussianInteger,
) -> (GaussianInteger, GaussianInteger) {
    let y_bits = y.max_significant_bits();
    assert!(y_bits != 0, "division by zero");
    let x_bits = x.max_significant_bits();
    if x_bits == 0 || x_bits + SMALL_DIVIDEND_BITS < y_bits {
        (GaussianInteger::ZERO, x.clone())
    } else if x_bits < y_bits + DOUBLE_QUOTIENT_BITS {
        let q = nearest_quotient_double(x, y, x_bits);
        let r = x - &q * y;
        (q, r)
    } else {
        div_rem_ref_ref(x, y)
    }
}

pub(super) fn div_rem_val_ref(
    x: GaussianInteger,
    y: &GaussianInteger,
) -> (GaussianInteger, GaussianInteger) {
    match quotient_or_zero(&x, y) {
        Some(q) => {
            let r = x - &q * y;
            (q, r)
        }
        None => (GaussianInteger::ZERO, x),
    }
}

pub(super) fn div_rem_ref_ref(
    x: &GaussianInteger,
    y: &GaussianInteger,
) -> (GaussianInteger, GaussianInteger) {
    match quotient_or_zero(x, y) {
        Some(q) => {
            let r = x - &q * y;
            (q, r)
        }
        None => (GaussianInteger::ZERO, x.clone()),
    }
}

impl DivRem<Self> for GaussianInteger {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`], taking both by value and
    /// returning the quotient and remainder.
    ///
    /// The quotient is the Gaussian integer nearest to the exact quotient, with each part rounded
    /// to the nearest integer and ties rounded up; the remainder is what is left over. This is the
    /// division of the Gaussian integers as a Euclidean domain: the quotient and remainder satisfy
    /// $x = qy + r$ and $N(r) \leq N(y) / 2$, where $N$ is the norm, so the remainder is always
    /// smaller than the divisor.
    ///
    /// $$
    /// f(x, y) = (q, x - qy), \quad \text{where } q = \left \lfloor \frac{x \bar{y}}{N(y)} +
    /// \frac{1 + i}{2} \right \rfloor
    /// $$
    /// and the floor is taken on each part.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (2+i)(3) + (-1) = 5+3i
    /// let x = GaussianInteger::from_str("5+3i").unwrap();
    /// let y = GaussianInteger::from_str("2+i").unwrap();
    /// let (q, r) = x.div_rem(y);
    /// assert_eq!(q.to_string(), "3");
    /// assert_eq!(r.to_string(), "-1");
    /// ```
    #[inline]
    fn div_rem(self, other: Self) -> (Self, Self) {
        div_rem_val_ref(self, &other)
    }
}

impl DivRem<&Self> for GaussianInteger {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`], taking the first by value and
    /// the second by reference and returning the quotient and remainder.
    ///
    /// The quotient is the Gaussian integer nearest to the exact quotient, with each part rounded
    /// to the nearest integer and ties rounded up; the remainder is what is left over. This is the
    /// division of the Gaussian integers as a Euclidean domain: the quotient and remainder satisfy
    /// $x = qy + r$ and $N(r) \leq N(y) / 2$, where $N$ is the norm, so the remainder is always
    /// smaller than the divisor.
    ///
    /// $$
    /// f(x, y) = (q, x - qy), \quad \text{where } q = \left \lfloor \frac{x \bar{y}}{N(y)} +
    /// \frac{1 + i}{2} \right \rfloor
    /// $$
    /// and the floor is taken on each part.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (2+i)(3) + (-1) = 5+3i
    /// let x = GaussianInteger::from_str("5+3i").unwrap();
    /// let y = GaussianInteger::from_str("2+i").unwrap();
    /// let (q, r) = x.div_rem(&y);
    /// assert_eq!(q.to_string(), "3");
    /// assert_eq!(r.to_string(), "-1");
    /// ```
    #[inline]
    fn div_rem(self, other: &Self) -> (Self, Self) {
        div_rem_val_ref(self, other)
    }
}

impl DivRem<GaussianInteger> for &GaussianInteger {
    type DivOutput = GaussianInteger;
    type RemOutput = GaussianInteger;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`], taking the first by reference
    /// and the second by value and returning the quotient and remainder.
    ///
    /// The quotient is the Gaussian integer nearest to the exact quotient, with each part rounded
    /// to the nearest integer and ties rounded up; the remainder is what is left over. This is the
    /// division of the Gaussian integers as a Euclidean domain: the quotient and remainder satisfy
    /// $x = qy + r$ and $N(r) \leq N(y) / 2$, where $N$ is the norm, so the remainder is always
    /// smaller than the divisor.
    ///
    /// $$
    /// f(x, y) = (q, x - qy), \quad \text{where } q = \left \lfloor \frac{x \bar{y}}{N(y)} +
    /// \frac{1 + i}{2} \right \rfloor
    /// $$
    /// and the floor is taken on each part.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (2+i)(3) + (-1) = 5+3i
    /// let x = GaussianInteger::from_str("5+3i").unwrap();
    /// let y = GaussianInteger::from_str("2+i").unwrap();
    /// let (q, r) = (&x).div_rem(y);
    /// assert_eq!(q.to_string(), "3");
    /// assert_eq!(r.to_string(), "-1");
    /// ```
    #[inline]
    fn div_rem(self, other: GaussianInteger) -> (GaussianInteger, GaussianInteger) {
        div_rem_ref_ref(self, &other)
    }
}

impl DivRem<&GaussianInteger> for &GaussianInteger {
    type DivOutput = GaussianInteger;
    type RemOutput = GaussianInteger;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`], taking both by reference and
    /// returning the quotient and remainder.
    ///
    /// The quotient is the Gaussian integer nearest to the exact quotient, with each part rounded
    /// to the nearest integer and ties rounded up; the remainder is what is left over. This is the
    /// division of the Gaussian integers as a Euclidean domain: the quotient and remainder satisfy
    /// $x = qy + r$ and $N(r) \leq N(y) / 2$, where $N$ is the norm, so the remainder is always
    /// smaller than the divisor.
    ///
    /// $$
    /// f(x, y) = (q, x - qy), \quad \text{where } q = \left \lfloor \frac{x \bar{y}}{N(y)} +
    /// \frac{1 + i}{2} \right \rfloor
    /// $$
    /// and the floor is taken on each part.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (2+i)(3) + (-1) = 5+3i
    /// let x = GaussianInteger::from_str("5+3i").unwrap();
    /// let y = GaussianInteger::from_str("2+i").unwrap();
    /// let (q, r) = (&x).div_rem(&y);
    /// assert_eq!(q.to_string(), "3");
    /// assert_eq!(r.to_string(), "-1");
    /// ```
    #[inline]
    fn div_rem(self, other: &GaussianInteger) -> (GaussianInteger, GaussianInteger) {
        div_rem_ref_ref(self, other)
    }
}

impl DivAssignRem<Self> for GaussianInteger {
    type RemOutput = Self;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`] in place, taking the
    /// [`GaussianInteger`] on the right-hand side by value and returning the remainder.
    ///
    /// The quotient is the Gaussian integer nearest to the exact quotient, with each part rounded
    /// to the nearest integer and ties rounded up; the remainder is what is left over. This is the
    /// division of the Gaussian integers as a Euclidean domain: the quotient and remainder satisfy
    /// $x = qy + r$ and $N(r) \leq N(y) / 2$, where $N$ is the norm, so the remainder is always
    /// smaller than the divisor.
    ///
    /// $$
    /// x \gets q, \quad f(x, y) = x - qy, \quad \text{where }
    /// q = \left \lfloor \frac{x \bar{y}}{N(y)} + \frac{1 + i}{2} \right \rfloor
    /// $$
    /// and the floor is taken on each part.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivAssignRem;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (2+i)(3) + (-1) = 5+3i
    /// let mut x = GaussianInteger::from_str("5+3i").unwrap();
    /// let r = x.div_assign_rem(GaussianInteger::from_str("2+i").unwrap());
    /// assert_eq!(x.to_string(), "3");
    /// assert_eq!(r.to_string(), "-1");
    /// ```
    #[inline]
    fn div_assign_rem(&mut self, other: Self) -> Self {
        let (q, r) = div_rem_val_ref(take(self), &other);
        *self = q;
        r
    }
}

impl DivAssignRem<&Self> for GaussianInteger {
    type RemOutput = Self;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`] in place, taking the
    /// [`GaussianInteger`] on the right-hand side by reference and returning the remainder.
    ///
    /// The quotient is the Gaussian integer nearest to the exact quotient, with each part rounded
    /// to the nearest integer and ties rounded up; the remainder is what is left over. This is the
    /// division of the Gaussian integers as a Euclidean domain: the quotient and remainder satisfy
    /// $x = qy + r$ and $N(r) \leq N(y) / 2$, where $N$ is the norm, so the remainder is always
    /// smaller than the divisor.
    ///
    /// $$
    /// x \gets q, \quad f(x, y) = x - qy, \quad \text{where }
    /// q = \left \lfloor \frac{x \bar{y}}{N(y)} + \frac{1 + i}{2} \right \rfloor
    /// $$
    /// and the floor is taken on each part.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivAssignRem;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (2+i)(3) + (-1) = 5+3i
    /// let mut x = GaussianInteger::from_str("5+3i").unwrap();
    /// let r = x.div_assign_rem(&GaussianInteger::from_str("2+i").unwrap());
    /// assert_eq!(x.to_string(), "3");
    /// assert_eq!(r.to_string(), "-1");
    /// ```
    #[inline]
    fn div_assign_rem(&mut self, other: &Self) -> Self {
        let (q, r) = div_rem_val_ref(take(self), other);
        *self = q;
        r
    }
}
