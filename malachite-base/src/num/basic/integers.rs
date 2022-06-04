use comparison::traits::{Max, Min};
use named::Named;
use num::arithmetic::traits::{
    AddMul, AddMulAssign, ArithmeticCheckedShl, ArithmeticCheckedShr, CeilingRoot,
    CeilingRootAssign, CeilingSqrt, CeilingSqrtAssign, CheckedAdd, CheckedAddMul, CheckedDiv,
    CheckedMul, CheckedNeg, CheckedPow, CheckedRoot, CheckedSqrt, CheckedSquare, CheckedSub,
    CheckedSubMul, DivAssignMod, DivAssignRem, DivExact, DivExactAssign, DivMod, DivRem, DivRound,
    DivRoundAssign, DivisibleBy, DivisibleByPowerOf2, EqMod, EqModPowerOf2, FloorRoot,
    FloorRootAssign, FloorSqrt, FloorSqrtAssign, Mod, ModAssign, ModPowerOf2, ModPowerOf2Assign,
    OverflowingAdd, OverflowingAddAssign, OverflowingAddMul, OverflowingAddMulAssign,
    OverflowingDiv, OverflowingDivAssign, OverflowingMul, OverflowingMulAssign, OverflowingNeg,
    OverflowingNegAssign, OverflowingPow, OverflowingPowAssign, OverflowingSquare,
    OverflowingSquareAssign, OverflowingSub, OverflowingSubAssign, OverflowingSubMul,
    OverflowingSubMulAssign, Parity, Pow, PowAssign, PowerOf2, RemPowerOf2, RemPowerOf2Assign,
    RoundToMultiple, RoundToMultipleAssign, RoundToMultipleOfPowerOf2,
    RoundToMultipleOfPowerOf2Assign, SaturatingAdd, SaturatingAddAssign, SaturatingAddMul,
    SaturatingAddMulAssign, SaturatingMul, SaturatingMulAssign, SaturatingPow, SaturatingPowAssign,
    SaturatingSquare, SaturatingSquareAssign, SaturatingSub, SaturatingSubAssign, SaturatingSubMul,
    SaturatingSubMulAssign, ShlRound, ShlRoundAssign, ShrRound, ShrRoundAssign, Sign, Square,
    SquareAssign, SubMul, SubMulAssign, WrappingAdd, WrappingAddAssign, WrappingAddMul,
    WrappingAddMulAssign, WrappingDiv, WrappingDivAssign, WrappingMul, WrappingMulAssign,
    WrappingNeg, WrappingNegAssign, WrappingPow, WrappingPowAssign, WrappingSquare,
    WrappingSquareAssign, WrappingSub, WrappingSubAssign, WrappingSubMul, WrappingSubMulAssign,
};
use num::basic::traits::{Iverson, One, Two, Zero};
use num::comparison::traits::{EqAbs, OrdAbs, PartialOrdAbs};
use num::conversion::traits::{
    CheckedFrom, CheckedInto, ConvertibleFrom, ExactFrom, ExactInto, FromSciString, FromStringBase,
    IsInteger, OverflowingFrom, OverflowingInto, RoundingFrom, RoundingInto, SaturatingFrom,
    SaturatingInto, ToSci, ToStringBase, WrappingFrom, WrappingInto,
};
use num::logic::traits::{
    BitAccess, BitBlockAccess, BitConvertible, BitIterable, BitScan, CountOnes, CountZeros,
    LeadingZeros, LowMask, NotAssign, SignificantBits, TrailingZeros,
};
use num::random::HasRandomPrimitiveInts;
use std::fmt::{Binary, Debug, Display, LowerHex, Octal, UpperHex};
use std::hash::Hash;
use std::iter::{Product, Sum};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};
use std::str::FromStr;

