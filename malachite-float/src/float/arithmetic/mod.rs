// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Absolute value of [`Float`](super::Float)s.
pub mod abs;
/// Implementations of [`AbsSquared`](malachite_base::num::arithmetic::traits::AbsSquared) and
/// [`AbsSquaredAssign`](malachite_base::num::arithmetic::traits::AbsSquaredAssign), traits for
/// computing the squared absolute value of a number. For real types this is the same as squaring.
pub mod abs_squared;
/// Addition of [`Float`](super::Float)s, and of [`Float`](super::Float)s with
/// [`Rational`](malachite_q::Rational)s.
pub mod add;
/// [`AddMul`](malachite_base::num::arithmetic::traits::AddMul) and
/// [`AddMulAssign`](malachite_base::num::arithmetic::traits::AddMulAssign), traits for adding a
/// [`Float`](super::Float) and the product of two other [`Float`](super::Float)s — or of a
/// [`Float`](super::Float) and a [`Rational`](malachite_q::Rational) — with a single rounding
/// (fused multiply-add), and the associated precision- and rounding-mode-aware functions.
pub mod add_mul;
/// [`Average`](malachite_base::num::arithmetic::traits::Average) and
/// [`AverageAssign`](malachite_base::num::arithmetic::traits::AverageAssign), traits for computing
/// the average (arithmetic mean) of two numbers, and the associated precision- and
/// rounding-mode-aware functions.
///
/// # average
/// ```
/// use malachite_base::num::arithmetic::traits::Average;
/// use malachite_float::Float;
///
/// assert_eq!(Float::from(1.5).average(Float::from(2.5)), 2.0);
/// assert_eq!((&Float::from(1.5)).average(&Float::from(2.5)), 2.0);
/// // the output precision is the maximum of the inputs', so the average of two one-bit
/// // `Float`s is itself rounded to one bit, here to the even neighbor
/// assert_eq!(Float::from(1.0).average(Float::from(2.0)), 2.0);
/// // the sum would overflow, but the average is in range
/// let max = Float::max_finite_value_with_prec(10);
/// assert_eq!((&max).average(&max), max);
/// ```
///
/// # average_assign
/// ```
/// use malachite_base::num::arithmetic::traits::AverageAssign;
/// use malachite_float::Float;
///
/// let mut x = Float::from(1.5);
/// x.average_assign(Float::from(2.5));
/// assert_eq!(x, 2.0);
/// ```
///
/// # average_prec_round
/// ```
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// // the exact average of 1 and 2 is 1.5, which needs 2 bits
/// let (avg, o) = Float::from(1.0).average_prec_round(Float::from(2.0), 2, Exact);
/// assert_eq!(avg, 1.5);
/// assert_eq!(o, Equal);
///
/// // at one bit it must be rounded
/// let (avg, o) = Float::from(1.0).average_prec_round(Float::from(2.0), 1, Floor);
/// assert_eq!(avg, 1.0);
/// assert_eq!(o, Less);
/// let (avg, o) = Float::from(1.0).average_prec_round(Float::from(2.0), 1, Ceiling);
/// assert_eq!(avg, 2.0);
/// assert_eq!(o, Greater);
/// ```
///
/// # average_prec
/// ```
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let (avg, o) = Float::from(1.0).average_prec(Float::from(2.0), 10);
/// assert_eq!(avg, 1.5);
/// assert_eq!(o, Equal);
/// ```
///
/// # average_round
/// ```
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// // the output precision is the maximum of the inputs' precisions
/// let (avg, o) = Float::from(1.5).average_round(Float::from(2.5), Nearest);
/// assert_eq!(avg, 2.0);
/// assert_eq!(o, Equal);
/// ```
///
/// # average_prec_round_assign
/// ```
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let mut x = Float::from(1.0);
/// assert_eq!(
///     x.average_prec_round_assign(Float::from(2.0), 1, Floor),
///     Less
/// );
/// assert_eq!(x, 1.0);
///
/// let mut x = Float::from(1.0);
/// assert_eq!(
///     x.average_prec_round_assign_ref(&Float::from(2.0), 2, Exact),
///     Equal
/// );
/// assert_eq!(x, 1.5);
/// ```
///
/// # average_prec_assign
/// ```
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let mut x = Float::from(1.0);
/// assert_eq!(x.average_prec_assign(Float::from(2.0), 10), Equal);
/// assert_eq!(x, 1.5);
///
/// let mut x = Float::from(1.0);
/// assert_eq!(x.average_prec_assign_ref(&Float::from(2.0), 10), Equal);
/// assert_eq!(x, 1.5);
/// ```
///
/// # average_round_assign
/// ```
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let mut x = Float::from(1.5);
/// assert_eq!(x.average_round_assign(Float::from(2.5), Nearest), Equal);
/// assert_eq!(x, 2.0);
///
/// let mut x = Float::from(1.5);
/// assert_eq!(
///     x.average_round_assign_ref(&Float::from(2.5), Nearest),
///     Equal
/// );
/// assert_eq!(x, 2.0);
/// ```
pub mod average;
/// An implementation of
/// [`CanonicalUnitIPow`](malachite_base::num::arithmetic::traits::CanonicalUnitIPow), a trait for
/// finding the power of $i$ that brings a number into canonical unit form.
pub mod canonical_unit_i_pow;
/// Implementations of
/// [`CanonicalizeGaussianUnit`](malachite_base::num::arithmetic::traits::CanonicalizeGaussianUnit)
/// and [`CanonicalizeGaussianUnitAssign`](
/// malachite_base::num::arithmetic::traits::CanonicalizeGaussianUnitAssign), traits for bringing a
/// number into canonical Gaussian-unit form.
pub mod canonicalize_gaussian_unit;
/// Implementations of
/// [`CanonicalizeUnit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit) and
/// [`CanonicalizeUnitAssign`](malachite_base::num::arithmetic::traits::CanonicalizeUnitAssign),
/// traits for bringing a number into canonical unit form.
pub mod canonicalize_unit;
/// Cube root of [`Float`](super::Float)s and of [`Rational`](malachite_q::Rational)s.
pub mod cbrt;
/// Implementations of [`Conjugate`](malachite_base::num::arithmetic::traits::Conjugate) and
/// [`ConjugateAssign`](malachite_base::num::arithmetic::traits::ConjugateAssign), traits for
/// computing the complex conjugate of a number. A real number is its own conjugate.
pub mod conjugate;
/// Division of [`Float`](super::Float)s, of [`Float`](super::Float)s by
/// [`Rational`](malachite_q::Rational)s, and of [`Rational`](malachite_q::Rational)s by
/// [`Float`](super::Float)s.
pub mod div;
/// Correctly-rounded dot products of [`Float`](super::Float) slices, with the products computed
/// exactly and a single rounding at the end, so that intermediate overflow and underflow cannot
/// occur.
pub mod dot;
/// Fractional parts of [`Float`](super::Float)s: the `fractional_part` and
/// `integer_and_fractional_parts` families.
pub mod fractional_part;
/// [`Hypot`](malachite_base::num::arithmetic::traits::Hypot) and
/// [`HypotAssign`](malachite_base::num::arithmetic::traits::HypotAssign), traits for computing the
/// hypotenuse of two numbers, $\sqrt{x^2+y^2}$.
///
/// # hypot
/// ```
/// use core::f64::consts::{E, PI};
/// use malachite_base::num::arithmetic::traits::Hypot;
/// use malachite_float::Float;
///
/// assert_eq!(
///     Float::from(PI).hypot(Float::from(E)).to_string(),
///     "4.1543544023133130"
/// );
/// ```
pub mod hypot;
/// An implementation of [`IsPowerOf2`](malachite_base::num::arithmetic::traits::IsPowerOf2), a
/// trait for determining whether a number is an integer power of 2.
pub mod is_power_of_2;
/// An implementation of [`IsUnit`](malachite_base::num::arithmetic::traits::IsUnit), a trait for
/// determining whether a number is a unit of its ring.
pub mod is_unit;
/// Multiplication of [`Float`](super::Float)s, and of [`Float`](super::Float)s with
/// [`Rational`](malachite_q::Rational)s.
pub mod mul;
/// [`MulAddMul`](malachite_base::num::arithmetic::traits::MulAddMul) and
/// [`MulAddMulAssign`](malachite_base::num::arithmetic::traits::MulAddMulAssign), traits for adding
/// the products of two pairs of [`Float`](super::Float)s with a single rounding, and the associated
/// precision- and rounding-mode-aware functions.
pub mod mul_add_mul;
/// [`MulSubMul`](malachite_base::num::arithmetic::traits::MulSubMul) and
/// [`MulSubMulAssign`](malachite_base::num::arithmetic::traits::MulSubMulAssign), traits for
/// subtracting the product of one pair of [`Float`](super::Float)s from the product of another pair
/// with a single rounding, and the associated precision- and rounding-mode-aware functions.
pub mod mul_sub_mul;
/// Negation of [`Float`](super::Float)s.
pub mod neg;
/// [`positive_difference_prec_round`](super::Float::positive_difference_prec_round) and related
/// functions, for computing positive differences of [`Float`](super::Float)s.
pub mod positive_difference;
/// Implementations of [`PowerOf2`](malachite_base::num::arithmetic::traits::PowerOf2), a trait for
/// computing a power of 2.
pub mod pow;
pub mod power_of_2;
/// Correctly-rounded multiplication of any number of [`Float`](super::Float)s: functions computing
/// a slice's product with a single rounding at the end, and the [`Product`](core::iter::Product)
/// implementations built on them.
pub mod product;
/// Implementations of [`Reciprocal`](malachite_base::num::arithmetic::traits::Reciprocal) and
/// [`ReciprocalAssign`](malachite_base::num::arithmetic::traits::ReciprocalAssign), traits for
/// computing the reciprocal of a number.
pub mod reciprocal;
/// [`ReciprocalSqrt`](malachite_base::num::arithmetic::traits::ReciprocalSqrt) and
/// [`ReciprocalSqrtAssign`](malachite_base::num::arithmetic::traits::ReciprocalSqrtAssign), traits
/// for computing the reciprocal of the square root of [`Float`](super::Float)s.
pub mod reciprocal_sqrt;
/// [`rem_prec_round`](super::Float::rem_prec_round) and related functions, for computing
/// floating-point remainders of [`Float`](super::Float)s.
pub mod rem;
/// [`root_u_prec_round`](super::Float::root_u_prec_round) and related functions, for computing
/// roots of [`Float`](super::Float)s.
pub mod root;
pub(crate) mod round_near_x;
pub mod round_to_integer;
/// Left-shifting a [`Float`](super::Float) (multiplying it by a power of 2).
///
/// # shl
/// ```
/// use malachite_base::num::basic::traits::{Infinity, Zero};
/// use malachite_float::Float;
///
/// assert_eq!(Float::ZERO << 10, 0);
/// assert_eq!(Float::INFINITY << 10, Float::INFINITY);
/// assert_eq!(
///     (Float::from(std::f64::consts::PI) << 10u8).to_string(),
///     "3216.9908772759482"
/// );
/// assert_eq!(
///     (Float::from(std::f64::consts::PI) << -10i8).to_string(),
///     "0.0030679615757712823"
/// );
///
/// assert_eq!(&Float::ZERO << 10, 0);
/// assert_eq!(&Float::INFINITY << 10, Float::INFINITY);
/// assert_eq!(
///     (&Float::from(std::f64::consts::PI) << 10u8).to_string(),
///     "3216.9908772759482"
/// );
/// assert_eq!(
///     (&Float::from(std::f64::consts::PI) << -10i8).to_string(),
///     "0.0030679615757712823"
/// );
/// ```
///
/// # shl_assign
/// ```
/// use malachite_base::num::basic::traits::{Infinity, Zero};
/// use malachite_float::Float;
///
/// let mut x = Float::ZERO;
/// x <<= 10;
/// assert_eq!(x, 0);
///
/// let mut x = Float::INFINITY;
/// x <<= 10;
/// assert_eq!(x, Float::INFINITY);
///
/// let mut x = Float::from(std::f64::consts::PI);
/// x <<= 10;
/// assert_eq!(x.to_string(), "3216.9908772759482");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// x <<= -10;
/// assert_eq!(x.to_string(), "0.0030679615757712823");
/// ```
pub mod shl;
/// Implementations of [`ShlRound`](malachite_base::num::arithmetic::traits::ShlRound) and
/// [`ShlRoundAssign`](malachite_base::num::arithmetic::traits::ShlRoundAssign), traits for
/// multiplying a number by a power of 2 and rounding according to a specified
/// [`RoundingMode`](malachite_base::rounding_modes::RoundingMode). For [`Float`](super::Float)s,
/// rounding is only necessary in the cases of overflow and underflow.
///
/// # shl_prec_round
/// ```
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_prec_round(10u8, 10, Nearest);
/// assert_eq!(shifted.to_string(), "3216.0");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_prec_round(-10i8, 10, Nearest);
/// assert_eq!(shifted.to_string(), "0.0030670");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_prec_round(u32::MAX, 10, Floor);
/// assert_eq!(shifted.to_string(), "2.0965e323228496");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_prec_round(u32::MAX, 10, Ceiling);
/// assert_eq!(shifted.to_string(), "Infinity");
/// assert_eq!(o, Greater);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_prec_round_ref(10u8, 10, Nearest);
/// assert_eq!(shifted.to_string(), "3216.0");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_prec_round_ref(-10i8, 10, Nearest);
/// assert_eq!(shifted.to_string(), "0.0030670");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_prec_round_ref(u32::MAX, 10, Floor);
/// assert_eq!(shifted.to_string(), "2.0965e323228496");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_prec_round_ref(u32::MAX, 10, Ceiling);
/// assert_eq!(shifted.to_string(), "Infinity");
/// assert_eq!(o, Greater);
/// ```
///
/// # shl_prec_round_assign
/// ```
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shl_prec_round_assign(10u8, 10, Nearest), Less);
/// assert_eq!(x.to_string(), "3216.0");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shl_prec_round_assign(-10i8, 10, Nearest), Less);
/// assert_eq!(x.to_string(), "0.0030670");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shl_prec_round_assign(u32::MAX, 10, Floor), Less);
/// assert_eq!(x.to_string(), "2.0965e323228496");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shl_prec_round_assign(u32::MAX, 10, Ceiling), Greater);
/// assert_eq!(x.to_string(), "Infinity");
/// ```
///
/// # shl_prec
/// ```
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_prec(10u8, 10);
/// assert_eq!(shifted.to_string(), "3216.0");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_prec(-10i8, 10);
/// assert_eq!(shifted.to_string(), "0.0030670");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_prec(u32::MAX, 10);
/// assert_eq!(shifted.to_string(), "Infinity");
/// assert_eq!(o, Greater);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_prec_ref(10u8, 10);
/// assert_eq!(shifted.to_string(), "3216.0");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_prec_ref(-10i8, 10);
/// assert_eq!(shifted.to_string(), "0.0030670");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_prec_ref(u32::MAX, 10);
/// assert_eq!(shifted.to_string(), "Infinity");
/// assert_eq!(o, Greater);
/// ```
///
/// # shl_prec_assign
/// ```
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shl_prec_assign(10u8, 10), Less);
/// assert_eq!(x.to_string(), "3216.0");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shl_prec_assign(-10i8, 10), Less);
/// assert_eq!(x.to_string(), "0.0030670");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shl_prec_assign(u32::MAX, 10), Greater);
/// assert_eq!(x.to_string(), "Infinity");
/// ```
///
/// # shl_round
/// ```
/// use malachite_base::num::arithmetic::traits::ShlRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_round(10u8, Nearest);
/// assert_eq!(shifted.to_string(), "3216.9908772759482");
/// assert_eq!(o, Equal);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_round(-10i8, Nearest);
/// assert_eq!(shifted.to_string(), "0.0030679615757712823");
/// assert_eq!(o, Equal);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_round(u32::MAX, Floor);
/// assert_eq!(shifted.to_string(), "2.0985787164673858e323228496");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shl_round(u32::MAX, Ceiling);
/// assert_eq!(shifted.to_string(), "Infinity");
/// assert_eq!(o, Greater);
///
/// let (shifted, o) = (&Float::from(std::f64::consts::PI)).shl_round(10u8, Nearest);
/// assert_eq!(shifted.to_string(), "3216.9908772759482");
/// assert_eq!(o, Equal);
///
/// let (shifted, o) = (&Float::from(std::f64::consts::PI)).shl_round(-10i8, Nearest);
/// assert_eq!(shifted.to_string(), "0.0030679615757712823");
/// assert_eq!(o, Equal);
///
/// let (shifted, o) = (&Float::from(std::f64::consts::PI)).shl_round(u32::MAX, Floor);
/// assert_eq!(shifted.to_string(), "2.0985787164673858e323228496");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = (&Float::from(std::f64::consts::PI)).shl_round(u32::MAX, Ceiling);
/// assert_eq!(shifted.to_string(), "Infinity");
/// assert_eq!(o, Greater);
/// ```
///
/// # shl_round_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ShlRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shl_round_assign(10u8, Nearest), Equal);
/// assert_eq!(x.to_string(), "3216.9908772759482");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shl_round_assign(-10i8, Nearest), Equal);
/// assert_eq!(x.to_string(), "0.0030679615757712823");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shl_round_assign(u32::MAX, Floor), Less);
/// assert_eq!(x.to_string(), "2.0985787164673858e323228496");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shl_round_assign(u32::MAX, Ceiling), Greater);
/// assert_eq!(x.to_string(), "Infinity");
/// ```
pub mod shl_round;
/// Right-shifting a [`Float`](super::Float) (dividing it by a power of 2).
///
/// # shr
/// ```
/// use malachite_base::num::basic::traits::{Infinity, Zero};
/// use malachite_float::Float;
///
/// assert_eq!(Float::ZERO >> 10, 0);
/// assert_eq!(Float::INFINITY >> 10, Float::INFINITY);
/// assert_eq!(
///     (Float::from(std::f64::consts::PI) >> 10u8).to_string(),
///     "0.0030679615757712823"
/// );
/// assert_eq!(
///     (Float::from(std::f64::consts::PI) >> -10i8).to_string(),
///     "3216.9908772759482"
/// );
///
/// assert_eq!(&Float::ZERO >> 10, 0);
/// assert_eq!(&Float::INFINITY >> 10, Float::INFINITY);
/// assert_eq!(
///     (&Float::from(std::f64::consts::PI) >> 10u8).to_string(),
///     "0.0030679615757712823"
/// );
/// assert_eq!(
///     (&Float::from(std::f64::consts::PI) >> -10i8).to_string(),
///     "3216.9908772759482"
/// );
/// ```
///
/// # shr_assign
/// ```
/// use malachite_base::num::basic::traits::{Infinity, Zero};
/// use malachite_float::Float;
///
/// let mut x = Float::ZERO;
/// x >>= 10;
/// assert_eq!(x, 0);
///
/// let mut x = Float::INFINITY;
/// x >>= 10;
/// assert_eq!(x, Float::INFINITY);
///
/// let mut x = Float::from(std::f64::consts::PI);
/// x >>= 10;
/// assert_eq!(x.to_string(), "0.0030679615757712823");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// x >>= -10;
/// assert_eq!(x.to_string(), "3216.9908772759482");
/// ```
pub mod shr;
/// Implementations of [`ShlRound`](malachite_base::num::arithmetic::traits::ShrRound) and
/// [`ShrRoundAssign`](malachite_base::num::arithmetic::traits::ShrRoundAssign), traits for dividing
/// a number by a power of 2 and rounding according to a specified
/// [`RoundingMode`](malachite_base::rounding_modes::RoundingMode). For [`Float`](super::Float)s,
/// rounding is only necessary in the cases of overflow and underflow.
///
/// # shr_prec_round
/// ```
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_prec_round(10u8, 10, Nearest);
/// assert_eq!(shifted.to_string(), "0.0030670");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_prec_round(-10i8, 10, Nearest);
/// assert_eq!(shifted.to_string(), "3216.0");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_prec_round(u32::MAX, 10, Floor);
/// assert_eq!(shifted.to_string(), "0.0");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_prec_round(u32::MAX, 10, Ceiling);
/// assert_eq!(shifted.to_string(), "2.3826e-323228497");
/// assert_eq!(o, Greater);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_prec_round_ref(10u8, 10, Nearest);
/// assert_eq!(shifted.to_string(), "0.0030670");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_prec_round_ref(-10i8, 10, Nearest);
/// assert_eq!(shifted.to_string(), "3216.0");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_prec_round_ref(u32::MAX, 10, Floor);
/// assert_eq!(shifted.to_string(), "0.0");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_prec_round_ref(u32::MAX, 10, Ceiling);
/// assert_eq!(shifted.to_string(), "2.3826e-323228497");
/// assert_eq!(o, Greater);
/// ```
///
/// # shr_prec_round_assign
/// ```
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shr_prec_round_assign(10u8, 10, Nearest), Less);
/// assert_eq!(x.to_string(), "0.0030670");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shr_prec_round_assign(-10i8, 10, Nearest), Less);
/// assert_eq!(x.to_string(), "3216.0");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shr_prec_round_assign(u32::MAX, 10, Floor), Less);
/// assert_eq!(x.to_string(), "0.0");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shr_prec_round_assign(u32::MAX, 10, Ceiling), Greater);
/// assert_eq!(x.to_string(), "2.3826e-323228497");
/// ```
///
/// # shr_prec
/// ```
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_prec(10u8, 10);
/// assert_eq!(shifted.to_string(), "0.0030670");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_prec(-10i8, 10);
/// assert_eq!(shifted.to_string(), "3216.0");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_prec(u32::MAX, 10);
/// assert_eq!(shifted.to_string(), "0.0");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_prec_ref(10u8, 10);
/// assert_eq!(shifted.to_string(), "0.0030670");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_prec_ref(-10i8, 10);
/// assert_eq!(shifted.to_string(), "3216.0");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_prec_ref(u32::MAX, 10);
/// assert_eq!(shifted.to_string(), "0.0");
/// assert_eq!(o, Less);
/// ```
///
/// # shr_prec_assign
/// ```
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shr_prec_assign(10u8, 10), Less);
/// assert_eq!(x.to_string(), "0.0030670");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shr_prec_assign(-10i8, 10), Less);
/// assert_eq!(x.to_string(), "3216.0");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shr_prec_assign(u32::MAX, 10), Less);
/// assert_eq!(x.to_string(), "0.0");
/// ```
///
/// # shr_round
/// ```
/// use malachite_base::num::arithmetic::traits::ShrRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_round(10u8, Nearest);
/// assert_eq!(shifted.to_string(), "0.0030679615757712823");
/// assert_eq!(o, Equal);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_round(-10i8, Nearest);
/// assert_eq!(shifted.to_string(), "3216.9908772759482");
/// assert_eq!(o, Equal);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_round(u32::MAX, Floor);
/// assert_eq!(shifted.to_string(), "0.0");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = Float::from(std::f64::consts::PI).shr_round(u32::MAX, Ceiling);
/// assert_eq!(shifted.to_string(), "2.3825649048879511e-323228497");
/// assert_eq!(o, Greater);
///
/// let (shifted, o) = (&Float::from(std::f64::consts::PI)).shr_round(10u8, Nearest);
/// assert_eq!(shifted.to_string(), "0.0030679615757712823");
/// assert_eq!(o, Equal);
///
/// let (shifted, o) = (&Float::from(std::f64::consts::PI)).shr_round(-10i8, Nearest);
/// assert_eq!(shifted.to_string(), "3216.9908772759482");
/// assert_eq!(o, Equal);
///
/// let (shifted, o) = (&Float::from(std::f64::consts::PI)).shr_round(u32::MAX, Floor);
/// assert_eq!(shifted.to_string(), "0.0");
/// assert_eq!(o, Less);
///
/// let (shifted, o) = (&Float::from(std::f64::consts::PI)).shr_round(u32::MAX, Ceiling);
/// assert_eq!(shifted.to_string(), "2.3825649048879511e-323228497");
/// assert_eq!(o, Greater);
/// ```
///
/// # shr_round_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ShrRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::Float;
/// use std::cmp::Ordering::*;
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shr_round_assign(10u8, Nearest), Equal);
/// assert_eq!(x.to_string(), "0.0030679615757712823");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shr_round_assign(-10i8, Nearest), Equal);
/// assert_eq!(x.to_string(), "3216.9908772759482");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shr_round_assign(u32::MAX, Floor), Less);
/// assert_eq!(x.to_string(), "0.0");
///
/// let mut x = Float::from(std::f64::consts::PI);
/// assert_eq!(x.shr_round_assign(u32::MAX, Ceiling), Greater);
/// assert_eq!(x.to_string(), "2.3825649048879511e-323228497");
/// ```
pub mod shr_round;
/// An implementation of [`Sign`](malachite_base::num::arithmetic::traits::Sign), a trait for
/// determining the sign of a number.
pub mod sign;
/// [`Sqrt`](malachite_base::num::arithmetic::traits::Sqrt) and
/// [`SqrtAssign`](malachite_base::num::arithmetic::traits::SqrtAssign), traits for computing the
/// square root of [`Float`](super::Float)s.
pub mod sqrt;
/// Squaring of [`Float`](super::Float)s.
pub mod square;
/// Subtraction of [`Float`](super::Float)s, of [`Float`](super::Float)s by
/// [`Rational`](malachite_q::Rational)s, and of [`Rational`](malachite_q::Rational)s by
/// [`Float`](super::Float)s.
pub mod sub;
/// [`SubMul`](malachite_base::num::arithmetic::traits::SubMul) and
/// [`SubMulAssign`](malachite_base::num::arithmetic::traits::SubMulAssign), traits for subtracting
/// the product of two [`Float`](super::Float)s — or of a [`Float`](super::Float) and a
/// [`Rational`](malachite_q::Rational) — from another [`Float`](super::Float) with a single
/// rounding (fused multiply-subtract), and the associated precision- and rounding-mode-aware
/// functions.
pub mod sub_mul;
/// Correctly-rounded summation of any number of [`Float`](super::Float)s: functions computing a
/// slice's sum with a single rounding at the end, and the [`Sum`](core::iter::Sum) implementations
/// built on them.
pub mod sum;
