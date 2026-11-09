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
    AddMulShlAssign, Average, AverageAssign, AverageRound, AverageRoundAssign, CanonicalUnitIPow,
    CanonicalizeGaussianUnit, CanonicalizeGaussianUnitAssign, CanonicalizeUnit,
    CanonicalizeUnitAssign, CeilingDivAssignMod, CeilingDivMod, CeilingRoot, CeilingRootAssign,
    CeilingSqrt, CeilingSqrtAssign, CheckedRoot, CheckedSqrt, Conjugate, ConjugateAssign,
    DivAssignMod, DivAssignModEuclidean, DivAssignRem, DivEuclidean, DivEuclideanAssign, DivExact,
    DivExactAssign, DivMod, DivModEuclidean, DivRem, DivRound, DivRoundAssign, FloorRoot,
    FloorRootAssign, FloorSqrt, FloorSqrtAssign, IsPowerOf2, IsUnit, MulShrRound,
    MulShrRoundAssign, Parity, Pow, PowAssign, PowerOf2, RoundToMultiple, RoundToMultipleAssign,
    RoundToMultipleOfPowerOf2, RoundToMultipleOfPowerOf2Assign, ShlRound, ShlRoundAssign, ShrRound,
    ShrRoundAssign, ShrRoundWithOrdering, ShrRoundWithOrderingAssign, Sign, Square, SquareAssign,
    SubMul, SubMulAssign, SubMulShl, SubMulShlAssign, UnsignedAbs,
};

