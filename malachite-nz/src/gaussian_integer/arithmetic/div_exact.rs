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
use crate::gaussian_integer::arithmetic::mul::{mul_val_ref, mul_val_val};
use crate::integer::Integer;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{
    AbsSquared, Conjugate, DivExact, DivExactAssign, DivIAssign, PowerOf2,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom, SciMantissaAndExponent};
use malachite_base::rounding_modes::RoundingMode::Down;

// Quotients this small are computed in double precision, as in `fmpzi_divexact`: with the quotient
// known to be exact and below 2^45, the rounding errors of the double-precision conjugate product
// and norm stay well below 1/2, so rounding to the nearest integer recovers it.
const DOUBLE_QUOTIENT_BITS: u64 = 45;

// Above this size the operands are scaled by 2^(-x_bits) before conversion to doubles, which keeps
// the intermediate products finite without changing the quotient.
const DOUBLE_SCALING_BITS: u64 = 500;

// `fmpz_get_d` truncates toward zero.
fn to_f64_truncated(x: &Integer) -> f64 {
    f64::rounding_from(x, Down).0
}

// An approximation to x * 2^(-shift), as in `fmpz_get_d_2exp` followed by `d_mul_2exp`; the
// exponent is clamped at -1024 as FLINT does, so the result can underflow to a subnormal or to zero
// only when it would be negligible anyway.
fn to_f64_scaled(x: &Integer, shift: u64) -> f64 {
    if *x == 0u32 {
        return 0.0;
    }
    let (m, e): (f64, u64) = x.unsigned_abs_ref().sci_mantissa_and_exponent();
    let v = m * f64::power_of_2((i64::exact_from(e) - i64::exact_from(shift)).max(-1024));
    if *x < 0u32 { -v } else { v }
}

// The double-precision path of `fmpzi_divexact`: the nearest-integer rounding of the exact quotient
// x * conj(y) / N(y), evaluated in doubles.
fn div_exact_double(x: &GaussianInteger, y: &GaussianInteger, x_bits: u64) -> GaussianInteger {
    let (a, b, c, d) = if x_bits < DOUBLE_SCALING_BITS {
        (
            to_f64_truncated(&x.real),
            to_f64_truncated(&x.imaginary),
            to_f64_truncated(&y.real),
            to_f64_truncated(&y.imaginary),
        )
    } else {
        (
            to_f64_scaled(&x.real, x_bits),
            to_f64_scaled(&x.imaginary, x_bits),
            to_f64_scaled(&y.real, x_bits),
            to_f64_scaled(&y.imaginary, x_bits),
        )
    };
    let t = a * c + b * d;
    let u = b * c - a * d;
    let v = c * c + d * d;
    let w = 0.5 / v;
    let t = (2.0 * t + v) * w;
    let u = (2.0 * u + v) * w;
    GaussianInteger {
        real: Integer::exact_from(t.floor()),
        imaginary: Integer::exact_from(u.floor()),
    }
}

// The general path: multiply by the conjugate and divide both parts exactly by the norm.
fn div_exact_general(t: GaussianInteger, norm: Integer) -> GaussianInteger {
    GaussianInteger {
        real: t.real.div_exact(&norm),
        imaginary: t.imaginary.div_exact(norm),
    }
}

// `fmpzi_divexact` also has a tier for very unbalanced operands, where both are truncated and an
// approximate division of the truncated values yields the exact quotient; it needs
// `fmpzi_divrem_approx`, which will be ported along with the Gaussian GCD, and until then such
// operands take the general path.
fn div_exact_val_ref(x: GaussianInteger, y: &GaussianInteger) -> GaussianInteger {
    if y.imaginary == 0u32 {
        assert!(y.real != 0u32, "division by zero");
        return GaussianInteger {
            real: x.real.div_exact(&y.real),
            imaginary: x.imaginary.div_exact(&y.real),
        };
    } else if y.real == 0u32 {
        let mut q = GaussianInteger {
            real: x.real.div_exact(&y.imaginary),
            imaginary: x.imaginary.div_exact(&y.imaginary),
        };
        q.div_i_assign();
        return q;
    }
    let x_bits = x.max_significant_bits();
    if x_bits == 0 {
        return GaussianInteger::from(0u32);
    }
    let y_bits = y.max_significant_bits();
    if x_bits < y_bits + DOUBLE_QUOTIENT_BITS {
        div_exact_double(&x, y, x_bits)
    } else {
        let norm = y.abs_squared();
        div_exact_general(mul_val_val(x, y.conjugate()), norm)
    }
}

