use misc::{CheckedFrom, CheckedInto, Max, Min, Named, Walkable, WrappingFrom, WrappingInto};
use round::RoundingMode;
use std;
use std::cmp::Ordering;
use std::fmt::{Binary, Debug, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex};
use std::hash::Hash;
use std::iter::{Product, Sum};
use std::num::{FpCategory, ParseIntError};
use std::ops::*;
use std::str::FromStr;

/// Converts a string slice in a given base to a value.
///
/// The string is expected to be an optional `+` sign followed by digits. Leading and trailing
/// whitespace represent an error. Digits are a subset of these characters, depending on `radix`:
///
/// * `0-9`
/// * `a-z`
/// * `A-Z`
pub trait FromStrRadix: Sized {
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError>;
}

/// Returns the number of ones in the binary representation of `self`.
pub trait CountOnes {
    fn count_ones(self) -> u32;
}

/// Returns the number of zeros in the binary representation of `self`.
pub trait CountZeros {
    fn count_zeros(self) -> u32;
}

/// Returns the number of leading zeros in the binary representation of `self`.
pub trait LeadingZeros {
    fn leading_zeros(self) -> u32;
}

/// Returns the number of trailing zeros in the binary representation of `self`.
pub trait TrailingZeros {
    fn trailing_zeros(self) -> u32;
}

/// Shifts the bits to the left by a specified amount, `n`, wrapping the truncated bits to the end
/// of the resulting value.
///
/// Please note this isn't the same operation as `<<`!
pub trait RotateLeft {
    fn rotate_left(self, n: u32) -> Self;
}

/// Shifts the bits to the right by a specified amount, `n`, wrapping the truncated bits to the end
/// of the resulting value.
///
/// Please note this isn't the same operation as `>>`!
pub trait RotateRight {
    fn rotate_right(self, n: u32) -> Self;
}

/// Defines functions for manipulating the endianness of a value.
pub trait Endian {
    /// Reverses the byte order of the value.
    fn swap_bytes(self) -> Self;

    /// Converts a value from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    fn from_be(x: Self) -> Self;

    /// Converts a value from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    fn from_le(x: Self) -> Self;

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    fn to_be(self) -> Self;

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    fn to_le(self) -> Self;
}

/// Checked addition. Computes `self + rhs`, returning `None` if there is no valid result.
pub trait CheckedAdd<RHS = Self> {
    type Output;

    fn checked_add(self, rhs: RHS) -> Option<Self::Output>;
}

/// Checked subtraction. Computes `self - rhs`, returning `None` if there is no valid result.
pub trait CheckedSub<RHS = Self> {
    type Output;

    fn checked_sub(self, rhs: RHS) -> Option<Self::Output>;
}

/// Checked multiplication. Computes `self * rhs`, returning `None` if there is no valid result.
pub trait CheckedMul<RHS = Self> {
    type Output;

    fn checked_mul(self, rhs: RHS) -> Option<Self::Output>;
}

/// Checked division. Computes `self / rhs`, returning `None` if there is no valid result.
pub trait CheckedDiv<RHS = Self> {
    type Output;

    fn checked_div(self, rhs: RHS) -> Option<Self::Output>;
}

/// Checked remainder. Computes `self % rhs`, returning `None` if there is no valid result.
pub trait CheckedRem<RHS = Self> {
    type Output;

    fn checked_rem(self, rhs: RHS) -> Option<Self::Output>;
}

/// Checked negation. Computes `-self`, returning `None` if there is no valid result.
pub trait CheckedNeg {
    type Output;

    fn checked_neg(self) -> Option<Self::Output>;
}

/// Checked shift left. Computes `self << rhs`, returning `None` if there is no valid result.
pub trait CheckedShl {
    type Output;

    fn checked_shl(self, rhs: u32) -> Option<Self::Output>;
}

/// Checked shift right. Computes `self >> rhs`, returning `None` if there is no valid result.
pub trait CheckedShr {
    type Output;

    fn checked_shr(self, rhs: u32) -> Option<Self::Output>;
}

/// Saturating addition. Computes `self + rhs`, saturating at the numeric bounds instead of
/// overflowing.
pub trait SaturatingAdd<RHS = Self> {
    type Output;

    fn saturating_add(self, rhs: RHS) -> Self::Output;
}

/// Saturating subtraction. Computes `self - rhs`, saturating at the numeric bounds instead of
/// overflowing.
pub trait SaturatingSub<RHS = Self> {
    type Output;

    fn saturating_sub(self, rhs: RHS) -> Self::Output;
}

/// Saturating multiplication. Computes `self * rhs`, saturating at the numeric bounds instead of
/// overflowing.
pub trait SaturatingMul<RHS = Self> {
    type Output;

    fn saturating_mul(self, rhs: RHS) -> Self::Output;
}

/// Wrapping (modular) addition. Computes `self + rhs`, wrapping around at the boundary of the type.
pub trait WrappingAdd<RHS = Self> {
    type Output;

    fn wrapping_add(self, rhs: RHS) -> Self::Output;
}

/// Wrapping (modular) subtraction. Computes `self - rhs`, wrapping around at the boundary of the
/// type.
pub trait WrappingSub<RHS = Self> {
    type Output;

    fn wrapping_sub(self, rhs: RHS) -> Self::Output;
}

/// Wrapping (modular) multiplication. Computes `self * rhs`, wrapping around at the boundary of the
/// type.
pub trait WrappingMul<RHS = Self> {
    type Output;

    fn wrapping_mul(self, rhs: RHS) -> Self::Output;
}

/// Wrapping (modular) division. Computes `self / rhs`, wrapping around at the boundary of the type.
pub trait WrappingDiv<RHS = Self> {
    type Output;

    fn wrapping_div(self, rhs: RHS) -> Self::Output;
}

/// Wrapping (modular) remainder. Computes `self % rhs`, wrapping around at the boundary of the
/// type.
pub trait WrappingRem<RHS = Self> {
    type Output;

    fn wrapping_rem(self, rhs: RHS) -> Self::Output;
}

/// Wrapping (modular) negation. Computes `-self`, wrapping around at the boundary of the type.
pub trait WrappingNeg {
    type Output;

    fn wrapping_neg(self) -> Self::Output;
}

/// Wrapping (modular) shift left. Computes `self << rhs`, wrapping around at the boundary of the
/// type.
pub trait WrappingShl {
    type Output;

    fn wrapping_shl(self, rhs: u32) -> Self::Output;
}

/// Wrapping (modular) shift right. Computes `self >> rhs`, wrapping around at the boundary of the
/// type.
pub trait WrappingShr {
    type Output;

    fn wrapping_shr(self, rhs: u32) -> Self::Output;
}

/// Calculates `self` + `rhs`.
///
/// Returns a tuple of the addition along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingAdd<RHS = Self> {
    type Output;

    fn overflowing_add(self, rhs: RHS) -> (Self::Output, bool);
}

/// Calculates `self` - `rhs`.
///
/// Returns a tuple of the subtraction along with a boolean indicating whether an arithmetic
/// overflow would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingSub<RHS = Self> {
    type Output;

    fn overflowing_sub(self, rhs: RHS) -> (Self::Output, bool);
}

/// Calculates `self` * `rhs`.
///
/// Returns a tuple of the multiplication along with a boolean indicating whether an arithmetic
/// overflow would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingMul<RHS = Self> {
    type Output;

    fn overflowing_mul(self, rhs: RHS) -> (Self::Output, bool);
}

/// Calculates `self` / `rhs`.
///
/// Returns a tuple of the quotient along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingDiv<RHS = Self> {
    type Output;

    fn overflowing_div(self, rhs: RHS) -> (Self::Output, bool);
}

/// Calculates `self` % `rhs`.
///
/// Returns a tuple of the remainder along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingRem<RHS = Self> {
    type Output;

    fn overflowing_rem(self, rhs: RHS) -> (Self::Output, bool);
}

/// Calculates -`self`.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingNeg {
    type Output;

    fn overflowing_neg(self) -> (Self::Output, bool);
}

/// Calculates `self` << `rhs`.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingShl {
    type Output;

    fn overflowing_shl(self, rhs: u32) -> (Self::Output, bool);
}

/// Calculates `self` >> `rhs`.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingShr {
    type Output;

    fn overflowing_shr(self, rhs: u32) -> (Self::Output, bool);
}

pub trait WrappingAddAssign<RHS = Self> {
    fn wrapping_add_assign(&mut self, rhs: RHS);
}

pub trait WrappingSubAssign<RHS = Self> {
    fn wrapping_sub_assign(&mut self, rhs: RHS);
}

pub trait WrappingDivAssign<RHS = Self> {
    fn wrapping_div_assign(&mut self, rhs: RHS);
}

pub trait WrappingMulAssign<RHS = Self> {
    fn wrapping_mul_assign(&mut self, rhs: RHS);
}

pub trait WrappingRemAssign<RHS = Self> {
    fn wrapping_rem_assign(&mut self, rhs: RHS);
}

pub trait SaturatingAddAssign<RHS = Self> {
    fn saturating_add_assign(&mut self, rhs: RHS);
}

pub trait SaturatingMulAssign<RHS = Self> {
    fn saturating_mul_assign(&mut self, rhs: RHS);
}

pub trait SaturatingSubAssign<RHS = Self> {
    fn saturating_sub_assign(&mut self, rhs: RHS);
}

pub trait OverflowingAddAssign<RHS = Self> {
    fn overflowing_add_assign(&mut self, rhs: RHS) -> bool;
}

pub trait OverflowingSubAssign<RHS = Self> {
    fn overflowing_sub_assign(&mut self, rhs: RHS) -> bool;
}

pub trait OverflowingDivAssign<RHS = Self> {
    fn overflowing_div_assign(&mut self, rhs: RHS) -> bool;
}

pub trait OverflowingMulAssign<RHS = Self> {
    fn overflowing_mul_assign(&mut self, rhs: RHS) -> bool;
}

pub trait OverflowingRemAssign<RHS = Self> {
    fn overflowing_rem_assign(&mut self, rhs: RHS) -> bool;
}

pub trait OverflowingNegAssign {
    fn overflowing_neg_assign(&mut self) -> bool;
}

/// Raises `self` to the power of `exp`.
pub trait Pow<RHS> {
    type Output;

    fn pow(self, exp: RHS) -> Self::Output;
}

//TODO WrappingPow, CheckedPow, SaturatingPow, OverflowingPow

/// Returns `true` iff `self == 2^k` for some integer `k`.
pub trait IsPowerOfTwo {
    fn is_power_of_two(self) -> bool;
}

/// Returns the smallest power of two greater than or equal to `self`.
pub trait NextPowerOfTwo {
    type Output;

    fn next_power_of_two(self) -> Self::Output;
}

pub trait NextPowerOfTwoAssign {
    fn next_power_of_two_assign(&mut self);
}

pub trait EqModPowerOfTwo<Rhs = Self> {
    fn eq_mod_power_of_two(self, other: Rhs, pow: u64) -> bool;
}

pub trait EqMod<Rhs = Self, Mod = Self> {
    fn eq_mod(self, other: Rhs, modulus: Mod) -> bool;
}

/// Returns the smallest power of two greater than or equal to `self`. If the next power of two is
/// greater than the type's maximum value, `None` is returned, otherwise the power of two is wrapped
/// in `Some`.
pub trait CheckedNextPowerOfTwo {
    type Output;

    fn checked_next_power_of_two(self) -> Option<Self::Output>;
}

/// Computes the absolute value of `self`.
pub trait Abs {
    type Output;

    fn abs(self) -> Self::Output;
}

/// Checked absolute value. Computes `self.abs()`, returning `None` if there is no valid result.
pub trait CheckedAbs {
    type Output;

    fn abs(self) -> Option<Self::Output>;
}

/// Wrapping (modular) absolute value. Computes `self.abs()`, wrapping around at the boundary of the
/// type.
pub trait WrappingAbs {
    type Output;

    fn abs(self) -> Self::Output;
}

/// Calculates `self.abs()`.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingAbs {
    type Output;

    fn abs(self) -> (Self::Output, bool);
}

pub trait WrappingNegAssign {
    fn wrapping_neg_assign(&mut self);
}

//TODO docs, test
pub trait BitScan {
    fn index_of_next_false_bit(self, starting_index: u64) -> Option<u64>;

    fn index_of_next_true_bit(self, starting_index: u64) -> Option<u64>;
}

pub trait Parity {
    fn even(self) -> bool;

    fn odd(self) -> bool;
}

pub trait DivisibleByPowerOfTwo {
    fn divisible_by_power_of_two(self, pow: u64) -> bool;
}

pub trait ModPowerOfTwo {
    type Output;

    fn mod_power_of_two(self, other: u64) -> Self::Output;
}

pub trait ModPowerOfTwoAssign {
    fn mod_power_of_two_assign(&mut self, other: u64);
}

pub trait NegModPowerOfTwo {
    type Output;

    fn neg_mod_power_of_two(self, other: u64) -> Self::Output;
}

pub trait NegModPowerOfTwoAssign {
    fn neg_mod_power_of_two_assign(&mut self, other: u64);
}

pub trait RemPowerOfTwo {
    type Output;

    fn rem_power_of_two(self, other: u64) -> Self::Output;
}

pub trait RemPowerOfTwoAssign {
    fn rem_power_of_two_assign(&mut self, other: u64);
}

pub trait CeilingModPowerOfTwo {
    type Output;

    fn ceiling_mod_power_of_two(self, other: u64) -> Self::Output;
}

