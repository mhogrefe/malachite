use std::fmt::{Binary, Debug, Display, LowerHex, Octal, UpperHex};
use std::hash::Hash;
use std::iter::{Product, Sum};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};
use std::str::FromStr;

use comparison::{Max, Min};
use crement::Crementable;
use named::Named;
use num::arithmetic::traits::{
    AddMul, AddMulAssign, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedPow, CheckedRem,
    CheckedSub, DivAssignMod, DivAssignRem, DivExact, DivExactAssign, DivMod, DivRem, DivRound,
    DivRoundAssign, DivisibleBy, DivisibleByPowerOfTwo, EqMod, EqModPowerOfTwo, Mod, ModAssign,
    ModPowerOfTwo, OverflowingAdd, OverflowingAddAssign, OverflowingDiv, OverflowingDivAssign,
    OverflowingMul, OverflowingMulAssign, OverflowingNeg, OverflowingNegAssign, OverflowingPow,
    OverflowingRem, OverflowingRemAssign, OverflowingSub, OverflowingSubAssign, Parity, Pow,
    PowerOfTwo, SaturatingAdd, SaturatingAddAssign, SaturatingMul, SaturatingMulAssign,
    SaturatingPow, SaturatingSub, SaturatingSubAssign, ShlRound, ShlRoundAssign, ShrRound,
    ShrRoundAssign, Sign, SubMul, SubMulAssign, TrueCheckedShl, TrueCheckedShr, WrappingAdd,
    WrappingAddAssign, WrappingDiv, WrappingDivAssign, WrappingMul, WrappingMulAssign, WrappingNeg,
    WrappingNegAssign, WrappingPow, WrappingRem, WrappingRemAssign, WrappingSub, WrappingSubAssign,
};
use num::basic::traits::{One, Two, Zero};
use num::comparison::traits::{OrdAbs, PartialOrdAbs};
use num::conversion::traits::{
    CheckedFrom, CheckedInto, ExactFrom, ExactInto, OverflowingFrom, OverflowingInto,
    SaturatingFrom, SaturatingInto, WrappingFrom, WrappingInto,
};
use num::logic::traits::{
    BitAccess, BitBlockAccess, BitConvertible, BitIterable, BitScan, CountOnes, CountZeros,
    LeadingZeros, LowMask, NotAssign, Rotate, SignificantBits, TrailingZeros,
};

