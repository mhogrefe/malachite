/// This module contains functions for getting the absolute value of a number.
///
/// Here are usage examples of the macro-generated functions:
///
/// # abs_assign
/// ```
/// use malachite_base::num::arithmetic::traits::AbsAssign;
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
/// ```
///
/// # unsigned_abs
/// ```
/// use malachite_base::num::arithmetic::traits::UnsignedAbs;
///
/// assert_eq!(0i8.unsigned_abs(), 0u8);
/// assert_eq!(100i64.unsigned_abs(), 100u64);
/// assert_eq!((-100i64).unsigned_abs(), 100u64);
/// assert_eq!((-128i8).unsigned_abs(), 128u8);
/// ```
pub mod abs;
/// This module contains functions for adding a number and the product of two other numbers.
///
/// Here are usage examples of the macro-generated functions:
///
/// # add_mul_assign
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
pub mod checked_abs;
pub mod checked_add;
pub mod checked_add_mul;
pub mod checked_div;
pub mod checked_mul;
pub mod checked_neg;
pub mod checked_next_power_of_two;
pub mod checked_pow;
pub mod checked_square;
pub mod checked_sub;
pub mod checked_sub_mul;
pub mod div_exact;
pub mod div_mod;
pub mod div_round;
pub mod divisible_by;
pub mod divisible_by_power_of_two;
pub mod eq_mod;
pub mod eq_mod_power_of_two;
pub mod is_power_of_two;
pub mod mod_add;
pub mod mod_is_reduced;
pub mod mod_mul;
pub mod mod_neg;
pub mod mod_op;
pub mod mod_pow;
pub mod mod_power_of_two;
pub mod mod_power_of_two_add;
pub mod mod_power_of_two_is_reduced;
pub mod mod_power_of_two_mul;
pub mod mod_power_of_two_neg;
pub mod mod_power_of_two_pow;
pub mod mod_power_of_two_shl;
pub mod mod_power_of_two_shr;
pub mod mod_power_of_two_square;
pub mod mod_power_of_two_sub;
pub mod mod_shl;
pub mod mod_shr;
pub mod mod_square;
pub mod mod_sub;
pub mod neg;
pub mod next_power_of_two;
pub mod overflowing_abs;
pub mod overflowing_add;
pub mod overflowing_add_mul;
pub mod overflowing_div;
pub mod overflowing_mul;
pub mod overflowing_neg;
pub mod overflowing_pow;
pub mod overflowing_square;
pub mod overflowing_sub;
pub mod overflowing_sub_mul;
pub mod parity;
pub mod pow;
pub mod power_of_two;
pub mod round_to_multiple;
pub mod round_to_multiple_of_power_of_two;
pub mod saturating_abs;
pub mod saturating_add;
pub mod saturating_add_mul;
pub mod saturating_mul;
pub mod saturating_neg;
pub mod saturating_pow;
pub mod saturating_square;
pub mod saturating_sub;
pub mod saturating_sub_mul;
pub mod shl_round;
pub mod shr_round;
pub mod sign;
pub mod square;
pub mod sub_mul;
pub mod traits;
pub mod unsigneds;
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
