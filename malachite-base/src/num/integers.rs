use std::fmt::{Binary, Debug, Display, LowerHex, Octal, UpperHex};
use std::hash::Hash;
use std::iter::{Product, Sum};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};
use std::str::FromStr;

use comparison::{Max, Min};
use conversion::{
    CheckedFrom, CheckedInto, OverflowingFrom, OverflowingInto, SaturatingFrom, SaturatingInto,
    WrappingFrom, WrappingInto,
};
use crement::Crementable;
use named::Named;
use num::traits::{
    BitAccess, BitScan, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedSub,
    CountOnes, CountZeros, DivAssignMod, DivAssignRem, DivExact, DivExactAssign, DivMod, DivRem,
    DivRound, DivRoundAssign, DivisibleBy, DivisibleByPowerOfTwo, Endian, EqMod, EqModPowerOfTwo,
    HammingDistance, LeadingZeros, Mod, ModAssign, NotAssign, One, OrdAbs, OverflowingAdd,
    OverflowingAddAssign, OverflowingDiv, OverflowingDivAssign, OverflowingMul,
    OverflowingMulAssign, OverflowingNeg, OverflowingNegAssign, OverflowingRem,
    OverflowingRemAssign, OverflowingShl, OverflowingShr, OverflowingSub, Parity, PartialOrdAbs,
    Pow, RotateLeft, RotateRight, SaturatingAdd, SaturatingAddAssign, SaturatingMul,
    SaturatingMulAssign, SaturatingSub, SaturatingSubAssign, ShlRound, ShlRoundAssign, ShrRound,
    ShrRoundAssign, SignificantBits, TrailingZeros, TrueCheckedShl, TrueCheckedShr, UnsignedAbs,
    WrappingAdd, WrappingAddAssign, WrappingDiv, WrappingDivAssign, WrappingMul, WrappingMulAssign,
    WrappingNeg, WrappingNegAssign, WrappingRem, WrappingRemAssign, WrappingShl, WrappingShr,
    WrappingSub, WrappingSubAssign, Zero,
};
use round::RoundingMode;