/// Defines functions on primitive integer types: uxx, ixx, usize, and isize.
///
/// The different types are distinguished by whether they are signed or unsigned, and by their
/// widths. The width $W$ is the number of bits in the type. For example, the width of [`u32`] or
/// [`i32`] is 32. Each type has $2^W$ distinct values.
///
/// Let $n$ be a value of type `Self`. If `Self` is unsigned, $0 \leq n < 2^W$. If `Self`
/// is signed, $2^{W-1} \leq n < 2^{W-1}$.
pub trait PrimitiveInt:
    'static
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + AddMul<Self, Self, Output = Self>
    + AddMulAssign<Self, Self>
    + ArithmeticCheckedShl<u8, Output = Self>
    + ArithmeticCheckedShl<u16, Output = Self>
    + ArithmeticCheckedShl<u32, Output = Self>
    + ArithmeticCheckedShl<u64, Output = Self>
    + ArithmeticCheckedShl<u128, Output = Self>
    + ArithmeticCheckedShl<usize, Output = Self>
    + ArithmeticCheckedShl<i8, Output = Self>
    + ArithmeticCheckedShl<i16, Output = Self>
    + ArithmeticCheckedShl<i32, Output = Self>
    + ArithmeticCheckedShl<i64, Output = Self>
    + ArithmeticCheckedShl<i128, Output = Self>
    + ArithmeticCheckedShl<isize, Output = Self>
    + ArithmeticCheckedShr<i8, Output = Self>
    + ArithmeticCheckedShr<i16, Output = Self>
    + ArithmeticCheckedShr<i32, Output = Self>
    + ArithmeticCheckedShr<i64, Output = Self>
    + ArithmeticCheckedShr<i128, Output = Self>
    + ArithmeticCheckedShr<isize, Output = Self>
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
    + CeilingRoot<u64, Output = Self>
    + CeilingRootAssign<u64>
    + CeilingSqrt<Output = Self>
    + CeilingSqrtAssign
    + CheckedAdd<Self, Output = Self>
    + CheckedAddMul<Self, Self, Output = Self>
    + CheckedDiv<Self, Output = Self>
    + CheckedFrom<f32>
    + CheckedFrom<f64>
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
    + CheckedRoot<u64, Output = Self>
    + CheckedSqrt<Output = Self>
    + CheckedSquare<Output = Self>
    + CheckedSub<Self, Output = Self>
    + CheckedSubMul<Self, Self, Output = Self>
    + Clone
    + ConvertibleFrom<f32>
    + ConvertibleFrom<f64>
    + ConvertibleFrom<u8>
    + ConvertibleFrom<u16>
    + ConvertibleFrom<u32>
    + ConvertibleFrom<u64>
    + ConvertibleFrom<u128>
    + ConvertibleFrom<usize>
    + ConvertibleFrom<i8>
    + ConvertibleFrom<i16>
    + ConvertibleFrom<i32>
    + ConvertibleFrom<i64>
    + ConvertibleFrom<i128>
    + ConvertibleFrom<isize>
    + Copy
    + CountOnes
    + CountZeros
    + Debug
    + Default
    + Display
    + Div<Self, Output = Self>
    + DivAssign<Self>
    + DivAssignMod<Self, ModOutput = Self>
    + DivAssignRem<Self, RemOutput = Self>
    + DivExact<Self, Output = Self>
    + DivExactAssign<Self>
    + DivisibleBy<Self>
    + DivisibleByPowerOf2
    + DivMod<Self, DivOutput = Self, ModOutput = Self>
    + DivRem<Self, DivOutput = Self, RemOutput = Self>
    + DivRound<Self, Output = Self>
    + DivRoundAssign<Self>
    + Eq
    + EqAbs<Self>
    + EqMod<Self, Self>
    + EqModPowerOf2<Self>
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
    + FloorRoot<u64, Output = Self>
    + FloorRootAssign<u64>
    + FloorSqrt<Output = Self>
    + FloorSqrtAssign
    + FromSciString
    + FromStr
    + FromStringBase
    + Hash
    + HasRandomPrimitiveInts
    + IsInteger
    + Iverson
    + LeadingZeros
    + LowerHex
    + LowMask
    + Min
    + Max
    + Mod<Self, Output = Self>
    + ModAssign<Self>
    + ModPowerOf2
    + ModPowerOf2Assign
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
    + OverflowingAddMul<Self, Self, Output = Self>
    + OverflowingAddMulAssign<Self, Self>
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
    + OverflowingPowAssign<u64>
    + OverflowingSquare<Output = Self>
    + OverflowingSquareAssign
    + OverflowingSub<Self, Output = Self>
    + OverflowingSubAssign<Self>
    + OverflowingSubMul<Self, Self, Output = Self>
    + OverflowingSubMulAssign<Self, Self>
    + Parity
    + PartialEq<Self>
    + PartialOrd<Self>
    + PartialOrdAbs<Self>
    + Pow<u64, Output = Self>
    + PowAssign<u64>
    + PowerOf2<u64>
    + Product
    + Rem<Self, Output = Self>
    + RemAssign<Self>
    + RemPowerOf2<Output = Self>
    + RemPowerOf2Assign
    + RoundingFrom<f32>
    + RoundingFrom<f64>
    + RoundingInto<f32>
    + RoundingInto<f64>
    + RoundToMultiple<Self, Output = Self>
    + RoundToMultipleAssign<Self>
    + RoundToMultipleOfPowerOf2<u64, Output = Self>
    + RoundToMultipleOfPowerOf2Assign<u64>
    + SaturatingAdd<Self, Output = Self>
    + SaturatingAddAssign<Self>
    + SaturatingAddMul<Self, Self, Output = Self>
    + SaturatingAddMulAssign<Self, Self>
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
    + SaturatingPowAssign<u64>
    + SaturatingSquare<Output = Self>
    + SaturatingSquareAssign
    + SaturatingSub<Self, Output = Self>
    + SaturatingSubAssign<Self>
    + SaturatingSubMul<Self, Self, Output = Self>
    + SaturatingSubMulAssign<Self, Self>
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
    + Square<Output = Self>
    + SquareAssign
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + SubMul<Self, Self, Output = Self>
    + SubMulAssign<Self, Self>
    + Sum<Self>
    + ToSci
    + ToStringBase
    + TrailingZeros
    + Two
    + UpperHex
    + WrappingAdd<Self, Output = Self>
    + WrappingAddAssign<Self>
    + WrappingAddMul<Self, Self, Output = Self>
    + WrappingAddMulAssign<Self, Self>
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
    + WrappingPowAssign<u64>
    + WrappingSquare<Output = Self>
    + WrappingSquareAssign
    + WrappingSub<Self, Output = Self>
    + WrappingSubAssign<Self>
    + WrappingSubMul<Self, Self, Output = Self>
    + WrappingSubMulAssign<Self, Self>
    + Zero
{
    /// The number of bits of `Self`.
    const WIDTH: u64;

    /// The base-2 logarithm of the number of bits of `Self`.
    ///
    /// Whenever you need to use `n / WIDTH`, you can use `n >> LOG_WIDTH` instead.
    ///
    /// This is $\log_2 W$.
    ///
    /// Note that this value is correct for all of the built-in primitive integer types, but it will
    /// not be correct for custom types whose $W$ is not a power of 2. For such implementations,
    /// `LOG_WIDTH` should not be used.
    const LOG_WIDTH: u64 = Self::WIDTH.trailing_zeros() as u64;

    /// A mask that consists of `LOG_WIDTH` bits.
    ///
    /// Whenever you need to use `n % WIDTH`, you can use `n & WIDTH_MASK` instead.
    ///
    /// This is $W - 1$.
    ///
    /// Note that this value is correct for all of the built-in primitive integer types, but it will
    /// not be correct for custom types whose $W$ is not a power of 2. For such implementations,
    /// `WIDTH_MASK` should not be used.
    const WIDTH_MASK: u64 = Self::WIDTH - 1;

    /// Gets the most-significant bit of `Self`. For signed integers, this is the sign bit.
    ///
    /// If `Self` is unsigned, $f(n) = (n \geq 2^{W-1})$. If `Self` is unsigned, $f(n) = (n < 0)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    ///
    /// assert_eq!(123u32.get_highest_bit(), false);
    /// assert_eq!(4000000000u32.get_highest_bit(), true);
    /// assert_eq!(2000000000i32.get_highest_bit(), false);
    /// assert_eq!((-2000000000i32).get_highest_bit(), true);
    /// ```
    #[inline]
    fn get_highest_bit(&self) -> bool {
        self.get_bit(Self::WIDTH - 1)
    }
}

