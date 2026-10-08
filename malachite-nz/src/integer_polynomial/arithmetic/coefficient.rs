// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::arithmetic::scalar_mul::{
    integers_mul_scalar_assign, integers_mul_scalar_to_out,
};
use crate::natural::InnerNatural::Small;
use crate::natural::{LIMB_MAX_QUARTER, Natural, WIDTH_MINUS_2};
use crate::platform::{Limb, SignedDoubleLimb, SignedLimb};
use alloc::vec::Vec;
use core::fmt::Debug;
use core::ops::{AddAssign, MulAssign, SubAssign};
use malachite_base::num::arithmetic::traits::{
    AddMulAssign, DivExactAssign, Pow, Square, SubMulAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};

crate_test_trait! {
// The coefficients of the polynomials that the multiplication kernels work with: `Integer`s, for
// `IntegerPolynomial`s, and `Natural`s, for `NaturalPolynomial`s. The kernels are FLINT's, which
// work with signed coefficients; a coefficient is read as a sign and an absolute value, and the
// results are built from them. For `Natural`s the sign is always non-negative, so after
// monomorphization the sign handling reduces to FLINT's paths for non-negative coefficients.
PolynomialCoefficient:
    Clone
    + Debug
    + Default
    + Zero
    + One
    + Eq
    + From<u64>
    + for<'a> AddAssign<&'a Self>
    + for<'a> SubAssign<&'a Self>
    + for<'a> MulAssign<&'a Self>
    + for<'a> DivExactAssign<&'a Self>
    + for<'a> AddMulAssign<&'a Self, &'a Self>
    + for<'a> SubMulAssign<&'a Self, &'a Self>
    + ExactFrom<SignedLimb>
    + ExactFrom<SignedDoubleLimb>
{
    // Whether the coefficient is zero. Comparing with `Self::ZERO` instead would build and drop a
    // temporary zero, which is measurably slower in the kernels' inner loops.
    fn is_zero(&self) -> bool;

    // Whether the coefficient is negative.
    fn is_negative(&self) -> bool;

    // The absolute value of the coefficient.
    fn unsigned_abs_ref(&self) -> &Natural;

    // The coefficient with the given sign (`true` for non-negative) and absolute value. A `Natural`
    // coefficient cannot be negative, so for `Natural`s `sign` must be `true` unless `abs` is zero.
    fn from_sign_and_abs(sign: bool, abs: Natural) -> Self;

    // The negative of a coefficient. A `Natural` coefficient cannot be negative, so for `Natural`s
    // the coefficient must be zero.
    fn negate(self) -> Self;

    // Doubles a coefficient in place.
    fn double_assign(&mut self);

    // The sum of two coefficients.
    fn add_ref(&self, other: &Self) -> Self;

    // The product of two coefficients.
    fn mul_ref(&self, other: &Self) -> Self;

    // The square of a coefficient.
    fn square_ref(&self) -> Self;

    // The coefficient raised to the power `e`.
    fn pow_ref(&self, e: u64) -> Self;

    // Sets `out` to the first `out.len()` elements of `xs`, each multiplied by `c`.
    fn vec_mul_scalar_to_out(out: &mut [Self], xs: &[Self], c: &Self);

    // Multiplies each element of `xs` by `c`.
    fn vec_mul_scalar_assign(xs: &mut [Self], c: &Self);
}}

impl PolynomialCoefficient for Integer {
    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0u32
    }

    #[inline]
    fn is_negative(&self) -> bool {
        !self.sign
    }

    #[inline]
    fn unsigned_abs_ref(&self) -> &Natural {
        &self.abs
    }

    #[inline]
    fn from_sign_and_abs(sign: bool, abs: Natural) -> Self {
        Self::from_sign_and_abs(sign, abs)
    }

    #[inline]
    fn negate(self) -> Self {
        -self
    }

    #[inline]
    fn double_assign(&mut self) {
        *self <<= 1u32;
    }

    #[inline]
    fn add_ref(&self, other: &Self) -> Self {
        self + other
    }

    #[inline]
    fn mul_ref(&self, other: &Self) -> Self {
        self * other
    }

    #[inline]
    fn square_ref(&self) -> Self {
        self.square()
    }

    #[inline]
    fn pow_ref(&self, e: u64) -> Self {
        self.pow(e)
    }

    #[inline]
    fn vec_mul_scalar_to_out(out: &mut [Self], xs: &[Self], c: &Self) {
        integers_mul_scalar_to_out(out, xs, c);
    }

    #[inline]
    fn vec_mul_scalar_assign(xs: &mut [Self], c: &Self) {
        integers_mul_scalar_assign(xs, c);
    }
}