pub trait CeilingModPowerOfTwoAssign {
    fn ceiling_mod_power_of_two_assign(&mut self, other: u64);
}

pub trait DivMod<RHS = Self> {
    type DivOutput;
    type ModOutput;

    fn div_mod(self, rhs: RHS) -> (Self::DivOutput, Self::ModOutput);
}

pub trait DivAssignMod<RHS = Self> {
    type ModOutput;

    fn div_assign_mod(&mut self, rhs: RHS) -> Self::ModOutput;
}

pub trait DivRem<RHS = Self> {
    type DivOutput;
    type RemOutput;

    fn div_rem(self, rhs: RHS) -> (Self::DivOutput, Self::RemOutput);
}

pub trait DivAssignRem<RHS = Self> {
    type RemOutput;

    fn div_assign_rem(&mut self, rhs: RHS) -> Self::RemOutput;
}

pub trait Mod<RHS = Self> {
    type Output;

    // would be called `mod`, but that's a keyword
    fn mod_op(self, rhs: RHS) -> Self::Output;
}

pub trait ModAssign<RHS = Self> {
    fn mod_assign(&mut self, rhs: RHS);
}

pub trait NegMod<RHS = Self> {
    type Output;

    fn neg_mod(self, rhs: RHS) -> Self::Output;
}

pub trait NegModAssign<RHS = Self> {
    fn neg_mod_assign(&mut self, rhs: RHS);
}

pub trait CeilingMod<RHS = Self> {
    type Output;

    fn ceiling_mod(self, rhs: RHS) -> Self::Output;
}

pub trait CeilingModAssign<RHS = Self> {
    fn ceiling_mod_assign(&mut self, rhs: RHS);
}

pub trait DivRound<RHS = Self> {
    type Output;

    fn div_round(self, rhs: RHS, rm: RoundingMode) -> Self::Output;
}

pub trait DivRoundAssign<RHS = Self> {
    fn div_round_assign(&mut self, rhs: RHS, rm: RoundingMode);
}

pub trait CeilingDivNegMod<RHS = Self> {
    type DivOutput;
    type ModOutput;

    fn ceiling_div_neg_mod(self, rhs: RHS) -> (Self::DivOutput, Self::ModOutput);
}

pub trait CeilingDivAssignNegMod<RHS = Self> {
    type ModOutput;

    fn ceiling_div_assign_neg_mod(&mut self, rhs: RHS) -> Self::ModOutput;
}

pub trait CeilingDivMod<RHS = Self> {
    type DivOutput;
    type ModOutput;

    fn ceiling_div_mod(self, rhs: RHS) -> (Self::DivOutput, Self::ModOutput);
}

pub trait CeilingDivAssignMod<RHS = Self> {
    type ModOutput;

    fn ceiling_div_assign_mod(&mut self, rhs: RHS) -> Self::ModOutput;
}

pub trait DivExact<RHS = Self> {
    type Output;

    fn div_exact(self, rhs: RHS) -> Self::Output;
}

pub trait DivExactAssign<RHS = Self> {
    fn div_exact_assign(&mut self, rhs: RHS);
}

pub trait DivisibleBy<RHS = Self> {
    fn divisible_by(self, rhs: RHS) -> bool;
}

pub trait Sign {
    fn sign(&self) -> Ordering;
}

//TODO is_positive, is_negative, sign

macro_rules! lossless_checked_from_impl {
    ($from:ty, $to:ty) => {
        impl CheckedFrom<$from> for $to {
            #[inline]
            fn checked_from(value: $from) -> Option<$to> {
                Some(value.into())
            }
        }
    };
}

