// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// [`Abs`](traits::Abs), [`AbsAssign`](traits::AbsAssign), and
/// [`UnsignedAbs`](traits::UnsignedAbs), traits for getting the absolute value of a number.
///
/// # abs_assign
/// ```
/// use malachite_base::num::arithmetic::traits::AbsAssign;
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
///
/// let mut x = 0i8;
/// x.abs_assign();
/// assert_eq!(x, 0i8);
///
/// let mut x = 100i64;
/// x.abs_assign();
/// assert_eq!(x, 100i64);
///
/// let mut x = -100i64;
/// x.abs_assign();
/// assert_eq!(x, 100i64);
///
/// let mut x = -0.0;
/// x.abs_assign();
/// assert_eq!(NiceFloat(x), NiceFloat(0.0));
///
/// let mut x = f64::NEGATIVE_INFINITY;
/// x.abs_assign();
/// assert_eq!(NiceFloat(x), NiceFloat(f64::INFINITY));
///
/// let mut x = 100.0;
/// x.abs_assign();
/// assert_eq!(NiceFloat(x), NiceFloat(100.0));
///
/// let mut x = -100.0;
/// x.abs_assign();
/// assert_eq!(NiceFloat(x), NiceFloat(100.0));
/// ```
pub mod abs;
/// [`AbsDiff`](traits::AbsDiff) and [`AbsDiffAssign`](traits::AbsDiffAssign), traits for getting
/// the absolute value of the difference between two numbers.
///
/// # abs_diff
/// ```
/// assert_eq!(10u8.abs_diff(20u8), 10u8);
/// assert_eq!(10i8.abs_diff(-10i8), 20u8);
/// ```
///
/// # abs_diff_assign
/// ```
/// use malachite_base::num::arithmetic::traits::AbsDiffAssign;
///
/// let mut x = 10u8;
/// x.abs_diff_assign(20u8);
/// assert_eq!(x, 10);
/// ```
pub mod abs_diff;
/// [`AddMul`](traits::AddMul) and [`AddMulAssign`](traits::AddMulAssign), traits for adding a
/// number and the product of two other numbers.
///
/// # add_mul
/// ```
/// use malachite_base::num::arithmetic::traits::AddMul;
///
/// assert_eq!(2u8.add_mul(3, 7), 23);
/// assert_eq!(127i8.add_mul(-2, 100), -73);
/// assert_eq!(1.0f32.add_mul(2.0, 3.0), 7.0);
/// ```
///
/// # add_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::AddMulAssign;
///
/// let mut x = 2u8;
/// x.add_mul_assign(3, 7);
/// assert_eq!(x, 23);
///
/// let mut x = 127i8;
/// x.add_mul_assign(-2, 100);
/// assert_eq!(x, -73);
///
/// let mut x = 1.0f32;
/// x.add_mul_assign(2.0, 3.0);
/// assert_eq!(x, 7.0);
/// ```
pub mod add_mul;
/// [`ArithmeticCheckedShl`](traits::ArithmeticCheckedShl), a trait for left-shifting a number and
/// checking whether the result is representable.
///
/// # arithmetic_checked_shl
/// ```
/// use malachite_base::num::arithmetic::traits::ArithmeticCheckedShl;
///
/// assert_eq!(3u8.arithmetic_checked_shl(6), Some(192u8));
/// assert_eq!(3u8.arithmetic_checked_shl(7), None);
/// assert_eq!(3u8.arithmetic_checked_shl(100), None);
/// assert_eq!(0u8.arithmetic_checked_shl(100), Some(0u8));
///
/// assert_eq!(3u8.arithmetic_checked_shl(6), Some(192u8));
/// assert_eq!(3u8.arithmetic_checked_shl(7), None);
/// assert_eq!(3u8.arithmetic_checked_shl(100), None);
/// assert_eq!(0u8.arithmetic_checked_shl(100), Some(0u8));
/// assert_eq!(100u8.arithmetic_checked_shl(-3), Some(12u8));
/// assert_eq!(100u8.arithmetic_checked_shl(-100), Some(0u8));
///
/// assert_eq!(3i8.arithmetic_checked_shl(5), Some(96i8));
/// assert_eq!(3i8.arithmetic_checked_shl(6), None);
/// assert_eq!((-3i8).arithmetic_checked_shl(5), Some(-96i8));
/// assert_eq!((-3i8).arithmetic_checked_shl(6), None);
/// assert_eq!(3i8.arithmetic_checked_shl(100), None);
/// assert_eq!((-3i8).arithmetic_checked_shl(100), None);
/// assert_eq!(0i8.arithmetic_checked_shl(100), Some(0i8));
///
/// assert_eq!(3i8.arithmetic_checked_shl(5), Some(96i8));
/// assert_eq!(3i8.arithmetic_checked_shl(6), None);
/// assert_eq!((-3i8).arithmetic_checked_shl(5), Some(-96i8));
/// assert_eq!((-3i8).arithmetic_checked_shl(6), None);
/// assert_eq!(3i8.arithmetic_checked_shl(100), None);
/// assert_eq!((-3i8).arithmetic_checked_shl(100), None);
/// assert_eq!(0i8.arithmetic_checked_shl(100), Some(0i8));
/// assert_eq!(100i8.arithmetic_checked_shl(-3), Some(12i8));
/// assert_eq!((-100i8).arithmetic_checked_shl(-3), Some(-13i8));
/// assert_eq!(100i8.arithmetic_checked_shl(-100), Some(0i8));
/// assert_eq!((-100i8).arithmetic_checked_shl(-100), Some(-1i8));
/// ```
pub mod arithmetic_checked_shl;
/// [`ArithmeticCheckedShr`](traits::ArithmeticCheckedShr), a trait for right-shifting a number and
/// checking whether the result is representable.
///
/// # arithmetic_checked_shr
/// ```
/// use malachite_base::num::arithmetic::traits::ArithmeticCheckedShr;
///
/// assert_eq!(100u8.arithmetic_checked_shr(3), Some(12u8));
/// assert_eq!(100u8.arithmetic_checked_shr(100), Some(0u8));
/// assert_eq!(3u8.arithmetic_checked_shr(-6), Some(192u8));
/// assert_eq!(3u8.arithmetic_checked_shr(-7), None);
/// assert_eq!(3u8.arithmetic_checked_shr(-100), None);
/// assert_eq!(0u8.arithmetic_checked_shr(-100), Some(0u8));
///
/// assert_eq!(100i8.arithmetic_checked_shr(3), Some(12i8));
/// assert_eq!((-100i8).arithmetic_checked_shr(3), Some(-13i8));
/// assert_eq!(100i8.arithmetic_checked_shr(100), Some(0i8));
/// assert_eq!((-100i8).arithmetic_checked_shr(100), Some(-1i8));
/// assert_eq!(3i8.arithmetic_checked_shr(-5), Some(96i8));
/// assert_eq!(3i8.arithmetic_checked_shr(-6), None);
/// assert_eq!((-3i8).arithmetic_checked_shr(-5), Some(-96i8));
/// assert_eq!((-3i8).arithmetic_checked_shr(-6), None);
/// assert_eq!(3i8.arithmetic_checked_shr(-100), None);
/// assert_eq!((-3i8).arithmetic_checked_shr(-100), None);
/// assert_eq!(0i8.arithmetic_checked_shr(-100), Some(0i8));
/// ```
pub mod arithmetic_checked_shr;
/// Traits for computing the binomial coefficient of two numbers. There is a trait whose
/// implementations panic if the result cannot be represented, and a checked trait whose
/// implementations return `None` in that case: [`BinomialCoefficient`](traits::BinomialCoefficient)
/// and [`CheckedBinomialCoefficient`](traits::CheckedBinomialCoefficient).
///
/// # binomial_coefficient
/// ```
/// use malachite_base::num::arithmetic::traits::BinomialCoefficient;
///
/// assert_eq!(u8::binomial_coefficient(3, 0), 1);
/// assert_eq!(u8::binomial_coefficient(3, 1), 3);
/// assert_eq!(u8::binomial_coefficient(3, 2), 3);
/// assert_eq!(u8::binomial_coefficient(3, 3), 1);
/// assert_eq!(u8::binomial_coefficient(10, 5), 252);
///
/// assert_eq!(i8::binomial_coefficient(-3, 0), 1);
/// assert_eq!(i8::binomial_coefficient(-3, 1), -3);
/// assert_eq!(i8::binomial_coefficient(-3, 2), 6);
/// assert_eq!(i8::binomial_coefficient(-3, 3), -10);
/// ```
///
/// # checked_binomial_coefficient
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedBinomialCoefficient;
///
/// assert_eq!(u8::checked_binomial_coefficient(3, 0), Some(1));
/// assert_eq!(u8::checked_binomial_coefficient(3, 1), Some(3));
/// assert_eq!(u8::checked_binomial_coefficient(3, 2), Some(3));
/// assert_eq!(u8::checked_binomial_coefficient(3, 3), Some(1));
/// assert_eq!(u8::checked_binomial_coefficient(10, 5), Some(252));
/// assert_eq!(u8::checked_binomial_coefficient(11, 5), None);
///
/// assert_eq!(i8::checked_binomial_coefficient(-3, 0), Some(1));
/// assert_eq!(i8::checked_binomial_coefficient(-3, 1), Some(-3));
/// assert_eq!(i8::checked_binomial_coefficient(-3, 2), Some(6));
/// assert_eq!(i8::checked_binomial_coefficient(-3, 3), Some(-10));
/// assert_eq!(i8::checked_binomial_coefficient(-3, -3), None);
/// assert_eq!(i8::checked_binomial_coefficient(11, 5), None);
/// ```
pub mod binomial_coefficient;
/// [`Ceiling`](traits::Ceiling) and [`CeilingAssign`](traits::CeilingAssign), traits for computing
/// the ceiling of a number.
///
/// # ceiling
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingAssign;
///
/// let mut x = 1.5f32;
/// x.ceiling_assign();
/// assert_eq!(x, 2.0);
///
/// let mut x = -1.5f32;
/// x.ceiling_assign();
/// assert_eq!(x, -1.0);
/// ```
pub mod ceiling;
/// [`CheckedAbs`](traits::CheckedAbs), a trait for computing the absolute value of number and
/// checking whether the result is representable.
pub mod checked_abs;
/// [`CheckedAdd`](traits::CheckedAdd), a trait for adding two numbers and checking whether the
/// result is representable.
pub mod checked_add;
/// [`CheckedAddMul`](traits::CheckedAddMul), a trait for adding a number and the product of two
/// other numbers, and checking whether the result is representable.
///
/// # checked_add_mul
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedAddMul;
///
/// assert_eq!(2u8.checked_add_mul(3, 7), Some(23));
/// assert_eq!(2u8.checked_add_mul(20, 20), None);
///
/// assert_eq!(127i8.checked_add_mul(-2, 100), Some(-73));
/// assert_eq!((-127i8).checked_add_mul(-2, 100), None);
/// ```
pub mod checked_add_mul;
/// [`CheckedDiv`](traits::CheckedDiv), a trait for dividing two numbers and checking whether the
/// result is representable.
pub mod checked_div;
/// [`CheckedMul`](traits::CheckedMul), a trait for multiplying two numbers and checking whether the
/// result is representable.
pub mod checked_mul;
/// [`CheckedNeg`](traits::CheckedNeg), a trait for negating a number and checking whether the
/// result is representable.
pub mod checked_neg;
/// [`CheckedNextPowerOf2`](traits::CheckedNextPowerOf2), a trait for getting the next-highest power
/// of 2, if it's representable.
pub mod checked_next_power_of_2;
/// [`CheckedPow`](traits::CheckedPow), a trait for raising a number to the power of a [`u64`] and
/// checking whether the result is representable.
pub mod checked_pow;
/// [`CheckedSquare`](traits::CheckedSquare), a trait for squaring a number and checking whether the
/// result is representable.
///
/// # checked_square
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedSquare;
///
/// assert_eq!(3u8.checked_square(), Some(9));
/// assert_eq!((-1000i32).checked_square(), Some(1000000));
/// assert_eq!((1000u16).checked_square(), None);
/// ```
pub mod checked_square;
/// [`CheckedSub`](traits::CheckedSub), a trait for subtracting two numbers and checking whether the
/// result is representable.
pub mod checked_sub;
/// [`CheckedSubMul`](traits::CheckedSubMul), a trait for subtracting the product of two numbers
/// from another number, and checking whether the result is representable.
///
/// # checked_sub_mul
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedSubMul;
///
/// assert_eq!(60u8.checked_sub_mul(5, 10), Some(10));
/// assert_eq!(2u8.checked_sub_mul(10, 5), None);
///
/// assert_eq!(127i8.checked_sub_mul(2, 100), Some(-73));
/// assert_eq!((-127i8).checked_sub_mul(2, 100), None);
/// ```
pub mod checked_sub_mul;
/// [`CoprimeWith`](traits::CoprimeWith), a trait for determining whether two numbers are coprime.
///
/// # coprime_with
/// ```
/// use malachite_base::num::arithmetic::traits::CoprimeWith;
///
/// assert_eq!(0u8.coprime_with(0), false);
/// assert_eq!(0u8.coprime_with(1), true);
/// assert_eq!(6u8.coprime_with(1), true);
/// assert_eq!(3u8.coprime_with(5), true);
/// assert_eq!(6u8.coprime_with(4), false);
/// assert_eq!(6u8.coprime_with(35), true);
/// ```
pub mod coprime_with;
/// [`DivExact`](traits::DivExact) and [`DivExactAssign`](traits::DivExactAssign), traits for
/// dividing two numbers when it's known that the division is exact.
///
/// # div_exact
/// ```
/// use malachite_base::num::arithmetic::traits::DivExact;
///
/// // 123 * 456 = 56088
/// assert_eq!(56088u32.div_exact(456), 123);
///
/// // -123 * -456 = 56088
/// assert_eq!(56088i64.div_exact(-456), -123);
/// ```
///
/// # div_exact_assign
/// ```
/// use malachite_base::num::arithmetic::traits::DivExactAssign;
///
/// // 123 * 456 = 56088
/// let mut x = 56088u32;
/// x.div_exact_assign(456);
/// assert_eq!(x, 123);
///
/// // -123 * -456 = 56088
/// let mut x = 56088i64;
/// x.div_exact_assign(-456);
/// assert_eq!(x, -123);
/// ```
pub mod div_exact;
/// Traits for simultaneously finding the quotient and remainder of two numbers, subject to various
/// rounding rules.
///
/// These are the traits:
///
/// | rounding     | by value or reference           | by mutable reference (assignment)      |
/// |--------------|---------------------------------|----------------------------------------|
/// | towards $-\infty$ | [`DivMod`](traits::DivMod) | [`DivAssignMod`](traits::DivAssignMod) |
/// | towards 0         | [`DivRem`](traits::DivRem) | [`DivAssignRem`](traits::DivAssignRem) |
/// | towards $\infty$  | [`CeilingDivMod`](traits::CeilingDivMod) | [`CeilingDivAssignMod`](traits::CeilingDivAssignMod) |
/// | towards $\infty$  | [`CeilingDivNegMod`](traits::CeilingDivNegMod) | [`CeilingDivAssignNegMod`](traits::CeilingDivAssignNegMod) |
///
/// [`CeilingDivMod`](traits::CeilingDivMod) and [`CeilingDivNegMod`](traits::CeilingDivNegMod) are
/// similar. The difference is that [`CeilingDivMod`](traits::CeilingDivMod) returns a remainder
/// less than or equal to 0, so that the usual relation $x = qy + r$ is satisfied, while
/// [`CeilingDivNegMod`](traits::CeilingDivNegMod) returns a remainder greater than or equal to
/// zero. This allows the remainder to have an unsigned type, but modifies the relation to $x = qy
/// - r$.
///
/// # div_mod
/// ```
/// use malachite_base::num::arithmetic::traits::DivMod;
///
/// // 2 * 10 + 3 = 23
/// assert_eq!(23u8.div_mod(10), (2, 3));
///
/// // 9 * 5 + 0 = 45
/// assert_eq!(45u32.div_mod(5), (9, 0));
///
/// // 2 * 10 + 3 = 23
/// assert_eq!(23i8.div_mod(10), (2, 3));
///
/// // -3 * -10 + -7 = 23
/// assert_eq!(23i16.div_mod(-10), (-3, -7));
///
/// // -3 * 10 + 7 = -23
/// assert_eq!((-23i32).div_mod(10), (-3, 7));
///
/// // 2 * -10 + -3 = -23
/// assert_eq!((-23i64).div_mod(-10), (2, -3));
/// ```
///
/// # div_assign_mod
/// ```
/// use malachite_base::num::arithmetic::traits::DivAssignMod;
///
/// // 2 * 10 + 3 = 23
/// let mut x = 23u8;
/// assert_eq!(x.div_assign_mod(10), 3);
/// assert_eq!(x, 2);
///
/// // 9 * 5 + 0 = 45
/// let mut x = 45u32;
/// assert_eq!(x.div_assign_mod(5), 0);
/// assert_eq!(x, 9);
///
/// // 2 * 10 + 3 = 23
/// let mut x = 23i8;
/// assert_eq!(x.div_assign_mod(10), 3);
/// assert_eq!(x, 2);
///
/// // -3 * -10 + -7 = 23
/// let mut x = 23i16;
/// assert_eq!(x.div_assign_mod(-10), -7);
/// assert_eq!(x, -3);
///
/// // -3 * 10 + 7 = -23
/// let mut x = -23i32;
/// assert_eq!(x.div_assign_mod(10), 7);
/// assert_eq!(x, -3);
///
/// // 2 * -10 + -3 = -23
/// let mut x = -23i64;
/// assert_eq!(x.div_assign_mod(-10), -3);
/// assert_eq!(x, 2);
/// ```
///
/// # div_rem
/// ```
/// use malachite_base::num::arithmetic::traits::DivRem;
///
/// // 2 * 10 + 3 = 23
/// assert_eq!(23u8.div_rem(10), (2, 3));
///
/// // 9 * 5 + 0 = 45
/// assert_eq!(45u32.div_rem(5), (9, 0));
///
/// // 2 * 10 + 3 = 23
/// assert_eq!(23i8.div_rem(10), (2, 3));
///
/// // -2 * -10 + 3 = 23
/// assert_eq!(23i16.div_rem(-10), (-2, 3));
///
/// // -2 * 10 + -3 = -23
/// assert_eq!((-23i32).div_rem(10), (-2, -3));
///
/// // 2 * -10 + -3 = -23
/// assert_eq!((-23i64).div_rem(-10), (2, -3));
/// ```
///
/// # div_assign_rem
/// ```
/// use malachite_base::num::arithmetic::traits::DivAssignRem;
///
/// // 2 * 10 + 3 = 23
/// let mut x = 23u8;
/// assert_eq!(x.div_assign_rem(10), 3);
/// assert_eq!(x, 2);
///
/// // 9 * 5 + 0 = 45
/// let mut x = 45u32;
/// assert_eq!(x.div_assign_rem(5), 0);
/// assert_eq!(x, 9);
///
/// // 2 * 10 + 3 = 23
/// let mut x = 23i8;
/// assert_eq!(x.div_assign_rem(10), 3);
/// assert_eq!(x, 2);
///
/// // -2 * -10 + 3 = 23
/// let mut x = 23i16;
/// assert_eq!(x.div_assign_rem(-10), 3);
/// assert_eq!(x, -2);
///
/// // -2 * 10 + -3 = -23
/// let mut x = -23i32;
/// assert_eq!(x.div_assign_rem(10), -3);
/// assert_eq!(x, -2);
///
/// // 2 * -10 + -3 = -23
/// let mut x = -23i64;
/// assert_eq!(x.div_assign_rem(-10), -3);
/// assert_eq!(x, 2);
/// ```
///
/// # ceiling_div_neg_mod
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingDivNegMod;
///
/// // 3 * 10 - 7 = 23
/// assert_eq!(23u8.ceiling_div_neg_mod(10), (3, 7));
///
/// // 9 * 5 + 0 = 45
/// assert_eq!(45u32.ceiling_div_neg_mod(5), (9, 0));
/// ```
///
/// # ceiling_div_assign_neg_mod
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingDivAssignNegMod;
///
/// // 3 * 10 - 7 = 23
/// let mut x = 23u8;
/// assert_eq!(x.ceiling_div_assign_neg_mod(10), 7);
/// assert_eq!(x, 3);
///
/// // 9 * 5 + 0 = 45
/// let mut x = 45u32;
/// assert_eq!(x.ceiling_div_assign_neg_mod(5), 0);
/// assert_eq!(x, 9);
/// ```
///
/// # ceiling_div_mod
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingDivMod;
///
/// // 3 * 10 + -7 = 23
/// assert_eq!(23i8.ceiling_div_mod(10), (3, -7));
///
/// // -2 * -10 + 3 = 23
/// assert_eq!(23i16.ceiling_div_mod(-10), (-2, 3));
///
/// // -2 * 10 + -3 = -23
/// assert_eq!((-23i32).ceiling_div_mod(10), (-2, -3));
///
/// // 3 * -10 + 7 = -23
/// assert_eq!((-23i64).ceiling_div_mod(-10), (3, 7));
/// ```
///
/// # ceiling_div_assign_mod
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingDivAssignMod;
///
/// // 3 * 10 + -7 = 23
/// let mut x = 23i8;
/// assert_eq!(x.ceiling_div_assign_mod(10), -7);
/// assert_eq!(x, 3);
///
/// // -2 * -10 + 3 = 23
/// let mut x = 23i16;
/// assert_eq!(x.ceiling_div_assign_mod(-10), 3);
/// assert_eq!(x, -2);
///
/// // -2 * 10 + -3 = -23
/// let mut x = -23i32;
/// assert_eq!(x.ceiling_div_assign_mod(10), -3);
/// assert_eq!(x, -2);
///
/// // 3 * -10 + 7 = -23
/// let mut x = -23i64;
/// assert_eq!(x.ceiling_div_assign_mod(-10), 7);
/// assert_eq!(x, 3);
/// ```
pub mod div_mod;
/// [`DivRound`](traits::DivRound) and [`DivExactAssign`](traits::DivRoundAssign), traits for
/// dividing two numbers according to a specified
/// [`RoundingMode`](crate::rounding_modes::RoundingMode).
///
/// # div_round
/// ```
/// use malachite_base::num::arithmetic::traits::DivRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use std::cmp::Ordering::*;
///
/// assert_eq!(10u8.div_round(4, Down), (2, Less));
/// assert_eq!(10u16.div_round(4, Up), (3, Greater));
/// assert_eq!(10u32.div_round(5, Exact), (2, Equal));
/// assert_eq!(10u64.div_round(3, Nearest), (3, Less));
/// assert_eq!(20u128.div_round(3, Nearest), (7, Greater));
/// assert_eq!(10usize.div_round(4, Nearest), (2, Less));
/// assert_eq!(14u8.div_round(4, Nearest), (4, Greater));
///
/// assert_eq!((-10i8).div_round(4, Down), (-2, Greater));
/// assert_eq!((-10i16).div_round(4, Up), (-3, Less));
/// assert_eq!((-10i32).div_round(5, Exact), (-2, Equal));
/// assert_eq!((-10i64).div_round(3, Nearest), (-3, Greater));
/// assert_eq!((-20i128).div_round(3, Nearest), (-7, Less));
/// assert_eq!((-10isize).div_round(4, Nearest), (-2, Greater));
/// assert_eq!((-14i8).div_round(4, Nearest), (-4, Less));
///
/// assert_eq!((-10i16).div_round(-4, Down), (2, Less));
/// assert_eq!((-10i32).div_round(-4, Up), (3, Greater));
/// assert_eq!((-10i64).div_round(-5, Exact), (2, Equal));
/// assert_eq!((-10i128).div_round(-3, Nearest), (3, Less));
/// assert_eq!((-20isize).div_round(-3, Nearest), (7, Greater));
/// assert_eq!((-10i8).div_round(-4, Nearest), (2, Less));
/// assert_eq!((-14i16).div_round(-4, Nearest), (4, Greater));
/// ```
///
/// # div_round_assign
/// ```
/// use malachite_base::num::arithmetic::traits::DivRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use std::cmp::Ordering::*;
///
/// let mut x = 10u8;
/// assert_eq!(x.div_round_assign(4, Down), Less);
/// assert_eq!(x, 2);
///
/// let mut x = 10u16;
/// assert_eq!(x.div_round_assign(4, Up), Greater);
/// assert_eq!(x, 3);
///
/// let mut x = 10u32;
/// assert_eq!(x.div_round_assign(5, Exact), Equal);
/// assert_eq!(x, 2);
///
/// let mut x = 10u64;
/// assert_eq!(x.div_round_assign(3, Nearest), Less);
/// assert_eq!(x, 3);
///
/// let mut x = 20u128;
/// assert_eq!(x.div_round_assign(3, Nearest), Greater);
/// assert_eq!(x, 7);
///
/// let mut x = 10usize;
/// assert_eq!(x.div_round_assign(4, Nearest), Less);
/// assert_eq!(x, 2);
///
/// let mut x = 14u8;
/// assert_eq!(x.div_round_assign(4, Nearest), Greater);
/// assert_eq!(x, 4);
///
/// let mut x = -10i8;
/// assert_eq!(x.div_round_assign(4, Down), Greater);
/// assert_eq!(x, -2);
///
/// let mut x = -10i16;
/// assert_eq!(x.div_round_assign(4, Up), Less);
/// assert_eq!(x, -3);
///
/// let mut x = -10i32;
/// assert_eq!(x.div_round_assign(5, Exact), Equal);
/// assert_eq!(x, -2);
///
/// let mut x = -10i64;
/// assert_eq!(x.div_round_assign(3, Nearest), Greater);
/// assert_eq!(x, -3);
///
/// let mut x = -20i128;
/// assert_eq!(x.div_round_assign(3, Nearest), Less);
/// assert_eq!(x, -7);
///
/// let mut x = -10isize;
/// assert_eq!(x.div_round_assign(4, Nearest), Greater);
/// assert_eq!(x, -2);
///
/// let mut x = -14i8;
/// assert_eq!(x.div_round_assign(4, Nearest), Less);
/// assert_eq!(x, -4);
///
/// let mut x = -10i16;
/// assert_eq!(x.div_round_assign(-4, Down), Less);
/// assert_eq!(x, 2);
///
/// let mut x = -10i32;
/// assert_eq!(x.div_round_assign(-4, Up), Greater);
/// assert_eq!(x, 3);
///
/// let mut x = -10i64;
/// assert_eq!(x.div_round_assign(-5, Exact), Equal);
/// assert_eq!(x, 2);
///
/// let mut x = -10i128;
/// assert_eq!(x.div_round_assign(-3, Nearest), Less);
/// assert_eq!(x, 3);
///
/// let mut x = -20isize;
/// assert_eq!(x.div_round_assign(-3, Nearest), Greater);
/// assert_eq!(x, 7);
///
/// let mut x = -10i8;
/// assert_eq!(x.div_round_assign(-4, Nearest), Less);
/// assert_eq!(x, 2);
///
/// let mut x = -14i16;
/// assert_eq!(x.div_round_assign(-4, Nearest), Greater);
/// assert_eq!(x, 4);
/// ```
pub mod div_round;
/// [`DivisibleBy`](traits::DivisibleBy), a trait for determining whether one number is divisible by
/// another.
///
/// # divisible_by
/// ```
/// use malachite_base::num::arithmetic::traits::DivisibleBy;
///
/// assert_eq!(0u8.divisible_by(0), true);
/// assert_eq!(100u16.divisible_by(3), false);
/// assert_eq!(102u32.divisible_by(3), true);
///
/// assert_eq!(0i8.divisible_by(0), true);
/// assert_eq!((-100i16).divisible_by(-3), false);
/// assert_eq!(102i32.divisible_by(-3), true);
/// ```
pub mod divisible_by;
/// [`DivisibleByPowerOf2`](traits::DivisibleByPowerOf2), a trait for determining whether a number
/// is divisible by $2^k$.
///
/// # divisible_by_power_of_2
/// ```
/// use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;
///
/// assert_eq!(0u8.divisible_by_power_of_2(100), true);
/// assert_eq!(96u16.divisible_by_power_of_2(5), true);
/// assert_eq!(96u32.divisible_by_power_of_2(6), false);
///
/// assert_eq!(0i8.divisible_by_power_of_2(100), true);
/// assert_eq!((-96i16).divisible_by_power_of_2(5), true);
/// assert_eq!(96i32.divisible_by_power_of_2(6), false);
/// ```
pub mod divisible_by_power_of_2;
/// [`EqMod`](traits::EqMod), a trait for determining whether one number is equal by another modulo
/// a third.
///
/// # eq_mod
/// ```
/// use malachite_base::num::arithmetic::traits::EqMod;
///
/// assert_eq!(123u16.eq_mod(223, 100), true);
/// assert_eq!((-123i32).eq_mod(277, 100), true);
/// assert_eq!((-123i64).eq_mod(278, 100), false);
/// ```
pub mod eq_mod;
/// [`EqModPowerOf2`](traits::EqModPowerOf2), a trait for determining whether one number is equal to
/// another modulo $2^k$.
///
/// # eq_mod_power_of_2
/// ```
/// use malachite_base::num::arithmetic::traits::EqModPowerOf2;
///
/// assert_eq!(0u16.eq_mod_power_of_2(256, 8), true);
/// assert_eq!((-0b1101i32).eq_mod_power_of_2(0b11011, 3), true);
/// assert_eq!((-0b1101i64).eq_mod_power_of_2(0b11011, 4), false);
/// ```
pub mod eq_mod_power_of_2;
/// [`ExtendedGcd`](traits::ExtendedGcd), a trait for computing the GCD (greatest common divisor) of
/// two numbers as well as the coefficients of Bézout's identity $ax+by=\gcd(a,b)$.
///
/// # extended_gcd
/// ```
/// use malachite_base::num::arithmetic::traits::ExtendedGcd;
///
/// assert_eq!(3u8.extended_gcd(5), (1, 2, -1));
/// assert_eq!(240u16.extended_gcd(46), (2, -9, 47));
/// assert_eq!((-111i16).extended_gcd(300), (3, 27, 10));
/// ```
pub mod extended_gcd;
/// Traits for computing the factorial, double factorial, multifactorial, and subfactorial. Each
/// function has a trait whose implementations panic if the result cannot be represented, and a
/// checked trait whose implementations return `None` in that case. The traits are
/// [`Factorial`](traits::Factorial), [`DoubleFactorial`](traits::DoubleFactorial),
/// [`Multifactorial`](traits::Multifactorial), [`Subfactorial`](traits::Subfactorial),
/// [`CheckedFactorial`](traits::CheckedFactorial),
/// [`CheckedDoubleFactorial`](traits::CheckedDoubleFactorial),
/// [`CheckedMultifactorial`](traits::CheckedMultifactorial), and
/// [`CheckedSubfactorial`](traits::CheckedSubfactorial).
///
/// # factorial
/// ```
/// use malachite_base::num::arithmetic::traits::Factorial;
///
/// assert_eq!(u8::factorial(0), 1);
/// assert_eq!(u8::factorial(1), 1);
/// assert_eq!(u8::factorial(2), 2);
/// assert_eq!(u8::factorial(3), 6);
/// assert_eq!(u8::factorial(4), 24);
/// assert_eq!(u8::factorial(5), 120);
/// assert_eq!(u32::factorial(10), 3628800);
/// ```
///
/// # checked_factorial
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedFactorial;
///
/// assert_eq!(u8::checked_factorial(0), Some(1));
/// assert_eq!(u8::checked_factorial(1), Some(1));
/// assert_eq!(u8::checked_factorial(2), Some(2));
/// assert_eq!(u8::checked_factorial(3), Some(6));
/// assert_eq!(u8::checked_factorial(4), Some(24));
/// assert_eq!(u8::checked_factorial(5), Some(120));
/// assert_eq!(u8::checked_factorial(6), None);
/// assert_eq!(u32::checked_factorial(10), Some(3628800));
/// assert_eq!(u32::checked_factorial(100), None);
/// ```
///
/// # double_factorial
/// ```
/// use malachite_base::num::arithmetic::traits::DoubleFactorial;
///
/// assert_eq!(u8::double_factorial(0), 1);
/// assert_eq!(u8::double_factorial(1), 1);
/// assert_eq!(u8::double_factorial(2), 2);
/// assert_eq!(u8::double_factorial(3), 3);
/// assert_eq!(u8::double_factorial(4), 8);
/// assert_eq!(u8::double_factorial(5), 15);
/// assert_eq!(u8::double_factorial(6), 48);
/// assert_eq!(u8::double_factorial(7), 105);
/// assert_eq!(u32::double_factorial(19), 654729075);
/// assert_eq!(u32::double_factorial(20), 3715891200);
/// ```
///
/// # checked_double_factorial
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedDoubleFactorial;
///
/// assert_eq!(u8::checked_double_factorial(0), Some(1));
/// assert_eq!(u8::checked_double_factorial(1), Some(1));
/// assert_eq!(u8::checked_double_factorial(2), Some(2));
/// assert_eq!(u8::checked_double_factorial(3), Some(3));
/// assert_eq!(u8::checked_double_factorial(4), Some(8));
/// assert_eq!(u8::checked_double_factorial(5), Some(15));
/// assert_eq!(u8::checked_double_factorial(6), Some(48));
/// assert_eq!(u8::checked_double_factorial(7), Some(105));
/// assert_eq!(u8::checked_double_factorial(8), None);
/// assert_eq!(u32::checked_double_factorial(19), Some(654729075));
/// assert_eq!(u32::checked_double_factorial(20), Some(3715891200));
/// assert_eq!(u32::checked_double_factorial(100), None);
/// ```
///
/// # multifactorial
/// ```
/// use malachite_base::num::arithmetic::traits::Multifactorial;
///
/// assert_eq!(u8::multifactorial(0, 1), 1);
/// assert_eq!(u8::multifactorial(1, 1), 1);
/// assert_eq!(u8::multifactorial(2, 1), 2);
/// assert_eq!(u8::multifactorial(3, 1), 6);
/// assert_eq!(u8::multifactorial(4, 1), 24);
/// assert_eq!(u8::multifactorial(5, 1), 120);
///
/// assert_eq!(u8::multifactorial(0, 2), 1);
/// assert_eq!(u8::multifactorial(1, 2), 1);
/// assert_eq!(u8::multifactorial(2, 2), 2);
/// assert_eq!(u8::multifactorial(3, 2), 3);
/// assert_eq!(u8::multifactorial(4, 2), 8);
/// assert_eq!(u8::multifactorial(5, 2), 15);
/// assert_eq!(u8::multifactorial(6, 2), 48);
/// assert_eq!(u8::multifactorial(7, 2), 105);
///
/// assert_eq!(u8::multifactorial(0, 3), 1);
/// assert_eq!(u8::multifactorial(1, 3), 1);
/// assert_eq!(u8::multifactorial(2, 3), 2);
/// assert_eq!(u8::multifactorial(3, 3), 3);
/// assert_eq!(u8::multifactorial(4, 3), 4);
/// assert_eq!(u8::multifactorial(5, 3), 10);
/// assert_eq!(u8::multifactorial(6, 3), 18);
/// assert_eq!(u8::multifactorial(7, 3), 28);
/// assert_eq!(u8::multifactorial(8, 3), 80);
/// assert_eq!(u8::multifactorial(9, 3), 162);
///
/// assert_eq!(u32::multifactorial(10, 1), 3628800);
/// assert_eq!(u32::multifactorial(20, 2), 3715891200);
/// assert_eq!(u32::multifactorial(25, 3), 608608000);
/// ```
///
/// # checked_multifactorial
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedMultifactorial;
///
/// assert_eq!(u8::checked_multifactorial(0, 1), Some(1));
/// assert_eq!(u8::checked_multifactorial(1, 1), Some(1));
/// assert_eq!(u8::checked_multifactorial(2, 1), Some(2));
/// assert_eq!(u8::checked_multifactorial(3, 1), Some(6));
/// assert_eq!(u8::checked_multifactorial(4, 1), Some(24));
/// assert_eq!(u8::checked_multifactorial(5, 1), Some(120));
/// assert_eq!(u8::checked_multifactorial(6, 1), None);
///
/// assert_eq!(u8::checked_multifactorial(0, 2), Some(1));
/// assert_eq!(u8::checked_multifactorial(1, 2), Some(1));
/// assert_eq!(u8::checked_multifactorial(2, 2), Some(2));
/// assert_eq!(u8::checked_multifactorial(3, 2), Some(3));
/// assert_eq!(u8::checked_multifactorial(4, 2), Some(8));
/// assert_eq!(u8::checked_multifactorial(5, 2), Some(15));
/// assert_eq!(u8::checked_multifactorial(6, 2), Some(48));
/// assert_eq!(u8::checked_multifactorial(7, 2), Some(105));
/// assert_eq!(u8::checked_multifactorial(8, 2), None);
///
/// assert_eq!(u8::checked_multifactorial(0, 3), Some(1));
/// assert_eq!(u8::checked_multifactorial(1, 3), Some(1));
/// assert_eq!(u8::checked_multifactorial(2, 3), Some(2));
/// assert_eq!(u8::checked_multifactorial(3, 3), Some(3));
/// assert_eq!(u8::checked_multifactorial(4, 3), Some(4));
/// assert_eq!(u8::checked_multifactorial(5, 3), Some(10));
/// assert_eq!(u8::checked_multifactorial(6, 3), Some(18));
/// assert_eq!(u8::checked_multifactorial(7, 3), Some(28));
/// assert_eq!(u8::checked_multifactorial(8, 3), Some(80));
/// assert_eq!(u8::checked_multifactorial(9, 3), Some(162));
/// assert_eq!(u8::checked_multifactorial(10, 3), None);
///
/// assert_eq!(u32::checked_multifactorial(10, 1), Some(3628800));
/// assert_eq!(u32::checked_multifactorial(20, 2), Some(3715891200));
/// assert_eq!(u32::checked_multifactorial(25, 3), Some(608608000));
/// assert_eq!(u32::checked_multifactorial(100, 1), None);
/// assert_eq!(u32::checked_multifactorial(100, 2), None);
/// assert_eq!(u32::checked_multifactorial(100, 3), None);
/// ```
///
/// # subfactorial
/// ```
/// use malachite_base::num::arithmetic::traits::Subfactorial;
///
/// assert_eq!(u8::subfactorial(0), 1);
/// assert_eq!(u8::subfactorial(1), 0);
/// assert_eq!(u8::subfactorial(2), 1);
/// assert_eq!(u8::subfactorial(3), 2);
/// assert_eq!(u8::subfactorial(4), 9);
/// assert_eq!(u8::subfactorial(5), 44);
/// assert_eq!(u32::subfactorial(10), 1334961);
/// ```
///
/// # checked_subfactorial
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedSubfactorial;
///
/// assert_eq!(u8::checked_subfactorial(0), Some(1));
/// assert_eq!(u8::checked_subfactorial(1), Some(0));
/// assert_eq!(u8::checked_subfactorial(2), Some(1));
/// assert_eq!(u8::checked_subfactorial(3), Some(2));
/// assert_eq!(u8::checked_subfactorial(4), Some(9));
/// assert_eq!(u8::checked_subfactorial(5), Some(44));
/// assert_eq!(u8::checked_subfactorial(6), None);
/// assert_eq!(u32::checked_subfactorial(10), Some(1334961));
/// assert_eq!(u32::checked_subfactorial(100), None);
/// ```
pub mod factorial;
/// [`Floor`](traits::Floor) and [`FloorAssign`](traits::FloorAssign), traits for computing the
/// floor of a number.
///
/// # floor_assign
/// ```
/// use malachite_base::num::arithmetic::traits::FloorAssign;
///
/// let mut x = 1.5f32;
/// x.floor_assign();
/// assert_eq!(x, 1.0);
///
/// let mut x = -1.5f32;
/// x.floor_assign();
/// assert_eq!(x, -2.0);
/// ```
pub mod floor;
/// [`Gcd`](traits::Gcd) and [`GcdAssign`](traits::GcdAssign), traits for computing the GCD
/// (greatest common divisor) of two numbers.
///
/// # gcd
/// ```
/// use malachite_base::num::arithmetic::traits::Gcd;
///
/// assert_eq!(3u8.gcd(5), 1);
/// assert_eq!(12u16.gcd(90), 6);
/// ```
///
/// # gcd_assign
/// ```
/// use malachite_base::num::arithmetic::traits::GcdAssign;
///
/// let mut x = 3u8;
/// x.gcd_assign(5);
/// assert_eq!(x, 1);
///
/// let mut x = 12u16;
/// x.gcd_assign(90);
/// assert_eq!(x, 6);
/// ```
pub mod gcd;
/// [`IsPowerOf2`](traits::IsPowerOf2), a trait for determining whether a number is an integer power
/// of 2.
///
/// # is_power_of_2
/// ```
/// use malachite_base::num::arithmetic::traits::IsPowerOf2;
///
/// assert_eq!(4.0.is_power_of_2(), true);
/// assert_eq!(0.25.is_power_of_2(), true);
/// assert_eq!(0.2.is_power_of_2(), false);
/// assert_eq!((-4.0).is_power_of_2(), false);
/// ```
pub mod is_power_of_2;
/// [`LegendreSymbol`](traits::LegendreSymbol), [`JacobiSymbol`](traits::JacobiSymbol), and
/// [`KroneckerSymbol`](traits::KroneckerSymbol), traits for computing the Legendre, Jacobi, and
/// Kronecker symbols of two numbers.
///
/// # legendre_symbol
/// ```
/// use malachite_base::num::arithmetic::traits::LegendreSymbol;
///
/// assert_eq!(10u8.legendre_symbol(5), 0);
/// assert_eq!(7u8.legendre_symbol(5), -1);
/// assert_eq!(11u8.legendre_symbol(5), 1);
///
/// assert_eq!((-7i8).legendre_symbol(5), -1);
/// assert_eq!((-11i8).legendre_symbol(5), 1);
/// ```
///
/// # jacobi_symbol
/// ```
/// use malachite_base::num::arithmetic::traits::JacobiSymbol;
///
/// assert_eq!(10u8.jacobi_symbol(5), 0);
/// assert_eq!(7u8.jacobi_symbol(5), -1);
/// assert_eq!(11u8.jacobi_symbol(5), 1);
/// assert_eq!(11u8.jacobi_symbol(9), 1);
///
/// assert_eq!((-7i8).jacobi_symbol(5), -1);
/// assert_eq!((-11i8).jacobi_symbol(5), 1);
/// assert_eq!((-11i8).jacobi_symbol(9), 1);
/// ```
///
/// # kronecker_symbol
/// ```
/// use malachite_base::num::arithmetic::traits::KroneckerSymbol;
///
/// assert_eq!(10u8.kronecker_symbol(5), 0);
/// assert_eq!(7u8.kronecker_symbol(5), -1);
/// assert_eq!(11u8.kronecker_symbol(5), 1);
/// assert_eq!(11u8.kronecker_symbol(9), 1);
/// assert_eq!(11u8.kronecker_symbol(8), -1);
///
/// assert_eq!((-7i8).kronecker_symbol(5), -1);
/// assert_eq!((-11i8).kronecker_symbol(5), 1);
/// assert_eq!((-11i8).kronecker_symbol(9), 1);
/// assert_eq!((-11i8).kronecker_symbol(8), -1);
/// assert_eq!((-11i8).kronecker_symbol(-8), 1);
/// ```
pub mod kronecker_symbol;
/// [`Lcm`](traits::Lcm), [`LcmAssign`](traits::LcmAssign), and [`CheckedLcm`](traits::CheckedLcm),
/// traits for computing the LCM (least common multiple) of two numbers.
///
/// # lcm
/// ```
/// use malachite_base::num::arithmetic::traits::Lcm;
///
/// assert_eq!(3u8.lcm(5), 15);
/// assert_eq!(12u16.lcm(90), 180);
/// ```
///
/// # lcm_assign
/// ```
/// use malachite_base::num::arithmetic::traits::LcmAssign;
///
/// let mut x = 3u8;
/// x.lcm_assign(5);
/// assert_eq!(x, 15);
///
/// let mut x = 12u16;
/// x.lcm_assign(90);
/// assert_eq!(x, 180);
/// ```
///
/// # checked_lcm
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedLcm;
///
/// assert_eq!(3u8.checked_lcm(5), Some(15));
/// assert_eq!(12u16.checked_lcm(90), Some(180));
/// assert_eq!(120u8.checked_lcm(90), None);
/// ```
pub mod lcm;
/// Traits for taking the base-$b$ logarithm of a number.
///
/// The traits are [`FloorLogBase`](traits::FloorLogBase),
/// [`CeilingLogBase`](traits::CeilingLogBase), and [`CheckedLogBase`](traits::CheckedLogBase).
///
/// # floor_log_base
/// ```
/// use malachite_base::num::arithmetic::traits::FloorLogBase;
///
/// assert_eq!(1u8.floor_log_base(5), 0);
/// assert_eq!(125u8.floor_log_base(5), 3);
/// assert_eq!(99u64.floor_log_base(10), 1);
/// assert_eq!(100u64.floor_log_base(10), 2);
/// assert_eq!(101u64.floor_log_base(10), 2);
/// ```
///
/// # ceiling_log_base
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingLogBase;
///
/// assert_eq!(1u8.ceiling_log_base(5), 0);
/// assert_eq!(125u8.ceiling_log_base(5), 3);
/// assert_eq!(99u64.ceiling_log_base(10), 2);
/// assert_eq!(100u64.ceiling_log_base(10), 2);
/// assert_eq!(101u64.ceiling_log_base(10), 3);
/// ```
///
/// # checked_log_base
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedLogBase;
///
/// assert_eq!(1u8.checked_log_base(5), Some(0));
/// assert_eq!(125u8.checked_log_base(5), Some(3));
/// assert_eq!(99u64.checked_log_base(10), None);
/// assert_eq!(100u64.checked_log_base(10), Some(2));
/// assert_eq!(101u64.checked_log_base(10), None);
/// ```
pub mod log_base;
/// Traits for taking the base-2 logarithm of a number.
///
/// The traits are [`FloorLogBase2`](traits::FloorLogBase2),
/// [`CeilingLogBase2`](traits::CeilingLogBase2), and [`CheckedLogBase2`](traits::CheckedLogBase2).
///
/// # floor_log_base_2
/// ```
/// use malachite_base::num::arithmetic::traits::FloorLogBase2;
///
/// assert_eq!(1u8.floor_log_base_2(), 0);
/// assert_eq!(100u64.floor_log_base_2(), 6);
///
/// assert_eq!(1.0f32.floor_log_base_2(), 0);
/// assert_eq!(100.0f32.floor_log_base_2(), 6);
/// assert_eq!(0.1f32.floor_log_base_2(), -4);
/// ```
///
/// # ceiling_log_base_2
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingLogBase2;
///
/// assert_eq!(1u8.ceiling_log_base_2(), 0);
/// assert_eq!(100u64.ceiling_log_base_2(), 7);
///
/// assert_eq!(1.0f32.ceiling_log_base_2(), 0);
/// assert_eq!(100.0f32.ceiling_log_base_2(), 7);
/// assert_eq!(0.1f32.ceiling_log_base_2(), -3);
/// ```
///
/// # checked_log_base_2
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedLogBase2;
///
/// assert_eq!(1u8.checked_log_base_2(), Some(0));
/// assert_eq!(100u64.checked_log_base_2(), None);
/// assert_eq!(128u64.checked_log_base_2(), Some(7));
///
/// assert_eq!(1.0f32.checked_log_base_2(), Some(0));
/// assert_eq!(100.0f32.checked_log_base_2(), None);
/// assert_eq!(128.0f32.checked_log_base_2(), Some(7));
/// assert_eq!(0.1f32.checked_log_base_2(), None);
/// assert_eq!(0.0625f32.checked_log_base_2(), Some(-4));
/// ```
pub mod log_base_2;
/// Traits for taking the base-$2^k$ logarithm of a number.
///
/// The traits are [`FloorLogBasePowerOf2`](traits::FloorLogBasePowerOf2),
/// [`CeilingLogBasePowerOf2`](traits::CeilingLogBasePowerOf2), and
/// [`CheckedLogBasePowerOf2`](traits::CheckedLogBasePowerOf2).
///
/// # floor_log_base_power_of_2
/// ```
/// use malachite_base::num::arithmetic::traits::FloorLogBasePowerOf2;
///
/// assert_eq!(1u8.floor_log_base_power_of_2(4), 0);
/// assert_eq!(100u64.floor_log_base_power_of_2(2), 3);
///
/// assert_eq!(0.1f32.floor_log_base_power_of_2(2), -2);
/// ```
///
/// # ceiling_log_base_power_of_2
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingLogBasePowerOf2;
///
/// assert_eq!(1u8.ceiling_log_base_power_of_2(4), 0);
/// assert_eq!(100u64.ceiling_log_base_power_of_2(2), 4);
///
/// assert_eq!(0.1f32.ceiling_log_base_power_of_2(2), -1);
/// ```
///
/// # checked_log_base_power_of_2
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedLogBasePowerOf2;
///
/// assert_eq!(1u8.checked_log_base_power_of_2(4), Some(0));
/// assert_eq!(100u64.checked_log_base_power_of_2(4), None);
/// assert_eq!(256u64.checked_log_base_power_of_2(4), Some(2));
///
/// assert_eq!(0.1f32.checked_log_base_power_of_2(2), None);
/// assert_eq!(0.0625f32.checked_log_base_power_of_2(2), Some(-2));
/// ```
pub mod log_base_power_of_2;
/// [`ModAdd`](traits::ModAdd) and [`ModAddAssign`](traits::ModAddAssign), traits for adding two
/// numbers modulo another number.
///
/// # mod_add
/// ```
/// use malachite_base::num::arithmetic::traits::ModAdd;
///
/// assert_eq!(0u8.mod_add(3, 5), 3);
/// assert_eq!(7u32.mod_add(5, 10), 2);
/// ```
///
/// # mod_add_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ModAddAssign;
///
/// let mut n = 0u8;
/// n.mod_add_assign(3, 5);
/// assert_eq!(n, 3);
///
/// let mut n = 7u32;
/// n.mod_add_assign(5, 10);
/// assert_eq!(n, 2);
/// ```
pub mod mod_add;
/// [`ModInverse`](traits::ModInverse), a trait for finding the multiplicative inverse of a number
/// modulo another number.
///
/// # mod_inverse
/// ```
/// use malachite_base::num::arithmetic::traits::ModInverse;
///
/// assert_eq!(7u8.mod_inverse(10), Some(3));
/// assert_eq!(8u8.mod_inverse(10), None);
/// assert_eq!(123u32.mod_inverse(4567), Some(854));
/// ```
pub mod mod_inverse;
/// [`ModIsReduced`](traits::ModIsReduced), a trait for checking whether a number is reduced modulo
/// another number.
///
/// # mod_is_reduced
/// ```
/// use malachite_base::num::arithmetic::traits::ModIsReduced;
///
/// assert_eq!(0u8.mod_is_reduced(&5), true);
/// assert_eq!(100u64.mod_is_reduced(&100), false);
/// assert_eq!(100u16.mod_is_reduced(&101), true);
/// ```
pub mod mod_is_reduced;
/// Traits for multiplying two numbers modulo another number.
///
/// The traits are [`ModMul`](traits::ModMul), [`ModMulAssign`](traits::ModMulAssign),
/// [`ModMulPrecomputed`](traits::ModMulPrecomputed), and
/// [`ModMulPrecomputedAssign`](traits::ModMulPrecomputedAssign).
/// [`ModMulPrecomputed`](traits::ModMulPrecomputed) and
/// [`ModMulPrecomputedAssign`](traits::ModMulPrecomputedAssign) are useful when having to make
/// several multiplications modulo the same modulus.
///
/// # mod_mul
/// ```
/// use malachite_base::num::arithmetic::traits::ModMul;
///
/// assert_eq!(2u8.mod_mul(3, 7), 6);
/// assert_eq!(7u32.mod_mul(3, 10), 1);
/// ```
///
/// # mod_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ModMulAssign;
///
/// let mut n = 2u8;
/// n.mod_mul_assign(3, 7);
/// assert_eq!(n, 6);
///
/// let mut n = 7u32;
/// n.mod_mul_assign(3, 10);
/// assert_eq!(n, 1);
/// ```
///
/// # mod_mul_precomputed
/// ```
/// use malachite_base::num::arithmetic::traits::ModMulPrecomputed;
///
/// let data = u32::precompute_mod_mul_data(&7);
/// assert_eq!(2u32.mod_mul_precomputed(3, 7, &data), 6);
/// assert_eq!(5u32.mod_mul_precomputed(3, 7, &data), 1);
/// assert_eq!(4u32.mod_mul_precomputed(4, 7, &data), 2);
///
/// let data = u64::precompute_mod_mul_data(&10);
/// assert_eq!(7u64.mod_mul_precomputed(3, 10, &data), 1);
/// assert_eq!(4u64.mod_mul_precomputed(9, 10, &data), 6);
/// assert_eq!(5u64.mod_mul_precomputed(8, 10, &data), 0);
///
/// let data = u8::precompute_mod_mul_data(&7);
/// assert_eq!(2u8.mod_mul_precomputed(3, 7, &data), 6);
/// assert_eq!(5u8.mod_mul_precomputed(3, 7, &data), 1);
/// assert_eq!(4u8.mod_mul_precomputed(4, 7, &data), 2);
///
/// let data = u16::precompute_mod_mul_data(&10);
/// assert_eq!(7u16.mod_mul_precomputed(3, 10, &data), 1);
/// assert_eq!(4u16.mod_mul_precomputed(9, 10, &data), 6);
/// assert_eq!(5u16.mod_mul_precomputed(8, 10, &data), 0);
///
/// let data = u128::precompute_mod_mul_data(&7);
/// assert_eq!(2u128.mod_mul_precomputed(3, 7, &data), 6);
/// assert_eq!(5u128.mod_mul_precomputed(3, 7, &data), 1);
/// assert_eq!(4u128.mod_mul_precomputed(4, 7, &data), 2);
///
/// let data = u128::precompute_mod_mul_data(&10);
/// assert_eq!(7u128.mod_mul_precomputed(3, 10, &data), 1);
/// assert_eq!(4u128.mod_mul_precomputed(9, 10, &data), 6);
/// assert_eq!(5u128.mod_mul_precomputed(8, 10, &data), 0);
/// ```
///
/// # mod_mul_precomputed_assign
/// ```
/// use malachite_base::num::arithmetic::traits::{ModMulPrecomputed, ModMulPrecomputedAssign};
///
/// let data = u8::precompute_mod_mul_data(&7);
///
/// let mut x = 2u8;
/// x.mod_mul_precomputed_assign(3, 7, &data);
/// assert_eq!(x, 6);
///
/// let mut x = 5u8;
/// x.mod_mul_precomputed_assign(3, 7, &data);
/// assert_eq!(x, 1);
///
/// let mut x = 4u8;
/// x.mod_mul_precomputed_assign(4, 7, &data);
/// assert_eq!(x, 2);
///
/// let data = u32::precompute_mod_mul_data(&10);
///
/// let mut x = 7u32;
/// x.mod_mul_precomputed_assign(3, 10, &data);
/// assert_eq!(x, 1);
///
/// let mut x = 4u32;
/// x.mod_mul_precomputed_assign(9, 10, &data);
/// assert_eq!(x, 6);
///
/// let mut x = 5u32;
/// x.mod_mul_precomputed_assign(8, 10, &data);
/// assert_eq!(x, 0);
/// ```
pub mod mod_mul;
/// [`ModNeg`](traits::ModNeg) and [`ModNegAssign`](traits::ModNegAssign), traits for negating a
/// number modulo another number.
///
/// # mod_neg
/// ```
/// use malachite_base::num::arithmetic::traits::ModNeg;
///
/// assert_eq!(0u8.mod_neg(5), 0);
/// assert_eq!(7u32.mod_neg(10), 3);
/// assert_eq!(100u16.mod_neg(101), 1);
/// ```
///
/// # mod_neg_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ModNegAssign;
///
/// let mut n = 0u8;
/// n.mod_neg_assign(5);
/// assert_eq!(n, 0);
///
/// let mut n = 7u32;
/// n.mod_neg_assign(10);
/// assert_eq!(n, 3);
///
/// let mut n = 100u16;
/// n.mod_neg_assign(101);
/// assert_eq!(n, 1);
/// ```
pub mod mod_neg;
/// Traits for finding the remainder of two numbers, subject to various rounding rules.
///
/// These are the traits:
///
/// | rounding          | by value or reference      | by mutable reference (assignment)      |
/// |-------------------|----------------------------|----------------------------------------|
/// | towards $-\infty$ | [`Mod`](traits::Mod)       | [`ModAssign`](traits::ModAssign)       |
/// | towards $\infty$  | [`CeilingMod`](traits::CeilingMod) | [`CeilingModAssign`](traits::CeilingModAssign) |
/// | towards $\infty$  | [`NegMod`](traits::NegMod) | [`NegModAssign`](traits::NegModAssign) |
///
/// [`CeilingMod`](traits::CeilingMod) and [`NegMod`](traits::NegMod) are similar. The difference is
/// that [`CeilingMod`](traits::CeilingMod) returns a remainder less than or equal to 0, so that the
/// usual relation $x = qy + r$ is satisfied, while [`NegMod`](traits::NegMod) returns a remainder
/// greater than or equal to zero. This allows the remainder to have an unsigned type, but modifies
/// the relation to $x = qy - r$.
///
/// The [`Rem`](std::ops::Rem) trait in the standard library rounds towards 0.
///
/// # mod_op
/// ```
/// use malachite_base::num::arithmetic::traits::Mod;
///
/// // 2 * 10 + 3 = 23
/// assert_eq!(23u8.mod_op(10), 3);
///
/// // 9 * 5 + 0 = 45
/// assert_eq!(45u32.mod_op(5), 0);
///
/// // 2 * 10 + 3 = 23
/// assert_eq!(23i8.mod_op(10), 3);
///
/// // -3 * -10 + -7 = 23
/// assert_eq!(23i16.mod_op(-10), -7);
///
/// // -3 * 10 + 7 = -23
/// assert_eq!((-23i32).mod_op(10), 7);
///
/// // 2 * -10 + -3 = -23
/// assert_eq!((-23i64).mod_op(-10), -3);
/// ```
///
/// # mod_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ModAssign;
///
/// // 2 * 10 + 3 = 23
/// let mut x = 23u8;
/// x.mod_assign(10);
/// assert_eq!(x, 3);
///
/// // 9 * 5 + 0 = 45
/// let mut x = 45u32;
/// x.mod_assign(5);
/// assert_eq!(x, 0);
///
/// // 2 * 10 + 3 = 23
/// let mut x = 23i8;
/// x.mod_assign(10);
/// assert_eq!(x, 3);
///
/// // -3 * -10 + -7 = 23
/// let mut x = 23i16;
/// x.mod_assign(-10);
/// assert_eq!(x, -7);
///
/// // -3 * 10 + 7 = -23
/// let mut x = -23i32;
/// x.mod_assign(10);
/// assert_eq!(x, 7);
///
/// // 2 * -10 + -3 = -23
/// let mut x = -23i64;
/// x.mod_assign(-10);
/// assert_eq!(x, -3);
/// ```
///
/// # neg_mod
/// ```
/// use malachite_base::num::arithmetic::traits::NegMod;
///
/// // 3 * 10 - 7 = 23
/// assert_eq!(23u8.neg_mod(10), 7);
///
/// // 9 * 5 + 0 = 45
/// assert_eq!(45u32.neg_mod(5), 0);
/// ```
///
/// # neg_mod_assign
/// ```
/// use malachite_base::num::arithmetic::traits::NegModAssign;
///
/// // 3 * 10 - 7 = 23
/// let mut x = 23u8;
/// x.neg_mod_assign(10);
/// assert_eq!(x, 7);
///
/// // 9 * 5 + 0 = 45
/// let mut x = 45u32;
/// x.neg_mod_assign(5);
/// assert_eq!(x, 0);
/// ```
///
/// # ceiling_mod
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingMod;
///
/// // 3 * 10 + -7 = 23
/// assert_eq!(23i8.ceiling_mod(10), -7);
///
/// // -2 * -10 + 3 = 23
/// assert_eq!(23i16.ceiling_mod(-10), 3);
///
/// // -2 * 10 + -3 = -23
/// assert_eq!((-23i32).ceiling_mod(10), -3);
///
/// // 3 * -10 + 7 = -23
/// assert_eq!((-23i64).ceiling_mod(-10), 7);
/// ```
///
/// # ceiling_mod_assign
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingModAssign;
///
/// // 3 * 10 + -7 = 23
/// let mut x = 23i8;
/// x.ceiling_mod_assign(10);
/// assert_eq!(x, -7);
///
/// // -2 * -10 + 3 = 23
/// let mut x = 23i16;
/// x.ceiling_mod_assign(-10);
/// assert_eq!(x, 3);
///
/// // -2 * 10 + -3 = -23
/// let mut x = -23i32;
/// x.ceiling_mod_assign(10);
/// assert_eq!(x, -3);
///
/// // 3 * -10 + 7 = -23
/// let mut x = -23i64;
/// x.ceiling_mod_assign(-10);
/// assert_eq!(x, 7);
/// ```
pub mod mod_op;
/// Traits for raising a number to a power modulo another number.
///
/// The traits are [`ModPow`](traits::ModPow), [`ModPowAssign`](traits::ModPowAssign), and
/// [`ModPowPrecomputed`](traits::ModPowPrecomputed).
/// [`ModPowPrecomputed`](traits::ModPowPrecomputed) is useful when having to make several
/// exponentiations modulo the same modulus.
///
/// # mod_pow
/// ```
/// use malachite_base::num::arithmetic::traits::ModPow;
///
/// assert_eq!(4u16.mod_pow(13, 497), 445);
/// assert_eq!(10u32.mod_pow(1000, 30), 10);
/// ```
///
/// # mod_pow_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowAssign;
///
/// let mut n = 4u16;
/// n.mod_pow_assign(13, 497);
/// assert_eq!(n, 445);
///
/// let mut n = 10u32;
/// n.mod_pow_assign(1000, 30);
/// assert_eq!(n, 10);
/// ```
///
/// # mod_pow_precomputed
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowPrecomputed;
///
/// let data = u32::precompute_mod_pow_data(&497);
/// assert_eq!(4u32.mod_pow_precomputed(13, 497, &data), 445);
/// assert_eq!(5u32.mod_pow_precomputed(3, 497, &data), 125);
/// assert_eq!(4u32.mod_pow_precomputed(100, 497, &data), 116);
///
/// let data = u64::precompute_mod_pow_data(&30);
/// assert_eq!(10u64.mod_pow_precomputed(1000, 30, &data), 10);
/// assert_eq!(4u64.mod_pow_precomputed(9, 30, &data), 4);
/// assert_eq!(5u64.mod_pow_precomputed(8, 30, &data), 25);
///
/// let data = u16::precompute_mod_pow_data(&497);
/// assert_eq!(4u16.mod_pow_precomputed(13, 497, &data), 445);
/// assert_eq!(5u16.mod_pow_precomputed(3, 497, &data), 125);
/// assert_eq!(4u16.mod_pow_precomputed(100, 497, &data), 116);
///
/// let data = u8::precompute_mod_pow_data(&30);
/// assert_eq!(10u8.mod_pow_precomputed(1000, 30, &data), 10);
/// assert_eq!(4u8.mod_pow_precomputed(9, 30, &data), 4);
/// assert_eq!(5u8.mod_pow_precomputed(8, 30, &data), 25);
///
/// let data = u128::precompute_mod_pow_data(&497);
/// assert_eq!(4u128.mod_pow_precomputed(13, 497, &data), 445);
/// assert_eq!(5u128.mod_pow_precomputed(3, 497, &data), 125);
/// assert_eq!(4u128.mod_pow_precomputed(100, 497, &data), 116);
///
/// let data = u128::precompute_mod_pow_data(&30);
/// assert_eq!(10u128.mod_pow_precomputed(1000, 30, &data), 10);
/// assert_eq!(4u128.mod_pow_precomputed(9, 30, &data), 4);
/// assert_eq!(5u128.mod_pow_precomputed(8, 30, &data), 25);
/// ```
///
/// # mod_pow_precomputed_assign
/// ```
/// use malachite_base::num::arithmetic::traits::{ModPowPrecomputed, ModPowPrecomputedAssign};
///
/// let data = u32::precompute_mod_pow_data(&497);
///
/// let mut x = 4u32;
/// x.mod_pow_precomputed_assign(13, 497, &data);
/// assert_eq!(x, 445);
///
/// let mut x = 5u32;
/// x.mod_pow_precomputed_assign(3, 497, &data);
/// assert_eq!(x, 125);
///
/// let mut x = 4u32;
/// x.mod_pow_precomputed_assign(100, 497, &data);
/// assert_eq!(x, 116);
///
/// let data = u64::precompute_mod_pow_data(&30);
///
/// let mut x = 10u64;
/// x.mod_pow_precomputed_assign(1000, 30, &data);
/// assert_eq!(x, 10);
///
/// let mut x = 4u64;
/// x.mod_pow_precomputed_assign(9, 30, &data);
/// assert_eq!(x, 4);
///
/// let mut x = 5u64;
/// x.mod_pow_precomputed_assign(8, 30, &data);
/// assert_eq!(x, 25);
/// ```
pub mod mod_pow;
/// Traits for finding the remainder of a number divided by $2^k$, subject to various rounding
/// rules.
///
/// These are the traits:
///
/// | rounding | by value or reference | by mutable reference (assignment) |
/// |----------|-----------------------|-----------------------------------|
/// | towards $-\infty$ | [`ModPowerOf2`](traits::ModPowerOf2) | [`ModPowerOf2Assign`](traits::ModPowerOf2Assign)       |
/// | towards 0 | [`RemPowerOf2`](traits::RemPowerOf2) | [`RemPowerOf2Assign`](traits::RemPowerOf2Assign)       |
/// | towards $\infty$  | [`CeilingModPowerOf2`](traits::CeilingModPowerOf2) | [`CeilingModPowerOf2Assign`](traits::CeilingModPowerOf2Assign) |
/// | towards $\infty$  | [`NegModPowerOf2`](traits::NegModPowerOf2) | [`NegModPowerOf2Assign`](traits::NegModPowerOf2Assign) |
///
/// [`CeilingModPowerOf2`](traits::CeilingModPowerOf2) and
/// [`NegModPowerOf2`](traits::NegModPowerOf2) are similar. The difference is that
/// [`CeilingModPowerOf2`](traits::CeilingModPowerOf2) returns a remainder less than or equal to 0,
/// so that the usual relation $x = q2^k + r$ is satisfied, while
/// [`NegModPowerOf2`](traits::NegModPowerOf2) returns a remainder greater than or equal to zero.
/// This allows the remainder to have an unsigned type, but modifies the relation to $x = q2^k - r$.
///
/// # mod_power_of_2
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2;
///
/// // 1 * 2^8 + 4 = 260
/// assert_eq!(260u16.mod_power_of_2(8), 4);
///
/// // 100 * 2^4 + 11 = 1611
/// assert_eq!(1611u32.mod_power_of_2(4), 11);
///
/// // 1 * 2^8 + 4 = 260
/// assert_eq!(260i16.mod_power_of_2(8), 4);
///
/// // -101 * 2^4 + 5 = -1611
/// assert_eq!((-1611i32).mod_power_of_2(4), 5);
/// ```
///
/// # mod_power_of_2_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2Assign;
///
/// // 1 * 2^8 + 4 = 260
/// let mut x = 260u16;
/// x.mod_power_of_2_assign(8);
/// assert_eq!(x, 4);
///
/// // 100 * 2^4 + 11 = 1611
/// let mut x = 1611u32;
/// x.mod_power_of_2_assign(4);
/// assert_eq!(x, 11);
///
/// // 1 * 2^8 + 4 = 260
/// let mut x = 260i16;
/// x.mod_power_of_2_assign(8);
/// assert_eq!(x, 4);
///
/// // -101 * 2^4 + 5 = -1611
/// let mut x = -1611i32;
/// x.mod_power_of_2_assign(4);
/// assert_eq!(x, 5);
/// ```
///
/// # rem_power_of_2
/// ```
/// use malachite_base::num::arithmetic::traits::RemPowerOf2;
///
/// // 1 * 2^8 + 4 = 260
/// assert_eq!(260u16.rem_power_of_2(8), 4);
///
/// // 100 * 2^4 + 11 = 1611
/// assert_eq!(1611u32.rem_power_of_2(4), 11);
///
/// // 1 * 2^8 + 4 = 260
/// assert_eq!(260i16.rem_power_of_2(8), 4);
///
/// // -100 * 2^4 + -11 = -1611
/// assert_eq!((-1611i32).rem_power_of_2(4), -11);
/// ```
///
/// # rem_power_of_2_assign
/// ```
/// use malachite_base::num::arithmetic::traits::RemPowerOf2Assign;
///
/// // 1 * 2^8 + 4 = 260
/// let mut x = 260u16;
/// x.rem_power_of_2_assign(8);
/// assert_eq!(x, 4);
///
/// // 100 * 2^4 + 11 = 1611
/// let mut x = 1611u32;
/// x.rem_power_of_2_assign(4);
/// assert_eq!(x, 11);
///
/// // 1 * 2^8 + 4 = 260
/// let mut x = 260i16;
/// x.rem_power_of_2_assign(8);
/// assert_eq!(x, 4);
///
/// // -100 * 2^4 + -11 = -1611
/// let mut x = -1611i32;
/// x.rem_power_of_2_assign(4);
/// assert_eq!(x, -11);
/// ```
///
/// # neg_mod_power_of_2
/// ```
/// use malachite_base::num::arithmetic::traits::NegModPowerOf2;
///
/// // 2 * 2^8 - 252 = 260
/// assert_eq!(260u16.neg_mod_power_of_2(8), 252);
///
/// // 101 * 2^4 - 5 = 1611
/// assert_eq!(1611u32.neg_mod_power_of_2(4), 5);
/// ```
///
/// # neg_mod_power_of_2_assign
/// ```
/// use malachite_base::num::arithmetic::traits::NegModPowerOf2Assign;
///
/// // 2 * 2^8 - 252 = 260
/// let mut x = 260u16;
/// x.neg_mod_power_of_2_assign(8);
/// assert_eq!(x, 252);
///
/// // 101 * 2^4 - 5 = 1611
/// let mut x = 1611u32;
/// x.neg_mod_power_of_2_assign(4);
/// assert_eq!(x, 5);
/// ```
///
/// # ceiling_mod_power_of_2
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingModPowerOf2;
///
/// // 2 * 2^8 + -252 = 260
/// assert_eq!(260i16.ceiling_mod_power_of_2(8), -252);
///
/// // -100 * 2^4 + -11 = -1611
/// assert_eq!((-1611i32).ceiling_mod_power_of_2(4), -11);
/// ```
///
/// # ceiling_mod_power_of_2_assign
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingModPowerOf2Assign;
///
/// // 2 * 2^8 + -252 = 260
/// let mut x = 260i16;
/// x.ceiling_mod_power_of_2_assign(8);
/// assert_eq!(x, -252);
///
/// // -100 * 2^4 + -11 = -1611
/// let mut x = -1611i32;
/// x.ceiling_mod_power_of_2_assign(4);
/// assert_eq!(x, -11);
/// ```
pub mod mod_power_of_2;
/// [`ModPowerOf2Add`](traits::ModPowerOf2Add) and
/// [`ModPowerOf2AddAssign`](traits::ModPowerOf2AddAssign), traits for adding two numbers modulo
/// $2^k$.
///
/// # mod_power_of_2_add
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2Add;
///
/// assert_eq!(0u8.mod_power_of_2_add(2, 5), 2);
/// assert_eq!(10u32.mod_power_of_2_add(14, 4), 8);
/// ```
///
/// # mod_power_of_2_add_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2AddAssign;
///
/// let mut n = 0u8;
/// n.mod_power_of_2_add_assign(2, 5);
/// assert_eq!(n, 2);
///
/// let mut n = 10u32;
/// n.mod_power_of_2_add_assign(14, 4);
/// assert_eq!(n, 8);
/// ```
pub mod mod_power_of_2_add;
/// [`ModPowerOf2Inverse`](traits::ModPowerOf2Inverse), a trait for finding the multiplicative
/// inverse of a number modulo $2^k$.
///
/// # mod_inverse
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2Inverse;
///
/// assert_eq!(7u8.mod_power_of_2_inverse(4), Some(7));
/// assert_eq!(8u8.mod_power_of_2_inverse(4), None);
/// assert_eq!(123u32.mod_power_of_2_inverse(7), Some(51));
/// ```
pub mod mod_power_of_2_inverse;
/// [`ModPowerOf2IsReduced`](traits::ModPowerOf2IsReduced), a trait for checking whether a number is
/// reduced modulo $2^k$.
///
/// # mod_power_of_2_is_reduced
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced;
///
/// assert_eq!(0u8.mod_power_of_2_is_reduced(5), true);
/// assert_eq!(100u64.mod_power_of_2_is_reduced(5), false);
/// assert_eq!(100u16.mod_power_of_2_is_reduced(8), true);
/// ```
pub mod mod_power_of_2_is_reduced;
/// [`ModPowerOf2Mul`](traits::ModPowerOf2Mul) and
/// [`ModPowerOf2MulAssign`](traits::ModPowerOf2MulAssign), traits for multiplying two numbers
/// modulo $2^k$.
///
/// # mod_power_of_2_mul
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
///
/// assert_eq!(3u8.mod_power_of_2_mul(2, 5), 6);
/// assert_eq!(10u32.mod_power_of_2_mul(14, 4), 12);
/// ```
///
/// # mod_power_of_2_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign;
///
/// let mut n = 3u8;
/// n.mod_power_of_2_mul_assign(2, 5);
/// assert_eq!(n, 6);
///
/// let mut n = 10u32;
/// n.mod_power_of_2_mul_assign(14, 4);
/// assert_eq!(n, 12);
/// ```
pub mod mod_power_of_2_mul;
/// [`ModPowerOf2Neg`](traits::ModPowerOf2Neg) and
/// [`ModPowerOf2NegAssign`](traits::ModPowerOf2NegAssign), traits for negating a number modulo
/// $2^k$.
///
/// # mod_power_of_2_neg
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2Neg;
///
/// assert_eq!(0u8.mod_power_of_2_neg(5), 0);
/// assert_eq!(10u32.mod_power_of_2_neg(4), 6);
/// assert_eq!(100u16.mod_power_of_2_neg(8), 156);
/// ```
///
/// # mod_power_of_2_neg_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2NegAssign;
///
/// let mut n = 0u8;
/// n.mod_power_of_2_neg_assign(5);
/// assert_eq!(n, 0);
///
/// let mut n = 10u32;
/// n.mod_power_of_2_neg_assign(4);
/// assert_eq!(n, 6);
///
/// let mut n = 100u16;
/// n.mod_power_of_2_neg_assign(8);
/// assert_eq!(n, 156);
/// ```
pub mod mod_power_of_2_neg;
/// [`ModPowerOf2Pow`](traits::ModPowerOf2Pow) and
/// [`ModPowerOf2PowAssign`](traits::ModPowerOf2PowAssign), traits for raising a number to a power
/// modulo $2^k$.
///
/// # mod_power_of_2_pow
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2Pow;
///
/// assert_eq!(5u8.mod_power_of_2_pow(13, 3), 5);
/// assert_eq!(7u32.mod_power_of_2_pow(1000, 6), 1);
/// ```
///
/// # mod_power_of_2_pow_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2PowAssign;
///
/// let mut n = 5u8;
/// n.mod_power_of_2_pow_assign(13, 3);
/// assert_eq!(n, 5);
///
/// let mut n = 7u32;
/// n.mod_power_of_2_pow_assign(1000, 6);
/// assert_eq!(n, 1);
/// ```
pub mod mod_power_of_2_pow;
/// [`ModPowerOf2Shl`](traits::ModPowerOf2Shl) and
/// [`ModPowerOf2ShlAssign`](traits::ModPowerOf2ShlAssign), traits for left-shifting a number modulo
/// $2^k$.
///
/// # mod_power_of_2_shl
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2Shl;
///
/// assert_eq!(12u32.mod_power_of_2_shl(2u8, 5), 16);
/// assert_eq!(10u8.mod_power_of_2_shl(100u64, 4), 0);
///
/// assert_eq!(12u32.mod_power_of_2_shl(2i8, 5), 16);
/// assert_eq!(10u8.mod_power_of_2_shl(-2i64, 4), 2);
/// ```
///
/// # mod_power_of_2_shl_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2ShlAssign;
///
/// let mut n = 12u32;
/// n.mod_power_of_2_shl_assign(2u8, 5);
/// assert_eq!(n, 16);
///
/// let mut n = 10u8;
/// n.mod_power_of_2_shl_assign(100u64, 4);
/// assert_eq!(n, 0);
///
/// let mut n = 12u32;
/// n.mod_power_of_2_shl_assign(2i8, 5);
/// assert_eq!(n, 16);
///
/// let mut n = 10u8;
/// n.mod_power_of_2_shl_assign(-2i64, 4);
/// assert_eq!(n, 2);
/// ```
pub mod mod_power_of_2_shl;
/// [`ModPowerOf2Shr`](traits::ModPowerOf2Shr) and
/// [`ModPowerOf2ShrAssign`](traits::ModPowerOf2ShrAssign), traits for right-shifting a number
/// modulo $2^k$.
///
/// # mod_power_of_2_shr
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2Shr;
///
/// assert_eq!(10u8.mod_power_of_2_shr(2i64, 4), 2);
/// assert_eq!(12u32.mod_power_of_2_shr(-2i8, 5), 16);
/// ```
///
/// # mod_power_of_2_shr_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2ShrAssign;
///
/// let mut n = 10u8;
/// n.mod_power_of_2_shr_assign(2i64, 4);
/// assert_eq!(n, 2);
///
/// let mut n = 12u32;
/// n.mod_power_of_2_shr_assign(-2i8, 5);
/// assert_eq!(n, 16);
/// ```
pub mod mod_power_of_2_shr;
/// [`ModPowerOf2Square`](traits::ModPowerOf2Square) and
/// [`ModPowerOf2SquareAssign`](traits::ModPowerOf2SquareAssign), traits for squaring a number
/// modulo $2^k$.
///
/// # mod_power_of_2_square
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2Square;
///
/// assert_eq!(5u8.mod_power_of_2_square(3), 1);
/// assert_eq!(100u32.mod_power_of_2_square(8), 16);
/// ```
///
/// # mod_power_of_2_square_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2SquareAssign;
///
/// let mut n = 5u8;
/// n.mod_power_of_2_square_assign(3);
/// assert_eq!(n, 1);
///
/// let mut n = 100u32;
/// n.mod_power_of_2_square_assign(8);
/// assert_eq!(n, 16);
/// ```
pub mod mod_power_of_2_square;
/// [`ModPowerOf2Sub`](traits::ModPowerOf2Sub) and
/// [`ModPowerOf2SubAssign`](traits::ModPowerOf2SubAssign), traits for subtracting one number by
/// another modulo $2^k$.
///
/// # mod_power_of_2_sub
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2Sub;
///
/// assert_eq!(5u8.mod_power_of_2_sub(2, 5), 3);
/// assert_eq!(10u32.mod_power_of_2_sub(14, 4), 12);
/// ```
///
/// # mod_power_of_2_sub_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2SubAssign;
///
/// let mut n = 5u8;
/// n.mod_power_of_2_sub_assign(2, 5);
/// assert_eq!(n, 3);
///
/// let mut n = 10u32;
/// n.mod_power_of_2_sub_assign(14, 4);
/// assert_eq!(n, 12);
/// ```
pub mod mod_power_of_2_sub;
/// [`ModShl`](traits::ModShl) and [`ModShlAssign`](traits::ModShlAssign), traits for left-shifting
/// a number modulo another number.
///
/// # mod_shl
/// ```
/// use malachite_base::num::arithmetic::traits::ModShl;
///
/// assert_eq!(8u32.mod_shl(2u8, 10), 2);
/// assert_eq!(10u8.mod_shl(100u64, 17), 7);
///
/// assert_eq!(8u32.mod_shl(2i8, 10), 2);
/// assert_eq!(10u8.mod_shl(-2i64, 15), 2);
/// ```
///
/// # mod_shl_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ModShlAssign;
///
/// let mut n = 8u32;
/// n.mod_shl_assign(2u8, 10);
/// assert_eq!(n, 2);
///
/// let mut n = 10u8;
/// n.mod_shl_assign(100u64, 17);
/// assert_eq!(n, 7);
///
/// let mut n = 8u32;
/// n.mod_shl_assign(2i8, 10);
/// assert_eq!(n, 2);
///
/// let mut n = 10u8;
/// n.mod_shl_assign(-2i64, 15);
/// assert_eq!(n, 2);
/// ```
pub mod mod_shl;
/// [`ModShr`](traits::ModShr) and [`ModShrAssign`](traits::ModShrAssign), traits for right-shifting
/// a number modulo another number.
///
/// # mod_shr
/// ```
/// use malachite_base::num::arithmetic::traits::ModShr;
///
/// assert_eq!(10u8.mod_shr(2i64, 15), 2);
/// assert_eq!(8u32.mod_shr(-2i8, 10), 2);
/// ```
///
/// # mod_shr_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ModShrAssign;
///
/// let mut n = 10u8;
/// n.mod_shr_assign(2i64, 15);
/// assert_eq!(n, 2);
///
/// let mut n = 8u32;
/// n.mod_shr_assign(-2i8, 10);
/// assert_eq!(n, 2);
/// ```
pub mod mod_shr;
/// Traits for squaring a number modulo another number.
///
/// The traits are [`ModSquare`](traits::ModSquare), [`ModSquareAssign`](traits::ModSquareAssign),
/// and [`ModSquarePrecomputed`](traits::ModSquarePrecomputed).
/// [`ModSquarePrecomputed`](traits::ModSquarePrecomputed) is useful when having to make several
/// squarings modulo the same modulus.
///
/// # mod_square
/// ```
/// use malachite_base::num::arithmetic::traits::ModSquare;
///
/// assert_eq!(2u8.mod_square(10), 4);
/// assert_eq!(100u32.mod_square(497), 60);
/// ```
///
/// # mod_square_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ModSquareAssign;
///
/// let mut n = 2u8;
/// n.mod_square_assign(10);
/// assert_eq!(n, 4);
///
/// let mut n = 100u32;
/// n.mod_square_assign(497);
/// assert_eq!(n, 60);
/// ```
///
/// # mod_square_precomputed
/// ```
/// use malachite_base::num::arithmetic::traits::{ModPowPrecomputed, ModSquarePrecomputed};
///
/// let data = u16::precompute_mod_pow_data(&497);
/// assert_eq!(100u16.mod_square_precomputed(497, &data), 60);
/// assert_eq!(200u16.mod_square_precomputed(497, &data), 240);
/// assert_eq!(300u16.mod_square_precomputed(497, &data), 43);
/// ```
///
/// # mod_square_precomputed_assign
/// ```
/// use malachite_base::num::arithmetic::traits::{ModPowPrecomputed, ModSquarePrecomputedAssign};
///
/// let data = u32::precompute_mod_pow_data(&497);
///
/// let mut x = 100u32;
/// x.mod_square_precomputed_assign(497, &data);
/// assert_eq!(x, 60);
///
/// let mut x = 200u32;
/// x.mod_square_precomputed_assign(497, &data);
/// assert_eq!(x, 240);
///
/// let mut x = 300u32;
/// x.mod_square_precomputed_assign(497, &data);
/// assert_eq!(x, 43);
/// ```
pub mod mod_square;
/// [`ModSub`](traits::ModSub) and [`ModSubAssign`](traits::ModSubAssign), traits for subtracting
/// two numbers modulo another number.
///
/// # mod_sub
/// ```
/// use malachite_base::num::arithmetic::traits::ModSub;
///
/// assert_eq!(4u8.mod_sub(3, 5), 1);
/// assert_eq!(7u32.mod_sub(9, 10), 8);
/// ```
///
/// # mod_sub_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ModSubAssign;
///
/// let mut n = 4u8;
/// n.mod_sub_assign(3, 5);
/// assert_eq!(n, 1);
///
/// let mut n = 7u32;
/// n.mod_sub_assign(9, 10);
/// assert_eq!(n, 8);
/// ```
pub mod mod_sub;
/// [`NegAssign`](traits::NegAssign), a trait for negating a number in place.
///
/// # neg_assign
/// ```
/// use malachite_base::num::arithmetic::traits::NegAssign;
///
/// let mut x = 0i8;
/// x.neg_assign();
/// assert_eq!(x, 0i8);
///
/// let mut x = 100i64;
/// x.neg_assign();
/// assert_eq!(x, -100i64);
///
/// let mut x = -100i64;
/// x.neg_assign();
/// assert_eq!(x, 100i64);
///
/// let mut x = 1.2f32;
/// x.neg_assign();
/// assert_eq!(x, -1.2f32);
/// ```
pub mod neg;
/// [`NextPowerOf2`](traits::NextPowerOf2) and [`NextPowerOf2Assign`](traits::NextPowerOf2Assign),
/// traits for getting the next-highest power of 2.
///
/// # next_power_of_2
/// ```
/// use malachite_base::num::arithmetic::traits::NextPowerOf2;
///
/// assert_eq!(100.0f32.next_power_of_2(), 128.0);
/// assert_eq!(0.01f32.next_power_of_2(), 0.015625);
/// ```
///
/// # next_power_of_2_assign
/// ```
/// use malachite_base::num::arithmetic::traits::NextPowerOf2Assign;
///
/// let mut x = 0u8;
/// x.next_power_of_2_assign();
/// assert_eq!(x, 1);
///
/// let mut x = 4u16;
/// x.next_power_of_2_assign();
/// assert_eq!(x, 4);
///
/// let mut x = 10u32;
/// x.next_power_of_2_assign();
/// assert_eq!(x, 16);
///
/// let mut x = (1u64 << 40) - 5;
/// x.next_power_of_2_assign();
/// assert_eq!(x, 1 << 40);
///
/// let mut x = 100.0f32;
/// x.next_power_of_2_assign();
/// assert_eq!(x, 128.0);
///
/// let mut x = 0.01f32;
/// x.next_power_of_2_assign();
/// assert_eq!(x, 0.015625);
/// ```
pub mod next_power_of_2;
/// [`OverflowingAbs`](traits::OverflowingAbs) and
/// [`OverflowingAbsAssign`](traits::OverflowingAbsAssign), traits for taking the absolute value of
/// a number and returning a boolean indicating whether an overflow occurred.
///
/// # overflowing_abs_assign
/// ```
/// use malachite_base::num::arithmetic::traits::OverflowingAbsAssign;
///
/// let mut x = 0i8;
/// assert_eq!(x.overflowing_abs_assign(), false);
/// assert_eq!(x, 0);
///
/// let mut x = 100i64;
/// assert_eq!(x.overflowing_abs_assign(), false);
/// assert_eq!(x, 100);
///
/// let mut x = -100i64;
/// assert_eq!(x.overflowing_abs_assign(), false);
/// assert_eq!(x, 100);
///
/// let mut x = -128i8;
/// assert_eq!(x.overflowing_abs_assign(), true);
/// assert_eq!(x, -128);
/// ```
pub mod overflowing_abs;
/// [`OverflowingAdd`](traits::OverflowingAdd) and
/// [`OverflowingAddAssign`](traits::OverflowingAddAssign), traits for adding two numbers and
/// returning a boolean indicating whether an overflow occurred.
///
/// # overflowing_add_assign
/// ```
/// use malachite_base::num::arithmetic::traits::OverflowingAddAssign;
///
/// let mut x = 123u16;
/// assert_eq!(x.overflowing_add_assign(456), false);
/// assert_eq!(x, 579);
///
/// let mut x = 123u8;
/// assert_eq!(x.overflowing_add_assign(200), true);
/// assert_eq!(x, 67);
/// ```
pub mod overflowing_add;
/// [`OverflowingAddMul`](traits::OverflowingAddMul) and
/// [`OverflowingAddMulAssign`](traits::OverflowingAddMulAssign), traits for adding the product of
/// two other numbers to a number and returning a boolean indicating whether an overflow occurred.
///
/// # overflowing_add_mul
/// ```
/// use malachite_base::num::arithmetic::traits::OverflowingAddMul;
///
/// assert_eq!(2u8.overflowing_add_mul(3, 7), (23, false));
/// assert_eq!(2u8.overflowing_add_mul(20, 20), (146, true));
///
/// assert_eq!(127i8.overflowing_add_mul(-2, 100), (-73, false));
/// assert_eq!((-127i8).overflowing_add_mul(-2, 100), (-71, true));
/// ```
///
/// # overflowing_add_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::OverflowingAddMulAssign;
///
/// let mut x = 2u8;
/// assert_eq!(x.overflowing_add_mul_assign(3, 7), false);
/// assert_eq!(x, 23);
///
/// let mut x = 2u8;
/// assert_eq!(x.overflowing_add_mul_assign(20, 20), true);
/// assert_eq!(x, 146);
///
/// let mut x = 127i8;
/// assert_eq!(x.overflowing_add_mul_assign(-2, 100), false);
/// assert_eq!(x, -73);
///
/// let mut x = -127i8;
/// assert_eq!(x.overflowing_add_mul_assign(-2, 100), true);
/// assert_eq!(x, -71);
/// ```
pub mod overflowing_add_mul;
/// [`OverflowingDiv`](traits::OverflowingDiv) and
/// [`OverflowingDivAssign`](traits::OverflowingDivAssign), traits for dividing two numbers and
/// returning a boolean indicating whether an overflow occurred.
///
/// # overflowing_div_assign
/// ```
/// use malachite_base::num::arithmetic::traits::OverflowingDivAssign;
///
/// let mut x = 100u16;
/// assert_eq!(x.overflowing_div_assign(3), false);
/// assert_eq!(x, 33);
///
/// let mut x = -128i8;
/// assert_eq!(x.overflowing_div_assign(-1), true);
/// assert_eq!(x, -128);
/// ```
pub mod overflowing_div;
/// [`OverflowingMul`](traits::OverflowingMul) and
/// [`OverflowingMulAssign`](traits::OverflowingMulAssign), traits for multiplying two numbers and
/// returning a boolean indicating whether an overflow occurred.
///
/// # overflowing_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::OverflowingMulAssign;
///
/// let mut x = 123u16;
/// assert_eq!(x.overflowing_mul_assign(456), false);
/// assert_eq!(x, 56088);
///
/// let mut x = 123u8;
/// assert_eq!(x.overflowing_mul_assign(200), true);
/// assert_eq!(x, 24);
/// ```
pub mod overflowing_mul;
/// [`OverflowingNeg`](traits::OverflowingNeg) and
/// [`OverflowingNegAssign`](traits::OverflowingNegAssign), traits for negating a number and
/// returning a boolean indicating whether an overflow occurred.
///
/// # overflowing_neg_assign
/// ```
/// use malachite_base::num::arithmetic::traits::OverflowingNegAssign;
///
/// let mut x = 0i8;
/// assert_eq!(x.overflowing_neg_assign(), false);
/// assert_eq!(x, 0);
///
/// let mut x = 100u64;
/// assert_eq!(x.overflowing_neg_assign(), true);
/// assert_eq!(x, 18446744073709551516);
///
/// let mut x = -100i64;
/// assert_eq!(x.overflowing_neg_assign(), false);
/// assert_eq!(x, 100);
///
/// let mut x = -128i8;
/// assert_eq!(x.overflowing_neg_assign(), true);
/// assert_eq!(x, -128);
/// ```
pub mod overflowing_neg;
/// [`OverflowingPow`](traits::OverflowingPow) and
/// [`OverflowingPowAssign`](traits::OverflowingPowAssign), traits for raising a number to a power
/// and returning a boolean indicating whether an overflow occurred.
///
/// # overflowing_pow_assign
/// ```
/// use malachite_base::num::arithmetic::traits::OverflowingPowAssign;
///
/// let mut x = 3u8;
/// assert_eq!(x.overflowing_pow_assign(3), false);
/// assert_eq!(x, 27);
///
/// let mut x = -10i32;
/// assert_eq!(x.overflowing_pow_assign(9), false);
/// assert_eq!(x, -1000000000);
///
/// let mut x = -10i16;
/// assert_eq!(x.overflowing_pow_assign(9), true);
/// assert_eq!(x, 13824);
/// ```
pub mod overflowing_pow;
/// [`OverflowingSquare`](traits::OverflowingSquare) and
/// [`OverflowingSquareAssign`](traits::OverflowingSquareAssign), traits for squaring a number and
/// returning a boolean indicating whether an overflow occurred.
///
/// # overflowing_square_assign
/// ```
/// use malachite_base::num::arithmetic::traits::OverflowingSquareAssign;
///
/// let mut x = 3u8;
/// assert_eq!(x.overflowing_square_assign(), false);
/// assert_eq!(x, 9);
///
/// let mut x = -1000i32;
/// assert_eq!(x.overflowing_square_assign(), false);
/// assert_eq!(x, 1000000);
///
/// let mut x = 1000u16;
/// assert_eq!(x.overflowing_square_assign(), true);
/// assert_eq!(x, 16960);
/// ```
pub mod overflowing_square;
/// [`OverflowingSub`](traits::OverflowingSub) and
/// [`OverflowingSubAssign`](traits::OverflowingSubAssign), traits for subtracting two numbers and
/// returning a boolean indicating whether an overflow occurred.
///
/// # overflowing_sub
/// ```
/// use malachite_base::num::arithmetic::traits::OverflowingSquare;
///
/// assert_eq!(3u8.overflowing_square(), (9, false));
/// assert_eq!((-1000i32).overflowing_square(), (1000000, false));
/// assert_eq!(1000u16.overflowing_square(), (16960, true));
/// ```
///
/// # overflowing_sub_assign
/// ```
/// use malachite_base::num::arithmetic::traits::OverflowingSubAssign;
///
/// let mut x = 456u16;
/// assert_eq!(x.overflowing_sub_assign(123), false);
/// assert_eq!(x, 333);
///
/// let mut x = 123u16;
/// assert_eq!(x.overflowing_sub_assign(456), true);
/// assert_eq!(x, 65203);
/// ```
pub mod overflowing_sub;
/// [`OverflowingSubMul`](traits::OverflowingSubMul) and
/// [`OverflowingSubMulAssign`](traits::OverflowingSubMulAssign), traits for subtracting the product
/// of two other numbers from a number and returning a boolean indicating whether an overflow
/// occurred.
///
/// # overflowing_sub_mul
/// ```
/// use malachite_base::num::arithmetic::traits::OverflowingSubMul;
///
/// assert_eq!(60u8.overflowing_sub_mul(5, 10), (10, false));
/// assert_eq!(2u8.overflowing_sub_mul(10, 5), (208, true));
///
/// assert_eq!(127i8.overflowing_sub_mul(2, 100), (-73, false));
/// assert_eq!((-127i8).overflowing_sub_mul(2, 100), (-71, true));
/// ```
///
/// # overflowing_sub_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::OverflowingSubMulAssign;
///
/// let mut x = 60u8;
/// assert_eq!(x.overflowing_sub_mul_assign(5, 10), false);
/// assert_eq!(x, 10);
///
/// let mut x = 2u8;
/// assert_eq!(x.overflowing_sub_mul_assign(10, 5), true);
/// assert_eq!(x, 208);
///
/// let mut x = 127i8;
/// assert_eq!(x.overflowing_sub_mul_assign(2, 100), false);
/// assert_eq!(x, -73);
///
/// let mut x = -127i8;
/// assert_eq!(x.overflowing_sub_mul_assign(2, 100), true);
/// assert_eq!(x, -71);
/// ```
pub mod overflowing_sub_mul;
/// [`Parity`](traits::Parity), a trait for determining whether a number is even or odd.
///
/// # even
/// ```
/// use malachite_base::num::arithmetic::traits::Parity;
///
/// assert_eq!(0u8.even(), true);
/// assert_eq!((-5i16).even(), false);
/// assert_eq!(4u32.even(), true);
/// ```
///
/// # odd
/// ```
/// use malachite_base::num::arithmetic::traits::Parity;
///
/// assert_eq!(0u8.odd(), false);
/// assert_eq!((-5i16).odd(), true);
/// assert_eq!(4u32.odd(), false);
/// ```
pub mod parity;
/// [`Pow`](traits::Pow) and [`PowAssign`](traits::PowAssign), traits for raising a number to a
/// power.
///
/// # pow_assign
/// ```
/// use malachite_base::num::arithmetic::traits::PowAssign;
///
/// let mut x = 3u8;
/// x.pow_assign(3);
/// assert_eq!(x, 27);
///
/// let mut x = -10i32;
/// x.pow_assign(9);
/// assert_eq!(x, -1000000000);
///
/// let mut x = 2.0f32;
/// x.pow_assign(5);
/// assert_eq!(x, 32.0);
///
/// let mut x = 2.0f32;
/// x.pow_assign(5.0);
/// assert_eq!(x, 32.0);
/// ```
pub mod pow;
/// [`PowerOf2`](traits::PowerOf2), a trait for computing a power of 2.
///
/// # power_of_2
/// ```
/// use malachite_base::num::arithmetic::traits::PowerOf2;
///
/// assert_eq!(u16::power_of_2(0), 1);
/// assert_eq!(u8::power_of_2(3), 8);
/// assert_eq!(u64::power_of_2(40), 1 << 40);
///
/// assert_eq!(i16::power_of_2(0), 1);
/// assert_eq!(i8::power_of_2(3), 8);
/// assert_eq!(i64::power_of_2(40), 1 << 40);
///
/// assert_eq!(f32::power_of_2(0), 1.0);
/// assert_eq!(f32::power_of_2(3), 8.0);
/// assert_eq!(f32::power_of_2(-3), 0.125);
/// ```
pub mod power_of_2;
/// Traits for computing the primorial and the product of the first $n$ primes. There is a trait
/// whose implementations panic if the result cannot be represented, and a checked trait whose
/// implementations return `None` in that case: [`Primorial`](traits::Primorial) and
/// [`CheckedPrimorial`](traits::CheckedPrimorial).
///
/// # primorial
/// ```
/// use malachite_base::num::arithmetic::traits::Primorial;
///
/// assert_eq!(u8::primorial(0), 1);
/// assert_eq!(u8::primorial(1), 1);
/// assert_eq!(u8::primorial(2), 2);
/// assert_eq!(u8::primorial(3), 6);
/// assert_eq!(u8::primorial(4), 6);
/// assert_eq!(u8::primorial(5), 30);
/// assert_eq!(u32::primorial(20), 9699690);
/// ```
///
/// # product_of_first_n_primes
/// ```
/// use malachite_base::num::arithmetic::traits::Primorial;
///
/// assert_eq!(u8::product_of_first_n_primes(0), 1);
/// assert_eq!(u8::product_of_first_n_primes(1), 2);
/// assert_eq!(u8::product_of_first_n_primes(2), 6);
/// assert_eq!(u8::product_of_first_n_primes(3), 30);
/// assert_eq!(u8::product_of_first_n_primes(4), 210);
/// assert_eq!(u32::product_of_first_n_primes(9), 223092870);
/// ```
///
/// # checked_primorial
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedPrimorial;
///
/// assert_eq!(u8::checked_primorial(0), Some(1));
/// assert_eq!(u8::checked_primorial(1), Some(1));
/// assert_eq!(u8::checked_primorial(2), Some(2));
/// assert_eq!(u8::checked_primorial(3), Some(6));
/// assert_eq!(u8::checked_primorial(4), Some(6));
/// assert_eq!(u8::checked_primorial(5), Some(30));
///
/// assert_eq!(u8::checked_primorial(11), None);
/// assert_eq!(u32::checked_primorial(20), Some(9699690));
/// assert_eq!(u32::checked_primorial(100), None);
/// ```
///
/// # checked_product_of_first_n_primes
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedPrimorial;
///
/// assert_eq!(u8::checked_product_of_first_n_primes(0), Some(1));
/// assert_eq!(u8::checked_product_of_first_n_primes(1), Some(2));
/// assert_eq!(u8::checked_product_of_first_n_primes(2), Some(6));
/// assert_eq!(u8::checked_product_of_first_n_primes(3), Some(30));
/// assert_eq!(u8::checked_product_of_first_n_primes(4), Some(210));
/// assert_eq!(u32::checked_product_of_first_n_primes(9), Some(223092870));
///
/// assert_eq!(u8::checked_product_of_first_n_primes(5), None);
/// assert_eq!(u32::checked_product_of_first_n_primes(100), None);
/// ```
pub mod primorial;
/// Traits for taking the $n$th root of a number.
///
/// The traits are [`FloorRoot`](traits::FloorRoot), [`FloorRootAssign`](traits::FloorRootAssign),
/// [`CeilingRoot`](traits::CeilingRoot), [`CeilingRootAssign`](traits::CeilingRootAssign),
/// [`CheckedRoot`](traits::CheckedRoot), [`RootRem`](traits::RootRem), and
/// [`RootAssignRem`](traits::RootAssignRem).
///
/// # floor_root
/// ```
/// use malachite_base::num::arithmetic::traits::FloorRoot;
///
/// assert_eq!(999u16.floor_root(3), 9);
/// assert_eq!(1000u16.floor_root(3), 10);
/// assert_eq!(1001u16.floor_root(3), 10);
/// assert_eq!(100000000000i64.floor_root(5), 158);
/// assert_eq!((-100000000000i64).floor_root(5), -159);
/// ```
///
/// # floor_root_assign
/// ```
/// use malachite_base::num::arithmetic::traits::FloorRootAssign;
///
/// let mut x = 999u16;
/// x.floor_root_assign(3);
/// assert_eq!(x, 9);
///
/// let mut x = 1000u16;
/// x.floor_root_assign(3);
/// assert_eq!(x, 10);
///
/// let mut x = 1001u16;
/// x.floor_root_assign(3);
/// assert_eq!(x, 10);
///
/// let mut x = 100000000000i64;
/// x.floor_root_assign(5);
/// assert_eq!(x, 158);
///
/// let mut x = -100000000000i64;
/// x.floor_root_assign(5);
/// assert_eq!(x, -159);
/// ```
///
/// # ceiling_root
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingRoot;
///
/// assert_eq!(999u16.ceiling_root(3), 10);
/// assert_eq!(1000u16.ceiling_root(3), 10);
/// assert_eq!(1001u16.ceiling_root(3), 11);
/// assert_eq!(100000000000i64.ceiling_root(5), 159);
/// assert_eq!((-100000000000i64).ceiling_root(5), -158);
/// ```
///
/// # ceiling_root_assign
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingRootAssign;
///
/// let mut x = 999u16;
/// x.ceiling_root_assign(3);
/// assert_eq!(x, 10);
///
/// let mut x = 1000u16;
/// x.ceiling_root_assign(3);
/// assert_eq!(x, 10);
///
/// let mut x = 1001u16;
/// x.ceiling_root_assign(3);
/// assert_eq!(x, 11);
///
/// let mut x = 100000000000i64;
/// x.ceiling_root_assign(5);
/// assert_eq!(x, 159);
///
/// let mut x = -100000000000i64;
/// x.ceiling_root_assign(5);
/// assert_eq!(x, -158);
/// ```
///
/// # checked_root
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedRoot;
///
/// assert_eq!(999u16.checked_root(3), None);
/// assert_eq!(1000u16.checked_root(3), Some(10));
/// assert_eq!(1001u16.checked_root(3), None);
/// assert_eq!(100000000000i64.checked_root(5), None);
/// assert_eq!((-100000000000i64).checked_root(5), None);
/// assert_eq!(10000000000i64.checked_root(5), Some(100));
/// assert_eq!((-10000000000i64).checked_root(5), Some(-100));
/// ```
///
/// # root_rem
/// ```
/// use malachite_base::num::arithmetic::traits::RootRem;
///
/// assert_eq!(999u16.root_rem(3), (9, 270));
/// assert_eq!(1000u16.root_rem(3), (10, 0));
/// assert_eq!(1001u16.root_rem(3), (10, 1));
/// assert_eq!(100000000000u64.root_rem(5), (158, 1534195232));
/// ```
///
/// # root_assign_rem
/// ```
/// use malachite_base::num::arithmetic::traits::RootAssignRem;
///
/// let mut x = 999u16;
/// assert_eq!(x.root_assign_rem(3), 270);
/// assert_eq!(x, 9);
///
/// let mut x = 1000u16;
/// assert_eq!(x.root_assign_rem(3), 0);
/// assert_eq!(x, 10);
///
/// let mut x = 1001u16;
/// assert_eq!(x.root_assign_rem(3), 1);
/// assert_eq!(x, 10);
///
/// let mut x = 100000000000u64;
/// assert_eq!(x.root_assign_rem(5), 1534195232);
/// assert_eq!(x, 158);
/// ```
pub mod root;
/// [`RotateLeft`](traits::RotateLeft), [`RotateLeftAssign`](traits::RotateLeftAssign),
/// [`RotateRight`](traits::RotateRight), and [`RotateRightAssign`](traits::RotateRightAssign),
/// traits for rotating a number's bits.
///
/// # rotate_left_assign
/// ```
/// use malachite_base::num::arithmetic::traits::RotateLeftAssign;
///
/// let mut x: u32 = 0xabcd6789;
/// x.rotate_left_assign(4);
/// assert_eq!(x, 0xbcd6789a);
///
/// x = 0xabcd6789;
/// x.rotate_left_assign(32);
/// assert_eq!(x, 0xabcd6789);
///
/// x = 0xabcd6789;
/// x.rotate_left_assign(36);
/// assert_eq!(x, 0xbcd6789a);
/// ```
///
/// # rotate_right_assign
/// ```
/// use malachite_base::num::arithmetic::traits::RotateRightAssign;
///
/// let mut x: u32 = 0xabcd6789;
/// x.rotate_right_assign(4);
/// assert_eq!(x, 0x9abcd678);
///
/// x = 0xabcd6789;
/// x.rotate_right_assign(32);
/// assert_eq!(x, 0xabcd6789);
///
/// x = 0xabcd6789;
/// x.rotate_right_assign(36);
/// assert_eq!(x, 0x9abcd678);
/// ```
pub mod rotate;
/// [`RoundToMultiple`](traits::RoundToMultiple) and
/// [`RoundToMultipleAssign`](traits::RoundToMultipleAssign), traits for rounding a number to a
/// multiple of another number.
///
/// # round_to_multiple
/// ```
/// use malachite_base::num::arithmetic::traits::RoundToMultiple;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use std::cmp::Ordering::*;
///
/// assert_eq!(5u32.round_to_multiple(0, Down), (0, Less));
///
/// assert_eq!(10u8.round_to_multiple(4, Down), (8, Less));
/// assert_eq!(10u16.round_to_multiple(4, Up), (12, Greater));
/// assert_eq!(10u32.round_to_multiple(5, Exact), (10, Equal));
/// assert_eq!(10u64.round_to_multiple(3, Nearest), (9, Less));
/// assert_eq!(20u128.round_to_multiple(3, Nearest), (21, Greater));
/// assert_eq!(10usize.round_to_multiple(4, Nearest), (8, Less));
/// assert_eq!(14u8.round_to_multiple(4, Nearest), (16, Greater));
///
/// assert_eq!((-5i32).round_to_multiple(0, Down), (0, Greater));
///
/// assert_eq!((-10i8).round_to_multiple(4, Down), (-8, Greater));
/// assert_eq!((-10i16).round_to_multiple(4, Up), (-12, Less));
/// assert_eq!((-10i32).round_to_multiple(5, Exact), (-10, Equal));
/// assert_eq!((-10i64).round_to_multiple(3, Nearest), (-9, Greater));
/// assert_eq!((-20i128).round_to_multiple(3, Nearest), (-21, Less));
/// assert_eq!((-10isize).round_to_multiple(4, Nearest), (-8, Greater));
/// assert_eq!((-14i8).round_to_multiple(4, Nearest), (-16, Less));
///
/// assert_eq!((-10i16).round_to_multiple(-4, Down), (-8, Greater));
/// assert_eq!((-10i32).round_to_multiple(-4, Up), (-12, Less));
/// assert_eq!((-10i64).round_to_multiple(-5, Exact), (-10, Equal));
/// assert_eq!((-10i128).round_to_multiple(-3, Nearest), (-9, Greater));
/// assert_eq!((-20isize).round_to_multiple(-3, Nearest), (-21, Less));
/// assert_eq!((-10i8).round_to_multiple(-4, Nearest), (-8, Greater));
/// assert_eq!((-14i16).round_to_multiple(-4, Nearest), (-16, Less));
/// ```
///
/// # round_to_multiple_assign
/// ```
/// use malachite_base::num::arithmetic::traits::RoundToMultipleAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use std::cmp::Ordering::*;
///
/// let mut x = 5u32;
/// assert_eq!(x.round_to_multiple_assign(0, Down), Less);
/// assert_eq!(x, 0);
///
/// let mut x = 10u8;
/// assert_eq!(x.round_to_multiple_assign(4, Down), Less);
/// assert_eq!(x, 8);
///
/// let mut x = 10u16;
/// assert_eq!(x.round_to_multiple_assign(4, Up), Greater);
/// assert_eq!(x, 12);
///
/// let mut x = 10u32;
/// assert_eq!(x.round_to_multiple_assign(5, Exact), Equal);
/// assert_eq!(x, 10);
///
/// let mut x = 10u64;
/// assert_eq!(x.round_to_multiple_assign(3, Nearest), Less);
/// assert_eq!(x, 9);
///
/// let mut x = 20u128;
/// assert_eq!(x.round_to_multiple_assign(3, Nearest), Greater);
/// assert_eq!(x, 21);
///
/// let mut x = 10usize;
/// assert_eq!(x.round_to_multiple_assign(4, Nearest), Less);
/// assert_eq!(x, 8);
///
/// let mut x = 14u8;
/// assert_eq!(x.round_to_multiple_assign(4, Nearest), Greater);
/// assert_eq!(x, 16);
///
/// let mut x = -5i32;
/// assert_eq!(x.round_to_multiple_assign(0, Down), Greater);
/// assert_eq!(x, 0);
///
/// let mut x = -10i8;
/// assert_eq!(x.round_to_multiple_assign(4, Down), Greater);
/// assert_eq!(x, -8);
///
/// let mut x = -10i16;
/// assert_eq!(x.round_to_multiple_assign(4, Up), Less);
/// assert_eq!(x, -12);
///
/// let mut x = -10i32;
/// assert_eq!(x.round_to_multiple_assign(5, Exact), Equal);
/// assert_eq!(x, -10);
///
/// let mut x = -10i64;
/// assert_eq!(x.round_to_multiple_assign(3, Nearest), Greater);
/// assert_eq!(x, -9);
///
/// let mut x = -20i128;
/// assert_eq!(x.round_to_multiple_assign(3, Nearest), Less);
/// assert_eq!(x, -21);
///
/// let mut x = -10isize;
/// assert_eq!(x.round_to_multiple_assign(4, Nearest), Greater);
/// assert_eq!(x, -8);
///
/// let mut x = -14i8;
/// assert_eq!(x.round_to_multiple_assign(4, Nearest), Less);
/// assert_eq!(x, -16);
///
/// let mut x = -10i16;
/// assert_eq!(x.round_to_multiple_assign(-4, Down), Greater);
/// assert_eq!(x, -8);
///
/// let mut x = -10i32;
/// assert_eq!(x.round_to_multiple_assign(-4, Up), Less);
/// assert_eq!(x, -12);
///
/// let mut x = -10i64;
/// assert_eq!(x.round_to_multiple_assign(-5, Exact), Equal);
/// assert_eq!(x, -10);
///
/// let mut x = -10i128;
/// assert_eq!(x.round_to_multiple_assign(-3, Nearest), Greater);
/// assert_eq!(x, -9);
///
/// let mut x = -20isize;
/// assert_eq!(x.round_to_multiple_assign(-3, Nearest), Less);
/// assert_eq!(x, -21);
///
/// let mut x = -10i8;
/// assert_eq!(x.round_to_multiple_assign(-4, Nearest), Greater);
/// assert_eq!(x, -8);
///
/// let mut x = -14i16;
/// assert_eq!(x.round_to_multiple_assign(-4, Nearest), Less);
/// assert_eq!(x, -16);
/// ```
pub mod round_to_multiple;
/// [`RoundToMultipleOfPowerOf2`](traits::RoundToMultipleOfPowerOf2) and
/// [`RoundToMultipleOfPowerOf2Assign`](traits::RoundToMultipleOfPowerOf2Assign), traits for
/// rounding a number to a multiple of a power of 2.
///
/// # round_to_multiple_of_power_of_2
/// ```
/// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use std::cmp::Ordering::*;
///
/// assert_eq!(10u8.round_to_multiple_of_power_of_2(2, Floor), (8, Less));
/// assert_eq!(
///     10u8.round_to_multiple_of_power_of_2(2, Ceiling),
///     (12, Greater)
/// );
/// assert_eq!(10u8.round_to_multiple_of_power_of_2(2, Down), (8, Less));
/// assert_eq!(10u8.round_to_multiple_of_power_of_2(2, Up), (12, Greater));
/// assert_eq!(10u8.round_to_multiple_of_power_of_2(2, Nearest), (8, Less));
/// assert_eq!(12u8.round_to_multiple_of_power_of_2(2, Exact), (12, Equal));
///
/// assert_eq!(
///     (-10i8).round_to_multiple_of_power_of_2(2, Floor),
///     (-12, Less)
/// );
/// assert_eq!(
///     (-10i8).round_to_multiple_of_power_of_2(2, Ceiling),
///     (-8, Greater)
/// );
/// assert_eq!(
///     (-10i8).round_to_multiple_of_power_of_2(2, Down),
///     (-8, Greater)
/// );
/// assert_eq!((-10i8).round_to_multiple_of_power_of_2(2, Up), (-12, Less));
/// assert_eq!(
///     (-10i8).round_to_multiple_of_power_of_2(2, Nearest),
///     (-8, Greater)
/// );
/// assert_eq!(
///     (-12i8).round_to_multiple_of_power_of_2(2, Exact),
///     (-12, Equal)
/// );
/// ```
///
/// # round_to_multiple_of_power_of_2_assign
/// ```
/// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2Assign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use std::cmp::Ordering::*;
///
/// let mut x = 10u8;
/// assert_eq!(x.round_to_multiple_of_power_of_2_assign(2, Floor), Less);
/// assert_eq!(x, 8);
///
/// let mut x = 10u8;
/// assert_eq!(
///     x.round_to_multiple_of_power_of_2_assign(2, Ceiling),
///     Greater
/// );
/// assert_eq!(x, 12);
///
/// let mut x = 10u8;
/// assert_eq!(x.round_to_multiple_of_power_of_2_assign(2, Down), Less);
/// assert_eq!(x, 8);
///
/// let mut x = 10u8;
/// assert_eq!(x.round_to_multiple_of_power_of_2_assign(2, Up), Greater);
/// assert_eq!(x, 12);
///
/// let mut x = 10u8;
/// assert_eq!(x.round_to_multiple_of_power_of_2_assign(2, Nearest), Less);
/// assert_eq!(x, 8);
///
/// let mut x = 12u8;
/// assert_eq!(x.round_to_multiple_of_power_of_2_assign(2, Exact), Equal);
/// assert_eq!(x, 12);
///
/// let mut x = -10i8;
/// assert_eq!(x.round_to_multiple_of_power_of_2_assign(2, Floor), Less);
/// assert_eq!(x, -12);
///
/// let mut x = -10i8;
/// assert_eq!(
///     x.round_to_multiple_of_power_of_2_assign(2, Ceiling),
///     Greater
/// );
/// assert_eq!(x, -8);
///
/// let mut x = -10i8;
/// assert_eq!(x.round_to_multiple_of_power_of_2_assign(2, Down), Greater);
/// assert_eq!(x, -8);
///
/// let mut x = -10i8;
/// assert_eq!(x.round_to_multiple_of_power_of_2_assign(2, Up), Less);
/// assert_eq!(x, -12);
///
/// let mut x = -10i8;
/// assert_eq!(
///     x.round_to_multiple_of_power_of_2_assign(2, Nearest),
///     Greater
/// );
/// assert_eq!(x, -8);
///
/// let mut x = -12i8;
/// assert_eq!(x.round_to_multiple_of_power_of_2_assign(2, Exact), Equal);
/// assert_eq!(x, -12);
/// ```
pub mod round_to_multiple_of_power_of_2;
/// [`SaturatingAbs`](traits::SaturatingAbs) and
/// [`SaturatingAbsAssign`](traits::SaturatingAbsAssign), traits for taking the absolute value of a
/// number and saturating at numeric bounds instead of overflowing.
///
/// # saturating_abs_assign
/// ```
/// use malachite_base::num::arithmetic::traits::SaturatingAbsAssign;
///
/// let mut x = 0i8;
/// x.saturating_abs_assign();
/// assert_eq!(x, 0);
///
/// let mut x = 100i64;
/// x.saturating_abs_assign();
/// assert_eq!(x, 100);
///
/// let mut x = -100i64;
/// x.saturating_abs_assign();
/// assert_eq!(x, 100);
///
/// let mut x = -128i8;
/// x.saturating_abs_assign();
/// assert_eq!(x, 127);
/// ```
pub mod saturating_abs;
/// [`SaturatingAdd`](traits::SaturatingAdd) and
/// [`SaturatingAddAssign`](traits::SaturatingAddAssign), traits for adding two numbers and
/// saturating at numeric bounds instead of overflowing.
///
/// # saturating_add_assign
/// ```
/// use malachite_base::num::arithmetic::traits::SaturatingAddAssign;
///
/// let mut x = 123u16;
/// x.saturating_add_assign(456);
/// assert_eq!(x, 579);
///
/// let mut x = 123u8;
/// x.saturating_add_assign(200);
/// assert_eq!(x, 255);
/// ```
pub mod saturating_add;
/// [`SaturatingAddMul`](traits::SaturatingAddMul) and
/// [`SaturatingAddMulAssign`](traits::SaturatingAddMulAssign), traits for adding the product of two
/// numbers to a number and saturating at numeric bounds instead of overflowing.
///
/// # saturating_add_mul
/// ```
/// use malachite_base::num::arithmetic::traits::SaturatingAddMul;
///
/// assert_eq!(2u8.saturating_add_mul(3, 7), 23);
/// assert_eq!(2u8.saturating_add_mul(20, 20), 255);
///
/// assert_eq!(127i8.saturating_add_mul(-2, 100), -73);
/// assert_eq!((-127i8).saturating_add_mul(-2, 100), -128);
/// ```
///
/// # saturating_add_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::SaturatingAddMulAssign;
///
/// let mut x = 2u8;
/// x.saturating_add_mul_assign(3, 7);
/// assert_eq!(x, 23);
///
/// let mut x = 2u8;
/// x.saturating_add_mul_assign(20, 20);
/// assert_eq!(x, 255);
///
/// let mut x = 127i8;
/// x.saturating_add_mul_assign(-2, 100);
/// assert_eq!(x, -73);
///
/// let mut x = -127i8;
/// x.saturating_add_mul_assign(-2, 100);
/// assert_eq!(x, -128);
/// ```
pub mod saturating_add_mul;
/// [`SaturatingMul`](traits::SaturatingMul) and
/// [`SaturatingMulAssign`](traits::SaturatingMulAssign), traits for multiplying two numbers and
/// saturating at numeric bounds instead of overflowing.
///
/// # saturating_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::SaturatingMulAssign;
///
/// let mut x = 123u16;
/// x.saturating_mul_assign(456);
/// assert_eq!(x, 56088);
///
/// let mut x = 123u8;
/// x.saturating_mul_assign(200);
/// assert_eq!(x, 255);
/// ```
pub mod saturating_mul;
/// [`SaturatingNeg`](traits::SaturatingNeg) and
/// [`SaturatingNegAssign`](traits::SaturatingNegAssign), traits for negating a number and
/// saturating at numeric bounds instead of overflowing.
///
/// # saturating_neg_assign
/// ```
/// use malachite_base::num::arithmetic::traits::SaturatingNegAssign;
///
/// let mut x = 0i8;
/// x.saturating_neg_assign();
/// assert_eq!(x, 0);
///
/// let mut x = 100i64;
/// x.saturating_neg_assign();
/// assert_eq!(x, -100);
///
/// let mut x = -100i64;
/// x.saturating_neg_assign();
/// assert_eq!(x, 100);
///
/// let mut x = -128i8;
/// x.saturating_neg_assign();
/// assert_eq!(x, 127);
/// ```
pub mod saturating_neg;
/// [`SaturatingPow`](traits::SaturatingPow) and
/// [`SaturatingPowAssign`](traits::SaturatingPowAssign), traits for raising a number to a power and
/// saturating at numeric bounds instead of overflowing.
///
/// # saturating_pow_assign
/// ```
/// use malachite_base::num::arithmetic::traits::SaturatingPowAssign;
///
/// let mut x = 3u8;
/// x.saturating_pow_assign(3);
/// assert_eq!(x, 27);
///
/// let mut x = -10i32;
/// x.saturating_pow_assign(9);
/// assert_eq!(x, -1000000000);
///
/// let mut x = -10i16;
/// x.saturating_pow_assign(9);
/// assert_eq!(x, -32768);
/// ```
pub mod saturating_pow;
/// [`SaturatingSquare`](traits::SaturatingSquare) and
/// [`SaturatingSquareAssign`](traits::SaturatingSquareAssign), traits for squaring a number and
/// saturating at numeric bounds instead of overflowing.
///
/// # saturating_square
/// ```
/// use malachite_base::num::arithmetic::traits::SaturatingSquare;
///
/// assert_eq!(3u8.saturating_square(), 9);
/// assert_eq!((-1000i32).saturating_square(), 1000000);
/// assert_eq!(1000u16.saturating_square(), u16::MAX);
/// ```
///
/// # saturating_square_assign
/// ```
/// use malachite_base::num::arithmetic::traits::SaturatingSquareAssign;
///
/// let mut x = 3u8;
/// x.saturating_square_assign();
/// assert_eq!(x, 9);
///
/// let mut x = -1000i32;
/// x.saturating_square_assign();
/// assert_eq!(x, 1000000);
///
/// let mut x = 1000u16;
/// x.saturating_square_assign();
/// assert_eq!(x, u16::MAX);
/// ```
pub mod saturating_square;
/// [`SaturatingSub`](traits::SaturatingSub) and
/// [`SaturatingSubAssign`](traits::SaturatingSubAssign), traits for subtracting two numbers and
/// saturating at numeric bounds instead of overflowing.
///
/// # saturating_sub_assign
/// ```
/// use malachite_base::num::arithmetic::traits::SaturatingSubAssign;
///
/// let mut x = 456u16;
/// x.saturating_sub_assign(123);
/// assert_eq!(x, 333);
///
/// let mut x = 123u16;
/// x.saturating_sub_assign(456);
/// assert_eq!(x, 0);
/// ```
pub mod saturating_sub;
/// [`SaturatingSubMul`](traits::SaturatingSubMul) and
/// [`SaturatingSubMulAssign`](traits::SaturatingSubMulAssign), traits for subtracting a number by
/// the product of two numbers and saturating at numeric bounds instead of overflowing.
///
/// # saturating_sub_mul
/// ```
/// use malachite_base::num::arithmetic::traits::SaturatingSubMul;
///
/// assert_eq!(60u8.saturating_sub_mul(5, 10), 10);
/// assert_eq!(2u8.saturating_sub_mul(10, 5), 0);
///
/// assert_eq!(127i8.saturating_sub_mul(2, 100), -73);
/// assert_eq!((-127i8).saturating_sub_mul(2, 100), -128);
/// ```
///
/// # saturating_sub_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::SaturatingSubMulAssign;
///
/// let mut x = 60u8;
/// x.saturating_sub_mul_assign(5, 10);
/// assert_eq!(x, 10);
///
/// let mut x = 2u8;
/// x.saturating_sub_mul_assign(10, 5);
/// assert_eq!(x, 0);
///
/// let mut x = 127i8;
/// x.saturating_sub_mul_assign(2, 100);
/// assert_eq!(x, -73);
///
/// let mut x = -127i8;
/// x.saturating_sub_mul_assign(2, 100);
/// assert_eq!(x, -128);
/// ```
pub mod saturating_sub_mul;
/// [`ShlRound`](traits::ShlRound) and [`ShlRoundAssign`](traits::ShlRoundAssign), traits for
/// multiplying a number by a power of 2 and rounding according to a specified
/// [`RoundingMode`](crate::rounding_modes::RoundingMode).
///
/// # shl_round
/// ```
/// use malachite_base::num::arithmetic::traits::ShlRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use std::cmp::Ordering::*;
///
/// assert_eq!(0x101u16.shl_round(-8i8, Down), (1, Less));
/// assert_eq!(0x101u32.shl_round(-8i16, Up), (2, Greater));
///
/// assert_eq!((-0x101i16).shl_round(-9i32, Down), (0, Greater));
/// assert_eq!((-0x101i32).shl_round(-9i64, Up), (-1, Less));
/// assert_eq!((-0x101i64).shl_round(-9i8, Nearest), (-1, Less));
/// assert_eq!((-0xffi32).shl_round(-9i16, Nearest), (0, Greater));
/// assert_eq!((-0x100i16).shl_round(-9i32, Nearest), (0, Greater));
///
/// assert_eq!(0x100u64.shl_round(-8i64, Exact), (1, Equal));
/// ```
///
/// # shl_round_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ShlRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use std::cmp::Ordering::*;
///
/// let mut x = 0x101u16;
/// assert_eq!(x.shl_round_assign(-8i8, Down), Less);
/// assert_eq!(x, 1);
///
/// let mut x = 0x101u32;
/// assert_eq!(x.shl_round_assign(-8i16, Up), Greater);
/// assert_eq!(x, 2);
///
/// let mut x = -0x101i16;
/// assert_eq!(x.shl_round_assign(-9i32, Down), Greater);
/// assert_eq!(x, 0);
///
/// let mut x = -0x101i32;
/// assert_eq!(x.shl_round_assign(-9i64, Up), Less);
/// assert_eq!(x, -1);
///
/// let mut x = -0x101i64;
/// assert_eq!(x.shl_round_assign(-9i8, Nearest), Less);
/// assert_eq!(x, -1);
///
/// let mut x = -0xffi32;
/// assert_eq!(x.shl_round_assign(-9i16, Nearest), Greater);
/// assert_eq!(x, 0);
///
/// let mut x = -0x100i16;
/// assert_eq!(x.shl_round_assign(-9i32, Nearest), Greater);
/// assert_eq!(x, 0);
///
/// let mut x = 0x100u64;
/// assert_eq!(x.shl_round_assign(-8i64, Exact), Equal);
/// assert_eq!(x, 1);
/// ```
pub mod shl_round;
/// [`ShrRound`](traits::ShrRound) and [`ShrRoundAssign`](traits::ShrRoundAssign), traits for
/// dividing a number by a power of 2 and rounding according to a specified
/// [`RoundingMode`](crate::rounding_modes::RoundingMode).
///
/// # shr_round
/// ```
/// use malachite_base::num::arithmetic::traits::ShrRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use std::cmp::Ordering::*;
///
/// assert_eq!(0x101u32.shr_round(8u8, Down), (1, Less));
/// assert_eq!(0x101u16.shr_round(8u16, Up), (2, Greater));
///
/// assert_eq!(0x101u64.shr_round(9u32, Down), (0, Less));
/// assert_eq!(0x101u32.shr_round(9u64, Up), (1, Greater));
/// assert_eq!(0x101u16.shr_round(9u8, Nearest), (1, Greater));
/// assert_eq!(0xffu8.shr_round(9u16, Nearest), (0, Less));
/// assert_eq!(0x100u32.shr_round(9u32, Nearest), (0, Less));
///
/// assert_eq!(0x100u32.shr_round(8u64, Exact), (1, Equal));
///
/// assert_eq!(0x101i32.shr_round(8u8, Down), (1, Less));
/// assert_eq!(0x101i16.shr_round(8u16, Up), (2, Greater));
///
/// assert_eq!((-0x101i32).shr_round(9u32, Down), (0, Greater));
/// assert_eq!((-0x101i64).shr_round(9u64, Up), (-1, Less));
/// assert_eq!((-0x101i16).shr_round(9u8, Nearest), (-1, Less));
/// assert_eq!((-0xffi32).shr_round(9u16, Nearest), (0, Greater));
/// assert_eq!((-0x100i64).shr_round(9u32, Nearest), (0, Greater));
///
/// assert_eq!(0x100i32.shr_round(8u64, Exact), (1, Equal));
///
/// assert_eq!(0x101u32.shr_round(8i8, Down), (1, Less));
/// assert_eq!(0x101u16.shr_round(8i16, Up), (2, Greater));
///
/// assert_eq!((-0x101i32).shr_round(9i32, Down), (0, Greater));
/// assert_eq!((-0x101i64).shr_round(9i64, Up), (-1, Less));
/// assert_eq!((-0x101i16).shr_round(9i8, Nearest), (-1, Less));
/// assert_eq!((-0xffi32).shr_round(9i16, Nearest), (0, Greater));
/// assert_eq!((-0x100i64).shr_round(9i32, Nearest), (0, Greater));
///
/// assert_eq!(0x100u32.shr_round(8i64, Exact), (1, Equal));
/// ```
///
/// # shr_round_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ShrRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use std::cmp::Ordering::*;
///
/// let mut x = 0x101u32;
/// assert_eq!(x.shr_round_assign(8u8, Down), Less);
/// assert_eq!(x, 1);
///
/// let mut x = 0x101u16;
/// assert_eq!(x.shr_round_assign(8u16, Up), Greater);
/// assert_eq!(x, 2);
///
/// let mut x = 0x101u64;
/// assert_eq!(x.shr_round_assign(9u32, Down), Less);
/// assert_eq!(x, 0);
///
/// let mut x = 0x101u32;
/// assert_eq!(x.shr_round_assign(9u64, Up), Greater);
/// assert_eq!(x, 1);
///
/// let mut x = 0x101u16;
/// assert_eq!(x.shr_round_assign(9u8, Nearest), Greater);
/// assert_eq!(x, 1);
///
/// let mut x = 0xffu8;
/// assert_eq!(x.shr_round_assign(9u16, Nearest), Less);
/// assert_eq!(x, 0);
///
/// let mut x = 0x100u32;
/// assert_eq!(x.shr_round_assign(9u32, Nearest), Less);
/// assert_eq!(x, 0);
///
/// let mut x = 0x100u32;
/// assert_eq!(x.shr_round_assign(8u64, Exact), Equal);
/// assert_eq!(x, 1);
///
/// let mut x = 0x101i32;
/// assert_eq!(x.shr_round_assign(8u8, Down), Less);
/// assert_eq!(x, 1);
///
/// let mut x = 0x101i16;
/// assert_eq!(x.shr_round_assign(8u16, Up), Greater);
/// assert_eq!(x, 2);
///
/// let mut x = -0x101i32;
/// assert_eq!(x.shr_round_assign(9u32, Down), Greater);
/// assert_eq!(x, 0);
///
/// let mut x = -0x101i64;
/// assert_eq!(x.shr_round_assign(9u64, Up), Less);
/// assert_eq!(x, -1);
///
/// let mut x = -0x101i16;
/// assert_eq!(x.shr_round_assign(9u8, Nearest), Less);
/// assert_eq!(x, -1);
///
/// let mut x = -0xffi32;
/// assert_eq!(x.shr_round_assign(9u16, Nearest), Greater);
/// assert_eq!(x, 0);
///
/// let mut x = -0x100i64;
/// assert_eq!(x.shr_round_assign(9u32, Nearest), Greater);
/// assert_eq!(x, 0);
///
/// let mut x = 0x100u32;
/// assert_eq!(x.shr_round_assign(8i64, Exact), Equal);
/// assert_eq!(x, 1);
///
/// let mut x = 0x101u32;
/// assert_eq!(x.shr_round_assign(8i8, Down), Less);
/// assert_eq!(x, 1);
///
/// let mut x = 0x101u16;
/// assert_eq!(x.shr_round_assign(8i16, Up), Greater);
/// assert_eq!(x, 2);
///
/// let mut x = -0x101i32;
/// assert_eq!(x.shr_round_assign(9i32, Down), Greater);
/// assert_eq!(x, 0);
///
/// let mut x = -0x101i64;
/// assert_eq!(x.shr_round_assign(9i64, Up), Less);
/// assert_eq!(x, -1);
///
/// let mut x = -0x101i16;
/// assert_eq!(x.shr_round_assign(9i8, Nearest), Less);
/// assert_eq!(x, -1);
///
/// let mut x = -0xffi32;
/// assert_eq!(x.shr_round_assign(9i16, Nearest), Greater);
/// assert_eq!(x, 0);
///
/// let mut x = -0x100i64;
/// assert_eq!(x.shr_round_assign(9i32, Nearest), Greater);
/// assert_eq!(x, 0);
///
/// let mut x = 0x100u32;
/// assert_eq!(x.shr_round_assign(8i64, Exact), Equal);
/// assert_eq!(x, 1);
/// ```
pub mod shr_round;
/// [`Sign`](traits::Sign), a trait for determining the sign of a number.
///
/// # sign
/// ```
/// use malachite_base::num::arithmetic::traits::Sign;
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use std::cmp::Ordering::*;
///
/// assert_eq!(0u8.sign(), Equal);
/// assert_eq!(100u64.sign(), Greater);
/// assert_eq!((-100i16).sign(), Less);
///
/// assert_eq!(0.0.sign(), Greater);
/// assert_eq!(1.0.sign(), Greater);
/// assert_eq!(f64::INFINITY.sign(), Greater);
///
/// assert_eq!((-0.0).sign(), Less);
/// assert_eq!((-1.0).sign(), Less);
/// assert_eq!(f64::NEGATIVE_INFINITY.sign(), Less);
///
/// assert_eq!(f64::NAN.sign(), Equal);
/// ```
pub mod sign;
/// Traits for taking the square root of a number.
///
/// The traits are [`FloorSqrt`](traits::FloorSqrt), [`FloorSqrtAssign`](traits::FloorSqrtAssign),
/// [`CeilingSqrt`](traits::CeilingSqrt), [`CeilingSqrtAssign`](traits::CeilingSqrtAssign),
/// [`CheckedSqrt`](traits::CheckedSqrt), [`SqrtRem`](traits::SqrtRem),
/// [`SqrtAssignRem`](traits::SqrtAssignRem), and [`SqrtAssign`](traits::SqrtAssign).
///
/// # floor_sqrt
/// ```
/// use malachite_base::num::arithmetic::traits::FloorSqrt;
///
/// assert_eq!(99u8.floor_sqrt(), 9);
/// assert_eq!(100u8.floor_sqrt(), 10);
/// assert_eq!(101u8.floor_sqrt(), 10);
/// assert_eq!(1000000000i32.floor_sqrt(), 31622);
/// assert_eq!(10000000000i64.floor_sqrt(), 100000);
/// ```
///
/// # floor_sqrt_assign
/// ```
/// use malachite_base::num::arithmetic::traits::FloorSqrtAssign;
///
/// let mut x = 99u8;
/// x.floor_sqrt_assign();
/// assert_eq!(x, 9);
///
/// let mut x = 100u8;
/// x.floor_sqrt_assign();
/// assert_eq!(x, 10);
///
/// let mut x = 101u8;
/// x.floor_sqrt_assign();
/// assert_eq!(x, 10);
///
/// let mut x = 1000000000i32;
/// x.floor_sqrt_assign();
/// assert_eq!(x, 31622);
///
/// let mut x = 10000000000i64;
/// x.floor_sqrt_assign();
/// assert_eq!(x, 100000);
/// ```
///
/// # ceiling_sqrt
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingSqrt;
///
/// assert_eq!(99u8.ceiling_sqrt(), 10);
/// assert_eq!(100u8.ceiling_sqrt(), 10);
/// assert_eq!(101u8.ceiling_sqrt(), 11);
/// assert_eq!(1000000000u32.ceiling_sqrt(), 31623);
/// assert_eq!(10000000000u64.ceiling_sqrt(), 100000);
/// ```
///
/// # ceiling_sqrt_assign
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingSqrtAssign;
///
/// let mut x = 99u8;
/// x.ceiling_sqrt_assign();
/// assert_eq!(x, 10);
///
/// let mut x = 100u8;
/// x.ceiling_sqrt_assign();
/// assert_eq!(x, 10);
///
/// let mut x = 101u8;
/// x.ceiling_sqrt_assign();
/// assert_eq!(x, 11);
///
/// let mut x = 1000000000i32;
/// x.ceiling_sqrt_assign();
/// assert_eq!(x, 31623);
///
/// let mut x = 10000000000i64;
/// x.ceiling_sqrt_assign();
/// assert_eq!(x, 100000);
/// ```
///
/// # checked_sqrt
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedSqrt;
///
/// assert_eq!(99u8.checked_sqrt(), None);
/// assert_eq!(100u8.checked_sqrt(), Some(10));
/// assert_eq!(101u8.checked_sqrt(), None);
/// assert_eq!(1000000000i32.checked_sqrt(), None);
/// assert_eq!(10000000000i64.checked_sqrt(), Some(100000));
/// ```
///
/// # sqrt_rem
/// ```
/// use malachite_base::num::arithmetic::traits::SqrtRem;
///
/// assert_eq!(99u8.sqrt_rem(), (9, 18));
/// assert_eq!(100u8.sqrt_rem(), (10, 0));
/// assert_eq!(101u8.sqrt_rem(), (10, 1));
/// assert_eq!(1000000000u32.sqrt_rem(), (31622, 49116));
/// assert_eq!(10000000000u64.sqrt_rem(), (100000, 0));
/// ```
///
/// # sqrt_assign_rem
/// ```
/// use malachite_base::num::arithmetic::traits::SqrtAssignRem;
///
/// let mut x = 99u8;
/// assert_eq!(x.sqrt_assign_rem(), 18);
/// assert_eq!(x, 9);
///
/// let mut x = 100u8;
/// assert_eq!(x.sqrt_assign_rem(), 0);
/// assert_eq!(x, 10);
///
/// let mut x = 101u8;
/// assert_eq!(x.sqrt_assign_rem(), 1);
/// assert_eq!(x, 10);
///
/// let mut x = 1000000000u32;
/// assert_eq!(x.sqrt_assign_rem(), 49116);
/// assert_eq!(x, 31622);
///
/// let mut x = 10000000000u64;
/// assert_eq!(x.sqrt_assign_rem(), 0);
/// assert_eq!(x, 100000);
/// ```
///
/// # sqrt_assign
/// ```
/// use malachite_base::num::arithmetic::traits::SqrtAssign;
/// use malachite_base::num::float::NiceFloat;
///
/// let mut x = 4.0f64;
/// x.sqrt_assign();
/// assert_eq!(NiceFloat(x), NiceFloat(2.0));
///
/// let mut x = 2.0f64;
/// x.sqrt_assign();
/// assert_eq!(NiceFloat(x), NiceFloat(std::f64::consts::SQRT_2));
/// ```
pub mod sqrt;
/// [`Square`](traits::Square) and [`SquareAssign`](traits::SquareAssign), traits for squaring a
/// number.
///
/// # square
/// ```
/// use malachite_base::num::arithmetic::traits::Square;
///
/// assert_eq!(3u8.square(), 9);
/// assert_eq!((-1000i32).square(), 1000000);
/// assert_eq!(1.5f32.square(), 2.25);
/// ```
///
/// # square_assign
/// ```
/// use malachite_base::num::arithmetic::traits::SquareAssign;
///
/// let mut x = 3u8;
/// x.square_assign();
/// assert_eq!(x, 9);
///
/// let mut x = -1000i32;
/// x.square_assign();
/// assert_eq!(x, 1000000);
///
/// let mut x = 1.5f32;
/// x.square_assign();
/// assert_eq!(x, 2.25);
/// ```
pub mod square;
/// [`SubMul`](traits::SubMul) and [`SubMulAssign`](traits::SubMulAssign), traits for subtracting
/// the product of two numbers from a number.
///
/// # sub_mul
/// ```
/// use malachite_base::num::arithmetic::traits::SubMul;
///
/// assert_eq!(60u32.sub_mul(5, 10), 10);
/// assert_eq!(127i8.sub_mul(2, 100), -73);
/// assert_eq!(1.0f32.sub_mul(2.0, 3.0), -5.0);
/// ```
///
/// # sub_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::SubMulAssign;
///
/// let mut x = 60u32;
/// x.sub_mul_assign(5, 10);
/// assert_eq!(x, 10);
///
/// let mut x = 127i8;
/// x.sub_mul_assign(2, 100);
/// assert_eq!(x, -73);
///
/// let mut x = 1.0f32;
/// x.sub_mul_assign(2.0, 3.0);
/// assert_eq!(x, -5.0);
/// ```
pub mod sub_mul;
/// Various traits for performing arithmetic operations on numbers.
pub mod traits;
/// [`WrappingAbs`](traits::WrappingAbs) and [`WrappingAbsAssign`](traits::WrappingAbsAssign),
/// traits for computing the absolute value of a number and wrapping at the boundary of the type.
///
/// # wrapping_abs_assign
/// ```
/// use malachite_base::num::arithmetic::traits::WrappingAbsAssign;
///
/// let mut x = 0i8;
/// x.wrapping_abs_assign();
/// assert_eq!(x, 0);
///
/// let mut x = 100i64;
/// x.wrapping_abs_assign();
/// assert_eq!(x, 100);
///
/// let mut x = -100i64;
/// x.wrapping_abs_assign();
/// assert_eq!(x, 100);
///
/// let mut x = -128i8;
/// x.wrapping_abs_assign();
/// assert_eq!(x, -128);
/// ```
pub mod wrapping_abs;
/// [`WrappingAdd`](traits::WrappingAdd) and [`WrappingAddAssign`](traits::WrappingAddAssign),
/// traits for adding two numbers and wrapping at the boundary of the type.
///
/// # wrapping_add_assign
/// ```
/// use malachite_base::num::arithmetic::traits::WrappingAddAssign;
///
/// let mut x = 123u16;
/// x.wrapping_add_assign(456);
/// assert_eq!(x, 579);
///
/// let mut x = 123u8;
/// x.wrapping_add_assign(200);
/// assert_eq!(x, 67);
/// ```
pub mod wrapping_add;
/// [`WrappingAddMul`](traits::WrappingAddMul) and
/// [`WrappingAddMulAssign`](traits::WrappingAddMulAssign), traits for adding the product of two
/// numbers to a third and wrapping at the boundary of the type.
///
/// # wrapping_add_mul
/// ```
/// use malachite_base::num::arithmetic::traits::WrappingAddMul;
///
/// assert_eq!(2u8.wrapping_add_mul(3, 7), 23);
/// assert_eq!((-127i8).wrapping_add_mul(-2, 100), -71);
/// ```
///
/// # wrapping_add_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::WrappingAddMulAssign;
///
/// let mut x = 2u8;
/// x.wrapping_add_mul_assign(3, 7);
/// assert_eq!(x, 23);
///
/// let mut x = -127i8;
/// x.wrapping_add_mul_assign(-2, 100);
/// assert_eq!(x, -71);
/// ```
pub mod wrapping_add_mul;
/// [`WrappingDiv`](traits::WrappingDiv) and [`WrappingDivAssign`](traits::WrappingDivAssign),
/// traits for dividing two numbers and wrapping at the boundary of the type.
///
/// # wrapping_div_assign
/// ```
/// use malachite_base::num::arithmetic::traits::WrappingDivAssign;
///
/// let mut x = 100u16;
/// x.wrapping_div_assign(3);
/// assert_eq!(x, 33);
///
/// let mut x = -128i8;
/// x.wrapping_div_assign(-1);
/// assert_eq!(x, -128);
/// ```
pub mod wrapping_div;
/// [`WrappingMul`](traits::WrappingMul) and [`WrappingMulAssign`](traits::WrappingMulAssign),
/// traits for multiplying two numbers and wrapping at the boundary of the type.
///
/// # wrapping_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::WrappingMulAssign;
///
/// let mut x = 123u16;
/// x.wrapping_mul_assign(456);
/// assert_eq!(x, 56088);
///
/// let mut x = 123u8;
/// x.wrapping_mul_assign(200);
/// assert_eq!(x, 24);
/// ```
pub mod wrapping_mul;
/// [`WrappingNeg`](traits::WrappingNeg) and [`WrappingNegAssign`](traits::WrappingNegAssign) for
/// negating a number and wrapping at the boundary of the type.
///
/// # wrapping_neg_assign
/// ```
/// use malachite_base::num::arithmetic::traits::WrappingNegAssign;
///
/// let mut x = 0i8;
/// x.wrapping_neg_assign();
/// assert_eq!(x, 0);
///
/// let mut x = 100u64;
/// x.wrapping_neg_assign();
/// assert_eq!(x, 18446744073709551516);
///
/// let mut x = -100i64;
/// x.wrapping_neg_assign();
/// assert_eq!(x, 100);
///
/// let mut x = -128i8;
/// x.wrapping_neg_assign();
/// assert_eq!(x, -128);
/// ```
pub mod wrapping_neg;
/// [`WrappingPow`](traits::WrappingPow) and [`WrappingPowAssign`](traits::WrappingPowAssign),
/// traits for raising a number to a power and wrapping at the boundary of the type.
///
/// # wrapping_pow_assign
/// ```
/// use malachite_base::num::arithmetic::traits::WrappingPowAssign;
///
/// let mut x = 3u8;
/// x.wrapping_pow_assign(3);
/// assert_eq!(x, 27);
///
/// let mut x = -10i32;
/// x.wrapping_pow_assign(9);
/// assert_eq!(x, -1000000000);
///
/// let mut x = -10i16;
/// x.wrapping_pow_assign(9);
/// assert_eq!(x, 13824);
/// ```
pub mod wrapping_pow;
/// [`WrappingSquare`](traits::WrappingSquare) and
/// [`WrappingSquareAssign`](traits::WrappingAbsAssign), traits for squaring a number and wrapping
/// at the boundary of the type.
///
/// # wrapping_square
/// ```
/// use malachite_base::num::arithmetic::traits::WrappingSquare;
///
/// assert_eq!(3u8.wrapping_square(), 9);
/// assert_eq!((-1000i32).wrapping_square(), 1000000);
/// assert_eq!(1000u16.wrapping_square(), 16960);
/// ```
///
/// # wrapping_square_assign
/// ```
/// use malachite_base::num::arithmetic::traits::WrappingSquareAssign;
///
/// let mut x = 3u8;
/// x.wrapping_square_assign();
/// assert_eq!(x, 9);
///
/// let mut x = -1000i32;
/// x.wrapping_square_assign();
/// assert_eq!(x, 1000000);
///
/// let mut x = 1000u16;
/// x.wrapping_square_assign();
/// assert_eq!(x, 16960);
/// ```
pub mod wrapping_square;
/// [`WrappingSub`](traits::WrappingSub) and [`WrappingSubAssign`](traits::WrappingSubAssign),
/// traits for subtracting two numbers and wrapping at the boundary of the type.
///
/// # wrapping_sub_assign
/// ```
/// use malachite_base::num::arithmetic::traits::WrappingSubAssign;
///
/// let mut x = 456u16;
/// x.wrapping_sub_assign(123);
/// assert_eq!(x, 333);
///
/// let mut x = 123u16;
/// x.wrapping_sub_assign(456);
/// assert_eq!(x, 65203);
/// ```
pub mod wrapping_sub;
/// [`WrappingSubMul`](traits::WrappingSubMul) and
/// [`WrappingSubMulAssign`](traits::WrappingSubMulAssign), traits for subtracting a number by the
/// product of two other numbers and wrapping at the boundary of the type.
///
/// # wrapping_sub_mul
/// ```
/// use malachite_base::num::arithmetic::traits::WrappingSubMul;
///
/// assert_eq!(127i8.wrapping_sub_mul(2, 100), -73);
/// assert_eq!((-127i8).wrapping_sub_mul(2, 100), -71);
/// ```
///
/// # wrapping_sub_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::WrappingAddMulAssign;
///
/// let mut x = 2u8;
/// x.wrapping_add_mul_assign(3, 7);
/// assert_eq!(x, 23);
///
/// let mut x = -127i8;
/// x.wrapping_add_mul_assign(-2, 100);
/// assert_eq!(x, -71);
/// ```
pub mod wrapping_sub_mul;
/// [`XMulYToZZ`](traits::XMulYToZZ), a trait for multiplying two numbers and returning the result
/// as a double-width number.
///
/// # x_mul_y_to_zz
/// ```
/// use malachite_base::num::arithmetic::traits::XMulYToZZ;
///
/// assert_eq!(u64::x_mul_y_to_zz(15, 3), (0, 45));
/// assert_eq!(u8::x_mul_y_to_zz(0x78, 0x9a), (0x48, 0x30));
/// ```
pub mod x_mul_y_to_zz;
/// [`XXAddYYToZZ`](traits::XXAddYYToZZ), a trait for adding two double-width numbers and returning
/// the result as a double-width number.
///
/// # xx_add_yy_to_zz
/// ```
/// use malachite_base::num::arithmetic::traits::XXAddYYToZZ;
///
/// assert_eq!(u64::xx_add_yy_to_zz(0x12, 0x34, 0x33, 0x33), (0x45, 0x67));
/// assert_eq!(u8::xx_add_yy_to_zz(0x78, 0x9a, 0xbc, 0xde), (0x35, 0x78));
/// ```
pub mod xx_add_yy_to_zz;
/// [`XXDivModYToQR`](traits::XXDivModYToQR), a trait for dividing a double-width number by a
/// single-width number and returning the quotient and remainder.
///
/// # xx_div_mod_y_to_qr
/// ```
/// use malachite_base::num::arithmetic::traits::XXDivModYToQR;
///
/// assert_eq!(
///     u64::xx_div_mod_y_to_qr(0x12, 0x34, 0x33),
///     (0x5a5a5a5a5a5a5a5b, 0x13)
/// );
/// assert_eq!(u8::xx_div_mod_y_to_qr(0x78, 0x9a, 0xbc), (0xa4, 0x2a));
/// ```
pub mod xx_div_mod_y_to_qr;
/// [`XXSubYYToZZ`](traits::XXSubYYToZZ), a trait for subtracting two double-width numbers and
/// returning the result as a double-width number.
///
/// # xx_sub_yy_to_zz
/// ```
/// use malachite_base::num::arithmetic::traits::XXSubYYToZZ;
///
/// assert_eq!(u64::xx_sub_yy_to_zz(0x67, 0x89, 0x33, 0x33), (0x34, 0x56));
/// assert_eq!(u8::xx_sub_yy_to_zz(0x78, 0x9a, 0xbc, 0xde), (0xbb, 0xbc));
/// ```
pub mod xx_sub_yy_to_zz;
/// [`XXXAddYYYToZZZ`](traits::XXXAddYYYToZZZ), a trait for adding two triple-width numbers and
/// returning the result as a triple-width number.
///
/// # xxx_add_yyy_to_zzz
/// ```
/// use malachite_base::num::arithmetic::traits::XXXAddYYYToZZZ;
///
/// assert_eq!(
///     u64::xxx_add_yyy_to_zzz(0x12, 0x34, 0x56, 0x33, 0x33, 0x33),
///     (0x45, 0x67, 0x89)
/// );
/// assert_eq!(
///     u8::xxx_add_yyy_to_zzz(0x78, 0x9a, 0xbc, 0xde, 0xfe, 0xdc),
///     (0x57, 0x99, 0x98)
/// );
/// ```
pub mod xxx_add_yyy_to_zzz;
/// [`XXXSubYYYToZZZ`](traits::XXXSubYYYToZZZ), a trait for subtracting two triple-width numbers and
/// returning the result as a triple-width number.
///
/// # xxx_sub_yyy_to_zzz
/// ```
/// use malachite_base::num::arithmetic::traits::XXXSubYYYToZZZ;
///
/// assert_eq!(
///     u64::xxx_sub_yyy_to_zzz(0x67, 0x89, 0xab, 0x33, 0x33, 0x33),
///     (0x34, 0x56, 0x78)
/// );
/// assert_eq!(
///     u8::xxx_sub_yyy_to_zzz(0x78, 0x9a, 0xbc, 0xde, 0xfe, 0xdc),
///     (0x99, 0x9b, 0xe0)
/// );
/// ```
pub mod xxx_sub_yyy_to_zzz;
/// [`XXXXAddYYYYToZZZZ`](traits::XXXXAddYYYYToZZZZ), a trait for adding two quadruple-width numbers
/// and returning the result as a quadruple-width number.
///
/// # xxxx_add_yyyy_to_zzzz
/// ```
/// use malachite_base::num::arithmetic::traits::XXXXAddYYYYToZZZZ;
///
/// assert_eq!(
///     u64::xxxx_add_yyyy_to_zzzz(0x12, 0x34, 0x56, 0x78, 0x33, 0x33, 0x33, 0x33),
///     (0x45, 0x67, 0x89, 0xab)
/// );
/// assert_eq!(
///     u8::xxxx_add_yyyy_to_zzzz(0x78, 0x9a, 0xbc, 0xde, 0xfe, 0xdc, 0xba, 0x98),
///     (0x77, 0x77, 0x77, 0x76)
/// );
/// ```
pub mod xxxx_add_yyyy_to_zzzz;