impl PolynomialCoefficient for Natural {
    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0u32
    }

    #[inline]
    fn is_negative(&self) -> bool {
        false
    }

    #[inline]
    fn unsigned_abs_ref(&self) -> &Natural {
        self
    }

    #[inline]
    fn from_sign_and_abs(sign: bool, abs: Natural) -> Self {
        assert!(
            sign || abs == 0u32,
            "a Natural coefficient cannot be negative"
        );
        abs
    }

    #[inline]
    fn negate(self) -> Self {
        assert_eq!(self, 0u32, "a Natural coefficient cannot be negative");
        self
    }

    #[inline]
    fn double_assign(&mut self) {
        *self <<= 1u32;
    }

    #[inline]
    fn add_ref(&self, other: &Self) -> Self {
        self + other
    }

    #[inline]
    fn mul_ref(&self, other: &Self) -> Self {
        self * other
    }

    #[inline]
    fn square_ref(&self) -> Self {
        self.square()
    }

    #[inline]
    fn pow_ref(&self, e: u64) -> Self {
        self.pow(e)
    }

    fn vec_mul_scalar_to_out(out: &mut [Self], xs: &[Self], c: &Self) {
        let xs = &xs[..out.len()];
        match *c {
            Self(Small(0)) => out.fill(Self::ZERO),
            Self(Small(1)) => out.clone_from_slice(xs),
            _ => {
                for (o, x) in out.iter_mut().zip(xs) {
                    *o = x * c;
                }
            }
        }
    }

    fn vec_mul_scalar_assign(xs: &mut [Self], c: &Self) {
        match *c {
            Self(Small(0)) => xs.fill(Self::ZERO),
            Self(Small(1)) => {}
            _ => {
                for x in xs {
                    *x *= c;
                }
            }
        }
    }
}

// Removes the zeros at the end of `xs`, the coefficients of a polynomial, so that its last element,
// if any, is nonzero.
pub(crate) fn trim_coefficients<C: PolynomialCoefficient>(xs: &mut Vec<C>) {
    while xs.last().is_some_and(PolynomialCoefficient::is_zero) {
        xs.pop();
    }
}

// Keeps only the first `len` of `xs`, the coefficients of a polynomial without zeros at the end,
// and then trims the result.
pub(crate) fn truncate_coefficients<C: PolynomialCoefficient>(xs: &mut Vec<C>, len: u64) {
    if let Ok(len) = usize::try_from(len)
        && len < xs.len()
    {
        xs.truncate(len);
        trim_coefficients(xs);
    }
}

// The largest absolute value of a small FLINT `fmpz`, which is stored in a single word with two
// bits to spare; larger values are stored as GMP integers. Some of FLINT's algorithms take a fast
// path for small values that depends on the spare bits, so a faithful translation takes it for the
// same values.
pub(crate) const COEFF_MAX: Limb = LIMB_MAX_QUARTER;

// The largest number of bits that FLINT stores in a small `fmpz`: `SMALL_FMPZ_BITCOUNT_MAX` from
// `flint.h`, FLINT 3.6.0.
pub(crate) const SMALL_FMPZ_BITCOUNT_MAX: u64 = WIDTH_MINUS_2;

// The value of `x` as a signed word, if FLINT would store `x` as a small `fmpz`; that is, if
// `COEFF_IS_MPZ` from `flint.h`, FLINT 3.6.0, would be false.
pub(crate) fn small_value<C: PolynomialCoefficient>(x: &C) -> Option<SignedLimb> {
    match *x.unsigned_abs_ref() {
        Natural(Small(small)) if small <= COEFF_MAX => {
            let value = SignedLimb::wrapping_from(small);
            Some(if x.is_negative() { -value } else { value })
        }
        _ => None,
    }
}

// The values of the elements of `xs`, each of which FLINT must store as a small `fmpz`, as signed
// words or as a wider signed type.
pub(crate) fn small_values<T: From<SignedLimb>, C: PolynomialCoefficient>(xs: &[C]) -> Vec<T> {
    xs.iter()
        .map(|x| T::from(small_value(x).unwrap()))
        .collect()
}