macro_rules! lossy_checked_from_impl_a {
    ($from:ident, $to:ty) => {
        impl CheckedFrom<$from> for $to {
            #[allow(unused_comparisons)]
            #[inline]
            fn checked_from(value: $from) -> Option<$to> {
                let result = value as $to;
                if (result < 0) == (value < 0) && $from::from(result) == value {
                    Some(result)
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! lossy_checked_from_impl_b {
    ($from:ident, $to:ty) => {
        impl CheckedFrom<$from> for $to {
            #[allow(unused_comparisons, clippy::cast_lossless)]
            #[inline]
            fn checked_from(value: $from) -> Option<$to> {
                let result = value as $to;
                if (result < 0) == (value < 0) && result as $from == value {
                    Some(result)
                } else {
                    None
                }
            }
        }
    };
}

lossless_checked_from_impl!(u8, u8);
lossless_checked_from_impl!(u8, u16);
lossless_checked_from_impl!(u8, u32);
lossless_checked_from_impl!(u8, u64);
lossless_checked_from_impl!(u8, u128);
lossy_checked_from_impl_b!(u8, i8);
lossless_checked_from_impl!(u8, i16);
lossless_checked_from_impl!(u8, i32);
lossless_checked_from_impl!(u8, i64);
lossless_checked_from_impl!(u8, i128);
lossy_checked_from_impl_a!(u16, u8);
lossless_checked_from_impl!(u16, u16);
lossless_checked_from_impl!(u16, u32);
lossless_checked_from_impl!(u16, u64);
lossless_checked_from_impl!(u16, u128);
lossy_checked_from_impl_b!(u16, i8);
lossy_checked_from_impl_b!(u16, i16);
lossless_checked_from_impl!(u16, i32);
lossless_checked_from_impl!(u16, i64);
lossless_checked_from_impl!(u16, i128);
lossy_checked_from_impl_a!(u32, u8);
lossy_checked_from_impl_a!(u32, u16);
lossless_checked_from_impl!(u32, u32);
lossless_checked_from_impl!(u32, u64);
lossless_checked_from_impl!(u32, u128);
lossy_checked_from_impl_b!(u32, i8);
lossy_checked_from_impl_b!(u32, i16);
lossy_checked_from_impl_b!(u32, i32);
lossless_checked_from_impl!(u32, i64);
lossless_checked_from_impl!(u32, i128);
lossy_checked_from_impl_a!(u64, u8);
lossy_checked_from_impl_a!(u64, u16);
lossy_checked_from_impl_a!(u64, u32);
lossless_checked_from_impl!(u64, u64);
lossless_checked_from_impl!(u64, u128);
lossy_checked_from_impl_b!(u64, i8);
lossy_checked_from_impl_b!(u64, i16);
lossy_checked_from_impl_b!(u64, i32);
lossy_checked_from_impl_b!(u64, i64);
lossless_checked_from_impl!(u64, i128);
lossy_checked_from_impl_a!(u128, u8);
lossy_checked_from_impl_a!(u128, u16);
lossy_checked_from_impl_a!(u128, u32);
lossy_checked_from_impl_a!(u128, u64);
lossless_checked_from_impl!(u128, u128);
lossy_checked_from_impl_b!(u128, i8);
lossy_checked_from_impl_b!(u128, i16);
lossy_checked_from_impl_b!(u128, i32);
lossy_checked_from_impl_b!(u128, i64);
lossy_checked_from_impl_b!(u128, i128);
lossy_checked_from_impl_b!(i8, u8);
lossy_checked_from_impl_b!(i8, u16);
lossy_checked_from_impl_b!(i8, u32);
lossy_checked_from_impl_b!(i8, u64);
lossy_checked_from_impl_b!(i8, u128);
lossless_checked_from_impl!(i8, i8);
lossless_checked_from_impl!(i8, i16);
lossless_checked_from_impl!(i8, i32);
lossless_checked_from_impl!(i8, i64);
lossless_checked_from_impl!(i8, i128);
lossy_checked_from_impl_a!(i16, u8);
lossy_checked_from_impl_b!(i16, u16);
lossy_checked_from_impl_b!(i16, u32);
lossy_checked_from_impl_b!(i16, u64);
lossy_checked_from_impl_b!(i16, u128);
lossy_checked_from_impl_a!(i16, i8);
lossless_checked_from_impl!(i16, i16);
lossless_checked_from_impl!(i16, i32);
lossless_checked_from_impl!(i16, i64);
lossless_checked_from_impl!(i16, i128);
lossy_checked_from_impl_a!(i32, u8);
lossy_checked_from_impl_a!(i32, u16);
lossy_checked_from_impl_b!(i32, u32);
lossy_checked_from_impl_b!(i32, u64);
lossy_checked_from_impl_b!(i32, u128);
lossy_checked_from_impl_a!(i32, i8);
lossy_checked_from_impl_a!(i32, i16);
lossless_checked_from_impl!(i32, i32);
lossless_checked_from_impl!(i32, i64);
lossless_checked_from_impl!(i32, i128);
lossy_checked_from_impl_a!(i64, u8);
lossy_checked_from_impl_a!(i64, u16);
lossy_checked_from_impl_a!(i64, u32);
lossy_checked_from_impl_b!(i64, u64);
lossy_checked_from_impl_b!(i64, u128);
lossy_checked_from_impl_a!(i64, i8);
lossy_checked_from_impl_a!(i64, i16);
lossy_checked_from_impl_a!(i64, i32);
lossless_checked_from_impl!(i64, i64);
lossless_checked_from_impl!(i64, i128);
lossy_checked_from_impl_a!(i128, u8);
lossy_checked_from_impl_a!(i128, u16);
lossy_checked_from_impl_a!(i128, u32);
lossy_checked_from_impl_b!(i128, u64);
lossy_checked_from_impl_b!(i128, u128);
lossy_checked_from_impl_a!(i128, i8);
lossy_checked_from_impl_a!(i128, i16);
lossy_checked_from_impl_a!(i128, i32);
lossy_checked_from_impl_a!(i128, i64);
lossless_checked_from_impl!(i128, i128);

macro_rules! wrapping_impl_inner {
    ($from:ty, $to:ty) => {
        #[allow(clippy::cast_lossless)]
        impl WrappingFrom<$from> for $to {
            #[inline]
            fn wrapping_from(value: $from) -> $to {
                value as $to
            }
        }
    };
}

macro_rules! wrapping_impl {
    ($from:ty) => {
        wrapping_impl_inner!($from, u8);
        wrapping_impl_inner!($from, u16);
        wrapping_impl_inner!($from, u32);
        wrapping_impl_inner!($from, u64);
        wrapping_impl_inner!($from, u128);
        wrapping_impl_inner!($from, i8);
        wrapping_impl_inner!($from, i16);
        wrapping_impl_inner!($from, i32);
        wrapping_impl_inner!($from, i64);
        wrapping_impl_inner!($from, i128);
    };
}

wrapping_impl!(u8);
wrapping_impl!(u16);
wrapping_impl!(u32);
wrapping_impl!(u64);
wrapping_impl!(u128);
wrapping_impl!(i8);
wrapping_impl!(i16);
wrapping_impl!(i32);
wrapping_impl!(i64);
wrapping_impl!(i128);

impl NotAssign for bool {
    fn not_assign(&mut self) {
        *self = !*self
    }
}

impl NegAssign for isize {
    fn neg_assign(&mut self) {
        *self = -*self
    }
}

impl NotAssign for isize {
    fn not_assign(&mut self) {
        *self = !*self
    }
}

//TODO docs
pub trait PrimitiveInteger:
    'static
    + Add<Output = Self>
    + AddAssign<Self>
    + Binary
    + BitAccess
    + BitAnd<Output = Self>
    + BitAndAssign<Self>
    + BitOr<Output = Self>
    + BitOrAssign<Self>
    + BitScan
    + BitXor<Output = Self>
    + BitXorAssign<Self>
    + CheckedAdd<Output = Self>
    + CheckedDiv<Output = Self>
    + CheckedFrom<u8>
    + CheckedFrom<u16>
    + CheckedFrom<u32>
    + CheckedFrom<u64>
    + CheckedFrom<u128>
    + CheckedFrom<i8>
    + CheckedFrom<i16>
    + CheckedFrom<i32>
    + CheckedFrom<i64>
    + CheckedFrom<i128>
    + CheckedInto<u8>
    + CheckedInto<u16>
    + CheckedInto<u32>
    + CheckedInto<u64>
    + CheckedInto<u128>
    + CheckedInto<i8>
    + CheckedInto<i16>
    + CheckedInto<i32>
    + CheckedInto<i64>
    + CheckedInto<i128>
    + CheckedMul<Output = Self>
    + CheckedNeg<Output = Self>
    + CheckedRem<Output = Self>
    + CheckedShl<Output = Self>
    + CheckedShr<Output = Self>
    + CheckedSub<Output = Self>
    + Clone
    + Copy
    + CountOnes
    + CountZeros
    + Debug
    + Default
    + Display
    + Div<Output = Self>
    + DivAssign
    + DivAssignMod<ModOutput = Self>
    + DivAssignRem<RemOutput = Self>
    + DivExact
    + DivExactAssign
    + DivisibleBy
    + DivisibleByPowerOfTwo
    + DivMod
    + DivRem<DivOutput = Self, RemOutput = Self>
    + DivRound<Output = Self>
    + DivRoundAssign
    + Endian
    + Eq
    + EqMod<Self, Self>
    + EqModPowerOfTwo<Self>
    + FromStr
    + HammingDistance<Self>
    + Hash
    + LeadingZeros
    + LowerHex
    + Min
    + Max
    + Mod
    + ModAssign
    + Mul<Output = Self>
    + MulAssign<Self>
    + Named
    + Not<Output = Self>
    + NotAssign
    + Octal
    + One
    + Ord
    + OrdAbs
    + OverflowingAdd<Output = Self>
    + OverflowingAddAssign
    + OverflowingDiv<Output = Self>
    + OverflowingDivAssign
    + OverflowingMul<Output = Self>
    + OverflowingMulAssign
    + OverflowingNeg<Output = Self>
    + OverflowingNegAssign
    + OverflowingRem<Output = Self>
    + OverflowingRemAssign
    + OverflowingShl<Output = Self>
    + OverflowingShr<Output = Self>
    + OverflowingSub<Output = Self>
    + Parity
    + PartialEq<Self>
    + PartialOrd<Self>
    + PartialOrdAbs<Self>
    + Pow<u32>
    + Product
    + Rem<Output = Self>
    + RemAssign<Self>
    + RotateLeft
    + RotateRight
    + SaturatingAdd<Output = Self>
    + SaturatingAddAssign
    + SaturatingMul<Output = Self>
    + SaturatingMulAssign
    + SaturatingSub<Output = Self>
    + SaturatingSubAssign
    + Shl<i8, Output = Self>
    + Shl<i16, Output = Self>
    + Shl<i32, Output = Self>
    + Shl<i64, Output = Self>
    + Shl<i128, Output = Self>
    + Shl<u8, Output = Self>
    + Shl<u16, Output = Self>
    + Shl<u32, Output = Self>
    + Shl<u64, Output = Self>
    + Shl<u128, Output = Self>
    + ShlAssign<i8>
    + ShlAssign<i16>
    + ShlAssign<i32>
    + ShlAssign<i64>
    + ShlAssign<i128>
    + ShlAssign<u8>
    + ShlAssign<u16>
    + ShlAssign<u32>
    + ShlAssign<u64>
    + ShlAssign<u128>
    + ShlRound<i8, Output = Self>
    + ShlRound<i16, Output = Self>
    + ShlRound<i32, Output = Self>
    + ShlRound<i64, Output = Self>
    + ShlRound<i128, Output = Self>
    + ShlRoundAssign<i8>
    + ShlRoundAssign<i16>
    + ShlRoundAssign<i32>
    + ShlRoundAssign<i64>
    + ShlRoundAssign<i128>
    + Shr<i8, Output = Self>
    + Shr<i16, Output = Self>
    + Shr<i32, Output = Self>
    + Shr<i64, Output = Self>
    + Shr<i128, Output = Self>
    + Shr<u8, Output = Self>
    + Shr<u16, Output = Self>
    + Shr<u32, Output = Self>
    + Shr<u64, Output = Self>
    + Shr<u128, Output = Self>
    + ShrAssign<i8>
    + ShrAssign<i16>
    + ShrAssign<i32>
    + ShrAssign<i64>
    + ShrAssign<i128>
    + ShrAssign<u8>
    + ShrAssign<u16>
    + ShrAssign<u32>
    + ShrAssign<u64>
    + ShrAssign<u128>
    + ShrRound<i8, Output = Self>
    + ShrRound<i16, Output = Self>
    + ShrRound<i32, Output = Self>
    + ShrRound<i64, Output = Self>
    + ShrRound<i128, Output = Self>
    + ShrRound<u8, Output = Self>
    + ShrRound<u16, Output = Self>
    + ShrRound<u32, Output = Self>
    + ShrRound<u64, Output = Self>
    + ShrRound<u128, Output = Self>
    + ShrRoundAssign<i8>
    + ShrRoundAssign<i16>
    + ShrRoundAssign<i32>
    + ShrRoundAssign<i64>
    + ShrRoundAssign<i128>
    + ShrRoundAssign<u8>
    + ShrRoundAssign<u16>
    + ShrRoundAssign<u32>
    + ShrRoundAssign<u64>
    + ShrRoundAssign<u128>
    + SignificantBits
    + Sized
    + Sub<Output = Self>
    + SubAssign<Self>
    + Sum<Self>
    + TrailingZeros
    + UpperHex
    + Walkable
    + WrappingAdd<Output = Self>
    + WrappingAddAssign
    + WrappingDiv<Output = Self>
    + WrappingDivAssign
    + WrappingFrom<u8>
    + WrappingFrom<u16>
    + WrappingFrom<u32>
    + WrappingFrom<u64>
    + WrappingFrom<u128>
    + WrappingFrom<i8>
    + WrappingFrom<i16>
    + WrappingFrom<i32>
    + WrappingFrom<i64>
    + WrappingFrom<i128>
    + WrappingInto<u8>
    + WrappingInto<u16>
    + WrappingInto<u32>
    + WrappingInto<u64>
    + WrappingInto<u128>
    + WrappingInto<i8>
    + WrappingInto<i16>
    + WrappingInto<i32>
    + WrappingInto<i64>
    + WrappingInto<i128>
    + WrappingMul<Output = Self>
    + WrappingMulAssign
    + WrappingNeg<Output = Self>
    + WrappingNegAssign
    + WrappingRem<Output = Self>
    + WrappingRemAssign
    + WrappingShl<Output = Self>
    + WrappingShr<Output = Self>
    + WrappingSub<Output = Self>
    + WrappingSubAssign
    + Zero
{
    const LOG_WIDTH: u32;
    const WIDTH: u32 = 1 << Self::LOG_WIDTH;
    const WIDTH_MASK: u32 = Self::WIDTH - 1;

    //TODO test
    fn get_highest_bit(&self) -> bool {
        self.get_bit(u64::from(Self::WIDTH - 1))
    }
}

//TODO docs
pub trait PrimitiveUnsigned:
    CeilingLogTwo
    + CeilingDivNegMod
    + CeilingDivAssignNegMod
    + CheckedNextPowerOfTwo<Output = Self>
    + FloorLogTwo
    + From<u8>
    + FromU32Slice
    + Into<u128>
    + IsPowerOfTwo
    + ModPowerOfTwo<Output = Self>
    + ModPowerOfTwoAssign
    + NegMod
    + NegModAssign
    + NextPowerOfTwo<Output = Self>
    + NextPowerOfTwoAssign
    + PrimitiveInteger
    + RemPowerOfTwo<Output = Self>
    + RemPowerOfTwoAssign
{
    type SignedOfEqualWidth: PrimitiveSigned;

    fn to_signed_bitwise(self) -> Self::SignedOfEqualWidth;

    fn to_signed_checked(self) -> Option<Self::SignedOfEqualWidth>;

    fn from_signed_bitwise(i: Self::SignedOfEqualWidth) -> Self;
}

//TODO docs
pub trait PrimitiveSigned:
    Abs<Output = Self>
    + CeilingMod
    + CeilingModAssign
    + CheckedAbs<Output = Self>
    + From<i8>
    + Into<i128>
    + Neg<Output = Self>
    + NegAssign
    + NegativeOne
    + OverflowingAbs<Output = Self>
    + PrimitiveInteger
    + Sign
    + UnsignedAbs
    + WrappingAbs<Output = Self>
{
    type UnsignedOfEqualWidth: PrimitiveUnsigned;

    fn to_unsigned_bitwise(self) -> Self::UnsignedOfEqualWidth;

    fn from_unsigned_bitwise(u: Self::UnsignedOfEqualWidth) -> Self;
}

//TODO docs
pub trait PrimitiveFloat:
    'static
    + Add<Output = Self>
    + AddAssign<Self>
    + Copy
    + Debug
    + Default
    + Display
    + Div<Output = Self>
    + DivAssign
    + Display
    + FromStr
    + LowerExp
    + Min
    + Max
    + Mul<Output = Self>
    + MulAssign<Self>
    + Named
    + Neg<Output = Self>
    + NegAssign
    + One
    + PartialEq<Self>
    + PartialOrd<Self>
    + Product
    + Rem<Output = Self>
    + RemAssign<Self>
    + Sized
    + Sub<Output = Self>
    + SubAssign<Self>
    + Sum<Self>
    + UpperExp
    + Zero
{
    type UnsignedOfEqualWidth: PrimitiveUnsigned;
    type SignedOfEqualWidth: PrimitiveSigned;

    const WIDTH: u32 = Self::UnsignedOfEqualWidth::WIDTH;
    const EXPONENT_WIDTH: u32 = Self::WIDTH - Self::MANTISSA_WIDTH - 1;
    const MANTISSA_WIDTH: u32;
    const MIN_NORMAL_EXPONENT: i32 = -((1 << (Self::EXPONENT_WIDTH - 1)) - 2);
    const MIN_EXPONENT: i32 = Self::MIN_NORMAL_EXPONENT - (Self::MANTISSA_WIDTH as i32);
    const MAX_EXPONENT: u32 = (1 << (Self::EXPONENT_WIDTH - 1)) - 1;

    const INFINITY: Self;
    const NEG_INFINITY: Self;
    const NEG_ZERO: Self;
    const NAN: Self;
    const MAX_FINITE: Self;
    const MIN_FINITE: Self;

    fn is_nan(self) -> bool;

    fn is_infinite(self) -> bool;

    fn is_finite(self) -> bool;

    fn is_normal(self) -> bool;

    fn classify(self) -> FpCategory;

    fn is_sign_positive(self) -> bool;

    fn is_sign_negative(self) -> bool;

    fn to_bits(self) -> Self::UnsignedOfEqualWidth;

    fn from_bits(v: Self::UnsignedOfEqualWidth) -> Self;

    fn to_adjusted_mantissa_and_exponent(self) -> (Self::UnsignedOfEqualWidth, u32) {
        let bits = self.to_bits();
        let mantissa = bits.mod_power_of_two(Self::MANTISSA_WIDTH.into());
        let exponent: u32 = (bits >> Self::MANTISSA_WIDTH).checked_into().unwrap();
        let exponent = exponent.mod_power_of_two(Self::EXPONENT_WIDTH.into());
        (mantissa, exponent)
    }

    fn from_adjusted_mantissa_and_exponent(
        mantissa: Self::UnsignedOfEqualWidth,
        exponent: u32,
    ) -> Self {
        Self::from_bits(
            (Self::UnsignedOfEqualWidth::checked_from(exponent).unwrap() << Self::MANTISSA_WIDTH)
                + mantissa,
        )
    }
}

/// This trait defines functions that access or modify individual bits in a value, indexed by a
/// `u64`.
pub trait BitAccess {
    /// Determines whether the bit at `index` is true or false.
    fn get_bit(&self, index: u64) -> bool;

    /// Sets the bit at `index` to true.
    fn set_bit(&mut self, index: u64);

    /// Sets the bit at `index` to false.
    fn clear_bit(&mut self, index: u64);

    /// Sets the bit at `index` to whichever value `bit` is.
    ///
    /// Time: worst case O(max(f(n), g(n))), where f(n) is the worst-case time complexity of
    ///     `Self::set_bit` and g(n) is the worst-case time complexity of `Self::clear_bit`.
    ///
    /// Additional memory: worst case O(max(f(n), g(n))), where f(n) is the worst-case
    ///     additional-memory complexity of `Self::set_bit` and g(n) is the worst-case
    ///     additional-memory complexity of `Self::clear_bit`.
    ///
    /// # Panics
    /// See panics for `set_bit` and `assign_bit`.
    #[inline]
    fn assign_bit(&mut self, index: u64, bit: bool) {
        if bit {
            self.set_bit(index);
        } else {
            self.clear_bit(index);
        }
    }

    /// Sets the bit at `index` to the opposite of its previous value.
    ///
    /// Time: worst case O(f(n) + max(g(n), h(n))), where f(n) is the worst-case time complexity of
    ///     `Self::get_bit`, g(n) is the worst-case time complexity of `Self::set_bit`, and h(n) is
    ///     the worst-case time complexity of `Self::clear_bit`.
    ///
    /// Additional memory: worst case O(f(n) + max(g(n), h(n))), where f(n) is the worst-case
    ///     additional-memory complexity of `Self::get_bit`, g(n) is the worst-case
    ///     additional-memory complexity of `Self::set_bit`, and h(n) is the worst-case
    ///     additional-memory complexity of `Self::clear_bit`.
    ///
    /// # Panics
    /// See panics for `get_bit`, `set_bit` and `assign_bit`.
    #[inline]
    fn flip_bit(&mut self, index: u64) {
        if self.get_bit(index) {
            self.clear_bit(index);
        } else {
            self.set_bit(index);
        }
    }
}

//TODO docs
macro_rules! integer_traits {
    ($t:ident, $log_width:expr) => {
        //TODO docs
        impl PrimitiveInteger for $t {
            const LOG_WIDTH: u32 = $log_width;
        }

        impl_named!($t);

        impl PartialOrdAbs<$t> for $t {
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                Some(self.cmp_abs(other))
            }
        }

        impl FromStrRadix for $t {
            #[inline]
            fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
                $t::from_str_radix(src, radix)
            }
        }

        impl CountZeros for $t {
            #[inline]
            fn count_zeros(self) -> u32 {
                $t::count_zeros(self)
            }
        }

        impl CountOnes for $t {
            #[inline]
            fn count_ones(self) -> u32 {
                $t::count_ones(self)
            }
        }

        impl LeadingZeros for $t {
            #[inline]
            fn leading_zeros(self) -> u32 {
                $t::leading_zeros(self)
            }
        }

        impl TrailingZeros for $t {
            #[inline]
            fn trailing_zeros(self) -> u32 {
                $t::trailing_zeros(self)
            }
        }

        impl RotateLeft for $t {
            #[inline]
            fn rotate_left(self, n: u32) -> $t {
                $t::rotate_left(self, n)
            }
        }

        impl RotateRight for $t {
            #[inline]
            fn rotate_right(self, n: u32) -> $t {
                $t::rotate_right(self, n)
            }
        }

        impl Endian for $t {
            #[inline]
            fn swap_bytes(self) -> $t {
                $t::swap_bytes(self)
            }

            #[inline]
            fn from_be(x: $t) -> $t {
                $t::from_be(x)
            }

            #[inline]
            fn from_le(x: $t) -> $t {
                $t::from_le(x)
            }

            #[inline]
            fn to_be(self) -> $t {
                $t::to_be(self)
            }

            #[inline]
            fn to_le(self) -> $t {
                $t::to_le(self)
            }
        }

        impl CheckedAdd<$t> for $t {
            type Output = $t;

            #[inline]
            fn checked_add(self, rhs: $t) -> Option<$t> {
                $t::checked_add(self, rhs)
            }
        }

        impl CheckedSub<$t> for $t {
            type Output = $t;

            #[inline]
            fn checked_sub(self, rhs: $t) -> Option<$t> {
                $t::checked_sub(self, rhs)
            }
        }

        impl CheckedMul<$t> for $t {
            type Output = $t;

            #[inline]
            fn checked_mul(self, rhs: $t) -> Option<$t> {
                $t::checked_mul(self, rhs)
            }
        }

        impl CheckedDiv<$t> for $t {
            type Output = $t;

            #[inline]
            fn checked_div(self, rhs: $t) -> Option<$t> {
                $t::checked_div(self, rhs)
            }
        }

        impl CheckedRem<$t> for $t {
            type Output = $t;

            #[inline]
            fn checked_rem(self, rhs: $t) -> Option<$t> {
                $t::checked_rem(self, rhs)
            }
        }

        impl CheckedNeg for $t {
            type Output = $t;

            #[inline]
            fn checked_neg(self) -> Option<$t> {
                $t::checked_neg(self)
            }
        }

        impl CheckedShl for $t {
            type Output = $t;

            #[inline]
            fn checked_shl(self, rhs: u32) -> Option<$t> {
                $t::checked_shl(self, rhs)
            }
        }

        impl CheckedShr for $t {
            type Output = $t;

            #[inline]
            fn checked_shr(self, rhs: u32) -> Option<$t> {
                $t::checked_shr(self, rhs)
            }
        }

        impl SaturatingAdd<$t> for $t {
            type Output = $t;

            #[inline]
            fn saturating_add(self, rhs: $t) -> $t {
                $t::saturating_add(self, rhs)
            }
        }

        impl SaturatingSub<$t> for $t {
            type Output = $t;

            #[inline]
            fn saturating_sub(self, rhs: $t) -> $t {
                $t::saturating_sub(self, rhs)
            }
        }

        impl SaturatingMul<$t> for $t {
            type Output = $t;

            #[inline]
            fn saturating_mul(self, rhs: $t) -> $t {
                $t::saturating_mul(self, rhs)
            }
        }

        impl WrappingAdd<$t> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_add(self, rhs: $t) -> $t {
                $t::wrapping_add(self, rhs)
            }
        }

        impl WrappingSub<$t> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_sub(self, rhs: $t) -> $t {
                $t::wrapping_sub(self, rhs)
            }
        }

        impl WrappingMul<$t> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_mul(self, rhs: $t) -> $t {
                $t::wrapping_mul(self, rhs)
            }
        }

        impl WrappingDiv<$t> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_div(self, rhs: $t) -> $t {
                $t::wrapping_div(self, rhs)
            }
        }

        impl WrappingRem<$t> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_rem(self, rhs: $t) -> $t {
                $t::wrapping_rem(self, rhs)
            }
        }

        impl WrappingNeg for $t {
            type Output = $t;

            #[inline]
            fn wrapping_neg(self) -> $t {
                $t::wrapping_neg(self)
            }
        }

        impl WrappingShl for $t {
            type Output = $t;

            #[inline]
            fn wrapping_shl(self, rhs: u32) -> $t {
                $t::wrapping_shl(self, rhs)
            }
        }

        impl WrappingShr for $t {
            type Output = $t;

            #[inline]
            fn wrapping_shr(self, rhs: u32) -> $t {
                $t::wrapping_shr(self, rhs)
            }
        }

        impl OverflowingAdd<$t> for $t {
            type Output = $t;

            #[inline]
            fn overflowing_add(self, rhs: $t) -> ($t, bool) {
                $t::overflowing_add(self, rhs)
            }
        }

        impl OverflowingSub<$t> for $t {
            type Output = $t;

            #[inline]
            fn overflowing_sub(self, rhs: $t) -> ($t, bool) {
                $t::overflowing_sub(self, rhs)
            }
        }

        impl OverflowingMul<$t> for $t {
            type Output = $t;

            #[inline]
            fn overflowing_mul(self, rhs: $t) -> ($t, bool) {
                $t::overflowing_mul(self, rhs)
            }
        }

        impl OverflowingDiv<$t> for $t {
            type Output = $t;

            #[inline]
            fn overflowing_div(self, rhs: $t) -> ($t, bool) {
                $t::overflowing_div(self, rhs)
            }
        }

        impl OverflowingRem<$t> for $t {
            type Output = $t;

            #[inline]
            fn overflowing_rem(self, rhs: $t) -> ($t, bool) {
                $t::overflowing_rem(self, rhs)
            }
        }

        impl OverflowingNeg for $t {
            type Output = $t;

            #[inline]
            fn overflowing_neg(self) -> ($t, bool) {
                $t::overflowing_neg(self)
            }
        }

        impl OverflowingShl for $t {
            type Output = $t;

            #[inline]
            fn overflowing_shl(self, rhs: u32) -> ($t, bool) {
                $t::overflowing_shl(self, rhs)
            }
        }

        impl OverflowingShr for $t {
            type Output = $t;

            #[inline]
            fn overflowing_shr(self, rhs: u32) -> ($t, bool) {
                $t::overflowing_shr(self, rhs)
            }
        }

        impl WrappingAddAssign for $t {
            #[inline]
            fn wrapping_add_assign(&mut self, rhs: $t) {
                *self = self.wrapping_add(rhs);
            }
        }

        impl WrappingSubAssign for $t {
            #[inline]
            fn wrapping_sub_assign(&mut self, rhs: $t) {
                *self = self.wrapping_sub(rhs);
            }
        }

        impl WrappingMulAssign for $t {
            #[inline]
            fn wrapping_mul_assign(&mut self, rhs: $t) {
                *self = self.wrapping_mul(rhs);
            }
        }

        impl WrappingDivAssign for $t {
            #[inline]
            fn wrapping_div_assign(&mut self, rhs: $t) {
                *self = self.wrapping_div(rhs);
            }
        }

        impl WrappingRemAssign for $t {
            #[inline]
            fn wrapping_rem_assign(&mut self, rhs: $t) {
                *self = self.wrapping_rem(rhs);
            }
        }

        impl OverflowingAddAssign for $t {
            #[inline]
            fn overflowing_add_assign(&mut self, rhs: $t) -> bool {
                let (result, overflow) = self.overflowing_add(rhs);
                *self = result;
                overflow
            }
        }

        impl OverflowingSubAssign for $t {
            #[inline]
            fn overflowing_sub_assign(&mut self, rhs: $t) -> bool {
                let (result, overflow) = self.overflowing_sub(rhs);
                *self = result;
                overflow
            }
        }

        impl OverflowingMulAssign for $t {
            #[inline]
            fn overflowing_mul_assign(&mut self, rhs: $t) -> bool {
                let (result, overflow) = self.overflowing_mul(rhs);
                *self = result;
                overflow
            }
        }

        impl OverflowingDivAssign for $t {
            #[inline]
            fn overflowing_div_assign(&mut self, rhs: $t) -> bool {
                let (result, overflow) = self.overflowing_div(rhs);
                *self = result;
                overflow
            }
        }

        impl OverflowingRemAssign for $t {
            #[inline]
            fn overflowing_rem_assign(&mut self, rhs: $t) -> bool {
                let (result, overflow) = self.overflowing_rem(rhs);
                *self = result;
                overflow
            }
        }

        impl OverflowingNegAssign for $t {
            #[inline]
            fn overflowing_neg_assign(&mut self) -> bool {
                let (result, overflow) = self.overflowing_neg();
                *self = result;
                overflow
            }
        }

        impl SaturatingAddAssign for $t {
            #[inline]
            fn saturating_add_assign(&mut self, rhs: $t) {
                *self = self.saturating_add(rhs);
            }
        }

        impl SaturatingSubAssign for $t {
            #[inline]
            fn saturating_sub_assign(&mut self, rhs: $t) {
                *self = self.saturating_sub(rhs);
            }
        }

        impl SaturatingMulAssign for $t {
            #[inline]
            fn saturating_mul_assign(&mut self, rhs: $t) {
                *self = self.saturating_mul(rhs);
            }
        }

        impl Pow<u32> for $t {
            type Output = $t;

            #[inline]
            fn pow(self, exp: u32) -> $t {
                $t::pow(self, exp)
            }
        }

        impl Min for $t {
            const MIN: $t = std::$t::MIN;
        }

        impl Max for $t {
            const MAX: $t = std::$t::MAX;
        }

        impl HammingDistance<$t> for $t {
            #[inline]
            fn hamming_distance(self, other: $t) -> u64 {
                u64::from((self ^ other).count_ones())
            }
        }

        impl Walkable for $t {
            /// Increments `self`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `self` == `self::MAX`.
            ///
            /// # Example
            /// ```
            /// use malachite_base::misc::Walkable;
            ///
            /// fn main() {
            ///     let mut i = 10;
            ///     i.increment();
            ///     assert_eq!(i, 11);
            ///
            ///     let mut i = -5;
            ///     i.increment();
            ///     assert_eq!(i, -4);
            /// }
            /// ```
            #[inline]
            fn increment(&mut self) {
                *self = self
                    .checked_add(1)
                    .expect("Cannot increment past the maximum value.");
            }

            /// Decrements `self`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `self` == `self::MIN`.
            ///
            /// # Example
            /// ```
            /// use malachite_base::misc::Walkable;
            ///
            /// fn main() {
            ///     let mut i = 10;
            ///     i.decrement();
            ///     assert_eq!(i, 9);
            ///
            ///     let mut i = -5;
            ///     i.decrement();
            ///     assert_eq!(i, -6);
            /// }
            /// ```
            #[inline]
            fn decrement(&mut self) {
                *self = self
                    .checked_sub(1)
                    .expect("Cannot decrement past the minimum value.");
            }
        }

        //TODO docs, test
        impl NotAssign for $t {
            #[inline]
            fn not_assign(&mut self) {
                *self = !*self;
            }
        }

        //TODO docs, test
        impl WrappingNegAssign for $t {
            #[inline]
            fn wrapping_neg_assign(&mut self) {
                *self = self.wrapping_neg();
            }
        }

        impl Parity for $t {
            #[inline]
            fn even(self) -> bool {
                (self & 1) == 0
            }

            #[inline]
            fn odd(self) -> bool {
                (self & 1) != 0
            }
        }

        impl EqModPowerOfTwo<Self> for $t {
            #[inline]
            fn eq_mod_power_of_two(self, other: $t, pow: u64) -> bool {
                (self ^ other).divisible_by_power_of_two(pow)
            }
        }

        impl DivRem for $t {
            type DivOutput = $t;
            type RemOutput = $t;

            #[inline]
            fn div_rem(self, rhs: $t) -> ($t, $t) {
                (self / rhs, self % rhs)
            }
        }

        impl DivAssignRem for $t {
            type RemOutput = $t;

            #[inline]
            fn div_assign_rem(&mut self, rhs: $t) -> $t {
                let rem = *self % rhs;
                *self /= rhs;
                rem
            }
        }

        impl DivExact for $t {
            type Output = $t;

            #[inline]
            fn div_exact(self, rhs: $t) -> $t {
                self / rhs
            }
        }

        impl DivExactAssign for $t {
            #[inline]
            fn div_exact_assign(&mut self, rhs: $t) {
                *self /= rhs;
            }
        }

        impl DivisibleBy for $t {
            #[inline]
            fn divisible_by(self, rhs: $t) -> bool {
                self == 0 || rhs != 0 && self % rhs == 0
            }
        }

        impl DivRoundAssign for $t {
            fn div_round_assign(&mut self, rhs: $t, rm: RoundingMode) {
                *self = self.div_round(rhs, rm);
            }
        }

        impl ModAssign for $t {
            #[inline]
            fn mod_assign(&mut self, rhs: $t) {
                *self %= rhs;
            }
        }

        impl EqMod for $t {
            #[inline]
            fn eq_mod(self, other: $t, modulus: $t) -> bool {
                self == other || modulus != 0 && self.mod_op(modulus) == other.mod_op(modulus)
            }
        }
    };
}

