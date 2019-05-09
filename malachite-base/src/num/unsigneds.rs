use std::cmp::Ordering;
use std::num::ParseIntError;

use comparison::{Max, Min};
use conversion::WrappingFrom;
use crement::Crementable;
use named::Named;
use num::integers::PrimitiveInteger;
use num::signeds::PrimitiveSigned;
use num::traits::{
    BitAccess, BitScan, CeilingDivAssignNegMod, CeilingDivNegMod, CeilingLogTwo, CheckedAdd,
    CheckedDiv, CheckedMul, CheckedNeg, CheckedNextPowerOfTwo, CheckedRem, CheckedShl, CheckedShr,
    CheckedSub, CountOnes, CountZeros, DivAssignMod, DivAssignRem, DivExact, DivExactAssign,
    DivMod, DivRem, DivRound, DivRoundAssign, DivisibleBy, DivisibleByPowerOfTwo, Endian, EqMod,
    EqModPowerOfTwo, FloorLogTwo, FromStrRadix, FromU32Slice, HammingDistance, HasHalf,
    IsPowerOfTwo, JoinHalves, LeadingZeros, Mod, ModAssign, ModPowerOfTwo, ModPowerOfTwoAssign,
    NegMod, NegModAssign, NextPowerOfTwo, NextPowerOfTwoAssign, NotAssign, One, OrdAbs,
    OverflowingAdd, OverflowingAddAssign, OverflowingDiv, OverflowingDivAssign, OverflowingMul,
    OverflowingMulAssign, OverflowingNeg, OverflowingNegAssign, OverflowingRem,
    OverflowingRemAssign, OverflowingShl, OverflowingShr, OverflowingSub, OverflowingSubAssign,
    Parity, PartialOrdAbs, Pow, RemPowerOfTwo, RemPowerOfTwoAssign, RotateLeft, RotateRight,
    SaturatingAdd, SaturatingAddAssign, SaturatingMul, SaturatingMulAssign, SaturatingSub,
    SaturatingSubAssign, ShrRound, ShrRoundAssign, SignificantBits, SplitInHalf, TrailingZeros,
    Two, WrappingAdd, WrappingAddAssign, WrappingDiv, WrappingDivAssign, WrappingMul,
    WrappingMulAssign, WrappingNeg, WrappingNegAssign, WrappingRem, WrappingRemAssign, WrappingShl,
    WrappingShr, WrappingSub, WrappingSubAssign, Zero,
};
use round::RoundingMode;

//TODO docs
pub trait PrimitiveUnsigned:
    CeilingLogTwo
    + CeilingDivNegMod
    + CeilingDivAssignNegMod
    + CheckedNextPowerOfTwo<Output = Self>
    + FloorLogTwo
    + From<u8>
    + FromU32Slice
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
}

//TODO docs
macro_rules! unsigned_traits {
    ($t:ident, $s:ident, $log_width:expr) => {
        integer_traits!($t, $log_width);

        impl PrimitiveUnsigned for $t {
            type SignedOfEqualWidth = $s;
        }

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
            /// use malachite_base::num::traits::SignificantBits;
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
            /// use malachite_base::num::traits::FloorLogTwo;
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
            /// use malachite_base::num::traits::CeilingLogTwo;
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
        /// use malachite_base::num::traits::BitAccess;
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
            /// use malachite_base::num::traits::BitAccess;
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
            /// use malachite_base::num::traits::BitAccess;
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
            /// use malachite_base::num::traits::BitAccess;
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

unsigned_traits!(u8, i8, 3);
unsigned_traits!(u16, i16, 4);
unsigned_traits!(u32, i32, 5);
unsigned_traits!(u64, i64, 6);
unsigned_traits!(u128, i128, 7);
unsigned_traits!(usize, isize, 0usize.trailing_zeros().trailing_zeros());

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

impl01_unsigned!(u8);
impl01_unsigned!(u16);
impl01_unsigned!(u32);
impl01_unsigned!(u64);
impl01_unsigned!(u128);
impl01_unsigned!(usize);

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
            /// use malachite_base::num::traits::JoinHalves;
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
        /// use malachite_base::num::traits::SplitInHalf;
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
            /// use malachite_base::num::traits::SplitInHalf;
            ///
            /// assert_eq!(258u16.lower_half(), 2);
            /// assert_eq!(0xabcd1234u32.lower_half(), 0x1234);
            /// ```
            #[inline]
            fn lower_half(&self) -> Self::Half {
                $ht::wrapping_from(*self)
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
            /// use malachite_base::num::traits::SplitInHalf;
            ///
            /// assert_eq!(258u16.upper_half(), 1);
            /// assert_eq!(0xabcd1234u32.upper_half(), 0xabcd);
            /// ```
            #[inline]
            fn upper_half(&self) -> Self::Half {
                $ht::wrapping_from(self >> $ht::WIDTH)
            }
        }
    };
}