fn div_exact_ref_ref(x: &GaussianInteger, y: &GaussianInteger) -> GaussianInteger {
    if y.imaginary == 0u32 {
        assert!(y.real != 0u32, "division by zero");
        return GaussianInteger {
            real: (&x.real).div_exact(&y.real),
            imaginary: (&x.imaginary).div_exact(&y.real),
        };
    } else if y.real == 0u32 {
        let mut q = GaussianInteger {
            real: (&x.real).div_exact(&y.imaginary),
            imaginary: (&x.imaginary).div_exact(&y.imaginary),
        };
        q.div_i_assign();
        return q;
    }
    let x_bits = x.max_significant_bits();
    if x_bits == 0 {
        return GaussianInteger::from(0u32);
    }
    let y_bits = y.max_significant_bits();
    if x_bits < y_bits + DOUBLE_QUOTIENT_BITS {
        div_exact_double(x, y, x_bits)
    } else {
        let norm = y.abs_squared();
        div_exact_general(mul_val_ref(y.conjugate(), x), norm)
    }
}

impl DivExact<Self> for GaussianInteger {
    type Output = Self;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`], taking both by value. The
    /// first [`GaussianInteger`] must be exactly divisible by the second. If it isn't, this
    /// function may panic or return a meaningless result.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
    /// $$
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
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (3+4i)(5-2i) = 23+14i
    /// let x = GaussianInteger::from_str("23+14i").unwrap();
    /// let y = GaussianInteger::from_str("5-2i").unwrap();
    /// assert_eq!((x.div_exact(y)).to_string(), "3+4i");
    /// ```
    #[inline]
    fn div_exact(self, other: Self) -> Self {
        div_exact_val_ref(self, &other)
    }
}

impl DivExact<&Self> for GaussianInteger {
    type Output = Self;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`], taking the first by value and
    /// the second by reference. The first [`GaussianInteger`] must be exactly divisible by the
    /// second. If it isn't, this function may panic or return a meaningless result.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
    /// $$
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
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (3+4i)(5-2i) = 23+14i
    /// let x = GaussianInteger::from_str("23+14i").unwrap();
    /// let y = GaussianInteger::from_str("5-2i").unwrap();
    /// assert_eq!((x.div_exact(&y)).to_string(), "3+4i");
    /// ```
    #[inline]
    fn div_exact(self, other: &Self) -> Self {
        div_exact_val_ref(self, other)
    }
}

impl DivExact<GaussianInteger> for &GaussianInteger {
    type Output = GaussianInteger;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`], taking the first by reference
    /// and the second by value. The first [`GaussianInteger`] must be exactly divisible by the
    /// second. If it isn't, this function may panic or return a meaningless result.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
    /// $$
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
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (3+4i)(5-2i) = 23+14i
    /// let x = GaussianInteger::from_str("23+14i").unwrap();
    /// let y = GaussianInteger::from_str("5-2i").unwrap();
    /// assert_eq!(((&x).div_exact(y)).to_string(), "3+4i");
    /// ```
    #[inline]
    fn div_exact(self, other: GaussianInteger) -> GaussianInteger {
        div_exact_ref_ref(self, &other)
    }
}

impl DivExact<&GaussianInteger> for &GaussianInteger {
    type Output = GaussianInteger;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`], taking both by reference. The
    /// first [`GaussianInteger`] must be exactly divisible by the second. If it isn't, this
    /// function may panic or return a meaningless result.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
    /// $$
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
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (3+4i)(5-2i) = 23+14i
    /// let x = GaussianInteger::from_str("23+14i").unwrap();
    /// let y = GaussianInteger::from_str("5-2i").unwrap();
    /// assert_eq!(((&x).div_exact(&y)).to_string(), "3+4i");
    /// ```
    #[inline]
    fn div_exact(self, other: &GaussianInteger) -> GaussianInteger {
        div_exact_ref_ref(self, other)
    }
}

impl DivExactAssign<Self> for GaussianInteger {
    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`] in place, taking the
    /// [`GaussianInteger`] on the right-hand side by value. The first [`GaussianInteger`] must be
    /// exactly divisible by the second. If it isn't, this function may panic or return a
    /// meaningless result.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
    /// $$
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
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivExactAssign;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (3+4i)(5-2i) = 23+14i
    /// let mut x = GaussianInteger::from_str("23+14i").unwrap();
    /// x.div_exact_assign(GaussianInteger::from_str("5-2i").unwrap());
    /// assert_eq!(x.to_string(), "3+4i");
    /// ```
    #[inline]
    fn div_exact_assign(&mut self, other: Self) {
        *self = div_exact_val_ref(take(self), &other);
    }
}

impl DivExactAssign<&Self> for GaussianInteger {
    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`] in place, taking the
    /// [`GaussianInteger`] on the right-hand side by reference. The first [`GaussianInteger`] must
    /// be exactly divisible by the second. If it isn't, this function may panic or return a
    /// meaningless result.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
    /// $$
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
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivExactAssign;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (3+4i)(5-2i) = 23+14i
    /// let mut x = GaussianInteger::from_str("23+14i").unwrap();
    /// x.div_exact_assign(&GaussianInteger::from_str("5-2i").unwrap());
    /// assert_eq!(x.to_string(), "3+4i");
    /// ```
    #[inline]
    fn div_exact_assign(&mut self, other: &Self) {
        *self = div_exact_val_ref(take(self), other);
    }
}
