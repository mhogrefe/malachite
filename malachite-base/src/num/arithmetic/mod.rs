/// This module contains functions for getting the absolute value of a number.
///
/// Here are usage examples of the macro-generated functions:
///
/// # abs_assign
/// ```
/// use malachite_base::num::arithmetic::traits::AbsAssign;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::float::PrimitiveFloat;
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
/// assert_eq!(NiceFloat(x), NiceFloat(f64::POSITIVE_INFINITY));
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
/// This module contains functions for adding a number and the product of two other numbers.
///
/// Here are usage examples of the macro-generated functions:
///
/// # add_mul
/// ```
/// use malachite_base::num::arithmetic::traits::AddMul;
///
/// assert_eq!(2u8.add_mul(3, 7), 23);
/// assert_eq!(127i8.add_mul(-2, 100), -73);
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
/// ```
pub mod add_mul;
/// This module contains functions for left-shifting a number and checking whether the result is
/// representable.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for right-shifting a number and checking whether the result is
/// representable.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module wraps the `checked_abs` function into an implementation of `CheckedAbs`.
pub mod checked_abs;
/// This module wraps the `checked_add` function into an implementation of `CheckedAdd`.
pub mod checked_add;
/// This module contains functions for adding a number and the product of two other numbers, and
/// checking whether the result is representable.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module wraps the `checked_div` function into an implementation of `CheckedDiv`.
pub mod checked_div;
/// This module wraps the `checked_mul` function into an implementation of `CheckedMul`.
pub mod checked_mul;
/// This module wraps the `checked_neg` function into an implementation of `CheckedNeg`.
pub mod checked_neg;
/// This module wraps the `checked_next_power_of_2` function into an implementation of
/// `CheckedNextPowerOf2`.
pub mod checked_next_power_of_2;
/// This module wraps the `checked_pow` function into an implementation of `CheckedPow`.
pub mod checked_pow;
/// This module contains functions for squaring a number and checking whether the result is
/// representable.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module wraps the `checked_sub` function into an implementation of `CheckedSub`.
pub mod checked_sub;
/// This module contains functions for subtracting the product of two numbers from another number,
/// and checking whether the result is representable.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for unchecked exact division.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for simultaneously finding the quotient and remainder of two
/// numbers, subject to various rounding rules.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions dividing two numbers according to a specified `RoundingMode`.
///
/// Here are usage examples of the macro-generated functions:
///
/// # div_round
/// ```
/// use malachite_base::num::arithmetic::traits::DivRound;
/// use malachite_base::rounding_modes::RoundingMode;
///
/// assert_eq!(10u8.div_round(4, RoundingMode::Down), 2);
/// assert_eq!(10u16.div_round(4, RoundingMode::Up), 3);
/// assert_eq!(10u32.div_round(5, RoundingMode::Exact), 2);
/// assert_eq!(10u64.div_round(3, RoundingMode::Nearest), 3);
/// assert_eq!(20u128.div_round(3, RoundingMode::Nearest), 7);
/// assert_eq!(10usize.div_round(4, RoundingMode::Nearest), 2);
/// assert_eq!(14u8.div_round(4, RoundingMode::Nearest), 4);
///
/// assert_eq!((-10i8).div_round(4, RoundingMode::Down), -2);
/// assert_eq!((-10i16).div_round(4, RoundingMode::Up), -3);
/// assert_eq!((-10i32).div_round(5, RoundingMode::Exact), -2);
/// assert_eq!((-10i64).div_round(3, RoundingMode::Nearest), -3);
/// assert_eq!((-20i128).div_round(3, RoundingMode::Nearest), -7);
/// assert_eq!((-10isize).div_round(4, RoundingMode::Nearest), -2);
/// assert_eq!((-14i8).div_round(4, RoundingMode::Nearest), -4);
///
/// assert_eq!((-10i16).div_round(-4, RoundingMode::Down), 2);
/// assert_eq!((-10i32).div_round(-4, RoundingMode::Up), 3);
/// assert_eq!((-10i64).div_round(-5, RoundingMode::Exact), 2);
/// assert_eq!((-10i128).div_round(-3, RoundingMode::Nearest), 3);
/// assert_eq!((-20isize).div_round(-3, RoundingMode::Nearest), 7);
/// assert_eq!((-10i8).div_round(-4, RoundingMode::Nearest), 2);
/// assert_eq!((-14i16).div_round(-4, RoundingMode::Nearest), 4);
/// ```
///
/// # div_round_assign
/// ```
/// use malachite_base::num::arithmetic::traits::DivRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode;
///
/// let mut x = 10u8;
/// x.div_round_assign(4, RoundingMode::Down);
/// assert_eq!(x, 2);
///
/// let mut x = 10u16;
/// x.div_round_assign(4, RoundingMode::Up);
/// assert_eq!(x, 3);
///
/// let mut x = 10u32;
/// x.div_round_assign(5, RoundingMode::Exact);
/// assert_eq!(x, 2);
///
/// let mut x = 10u64;
/// x.div_round_assign(3, RoundingMode::Nearest);
/// assert_eq!(x, 3);
///
/// let mut x = 20u128;
/// x.div_round_assign(3, RoundingMode::Nearest);
/// assert_eq!(x, 7);
///
/// let mut x = 10usize;
/// x.div_round_assign(4, RoundingMode::Nearest);
/// assert_eq!(x, 2);
///
/// let mut x = 14u8;
/// x.div_round_assign(4, RoundingMode::Nearest);
/// assert_eq!(x, 4);
///
/// let mut x = -10i8;
/// x.div_round_assign(4, RoundingMode::Down);
/// assert_eq!(x, -2);
///
/// let mut x = -10i16;
/// x.div_round_assign(4, RoundingMode::Up);
/// assert_eq!(x, -3);
///
/// let mut x = -10i32;
/// x.div_round_assign(5, RoundingMode::Exact);
/// assert_eq!(x, -2);
///
/// let mut x = -10i64;
/// x.div_round_assign(3, RoundingMode::Nearest);
/// assert_eq!(x, -3);
///
/// let mut x = -20i128;
/// x.div_round_assign(3, RoundingMode::Nearest);
/// assert_eq!(x, -7);
///
/// let mut x = -10isize;
/// x.div_round_assign(4, RoundingMode::Nearest);
/// assert_eq!(x, -2);
///
/// let mut x = -14i8;
/// x.div_round_assign(4, RoundingMode::Nearest);
/// assert_eq!(x, -4);
///
/// let mut x = -10i16;
/// x.div_round_assign(-4, RoundingMode::Down);
/// assert_eq!(x, 2);
///
/// let mut x = -10i32;
/// x.div_round_assign(-4, RoundingMode::Up);
/// assert_eq!(x, 3);
///
/// let mut x = -10i64;
/// x.div_round_assign(-5, RoundingMode::Exact);
/// assert_eq!(x, 2);
///
/// let mut x = -10i128;
/// x.div_round_assign(-3, RoundingMode::Nearest);
/// assert_eq!(x, 3);
///
/// let mut x = -20isize;
/// x.div_round_assign(-3, RoundingMode::Nearest);
/// assert_eq!(x, 7);
///
/// let mut x = -10i8;
/// x.div_round_assign(-4, RoundingMode::Nearest);
/// assert_eq!(x, 2);
///
/// let mut x = -14i16;
/// x.div_round_assign(-4, RoundingMode::Nearest);
/// assert_eq!(x, 4);
/// ```
pub mod div_round;
/// This module contains functions determining whether one number is divisible by another.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions determining whether one number is divisible by a power of 2.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions determining whether one number is equal by another, mod a third.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions determining whether one number is equal by another, mod a power
/// of 2.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module wraps the `is_power_of_two` function into an implementation of `IsPowerOf2`.
pub mod is_power_of_2;
/// This module contains functions for taking the base-$b$ logarithm of a number.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for taking the base-2 logarithm of a number.
///
/// Here are usage examples of the macro-generated functions:
///
/// # floor_log_base_2
/// ```
/// use malachite_base::num::arithmetic::traits::FloorLogBase2;
///
/// assert_eq!(1u8.floor_log_base_2(), 0);
/// assert_eq!(100u64.floor_log_base_2(), 6);
/// ```
///
/// # ceiling_log_base_2
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingLogBase2;
///
/// assert_eq!(1u8.ceiling_log_base_2(), 0);
/// assert_eq!(100u64.ceiling_log_base_2(), 7);
/// ```
///
/// # checked_log_base_2
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedLogBase2;
///
/// assert_eq!(1u8.checked_log_base_2(), Some(0));
/// assert_eq!(100u64.checked_log_base_2(), None);
/// assert_eq!(128u64.checked_log_base_2(), Some(7));
/// ```
pub mod log_base_2;
/// This module contains functions for taking the base-$b$ logarithm of a number, where $b$ is a
/// power of 2.
///
/// Here are usage examples of the macro-generated functions:
///
/// # floor_log_base_power_of_2
/// ```
/// use malachite_base::num::arithmetic::traits::FloorLogBasePowerOf2;
///
/// assert_eq!(1u8.floor_log_base_power_of_2(4), 0);
/// assert_eq!(100u64.floor_log_base_power_of_2(2), 3);
/// ```
///
/// # ceiling_log_base_power_of_2
/// ```
/// use malachite_base::num::arithmetic::traits::CeilingLogBasePowerOf2;
///
/// assert_eq!(1u8.ceiling_log_base_power_of_2(4), 0);
/// assert_eq!(100u64.ceiling_log_base_power_of_2(2), 4);
/// ```
///
/// # checked_log_base_power_of_2
/// ```
/// use malachite_base::num::arithmetic::traits::CheckedLogBasePowerOf2;
///
/// assert_eq!(1u8.checked_log_base_power_of_2(4), Some(0));
/// assert_eq!(100u64.checked_log_base_power_of_2(4), None);
/// assert_eq!(256u64.checked_log_base_power_of_2(4), Some(2));
/// ```
pub mod log_base_power_of_2;
/// This module contains functions for adding two numbers mod another number.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for checking whether a number is reduced mod another number.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for multiplying two numbers mod another number.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for negating a number mod another number.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for finding the remainder of two numbers, subject to various
/// rounding rules.
///
/// Here are usage examples of the macro-generated functions:
///
/// # mod_op
/// ```
/// use malachite_base::num::arithmetic::traits::Mod;
///
/// // 2 * 10 + 3 = 23
/// assert_eq!(23u8.mod_op(10), 3);
///
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
/// This module contains functions for raising a number to a power mod another number.
///
/// Here are usage examples of the macro-generated functions:
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
/// use malachite_base::num::arithmetic::traits::{
///     ModPowPrecomputed, ModPowPrecomputedAssign
/// };
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
/// This module contains functions for finding the remainder of a number divided by a power of 2,
/// subject to various rounding rules.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for adding two numbers mod a power of 2.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for checking whether a number is reduced mod a power of 2.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for multiplying two numbers mod a power of 2.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for negating a number mod a power of 2.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for raising a number to a power mod a power of 2.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for left-shifting a number mod a power of 2.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for right-shifting a number mod a power of 2.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for squaring a number mod a power of 2.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for subtracting one number by another, mod a power of 2.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for left-shifting a number mod another number
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for right-shifting a number mod another number
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for squaring a number mod another number.
///
/// Here are usage examples of the macro-generated functions:
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
/// use malachite_base::num::arithmetic::traits::{
///     ModPowPrecomputed, ModSquarePrecomputed
/// };
///
/// let data = u16::precompute_mod_pow_data(&497);
/// assert_eq!(100u16.mod_square_precomputed(497, &data), 60);
/// assert_eq!(200u16.mod_square_precomputed(497, &data), 240);
/// assert_eq!(300u16.mod_square_precomputed(497, &data), 43);
/// ```
///
/// # mod_square_precomputed_assign
/// ```
/// use malachite_base::num::arithmetic::traits::{
///     ModPowPrecomputed, ModSquarePrecomputedAssign
/// };
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
/// This module contains functions for subtracting one number by another, mod a third number.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for negating a number.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for getting the next-highest power of 2.
///
/// Here are usage examples of the macro-generated functions:
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
/// ```
pub mod next_power_of_2;
/// This module contains functions for taking the absolute value of a number and returning a
/// boolean indicating whether an overflow occurred.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for adding two numbers and returning a boolean indicating
/// whether an overflow occurred.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for adding a number and the product of two other numbers, and
/// returning a boolean indicating whether an overflow occurred.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for dividing a number by another number and returning a boolean
/// indicating whether an overflow occurred.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for multiplying two numbers and returning a boolean indicating
/// whether an overflow occurred.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for negating a number and returning a boolean indicating whether
/// an overflow occurred.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for raising a number to a power and returning a boolean
/// indicating whether an overflow occurred.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for squaring a number and returning a boolean indicating whether
/// an overflow occurred.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for subtracting a number by another number and returning a
/// boolean indicating whether an overflow occurred.
///
/// Here are usage examples of the macro-generated functions:
///
/// # overflowing_square
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
/// This module contains functions for subtracting a number by the product of two other numbers,
/// and returning a boolean indicating whether an overflow occurred.
///
/// Here are usage examples of the macro-generated functions:
///
/// # overflowing_sub_mul
/// ```
/// use malachite_base::num::arithmetic::traits::OverflowingSubMul;
///
/// assert_eq!(60u8.overflowing_sub_mul(5, 10), (10, false));
/// assert_eq!(2u8.overflowing_sub_mul(10, 5), (208, true));
///
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
/// This module contains functions for determining whether a number is even or odd.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for raising a number to a power.
///
/// Here are usage examples of the macro-generated functions:
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
/// ```
pub mod pow;
/// This module contains functions for computing a power of 2.
///
/// Here are usage examples of the macro-generated functions:
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
/// ```
pub mod power_of_2;
/// This module contains functions for rounding a number to a multiple of another number.
///
/// Here are usage examples of the macro-generated functions:
///
/// # round_to_multiple
/// ```
/// use malachite_base::num::arithmetic::traits::RoundToMultiple;
/// use malachite_base::rounding_modes::RoundingMode;
///
/// assert_eq!(5u32.round_to_multiple(0, RoundingMode::Down), 0);
///
/// assert_eq!(10u8.round_to_multiple(4, RoundingMode::Down), 8);
/// assert_eq!(10u16.round_to_multiple(4, RoundingMode::Up), 12);
/// assert_eq!(10u32.round_to_multiple(5, RoundingMode::Exact), 10);
/// assert_eq!(10u64.round_to_multiple(3, RoundingMode::Nearest), 9);
/// assert_eq!(20u128.round_to_multiple(3, RoundingMode::Nearest), 21);
/// assert_eq!(10usize.round_to_multiple(4, RoundingMode::Nearest), 8);
/// assert_eq!(14u8.round_to_multiple(4, RoundingMode::Nearest), 16);
///
/// assert_eq!((-5i32).round_to_multiple(0, RoundingMode::Down), 0);
///
/// assert_eq!((-10i8).round_to_multiple(4, RoundingMode::Down), -8);
/// assert_eq!((-10i16).round_to_multiple(4, RoundingMode::Up), -12);
/// assert_eq!((-10i32).round_to_multiple(5, RoundingMode::Exact), -10);
/// assert_eq!((-10i64).round_to_multiple(3, RoundingMode::Nearest), -9);
/// assert_eq!((-20i128).round_to_multiple(3, RoundingMode::Nearest), -21);
/// assert_eq!((-10isize).round_to_multiple(4, RoundingMode::Nearest), -8);
/// assert_eq!((-14i8).round_to_multiple(4, RoundingMode::Nearest), -16);
///
/// assert_eq!((-10i16).round_to_multiple(-4, RoundingMode::Down), -8);
/// assert_eq!((-10i32).round_to_multiple(-4, RoundingMode::Up), -12);
/// assert_eq!((-10i64).round_to_multiple(-5, RoundingMode::Exact), -10);
/// assert_eq!((-10i128).round_to_multiple(-3, RoundingMode::Nearest), -9);
/// assert_eq!((-20isize).round_to_multiple(-3, RoundingMode::Nearest), -21);
/// assert_eq!((-10i8).round_to_multiple(-4, RoundingMode::Nearest), -8);
/// assert_eq!((-14i16).round_to_multiple(-4, RoundingMode::Nearest), -16);
/// ```
///
/// # round_to_multiple_assign
/// ```
/// use malachite_base::num::arithmetic::traits::RoundToMultipleAssign;
/// use malachite_base::rounding_modes::RoundingMode;
///
/// let mut x = 5u32;
/// x.round_to_multiple_assign(0, RoundingMode::Down);
/// assert_eq!(x, 0);
///
/// let mut x = 10u8;
/// x.round_to_multiple_assign(4, RoundingMode::Down);
/// assert_eq!(x, 8);
///
/// let mut x = 10u16;
/// x.round_to_multiple_assign(4, RoundingMode::Up);
/// assert_eq!(x, 12);
///
/// let mut x = 10u32;
/// x.round_to_multiple_assign(5, RoundingMode::Exact);
/// assert_eq!(x, 10);
///
/// let mut x = 10u64;
/// x.round_to_multiple_assign(3, RoundingMode::Nearest);
/// assert_eq!(x, 9);
///
/// let mut x = 20u128;
/// x.round_to_multiple_assign(3, RoundingMode::Nearest);
/// assert_eq!(x, 21);
///
/// let mut x = 10usize;
/// x.round_to_multiple_assign(4, RoundingMode::Nearest);
/// assert_eq!(x, 8);
///
/// let mut x = 14u8;
/// x.round_to_multiple_assign(4, RoundingMode::Nearest);
/// assert_eq!(x, 16);
///
/// let mut x = -5i32;
/// x.round_to_multiple_assign(0, RoundingMode::Down);
/// assert_eq!(x, 0);
///
/// let mut x = -10i8;
/// x.round_to_multiple_assign(4, RoundingMode::Down);
/// assert_eq!(x, -8);
///
/// let mut x = -10i16;
/// x.round_to_multiple_assign(4, RoundingMode::Up);
/// assert_eq!(x, -12);
///
/// let mut x = -10i32;
/// x.round_to_multiple_assign(5, RoundingMode::Exact);
/// assert_eq!(x, -10);
///
/// let mut x = -10i64;
/// x.round_to_multiple_assign(3, RoundingMode::Nearest);
/// assert_eq!(x, -9);
///
/// let mut x = -20i128;
/// x.round_to_multiple_assign(3, RoundingMode::Nearest);
/// assert_eq!(x, -21);
///
/// let mut x = -10isize;
/// x.round_to_multiple_assign(4, RoundingMode::Nearest);
/// assert_eq!(x, -8);
///
/// let mut x = -14i8;
/// x.round_to_multiple_assign(4, RoundingMode::Nearest);
/// assert_eq!(x, -16);
///
/// let mut x = -10i16;
/// x.round_to_multiple_assign(-4, RoundingMode::Down);
/// assert_eq!(x, -8);
///
/// let mut x = -10i32;
/// x.round_to_multiple_assign(-4, RoundingMode::Up);
/// assert_eq!(x, -12);
///
/// let mut x = -10i64;
/// x.round_to_multiple_assign(-5, RoundingMode::Exact);
/// assert_eq!(x, -10);
///
/// let mut x = -10i128;
/// x.round_to_multiple_assign(-3, RoundingMode::Nearest);
/// assert_eq!(x, -9);
///
/// let mut x = -20isize;
/// x.round_to_multiple_assign(-3, RoundingMode::Nearest);
/// assert_eq!(x, -21);
///
/// let mut x = -10i8;
/// x.round_to_multiple_assign(-4, RoundingMode::Nearest);
/// assert_eq!(x, -8);
///
/// let mut x = -14i16;
/// x.round_to_multiple_assign(-4, RoundingMode::Nearest);
/// assert_eq!(x, -16);
/// ```
pub mod round_to_multiple;
/// This module contains functions for rounding a number to a multiple of a power of 2.
///
/// Here are usage examples of the macro-generated functions:
///
/// # round_to_multiple_of_power_of_2
/// ```
/// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2;
/// use malachite_base::rounding_modes::RoundingMode;
///
/// assert_eq!(10u8.round_to_multiple_of_power_of_2(2, RoundingMode::Floor), 8);
/// assert_eq!(10u8.round_to_multiple_of_power_of_2(2, RoundingMode::Ceiling), 12);
/// assert_eq!(10u8.round_to_multiple_of_power_of_2(2, RoundingMode::Down), 8);
/// assert_eq!(10u8.round_to_multiple_of_power_of_2(2, RoundingMode::Up), 12);
/// assert_eq!(10u8.round_to_multiple_of_power_of_2(2, RoundingMode::Nearest), 8);
/// assert_eq!(12u8.round_to_multiple_of_power_of_2(2, RoundingMode::Exact), 12);
/// assert_eq!((-10i8).round_to_multiple_of_power_of_2(2, RoundingMode::Floor), -12);
/// assert_eq!((-10i8).round_to_multiple_of_power_of_2(2, RoundingMode::Ceiling), -8);
/// assert_eq!((-10i8).round_to_multiple_of_power_of_2(2, RoundingMode::Down), -8);
/// assert_eq!((-10i8).round_to_multiple_of_power_of_2(2, RoundingMode::Up), -12);
/// assert_eq!((-10i8).round_to_multiple_of_power_of_2(2, RoundingMode::Nearest), -8);
/// assert_eq!((-12i8).round_to_multiple_of_power_of_2(2, RoundingMode::Exact), -12);
/// ```
///
/// # round_to_multiple_of_power_of_2_assign
/// ```
/// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2Assign;
/// use malachite_base::rounding_modes::RoundingMode;
///
/// let mut x = 10u8;
/// x.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Floor);
/// assert_eq!(x, 8);
///
/// let mut x = 10u8;
/// x.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Ceiling);
/// assert_eq!(x, 12);
///
/// let mut x = 10u8;
/// x.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Down);
/// assert_eq!(x, 8);
///
/// let mut x = 10u8;
/// x.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Up);
/// assert_eq!(x, 12);
///
/// let mut x = 10u8;
/// x.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Nearest);
/// assert_eq!(x, 8);
///
/// let mut x = 12u8;
/// x.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Exact);
/// assert_eq!(x, 12);
///
/// let mut x = -10i8;
/// x.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Floor);
/// assert_eq!(x, -12);
///
/// let mut x = -10i8;
/// x.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Ceiling);
/// assert_eq!(x, -8);
///
/// let mut x = -10i8;
/// x.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Down);
/// assert_eq!(x, -8);
///
/// let mut x = -10i8;
/// x.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Up);
/// assert_eq!(x, -12);
///
/// let mut x = -10i8;
/// x.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Nearest);
/// assert_eq!(x, -8);
///
/// let mut x = -12i8;
/// x.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Exact);
/// assert_eq!(x, -12);
/// ```
pub mod round_to_multiple_of_power_of_2;
/// This module contains functions for taking the absolute value of a number, saturating at numeric
/// bounds instead of overflowing.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions adding two numbers, saturating at numeric bounds instead of
/// overflowing.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for adding the product of two numbers to a number, saturating
/// at numeric bounds instead of overflowing.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for multiplying two numbers, saturating at numeric bounds
/// instead of overflowing.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for negating a number, saturating at numeric bounds instead of
/// overflowing.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for raising a number to a power, saturating at numeric bounds
/// instead of overflowing.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for squaring a number, saturating at numeric bounds instead of
/// overflowing.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for subtracting two numbers, saturating at numeric bounds
/// instead of overflowing.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for subtracting a number by the product of two numbers,
/// saturating at numeric bounds instead of overflowing.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for multiplying a number by a power of 2 and rounding according
/// to a specified rounding mode.
///
/// # shl_round
/// ```
/// use malachite_base::rounding_modes::RoundingMode;
/// use malachite_base::num::arithmetic::traits::ShlRound;
///
/// assert_eq!(0x101u16.shl_round(-8i8, RoundingMode::Down), 1);
/// assert_eq!(0x101u32.shl_round(-8i16, RoundingMode::Up), 2);
///
/// assert_eq!((-0x101i16).shl_round(-9i32, RoundingMode::Down), 0);
/// assert_eq!((-0x101i32).shl_round(-9i64, RoundingMode::Up), -1);
/// assert_eq!((-0x101i64).shl_round(-9i8, RoundingMode::Nearest), -1);
/// assert_eq!((-0xffi32).shl_round(-9i16, RoundingMode::Nearest), 0);
/// assert_eq!((-0x100i16).shl_round(-9i32, RoundingMode::Nearest), 0);
///
/// assert_eq!(0x100u64.shl_round(-8i64, RoundingMode::Exact), 1);
/// ```
///
/// # shl_round_assign
/// ```
/// use malachite_base::rounding_modes::RoundingMode;
/// use malachite_base::num::arithmetic::traits::ShlRoundAssign;
///
/// let mut x = 0x101u16;
/// x.shl_round_assign(-8i8, RoundingMode::Down);
/// assert_eq!(x, 1);
///
/// let mut x = 0x101u32;
/// x.shl_round_assign(-8i16, RoundingMode::Up);
/// assert_eq!(x, 2);
///
/// let mut x = -0x101i16;
/// x.shl_round_assign(-9i32, RoundingMode::Down);
/// assert_eq!(x, 0);
///
/// let mut x = -0x101i32;
/// x.shl_round_assign(-9i64, RoundingMode::Up);
/// assert_eq!(x, -1);
///
/// let mut x = -0x101i64;
/// x.shl_round_assign(-9i8, RoundingMode::Nearest);
/// assert_eq!(x, -1);
///
/// let mut x = -0xffi32;
/// x.shl_round_assign(-9i16, RoundingMode::Nearest);
/// assert_eq!(x, 0);
///
/// let mut x = -0x100i16;
/// x.shl_round_assign(-9i32, RoundingMode::Nearest);
/// assert_eq!(x, 0);
///
/// let mut x = 0x100u64;
/// x.shl_round_assign(-8i64, RoundingMode::Exact);
/// assert_eq!(x, 1);
/// ```
pub mod shl_round;
/// This module contains functions for multiplying a number by a power of 2 and rounding according
/// to a specified rounding mode.
///
/// # shr_round
/// ```
/// use malachite_base::rounding_modes::RoundingMode;
/// use malachite_base::num::arithmetic::traits::ShrRound;
///
/// assert_eq!(0x101u32.shr_round(8u8, RoundingMode::Down), 1);
/// assert_eq!(0x101u16.shr_round(8u16, RoundingMode::Up), 2);
///
/// assert_eq!(0x101u64.shr_round(9u32, RoundingMode::Down), 0);
/// assert_eq!(0x101u32.shr_round(9u64, RoundingMode::Up), 1);
/// assert_eq!(0x101u16.shr_round(9u8, RoundingMode::Nearest), 1);
/// assert_eq!(0xffu8.shr_round(9u16, RoundingMode::Nearest), 0);
/// assert_eq!(0x100u32.shr_round(9u32, RoundingMode::Nearest), 0);
///
/// assert_eq!(0x100u32.shr_round(8u64, RoundingMode::Exact), 1);
///
/// assert_eq!(0x101i32.shr_round(8u8, RoundingMode::Down), 1);
/// assert_eq!(0x101i16.shr_round(8u16, RoundingMode::Up), 2);
///
/// assert_eq!((-0x101i32).shr_round(9u32, RoundingMode::Down), 0);
/// assert_eq!((-0x101i64).shr_round(9u64, RoundingMode::Up), -1);
/// assert_eq!((-0x101i16).shr_round(9u8, RoundingMode::Nearest), -1);
/// assert_eq!((-0xffi32).shr_round(9u16, RoundingMode::Nearest), 0);
/// assert_eq!((-0x100i64).shr_round(9u32, RoundingMode::Nearest), 0);
///
/// assert_eq!(0x100i32.shr_round(8u64, RoundingMode::Exact), 1);
///
/// assert_eq!(0x101u32.shr_round(8i8, RoundingMode::Down), 1);
/// assert_eq!(0x101u16.shr_round(8i16, RoundingMode::Up), 2);
///
/// assert_eq!((-0x101i32).shr_round(9i32, RoundingMode::Down), 0);
/// assert_eq!((-0x101i64).shr_round(9i64, RoundingMode::Up), -1);
/// assert_eq!((-0x101i16).shr_round(9i8, RoundingMode::Nearest), -1);
/// assert_eq!((-0xffi32).shr_round(9i16, RoundingMode::Nearest), 0);
/// assert_eq!((-0x100i64).shr_round(9i32, RoundingMode::Nearest), 0);
///
/// assert_eq!(0x100u32.shr_round(8i64, RoundingMode::Exact), 1);
/// ```
///
/// # shr_round_assign
/// ```
/// use malachite_base::rounding_modes::RoundingMode;
/// use malachite_base::num::arithmetic::traits::ShrRoundAssign;
///
/// let mut x = 0x101u32;
/// x.shr_round_assign(8u8, RoundingMode::Down);
/// assert_eq!(x, 1);
///
/// let mut x = 0x101u16;
/// x.shr_round_assign(8u16, RoundingMode::Up);
/// assert_eq!(x, 2);
///
/// let mut x = 0x101u64;
/// x.shr_round_assign(9u32, RoundingMode::Down);
/// assert_eq!(x, 0);
///
/// let mut x = 0x101u32;
/// x.shr_round_assign(9u64, RoundingMode::Up);
/// assert_eq!(x, 1);
///
/// let mut x = 0x101u16;
/// x.shr_round_assign(9u8, RoundingMode::Nearest);
/// assert_eq!(x, 1);
///
/// let mut x = 0xffu8;
/// x.shr_round_assign(9u16, RoundingMode::Nearest);
/// assert_eq!(x, 0);
///
/// let mut x = 0x100u32;
/// x.shr_round_assign(9u32, RoundingMode::Nearest);
/// assert_eq!(x, 0);
///
/// let mut x = 0x100u32;
/// x.shr_round_assign(8u64, RoundingMode::Exact);
/// assert_eq!(x, 1);
///
/// let mut x = 0x101i32;
/// x.shr_round_assign(8u8, RoundingMode::Down);
/// assert_eq!(x, 1);
///
/// let mut x = 0x101i16;
/// x.shr_round_assign(8u16, RoundingMode::Up);
/// assert_eq!(x, 2);
///
/// let mut x = -0x101i32;
/// x.shr_round_assign(9u32, RoundingMode::Down);
/// assert_eq!(x, 0);
///
/// let mut x = -0x101i64;
/// x.shr_round_assign(9u64, RoundingMode::Up);
/// assert_eq!(x, -1);
///
/// let mut x = -0x101i16;
/// x.shr_round_assign(9u8, RoundingMode::Nearest);
/// assert_eq!(x, -1);
///
/// let mut x = -0xffi32;
/// x.shr_round_assign(9u16, RoundingMode::Nearest);
/// assert_eq!(x, 0);
///
/// let mut x = -0x100i64;
/// x.shr_round_assign(9u32, RoundingMode::Nearest);
/// assert_eq!(x, 0);
///
/// let mut x = 0x100u32;
/// x.shr_round_assign(8i64, RoundingMode::Exact);
/// assert_eq!(x, 1);
///
/// let mut x = 0x101u32;
/// x.shr_round_assign(8i8, RoundingMode::Down);
/// assert_eq!(x, 1);
///
/// let mut x = 0x101u16;
/// x.shr_round_assign(8i16, RoundingMode::Up);
/// assert_eq!(x, 2);
///
/// let mut x = -0x101i32;
/// x.shr_round_assign(9i32, RoundingMode::Down);
/// assert_eq!(x, 0);
///
/// let mut x = -0x101i64;
/// x.shr_round_assign(9i64, RoundingMode::Up);
/// assert_eq!(x, -1);
///
/// let mut x = -0x101i16;
/// x.shr_round_assign(9i8, RoundingMode::Nearest);
/// assert_eq!(x, -1);
///
/// let mut x = -0xffi32;
/// x.shr_round_assign(9i16, RoundingMode::Nearest);
/// assert_eq!(x, 0);
///
/// let mut x = -0x100i64;
/// x.shr_round_assign(9i32, RoundingMode::Nearest);
/// assert_eq!(x, 0);
///
/// let mut x = 0x100u32;
/// x.shr_round_assign(8i64, RoundingMode::Exact);
/// assert_eq!(x, 1);
/// ```
pub mod shr_round;
/// This module contains functions for determining the sign of a number.
///
/// Here are usage examples of the macro-generated functions:
///
/// # sign
/// ```
/// use malachite_base::num::arithmetic::traits::Sign;
/// use malachite_base::num::float::PrimitiveFloat;
/// use std::cmp::Ordering;
///
/// assert_eq!(0u8.sign(), Ordering::Equal);
/// assert_eq!(100u64.sign(), Ordering::Greater);
/// assert_eq!((-100i16).sign(), Ordering::Less);
///
/// assert_eq!(0.0.sign(), Ordering::Greater);
/// assert_eq!(1.0.sign(), Ordering::Greater);
/// assert_eq!(f64::POSITIVE_INFINITY.sign(), Ordering::Greater);
///
/// assert_eq!((-0.0).sign(), Ordering::Less);
/// assert_eq!((-1.0).sign(), Ordering::Less);
/// assert_eq!(f64::NEGATIVE_INFINITY.sign(), Ordering::Less);
///
/// assert_eq!(f64::NAN.sign(), Ordering::Equal);
/// ```
pub mod sign;
/// This module contains functions for taking the square root of a number.
///
/// Here are usage examples of the macro-generated functions:
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
/// assert_eq!(1000000000i32.sqrt_rem(), (31622, 49116));
/// assert_eq!(10000000000i64.sqrt_rem(), (100000, 0));
/// ```
///
/// # sqrt_rem_assign
/// ```
/// use malachite_base::num::arithmetic::traits::SqrtRemAssign;
///
/// let mut x = 99u8;
/// assert_eq!(x.sqrt_rem_assign(), 18);
/// assert_eq!(x, 9);
///
/// let mut x = 100u8;
/// assert_eq!(x.sqrt_rem_assign(), 0);
/// assert_eq!(x, 10);
///
/// let mut x = 101u8;
/// assert_eq!(x.sqrt_rem_assign(), 1);
/// assert_eq!(x, 10);
///
/// let mut x = 1000000000i32;
/// assert_eq!(x.sqrt_rem_assign(), 49116);
/// assert_eq!(x, 31622);
///
/// let mut x = 10000000000i64;
/// assert_eq!(x.sqrt_rem_assign(), 0);
/// assert_eq!(x, 100000);
/// ```
pub mod sqrt;
/// This module contains functions for squaring a number.
///
/// Here are usage examples of the macro-generated functions:
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
/// This module contains functions for subtracting the product of two numbers from a number.
///
/// Here are usage examples of the macro-generated functions:
///
/// # sub_mul
/// ```
/// use malachite_base::num::arithmetic::traits::SubMul;
///
/// assert_eq!(60u32.sub_mul(5, 10), 10);
/// assert_eq!(127i8.sub_mul(2, 100), -73);
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
/// ```
pub mod sub_mul;
pub mod traits;
pub mod wrapping_abs;
pub mod wrapping_add;
pub mod wrapping_add_mul;
pub mod wrapping_div;
pub mod wrapping_mul;
pub mod wrapping_neg;
pub mod wrapping_pow;
pub mod wrapping_square;
pub mod wrapping_sub;
pub mod wrapping_sub_mul;
pub mod x_mul_y_is_zz;
pub mod xx_add_yy_is_zz;
pub mod xx_div_mod_y_is_qr;
pub mod xx_sub_yy_is_zz;
pub mod xxx_add_yyy_is_zzz;
pub mod xxx_sub_yyy_is_zzz;
pub mod xxxx_add_yyyy_is_zzzz;