/// This trait defines functions on primitive integral types: uxx, ixx, usize, and isize.
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
    + CheckedFrom<usize>
    + CheckedFrom<i8>
    + CheckedFrom<i16>
    + CheckedFrom<i32>
    + CheckedFrom<i64>
    + CheckedFrom<i128>
    + CheckedFrom<isize>
    + CheckedInto<u8>
    + CheckedInto<u16>
    + CheckedInto<u32>
    + CheckedInto<u64>
    + CheckedInto<u128>
    + CheckedInto<usize>
    + CheckedInto<i8>
    + CheckedInto<i16>
    + CheckedInto<i32>
    + CheckedInto<i64>
    + CheckedInto<i128>
    + CheckedInto<isize>
    + CheckedMul<Output = Self>
    + CheckedNeg<Output = Self>
    + CheckedRem<Output = Self>
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
    + OverflowingFrom<u8>
    + OverflowingFrom<u16>
    + OverflowingFrom<u32>
    + OverflowingFrom<u64>
    + OverflowingFrom<u128>
    + OverflowingFrom<usize>
    + OverflowingFrom<i8>
    + OverflowingFrom<i16>
    + OverflowingFrom<i32>
    + OverflowingFrom<i64>
    + OverflowingFrom<i128>
    + OverflowingFrom<isize>
    + OverflowingInto<u8>
    + OverflowingInto<u16>
    + OverflowingInto<u32>
    + OverflowingInto<u64>
    + OverflowingInto<u128>
    + OverflowingInto<usize>
    + OverflowingInto<i8>
    + OverflowingInto<i16>
    + OverflowingInto<i32>
    + OverflowingInto<i64>
    + OverflowingInto<i128>
    + OverflowingInto<isize>
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
    + SaturatingFrom<u8>
    + SaturatingFrom<u16>
    + SaturatingFrom<u32>
    + SaturatingFrom<u64>
    + SaturatingFrom<u128>
    + SaturatingFrom<usize>
    + SaturatingFrom<i8>
    + SaturatingFrom<i16>
    + SaturatingFrom<i32>
    + SaturatingFrom<i64>
    + SaturatingFrom<i128>
    + SaturatingFrom<isize>
    + SaturatingInto<u8>
    + SaturatingInto<u16>
    + SaturatingInto<u32>
    + SaturatingInto<u64>
    + SaturatingInto<u128>
    + SaturatingInto<usize>
    + SaturatingInto<i8>
    + SaturatingInto<i16>
    + SaturatingInto<i32>
    + SaturatingInto<i64>
    + SaturatingInto<i128>
    + SaturatingInto<isize>
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
    + ShlAssign<u8>
    + ShlAssign<u16>
    + ShlAssign<u32>
    + ShlAssign<u64>
    + ShlAssign<u128>
    + ShlAssign<usize>
    + ShlAssign<i8>
    + ShlAssign<i16>
    + ShlAssign<i32>
    + ShlAssign<i64>
    + ShlAssign<i128>
    + ShlAssign<isize>
    + ShlRound<i8, Output = Self>
    + ShlRound<i16, Output = Self>
    + ShlRound<i32, Output = Self>
    + ShlRound<i64, Output = Self>
    + ShlRound<i128, Output = Self>
    + ShlRound<isize, Output = Self>
    + ShlRoundAssign<i8>
    + ShlRoundAssign<i16>
    + ShlRoundAssign<i32>
    + ShlRoundAssign<i64>
    + ShlRoundAssign<i128>
    + ShlRoundAssign<isize>
    + Shr<u8, Output = Self>
    + Shr<u16, Output = Self>
    + Shr<u32, Output = Self>
    + Shr<u64, Output = Self>
    + Shr<u128, Output = Self>
    + Shr<usize, Output = Self>
    + Shr<i8, Output = Self>
    + Shr<i16, Output = Self>
    + Shr<i32, Output = Self>
    + Shr<i64, Output = Self>
    + Shr<i128, Output = Self>
    + Shr<isize, Output = Self>
    + ShrAssign<u8>
    + ShrAssign<u16>
    + ShrAssign<u32>
    + ShrAssign<u64>
    + ShrAssign<u128>
    + ShrAssign<usize>
    + ShrAssign<i8>
    + ShrAssign<i16>
    + ShrAssign<i32>
    + ShrAssign<i64>
    + ShrAssign<i128>
    + ShrAssign<isize>
    + ShrRound<u8, Output = Self>
    + ShrRound<u16, Output = Self>
    + ShrRound<u32, Output = Self>
    + ShrRound<u64, Output = Self>
    + ShrRound<u128, Output = Self>
    + ShrRound<usize, Output = Self>
    + ShrRound<i8, Output = Self>
    + ShrRound<i16, Output = Self>
    + ShrRound<i32, Output = Self>
    + ShrRound<i64, Output = Self>
    + ShrRound<i128, Output = Self>
    + ShrRound<isize, Output = Self>
    + ShrRoundAssign<u8>
    + ShrRoundAssign<u16>
    + ShrRoundAssign<u32>
    + ShrRoundAssign<u64>
    + ShrRoundAssign<u128>
    + ShrRoundAssign<usize>
    + ShrRoundAssign<i8>
    + ShrRoundAssign<i16>
    + ShrRoundAssign<i32>
    + ShrRoundAssign<i64>
    + ShrRoundAssign<i128>
    + ShrRoundAssign<isize>
    + SignificantBits
    + Sized
    + Sub<Output = Self>
    + SubAssign<Self>
    + Sum<Self>
    + TrailingZeros
    + TrueCheckedShl<Output = Self>
    + TrueCheckedShr<Output = Self>
    + UpperHex
    + Crementable
    + WrappingAdd<Output = Self>
    + WrappingAddAssign
    + WrappingDiv<Output = Self>
    + WrappingDivAssign
    + WrappingFrom<u8>
    + WrappingFrom<u16>
    + WrappingFrom<u32>
    + WrappingFrom<u64>
    + WrappingFrom<u128>
    + WrappingFrom<usize>
    + WrappingFrom<i8>
    + WrappingFrom<i16>
    + WrappingFrom<i32>
    + WrappingFrom<i64>
    + WrappingFrom<i128>
    + WrappingFrom<isize>
    + WrappingInto<u8>
    + WrappingInto<u16>
    + WrappingInto<u32>
    + WrappingInto<u64>
    + WrappingInto<u128>
    + WrappingInto<usize>
    + WrappingInto<i8>
    + WrappingInto<i16>
    + WrappingInto<i32>
    + WrappingInto<i64>
    + WrappingInto<i128>
    + WrappingInto<isize>
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
    /// The number of bits of `Self`.
    const WIDTH: u32;

    /// The base-2 logarithm of the number of bits of `Self`. Instead of `n / WIDTH`, use
    /// `n >> LOG_WIDTH`.
    ///
    /// Note that this value is correct for all of the built-in primitive integer types, but it will
    /// not be correct for custom types with a non-power-of-two `WIDTH`. For such implementations
    /// `LOG_WIDTH` should not be used.
    const LOG_WIDTH: u32 = Self::WIDTH.trailing_zeros();

    /// A mask that consists of `LOG_WIDTH` bits. Instead of `n % WIDTH`, use `n & WIDTH_MASK`.
    ///
    /// Note that this value is correct for all of the built-in primitive integer types, but it will
    /// not be correct for custom types with a non-power-of-two `WIDTH`. For such implementations
    /// `WIDTH_MASK` should not be used.
    const WIDTH_MASK: u32 = Self::WIDTH - 1;

    /// Gets the most-significant bit of `Self`. For signed integers, this is the sign bit.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::integers::PrimitiveInteger;
    ///
    /// assert_eq!(123u32.get_highest_bit(), false);
    /// assert_eq!(4_000_000_000u32.get_highest_bit(), true);
    /// assert_eq!(2_000_000_000i32.get_highest_bit(), false);
    /// assert_eq!((-2_000_000_000i32).get_highest_bit(), true);
    /// ```
    #[inline]
    fn get_highest_bit(&self) -> bool {
        self.get_bit(u64::from(Self::WIDTH - 1))
    }
}

