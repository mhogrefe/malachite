// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(doc)]
use malachite_base::polynomial::arithmetic::traits::{
    BitPack, BitUnpack, Compose, ComposeAssign, ComposePowerOfX, ComposePowerOfXAssign,
    DeflatePowerOfX, DeflatePowerOfXAssign, DivPowerOfX, DivPowerOfXAssign, Evaluate, EvaluateMany,
    ExponentGcd, FloorL2Norm, L2NormSquared, MulMiddle, MulMiddleAssign, MulPowerOfX,
    MulPowerOfXAssign, TaylorShift, TaylorShiftAssign,
};

/// Implementations of [`Add`](core::ops::Add) and [`AddAssign`](core::ops::AddAssign), for adding
/// two polynomials.
pub mod add;
/// Implementations of [`BitPack`], which packs a polynomial's coefficients into fixed-width fields
/// of a single number.
pub mod bit_pack;
/// Implementations of [`BitUnpack`], which unpacks a polynomial from the fixed-width fields of a
/// single number.
pub mod bit_unpack;
/// Implementations of [`CanonicalPrimitivePart`](
/// malachite_base::num::arithmetic::traits::CanonicalPrimitivePart),
/// [`CanonicalPrimitivePartAssign`](
/// malachite_base::num::arithmetic::traits::CanonicalPrimitivePartAssign), and
/// [`ContentAndCanonicalPrimitivePart`](
/// malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart), which, for a
/// polynomial with non-negative coefficients, are the same as the primitive part.
pub mod canonical_primitive_part;
/// Implementations of
/// [`CanonicalizeUnit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit) and
/// [`CanonicalizeUnitAssign`](malachite_base::num::arithmetic::traits::CanonicalizeUnitAssign),
/// which bring a polynomial into canonical unit form.
pub mod canonicalize_unit;
/// Implementations of [`Compose`] and [`ComposeAssign`], for composing two polynomials.
pub mod compose;
/// Implementations of [`ComposePowerOfX`] and [`ComposePowerOfXAssign`], for substituting a power
/// of the variable into a polynomial.
pub mod compose_power_of_x;
/// Implementations of [`Content`](malachite_base::num::arithmetic::traits::Content),
/// [`PrimitivePart`](malachite_base::num::arithmetic::traits::PrimitivePart),
/// [`PrimitivePartAssign`](malachite_base::num::arithmetic::traits::PrimitivePartAssign), and
/// [`ContentAndPrimitivePart`](malachite_base::num::arithmetic::traits::ContentAndPrimitivePart),
/// which compute the GCD of a polynomial's coefficients and the polynomial divided by it.
pub mod content;
/// Implementations of [`DeflatePowerOfX`] and [`DeflatePowerOfXAssign`], for undoing the
/// substitution of a power of the variable into a polynomial.
pub mod deflate_power_of_x;
/// Implementations of [`DivPowerOfX`] and [`DivPowerOfXAssign`], for dividing a polynomial by a
/// power of its variable and discarding the remainder.
pub mod div_power_of_x;
/// Implementations of [`Evaluate`], which evaluates a polynomial at a value.
pub mod evaluate;
/// An implementation of [`EvaluateMany`], which evaluates a polynomial at several values.
pub mod evaluate_many;
/// An implementation of [`ExponentGcd`], the greatest common divisor of the exponents at which a
/// polynomial has nonzero coefficients.
pub mod exponent_gcd;
/// An implementation of [`FloorL2Norm`], the floor of a polynomial's $L^2$ norm.
pub mod floor_l2_norm;
/// Implementations of [`Height`](malachite_base::num::arithmetic::traits::Height) and
/// [`HeightRef`](malachite_base::num::arithmetic::traits::HeightRef), the largest of the magnitudes
/// of a polynomial's coefficients.
pub mod height;
/// An implementation of [`IsUnit`](malachite_base::num::arithmetic::traits::IsUnit), a trait for
/// determining whether a number is a unit of its ring.
pub mod is_unit;
/// An implementation of [`L2NormSquared`], the sum of the squares of a polynomial's coefficients.
pub mod l2_norm_squared;
/// Implementations of [`Mul`](core::ops::Mul) and [`MulAssign`](core::ops::MulAssign), for
/// multiplying two polynomials.
pub mod mul;
/// Implementations of [`MulMiddle`] and [`MulMiddleAssign`], for multiplying two polynomials and
/// keeping only a middle range of coefficients of the product.
pub mod mul_middle;
/// Implementations of [`MulPowerOfX`] and [`MulPowerOfXAssign`], for multiplying a polynomial by a
/// power of its variable.
pub mod mul_power_of_x;
/// Implementations of [`Pow`](malachite_base::num::arithmetic::traits::Pow) and
/// [`PowAssign`](malachite_base::num::arithmetic::traits::PowAssign), for raising a polynomial to a
/// power.
pub mod pow;
/// Left-shifting a [`NaturalPolynomial`](super::NaturalPolynomial) (multiplying it by a power of
/// 2), by shifting every coefficient.
///
/// # shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::natural_polynomial::NaturalPolynomial;
///
/// assert_eq!((NaturalPolynomial::ZERO << 10u8).to_string(), "0");
/// assert_eq!(
///     (NaturalPolynomial::from_str("x^2+3*x+5").unwrap() << 0u16).to_string(),
///     "x^2+3*x+5"
/// );
/// assert_eq!(
///     (NaturalPolynomial::from_str("x^2+3*x+5").unwrap() << 2u32).to_string(),
///     "4*x^2+12*x+20"
/// );
/// assert_eq!(
///     (NaturalPolynomial::from_str("x^2+3*x+5").unwrap() << 100u64).to_string(),
///     "1267650600228229401496703205376*x^2+3802951800684688204490109616128*x+\
///     6338253001141147007483516026880"
/// );
///
/// assert_eq!((&NaturalPolynomial::ZERO << 10u8).to_string(), "0");
/// assert_eq!(
///     (&NaturalPolynomial::from_str("x^2+3*x+5").unwrap() << 0u16).to_string(),
///     "x^2+3*x+5"
/// );
/// assert_eq!(
///     (&NaturalPolynomial::from_str("x^2+3*x+5").unwrap() << 2u32).to_string(),
///     "4*x^2+12*x+20"
/// );
/// assert_eq!(
///     (&NaturalPolynomial::from_str("x^2+3*x+5").unwrap() << 100u64).to_string(),
///     "1267650600228229401496703205376*x^2+3802951800684688204490109616128*x+\
///     6338253001141147007483516026880"
/// );
/// ```
///
/// # shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::natural_polynomial::NaturalPolynomial;
///
/// let mut p = NaturalPolynomial::ZERO;
/// p <<= 10u8;
/// assert_eq!(p.to_string(), "0");
///
/// let mut p = NaturalPolynomial::from_str("x^2+3*x+5").unwrap();
/// p <<= 0u16;
/// assert_eq!(p.to_string(), "x^2+3*x+5");
///
/// let mut p = NaturalPolynomial::from_str("x^2+3*x+5").unwrap();
/// p <<= 2u32;
/// assert_eq!(p.to_string(), "4*x^2+12*x+20");
///
/// let mut p = NaturalPolynomial::from_str("x^2+3*x+5").unwrap();
/// p <<= 100u64;
/// assert_eq!(
///     p.to_string(),
///     "1267650600228229401496703205376*x^2+3802951800684688204490109616128*x+\
///     6338253001141147007483516026880"
/// );
/// ```
pub mod shl;
/// Implementations of [`Square`](malachite_base::num::arithmetic::traits::Square) and
/// [`SquareAssign`](malachite_base::num::arithmetic::traits::SquareAssign), for squaring a
/// polynomial.
pub mod square;
/// Implementations of [`TaylorShift`] and [`TaylorShiftAssign`], for shifting a polynomial by a
/// constant.
pub mod taylor_shift;