/// This trait defines functions on primitive integral types: uxx, ixx, usize, and isize.
pub trait PrimitiveInteger:
    'static
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + AddMul<Self, Self, Output = Self>
    + AddMulAssign<Self, Self>
    + Binary
    + BitAccess
    + BitAnd<Self, Output = Self>
    + BitAndAssign<Self>
    + BitBlockAccess
    + BitConvertible
    + BitIterable
    + BitOr<Self, Output = Self>
    + BitOrAssign<Self>
    + BitScan
    + BitXor<Self, Output = Self>
    + BitXorAssign<Self>
    + CheckedAdd<Self, Output = Self>
    + CheckedDiv<Self, Output = Self>
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
    + CheckedMul<Self, Output = Self>
    + CheckedNeg<Output = Self>
    + CheckedPow<u64, Output = Self>
    + CheckedRem<Self, Output = Self>
    + CheckedSub<Self, Output = Self>
    + Clone
    + Copy
    + CountOnes
    + CountZeros
    + Debug
    + Default
    + Display
    + Div<Self, Output = Self>
    + DivAssign
    + DivAssignMod<Self, ModOutput = Self>
    + DivAssignRem<Self, RemOutput = Self>
    + DivExact
    + DivExactAssign
    + DivisibleBy
    + DivisibleByPowerOfTwo
    + DivMod<Self, DivOutput = Self, ModOutput = Self>
    + DivRem<Self, DivOutput = Self, RemOutput = Self>
    + DivRound<Self, Output = Self>
    + DivRoundAssign
    + Eq
    + EqMod<Self, Self>
    + EqModPowerOfTwo<Self>
    + ExactFrom<u8>
    + ExactFrom<u16>
    + ExactFrom<u32>
    + ExactFrom<u64>
    + ExactFrom<u128>
    + ExactFrom<usize>
    + ExactFrom<i8>
    + ExactFrom<i16>
    + ExactFrom<i32>
    + ExactFrom<i64>
    + ExactFrom<i128>
    + ExactFrom<isize>
    + ExactInto<u8>
    + ExactInto<u16>
    + ExactInto<u32>
    + ExactInto<u64>
    + ExactInto<u128>
    + ExactInto<usize>
    + ExactInto<i8>
    + ExactInto<i16>
    + ExactInto<i32>
    + ExactInto<i64>
    + ExactInto<i128>
    + ExactInto<isize>
    + FromStr
    + Hash
    + LeadingZeros
    + LowerHex
    + LowMask
    + Min
    + Max
    + Mod
    + ModAssign<Self>
    + ModPowerOfTwo
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Named
    + Not<Output = Self>
    + NotAssign
    + Octal
    + One
    + Ord
    + OrdAbs
    + OverflowingAdd<Self, Output = Self>
    + OverflowingAddAssign<Self>
    + OverflowingDiv<Self, Output = Self>
    + OverflowingDivAssign<Self>
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
    + OverflowingMul<Self, Output = Self>
    + OverflowingMulAssign<Self>
    + OverflowingNeg<Output = Self>
    + OverflowingNegAssign
    + OverflowingPow<u64, Output = Self>
    + OverflowingRem<Self, Output = Self>
    + OverflowingRemAssign<Self>
    + OverflowingSub<Self, Output = Self>
    + OverflowingSubAssign<Self>
    + Parity
    + PartialEq<Self>
    + PartialOrd<Self>
    + PartialOrdAbs<Self>
    + Pow<u64>
    + PowerOfTwo
    + Product
    + Rem<Self, Output = Self>
    + RemAssign<Self>
    + Rotate
    + SaturatingAdd<Self, Output = Self>
    + SaturatingAddAssign<Self>
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
    + SaturatingMul<Self, Output = Self>
    + SaturatingMulAssign<Self>
    + SaturatingPow<u64, Output = Self>
    + SaturatingSub<Self, Output = Self>
    + SaturatingSubAssign<Self>
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
    + Sign
    + SignificantBits
    + Sized
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + SubMul<Self, Self, Output = Self>
    + SubMulAssign<Self, Self>
    + Sum<Self>
    + TrailingZeros
    + TrueCheckedShl<Output = Self>
    + TrueCheckedShr<Output = Self>
    + Two
    + UpperHex
    + Crementable
    + WrappingAdd<Self, Output = Self>
    + WrappingAddAssign<Self>
    + WrappingDiv<Self, Output = Self>
    + WrappingDivAssign<Self>
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
    + WrappingMul<Self, Output = Self>
    + WrappingMulAssign<Self>
    + WrappingNeg<Output = Self>
    + WrappingNegAssign
    + WrappingPow<u64, Output = Self>
    + WrappingRem<Self, Output = Self>
    + WrappingRemAssign<Self>
    + WrappingSub<Self, Output = Self>
    + WrappingSubAssign<Self>
    + Zero
{
    /// The number of bits of `Self`.
    const WIDTH: u64;

    /// The base-2 logarithm of the number of bits of `Self`. Instead of `n / WIDTH`, use
    /// `n >> LOG_WIDTH`.
    ///
    /// Note that this value is correct for all of the built-in primitive integer types, but it will
    /// not be correct for custom types with a non-power-of-two `WIDTH`. For such implementations
    /// `LOG_WIDTH` should not be used.
    const LOG_WIDTH: u64 = Self::WIDTH.trailing_zeros() as u64;

    /// A mask that consists of `LOG_WIDTH` bits. Instead of `n % WIDTH`, use `n & WIDTH_MASK`.
    ///
    /// Note that this value is correct for all of the built-in primitive integer types, but it will
    /// not be correct for custom types with a non-power-of-two `WIDTH`. For such implementations
    /// `WIDTH_MASK` should not be used.
    const WIDTH_MASK: u64 = Self::WIDTH - 1;

    /// Gets the most-significant bit of `Self`. For signed integers, this is the sign bit.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::integers::PrimitiveInteger;
    ///
    /// assert_eq!(123u32.get_highest_bit(), false);
    /// assert_eq!(4_000_000_000u32.get_highest_bit(), true);
    /// assert_eq!(2_000_000_000i32.get_highest_bit(), false);
    /// assert_eq!((-2_000_000_000i32).get_highest_bit(), true);
    /// ```
    #[inline]
    fn get_highest_bit(&self) -> bool {
        self.get_bit(Self::WIDTH - 1)
    }
}

/// This macro defines basic trait implementations that are the same for unsigned and signed types.
macro_rules! impl_basic_traits {
    ($t:ident, $width:expr) => {
        /// # Examples
        /// ```
        /// use malachite_base::num::basic::integers::PrimitiveInteger;
        ///
        /// assert_eq!(u32::WIDTH, 32);
        /// assert_eq!(u32::LOG_WIDTH, 5);
        /// assert_eq!(u32::WIDTH_MASK, 0x1f);
        /// ```
        impl PrimitiveInteger for $t {
            const WIDTH: u64 = $width;
        }

        impl_named!($t);

        /// The constant 0.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl Zero for $t {
            const ZERO: $t = 0;
        }

        /// The constant 1.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl One for $t {
            const ONE: $t = 1;
        }

        /// The constant 2.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl Two for $t {
            const TWO: $t = 2;
        }

        /// The lowest value representable by this type.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl Min for $t {
            const MIN: $t = std::$t::MIN;
        }

        /// The highest value representable by this type.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl Max for $t {
            const MAX: $t = std::$t::MAX;
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
            /// let mut i = 10;
            /// i.increment();
            /// assert_eq!(i, 11);
            ///
            /// let mut i = -5;
            /// i.increment();
            /// assert_eq!(i, -4);
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
            /// let mut i = 10;
            /// i.decrement();
            /// assert_eq!(i, 9);
            ///
            /// let mut i = -5;
            /// i.decrement();
            /// assert_eq!(i, -6);
            /// ```
            #[inline]
            fn decrement(&mut self) {
                *self = self
                    .checked_sub(1)
                    .expect("Cannot decrement past the minimum value.");
            }
        }
    };
}

impl_basic_traits!(u8, 8);
impl_basic_traits!(u16, 16);
impl_basic_traits!(u32, 32);
impl_basic_traits!(u64, 64);
impl_basic_traits!(u128, 128);
impl_basic_traits!(usize, 0usize.trailing_zeros() as u64);
impl_basic_traits!(i8, 8);
impl_basic_traits!(i16, 16);
impl_basic_traits!(i32, 32);
impl_basic_traits!(i64, 64);
impl_basic_traits!(i128, 128);
impl_basic_traits!(isize, 0usize.trailing_zeros() as u64);