impl_halves_unsigned!(u16, u8);
impl_halves_unsigned!(u32, u16);
impl_halves_unsigned!(u64, u32);
impl_halves_unsigned!(u128, u64);

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
round_shift_unsigned_unsigned!(u8, usize);
round_shift_unsigned_unsigned!(u16, u8);
round_shift_unsigned_unsigned!(u16, u16);
round_shift_unsigned_unsigned!(u16, u32);
round_shift_unsigned_unsigned!(u16, u64);
round_shift_unsigned_unsigned!(u16, u128);
round_shift_unsigned_unsigned!(u16, usize);
round_shift_unsigned_unsigned!(u32, u8);
round_shift_unsigned_unsigned!(u32, u16);
round_shift_unsigned_unsigned!(u32, u32);
round_shift_unsigned_unsigned!(u32, u64);
round_shift_unsigned_unsigned!(u32, u128);
round_shift_unsigned_unsigned!(u32, usize);
round_shift_unsigned_unsigned!(u64, u8);
round_shift_unsigned_unsigned!(u64, u16);
round_shift_unsigned_unsigned!(u64, u32);
round_shift_unsigned_unsigned!(u64, u64);
round_shift_unsigned_unsigned!(u64, u128);
round_shift_unsigned_unsigned!(u64, usize);
round_shift_unsigned_unsigned!(u128, u8);
round_shift_unsigned_unsigned!(u128, u16);
round_shift_unsigned_unsigned!(u128, u32);
round_shift_unsigned_unsigned!(u128, u64);
round_shift_unsigned_unsigned!(u128, u128);
round_shift_unsigned_unsigned!(u128, usize);
round_shift_unsigned_unsigned!(usize, u8);
round_shift_unsigned_unsigned!(usize, u16);
round_shift_unsigned_unsigned!(usize, u32);
round_shift_unsigned_unsigned!(usize, u64);
round_shift_unsigned_unsigned!(usize, u128);
round_shift_unsigned_unsigned!(usize, usize);

//TODO doc and test
impl FromU32Slice for u8 {
    #[inline]
    fn from_u32_slice(slice: &[u32]) -> Self {
        assert!(!slice.is_empty());
        u8::wrapping_from(slice[0])
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
        u16::wrapping_from(slice[0])
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

//TODO doc and test
impl FromU32Slice for usize {
    #[inline]
    fn from_u32_slice(slice: &[u32]) -> Self {
        match usize::WIDTH {
            u32::WIDTH => usize::wrapping_from(u32::from_u32_slice(slice)),
            u64::WIDTH => usize::wrapping_from(u64::from_u32_slice(slice)),
            _ => panic!("unexpected usize size: {}", usize::WIDTH),
        }
    }

    fn copy_from_u32_slice(out_slice: &mut [usize], in_slice: &[u32]) {
        match usize::WIDTH {
            u32::WIDTH => {
                let out_len = out_slice.len();
                assert!(out_len >= in_slice.len());
                for (out, &x) in out_slice.iter_mut().zip(in_slice.iter()) {
                    *out = usize::wrapping_from(x);
                }
            }
            u64::WIDTH => {
                let out_len = out_slice.len();
                assert!(out_len >= in_slice.len() >> 1);
                let mut i = 0;
                for out in out_slice.iter_mut() {
                    *out = usize::wrapping_from(u64::join_halves(in_slice[i + 1], in_slice[i]));
                    i += 2;
                }
            }
            _ => panic!("unexpected usize size: {}", usize::WIDTH),
        }
    }
}
