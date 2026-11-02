// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// [`BalancedMod`](traits::BalancedMod) and [`BalancedModAssign`](traits::BalancedModAssign),
/// traits for finding the representative of a number modulo another number that is closest to zero.
///
/// # balanced_mod
/// ```
/// use malachite_base::num::modular::traits::BalancedMod;
///
/// assert_eq!(23u32.balanced_mod(10), 3);
/// // 7 is more than half of 10, so the closest representative is negative
/// assert_eq!(27u32.balanced_mod(10), -3);
/// // exactly half the modulus is the top of the range, so it stays positive
/// assert_eq!(25u32.balanced_mod(10), 5);
///
/// assert_eq!((-23i32).balanced_mod(10), -3);
/// // only the magnitude of the modulus matters
/// assert_eq!(27i32.balanced_mod(-10), -3);
/// ```
///
/// # balanced_mod_assign
/// ```
/// use malachite_base::num::modular::traits::BalancedModAssign;
///
/// let mut x = 27i32;
/// x.balanced_mod_assign(10);
/// assert_eq!(x, -3);
/// ```
pub mod balanced_mod;
/// [`BalancedModPowerOf2`](traits::BalancedModPowerOf2) and
/// [`BalancedModPowerOf2Assign`](traits::BalancedModPowerOf2Assign), traits for finding the
/// representative of a number modulo $2^k$ that is closest to zero.
///
/// # balanced_mod_power_of_2
/// ```
/// use malachite_base::num::modular::traits::BalancedModPowerOf2;
///
/// assert_eq!(19u32.balanced_mod_power_of_2(3), 3);
/// // 7 is more than half of 8, so the closest representative is negative
/// assert_eq!(23u32.balanced_mod_power_of_2(3), -1);
/// // exactly half the modulus is the top of the range, so it stays positive
/// assert_eq!(20u32.balanced_mod_power_of_2(3), 4);
/// // with pow equal to the width, the low bits are reinterpreted around zero
/// assert_eq!(200u8.balanced_mod_power_of_2(8), -56);
///
/// assert_eq!((-19i32).balanced_mod_power_of_2(3), -3);
/// assert_eq!((-20i32).balanced_mod_power_of_2(3), 4);
/// // every i8 is its own balanced remainder modulo 2^9
/// assert_eq!((-100i8).balanced_mod_power_of_2(9), -100);
/// ```
///
/// # balanced_mod_power_of_2_assign
/// ```
/// use malachite_base::num::modular::traits::BalancedModPowerOf2Assign;
///
/// let mut x = 23i32;
/// x.balanced_mod_power_of_2_assign(3);
/// assert_eq!(x, -1);
/// ```
pub mod balanced_mod_power_of_2;
/// [`Crt`](traits::Crt), a trait for combining two congruences by the Chinese remainder theorem.
///
/// # crt
/// ```
/// use malachite_base::num::modular::traits::Crt;
///
/// // 8 is 2 mod 3 and 3 mod 5.
/// assert_eq!(2u8.crt(3, 3, 5), Some(8));
/// assert_eq!(100u16.crt(101, 200, 251), Some(8483));
/// // The moduli 4 and 6 are not coprime.
/// assert_eq!(1u32.crt(4, 3, 6), None);
/// assert_eq!(5u64.crt(10, 0, 1), Some(5));
/// ```
pub mod crt;
/// [`DivisibleBy`](traits::DivisibleBy), a trait for determining whether one number is divisible by
/// another.
///
/// # divisible_by
/// ```
/// use malachite_base::num::modular::traits::DivisibleBy;
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
/// use malachite_base::num::modular::traits::DivisibleByPowerOf2;
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
/// use malachite_base::num::modular::traits::EqMod;
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
/// use malachite_base::num::modular::traits::EqModPowerOf2;
///
/// assert_eq!(0u16.eq_mod_power_of_2(256, 8), true);
/// assert_eq!((-0b1101i32).eq_mod_power_of_2(0b11011, 3), true);
/// assert_eq!((-0b1101i64).eq_mod_power_of_2(0b11011, 4), false);
/// ```
pub mod eq_mod_power_of_2;
/// [`ModAbsSquared`](traits::ModAbsSquared) and
/// [`ModAbsSquaredAssign`](traits::ModAbsSquaredAssign), traits for computing the squared absolute
/// value of a number modulo another number, which for an unsigned number is its square.
///
/// # mod_abs_squared
/// ```
/// use malachite_base::num::modular::traits::ModAbsSquared;
///
/// assert_eq!(2u8.mod_abs_squared(10), 4);
/// assert_eq!(100u32.mod_abs_squared(497), 60);
/// ```
///
/// # mod_abs_squared_assign
/// ```
/// use malachite_base::num::modular::traits::ModAbsSquaredAssign;
///
/// let mut n = 2u8;
/// n.mod_abs_squared_assign(10);
/// assert_eq!(n, 4);
///
/// let mut n = 100u32;
/// n.mod_abs_squared_assign(497);
/// assert_eq!(n, 60);
/// ```
pub mod mod_abs_squared;
/// [`ModAdd`](traits::ModAdd) and [`ModAddAssign`](traits::ModAddAssign), traits for adding two
/// numbers modulo another number.
///
/// # mod_add
/// ```
/// use malachite_base::num::modular::traits::ModAdd;
///
/// assert_eq!(0u8.mod_add(3, 5), 3);
/// assert_eq!(7u32.mod_add(5, 10), 2);
/// ```
///
/// # mod_add_assign
/// ```
/// use malachite_base::num::modular::traits::ModAddAssign;
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
/// [`ModAddMul`](traits::ModAddMul) and [`ModAddMulAssign`](traits::ModAddMulAssign), traits for
/// adding a number and the product of two other numbers modulo another number.
///
/// # mod_add_mul
/// ```
/// use malachite_base::num::modular::traits::ModAddMul;
///
/// assert_eq!(3u8.mod_add_mul(2, 5, 7), 6);
/// assert_eq!(10u32.mod_add_mul(14, 3, 15), 7);
/// assert_eq!(200u16.mod_add_mul(100, 3, 255), 245);
/// ```
///
/// # mod_add_mul_assign
/// ```
/// use malachite_base::num::modular::traits::ModAddMulAssign;
///
/// let mut x = 10u32;
/// x.mod_add_mul_assign(14, 3, 15);
/// assert_eq!(x, 7);
/// ```
pub mod mod_add_mul;
/// [`ModAddMulShl`](traits::ModAddMulShl) and [`ModAddMulShlAssign`](traits::ModAddMulShlAssign),
/// traits for adding a number and the product of two other numbers, shifted left, modulo another
/// number.
///
/// # mod_add_mul_shl
/// ```
/// use malachite_base::num::modular::traits::ModAddMulShl;
///
/// assert_eq!(3u8.mod_add_mul_shl(2, 5, 1, 7), 2);
/// assert_eq!(10u32.mod_add_mul_shl(14, 3, 2, 15), 13);
/// assert_eq!(200u16.mod_add_mul_shl(100, 3, 40, 255), 245);
/// ```
///
/// # mod_add_mul_shl_assign
/// ```
/// use malachite_base::num::modular::traits::ModAddMulShlAssign;
///
/// let mut x = 10u32;
/// x.mod_add_mul_shl_assign(14, 3, 2, 15);
/// assert_eq!(x, 13);
/// ```
pub mod mod_add_mul_shl;
/// [`ModDiv`](traits::ModDiv), a trait for dividing two numbers modulo another number.
///
/// # mod_div
/// ```
/// use malachite_base::num::modular::traits::ModDiv;
///
/// assert_eq!(6u8.mod_div(4, 10), Some(4));
/// assert_eq!(1u32.mod_div(3, 10), Some(7));
/// assert_eq!(2u16.mod_div(5, 10), None);
/// assert_eq!(123u64.mod_div(456, 789), Some(265));
/// ```
pub mod mod_div;
/// [`ModDivList`](traits::ModDivList), a trait for finding all quotients of two numbers modulo
/// another number.
///
/// # mod_div_list
/// ```
/// use malachite_base::num::modular::traits::ModDivList;
///
/// // The quotients of 6 and 4 mod 10 are 4 and 9: 4 + 5 * i for 0 <= i < 2.
/// assert_eq!(6u8.mod_div_list(4, 10), Some((4, 5, 2)));
/// assert_eq!(1u16.mod_div_list(3, 10), Some((7, 10, 1)));
/// assert_eq!(2u32.mod_div_list(5, 10), None);
/// assert_eq!(0u64.mod_div_list(0, 10), Some((0, 1, 10)));
/// ```
pub mod mod_div_list;
/// [`ModEuclidean`](traits::ModEuclidean) and [`ModEuclideanAssign`](traits::ModEuclideanAssign),
/// traits for finding the remainder of two numbers, where the remainder is always nonnegative.
///
/// # mod_euclidean
/// ```
/// use malachite_base::num::modular::traits::ModEuclidean;
///
/// // 2 * 10 + 3 = 23
/// assert_eq!(23u8.mod_euclidean(10), 3);
///
/// // 9 * 5 + 0 = 45
/// assert_eq!(45u32.mod_euclidean(5), 0);
///
/// // 2 * 10 + 3 = 23
/// assert_eq!(23i8.mod_euclidean(10), 3);
///
/// // -2 * -10 + 3 = 23
/// assert_eq!(23i16.mod_euclidean(-10), 3);
///
/// // -3 * 10 + 7 = -23
/// assert_eq!((-23i32).mod_euclidean(10), 7);
///
/// // 3 * -10 + 7 = -23
/// assert_eq!((-23i64).mod_euclidean(-10), 7);
/// ```
///
/// # mod_euclidean_assign
/// ```
/// use malachite_base::num::modular::traits::ModEuclideanAssign;
///
/// // 2 * 10 + 3 = 23
/// let mut x = 23u8;
/// x.mod_euclidean_assign(10);
/// assert_eq!(x, 3);
///
/// // -3 * 10 + 7 = -23
/// let mut x = -23i32;
/// x.mod_euclidean_assign(10);
/// assert_eq!(x, 7);
///
/// // 3 * -10 + 7 = -23
/// let mut x = -23i64;
/// x.mod_euclidean_assign(-10);
/// assert_eq!(x, 7);
/// ```
pub mod mod_euclidean;
/// [`ModInverse`](traits::ModInverse), a trait for finding the multiplicative inverse of a number
/// modulo another number.
///
/// # mod_inverse
/// ```
/// use malachite_base::num::modular::traits::ModInverse;
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
/// use malachite_base::num::modular::traits::ModIsReduced;
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
/// use malachite_base::num::modular::traits::ModMul;
///
/// assert_eq!(2u8.mod_mul(3, 7), 6);
/// assert_eq!(7u32.mod_mul(3, 10), 1);
/// ```
///
/// # mod_mul_assign
/// ```
/// use malachite_base::num::modular::traits::ModMulAssign;
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
/// use malachite_base::num::modular::traits::ModMulPrecomputed;
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
/// use malachite_base::num::modular::traits::{ModMulPrecomputed, ModMulPrecomputedAssign};
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
/// use malachite_base::num::modular::traits::ModNeg;
///
/// assert_eq!(0u8.mod_neg(5), 0);
/// assert_eq!(7u32.mod_neg(10), 3);
/// assert_eq!(100u16.mod_neg(101), 1);
/// ```
///
/// # mod_neg_assign
/// ```
/// use malachite_base::num::modular::traits::ModNegAssign;
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
/// use malachite_base::num::modular::traits::Mod;
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
/// use malachite_base::num::modular::traits::ModAssign;
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
/// use malachite_base::num::modular::traits::NegMod;
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
/// use malachite_base::num::modular::traits::NegModAssign;
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
/// use malachite_base::num::modular::traits::CeilingMod;
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
/// use malachite_base::num::modular::traits::CeilingModAssign;
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
#[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]
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
/// use malachite_base::num::modular::traits::ModPow;
///
/// assert_eq!(4u16.mod_pow(13, 497), 445);
/// assert_eq!(10u32.mod_pow(1000, 30), 10);
/// ```
///
/// # mod_pow_assign
/// ```
/// use malachite_base::num::modular::traits::ModPowAssign;
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
/// use malachite_base::num::modular::traits::ModPowPrecomputed;
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
/// use malachite_base::num::modular::traits::{ModPowPrecomputed, ModPowPrecomputedAssign};
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
/// traits::ModPowerOf2Assign)       | traits::RemPowerOf2Assign)       |
/// traits::NegModPowerOf2Assign) |
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
/// use malachite_base::num::modular::traits::ModPowerOf2;
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
/// use malachite_base::num::modular::traits::ModPowerOf2Assign;
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
/// use malachite_base::num::modular::traits::RemPowerOf2;
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
/// use malachite_base::num::modular::traits::RemPowerOf2Assign;
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
/// use malachite_base::num::modular::traits::NegModPowerOf2;
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
/// use malachite_base::num::modular::traits::NegModPowerOf2Assign;
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
/// use malachite_base::num::modular::traits::CeilingModPowerOf2;
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
/// use malachite_base::num::modular::traits::CeilingModPowerOf2Assign;
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
#[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]
pub mod mod_power_of_2;
/// [`ModPowerOf2AbsSquared`](traits::ModPowerOf2AbsSquared) and
/// [`ModPowerOf2AbsSquaredAssign`](traits::ModPowerOf2AbsSquaredAssign), traits for computing the
/// squared absolute value of a number modulo $2^k$, which for an unsigned number is its square.
///
/// # mod_power_of_2_abs_squared
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2AbsSquared;
///
/// assert_eq!(5u8.mod_power_of_2_abs_squared(3), 1);
/// assert_eq!(100u32.mod_power_of_2_abs_squared(8), 16);
/// ```
///
/// # mod_power_of_2_abs_squared_assign
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2AbsSquaredAssign;
///
/// let mut n = 5u8;
/// n.mod_power_of_2_abs_squared_assign(3);
/// assert_eq!(n, 1);
///
/// let mut n = 100u32;
/// n.mod_power_of_2_abs_squared_assign(8);
/// assert_eq!(n, 16);
/// ```
pub mod mod_power_of_2_abs_squared;
/// [`ModPowerOf2Add`](traits::ModPowerOf2Add) and
/// [`ModPowerOf2AddAssign`](traits::ModPowerOf2AddAssign), traits for adding two numbers modulo
/// $2^k$.
///
/// # mod_power_of_2_add
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2Add;
///
/// assert_eq!(0u8.mod_power_of_2_add(2, 5), 2);
/// assert_eq!(10u32.mod_power_of_2_add(14, 4), 8);
/// ```
///
/// # mod_power_of_2_add_assign
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2AddAssign;
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
/// [`ModPowerOf2AddMul`](traits::ModPowerOf2AddMul) and
/// [`ModPowerOf2AddMulAssign`](traits::ModPowerOf2AddMulAssign), traits for adding a number and the
/// product of two other numbers modulo $2^k$.
///
/// # mod_power_of_2_add_mul
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2AddMul;
///
/// assert_eq!(3u8.mod_power_of_2_add_mul(2, 5, 5), 13);
/// assert_eq!(10u32.mod_power_of_2_add_mul(14, 3, 4), 4);
/// assert_eq!(200u16.mod_power_of_2_add_mul(100, 3, 8), 244);
/// ```
///
/// # mod_power_of_2_add_mul_assign
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2AddMulAssign;
///
/// let mut x = 10u32;
/// x.mod_power_of_2_add_mul_assign(14, 3, 4);
/// assert_eq!(x, 4);
/// ```
pub mod mod_power_of_2_add_mul;
/// [`ModPowerOf2AddMulShl`](traits::ModPowerOf2AddMulShl) and
/// [`ModPowerOf2AddMulShlAssign`](traits::ModPowerOf2AddMulShlAssign), traits for adding a number
/// and the product of two other numbers, shifted left, modulo $2^k$.
///
/// # mod_power_of_2_add_mul_shl
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2AddMulShl;
///
/// assert_eq!(3u8.mod_power_of_2_add_mul_shl(2, 5, 1, 5), 23);
/// assert_eq!(10u32.mod_power_of_2_add_mul_shl(14, 3, 2, 6), 50);
/// assert_eq!(7u16.mod_power_of_2_add_mul_shl(1, 1, 8, 8), 7);
/// ```
///
/// # mod_power_of_2_add_mul_shl_assign
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2AddMulShlAssign;
///
/// let mut x = 10u32;
/// x.mod_power_of_2_add_mul_shl_assign(14, 3, 2, 6);
/// assert_eq!(x, 50);
/// ```
pub mod mod_power_of_2_add_mul_shl;
/// [`ModPowerOf2Div`](traits::ModPowerOf2Div), a trait for dividing two numbers modulo $2^k$.
///
/// # mod_power_of_2_div
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2Div;
///
/// assert_eq!(6u8.mod_power_of_2_div(3, 4), Some(2));
/// assert_eq!(1u32.mod_power_of_2_div(3, 4), Some(11));
/// assert_eq!(6u16.mod_power_of_2_div(4, 4), None);
/// assert_eq!(12u64.mod_power_of_2_div(4, 4), Some(3));
/// ```
pub mod mod_power_of_2_div;
/// [`ModPowerOf2Inverse`](traits::ModPowerOf2Inverse), a trait for finding the multiplicative
/// inverse of a number modulo $2^k$.
///
/// # mod_inverse
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2Inverse;
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
/// use malachite_base::num::modular::traits::ModPowerOf2IsReduced;
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
/// use malachite_base::num::modular::traits::ModPowerOf2Mul;
///
/// assert_eq!(3u8.mod_power_of_2_mul(2, 5), 6);
/// assert_eq!(10u32.mod_power_of_2_mul(14, 4), 12);
/// ```
///
/// # mod_power_of_2_mul_assign
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2MulAssign;
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
/// use malachite_base::num::modular::traits::ModPowerOf2Neg;
///
/// assert_eq!(0u8.mod_power_of_2_neg(5), 0);
/// assert_eq!(10u32.mod_power_of_2_neg(4), 6);
/// assert_eq!(100u16.mod_power_of_2_neg(8), 156);
/// ```
///
/// # mod_power_of_2_neg_assign
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2NegAssign;
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
/// use malachite_base::num::modular::traits::ModPowerOf2Pow;
///
/// assert_eq!(5u8.mod_power_of_2_pow(13, 3), 5);
/// assert_eq!(7u32.mod_power_of_2_pow(1000, 6), 1);
/// ```
///
/// # mod_power_of_2_pow_assign
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2PowAssign;
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
/// [`ModPowerOf2Product`](traits::ModPowerOf2Product), a trait for multiplying together the numbers
/// in an iterator modulo $2^k$.
///
/// # mod_power_of_2_product
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2Product;
///
/// assert_eq!(
///     u8::mod_power_of_2_product([200, 100, 50].into_iter(), 8),
///     64
/// );
/// assert_eq!(u32::mod_power_of_2_product([2, 3, 5, 7].into_iter(), 4), 2);
/// assert_eq!(
///     u64::mod_power_of_2_product(core::iter::empty::<u64>(), 10),
///     1
/// );
/// assert_eq!(
///     u64::mod_power_of_2_product(core::iter::empty::<u64>(), 0),
///     0
/// );
/// assert_eq!(u8::mod_power_of_2_product([3, 5, 7].iter(), 4), 9);
/// ```
pub mod mod_power_of_2_product;
/// [`ModPowerOf2Shl`](traits::ModPowerOf2Shl) and
/// [`ModPowerOf2ShlAssign`](traits::ModPowerOf2ShlAssign), traits for left-shifting a number modulo
/// $2^k$.
///
/// # mod_power_of_2_shl
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2Shl;
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
/// use malachite_base::num::modular::traits::ModPowerOf2ShlAssign;
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
/// use malachite_base::num::modular::traits::ModPowerOf2Shr;
///
/// assert_eq!(10u8.mod_power_of_2_shr(2i64, 4), 2);
/// assert_eq!(12u32.mod_power_of_2_shr(-2i8, 5), 16);
/// ```
///
/// # mod_power_of_2_shr_assign
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2ShrAssign;
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
/// [`ModPowerOf2Sqrt`](traits::ModPowerOf2Sqrt), a trait for finding the least square root of a
/// number modulo $2^k$.
///
/// # mod_power_of_2_sqrt
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2Sqrt;
///
/// // The roots of 17 modulo 32 are 7, 9, 23, and 25.
/// assert_eq!(17u8.mod_power_of_2_sqrt(5), Some(7));
/// assert_eq!(17u64.mod_power_of_2_sqrt(64), Some(405959429219100393));
/// assert_eq!(
///     (17u64 << 20).mod_power_of_2_sqrt(64),
///     Some(2195515552539648)
/// );
/// // 3 is not 1 mod 8, and 8 = 2 ^ 3 has an odd power of 2.
/// assert_eq!(3u16.mod_power_of_2_sqrt(4), None);
/// assert_eq!(8u32.mod_power_of_2_sqrt(4), None);
/// assert_eq!(0u128.mod_power_of_2_sqrt(10), Some(0));
/// ```
pub mod mod_power_of_2_sqrt;
/// [`ModPowerOf2Square`](traits::ModPowerOf2Square) and
/// [`ModPowerOf2SquareAssign`](traits::ModPowerOf2SquareAssign), traits for squaring a number
/// modulo $2^k$.
///
/// # mod_power_of_2_square
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2Square;
///
/// assert_eq!(5u8.mod_power_of_2_square(3), 1);
/// assert_eq!(100u32.mod_power_of_2_square(8), 16);
/// ```
///
/// # mod_power_of_2_square_assign
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2SquareAssign;
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
/// use malachite_base::num::modular::traits::ModPowerOf2Sub;
///
/// assert_eq!(5u8.mod_power_of_2_sub(2, 5), 3);
/// assert_eq!(10u32.mod_power_of_2_sub(14, 4), 12);
/// ```
///
/// # mod_power_of_2_sub_assign
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2SubAssign;
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
/// [`ModPowerOf2SubMul`](traits::ModPowerOf2SubMul) and
/// [`ModPowerOf2SubMulAssign`](traits::ModPowerOf2SubMulAssign), traits for subtracting the product
/// of two numbers from a number modulo $2^k$.
///
/// # mod_power_of_2_sub_mul
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2SubMul;
///
/// assert_eq!(13u8.mod_power_of_2_sub_mul(2, 5, 5), 3);
/// assert_eq!(4u32.mod_power_of_2_sub_mul(14, 3, 4), 10);
/// assert_eq!(10u16.mod_power_of_2_sub_mul(100, 3, 8), 222);
/// ```
///
/// # mod_power_of_2_sub_mul_assign
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2SubMulAssign;
///
/// let mut x = 4u32;
/// x.mod_power_of_2_sub_mul_assign(14, 3, 4);
/// assert_eq!(x, 10);
/// ```
pub mod mod_power_of_2_sub_mul;
/// [`ModPowerOf2SubMulShl`](traits::ModPowerOf2SubMulShl) and
/// [`ModPowerOf2SubMulShlAssign`](traits::ModPowerOf2SubMulShlAssign), traits for subtracting the
/// product of two numbers, shifted left, from a number modulo $2^k$.
///
/// # mod_power_of_2_sub_mul_shl
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2SubMulShl;
///
/// assert_eq!(3u8.mod_power_of_2_sub_mul_shl(2, 5, 1, 5), 15);
/// assert_eq!(10u32.mod_power_of_2_sub_mul_shl(14, 3, 2, 6), 34);
/// assert_eq!(7u16.mod_power_of_2_sub_mul_shl(1, 1, 8, 8), 7);
/// ```
///
/// # mod_power_of_2_sub_mul_shl_assign
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2SubMulShlAssign;
///
/// let mut x = 10u32;
/// x.mod_power_of_2_sub_mul_shl_assign(14, 3, 2, 6);
/// assert_eq!(x, 34);
/// ```
pub mod mod_power_of_2_sub_mul_shl;
/// [`ModPowerOf2Sum`](traits::ModPowerOf2Sum), a trait for adding up the numbers in an iterator
/// modulo $2^k$.
///
/// # mod_power_of_2_sum
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2Sum;
///
/// assert_eq!(u8::mod_power_of_2_sum([200, 100, 50].into_iter(), 8), 94);
/// assert_eq!(u32::mod_power_of_2_sum([2, 3, 5, 7].into_iter(), 4), 1);
/// assert_eq!(u64::mod_power_of_2_sum(core::iter::empty::<u64>(), 10), 0);
/// assert_eq!(u8::mod_power_of_2_sum([200, 100, 50].iter(), 8), 94);
/// ```
pub mod mod_power_of_2_sum;
/// [`ModProduct`](traits::ModProduct), a trait for multiplying together the numbers in an iterator
/// modulo another number.
///
/// # mod_product
/// ```
/// use malachite_base::num::modular::traits::ModProduct;
///
/// assert_eq!(u8::mod_product([200, 100, 50].into_iter(), 255), 145);
/// assert_eq!(u32::mod_product([2, 3, 5, 7].into_iter(), 11), 1);
/// assert_eq!(u64::mod_product(core::iter::empty::<u64>(), 10), 1);
/// assert_eq!(u64::mod_product(core::iter::empty::<u64>(), 1), 0);
/// assert_eq!(u8::mod_product([3, 5, 7].iter(), 10), 5);
/// ```
pub mod mod_product;
/// [`ModShl`](traits::ModShl) and [`ModShlAssign`](traits::ModShlAssign), traits for left-shifting
/// a number modulo another number.
///
/// # mod_shl
/// ```
/// use malachite_base::num::modular::traits::ModShl;
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
/// use malachite_base::num::modular::traits::ModShlAssign;
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
/// use malachite_base::num::modular::traits::ModShr;
///
/// assert_eq!(10u8.mod_shr(2i64, 15), 2);
/// assert_eq!(8u32.mod_shr(-2i8, 10), 2);
/// ```
///
/// # mod_shr_assign
/// ```
/// use malachite_base::num::modular::traits::ModShrAssign;
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
/// [`ModSqrt`](traits::ModSqrt), a trait for computing a square root of a number modulo another
/// number.
///
/// # mod_sqrt
/// ```
/// use malachite_base::num::modular::traits::ModSqrt;
///
/// assert_eq!(4u32.mod_sqrt(5), Some(2));
/// assert_eq!(2u32.mod_sqrt(3), None);
/// assert_eq!(12909u64.mod_sqrt(65537), Some(50618));
/// assert_eq!(3u16.mod_sqrt(611), Some(183));
/// ```
pub mod mod_sqrt;
/// Traits for squaring a number modulo another number.
///
/// The traits are [`ModSquare`](traits::ModSquare), [`ModSquareAssign`](traits::ModSquareAssign),
/// and [`ModSquarePrecomputed`](traits::ModSquarePrecomputed).
/// [`ModSquarePrecomputed`](traits::ModSquarePrecomputed) is useful when having to make several
/// squarings modulo the same modulus.
///
/// # mod_square
/// ```
/// use malachite_base::num::modular::traits::ModSquare;
///
/// assert_eq!(2u8.mod_square(10), 4);
/// assert_eq!(100u32.mod_square(497), 60);
/// ```
///
/// # mod_square_assign
/// ```
/// use malachite_base::num::modular::traits::ModSquareAssign;
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
/// use malachite_base::num::modular::traits::{ModPowPrecomputed, ModSquarePrecomputed};
///
/// let data = u16::precompute_mod_pow_data(&497);
/// assert_eq!(100u16.mod_square_precomputed(497, &data), 60);
/// assert_eq!(200u16.mod_square_precomputed(497, &data), 240);
/// assert_eq!(300u16.mod_square_precomputed(497, &data), 43);
/// ```
///
/// # mod_square_precomputed_assign
/// ```
/// use malachite_base::num::modular::traits::{ModPowPrecomputed, ModSquarePrecomputedAssign};
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
/// use malachite_base::num::modular::traits::ModSub;
///
/// assert_eq!(4u8.mod_sub(3, 5), 1);
/// assert_eq!(7u32.mod_sub(9, 10), 8);
/// ```
///
/// # mod_sub_assign
/// ```
/// use malachite_base::num::modular::traits::ModSubAssign;
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
/// [`ModSubMul`](traits::ModSubMul) and [`ModSubMulAssign`](traits::ModSubMulAssign), traits for
/// subtracting the product of two numbers from a number modulo another number.
///
/// # mod_sub_mul
/// ```
/// use malachite_base::num::modular::traits::ModSubMul;
///
/// assert_eq!(6u8.mod_sub_mul(2, 5, 7), 3);
/// assert_eq!(10u32.mod_sub_mul(14, 3, 15), 13);
/// assert_eq!(200u16.mod_sub_mul(100, 3, 255), 155);
/// ```
///
/// # mod_sub_mul_assign
/// ```
/// use malachite_base::num::modular::traits::ModSubMulAssign;
///
/// let mut x = 10u32;
/// x.mod_sub_mul_assign(14, 3, 15);
/// assert_eq!(x, 13);
/// ```
pub mod mod_sub_mul;
/// [`ModSubMulShl`](traits::ModSubMulShl) and [`ModSubMulShlAssign`](traits::ModSubMulShlAssign),
/// traits for subtracting the product of two numbers, shifted left, from a number modulo another
/// number.
///
/// # mod_sub_mul_shl
/// ```
/// use malachite_base::num::modular::traits::ModSubMulShl;
///
/// assert_eq!(3u8.mod_sub_mul_shl(2, 5, 1, 7), 4);
/// assert_eq!(10u32.mod_sub_mul_shl(14, 3, 2, 15), 7);
/// assert_eq!(200u16.mod_sub_mul_shl(100, 3, 40, 255), 155);
/// ```
///
/// # mod_sub_mul_shl_assign
/// ```
/// use malachite_base::num::modular::traits::ModSubMulShlAssign;
///
/// let mut x = 10u32;
/// x.mod_sub_mul_shl_assign(14, 3, 2, 15);
/// assert_eq!(x, 7);
/// ```
pub mod mod_sub_mul_shl;
/// [`ModSum`](traits::ModSum), a trait for adding up the numbers in an iterator modulo another
/// number.
///
/// # mod_sum
/// ```
/// use malachite_base::num::modular::traits::ModSum;
///
/// assert_eq!(u8::mod_sum([200, 100, 50].into_iter(), 255), 95);
/// assert_eq!(u32::mod_sum([2, 3, 5, 7].into_iter(), 10), 7);
/// assert_eq!(u64::mod_sum(core::iter::empty::<u64>(), 10), 0);
/// assert_eq!(u8::mod_sum([200, 100, 50].iter(), 255), 95);
/// ```
pub mod mod_sum;
/// Various traits for modular arithmetic.
pub mod traits;