/// This macro defines trait implementations that are the same for unsigned and signed types.
macro_rules! integer_traits {
    ($t:ident, $width:expr) => {
        /// # Examples
        /// ```
        /// use malachite_base::num::integers::PrimitiveInteger;
        ///
        /// assert_eq!(u32::WIDTH, 32);
        /// assert_eq!(u32::LOG_WIDTH, 5);
        /// assert_eq!(u32::WIDTH_MASK, 0x1f);
        /// ```
        impl PrimitiveInteger for $t {
            const WIDTH: u32 = $width;
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

        impl_named!($t);

        impl PartialOrdAbs<$t> for $t {
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                Some(self.cmp_abs(other))
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

        //TODO continue with checked_pow

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

        impl Crementable for $t {
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
            /// use malachite_base::crement::Crementable;
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
            /// use malachite_base::crement::Crementable;
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

macro_rules! round_shift_primitive_signed {
    ($t:ident, $u:ident) => {
        impl ShlRound<$u> for $t {
            type Output = $t;

            #[inline]
            fn shl_round(self, other: $u, rm: RoundingMode) -> $t {
                if other >= 0 {
                    self << $u::wrapping_from(other)
                } else {
                    self.shr_round(other.unsigned_abs(), rm)
                }
            }
        }

        impl ShlRoundAssign<$u> for $t {
            #[inline]
            fn shl_round_assign(&mut self, other: $u, rm: RoundingMode) {
                if other >= 0 {
                    *self <<= $u::wrapping_from(other);
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
                    self.shr_round($u::wrapping_from(other), rm)
                } else {
                    self << other.unsigned_abs()
                }
            }
        }

        impl ShrRoundAssign<$u> for $t {
            #[inline]
            fn shr_round_assign(&mut self, other: $u, rm: RoundingMode) {
                if other >= 0 {
                    self.shr_round_assign($u::wrapping_from(other), rm);
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
round_shift_primitive_signed!(u8, isize);
round_shift_primitive_signed!(u16, i8);
round_shift_primitive_signed!(u16, i16);
round_shift_primitive_signed!(u16, i32);
round_shift_primitive_signed!(u16, i64);
round_shift_primitive_signed!(u16, i128);
round_shift_primitive_signed!(u16, isize);
round_shift_primitive_signed!(u32, i8);
round_shift_primitive_signed!(u32, i16);
round_shift_primitive_signed!(u32, i32);
round_shift_primitive_signed!(u32, i64);
round_shift_primitive_signed!(u32, i128);
round_shift_primitive_signed!(u32, isize);
round_shift_primitive_signed!(u64, i8);
round_shift_primitive_signed!(u64, i16);
round_shift_primitive_signed!(u64, i32);
round_shift_primitive_signed!(u64, i64);
round_shift_primitive_signed!(u64, i128);
round_shift_primitive_signed!(u64, isize);
round_shift_primitive_signed!(u128, i8);
round_shift_primitive_signed!(u128, i16);
round_shift_primitive_signed!(u128, i32);
round_shift_primitive_signed!(u128, i64);
round_shift_primitive_signed!(u128, i128);
round_shift_primitive_signed!(u128, isize);
round_shift_primitive_signed!(usize, i8);
round_shift_primitive_signed!(usize, i16);
round_shift_primitive_signed!(usize, i32);
round_shift_primitive_signed!(usize, i64);
round_shift_primitive_signed!(usize, i128);
round_shift_primitive_signed!(usize, isize);
round_shift_primitive_signed!(i8, i8);
round_shift_primitive_signed!(i8, i16);
round_shift_primitive_signed!(i8, i32);
round_shift_primitive_signed!(i8, i64);
round_shift_primitive_signed!(i8, i128);
round_shift_primitive_signed!(i8, isize);
round_shift_primitive_signed!(i16, i8);
round_shift_primitive_signed!(i16, i16);
round_shift_primitive_signed!(i16, i32);
round_shift_primitive_signed!(i16, i64);
round_shift_primitive_signed!(i16, i128);
round_shift_primitive_signed!(i16, isize);
round_shift_primitive_signed!(i32, i8);
round_shift_primitive_signed!(i32, i16);
round_shift_primitive_signed!(i32, i32);
round_shift_primitive_signed!(i32, i64);
round_shift_primitive_signed!(i32, i128);
round_shift_primitive_signed!(i32, isize);
round_shift_primitive_signed!(i64, i8);
round_shift_primitive_signed!(i64, i16);
round_shift_primitive_signed!(i64, i32);
round_shift_primitive_signed!(i64, i64);
round_shift_primitive_signed!(i64, i128);
round_shift_primitive_signed!(i64, isize);
round_shift_primitive_signed!(i128, i8);
round_shift_primitive_signed!(i128, i16);
round_shift_primitive_signed!(i128, i32);
round_shift_primitive_signed!(i128, i64);
round_shift_primitive_signed!(i128, i128);
round_shift_primitive_signed!(i128, isize);
round_shift_primitive_signed!(isize, i8);
round_shift_primitive_signed!(isize, i16);
round_shift_primitive_signed!(isize, i32);
round_shift_primitive_signed!(isize, i64);
round_shift_primitive_signed!(isize, i128);
round_shift_primitive_signed!(isize, isize);

macro_rules! round_shift_signed_unsigned {
    ($t:ident, $u:ident) => {
        impl ShrRound<$u> for $t {
            type Output = $t;

            fn shr_round(self, other: $u, rm: RoundingMode) -> $t {
                let abs = self.unsigned_abs();
                if self >= 0 {
                    $t::wrapping_from(abs.shr_round(other, rm))
                } else {
                    let abs_shifted = abs.shr_round(other, -rm);
                    if abs_shifted == 0 {
                        0
                    } else if abs_shifted == $t::MIN.unsigned_abs() {
                        $t::MIN
                    } else {
                        -$t::wrapping_from(abs_shifted)
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
round_shift_signed_unsigned!(i8, usize);
round_shift_signed_unsigned!(i16, u8);
round_shift_signed_unsigned!(i16, u16);
round_shift_signed_unsigned!(i16, u32);
round_shift_signed_unsigned!(i16, u64);
round_shift_signed_unsigned!(i16, u128);
round_shift_signed_unsigned!(i16, usize);
round_shift_signed_unsigned!(i32, u8);
round_shift_signed_unsigned!(i32, u16);
round_shift_signed_unsigned!(i32, u32);
round_shift_signed_unsigned!(i32, u64);
round_shift_signed_unsigned!(i32, u128);
round_shift_signed_unsigned!(i32, usize);
round_shift_signed_unsigned!(i64, u8);
round_shift_signed_unsigned!(i64, u16);
round_shift_signed_unsigned!(i64, u32);
round_shift_signed_unsigned!(i64, u64);
round_shift_signed_unsigned!(i64, u128);
round_shift_signed_unsigned!(i64, usize);
round_shift_signed_unsigned!(i128, u8);
round_shift_signed_unsigned!(i128, u16);
round_shift_signed_unsigned!(i128, u32);
round_shift_signed_unsigned!(i128, u64);
round_shift_signed_unsigned!(i128, u128);
round_shift_signed_unsigned!(i128, usize);
round_shift_signed_unsigned!(isize, u8);
round_shift_signed_unsigned!(isize, u16);
round_shift_signed_unsigned!(isize, u32);
round_shift_signed_unsigned!(isize, u64);
round_shift_signed_unsigned!(isize, u128);
round_shift_signed_unsigned!(isize, usize);