/// Defines basic trait implementations that are the same for unsigned and signed types.
macro_rules! impl_basic_traits_primitive_int {
    ($t:ident, $width:expr) => {
        /// # Examples
        ///
        /// See [here](self).
        impl PrimitiveInt for $t {
            const WIDTH: u64 = $width;
        }

        impl_named!($t);

        /// The constant 0.
        ///
        /// # Examples
        /// See [here](self).
        impl Zero for $t {
            const ZERO: $t = 0;
        }

        /// The constant 1.
        ///
        /// # Examples
        /// See [here](self).
        impl One for $t {
            const ONE: $t = 1;
        }

        /// The constant 2.
        ///
        /// # Examples
        /// See [here](self).
        impl Two for $t {
            const TWO: $t = 2;
        }

        /// The lowest value representable by this type.
        ///
        /// If `Self` is unsigned, `MIN` is 0. If `Self` is signed, `MIN` is $-2^{W-1}$.
        ///
        /// # Examples
        /// See [here](self).
        impl Min for $t {
            const MIN: $t = std::$t::MIN;
        }

        /// The highest value representable by this type.
        ///
        /// If `Self` is unsigned, `MAX` is $2^W-1$. If `Self` is signed, `MAX` is $2^{W-1}-1$.
        ///
        /// # Examples
        /// See [here](self).
        impl Max for $t {
            const MAX: $t = std::$t::MAX;
        }
    };
}
impl_basic_traits_primitive_int!(u8, 8);
impl_basic_traits_primitive_int!(u16, 16);
impl_basic_traits_primitive_int!(u32, 32);
impl_basic_traits_primitive_int!(u64, 64);
impl_basic_traits_primitive_int!(u128, 128);
impl_basic_traits_primitive_int!(usize, 0usize.trailing_zeros() as u64);
impl_basic_traits_primitive_int!(i8, 8);
impl_basic_traits_primitive_int!(i16, 16);
impl_basic_traits_primitive_int!(i32, 32);
impl_basic_traits_primitive_int!(i64, 64);
impl_basic_traits_primitive_int!(i128, 128);
impl_basic_traits_primitive_int!(isize, 0usize.trailing_zeros() as u64);
