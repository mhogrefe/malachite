// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Absolute value of [`Rational`](super::Rational)s.
pub mod abs;
/// Implementations of [`AbsDiff`](malachite_base::num::arithmetic::traits::AbsDiff) and
/// [`AbsDiffAssign`](malachite_base::num::arithmetic::traits::AbsDiffAssign), traits for getting
/// the absolute value of the difference between two numbers.
pub mod abs_diff;
/// Implementations of [`AbsSquared`](malachite_base::num::arithmetic::traits::AbsSquared) and
/// [`AbsSquaredAssign`](malachite_base::num::arithmetic::traits::AbsSquaredAssign), traits for
/// computing the squared absolute value of a number. For real types this is the same as squaring.
pub mod abs_squared;
/// Addition of [`Rational`](super::Rational)s.
pub mod add;
/// Implementations of [`AddMul`](malachite_base::num::arithmetic::traits::AddMul) and
/// [`AddMulAssign`](malachite_base::num::arithmetic::traits::AddMulAssign), traits for adding a
/// number and the product of two other numbers.
pub mod add_mul;
/// Implementations of [`Approximate`](traits::Approximate) and
/// [`ApproximateAssign`](traits::ApproximateAssign), traits for approximating a
/// [`Rational`](super::Rational) by a [`Rational`](super::Rational) with a bounded denominator.
pub mod approximate;
/// [`Average`](malachite_base::num::arithmetic::traits::Average) and
/// [`AverageAssign`](malachite_base::num::arithmetic::traits::AverageAssign), traits for computing
/// the average (arithmetic mean) of two numbers.
pub mod average;
/// An implementation of
/// [`CanonicalUnitIPow`](malachite_base::num::arithmetic::traits::CanonicalUnitIPow), a trait for
/// finding the power of $i$ that brings a number into canonical unit form.
pub mod canonical_unit_i_pow;
/// Implementations of
/// [`CanonicalizeUnit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit) and
/// [`CanonicalizeUnitAssign`](malachite_base::num::arithmetic::traits::CanonicalizeUnitAssign),
/// traits for bringing a number into canonical unit form.
pub mod canonicalize_unit;
/// Implementations of [`Ceiling`](malachite_base::num::arithmetic::traits::Ceiling) and
/// [`CeilingAssign`](malachite_base::num::arithmetic::traits::CeilingAssign), traits for taking the
/// ceiling of a number.
pub mod ceiling;
/// The shared continued-fraction and half-gcd machinery, adopted from FLINT.
#[cfg(feature = "test_build")]
pub mod cfrac_helpers;
/// The shared continued-fraction and half-gcd machinery, adopted from FLINT.
#[cfg(not(feature = "test_build"))]
pub(crate) mod cfrac_helpers;
/// Implementations of [`Conjugate`](malachite_base::num::arithmetic::traits::Conjugate) and
/// [`ConjugateAssign`](malachite_base::num::arithmetic::traits::ConjugateAssign), traits for
/// computing the complex conjugate of a number. A real number is its own conjugate.
pub mod conjugate;
/// A function for computing Dedekind sums.
pub mod dedekind_sum;
/// Getting all denominators of [`Rational`](super::Rational)s that appear in a given closed
/// interval.
pub mod denominators_in_closed_interval;
/// Division of [`Rational`](super::Rational)s.
pub mod div;
/// Implementation of
/// [`ExpressAsPower`](malachite_base::num::factorization::traits::ExpressAsPower), a trait for
/// expressing a number as a perfect power.
pub mod express_as_power;
/// Implementations of [`ExtendedGcd`](malachite_base::num::arithmetic::traits::ExtendedGcd), a
/// trait for computing the GCD of two numbers along with Bézout cofactors.
pub mod extended_gcd;
/// Finding the neighbors of a [`Rational`](super::Rational) in a Farey sequence.
pub mod farey_neighbors;
/// Implementations of [`Floor`](malachite_base::num::arithmetic::traits::Floor) and
/// [`FloorAssign`](malachite_base::num::arithmetic::traits::FloorAssign), traits for taking the
/// floor of a number.
pub mod floor;
/// Implementations of [`Gcd`](malachite_base::num::arithmetic::traits::Gcd) and
/// [`GcdAssign`](malachite_base::num::arithmetic::traits::GcdAssign), traits for computing the GCD
/// (greatest common divisor) of two numbers.
pub mod gcd;
/// A function for computing harmonic numbers.
pub mod harmonic_number;
/// An implementation of the height of a [`Rational`](crate::Rational): the larger of the absolute
/// value of its numerator and its denominator.
pub mod height;
/// An implementation of [`IsPowerOf2`](malachite_base::num::arithmetic::traits::IsPowerOf2), a
/// trait for determining whether a number is an integer power of 2.
pub mod is_power_of_2;
/// An implementation of [`IsUnit`](malachite_base::num::arithmetic::traits::IsUnit), a trait for
/// determining whether a number is a unit of its ring.
pub mod is_unit;
/// Implementations of traits for taking the base-$b$ logarithm of a number.
///
/// The traits are [`FloorLogBase`](malachite_base::num::arithmetic::traits::FloorLogBase),
/// [`CeilingLogBase`](malachite_base::num::arithmetic::traits::CeilingLogBase), and
/// [`CheckedLogBase`](malachite_base::num::arithmetic::traits::CheckedLogBase).
pub mod log_base;
/// Implementations of traits for taking the base-2 logarithm of a number.
///
/// The traits are [`FloorLogBase2`](malachite_base::num::arithmetic::traits::FloorLogBase2),
/// [`CeilingLogBase2`](malachite_base::num::arithmetic::traits::CeilingLogBase2), and
/// [`CheckedLogBase2`](malachite_base::num::arithmetic::traits::CheckedLogBase2).
pub mod log_base_2;
/// Implementations of traits for taking the base-$2^k$ logarithm of a number.
///
/// The traits are
/// [`FloorLogBasePowerOf2`](malachite_base::num::arithmetic::traits::FloorLogBasePowerOf2),
/// [`CeilingLogBasePowerOf2`](malachite_base::num::arithmetic::traits::CeilingLogBasePowerOf2), and
/// [`CheckedLogBasePowerOf2`](malachite_base::num::arithmetic::traits::CheckedLogBasePowerOf2).
pub mod log_base_power_of_2;
/// Implementations of traits for finding the remainder of two numbers, subject to various rounding
/// rules.
///
/// These are the traits:
///
/// | rounding          | by value or reference      | by mutable reference (assignment)      |
/// |-------------------|----------------------------|----------------------------------------|
/// | towards $-\infty$ | [`Mod`](malachite_base::num::arithmetic::traits::Mod) | [`ModAssign`](malachite_base::num::arithmetic::traits::ModAssign)       |
/// | towards $\infty$ | [`CeilingMod`](malachite_base::num::arithmetic::traits::CeilingMod) | [`CeilingModAssign`](malachite_base::num::arithmetic::traits::CeilingModAssign) |
///
/// The [`Rem`](core::ops::Rem) trait in the standard library rounds towards 0.
#[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]
pub mod mod_op;
/// Multiplication of [`Rational`](super::Rational)s.
pub mod mul;
/// Implementations of [`MulAddMul`](malachite_base::num::arithmetic::traits::MulAddMul) and
/// [`MulAddMulAssign`](malachite_base::num::arithmetic::traits::MulAddMulAssign), traits for adding
/// the products of two pairs of numbers.
pub mod mul_add_mul;
/// Implementations of [`MulSubMul`](malachite_base::num::arithmetic::traits::MulSubMul) and
/// [`MulSubMulAssign`](malachite_base::num::arithmetic::traits::MulSubMulAssign), traits for
/// subtracting the product of one pair of numbers from the product of another.
pub mod mul_sub_mul;
/// Negation of [`Rational`](super::Rational)s.
pub mod neg;
/// Implementations of [`NextPowerOf2`](malachite_base::num::arithmetic::traits::NextPowerOf2) and
/// [`NextPowerOf2Assign`](malachite_base::num::arithmetic::traits::NextPowerOf2Assign), traits for
/// getting the next-highest power of 2.
pub mod next_power_of_2;
/// Implementations of [`Pow`](malachite_base::num::arithmetic::traits::Pow) and
/// [`PowAssign`](malachite_base::num::arithmetic::traits::PowAssign), traits for raising a number
/// to a power.
pub mod pow;
/// Implementations of [`PowerOf2`](malachite_base::num::arithmetic::traits::PowerOf2), a trait for
/// computing a power of 2.
pub mod power_of_2;
/// Implementations of [`Reciprocal`](malachite_base::num::arithmetic::traits::Reciprocal) and
/// [`ReciprocalAssign`](malachite_base::num::arithmetic::traits::ReciprocalAssign), traits for
/// computing the reciprocal of a number.
pub mod reciprocal;
/// Rational reconstruction: recovering a [`Rational`](crate::Rational) from its residue modulo a
/// [`Natural`](malachite_nz::natural::Natural).
pub mod reconstruct;
/// Implementations of [`CheckedRoot`](malachite_base::num::arithmetic::traits::CheckedRoot), a
/// trait for computing the root of a number, if the number is a perfect power.
pub mod root;
/// Implementations of [`RoundToMultiple`](malachite_base::num::arithmetic::traits::RoundToMultiple)
/// and [`RoundToMultipleAssign`](malachite_base::num::arithmetic::traits::RoundToMultipleAssign),
/// traits for rounding a number to a multiple of another number.
pub mod round_to_multiple;
/// Implementations of
/// [`RoundToMultipleOfPowerOf2`](malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2)
/// and
/// [`RoundToMultipleOfPowerOf2Assign`](malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2Assign),
/// traits for rounding a number to a multiple of a power of 2.
#[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]
pub mod round_to_multiple_of_power_of_2;
/// Left-shifting a [`Rational`](super::Rational) (multiplying it by a power of 2).
///
/// # shl
/// ```
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_q::Rational;
///
/// assert_eq!(Rational::ZERO << 10u8, 0);
/// assert_eq!(Rational::from(123) << 2u16, 492);
/// assert_eq!((Rational::from_signeds(7, 22) << 2u16).to_string(), "14/11");
///
/// assert_eq!(Rational::ZERO << 10i8, 0);
/// assert_eq!(Rational::from(123) << 2i16, 492);
/// assert_eq!((Rational::from(123) << -2i16).to_string(), "123/4");
/// assert_eq!((Rational::from_signeds(7, 22) << 2i16).to_string(), "14/11");
/// assert_eq!(
///     (Rational::from_signeds(22, 7) << -2i16).to_string(),
///     "11/14"
/// );
///
/// assert_eq!(&Rational::ZERO << 10u8, 0);
/// assert_eq!(&Rational::from(123) << 2u16, 492);
/// assert_eq!(
///     (&Rational::from_signeds(7, 22) << 2u16).to_string(),
///     "14/11"
/// );
///
/// assert_eq!(&Rational::ZERO << 10i8, 0);
/// assert_eq!(&Rational::from(123) << 2i16, 492);
/// assert_eq!((&Rational::from(123) << -2i16).to_string(), "123/4");
/// assert_eq!(
///     (&Rational::from_signeds(7, 22) << 2i16).to_string(),
///     "14/11"
/// );
/// assert_eq!(
///     (&Rational::from_signeds(22, 7) << -2i16).to_string(),
///     "11/14"
/// );
/// ```
///
/// # shl_assign
/// ```
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_q::Rational;
///
/// let mut x = Rational::ZERO;
/// x <<= 10u8;
/// assert_eq!(x, 0);
///
/// let mut x = Rational::from(123);
/// x <<= 2u16;
/// assert_eq!(x, 492);
///
/// let mut x = Rational::from_signeds(7, 22);
/// x <<= 2u16;
/// assert_eq!(x.to_string(), "14/11");
///
/// let mut x = Rational::ZERO;
/// x <<= 10i8;
/// assert_eq!(x, 0);
///
/// let mut x = Rational::from(123);
/// x <<= 2i16;
/// assert_eq!(x, 492);
///
/// let mut x = Rational::from(123);
/// x <<= -2i16;
/// assert_eq!(x.to_string(), "123/4");
///
/// let mut x = Rational::from_signeds(7, 22);
/// x <<= 2i16;
/// assert_eq!(x.to_string(), "14/11");
///
/// let mut x = Rational::from_signeds(22, 7);
/// x <<= -2i16;
/// assert_eq!(x.to_string(), "11/14");
/// ```
pub mod shl;
/// Right-shifting a [`Rational`](super::Rational) (dividing it by a power of 2).
///
/// # shr
/// ```
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_q::Rational;
///
/// assert_eq!(Rational::ZERO >> 10u8, 0);
/// assert_eq!((Rational::from(123) >> 2u16).to_string(), "123/4");
/// assert_eq!((Rational::from_signeds(22, 7) >> 2u16).to_string(), "11/14");
///
/// assert_eq!(Rational::ZERO >> 10i8, 0);
/// assert_eq!((Rational::from(123) >> 2i16).to_string(), "123/4");
/// assert_eq!(Rational::from(123) >> -2i16, 492);
/// assert_eq!((Rational::from_signeds(22, 7) >> 2i16).to_string(), "11/14");
/// assert_eq!(
///     (Rational::from_signeds(7, 22) >> -2i16).to_string(),
///     "14/11"
/// );
///
/// assert_eq!(&Rational::ZERO >> 10u8, 0);
/// assert_eq!((&Rational::from(123) >> 2u16).to_string(), "123/4");
/// assert_eq!(
///     (&Rational::from_signeds(22, 7) >> 2u16).to_string(),
///     "11/14"
/// );
///
/// assert_eq!(&Rational::ZERO >> 10i8, 0);
/// assert_eq!((&Rational::from(123) >> 2i16).to_string(), "123/4");
/// assert_eq!(&Rational::from(123) >> -2i16, 492);
/// assert_eq!(
///     (&Rational::from_signeds(22, 7) >> 2i16).to_string(),
///     "11/14"
/// );
/// assert_eq!(
///     (&Rational::from_signeds(7, 22) >> -2i16).to_string(),
///     "14/11"
/// );
/// ```
///
/// # shr_assign
/// ```
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_q::Rational;
///
/// let mut x = Rational::ZERO;
/// x >>= 10u8;
/// assert_eq!(x, 0);
///
/// let mut x = Rational::from(123);
/// x >>= 2u16;
/// assert_eq!(x.to_string(), "123/4");
///
/// let mut x = Rational::from_signeds(22, 7);
/// x >>= 2u16;
/// assert_eq!(x.to_string(), "11/14");
///
/// let mut x = Rational::ZERO;
/// x >>= 10i8;
/// assert_eq!(x, 0);
///
/// let mut x = Rational::from(123);
/// x >>= 2i16;
/// assert_eq!(x.to_string(), "123/4");
///
/// let mut x = Rational::from(123);
/// x >>= -2i16;
/// assert_eq!(x, 492);
///
/// let mut x = Rational::from_signeds(22, 7);
/// x >>= 2i16;
/// assert_eq!(x.to_string(), "11/14");
///
/// let mut x = Rational::from_signeds(7, 22);
/// x >>= -2i16;
/// assert_eq!(x.to_string(), "14/11");
/// ```
pub mod shr;
/// An implementation of [`Sign`](malachite_base::num::arithmetic::traits::Sign), a trait for
/// determining the sign of a number.
pub mod sign;
/// Functions for finding the simplest (lowest-denominator) [`Rational`](super::Rational) in an
/// interval.
pub mod simplest_rational_in_interval;
/// Implementations of [`CheckedSqrt`](malachite_base::num::arithmetic::traits::CheckedSqrt), a
/// trait for computing the root of a number, if the number is a perfect square.
pub mod sqrt;
/// Implementations of [`Square`](malachite_base::num::arithmetic::traits::Square) and
/// [`SquareAssign`](malachite_base::num::arithmetic::traits::SquareAssign), traits for squaring a
/// number.
pub mod square;
/// Subtraction of [`Rational`](super::Rational)s.
pub mod sub;
/// Implementations of [`SubMul`](malachite_base::num::arithmetic::traits::SubMul) and
/// [`SubMulAssign`](malachite_base::num::arithmetic::traits::SubMulAssign), traits for subtracting
/// the product of two numbers from a number.
pub mod sub_mul;
/// Various traits for performing arithmetic operations on numbers.
pub mod traits;