//TODO fix code duplication
impl Parity for usize {
    #[inline]
    fn even(self) -> bool {
        (self & 1) == 0
    }

    #[inline]
    fn odd(self) -> bool {
        (self & 1) != 0
    }
}

impl ModPowerOfTwo for usize {
    type Output = usize;

    #[inline]
    fn mod_power_of_two(self, pow: u64) -> usize {
        if self == 0 || pow >= u64::from(0usize.trailing_zeros()) {
            self
        } else {
            self & ((1 << pow) - 1)
        }
    }
}

impl DivisibleByPowerOfTwo for usize {
    #[inline]
    fn divisible_by_power_of_two(self, pow: u64) -> bool {
        self.mod_power_of_two(pow) == 0
    }
}

impl EqModPowerOfTwo<usize> for usize {
    #[inline]
    fn eq_mod_power_of_two(self, other: usize, pow: u64) -> bool {
        (self ^ other).divisible_by_power_of_two(pow)
    }
}

impl Parity for isize {
    #[inline]
    fn even(self) -> bool {
        (self & 1) == 0
    }

    #[inline]
    fn odd(self) -> bool {
        (self & 1) != 0
    }
}

impl ShrRound<u32> for usize {
    type Output = usize;

    fn shr_round(self, other: u32, rm: RoundingMode) -> usize {
        if other == 0 || self == 0 {
            return self;
        }
        let width = u32::wrapping_from(0usize.trailing_zeros());
        match rm {
            RoundingMode::Down | RoundingMode::Floor if other >= width => 0,
            RoundingMode::Down | RoundingMode::Floor => self >> other,
            RoundingMode::Up | RoundingMode::Ceiling if other >= width => 1,
            RoundingMode::Up | RoundingMode::Ceiling => {
                let shifted = self >> other;
                if shifted << other == self {
                    shifted
                } else {
                    shifted + 1
                }
            }
            RoundingMode::Nearest
                if other == width && self > (1 << (0usize.trailing_zeros() - 1)) =>
            {
                1
            }
            RoundingMode::Nearest if other >= width => 0,
            RoundingMode::Nearest => {
                let mostly_shifted = self >> (other - 1);
                if mostly_shifted.even() {
                    // round down
                    mostly_shifted >> 1
                } else if mostly_shifted << (other - 1) != self {
                    // round up
                    (mostly_shifted >> 1) + 1
                } else {
                    // result is half-integer; round to even
                    let shifted = mostly_shifted >> 1;
                    if shifted.even() {
                        shifted
                    } else {
                        shifted + 1
                    }
                }
            }
            RoundingMode::Exact if other >= width => {
                panic!("Right shift is not exact: {} >> {}", self, other);
            }
            RoundingMode::Exact => {
                let shifted = self >> other;
                if shifted << other != self {
                    panic!("Right shift is not exact: {} >> {}", self, other);
                }
                shifted
            }
        }
    }
}

