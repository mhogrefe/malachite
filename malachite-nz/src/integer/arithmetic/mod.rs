// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Absolute value of [`Integer`](super::Integer)s, including implementations of
/// [`UnsignedAbs`](malachite_base::num::arithmetic::traits::UnsignedAbs).
pub mod abs;
/// Addition of [`Integer`](super::Integer)s.
pub mod add;
/// Implementations of [`AddMul`](malachite_base::num::arithmetic::traits::AddMul) and
/// [`AddMulAssign`](malachite_base::num::arithmetic::traits::AddMulAssign), traits for adding a
/// number and the product of two other numbers.
pub mod add_mul;
/// Implementations of
/// [`BinomialCoefficient`](malachite_base::num::arithmetic::traits::BinomialCoefficient), a trait
/// for computing the binomial coefficient of two numbers.
pub mod binomial_coefficient;
/// Division of [`Integer`](super::Integer)s.
pub mod div;
/// Implementations of [`DivExact`](malachite_base::num::arithmetic::traits::DivExact) and
/// [`DivExactAssign`](malachite_base::num::arithmetic::traits::DivExactAssign), traits for dividing
/// two numbers when it's known that the division is exact.
pub mod div_exact;
/// Implementations of raits for simultaneously finding the quotient and remainder of two numbers,
/// subject to various rounding rules.
///
/// These are the traits:
///
/// | rounding     | by value or reference           | by mutable reference (assignment)      |
/// |--------------|---------------------------------|----------------------------------------|
/// | towards $-\infty$ | [`DivMod`](malachite_base::num::arithmetic::traits::DivMod) | [`DivAssignMod`](malachite_base::num::arithmetic::traits::DivAssignMod) |
/// | towards 0         | [`DivRem`](malachite_base::num::arithmetic::traits::DivRem) | [`DivAssignRem`](malachite_base::num::arithmetic::traits::DivAssignRem) |
/// | towards $\infty$  | [`CeilingDivMod`](malachite_base::num::arithmetic::traits::CeilingDivMod) | [`CeilingDivAssignMod`](malachite_base::num::arithmetic::traits::CeilingDivAssignMod) |
pub mod div_mod;
/// Implementations of [`DivRound`](malachite_base::num::arithmetic::traits::DivRound) and
/// [`DivExactAssign`](malachite_base::num::arithmetic::traits::DivRoundAssign), traits for dividing
/// two numbers according to a specified
/// [`RoundingMode`](malachite_base::rounding_modes::RoundingMode).
pub mod div_round;
/// Implementations of [`DivisibleBy`](malachite_base::num::arithmetic::traits::DivisibleBy), a
/// trait for determining whether one number is divisible by another.
pub mod divisible_by;
/// Implementations of
/// [`DivisibleByPowerOf2`](malachite_base::num::arithmetic::traits::DivisibleByPowerOf2), a trait
/// for determining whether a number is divisible by $2^k$.
pub mod divisible_by_power_of_2;
/// Implementations of [`EqMod`](malachite_base::num::arithmetic::traits::EqMod), a trait for
/// determining whether one number is equal by another modulo a third.
pub mod eq_mod;
/// Implementations of [`EqModPowerOf2`](malachite_base::num::arithmetic::traits::EqModPowerOf2), a
/// trait for determining whether one number is equal to another modulo $2^k$.
pub mod eq_mod_power_of_2;
/// Implementations of [`ExtendedGcd`](malachite_base::num::arithmetic::traits::ExtendedGcd), a
/// trait for computing the extended GCD of two numbers.
pub mod extended_gcd;
/// Implementations of [`LegendreSymbol`](malachite_base::num::arithmetic::traits::LegendreSymbol),
/// [`JacobiSymbol`](malachite_base::num::arithmetic::traits::JacobiSymbol), and
/// [`KroneckerSymbol`](malachite_base::num::arithmetic::traits::KroneckerSymbol), traits for
/// computing the Legendre, Jacobi, and Kronecker symbols of two numbers.
pub mod kronecker_symbol;
/// Implementations of traits for finding the remainder of two numbers, subject to various rounding
/// rules.
///
/// These are the traits:
///
/// | rounding          | by value or reference      | by mutable reference (assignment)      |
/// |-------------------|----------------------------|----------------------------------------|
/// | towards $-\infty$ | [`Mod`](malachite_base::num::arithmetic::traits::Mod)       | [`ModAssign`](malachite_base::num::arithmetic::traits::ModAssign)       |
/// | towards $\infty$  | [`CeilingMod`](malachite_base::num::arithmetic::traits::CeilingMod) | [`CeilingModAssign`](malachite_base::num::arithmetic::traits::CeilingModAssign) |
///
/// The [`Rem`](core::ops::Rem) trait in the standard library rounds towards 0.
pub mod mod_op;
/// Implementations of traits for finding the remainder of a number divided by $2^k$, subject to
/// various rounding rules.
///
/// These are the traits:
///
/// | rounding | by value or reference | by mutable reference (assignment) |
/// |----------|-----------------------|-----------------------------------|
/// | towards $-\infty$ | [`ModPowerOf2`](malachite_base::num::arithmetic::traits::ModPowerOf2) | [`ModPowerOf2Assign`](malachite_base::num::arithmetic::traits::ModPowerOf2Assign)       |
/// | towards 0 | [`RemPowerOf2`](malachite_base::num::arithmetic::traits::RemPowerOf2) | [`RemPowerOf2Assign`](malachite_base::num::arithmetic::traits::RemPowerOf2Assign)       |
/// | towards $\infty$  | [`CeilingModPowerOf2`](malachite_base::num::arithmetic::traits::CeilingModPowerOf2) | [`CeilingModPowerOf2Assign`](malachite_base::num::arithmetic::traits::CeilingModPowerOf2Assign) |
pub mod mod_power_of_2;
/// Multiplication of [`Integer`](super::Integer)s.
pub mod mul;
/// Negation of an [`Integer`](super::Integer).
pub mod neg;
/// Implementations of [`Parity`](malachite_base::num::arithmetic::traits::Parity), a trait for
/// determining whether a number is even or odd.
pub mod parity;
/// Implementations of [`Pow`](malachite_base::num::arithmetic::traits::Pow) and
/// [`PowAssign`](malachite_base::num::arithmetic::traits::PowAssign), traits for raising a number
/// to a power.
pub mod pow;
/// Implementations of [`PowerOf2`](malachite_base::num::arithmetic::traits::PowerOf2), a trait for
/// computing a power of 2.
pub mod power_of_2;
/// Implementations of traits for taking the $n$th root of a number.
///
/// The traits are [`FloorRoot`](malachite_base::num::arithmetic::traits::FloorRoot),
/// [`FloorRootAssign`](malachite_base::num::arithmetic::traits::FloorRootAssign),
/// [`CeilingRoot`](malachite_base::num::arithmetic::traits::CeilingRoot),
/// [`CeilingRootAssign`](malachite_base::num::arithmetic::traits::CeilingRootAssign), and
/// [`CheckedRoot`](malachite_base::num::arithmetic::traits::CheckedRoot).
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
///     (Integer::from(123) << 100u32).to_string(),
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
///     (&Integer::from(123) << 100u32).to_string(),
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
/// x <<= 3u32;
/// x <<= 4u64;
/// assert_eq!(x, 1024);
/// let mut x = Integer::NEGATIVE_ONE;
/// x <<= 1u8;
/// x <<= 2u16;
/// x <<= 3u32;
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
/// Implementations of [`ShlRound`](malachite_base::num::arithmetic::traits::ShlRound) and
/// [`ShlRoundAssign`](malachite_base::num::arithmetic::traits::ShlRoundAssign), traits for
/// multiplying a number by a power of 2 and rounding according to a specified
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
/// assert_eq!(-Integer::from(10u32).pow(12) >> 10u32, -976562500);
/// assert_eq!(&Integer::ZERO >> 10u8, 0);
/// assert_eq!(&Integer::from(492) >> 2u16, 123);
/// assert_eq!(&-Integer::from(10u32).pow(12) >> 10u32, -976562500);
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
/// x >>= 3u32;
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
/// Implementations of [`ShrRound`](malachite_base::num::arithmetic::traits::ShrRound) and
/// [`ShrRoundAssign`](malachite_base::num::arithmetic::traits::ShrRoundAssign), traits for dividing
/// a number by a power of 2 and rounding according to a specified
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
/// Implementations of [`Sign`](malachite_base::num::arithmetic::traits::Sign), a trait for
/// determining the sign of a number.
pub mod sign;
/// Implementations of traits for taking the square root of a number.
///
/// The traits are [`FloorSqrt`](malachite_base::num::arithmetic::traits::FloorSqrt),
/// [`FloorSqrtAssign`](malachite_base::num::arithmetic::traits::FloorSqrtAssign),
/// [`CeilingSqrt`](malachite_base::num::arithmetic::traits::CeilingSqrt),
/// [`CeilingSqrtAssign`](malachite_base::num::arithmetic::traits::CeilingSqrtAssign), and
/// [`CheckedSqrt`](malachite_base::num::arithmetic::traits::CheckedSqrt).
pub mod sqrt;
/// Implementations of [`Square`](malachite_base::num::arithmetic::traits::Square) and
/// [`SquareAssign`](malachite_base::num::arithmetic::traits::SquareAssign), traits for squaring a
/// number.
pub mod square;
/// Subtraction of [`Integer`](super::Integer)s.
pub mod sub;
/// Implementations of [`SubMul`](malachite_base::num::arithmetic::traits::SubMul) and
/// [`SubMulAssign`](malachite_base::num::arithmetic::traits::SubMulAssign), traits for subtracting
/// the product of two numbers from a number.
pub mod sub_mul;