/// Absolute value of [`Integer`](super::Integer)s, including implementations of [`UnsignedAbs`].
pub mod abs;
/// Implementations of [`AbsDiff`] and [`AbsDiffAssign`], traits for getting the absolute value of
/// the difference between two numbers.
pub mod abs_diff;
/// Implementations of [`AbsSquared`] and [`AbsSquaredAssign`], traits for computing the squared
/// absolute value of a number. For real types this is the same as squaring.
pub mod abs_squared;
/// Addition of [`Integer`](super::Integer)s.
pub mod add;
/// Implementations of [`AddMul`] and [`AddMulAssign`], traits for adding a number and the product
/// of two other numbers.
pub mod add_mul;
/// Implementations of [`AddMulShl`] and [`AddMulShlAssign`], traits for adding a number and the
/// product of two other numbers shifted left.
pub mod add_mul_shl;
/// [`Average`], [`AverageAssign`], [`AverageRound`], and [`AverageRoundAssign`], traits for
/// computing the average (arithmetic mean) of two numbers.
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
/// Implementations of [`Conjugate`] and [`ConjugateAssign`], traits for computing the complex
/// conjugate of a number. A real number is its own conjugate.
pub mod conjugate;
/// Division of [`Integer`](super::Integer)s.
pub mod div;
/// Implementations of [`DivEuclidean`] and [`DivEuclideanAssign`], traits for finding the quotient
/// of two numbers, rounded so that the remainder would be nonnegative.
pub mod div_euclidean;
/// Implementations of [`DivExact`] and [`DivExactAssign`], traits for dividing two numbers when
/// it's known that the division is exact.
pub mod div_exact;
/// Implementations of traits for simultaneously finding the quotient and remainder of two numbers,
/// subject to various rounding rules.
///
/// These are the traits:
///
/// | rounding     | by value or reference           | by mutable reference (assignment)      |
/// |--------------|---------------------------------|----------------------------------------|
/// | towards $-\infty$ | [`DivMod`] | [`DivAssignMod`] |
/// | towards 0         | [`DivRem`] | [`DivAssignRem`] |
/// | towards $\infty$  | [`CeilingDivMod`] | [`CeilingDivAssignMod`] |
pub mod div_mod;
/// Implementations of [`DivModEuclidean`] and [`DivAssignModEuclidean`], traits for simultaneously
/// finding the quotient and remainder of two numbers, where the remainder is nonnegative.
pub mod div_mod_euclidean;
/// Implementations of [`DivRound`] and [`DivExactAssign`](DivRoundAssign), traits for dividing two
/// numbers according to a specified [`RoundingMode`](malachite_base::rounding_modes::RoundingMode).
pub mod div_round;
/// An implementation of [`IsPowerOf2`], a trait for determining whether a number is an integer
/// power of 2.
pub mod is_power_of_2;
/// An implementation of [`IsUnit`], a trait for determining whether a number is a unit of its ring.
pub mod is_unit;
/// Multiplication of [`Integer`](super::Integer)s.
pub mod mul;
pub mod mul_add_mul;
/// Implementations of [`MulShrRound`] and [`MulShrRoundAssign`], traits for multiplying two numbers
/// and right-shifting the product with a specified rounding mode. When most of the product is
/// discarded, a short product avoids computing the rest.
pub mod mul_shr_round;
pub mod mul_sub_mul;
pub mod neg;
/// Implementations of [`Parity`], a trait for determining whether a number is even or odd.
pub mod parity;
/// Implementations of [`Pow`] and [`PowAssign`], traits for raising a number to a power.
pub mod pow;
/// Implementations of [`PowerOf2`], a trait for computing a power of 2.
pub mod power_of_2;
/// Implementations of traits for taking the $n$th root of a number.
///
/// The traits are [`FloorRoot`], [`FloorRootAssign`], [`CeilingRoot`], [`CeilingRootAssign`], and
/// [`CheckedRoot`].
pub mod root;
/// Implementations of [`RoundToMultiple`] and [`RoundToMultipleAssign`], traits for rounding a
/// number to a multiple of another number.
pub mod round_to_multiple;
/// Implementations of [`RoundToMultipleOfPowerOf2`] and [`RoundToMultipleOfPowerOf2Assign`], traits
/// for rounding a number to a multiple of a power of 2.
pub mod round_to_multiple_of_power_of_2;
/// Left-shifting an [`Integer`](super::Integer) (multiplying it by a power of 2).
///
/// # shl
/// ```
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(Integer::ZERO << 10u8, 0);
/// assert_eq!(Integer::from(123) << 2u16, 492);
/// assert_eq!(
///     (Integer::from(123) << 100u64).to_string(),
///     "155921023828072216384094494261248"
/// );
/// assert_eq!(Integer::from(-123) << 2u64, -492);
/// assert_eq!(
///     (Integer::from(-123) << 100u8).to_string(),
///     "-155921023828072216384094494261248"
/// );
/// assert_eq!(&Integer::ZERO << 10u8, 0);
/// assert_eq!(&Integer::from(123) << 2u16, 492);
/// assert_eq!(
///     (&Integer::from(123) << 100u64).to_string(),
///     "155921023828072216384094494261248"
/// );
/// assert_eq!(&Integer::from(-123) << 2u64, -492);
/// assert_eq!(
///     (&Integer::from(-123) << 100u8).to_string(),
///     "-155921023828072216384094494261248"
/// );
///
/// assert_eq!(Integer::ZERO << 10i8, 0);
/// assert_eq!(Integer::from(123) << 2i16, 492);
/// assert_eq!(
///     (Integer::from(123) << 100i32).to_string(),
///     "155921023828072216384094494261248"
/// );
/// assert_eq!(Integer::from(-123) << 2i64, -492);
/// assert_eq!(
///     (Integer::from(-123) << 100i8).to_string(),
///     "-155921023828072216384094494261248"
/// );
/// assert_eq!(Integer::ZERO << -10i16, 0);
/// assert_eq!(Integer::from(492) << -2i32, 123);
/// assert_eq!(-Integer::from(10u32).pow(12) << -10i64, -976562500);
/// assert_eq!(&Integer::ZERO << 10i8, 0);
/// assert_eq!(&Integer::from(123) << 2i16, 492);
/// assert_eq!(
///     (&Integer::from(123) << 100i32).to_string(),
///     "155921023828072216384094494261248"
/// );
/// assert_eq!(&Integer::from(-123) << 2i64, -492);
/// assert_eq!(
///     (&Integer::from(-123) << 100i8).to_string(),
///     "-155921023828072216384094494261248"
/// );
/// assert_eq!(&Integer::ZERO << -10i16, 0);
/// assert_eq!(&Integer::from(492) << -2i32, 123);
/// assert_eq!(&(-Integer::from(10u32).pow(12)) << -10i64, -976562500);
/// ```
///
/// # shl_assign
/// ```
/// use malachite_base::num::basic::traits::{NegativeOne, One};
/// use malachite_nz::integer::Integer;
///
/// let mut x = Integer::ONE;
/// x <<= 1u8;
/// x <<= 2u16;
/// x <<= 3u64;
/// x <<= 4u64;
/// assert_eq!(x, 1024);
/// let mut x = Integer::NEGATIVE_ONE;
/// x <<= 1u8;
/// x <<= 2u16;
/// x <<= 3u64;
/// x <<= 4u64;
/// assert_eq!(x, -1024);
///
/// let mut x = Integer::ONE;
/// x <<= 1i8;
/// x <<= 2i16;
/// x <<= 3i32;
/// x <<= 4i64;
/// assert_eq!(x, 1024);
/// let mut x = Integer::NEGATIVE_ONE;
/// x <<= 1i8;
/// x <<= 2i16;
/// x <<= 3i32;
/// x <<= 4i64;
/// assert_eq!(x, -1024);
///
/// let mut x = Integer::from(1024);
/// x <<= -1i8;
/// x <<= -2i16;
/// x <<= -3i32;
/// x <<= -4i64;
/// assert_eq!(x, 1);
/// ```
pub mod shl;
/// Implementations of [`ShlRound`] and [`ShlRoundAssign`], traits for multiplying a number by a
/// power of 2 and rounding according to a specified
/// [`RoundingMode`](malachite_base::rounding_modes::RoundingMode).
///
/// # shl_round
/// ```
/// use malachite_base::num::arithmetic::traits::ShlRound;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     Integer::from(0x101).shl_round(-8i8, Down).to_debug_string(),
///     "(1, Less)"
/// );
/// assert_eq!(
///     Integer::from(0x101).shl_round(-8i16, Up).to_debug_string(),
///     "(2, Greater)"
/// );
///
/// assert_eq!(
///     Integer::from(-0x101)
///         .shl_round(-9i32, Down)
///         .to_debug_string(),
///     "(0, Greater)"
/// );
/// assert_eq!(
///     Integer::from(-0x101).shl_round(-9i64, Up).to_debug_string(),
///     "(-1, Less)"
/// );
/// assert_eq!(
///     Integer::from(-0x101)
///         .shl_round(-9i8, Nearest)
///         .to_debug_string(),
///     "(-1, Less)"
/// );
/// assert_eq!(
///     Integer::from(-0xff)
///         .shl_round(-9i16, Nearest)
///         .to_debug_string(),
///     "(0, Greater)"
/// );
/// assert_eq!(
///     Integer::from(-0x100)
///         .shl_round(-9i32, Nearest)
///         .to_debug_string(),
///     "(0, Greater)"
/// );
///
/// assert_eq!(
///     Integer::from(0x100)
///         .shl_round(-8i64, Exact)
///         .to_debug_string(),
///     "(1, Equal)"
/// );
///
/// assert_eq!(
///     Integer::ZERO.shl_round(10i8, Exact).to_debug_string(),
///     "(0, Equal)"
/// );
/// assert_eq!(
///     Integer::from(123u32)
///         .shl_round(2i16, Exact)
///         .to_debug_string(),
///     "(492, Equal)"
/// );
/// assert_eq!(
///     Integer::from(123u32)
///         .shl_round(100i32, Exact)
///         .to_debug_string(),
///     "(155921023828072216384094494261248, Equal)"
/// );
///
/// assert_eq!(
///     (&Integer::from(0x101))
///         .shl_round(-8i8, Down)
///         .to_debug_string(),
///     "(1, Less)"
/// );
/// assert_eq!(
///     (&Integer::from(0x101))
///         .shl_round(-8i16, Up)
///         .to_debug_string(),
///     "(2, Greater)"
/// );
///
/// assert_eq!(
///     (&Integer::from(-0x101))
///         .shl_round(-9i32, Down)
///         .to_debug_string(),
///     "(0, Greater)"
/// );
/// assert_eq!(
///     (&Integer::from(-0x101))
///         .shl_round(-9i64, Up)
///         .to_debug_string(),
///     "(-1, Less)"
/// );
/// assert_eq!(
///     (&Integer::from(-0x101))
///         .shl_round(-9i8, Nearest)
///         .to_debug_string(),
///     "(-1, Less)"
/// );
/// assert_eq!(
///     (&Integer::from(-0xff))
///         .shl_round(-9i16, Nearest)
///         .to_debug_string(),
///     "(0, Greater)"
/// );
/// assert_eq!(
///     (&Integer::from(-0x100))
///         .shl_round(-9i32, Nearest)
///         .to_debug_string(),
///     "(0, Greater)"
/// );
///
/// assert_eq!(
///     (&Integer::from(0x100))
///         .shl_round(-8i64, Exact)
///         .to_debug_string(),
///     "(1, Equal)"
/// );
///
/// assert_eq!(
///     (&Integer::ZERO).shl_round(10i8, Exact).to_debug_string(),
///     "(0, Equal)"
/// );
/// assert_eq!(
///     (&Integer::from(123u32))
///         .shl_round(2i16, Exact)
///         .to_debug_string(),
///     "(492, Equal)"
/// );
/// assert_eq!(
///     (&Integer::from(123u32))
///         .shl_round(100i32, Exact)
///         .to_debug_string(),
///     "(155921023828072216384094494261248, Equal)"
/// );
/// ```
///
/// # shl_round_assign
/// ```
/// use core::cmp::Ordering::*;
/// use malachite_base::num::arithmetic::traits::ShlRoundAssign;
/// use malachite_base::num::basic::traits::One;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::integer::Integer;
///
/// let mut n = Integer::from(0x101);
/// assert_eq!(n.shl_round_assign(-8i8, Down), Less);
/// assert_eq!(n, 1);
///
/// let mut n = Integer::from(0x101);
/// assert_eq!(n.shl_round_assign(-8i16, Up), Greater);
/// assert_eq!(n, 2);
///
/// let mut n = Integer::from(-0x101);
/// assert_eq!(n.shl_round_assign(-9i32, Down), Greater);
/// assert_eq!(n, 0);
///
/// let mut n = Integer::from(-0x101);
/// assert_eq!(n.shl_round_assign(-9i64, Up), Less);
/// assert_eq!(n, -1);
///
/// let mut n = Integer::from(-0x101);
/// assert_eq!(n.shl_round_assign(-9i8, Nearest), Less);
/// assert_eq!(n, -1);
///
/// let mut n = Integer::from(-0xff);
/// assert_eq!(n.shl_round_assign(-9i16, Nearest), Greater);
/// assert_eq!(n, 0);
///
/// let mut n = Integer::from(-0x100);
/// assert_eq!(n.shl_round_assign(-9i32, Nearest), Greater);
/// assert_eq!(n, 0);
///
/// let mut n = Integer::from(0x100);
/// assert_eq!(n.shl_round_assign(-8i64, Exact), Equal);
/// assert_eq!(n, 1);
///
/// let mut x = Integer::ONE;
/// assert_eq!(x.shl_round_assign(1i8, Exact), Equal);
/// assert_eq!(x.shl_round_assign(2i16, Exact), Equal);
/// assert_eq!(x.shl_round_assign(3i32, Exact), Equal);
/// assert_eq!(x.shl_round_assign(4i64, Exact), Equal);
/// assert_eq!(x, 1024);
/// ```
pub mod shl_round;
/// Right-shifting an [`Integer`](super::Integer) (dividing it by a power of 2).
///
/// # shr
/// ```
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(Integer::ZERO >> 10u8, 0);
/// assert_eq!(Integer::from(492) >> 2u16, 123);
/// assert_eq!(-Integer::from(10u32).pow(12) >> 10u64, -976562500);
/// assert_eq!(&Integer::ZERO >> 10u8, 0);
/// assert_eq!(&Integer::from(492) >> 2u16, 123);
/// assert_eq!(&-Integer::from(10u32).pow(12) >> 10u64, -976562500);
///
/// assert_eq!(Integer::ZERO >> 10i8, 0);
/// assert_eq!(Integer::from(492) >> 2i16, 123);
/// assert_eq!(-Integer::from(10u32).pow(12) >> 10i64, -976562500);
/// assert_eq!(Integer::ZERO >> -10i8, 0);
/// assert_eq!(Integer::from(123) >> -2i16, 492);
/// assert_eq!(
///     (Integer::from(123) >> -100i32).to_string(),
///     "155921023828072216384094494261248"
/// );
/// assert_eq!(Integer::from(-123) >> -2i64, -492);
/// assert_eq!(
///     (Integer::from(-123) >> -100i8).to_string(),
///     "-155921023828072216384094494261248"
/// );
/// assert_eq!(&Integer::ZERO >> 10i8, 0);
/// assert_eq!(&Integer::from(492) >> 2i16, 123);
/// assert_eq!(&-Integer::from(10u32).pow(12) >> 10i64, -976562500);
/// assert_eq!(&Integer::ZERO >> -10i8, 0);
/// assert_eq!(&Integer::from(123) >> -2i16, 492);
/// assert_eq!(
///     (&Integer::from(123) >> -100i32).to_string(),
///     "155921023828072216384094494261248"
/// );
/// assert_eq!(&Integer::from(-123) >> -2i64, -492);
/// assert_eq!(
///     (&Integer::from(-123) >> -100i8).to_string(),
///     "-155921023828072216384094494261248"
/// );
/// ```
///
/// # shr_assign
/// ```
/// use malachite_base::num::basic::traits::{NegativeOne, One};
/// use malachite_nz::integer::Integer;
///
/// let mut x = Integer::from(1024);
/// x >>= 1u8;
/// x >>= 2u16;
/// x >>= 3u64;
/// x >>= 4u64;
/// assert_eq!(x, 1);
///
/// let mut x = Integer::from(1024);
/// x >>= 1i8;
/// x >>= 2i16;
/// x >>= 3i32;
/// x >>= 4i64;
/// assert_eq!(x, 1);
///
/// let mut x = Integer::ONE;
/// x >>= -1i8;
/// x >>= -2i16;
/// x >>= -3i32;
/// x >>= -4i64;
/// assert_eq!(x, 1024);
///
/// let mut x = Integer::NEGATIVE_ONE;
/// x >>= -1i8;
/// x >>= -2i16;
/// x >>= -3i32;
/// x >>= -4i64;
/// assert_eq!(x, -1024);
/// ```
pub mod shr;
/// Implementations of [`ShrRound`] and [`ShrRoundAssign`], traits for dividing a number by a power
/// of 2 and rounding according to a specified
/// [`RoundingMode`](malachite_base::rounding_modes::RoundingMode).
///
/// # shr_round
/// ```
/// use malachite_base::num::arithmetic::traits::ShrRound;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     Integer::from(0x101).shr_round(8u8, Down).to_debug_string(),
///     "(1, Less)"
/// );
/// assert_eq!(
///     Integer::from(0x101).shr_round(8u16, Up).to_debug_string(),
///     "(2, Greater)"
/// );
///
/// assert_eq!(
///     Integer::from(-0x101)
///         .shr_round(9u32, Down)
///         .to_debug_string(),
///     "(0, Greater)"
/// );
/// assert_eq!(
///     Integer::from(-0x101).shr_round(9u64, Up).to_debug_string(),
///     "(-1, Less)"
/// );
/// assert_eq!(
///     Integer::from(-0x101)
///         .shr_round(9u8, Nearest)
///         .to_debug_string(),
///     "(-1, Less)"
/// );
/// assert_eq!(
///     Integer::from(-0xff)
///         .shr_round(9u16, Nearest)
///         .to_debug_string(),
///     "(0, Greater)"
/// );
/// assert_eq!(
///     Integer::from(-0x100)
///         .shr_round(9u64, Nearest)
///         .to_debug_string(),
///     "(0, Greater)"
/// );
///
/// assert_eq!(
///     Integer::from(0x100u32)
///         .shr_round(8u32, Exact)
///         .to_debug_string(),
///     "(1, Equal)"
/// );
///
/// assert_eq!(
///     (&Integer::from(0x101))
///         .shr_round(8u8, Down)
///         .to_debug_string(),
///     "(1, Less)"
/// );
/// assert_eq!(
///     (&Integer::from(0x101))
///         .shr_round(8u16, Up)
///         .to_debug_string(),
///     "(2, Greater)"
/// );
///
/// assert_eq!(
///     (&Integer::from(-0x101))
///         .shr_round(9u32, Down)
///         .to_debug_string(),
///     "(0, Greater)"
/// );
/// assert_eq!(
///     (&Integer::from(-0x101))
///         .shr_round(9u64, Up)
///         .to_debug_string(),
///     "(-1, Less)"
/// );
/// assert_eq!(
///     (&Integer::from(-0x101))
///         .shr_round(9u8, Nearest)
///         .to_debug_string(),
///     "(-1, Less)"
/// );
/// assert_eq!(
///     (&Integer::from(-0xff))
///         .shr_round(9u16, Nearest)
///         .to_debug_string(),
///     "(0, Greater)"
/// );
/// assert_eq!(
///     (&Integer::from(-0x100))
///         .shr_round(9u64, Nearest)
///         .to_debug_string(),
///     "(0, Greater)"
/// );
///
/// assert_eq!(
///     (&Integer::from(0x100u32))
///         .shr_round(8u32, Exact)
///         .to_debug_string(),
///     "(1, Equal)"
/// );
///
/// assert_eq!(
///     Integer::from(0x101u32)
///         .shr_round(8i8, Down)
///         .to_debug_string(),
///     "(1, Less)"
/// );
/// assert_eq!(
///     Integer::from(0x101u32)
///         .shr_round(8i16, Up)
///         .to_debug_string(),
///     "(2, Greater)"
/// );
///
/// assert_eq!(
///     Integer::from(-0x101)
///         .shr_round(9i32, Down)
///         .to_debug_string(),
///     "(0, Greater)"
/// );
/// assert_eq!(
///     Integer::from(-0x101).shr_round(9i64, Up).to_debug_string(),
///     "(-1, Less)"
/// );
/// assert_eq!(
///     Integer::from(-0x101)
///         .shr_round(9i8, Nearest)
///         .to_debug_string(),
///     "(-1, Less)"
/// );
/// assert_eq!(
///     Integer::from(-0xff)
///         .shr_round(9i16, Nearest)
///         .to_debug_string(),
///     "(0, Greater)"
/// );
/// assert_eq!(
///     Integer::from(-0x100)
///         .shr_round(9i32, Nearest)
///         .to_debug_string(),
///     "(0, Greater)"
/// );
///
/// assert_eq!(
///     Integer::from(0x100u32)
///         .shr_round(8i64, Exact)
///         .to_debug_string(),
///     "(1, Equal)"
/// );
///
/// assert_eq!(
///     Integer::ZERO.shr_round(-10i8, Exact).to_debug_string(),
///     "(0, Equal)"
/// );
/// assert_eq!(
///     Integer::from(123u32)
///         .shr_round(-2i16, Exact)
///         .to_debug_string(),
///     "(492, Equal)"
/// );
/// assert_eq!(
///     Integer::from(123u32)
///         .shr_round(-100i32, Exact)
///         .to_debug_string(),
///     "(155921023828072216384094494261248, Equal)"
/// );
///
/// assert_eq!(
///     (&Integer::from(0x101u32))
///         .shr_round(8i8, Down)
///         .to_debug_string(),
///     "(1, Less)"
/// );
/// assert_eq!(
///     (&Integer::from(0x101u32))
///         .shr_round(8i16, Up)
///         .to_debug_string(),
///     "(2, Greater)"
/// );
///
/// assert_eq!(
///     (&Integer::from(-0x101))
///         .shr_round(9i32, Down)
///         .to_debug_string(),
///     "(0, Greater)"
/// );
/// assert_eq!(
///     (&Integer::from(-0x101))
///         .shr_round(9i64, Up)
///         .to_debug_string(),
///     "(-1, Less)"
/// );
/// assert_eq!(
///     (&Integer::from(-0x101))
///         .shr_round(9i8, Nearest)
///         .to_debug_string(),
///     "(-1, Less)"
/// );
/// assert_eq!(
///     (&Integer::from(-0xff))
///         .shr_round(9i16, Nearest)
///         .to_debug_string(),
///     "(0, Greater)"
/// );
/// assert_eq!(
///     (&Integer::from(-0x100))
///         .shr_round(9i32, Nearest)
///         .to_debug_string(),
///     "(0, Greater)"
/// );
///
/// assert_eq!(
///     (&Integer::from(0x100u32))
///         .shr_round(8i64, Exact)
///         .to_debug_string(),
///     "(1, Equal)"
/// );
///
/// assert_eq!(
///     (&Integer::ZERO).shr_round(-10i8, Exact).to_debug_string(),
///     "(0, Equal)"
/// );
/// assert_eq!(
///     (&Integer::from(123u32))
///         .shr_round(-2i16, Exact)
///         .to_debug_string(),
///     "(492, Equal)"
/// );
/// assert_eq!(
///     (&Integer::from(123u32))
///         .shr_round(-100i32, Exact)
///         .to_debug_string(),
///     "(155921023828072216384094494261248, Equal)"
/// );
/// ```
///
/// # shr_round_assign
/// ```
/// use core::cmp::Ordering::*;
/// use malachite_base::num::arithmetic::traits::ShrRoundAssign;
/// use malachite_base::num::basic::traits::One;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::integer::Integer;
///
/// let mut n = Integer::from(0x101);
/// assert_eq!(n.shr_round_assign(8u8, Down), Less);
/// assert_eq!(n, 1);
///
/// let mut n = Integer::from(0x101);
/// assert_eq!(n.shr_round_assign(8u16, Up), Greater);
/// assert_eq!(n, 2);
///
/// let mut n = Integer::from(-0x101);
/// assert_eq!(n.shr_round_assign(9u32, Down), Greater);
/// assert_eq!(n, 0);
///
/// let mut n = Integer::from(-0x101);
/// assert_eq!(n.shr_round_assign(9u64, Up), Less);
/// assert_eq!(n, -1);
///
/// let mut n = Integer::from(-0x101);
/// assert_eq!(n.shr_round_assign(9u8, Nearest), Less);
/// assert_eq!(n, -1);
///
/// let mut n = Integer::from(-0xff);
/// assert_eq!(n.shr_round_assign(9u16, Nearest), Greater);
/// assert_eq!(n, 0);
///
/// let mut n = Integer::from(-0x100);
/// assert_eq!(n.shr_round_assign(9u32, Nearest), Greater);
/// assert_eq!(n, 0);
///
/// let mut n = Integer::from(0x100);
/// assert_eq!(n.shr_round_assign(8u64, Exact), Equal);
/// assert_eq!(n, 1);
///
/// let mut n = Integer::from(0x101u32);
/// assert_eq!(n.shr_round_assign(8i8, Down), Less);
/// assert_eq!(n, 1);
///
/// let mut n = Integer::from(0x101u32);
/// assert_eq!(n.shr_round_assign(8i16, Up), Greater);
/// assert_eq!(n, 2);
///
/// let mut n = Integer::from(-0x101);
/// assert_eq!(n.shr_round_assign(9i32, Down), Greater);
/// assert_eq!(n, 0);
///
/// let mut n = Integer::from(-0x101);
/// assert_eq!(n.shr_round_assign(9i64, Up), Less);
/// assert_eq!(n, -1);
///
/// let mut n = Integer::from(-0x101);
/// assert_eq!(n.shr_round_assign(9i8, Nearest), Less);
/// assert_eq!(n, -1);
///
/// let mut n = Integer::from(-0xff);
/// assert_eq!(n.shr_round_assign(9i16, Nearest), Greater);
/// assert_eq!(n, 0);
///
/// let mut n = Integer::from(-0x100);
/// assert_eq!(n.shr_round_assign(9i32, Nearest), Greater);
/// assert_eq!(n, 0);
///
/// let mut n = Integer::from(0x100u32);
/// assert_eq!(n.shr_round_assign(8i64, Exact), Equal);
/// assert_eq!(n, 1);
///
/// let mut x = Integer::ONE;
/// assert_eq!(x.shr_round_assign(-1i8, Exact), Equal);
/// assert_eq!(x.shr_round_assign(-2i16, Exact), Equal);
/// assert_eq!(x.shr_round_assign(-3i32, Exact), Equal);
/// assert_eq!(x.shr_round_assign(-4i64, Exact), Equal);
/// assert_eq!(x, 1024);
/// ```
pub mod shr_round;
/// Implementations of [`ShrRoundWithOrdering`] and [`ShrRoundWithOrderingAssign`], traits for
/// right-shifting a number that is itself an approximation of an exact value, whose
/// [`Ordering`](core::cmp::Ordering) against the exact value is known, so that a value can be
/// rounded a second time without double-rounding error.
pub mod shr_round_with_ordering;
/// Implementations of [`Sign`], a trait for determining the sign of a number.
pub mod sign;
/// Implementations of traits for taking the square root of a number.
///
/// The traits are [`FloorSqrt`], [`FloorSqrtAssign`], [`CeilingSqrt`], [`CeilingSqrtAssign`], and
/// [`CheckedSqrt`].
pub mod sqrt;
/// Implementations of [`Square`] and [`SquareAssign`], traits for squaring a number.
pub mod square;
/// Subtraction of [`Integer`](super::Integer)s.
pub mod sub;
/// Implementations of [`SubMul`] and [`SubMulAssign`], traits for subtracting the product of two
/// numbers from a number.
pub mod sub_mul;
/// Implementations of [`SubMulShl`] and [`SubMulShlAssign`], traits for subtracting the product of
/// two numbers, shifted left, from a number.
pub mod sub_mul_shl;