//TODO docs
macro_rules! unsigned_traits {
    ($t:ident, $log_width:expr) => {
        integer_traits!($t, $log_width);

        impl OrdAbs for $t {
            #[inline]
            fn cmp_abs(&self, other: &Self) -> Ordering {
                self.cmp(other)
            }
        }

        impl IsPowerOfTwo for $t {
            #[inline]
            fn is_power_of_two(self) -> bool {
                $t::is_power_of_two(self)
            }
        }

        impl NextPowerOfTwo for $t {
            type Output = $t;

            #[inline]
            fn next_power_of_two(self) -> $t {
                $t::next_power_of_two(self)
            }
        }

        impl NextPowerOfTwoAssign for $t {
            #[inline]
            fn next_power_of_two_assign(&mut self) {
                *self = $t::next_power_of_two(*self)
            }
        }

        impl CheckedNextPowerOfTwo for $t {
            type Output = $t;

            #[inline]
            fn checked_next_power_of_two(self) -> Option<$t> {
                $t::checked_next_power_of_two(self)
            }
        }

        impl SignificantBits for $t {
            /// Returns the number of significant bits of a primitive unsigned integer; this is the
            /// integer's width minus the number of leading zeros.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::SignificantBits;
            ///
            /// fn main() {
            ///     assert_eq!(0u8.significant_bits(), 0);
            ///     assert_eq!(100u64.significant_bits(), 7);
            /// }
            /// ```
            #[inline]
            fn significant_bits(self) -> u64 {
                (Self::WIDTH - self.leading_zeros()).into()
            }
        }

        impl FloorLogTwo for $t {
            /// Returns the floor of the base-2 logarithm of a positive primitive unsigned integer.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `self` is 0.
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::FloorLogTwo;
            ///
            /// fn main() {
            ///     assert_eq!(1u8.floor_log_two(), 0);
            ///     assert_eq!(100u64.floor_log_two(), 6);
            /// }
            /// ```
            #[inline]
            fn floor_log_two(self) -> u64 {
                if self == 0 {
                    panic!("Cannot take the base-2 logarithm of 0.");
                }
                self.significant_bits() - 1
            }
        }

        impl CeilingLogTwo for $t {
            /// Returns the ceiling of the base-2 logarithm of a positive primitive unsigned
            /// integer.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `self` is 0.
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::CeilingLogTwo;
            ///
            /// fn main() {
            ///     assert_eq!(1u8.ceiling_log_two(), 0);
            ///     assert_eq!(100u64.ceiling_log_two(), 7);
            /// }
            /// ```
            #[inline]
            fn ceiling_log_two(self) -> u64 {
                let floor_log_two = self.floor_log_two();
                if self.is_power_of_two() {
                    floor_log_two
                } else {
                    floor_log_two + 1
                }
            }
        }

        /// Provides functions for accessing and modifying the `index`th bit of a primitive unsigned
        /// integer, or the coefficient of 2^<pow>`index`</pow> in its binary expansion.
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::BitAccess;
        ///
        /// let mut x = 0;
        /// x.assign_bit(2, true);
        /// x.assign_bit(5, true);
        /// x.assign_bit(6, true);
        /// assert_eq!(x, 100);
        /// x.assign_bit(2, false);
        /// x.assign_bit(5, false);
        /// x.assign_bit(6, false);
        /// assert_eq!(x, 0);
        ///
        /// let mut x = 0u64;
        /// x.flip_bit(10);
        /// assert_eq!(x, 1024);
        /// x.flip_bit(10);
        /// assert_eq!(x, 0);
        /// ```
        impl BitAccess for $t {
            /// Determines whether the `index`th bit of a primitive unsigned integer, or the
            /// coefficient of 2<pow>`index`</pow> in its binary expansion, is 0 or 1. `false`
            /// means 0, `true` means 1.
            ///
            /// Getting bits beyond the type's width is allowed; those bits are false.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::BitAccess;
            ///
            /// assert_eq!(123u8.get_bit(2), false);
            /// assert_eq!(123u16.get_bit(3), true);
            /// assert_eq!(123u32.get_bit(100), false);
            /// assert_eq!(1_000_000_000_000u64.get_bit(12), true);
            /// assert_eq!(1_000_000_000_000u64.get_bit(100), false);
            /// ```
            #[inline]
            fn get_bit(&self, index: u64) -> bool {
                index < Self::WIDTH.into() && *self & (1 << index) != 0
            }

            /// Sets the `index`th bit of a primitive unsigned integer, or the coefficient of
            /// 2<pow>`index`</pow> in its binary expansion, to 1.
            ///
            /// Setting bits beyond the type's width is disallowed.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `index >= Self::WIDTH`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::BitAccess;
            ///
            /// let mut x = 0u8;
            /// x.set_bit(2);
            /// x.set_bit(5);
            /// x.set_bit(6);
            /// assert_eq!(x, 100);
            /// ```
            #[inline]
            fn set_bit(&mut self, index: u64) {
                if index < Self::WIDTH.into() {
                    *self |= 1 << index;
                } else {
                    panic!(
                        "Cannot set bit {} in non-negative value of width {}",
                        index,
                        Self::WIDTH
                    );
                }
            }

            /// Sets the `index`th bit of a primitive unsigned integer, or the coefficient of
            /// 2<pow>`index`</pow> in its binary expansion, to 0.
            ///
            /// Clearing bits beyond the type's width is allowed; since those bits are already
            /// false, clearing them does nothing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::BitAccess;
            ///
            /// let mut x = 0x7fu8;
            /// x.clear_bit(0);
            /// x.clear_bit(1);
            /// x.clear_bit(3);
            /// x.clear_bit(4);
            /// assert_eq!(x, 100);
            /// ```
            #[inline]
            fn clear_bit(&mut self, index: u64) {
                if index < Self::WIDTH.into() {
                    *self &= !(1 << index);
                }
            }
        }

        impl BitScan for $t {
            #[inline]
            fn index_of_next_false_bit(self, starting_index: u64) -> Option<u64> {
                Some(if starting_index >= Self::WIDTH.into() {
                    starting_index
                } else {
                    (!(self | ((1 << starting_index) - 1)))
                        .trailing_zeros()
                        .into()
                })
            }

            #[inline]
            fn index_of_next_true_bit(self, starting_index: u64) -> Option<u64> {
                if starting_index >= Self::WIDTH.into() {
                    None
                } else {
                    let index = (self & !((1 << starting_index) - 1))
                        .trailing_zeros()
                        .into();
                    if index == Self::WIDTH.into() {
                        None
                    } else {
                        Some(index)
                    }
                }
            }
        }

        impl DivisibleByPowerOfTwo for $t {
            #[inline]
            fn divisible_by_power_of_two(self, pow: u64) -> bool {
                self.mod_power_of_two(pow) == 0
            }
        }

        impl ModPowerOfTwo for $t {
            type Output = $t;

            #[inline]
            fn mod_power_of_two(self, pow: u64) -> $t {
                if self == 0 || pow >= $t::WIDTH.into() {
                    self
                } else {
                    self & ((1 << pow) - 1)
                }
            }
        }

        impl ModPowerOfTwoAssign for $t {
            #[inline]
            fn mod_power_of_two_assign(&mut self, pow: u64) {
                if *self != 0 && pow < $t::WIDTH.into() {
                    *self &= (1 << pow) - 1;
                }
            }
        }

        impl RemPowerOfTwo for $t {
            type Output = $t;

            #[inline]
            fn rem_power_of_two(self, pow: u64) -> $t {
                self.mod_power_of_two(pow)
            }
        }

        impl RemPowerOfTwoAssign for $t {
            #[inline]
            fn rem_power_of_two_assign(&mut self, pow: u64) {
                self.mod_power_of_two_assign(pow)
            }
        }

        impl DivMod for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            #[inline]
            fn div_mod(self, rhs: $t) -> ($t, $t) {
                (self / rhs, self % rhs)
            }
        }

        impl DivAssignMod for $t {
            type ModOutput = $t;

            #[inline]
            fn div_assign_mod(&mut self, rhs: $t) -> $t {
                let rem = *self % rhs;
                *self /= rhs;
                rem
            }
        }

        impl Mod for $t {
            type Output = $t;

            #[inline]
            fn mod_op(self, rhs: $t) -> $t {
                self % rhs
            }
        }

        impl NegMod for $t {
            type Output = $t;

            #[inline]
            fn neg_mod(self, rhs: $t) -> $t {
                let rem = self % rhs;
                if rem == 0 {
                    0
                } else {
                    rhs - rem
                }
            }
        }

        impl NegModAssign for $t {
            #[inline]
            fn neg_mod_assign(&mut self, rhs: $t) {
                *self = self.neg_mod(rhs);
            }
        }

        impl DivRound for $t {
            type Output = $t;

            fn div_round(self, rhs: $t, rm: RoundingMode) -> $t {
                let quotient = self / rhs;
                if rm == RoundingMode::Down || rm == RoundingMode::Floor {
                    quotient
                } else {
                    let remainder = self % rhs;
                    match rm {
                        _ if remainder == 0 => quotient,
                        RoundingMode::Up | RoundingMode::Ceiling => quotient + 1,
                        RoundingMode::Nearest => {
                            let shifted_rhs = rhs >> 1;
                            if remainder > shifted_rhs
                                || remainder == shifted_rhs && rhs.even() && quotient.odd()
                            {
                                quotient + 1
                            } else {
                                quotient
                            }
                        }
                        RoundingMode::Exact => {
                            panic!("Division is not exact: {} / {}", self, rhs);
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }

        impl CeilingDivNegMod for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            #[inline]
            fn ceiling_div_neg_mod(self, rhs: $t) -> ($t, $t) {
                let quotient = self / rhs;
                let remainder = self % rhs;
                if remainder == 0 {
                    (quotient, 0)
                } else {
                    (quotient + 1, rhs - remainder)
                }
            }
        }

        impl CeilingDivAssignNegMod for $t {
            type ModOutput = $t;

            #[inline]
            fn ceiling_div_assign_neg_mod(&mut self, rhs: $t) -> $t {
                let remainder = *self % rhs;
                *self /= rhs;
                if remainder == 0 {
                    0
                } else {
                    *self += 1;
                    rhs - remainder
                }
            }
        }
    };
}

//TODO docs
macro_rules! signed_traits {
    (
        $t:ident,
        $ut:ident,
        $log_width:expr
    ) => {
        integer_traits!($t, $log_width);

        //TODO docs
        impl PrimitiveSigned for $t {
            type UnsignedOfEqualWidth = $ut;

            #[inline]
            fn to_unsigned_bitwise(self) -> Self::UnsignedOfEqualWidth {
                self as $ut
            }

            #[inline]
            fn from_unsigned_bitwise(u: Self::UnsignedOfEqualWidth) -> Self {
                u as $t
            }
        }

        impl OrdAbs for $t {
            #[inline]
            fn cmp_abs(&self, other: &Self) -> Ordering {
                self.unsigned_abs().cmp(&other.unsigned_abs())
            }
        }

        impl Abs for $t {
            type Output = $t;

            #[inline]
            fn abs(self) -> $t {
                $t::abs(self)
            }
        }

        impl UnsignedAbs for $t {
            type Output = $ut;

            #[inline]
            fn unsigned_abs(self) -> $ut {
                $t::wrapping_abs(self) as $ut
            }
        }

        impl CheckedAbs for $t {
            type Output = $t;

            #[inline]
            fn abs(self) -> Option<$t> {
                $t::checked_abs(self)
            }
        }

        impl WrappingAbs for $t {
            type Output = $t;

            #[inline]
            fn abs(self) -> $t {
                $t::wrapping_abs(self)
            }
        }

        impl OverflowingAbs for $t {
            type Output = $t;

            #[inline]
            fn abs(self) -> ($t, bool) {
                $t::overflowing_abs(self)
            }
        }

        /// Returns the number of significant bits of a primitive signed integer; this is the
        /// integer's width minus the number of leading zeros of its absolute value.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Example
        /// ```
        /// use malachite_base::num::SignificantBits;
        ///
        /// fn main() {
        ///     assert_eq!(0i8.significant_bits(), 0);
        ///     assert_eq!((-100i64).significant_bits(), 7);
        /// }
        /// ```
        impl SignificantBits for $t {
            #[inline]
            fn significant_bits(self) -> u64 {
                self.unsigned_abs().significant_bits()
            }
        }

        /// Provides functions for accessing and modifying the `index`th bit of a primitive signed
        /// integer, or the coefficient of 2^<pow>`index`</pow> in its binary expansion.
        ///
        /// Negative integers are represented in two's complement.
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::BitAccess;
        ///
        /// let mut x = 0i8;
        /// x.assign_bit(2, true);
        /// x.assign_bit(5, true);
        /// x.assign_bit(6, true);
        /// assert_eq!(x, 100);
        /// x.assign_bit(2, false);
        /// x.assign_bit(5, false);
        /// x.assign_bit(6, false);
        /// assert_eq!(x, 0);
        ///
        /// let mut x = -0x100i16;
        /// x.assign_bit(2, true);
        /// x.assign_bit(5, true);
        /// x.assign_bit(6, true);
        /// assert_eq!(x, -156);
        /// x.assign_bit(2, false);
        /// x.assign_bit(5, false);
        /// x.assign_bit(6, false);
        /// assert_eq!(x, -256);
        ///
        /// let mut x = 0i32;
        /// x.flip_bit(10);
        /// assert_eq!(x, 1024);
        /// x.flip_bit(10);
        /// assert_eq!(x, 0);
        ///
        /// let mut x = -1i64;
        /// x.flip_bit(10);
        /// assert_eq!(x, -1025);
        /// x.flip_bit(10);
        /// assert_eq!(x, -1);
        /// ```
        impl BitAccess for $t {
            /// Determines whether the `index`th bit of a primitive signed integer, or the
            /// coefficient of 2<pow>`index`</pow> in its binary expansion, is 0 or 1. `false` means
            /// 0, `true` means 1.
            ///
            /// Negative integers are represented in two's complement.
            ///
            /// Accessing bits beyond the type's width is allowed; those bits are false if the
            /// integer is non-negative and true if it is negative.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::BitAccess;
            ///
            /// assert_eq!(123i8.get_bit(2), false);
            /// assert_eq!(123i16.get_bit(3), true);
            /// assert_eq!(123i32.get_bit(100), false);
            /// assert_eq!((-123i8).get_bit(0), true);
            /// assert_eq!((-123i16).get_bit(1), false);
            /// assert_eq!((-123i32).get_bit(100), true);
            /// assert_eq!(1_000_000_000_000i64.get_bit(12), true);
            /// assert_eq!(1_000_000_000_000i64.get_bit(100), false);
            /// assert_eq!((-1_000_000_000_000i64).get_bit(12), true);
            /// assert_eq!((-1_000_000_000_000i64).get_bit(100), true);
            /// ```
            #[inline]
            fn get_bit(&self, index: u64) -> bool {
                if index < Self::WIDTH.into() {
                    self & (1 << index) != 0
                } else {
                    *self < 0
                }
            }

            /// Sets the `index`th bit of a primitive signed integer, or the coefficient of
            /// 2<pow>`index`</pow> in its binary expansion, to 1.
            ///
            /// Negative integers are represented in two's complement.
            ///
            /// Setting bits beyond the type's width is disallowed if the integer is non-negative;
            /// if it is negative, it's allowed but does nothing since those bits are already true.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `index >= Self::WIDTH && self >= 0`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::BitAccess;
            ///
            /// let mut x = 0i8;
            /// x.set_bit(2);
            /// x.set_bit(5);
            /// x.set_bit(6);
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -0x100i16;
            /// x.set_bit(2);
            /// x.set_bit(5);
            /// x.set_bit(6);
            /// assert_eq!(x, -156);
            /// ```
            #[inline]
            fn set_bit(&mut self, index: u64) {
                if index < Self::WIDTH.into() {
                    *self |= 1 << index;
                } else if *self >= 0 {
                    panic!(
                        "Cannot set bit {} in non-negative value of width {}",
                        index,
                        Self::WIDTH
                    );
                }
            }

            /// Sets the `index`th bit of a primitive signed integer, or the coefficient of
            /// 2<pow>`index`</pow> in its binary expansion, to 0.
            ///
            /// Negative integers are represented in two's complement.
            ///
            /// Clearing bits beyond the type's width is disallowed if the integer is negative; if
            /// it is non-negative, it's allowed but does nothing since those bits are already
            /// false.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `index >= Self::WIDTH && self < 0`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::BitAccess;
            ///
            /// let mut x = 0x7fi8;
            /// x.clear_bit(0);
            /// x.clear_bit(1);
            /// x.clear_bit(3);
            /// x.clear_bit(4);
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -156i16;
            /// x.clear_bit(2);
            /// x.clear_bit(5);
            /// x.clear_bit(6);
            /// assert_eq!(x, -256);
            /// ```
            #[inline]
            fn clear_bit(&mut self, index: u64) {
                if index < Self::WIDTH.into() {
                    *self &= !(1 << index);
                } else if *self < 0 {
                    panic!(
                        "Cannot clear bit {} in negative value of width {}",
                        index,
                        Self::WIDTH
                    );
                }
            }
        }

        //TODO docs, test
        impl NegAssign for $t {
            #[inline]
            fn neg_assign(&mut self) {
                *self = -*self;
            }
        }

        //TODO
        impl BitScan for $t {
            #[inline]
            fn index_of_next_false_bit(self, starting_index: u64) -> Option<u64> {
                if starting_index >= u64::from(Self::WIDTH) - 1 {
                    if self >= 0 {
                        Some(starting_index)
                    } else {
                        None
                    }
                } else {
                    let index = (!(self | ((1 << starting_index) - 1)))
                        .trailing_zeros()
                        .into();
                    if index == $t::WIDTH.into() {
                        None
                    } else {
                        Some(index)
                    }
                }
            }

            #[inline]
            fn index_of_next_true_bit(self, starting_index: u64) -> Option<u64> {
                if starting_index >= u64::from(Self::WIDTH) - 1 {
                    if self >= 0 {
                        None
                    } else {
                        Some(starting_index)
                    }
                } else {
                    let index = (self & !((1 << starting_index) - 1))
                        .trailing_zeros()
                        .into();
                    if index == $t::WIDTH.into() {
                        None
                    } else {
                        Some(index)
                    }
                }
            }
        }

        impl DivisibleByPowerOfTwo for $t {
            #[inline]
            fn divisible_by_power_of_two(self, pow: u64) -> bool {
                self.to_unsigned_bitwise().divisible_by_power_of_two(pow)
            }
        }

        impl DivMod for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            #[inline]
            fn div_mod(self, other: $t) -> ($t, $t) {
                let (quotient, remainder) = if (self >= 0) == (other >= 0) {
                    let (quotient, remainder) = self.unsigned_abs().div_mod(other.unsigned_abs());
                    ($t::checked_from(quotient).unwrap(), remainder)
                } else {
                    let (quotient, remainder) = self
                        .unsigned_abs()
                        .ceiling_div_neg_mod(other.unsigned_abs());
                    (-$t::checked_from(quotient).unwrap(), remainder)
                };
                (
                    quotient,
                    if other >= 0 {
                        $t::checked_from(remainder).unwrap()
                    } else {
                        -$t::checked_from(remainder).unwrap()
                    },
                )
            }
        }

        impl DivAssignMod for $t {
            type ModOutput = $t;

            #[inline]
            fn div_assign_mod(&mut self, rhs: $t) -> $t {
                let (q, r) = self.div_mod(rhs);
                *self = q;
                r
            }
        }

        impl Mod for $t {
            type Output = $t;

            #[inline]
            fn mod_op(self, other: $t) -> $t {
                let remainder = if (self >= 0) == (other >= 0) {
                    self.unsigned_abs().mod_op(other.unsigned_abs())
                } else {
                    self.unsigned_abs().neg_mod(other.unsigned_abs())
                };
                if other >= 0 {
                    $t::checked_from(remainder).unwrap()
                } else {
                    -$t::checked_from(remainder).unwrap()
                }
            }
        }

        impl CeilingDivMod for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            #[inline]
            fn ceiling_div_mod(self, other: $t) -> ($t, $t) {
                let (quotient, remainder) = if (self >= 0) == (other >= 0) {
                    let (quotient, remainder) = self
                        .unsigned_abs()
                        .ceiling_div_neg_mod(other.unsigned_abs());
                    ($t::checked_from(quotient).unwrap(), remainder)
                } else {
                    let (quotient, remainder) = self.unsigned_abs().div_mod(other.unsigned_abs());
                    (-$t::checked_from(quotient).unwrap(), remainder)
                };
                (
                    quotient,
                    if other >= 0 {
                        -$t::checked_from(remainder).unwrap()
                    } else {
                        $t::checked_from(remainder).unwrap()
                    },
                )
            }
        }

        impl CeilingDivAssignMod for $t {
            type ModOutput = $t;

            #[inline]
            fn ceiling_div_assign_mod(&mut self, rhs: $t) -> $t {
                let (q, r) = self.ceiling_div_mod(rhs);
                *self = q;
                r
            }
        }

        impl CeilingMod for $t {
            type Output = $t;

            #[inline]
            fn ceiling_mod(self, other: $t) -> $t {
                let remainder = if (self >= 0) == (other >= 0) {
                    self.unsigned_abs().neg_mod(other.unsigned_abs())
                } else {
                    self.unsigned_abs().mod_op(other.unsigned_abs())
                };
                if other >= 0 {
                    -$t::checked_from(remainder).unwrap()
                } else {
                    $t::checked_from(remainder).unwrap()
                }
            }
        }

        impl CeilingModAssign for $t {
            #[inline]
            fn ceiling_mod_assign(&mut self, rhs: $t) {
                *self = self.ceiling_mod(rhs);
            }
        }

        impl Sign for $t {
            fn sign(&self) -> Ordering {
                self.cmp(&0)
            }
        }

        impl DivRound for $t {
            type Output = $t;

            fn div_round(self, other: $t, rm: RoundingMode) -> $t {
                let result_sign = (self >= 0) == (other >= 0);
                let abs = if result_sign {
                    self.unsigned_abs().div_round(other.unsigned_abs(), rm)
                } else {
                    self.unsigned_abs().div_round(other.unsigned_abs(), -rm)
                };
                if result_sign {
                    $t::checked_from(abs).unwrap()
                } else {
                    -$t::checked_from(abs).unwrap()
                }
            }
        }
    };
}

