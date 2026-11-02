// Copyright © 2026 Mikhail Hogrefe
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
/// let mut x = f64::NEG_INFINITY;
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
/// [`AbsSquared`](traits::AbsSquared) and [`AbsSquaredAssign`](traits::AbsSquaredAssign), traits
/// for computing the squared absolute value of a number.
///
/// # abs_squared
/// ```
/// use malachite_base::num::arithmetic::traits::AbsSquared;
///
/// assert_eq!(12u8.abs_squared(), 144);
/// assert_eq!(123u32.abs_squared(), 15129);
/// assert_eq!((-123i32).abs_squared(), 15129);
/// assert_eq!(1.5f32.abs_squared(), 2.25);
/// assert_eq!((-1.5f32).abs_squared(), 2.25);
/// ```
///
/// # abs_squared_assign
/// ```
/// use malachite_base::num::arithmetic::traits::AbsSquaredAssign;
///
/// let mut x = -123i32;
/// x.abs_squared_assign();
/// assert_eq!(x, 15129);
///
/// let mut x = -1.5f32;
/// x.abs_squared_assign();
/// assert_eq!(x, 2.25);
/// ```
pub mod abs_squared;
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
/// [`Average`](traits::Average), [`AverageAssign`](traits::AverageAssign),
/// [`AverageRound`](traits::AverageRound), and [`AverageRoundAssign`](traits::AverageRoundAssign),
/// traits for computing the average (arithmetic mean) of two numbers without overflow.
///
/// # average
/// ```
/// use malachite_base::num::arithmetic::traits::Average;
///
/// assert_eq!(4u8.average(6), 5);
/// // 4.5 rounds to the even neighbor, 4
/// assert_eq!(4u8.average(5), 4);
/// // 5.5 rounds to the even neighbor, 6
/// assert_eq!(5u8.average(6), 6);
/// assert_eq!((-5i8).average(-6), -6);
/// assert_eq!(u8::MAX.average(u8::MAX - 2), 254);
///
/// // for floats, the average is correctly rounded and cannot overflow
/// assert_eq!(1.0f64.average(2.0), 1.5);
/// assert_eq!(f64::MAX.average(f64::MAX), f64::MAX);
/// assert_eq!(f64::MAX.average(0.0), f64::MAX / 2.0);
/// assert_eq!(f32::INFINITY.average(1.0), f32::INFINITY);
/// assert!(f32::INFINITY.average(f32::NEG_INFINITY).is_nan());
/// ```
///
/// # average_assign
/// ```
/// use malachite_base::num::arithmetic::traits::AverageAssign;
///
/// let mut x = 4u8;
/// x.average_assign(6);
/// assert_eq!(x, 5);
///
/// let mut x = -5i8;
/// x.average_assign(-6);
/// assert_eq!(x, -6);
///
/// let mut x = 1.0f64;
/// x.average_assign(2.0);
/// assert_eq!(x, 1.5);
/// ```
///
/// # average_round
/// ```
/// use malachite_base::num::arithmetic::traits::AverageRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use std::cmp::Ordering::*;
///
/// assert_eq!(4u8.average_round(7, Floor), (5, Less));
/// assert_eq!(4u8.average_round(7, Ceiling), (6, Greater));
/// assert_eq!(4u8.average_round(7, Down), (5, Less));
/// assert_eq!(4u8.average_round(6, Exact), (5, Equal));
///
/// // the exact average is -5.5
/// assert_eq!((-4i8).average_round(-7, Down), (-5, Greater));
/// assert_eq!((-4i8).average_round(-7, Up), (-6, Less));
///
/// // the exact average is -0.5
/// assert_eq!(i8::MIN.average_round(i8::MAX, Floor), (-1, Less));
/// assert_eq!(i8::MIN.average_round(i8::MAX, Nearest), (0, Greater));
/// ```
///
/// # average_round_assign
/// ```
/// use malachite_base::num::arithmetic::traits::AverageRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use std::cmp::Ordering::*;
///
/// let mut x = 4u8;
/// assert_eq!(x.average_round_assign(7, Floor), Less);
/// assert_eq!(x, 5);
///
/// let mut x = -4i8;
/// assert_eq!(x.average_round_assign(-7, Up), Less);
/// assert_eq!(x, -6);
/// ```
pub mod average;
/// [`CanonicalUnitIPow`](traits::CanonicalUnitIPow), a trait for finding the power of $i$ that
/// brings a number into canonical Gaussian-unit form.
///
/// # canonical_unit_i_pow
/// ```
/// use malachite_base::num::arithmetic::traits::CanonicalUnitIPow;
///
/// assert_eq!(123u32.canonical_unit_i_pow(), 0);
/// assert_eq!(123i32.canonical_unit_i_pow(), 0);
/// assert_eq!((-123i32).canonical_unit_i_pow(), 2);
/// assert_eq!((-0.0f64).canonical_unit_i_pow(), 2);
/// assert_eq!(f64::NAN.canonical_unit_i_pow(), 0);
/// ```
pub mod canonical_unit_i_pow;
/// [`CanonicalizeGaussianUnit`](traits::CanonicalizeGaussianUnit) and
/// [`CanonicalizeGaussianUnitAssign`](traits::CanonicalizeGaussianUnitAssign), traits for bringing
/// a number into canonical Gaussian-unit form: for a real number, its absolute value.
///
/// # canonicalize_gaussian_unit
/// ```
/// use malachite_base::num::arithmetic::traits::CanonicalizeGaussianUnit;
///
/// assert_eq!(123u32.canonicalize_gaussian_unit(), 123);
/// assert_eq!((-123i32).canonicalize_gaussian_unit(), 123);
/// assert_eq!((-1.5f64).canonicalize_gaussian_unit(), 1.5);
/// assert_eq!((-0.0f64).canonicalize_gaussian_unit(), 0.0);
/// assert_eq!(
///     f64::NEG_INFINITY.canonicalize_gaussian_unit(),
///     f64::INFINITY
/// );
/// ```
///
/// # canonicalize_gaussian_unit_assign
/// ```
/// use malachite_base::num::arithmetic::traits::CanonicalizeGaussianUnitAssign;
///
/// let mut x = -123i32;
/// x.canonicalize_gaussian_unit_assign();
/// assert_eq!(x, 123);
/// ```
pub mod canonicalize_gaussian_unit;
/// [`CanonicalizeUnit`](traits::CanonicalizeUnit) and
/// [`CanonicalizeUnitAssign`](traits::CanonicalizeUnitAssign), traits for bringing a number into
/// canonical unit form.
///
/// # canonicalize_unit
/// ```
/// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
///
/// assert_eq!(123u32.canonicalize_unit(), 123);
/// assert_eq!((-123i32).canonicalize_unit(), 123);
/// assert_eq!((-1.5f64).canonicalize_unit(), 1.0);
/// assert_eq!((-0.0f64).canonicalize_unit(), 0.0);
/// assert_eq!(f64::NEG_INFINITY.canonicalize_unit(), f64::INFINITY);
/// ```
///
/// # canonicalize_unit_assign
/// ```
/// use malachite_base::num::arithmetic::traits::CanonicalizeUnitAssign;
///
/// let mut x = -123i32;
/// x.canonicalize_unit_assign();
/// assert_eq!(x, 123);
/// ```
pub mod canonicalize_unit;
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
/// [`CheckedMulAddMul`](traits::CheckedMulAddMul), a trait for adding the products of two pairs of
/// numbers.
///
/// # checked_mul_add_mul
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedMulAddMul;
///
/// assert_eq!(2u8.checked_mul_add_mul(3, 4, 5), Some(26));
/// assert_eq!(200u8.checked_mul_add_mul(200, 100, 100), None);
/// assert_eq!(10i8.checked_mul_add_mul(-2, 3, 5), Some(-5));
/// // Neither product fits in an i8, but their sum does.
/// assert_eq!(100i8.checked_mul_add_mul(100, -99, 100), Some(100));
/// ```
pub mod checked_mul_add_mul;
/// [`CheckedMulSubMul`](traits::CheckedMulSubMul), a trait for subtracting the product of one pair
/// of numbers from the product of another.
///
/// # checked_mul_sub_mul
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedMulSubMul;
///
/// assert_eq!(10u8.checked_mul_sub_mul(3, 4, 5), Some(10));
/// assert_eq!(1u8.checked_mul_sub_mul(1, 2, 2), None);
/// assert_eq!(2i8.checked_mul_sub_mul(3, 4, 5), Some(-14));
/// // Neither product fits in an i8, but their difference does.
/// assert_eq!(100i8.checked_mul_sub_mul(100, 99, 100), Some(100));
/// ```
pub mod checked_mul_sub_mul;
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
/// [`Conjugate`](traits::Conjugate) and [`ConjugateAssign`](traits::ConjugateAssign), traits for
/// computing the complex conjugate of a number. A real number is its own conjugate.
///
/// # conjugate
/// ```
/// use malachite_base::num::arithmetic::traits::Conjugate;
///
/// assert_eq!(123u32.conjugate(), 123);
/// assert_eq!((-123i32).conjugate(), -123);
/// assert_eq!(1.5f32.conjugate(), 1.5);
/// ```
///
/// # conjugate_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ConjugateAssign;
///
/// let mut x = -123i32;
/// x.conjugate_assign();
/// assert_eq!(x, -123);
/// ```
pub mod conjugate;
/// [`DivEuclidean`](traits::DivEuclidean) and [`DivEuclideanAssign`](traits::DivEuclideanAssign),
/// traits for finding the quotient of two numbers, rounded so that the remainder would be
/// nonnegative.
///
/// # div_euclidean
/// ```
/// use malachite_base::num::arithmetic::traits::DivEuclidean;
///
/// // 2 * 10 + 3 = 23
/// assert_eq!(23u8.div_euclidean(10), 2);
///
/// // 2 * 10 + 3 = 23
/// assert_eq!(23i8.div_euclidean(10), 2);
///
/// // -2 * -10 + 3 = 23
/// assert_eq!(23i16.div_euclidean(-10), -2);
///
/// // -3 * 10 + 7 = -23
/// assert_eq!((-23i32).div_euclidean(10), -3);
///
/// // 3 * -10 + 7 = -23
/// assert_eq!((-23i64).div_euclidean(-10), 3);
/// ```
///
/// # div_euclidean_assign
/// ```
/// use malachite_base::num::arithmetic::traits::DivEuclideanAssign;
///
/// // 2 * 10 + 3 = 23
/// let mut x = 23u8;
/// x.div_euclidean_assign(10);
/// assert_eq!(x, 2);
///
/// // -3 * 10 + 7 = -23
/// let mut x = -23i32;
/// x.div_euclidean_assign(10);
/// assert_eq!(x, -3);
/// ```
pub mod div_euclidean;
/// [`DivExact`](traits::DivExact) and [`DivExactAssign`](traits::DivExactAssign), traits for
/// dividing two numbers when it's known that the division is exact.
///
/// # div_exact
/// ```
/// use malachite_base::num::arithmetic::traits::DivExact;
///
/// // 123 * 456 = 56088
/// #[allow(unstable_name_collisions)]
/// {
///     assert_eq!(56088u32.div_exact(456), 123);
/// }
///
/// // -123 * -456 = 56088
/// #[allow(unstable_name_collisions)]
/// {
///     assert_eq!(56088i64.div_exact(-456), -123);
/// }
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
///
/// # div_mod_precomputed
/// ```
/// use malachite_base::num::arithmetic::traits::DivModPrecomputed;
///
/// let data = u32::precompute_div_mod_data(&10);
/// assert_eq!(23u32.div_mod_precomputed(10, &data), (2, 3));
/// assert_eq!(125u32.div_mod_precomputed(10, &data), (12, 5));
///
/// let data = u64::precompute_div_mod_data(&123);
/// assert_eq!(456u64.div_mod_precomputed(123, &data), (3, 87));
///
/// let data = u8::precompute_div_mod_data(&10);
/// assert_eq!(23u8.div_mod_precomputed(10, &data), (2, 3));
///
/// let data = u16::precompute_div_mod_data(&10);
/// assert_eq!(23u16.div_mod_precomputed(10, &data), (2, 3));
///
/// let data = u128::precompute_div_mod_data(&10);
/// assert_eq!(23u128.div_mod_precomputed(10, &data), (2, 3));
///
/// let data = i32::precompute_div_mod_data(&10);
/// assert_eq!(23i32.div_mod_precomputed(10, &data), (2, 3));
/// assert_eq!((-23i32).div_mod_precomputed(10, &data), (-3, 7));
///
/// let data = i64::precompute_div_mod_data(&-10);
/// assert_eq!(23i64.div_mod_precomputed(-10, &data), (-3, -7));
/// assert_eq!((-23i64).div_mod_precomputed(-10, &data), (2, -3));
/// ```
///
/// # div_assign_mod_precomputed
/// ```
/// use malachite_base::num::arithmetic::traits::{DivAssignModPrecomputed, DivModPrecomputed};
///
/// let data = u32::precompute_div_mod_data(&10);
/// let mut x = 23u32;
/// assert_eq!(x.div_assign_mod_precomputed(10, &data), 3);
/// assert_eq!(x, 2);
///
/// let data = i64::precompute_div_mod_data(&10);
/// let mut x = -23i64;
/// assert_eq!(x.div_assign_mod_precomputed(10, &data), 7);
/// assert_eq!(x, -3);
/// ```
#[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]
pub mod div_mod;
/// [`DivModEuclidean`](traits::DivModEuclidean) and
/// [`DivAssignModEuclidean`](traits::DivAssignModEuclidean), traits for simultaneously finding the
/// quotient and remainder of two numbers, where the remainder is always nonnegative.
///
/// # div_mod_euclidean
/// ```
/// use malachite_base::num::arithmetic::traits::DivModEuclidean;
///
/// // 2 * 10 + 3 = 23
/// assert_eq!(23u8.div_mod_euclidean(10), (2, 3));
///
/// // 9 * 5 + 0 = 45
/// assert_eq!(45u32.div_mod_euclidean(5), (9, 0));
///
/// // 2 * 10 + 3 = 23
/// assert_eq!(23i8.div_mod_euclidean(10), (2, 3));
///
/// // -2 * -10 + 3 = 23
/// assert_eq!(23i16.div_mod_euclidean(-10), (-2, 3));
///
/// // -3 * 10 + 7 = -23
/// assert_eq!((-23i32).div_mod_euclidean(10), (-3, 7));
///
/// // 3 * -10 + 7 = -23
/// assert_eq!((-23i64).div_mod_euclidean(-10), (3, 7));
/// ```
///
/// # div_assign_mod_euclidean
/// ```
/// use malachite_base::num::arithmetic::traits::DivAssignModEuclidean;
///
/// // 2 * 10 + 3 = 23
/// let mut x = 23u8;
/// assert_eq!(x.div_assign_mod_euclidean(10), 3);
/// assert_eq!(x, 2);
///
/// // -3 * 10 + 7 = -23
/// let mut x = -23i32;
/// assert_eq!(x.div_assign_mod_euclidean(10), 7);
/// assert_eq!(x, -3);
///
/// // 3 * -10 + 7 = -23
/// let mut x = -23i64;
/// assert_eq!(x.div_assign_mod_euclidean(-10), 7);
/// assert_eq!(x, 3);
/// ```
pub mod div_mod_euclidean;
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
/// [`IsPowerOf2`](traits::IsPowerOf2), a trait for determining whether a number is an integer power
/// of 2.
///
/// # is_power_of_2
/// ```
/// use malachite_base::num::arithmetic::traits::IsPowerOf2;
///
/// assert_eq!(4i32.is_power_of_2(), true);
/// assert_eq!(5i32.is_power_of_2(), false);
/// assert_eq!((-4i32).is_power_of_2(), false);
///
/// assert_eq!(4.0.is_power_of_2(), true);
/// assert_eq!(0.25.is_power_of_2(), true);
/// assert_eq!(0.2.is_power_of_2(), false);
/// assert_eq!((-4.0).is_power_of_2(), false);
/// ```
pub mod is_power_of_2;
/// [`IsUnit`](traits::IsUnit), a trait for determining whether a number is a unit of its ring.
///
/// # is_unit
/// ```
/// use malachite_base::num::arithmetic::traits::IsUnit;
///
/// assert_eq!(1u32.is_unit(), true);
/// assert_eq!(2u32.is_unit(), false);
/// assert_eq!((-1i32).is_unit(), true);
/// assert_eq!(0i32.is_unit(), false);
/// assert_eq!(0.5f64.is_unit(), true);
/// assert_eq!(0.0f64.is_unit(), false);
/// assert_eq!(f64::INFINITY.is_unit(), false);
/// ```
pub mod is_unit;
/// [`MulAddMul`](traits::MulAddMul) and [`MulAddMulAssign`](traits::MulAddMulAssign), traits for
/// adding the products of two pairs of numbers.
///
/// # mul_add_mul
/// ```
/// use malachite_base::num::arithmetic::traits::MulAddMul;
///
/// assert_eq!(2u8.mul_add_mul(3, 4, 5), 26);
/// assert_eq!(10i8.mul_add_mul(-2, 3, 5), -5);
/// ```
///
/// # mul_add_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
///
/// let mut x = 2u8;
/// x.mul_add_mul_assign(3, 4, 5);
/// assert_eq!(x, 26);
///
/// let mut x = 10i8;
/// x.mul_add_mul_assign(-2, 3, 5);
/// assert_eq!(x, -5);
/// ```
pub mod mul_add_mul;
/// Implementations of [`MulShrRound`](traits::MulShrRound) and
/// [`MulShrRoundAssign`](traits::MulShrRoundAssign), traits for multiplying two numbers and
/// right-shifting the product (dividing it by a power of 2) with a specified rounding mode. The
/// product is computed at twice the width of the type, so the operation is exact even when the
/// product itself would overflow.
///
/// # mul_shr_round
/// ```
/// use malachite_base::num::arithmetic::traits::MulShrRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use std::cmp::Ordering::*;
///
/// assert_eq!(100u8.mul_shr_round(200, 8u32, Down), (78, Less));
/// assert_eq!(100u8.mul_shr_round(200, 8u32, Up), (79, Greater));
/// assert_eq!(100u8.mul_shr_round(200, 8u32, Nearest), (78, Less));
/// assert_eq!(96u8.mul_shr_round(8, 8u32, Exact), (3, Equal));
///
/// // a tie, broken toward the even neighbor
/// assert_eq!(5u8.mul_shr_round(102, 2u32, Nearest), (128, Greater));
///
/// // the whole point: the product of two u64s does not fit a u64, but its high half does
/// assert_eq!(
///     u64::MAX.mul_shr_round(u64::MAX, 64u32, Down),
///     (0xfffffffffffffffe, Less)
/// );
///
/// // u128 works the same way, despite there being no wider type to multiply into
/// assert_eq!(
///     10u128.pow(30).mul_shr_round(10u128.pow(30), 128u32, Down),
///     (2938735877055718769921, Less)
/// );
///
/// // Floor and Down differ for negative products
/// assert_eq!((-100i16).mul_shr_round(200, 8u32, Floor), (-79, Less));
/// assert_eq!((-100i16).mul_shr_round(200, 8u32, Down), (-78, Greater));
/// assert_eq!(
///     (-1000000000007i64).mul_shr_round(1000000000009, 64u32, Floor),
///     (-54211, Less)
/// );
///
/// // negative bits shift left; the shift is exact
/// assert_eq!(3u8.mul_shr_round(5, -2i8, Floor), (60, Equal));
/// ```
///
/// # mul_shr_round_assign
/// ```
/// use malachite_base::num::arithmetic::traits::MulShrRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use std::cmp::Ordering::*;
///
/// let mut x = 100u8;
/// assert_eq!(x.mul_shr_round_assign(200, 8u32, Down), Less);
/// assert_eq!(x, 78);
///
/// let mut x = u64::MAX;
/// assert_eq!(x.mul_shr_round_assign(u64::MAX, 64u32, Down), Less);
/// assert_eq!(x, 0xfffffffffffffffe);
///
/// let mut x = -100i16;
/// assert_eq!(x.mul_shr_round_assign(200, 8u32, Floor), Less);
/// assert_eq!(x, -79);
/// ```
pub mod mul_shr_round;
/// [`MulSubMul`](traits::MulSubMul) and [`MulSubMulAssign`](traits::MulSubMulAssign), traits for
/// subtracting the product of one pair of numbers from the product of another.
///
/// # mul_sub_mul
/// ```
/// use malachite_base::num::arithmetic::traits::MulSubMul;
///
/// assert_eq!(10u8.mul_sub_mul(3, 4, 5), 10);
/// assert_eq!(2i8.mul_sub_mul(3, 4, 5), -14);
/// ```
///
/// # mul_sub_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
///
/// let mut x = 10u8;
/// x.mul_sub_mul_assign(3, 4, 5);
/// assert_eq!(x, 10);
///
/// let mut x = 2i8;
/// x.mul_sub_mul_assign(3, 4, 5);
/// assert_eq!(x, -14);
/// ```
pub mod mul_sub_mul;
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
/// [`OverflowingMulAddMul`](traits::OverflowingMulAddMul) and
/// [`OverflowingMulAddMulAssign`](traits::OverflowingMulAddMulAssign), traits for adding the
/// products of two pairs of numbers.
///
/// # overflowing_mul_add_mul
/// ```
/// use malachite_base::num::arithmetic::traits::OverflowingMulAddMul;
///
/// assert_eq!(2u8.overflowing_mul_add_mul(3, 4, 5), (26, false));
/// assert_eq!(200u8.overflowing_mul_add_mul(200, 100, 100), (80, true));
/// assert_eq!(10i8.overflowing_mul_add_mul(-2, 3, 5), (-5, false));
/// ```
///
/// # overflowing_mul_add_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::OverflowingMulAddMulAssign;
///
/// let mut x = 200u8;
/// assert_eq!(x.overflowing_mul_add_mul_assign(200, 100, 100), true);
/// assert_eq!(x, 80);
/// ```
pub mod overflowing_mul_add_mul;
/// [`OverflowingMulSubMul`](traits::OverflowingMulSubMul) and
/// [`OverflowingMulSubMulAssign`](traits::OverflowingMulSubMulAssign), traits for subtracting the
/// product of one pair of numbers from the product of another.
///
/// # overflowing_mul_sub_mul
/// ```
/// use malachite_base::num::arithmetic::traits::OverflowingMulSubMul;
///
/// assert_eq!(10u8.overflowing_mul_sub_mul(3, 4, 5), (10, false));
/// assert_eq!(1u8.overflowing_mul_sub_mul(1, 2, 2), (253, true));
/// assert_eq!(2i8.overflowing_mul_sub_mul(3, 4, 5), (-14, false));
/// ```
///
/// # overflowing_mul_sub_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::OverflowingMulSubMulAssign;
///
/// let mut x = 1u8;
/// assert_eq!(x.overflowing_mul_sub_mul_assign(1, 2, 2), true);
/// assert_eq!(x, 253);
/// ```
pub mod overflowing_mul_sub_mul;
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
/// assert_eq!(f32::power_of_2(0u64), 1.0);
/// assert_eq!(f32::power_of_2(3u64), 8.0);
/// assert_eq!(f32::power_of_2(-3i64), 0.125);
/// ```
pub mod power_of_2;
/// [`Reciprocal`](traits::Reciprocal) and [`ReciprocalAssign`](traits::ReciprocalAssign), traits
/// for computing the reciprocal (multiplicative inverse) of a number.
///
/// # reciprocal
/// ```
/// use malachite_base::num::arithmetic::traits::Reciprocal;
///
/// assert_eq!(0.0f32.reciprocal(), f32::INFINITY);
/// assert_eq!(1.5f32.reciprocal(), 0.6666667);
/// ```
///
/// # reciprocal_assign
/// ```
/// use malachite_base::num::arithmetic::traits::ReciprocalAssign;
///
/// let mut x = 0.0f32;
/// x.reciprocal_assign();
/// assert_eq!(x, f32::INFINITY);
///
/// let mut x = 1.5f32;
/// x.reciprocal_assign();
/// assert_eq!(x, 0.6666667);
/// ```
pub mod reciprocal;
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
/// [`SaturatingMulAddMul`](traits::SaturatingMulAddMul) and
/// [`SaturatingMulAddMulAssign`](traits::SaturatingMulAddMulAssign), traits for adding the products
/// of two pairs of numbers.
///
/// # saturating_mul_add_mul
/// ```
/// use malachite_base::num::arithmetic::traits::SaturatingMulAddMul;
///
/// assert_eq!(2u8.saturating_mul_add_mul(3, 4, 5), 26);
/// assert_eq!(200u8.saturating_mul_add_mul(200, 100, 100), 255);
/// assert_eq!(10i8.saturating_mul_add_mul(-2, 3, 5), -5);
/// assert_eq!(100i8.saturating_mul_add_mul(100, 100, 100), 127);
/// assert_eq!((-100i8).saturating_mul_add_mul(100, -100, 100), -128);
/// ```
///
/// # saturating_mul_add_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::SaturatingMulAddMulAssign;
///
/// let mut x = 200u8;
/// x.saturating_mul_add_mul_assign(200, 100, 100);
/// assert_eq!(x, 255);
/// ```
pub mod saturating_mul_add_mul;
/// [`SaturatingMulSubMul`](traits::SaturatingMulSubMul) and
/// [`SaturatingMulSubMulAssign`](traits::SaturatingMulSubMulAssign), traits for subtracting the
/// product of one pair of numbers from the product of another.
///
/// # saturating_mul_sub_mul
/// ```
/// use malachite_base::num::arithmetic::traits::SaturatingMulSubMul;
///
/// assert_eq!(10u8.saturating_mul_sub_mul(3, 4, 5), 10);
/// // The exact result is negative, so it saturates to zero.
/// assert_eq!(1u8.saturating_mul_sub_mul(1, 2, 2), 0);
/// assert_eq!(2i8.saturating_mul_sub_mul(3, 4, 5), -14);
/// assert_eq!(100i8.saturating_mul_sub_mul(100, -100, 100), 127);
/// ```
///
/// # saturating_mul_sub_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::SaturatingMulSubMulAssign;
///
/// let mut x = 1u8;
/// x.saturating_mul_sub_mul_assign(1, 2, 2);
/// assert_eq!(x, 0);
/// ```
pub mod saturating_mul_sub_mul;
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
/// assert_eq!(f64::NEG_INFINITY.sign(), Less);
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
/// [`WrappingMulAddMul`](traits::WrappingMulAddMul) and
/// [`WrappingMulAddMulAssign`](traits::WrappingMulAddMulAssign), traits for adding the products of
/// two pairs of numbers.
///
/// # wrapping_mul_add_mul
/// ```
/// use malachite_base::num::arithmetic::traits::WrappingMulAddMul;
///
/// assert_eq!(2u8.wrapping_mul_add_mul(3, 4, 5), 26);
/// assert_eq!(200u8.wrapping_mul_add_mul(200, 100, 100), 80);
/// assert_eq!(10i8.wrapping_mul_add_mul(-2, 3, 5), -5);
/// ```
///
/// # wrapping_mul_add_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::WrappingMulAddMulAssign;
///
/// let mut x = 200u8;
/// x.wrapping_mul_add_mul_assign(200, 100, 100);
/// assert_eq!(x, 80);
/// ```
pub mod wrapping_mul_add_mul;
/// [`WrappingMulSubMul`](traits::WrappingMulSubMul) and
/// [`WrappingMulSubMulAssign`](traits::WrappingMulSubMulAssign), traits for subtracting the product
/// of one pair of numbers from the product of another.
///
/// # wrapping_mul_sub_mul
/// ```
/// use malachite_base::num::arithmetic::traits::WrappingMulSubMul;
///
/// assert_eq!(10u8.wrapping_mul_sub_mul(3, 4, 5), 10);
/// assert_eq!(1u8.wrapping_mul_sub_mul(1, 2, 2), 253);
/// assert_eq!(2i8.wrapping_mul_sub_mul(3, 4, 5), -14);
/// ```
///
/// # wrapping_mul_sub_mul_assign
/// ```
/// use malachite_base::num::arithmetic::traits::WrappingMulSubMulAssign;
///
/// let mut x = 1u8;
/// x.wrapping_mul_sub_mul_assign(1, 2, 2);
/// assert_eq!(x, 253);
/// ```
pub mod wrapping_mul_sub_mul;
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
