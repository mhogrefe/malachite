// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(doc)]
use malachite_base::num::arithmetic::traits::{
    AbsDiff, AbsDiffAssign, AbsSquared, AbsSquaredAssign, AddMul, AddMulAssign, AddMulShl,
    AddMulShlAssign, Average, AverageAssign, CanonicalUnitIPow, CanonicalizeGaussianUnit,
    CanonicalizeGaussianUnitAssign, CanonicalizeUnit, CanonicalizeUnitAssign, Ceiling,
    CeilingAssign, CheckedRoot, CheckedSqrt, Conjugate, ConjugateAssign, Floor, FloorAssign,
    IsPowerOf2, IsUnit, MulAddMul, MulAddMulAssign, MulSubMul, MulSubMulAssign, NextPowerOf2,
    NextPowerOf2Assign, Pow, PowAssign, PowerOf2, Reciprocal, ReciprocalAssign, RoundToMultiple,
    RoundToMultipleAssign, RoundToMultipleOfPowerOf2, RoundToMultipleOfPowerOf2Assign, Sign,
    Square, SquareAssign, SubMul, SubMulAssign, SubMulShl, SubMulShlAssign,
};

/// Absolute value of [`Rational`](super::Rational)s.
pub mod abs;
/// Implementations of [`AbsDiff`] and [`AbsDiffAssign`], traits for getting the absolute value of
/// the difference between two numbers.
pub mod abs_diff;
/// Implementations of [`AbsSquared`] and [`AbsSquaredAssign`], traits for computing the squared
/// absolute value of a number. For real types this is the same as squaring.
pub mod abs_squared;
/// Addition of [`Rational`](super::Rational)s.
pub mod add;
/// Implementations of [`AddMul`] and [`AddMulAssign`], traits for adding a number and the product
/// of two other numbers.
pub mod add_mul;
/// Implementations of [`AddMulShl`] and [`AddMulShlAssign`], traits for adding a number and the
/// product of two other numbers shifted left.
pub mod add_mul_shl;
/// [`Average`] and [`AverageAssign`], traits for computing the average (arithmetic mean) of two
/// numbers.
pub mod average;
/// An implementation of [`CanonicalUnitIPow`], a trait for finding the power of $i$ that brings a
/// number into canonical unit form.
pub mod canonical_unit_i_pow;
/// Implementations of [`CanonicalizeGaussianUnit`] and [`CanonicalizeGaussianUnitAssign`], traits
/// for bringing a number into canonical Gaussian-unit form.
pub mod canonicalize_gaussian_unit;
/// Implementations of [`CanonicalizeUnit`] and [`CanonicalizeUnitAssign`], traits for bringing a
/// number into canonical unit form.
pub mod canonicalize_unit;
/// Implementations of [`Ceiling`] and [`CeilingAssign`], traits for taking the ceiling of a number.
pub mod ceiling;
/// Implementations of [`Conjugate`] and [`ConjugateAssign`], traits for computing the complex
/// conjugate of a number. A real number is its own conjugate.
pub mod conjugate;
/// Division of [`Rational`](super::Rational)s.
pub mod div;
/// Implementations of [`Floor`] and [`FloorAssign`], traits for taking the floor of a number.
pub mod floor;
/// An implementation of the height of a [`Rational`](crate::Rational): the larger of the absolute
/// value of its numerator and its denominator.
pub mod height;
/// An implementation of [`IsPowerOf2`], a trait for determining whether a number is an integer
/// power of 2.
pub mod is_power_of_2;
/// An implementation of [`IsUnit`], a trait for determining whether a number is a unit of its ring.
pub mod is_unit;
/// Multiplication of [`Rational`](super::Rational)s.
pub mod mul;
/// Implementations of [`MulAddMul`] and [`MulAddMulAssign`], traits for adding the products of two
/// pairs of numbers.
pub mod mul_add_mul;
/// Implementations of [`MulSubMul`] and [`MulSubMulAssign`], traits for subtracting the product of
/// one pair of numbers from the product of another.
pub mod mul_sub_mul;
/// Negation of [`Rational`](super::Rational)s.
pub mod neg;
/// Implementations of [`NextPowerOf2`] and [`NextPowerOf2Assign`], traits for getting the
/// next-highest power of 2.
pub mod next_power_of_2;
/// Implementations of [`Pow`] and [`PowAssign`], traits for raising a number to a power.
pub mod pow;
/// Implementations of [`PowerOf2`], a trait for computing a power of 2.
pub mod power_of_2;
/// Implementations of [`Reciprocal`] and [`ReciprocalAssign`], traits for computing the reciprocal
/// of a number.
pub mod reciprocal;
/// Implementations of [`CheckedRoot`], a trait for computing the root of a number, if the number is
/// a perfect power.
pub mod root;
/// Implementations of [`RoundToMultiple`] and [`RoundToMultipleAssign`], traits for rounding a
/// number to a multiple of another number.
pub mod round_to_multiple;
/// Implementations of [`RoundToMultipleOfPowerOf2`] and [`RoundToMultipleOfPowerOf2Assign`], traits
/// for rounding a number to a multiple of a power of 2.
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
/// An implementation of [`Sign`], a trait for determining the sign of a number.
pub mod sign;
/// Implementations of [`CheckedSqrt`], a trait for computing the root of a number, if the number is
/// a perfect square.
pub mod sqrt;
/// Implementations of [`Square`] and [`SquareAssign`], traits for squaring a number.
pub mod square;
/// Subtraction of [`Rational`](super::Rational)s.
pub mod sub;
/// Implementations of [`SubMul`] and [`SubMulAssign`], traits for subtracting the product of two
/// numbers from a number.
pub mod sub_mul;
/// Implementations of [`SubMulShl`] and [`SubMulShlAssign`], traits for subtracting the product of
/// two numbers, shifted left, from a number.
pub mod sub_mul_shl;
/// Various traits for performing arithmetic operations on numbers.
pub mod traits;