//TODO docs
macro_rules! float_traits {
    ($t:ident, $ut:ident) => {
        //TODO docs
        impl PrimitiveFloat for $t {
            type UnsignedOfEqualWidth = $ut;
            type SignedOfEqualWidth = <$ut as PrimitiveUnsigned>::SignedOfEqualWidth;
            const MANTISSA_WIDTH: u32 = std::$t::MANTISSA_DIGITS - 1;

            const INFINITY: Self = std::$t::INFINITY;
            const NEG_INFINITY: Self = std::$t::NEG_INFINITY;
            const NEG_ZERO: Self = -0.0;
            const NAN: Self = std::$t::NAN;
            const MAX_FINITE: Self = std::$t::MAX;
            const MIN_FINITE: Self = std::$t::MIN;

            #[inline]
            fn is_nan(self) -> bool {
                $t::is_nan(self)
            }

            #[inline]
            fn is_infinite(self) -> bool {
                $t::is_infinite(self)
            }

            #[inline]
            fn is_finite(self) -> bool {
                $t::is_finite(self)
            }

            #[inline]
            fn is_normal(self) -> bool {
                $t::is_normal(self)
            }

            #[inline]
            fn classify(self) -> FpCategory {
                $t::classify(self)
            }

            #[inline]
            fn is_sign_positive(self) -> bool {
                $t::is_sign_positive(self)
            }

            #[inline]
            fn is_sign_negative(self) -> bool {
                $t::is_sign_negative(self)
            }

            #[inline]
            fn to_bits(self) -> $ut {
                $t::to_bits(self)
            }

            #[inline]
            fn from_bits(v: $ut) -> $t {
                $t::from_bits(v)
            }
        }

        impl_named!($t);

        impl Min for $t {
            const MIN: $t = $t::NEG_INFINITY;
        }

        impl Max for $t {
            const MAX: $t = $t::INFINITY;
        }

        impl NegAssign for $t {
            #[inline]
            fn neg_assign(&mut self) {
                *self = -*self;
            }
        }
    };
}

//TODO docs
impl PrimitiveUnsigned for u8 {
    type SignedOfEqualWidth = i8;

    #[inline]
    fn to_signed_bitwise(self) -> i8 {
        self as i8
    }

    #[inline]
    fn to_signed_checked(self) -> Option<i8> {
        if self <= i8::MAX as u8 {
            Some(self as i8)
        } else {
            None
        }
    }

    #[inline]
    fn from_signed_bitwise(i: i8) -> u8 {
        i as u8
    }
}

impl PrimitiveUnsigned for u16 {
    type SignedOfEqualWidth = i16;

    #[inline]
    fn to_signed_bitwise(self) -> i16 {
        self as i16
    }

    #[inline]
    fn to_signed_checked(self) -> Option<i16> {
        if self <= i16::MAX as u16 {
            Some(self as i16)
        } else {
            None
        }
    }

    #[inline]
    fn from_signed_bitwise(i: i16) -> u16 {
        i as u16
    }
}

impl PrimitiveUnsigned for u32 {
    type SignedOfEqualWidth = i32;

    #[inline]
    fn to_signed_bitwise(self) -> i32 {
        self as i32
    }

    #[inline]
    fn to_signed_checked(self) -> Option<i32> {
        if self <= i32::MAX as u32 {
            Some(self as i32)
        } else {
            None
        }
    }

    #[inline]
    fn from_signed_bitwise(i: i32) -> u32 {
        i as u32
    }
}

impl PrimitiveUnsigned for u64 {
    type SignedOfEqualWidth = i64;

    #[inline]
    fn to_signed_bitwise(self) -> i64 {
        self as i64
    }

    #[inline]
    fn to_signed_checked(self) -> Option<i64> {
        if self <= i64::MAX as u64 {
            Some(self as i64)
        } else {
            None
        }
    }

    #[inline]
    fn from_signed_bitwise(i: i64) -> u64 {
        i as u64
    }
}

impl PrimitiveUnsigned for u128 {
    type SignedOfEqualWidth = i128;

    #[inline]
    fn to_signed_bitwise(self) -> i128 {
        self as i128
    }

    #[inline]
    fn to_signed_checked(self) -> Option<i128> {
        if self <= i128::MAX as u128 {
            Some(self as i128)
        } else {
            None
        }
    }

    #[inline]
    fn from_signed_bitwise(i: i128) -> u128 {
        i as u128
    }
}

unsigned_traits!(u8, 3);
unsigned_traits!(u16, 4);
unsigned_traits!(u32, 5);
unsigned_traits!(u64, 6);
unsigned_traits!(u128, 7);

signed_traits!(i8, u8, 3);
signed_traits!(i16, u16, 4);
signed_traits!(i32, u32, 5);
signed_traits!(i64, u64, 6);
signed_traits!(i128, u128, 7);

float_traits!(f32, u32);
float_traits!(f64, u64);

pub trait AbsAssign {
    fn abs_assign(&mut self);
}

pub trait Assign<Rhs = Self> {
    fn assign(&mut self, rhs: Rhs);
}

pub trait NegAssign {
    fn neg_assign(&mut self);
}

pub trait NotAssign {
    fn not_assign(&mut self);
}

pub trait AddMulAssign<B, C> {
    // Equivalent to self += b * c
    fn add_mul_assign(&mut self, b: B, c: C);
}

pub trait AddMul<B, C> {
    type Output;

    // Equivalent to self + b * c
    fn add_mul(self, b: B, c: C) -> Self::Output;
}

pub trait SubMulAssign<B, C> {
    // Equivalent to self -= b * c
    fn sub_mul_assign(&mut self, b: B, c: C);
}

pub trait SubMul<B, C> {
    type Output;

    // Equivalent to self - b * c
    fn sub_mul(self, b: B, c: C) -> Self::Output;
}

pub trait PartialOrdAbs<Rhs: ?Sized = Self>: PartialEq<Rhs> {
    fn partial_cmp_abs(&self, other: &Rhs) -> Option<Ordering>;

    #[inline]
    fn lt_abs(&self, other: &Rhs) -> bool {
        match self.partial_cmp_abs(other) {
            Some(Ordering::Less) => true,
            _ => false,
        }
    }

    #[inline]
    fn le_abs(&self, other: &Rhs) -> bool {
        match self.partial_cmp_abs(other) {
            Some(Ordering::Less) | Some(Ordering::Equal) => true,
            _ => false,
        }
    }

    #[inline]
    fn gt_abs(&self, other: &Rhs) -> bool {
        match self.partial_cmp_abs(other) {
            Some(Ordering::Greater) => true,
            _ => false,
        }
    }

    #[inline]
    fn ge_abs(&self, other: &Rhs) -> bool {
        match self.partial_cmp_abs(other) {
            Some(Ordering::Greater) | Some(Ordering::Equal) => true,
            _ => false,
        }
    }
}

pub trait OrdAbs: Eq + PartialOrdAbs<Self> {
    fn cmp_abs(&self, other: &Self) -> Ordering;
}

/// Provides the constant 0.
#[allow(clippy::declare_interior_mutable_const)]
pub trait Zero {
    const ZERO: Self;
}

/// Provides the constant 1.
#[allow(clippy::declare_interior_mutable_const)]
pub trait One {
    const ONE: Self;
}

/// Provides the constant 2.
#[allow(clippy::declare_interior_mutable_const)]
pub trait Two {
    const TWO: Self;
}

/// Provides the constant -1.
#[allow(clippy::declare_interior_mutable_const)]
pub trait NegativeOne {
    const NEGATIVE_ONE: Self;
}

/// Implements the constants 0, 1, and 2 for unsigned primitive integers.
macro_rules! impl01_unsigned {
    ($t:ty) => {
        /// The constant 0 for unsigned primitive integers.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl Zero for $t {
            const ZERO: $t = 0;
        }

        /// The constant 1 for unsigned primitive integers.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl One for $t {
            const ONE: $t = 1;
        }

        /// The constant 2 for unsigned primitive integers.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl Two for $t {
            const TWO: $t = 2;
        }
    };
}

/// Implements the constants 0, 1, 2, and -1 for signed primitive integers.
macro_rules! impl01_signed {
    ($t:ty) => {
        impl01_unsigned!($t);

        /// The constant -1 for signed primitive integers.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl NegativeOne for $t {
            const NEGATIVE_ONE: $t = -1;
        }
    };
}

/// Implements the constants 0, 1, 2, and -1 for primitive floating-point types.
macro_rules! impl01_float {
    ($t:ty) => {
        /// The constant 0.0 for primitive floating-point types.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl Zero for $t {
            const ZERO: $t = 0.0;
        }

        /// The constant 1.0 for primitive floating-point types.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl One for $t {
            const ONE: $t = 1.0;
        }

        /// The constant 2.0 for primitive floating-point types.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl Two for $t {
            const TWO: $t = 2.0;
        }

        /// The constant -1.0 for primitive floating-point types.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl NegativeOne for $t {
            const NEGATIVE_ONE: $t = -1.0;
        }
    };
}

impl01_unsigned!(u8);
impl01_unsigned!(u16);
impl01_unsigned!(u32);
impl01_unsigned!(u64);
impl01_unsigned!(u128);
impl01_unsigned!(usize);

impl01_signed!(i8);
impl01_signed!(i16);
impl01_signed!(i32);
impl01_signed!(i64);
impl01_signed!(i128);
impl01_signed!(isize);

impl01_float!(f32);
impl01_float!(f64);

/// Provides a function to get the number of significant bits of `self`.
pub trait SignificantBits {
    /// The number of bits it takes to represent `self`. This is useful when benchmarking functions;
    /// the functions' inputs can be bucketed based on their number of significant bits.
    fn significant_bits(self) -> u64;
}

/// Provides a function to get the floor of the base-2 logarithm of `self`.
pub trait FloorLogTwo {
    /// floor(log<sub>2</sub>(`self`))
    fn floor_log_two(self) -> u64;
}

/// Provides a function to get the ceiling of the base-2 logarithm of `self`.
pub trait CeilingLogTwo {
    /// ceiling(log<sub>2</sub>(`self`))
    fn ceiling_log_two(self) -> u64;
}

/// Associates with `Self` a type that's half `Self`'s size.
pub trait HasHalf {
    /// The type that's half the size of `Self`.
    type Half;
}

/// Provides a function to join two pieces into a value. For example, two `u32`s may be joined to
/// form a `u64`.
pub trait JoinHalves: HasHalf {
    /// Joins two values into a single value; the upper, or most significant half, comes first.
    fn join_halves(upper: Self::Half, lower: Self::Half) -> Self;
}

/// Provides functions to split a value into two pieces. For example, a `u64` may be split into two
/// `u32`s.
pub trait SplitInHalf: HasHalf {
    /// Extracts the lower, or least significant half, of `self`.
    ///
    fn lower_half(&self) -> Self::Half;

    /// Extracts the upper, or most significant half, of `self`.
    fn upper_half(&self) -> Self::Half;

    /// Extracts both halves of `self`; the upper, or most significant half, comes first.
    ///
    /// Time: worst case O(max(f(n), g(n))), where f(n) is the worst-case time complexity of
    ///     `Self::lower_half` and g(n) is the worst-case time complexity of `Self::upper_half`.
    ///
    /// Additional memory: worst case O(max(f(n), g(n))), where f(n) is the worst-case
    ///     additional-memory complexity of `Self::lower_half` and g(n) is the worst-case
    ///     additional-memory complexity of `Self::upper_half.
    ///
    #[inline]
    fn split_in_half(&self) -> (Self::Half, Self::Half) {
        (self.upper_half(), self.lower_half())
    }
}

/// Implements `JoinHalves` and `SplitInHalf` for unsigned primitive integers.
macro_rules! impl_halves_unsigned {
    ($t:ident, $ht:ident) => {
        /// Implements `HasHalf` for unsigned primitive integers.
        impl HasHalf for $t {
            /// The primitive integer type whose width is half of `Self`.
            type Half = $ht;
        }

        /// Implements `JoinHalves` for unsigned primitive integers.
        impl JoinHalves for $t {
            /// Joins two unsigned integers to form an unsigned integer with twice the width.
            /// `join_halves(upper, lower)`, where `upper` and `lower` are integers with w bits,
            /// yields an integer with 2w bits whose value is `upper` * 2<sup>w</sup> + `lower`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::JoinHalves;
            ///
            /// assert_eq!(u16::join_halves(1, 2), 258);
            /// assert_eq!(u32::join_halves(0xabcd, 0x1234), 0xabcd1234);
            /// ```
            #[inline]
            fn join_halves(upper: Self::Half, lower: Self::Half) -> Self {
                $t::from(upper) << $ht::WIDTH | $t::from(lower)
            }
        }

        /// Implements `SplitInHalf` for unsigned primitive integers.
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::SplitInHalf;
        ///
        /// assert_eq!(258u16.split_in_half(), (1, 2));
        /// assert_eq!(0xabcd1234u32.split_in_half(), (0xabcd, 0x1234));
        /// ```
        impl SplitInHalf for $t {
            /// Extracts the lower, or least significant half, of and unsigned integer.
            /// `n.lower_half()`, where `n` is an integer with w bits, yields an integer with w/2
            /// bits whose value is `n` mod 2<sup>w/2</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::SplitInHalf;
            ///
            /// assert_eq!(258u16.lower_half(), 2);
            /// assert_eq!(0xabcd1234u32.lower_half(), 0x1234);
            /// ```
            #[inline]
            fn lower_half(&self) -> Self::Half {
                *self as $ht
            }

            /// Extracts the upper, or most significant half, of and unsigned integer.
            /// `n.upper_half()`, where `n` is an integer with w bits, yields an integer with w/2
            /// bits whose value is floor(`n` / 2<sup>w/2</sup>).
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::SplitInHalf;
            ///
            /// assert_eq!(258u16.upper_half(), 1);
            /// assert_eq!(0xabcd1234u32.upper_half(), 0xabcd);
            /// ```
            #[inline]
            fn upper_half(&self) -> Self::Half {
                (self >> $ht::WIDTH) as $ht
            }
        }
    };
}

impl_halves_unsigned!(u16, u8);
impl_halves_unsigned!(u32, u16);
impl_halves_unsigned!(u64, u32);
impl_halves_unsigned!(u128, u64);

/// Computes the absolute value of `self` and converts to the unsigned equivalent.
pub trait UnsignedAbs {
    type Output;

    fn unsigned_abs(self) -> Self::Output;
}

//TODO doc and test
pub trait HammingDistance<RHS> {
    fn hamming_distance(self, rhs: RHS) -> u64;
}

//TODO doc and test
pub trait CheckedHammingDistance<RHS> {
    fn checked_hamming_distance(self, rhs: RHS) -> Option<u64>;
}

pub trait ShlRound<RHS> {
    type Output;

    fn shl_round(self, rhs: RHS, rm: RoundingMode) -> Self::Output;
}

pub trait ShrRound<RHS> {
    type Output;

    fn shr_round(self, rhs: RHS, rm: RoundingMode) -> Self::Output;
}

pub trait ShlRoundAssign<Rhs = Self> {
    fn shl_round_assign(&mut self, rhs: Rhs, rm: RoundingMode);
}

pub trait ShrRoundAssign<Rhs = Self> {
    fn shr_round_assign(&mut self, rhs: Rhs, rm: RoundingMode);
}

macro_rules! round_shift_unsigned_unsigned {
    ($t:ident, $u:ident) => {
        impl ShrRound<$u> for $t {
            type Output = $t;

            fn shr_round(self, other: $u, rm: RoundingMode) -> $t {
                if other == 0 || self == 0 {
                    return self;
                }
                let width = $u::wrapping_from($t::WIDTH);
                match rm {
                    RoundingMode::Down | RoundingMode::Floor if other >= width => 0,
                    RoundingMode::Down | RoundingMode::Floor => self >> other,
                    RoundingMode::Up | RoundingMode::Ceiling if other >= width => 1,
                    RoundingMode::Up | RoundingMode::Ceiling => {
                        let shifted = self >> other;
                        if shifted << other == self {
                            shifted
                        } else {
                            shifted + 1
                        }
                    }
                    RoundingMode::Nearest if other == width && self > (1 << ($t::WIDTH - 1)) => 1,
                    RoundingMode::Nearest if other >= width => 0,
                    RoundingMode::Nearest => {
                        let mostly_shifted = self >> (other - 1);
                        if mostly_shifted.even() {
                            // round down
                            mostly_shifted >> 1
                        } else if mostly_shifted << (other - 1) != self {
                            // round up
                            (mostly_shifted >> 1) + 1
                        } else {
                            // result is half-integer; round to even
                            let shifted = mostly_shifted >> 1;
                            if shifted.even() {
                                shifted
                            } else {
                                shifted + 1
                            }
                        }
                    }
                    RoundingMode::Exact if other >= width => {
                        panic!("Right shift is not exact: {} >> {}", self, other);
                    }
                    RoundingMode::Exact => {
                        let shifted = self >> other;
                        if shifted << other != self {
                            panic!("Right shift is not exact: {} >> {}", self, other);
                        }
                        shifted
                    }
                }
            }
        }

        impl ShrRoundAssign<$u> for $t {
            fn shr_round_assign(&mut self, other: $u, rm: RoundingMode) {
                if other == 0 || *self == 0 {
                    return;
                }
                let width = $u::wrapping_from($t::WIDTH);
                match rm {
                    RoundingMode::Down | RoundingMode::Floor if other >= width => *self = 0,
                    RoundingMode::Down | RoundingMode::Floor => *self >>= other,
                    RoundingMode::Up | RoundingMode::Ceiling if other >= width => *self = 1,
                    RoundingMode::Up | RoundingMode::Ceiling => {
                        let original = *self;
                        *self >>= other;
                        if *self << other != original {
                            *self += 1;
                        }
                    }
                    RoundingMode::Nearest if other == width && *self > (1 << ($t::WIDTH - 1)) => {
                        *self = 1;
                    }
                    RoundingMode::Nearest if other >= width => *self = 0,
                    RoundingMode::Nearest => {
                        let original = *self;
                        *self >>= other - 1;
                        if self.even() {
                            // round down
                            *self >>= 1;
                        } else if *self << (other - 1) != original {
                            // round up
                            *self >>= 1;
                            *self += 1;
                        } else {
                            // result is half-integer; round to even
                            *self >>= 1;
                            if self.odd() {
                                *self += 1;
                            }
                        }
                    }
                    RoundingMode::Exact if other >= width => {
                        panic!("Right shift is not exact: {} >>= {}", *self, other);
                    }
                    RoundingMode::Exact => {
                        let original = *self;
                        *self >>= other;
                        if *self << other != original {
                            panic!("Right shift is not exact: {} >>= {}", original, other);
                        }
                    }
                }
            }
        }
    };
}
round_shift_unsigned_unsigned!(u8, u8);
round_shift_unsigned_unsigned!(u8, u16);
round_shift_unsigned_unsigned!(u8, u32);
round_shift_unsigned_unsigned!(u8, u64);
round_shift_unsigned_unsigned!(u8, u128);
round_shift_unsigned_unsigned!(u16, u8);
round_shift_unsigned_unsigned!(u16, u16);
round_shift_unsigned_unsigned!(u16, u32);
round_shift_unsigned_unsigned!(u16, u64);
round_shift_unsigned_unsigned!(u16, u128);
round_shift_unsigned_unsigned!(u32, u8);
round_shift_unsigned_unsigned!(u32, u16);
round_shift_unsigned_unsigned!(u32, u32);
round_shift_unsigned_unsigned!(u32, u64);
round_shift_unsigned_unsigned!(u32, u128);
round_shift_unsigned_unsigned!(u64, u8);
round_shift_unsigned_unsigned!(u64, u16);
round_shift_unsigned_unsigned!(u64, u32);
round_shift_unsigned_unsigned!(u64, u64);
round_shift_unsigned_unsigned!(u64, u128);
round_shift_unsigned_unsigned!(u128, u8);
round_shift_unsigned_unsigned!(u128, u16);
round_shift_unsigned_unsigned!(u128, u32);
round_shift_unsigned_unsigned!(u128, u64);
round_shift_unsigned_unsigned!(u128, u128);

macro_rules! round_shift_primitive_signed {
    ($t:ident, $u:ident) => {
        impl ShlRound<$u> for $t {
            type Output = $t;

            #[inline]
            fn shl_round(self, other: $u, rm: RoundingMode) -> $t {
                if other >= 0 {
                    self << other.to_unsigned_bitwise()
                } else {
                    self.shr_round(other.unsigned_abs(), rm)
                }
            }
        }

        impl ShlRoundAssign<$u> for $t {
            #[inline]
            fn shl_round_assign(&mut self, other: $u, rm: RoundingMode) {
                if other >= 0 {
                    *self <<= other.to_unsigned_bitwise();
                } else {
                    self.shr_round_assign(other.unsigned_abs(), rm);
                }
            }
        }

        impl ShrRound<$u> for $t {
            type Output = $t;

            #[inline]
            fn shr_round(self, other: $u, rm: RoundingMode) -> $t {
                if other >= 0 {
                    self.shr_round(other.to_unsigned_bitwise(), rm)
                } else {
                    self << other.unsigned_abs()
                }
            }
        }

        impl ShrRoundAssign<$u> for $t {
            #[inline]
            fn shr_round_assign(&mut self, other: $u, rm: RoundingMode) {
                if other >= 0 {
                    self.shr_round_assign(other.to_unsigned_bitwise(), rm);
                } else {
                    *self <<= other.unsigned_abs()
                }
            }
        }
    };
}
round_shift_primitive_signed!(u8, i8);
round_shift_primitive_signed!(u8, i16);
round_shift_primitive_signed!(u8, i32);
round_shift_primitive_signed!(u8, i64);
round_shift_primitive_signed!(u8, i128);
round_shift_primitive_signed!(u16, i8);
round_shift_primitive_signed!(u16, i16);
round_shift_primitive_signed!(u16, i32);
round_shift_primitive_signed!(u16, i64);
round_shift_primitive_signed!(u16, i128);
round_shift_primitive_signed!(u32, i8);
round_shift_primitive_signed!(u32, i16);
round_shift_primitive_signed!(u32, i32);
round_shift_primitive_signed!(u32, i64);
round_shift_primitive_signed!(u32, i128);
round_shift_primitive_signed!(u64, i8);
round_shift_primitive_signed!(u64, i16);
round_shift_primitive_signed!(u64, i32);
round_shift_primitive_signed!(u64, i64);
round_shift_primitive_signed!(u64, i128);
round_shift_primitive_signed!(u128, i8);
round_shift_primitive_signed!(u128, i16);
round_shift_primitive_signed!(u128, i32);
round_shift_primitive_signed!(u128, i64);
round_shift_primitive_signed!(u128, i128);
round_shift_primitive_signed!(i8, i8);
round_shift_primitive_signed!(i8, i16);
round_shift_primitive_signed!(i8, i32);
round_shift_primitive_signed!(i8, i64);
round_shift_primitive_signed!(i8, i128);
round_shift_primitive_signed!(i16, i8);
round_shift_primitive_signed!(i16, i16);
round_shift_primitive_signed!(i16, i32);
round_shift_primitive_signed!(i16, i64);
round_shift_primitive_signed!(i16, i128);
round_shift_primitive_signed!(i32, i8);
round_shift_primitive_signed!(i32, i16);
round_shift_primitive_signed!(i32, i32);
round_shift_primitive_signed!(i32, i64);
round_shift_primitive_signed!(i32, i128);
round_shift_primitive_signed!(i64, i8);
round_shift_primitive_signed!(i64, i16);
round_shift_primitive_signed!(i64, i32);
round_shift_primitive_signed!(i64, i64);
round_shift_primitive_signed!(i64, i128);
round_shift_primitive_signed!(i128, i8);
round_shift_primitive_signed!(i128, i16);
round_shift_primitive_signed!(i128, i32);
round_shift_primitive_signed!(i128, i64);
round_shift_primitive_signed!(i128, i128);

macro_rules! round_shift_signed_unsigned {
    ($t:ident, $u:ident) => {
        impl ShrRound<$u> for $t {
            type Output = $t;

            fn shr_round(self, other: $u, rm: RoundingMode) -> $t {
                let abs = self.unsigned_abs();
                if self >= 0 {
                    abs.shr_round(other, rm).to_signed_bitwise()
                } else {
                    let abs_shifted = abs.shr_round(other, -rm);
                    if abs_shifted == 0 {
                        0
                    } else if abs_shifted == $t::MIN.unsigned_abs() {
                        $t::MIN
                    } else {
                        -abs_shifted.to_signed_bitwise()
                    }
                }
            }
        }

        impl ShrRoundAssign<$u> for $t {
            #[inline]
            fn shr_round_assign(&mut self, other: $u, rm: RoundingMode) {
                *self = self.shr_round(other, rm);
            }
        }
    };
}
round_shift_signed_unsigned!(i8, u8);
round_shift_signed_unsigned!(i8, u16);
round_shift_signed_unsigned!(i8, u32);
round_shift_signed_unsigned!(i8, u64);
round_shift_signed_unsigned!(i8, u128);
round_shift_signed_unsigned!(i16, u8);
round_shift_signed_unsigned!(i16, u16);
round_shift_signed_unsigned!(i16, u32);
round_shift_signed_unsigned!(i16, u64);
round_shift_signed_unsigned!(i16, u128);
round_shift_signed_unsigned!(i32, u8);
round_shift_signed_unsigned!(i32, u16);
round_shift_signed_unsigned!(i32, u32);
round_shift_signed_unsigned!(i32, u64);
round_shift_signed_unsigned!(i32, u128);
round_shift_signed_unsigned!(i64, u8);
round_shift_signed_unsigned!(i64, u16);
round_shift_signed_unsigned!(i64, u32);
round_shift_signed_unsigned!(i64, u64);
round_shift_signed_unsigned!(i64, u128);
round_shift_signed_unsigned!(i128, u8);
round_shift_signed_unsigned!(i128, u16);
round_shift_signed_unsigned!(i128, u32);
round_shift_signed_unsigned!(i128, u64);
round_shift_signed_unsigned!(i128, u128);

//TODO doc and test
pub trait FromU32Slice: Sized {
    fn from_u32_slice(slice: &[u32]) -> Self;

    fn copy_from_u32_slice(out_slice: &mut [Self], in_slice: &[u32]);
}

//TODO doc and test
impl FromU32Slice for u32 {
    #[inline]
    fn from_u32_slice(slice: &[u32]) -> Self {
        assert!(!slice.is_empty());
        slice[0]
    }

    #[inline]
    fn copy_from_u32_slice(out_slice: &mut [u32], in_slice: &[u32]) {
        let out_len = out_slice.len();
        assert!(out_len >= in_slice.len());
        out_slice.copy_from_slice(&in_slice[..out_len]);
    }
}

//TODO doc and test
impl FromU32Slice for u8 {
    #[inline]
    fn from_u32_slice(slice: &[u32]) -> Self {
        assert!(!slice.is_empty());
        slice[0] as u8
    }

    fn copy_from_u32_slice(out_slice: &mut [u8], in_slice: &[u32]) {
        let out_len = out_slice.len();
        assert!(out_len >= in_slice.len() << 2);
        let mut i = 0;
        for u in in_slice {
            let (upper, lower) = u.split_in_half();
            let (upper_upper, lower_upper) = upper.split_in_half();
            let (upper_lower, lower_lower) = lower.split_in_half();
            out_slice[i] = lower_lower;
            out_slice[i + 1] = upper_lower;
            out_slice[i + 2] = lower_upper;
            out_slice[i + 3] = upper_upper;
            i += 4;
        }
    }
}

//TODO doc and test
impl FromU32Slice for u16 {
    #[inline]
    fn from_u32_slice(slice: &[u32]) -> Self {
        assert!(!slice.is_empty());
        slice[0] as u16
    }

    fn copy_from_u32_slice(out_slice: &mut [u16], in_slice: &[u32]) {
        let out_len = out_slice.len();
        assert!(out_len >= in_slice.len() << 1);
        let mut i = 0;
        for u in in_slice {
            let (upper, lower) = u.split_in_half();
            out_slice[i] = lower;
            out_slice[i + 1] = upper;
            i += 2;
        }
    }
}

//TODO doc and test
impl FromU32Slice for u64 {
    #[inline]
    fn from_u32_slice(slice: &[u32]) -> Self {
        assert!(slice.len() >= 2);
        u64::join_halves(slice[1], slice[0])
    }

    fn copy_from_u32_slice(out_slice: &mut [u64], in_slice: &[u32]) {
        let out_len = out_slice.len();
        assert!(out_len >= in_slice.len() >> 1);
        let mut i = 0;
        for out in out_slice.iter_mut() {
            *out = u64::join_halves(in_slice[i + 1], in_slice[i]);
            i += 2;
        }
    }
}

//TODO doc and test
impl FromU32Slice for u128 {
    #[inline]
    fn from_u32_slice(slice: &[u32]) -> Self {
        assert!(slice.len() >= 4);
        u128::join_halves(
            u64::join_halves(slice[3], slice[2]),
            u64::join_halves(slice[1], slice[0]),
        )
    }

    fn copy_from_u32_slice(out_slice: &mut [u128], in_slice: &[u32]) {
        let out_len = out_slice.len();
        assert!(out_len >= in_slice.len() >> 2);
        let mut i = 0;
        for out in out_slice.iter_mut() {
            *out = u128::join_halves(
                u64::join_halves(in_slice[i + 3], in_slice[i + 2]),
                u64::join_halves(in_slice[i + 1], in_slice[i]),
            );
            i += 4;
        }
    }
}
