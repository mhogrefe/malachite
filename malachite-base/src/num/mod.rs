use misc::{Max, Min, Named, Walkable};
use rand::distributions::range::SampleRange;
use rand::Rand;
use round::RoundingMode;
use std;
use std::cmp::Ordering;
use std::fmt::{Binary, Debug, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex};
use std::hash::Hash;
use std::iter::{Product, Sum};
use std::num::ParseIntError;
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
    fn rotate_left(&self, n: u32) -> Self;
}

/// Shifts the bits to the right by a specified amount, `n`, wrapping the truncated bits to the end
/// of the resulting value.
///
/// Please note this isn't the same operation as `>>`!
pub trait RotateRight {
    fn rotate_right(&self, n: u32) -> Self;
}

/// Defines functions for manipulating the endianness of a value.
pub trait Endian {
    /// Reverses the byte order of the value.
    fn swap_bytes(&self) -> Self;

    /// Converts a value from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    fn from_be(x: &Self) -> Self;

    /// Converts a value from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    fn from_le(x: &Self) -> Self;

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    fn to_be(&self) -> Self;

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    fn to_le(&self) -> Self;
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

/// Raises `self` to the power of `exp`.
pub trait Pow<RHS> {
    type Output;

    fn pow(self, exp: RHS) -> Self::Output;
}

/// Returns `true` iff `self == 2^k` for some integer `k`.
pub trait IsPowerOfTwo {
    fn is_power_of_two(&self) -> bool;
}

/// Returns the smallest power of two greater than or equal to `self`.
pub trait NextPowerOfTwo {
    fn next_power_of_two(&self) -> Self;
}

/// Returns the smallest power of two greater than or equal to `self`. If the next power of two is
/// greater than the type's maximum value, `None` is returned, otherwise the power of two is wrapped
/// in `Some`.
pub trait CheckedNextPowerOfTwo: Sized {
    fn checked_next_power_of_two(&self) -> Option<Self>;
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

//TODO is_positive, is_negative, sign

//TODO docs
pub trait PrimitiveInteger
    : Add<Output = Self>
    + AddAssign<Self>
    + Binary
    + BitAccess
    + BitAnd<Output = Self>
    + BitAndAssign<Self>
    + BitOr<Output = Self>
    + BitOrAssign<Self>
    + BitXor<Output = Self>
    + BitXorAssign<Self>
    + CheckedAdd<Output = Self>
    + CheckedDiv<Output = Self>
    + CheckedMul<Output = Self>
    + CheckedNeg<Output = Self>
    + CheckedRem<Output = Self>
    + CheckedShl<Output = Self>
    + CheckedShr<Output = Self>
    + CheckedSub<Output = Self>
    + Copy
    + CountOnes
    + CountZeros
    + Debug
    + Default
    + Display
    + Div<Output = Self>
    + DivAssign
    + Endian
    + Eq
    + FromStr
    + Hash
    + LeadingZeros
    + LowerHex
    + Min
    + Max
    + Mul<Output = Self>
    + MulAssign<Self>
    + Named
    + Not<Output = Self>
    + Octal
    + One
    + Ord
    + OverflowingAdd<Output = Self>
    + OverflowingDiv<Output = Self>
    + OverflowingMul<Output = Self>
    + OverflowingNeg<Output = Self>
    + OverflowingRem<Output = Self>
    + OverflowingShl<Output = Self>
    + OverflowingShr<Output = Self>
    + OverflowingSub<Output = Self>
    + PartialEq<Self>
    + PartialOrd<Self>
    + Pow<u32>
    + Product
    + Rand
    + Rem<Output = Self>
    + RemAssign<Self>
    + RotateLeft
    + RotateRight
    + SampleRange
    + SaturatingAdd<Output = Self>
    + SaturatingMul<Output = Self>
    + SaturatingSub<Output = Self>
    + Shl<u32, Output = Self>
    + ShlAssign<u32>
    + Shr<u32, Output = Self>
    + ShrAssign<u32>
    + SignificantBits
    + Sized
    + Sub<Output = Self>
    + SubAssign<Self>
    + Sum<Self>
    + TrailingZeros
    + UpperHex
    + Walkable
    + WrappingAdd<Output = Self>
    + WrappingDiv<Output = Self>
    + WrappingMul<Output = Self>
    + WrappingNeg<Output = Self>
    + WrappingRem<Output = Self>
    + WrappingShl<Output = Self>
    + WrappingShr<Output = Self>
    + WrappingSub<Output = Self>
    + Zero {
    const WIDTH: u32;

    fn from_u32(u: u32) -> Self;

    fn from_u64(u: u64) -> Self;
}

//TODO docs
pub trait PrimitiveUnsigned
    : CheckedNextPowerOfTwo + IsPowerOfTwo + NextPowerOfTwo + PrimitiveInteger {
    type SignedOfEqualWidth;

    fn to_u64(&self) -> u64;
}

//TODO docs
pub trait PrimitiveSigned
    : Abs<Output = Self>
    + CheckedAbs<Output = Self>
    + Neg<Output = Self>
    + NegativeOne
    + Not<Output = Self>
    + OverflowingAbs<Output = Self>
    + PrimitiveInteger
    + WrappingAbs<Output = Self> {
    type UnsignedOfEqualWidth;

    fn from_i32(i: i32) -> Self;

    fn from_i64(i: i64) -> Self;
}

//TODO docs
pub trait FloatingPoint
    : Add<Output = Self>
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
    + One
    + PartialEq<Self>
    + PartialOrd<Self>
    + Product
    + Rand
    + Rem<Output = Self>
    + RemAssign<Self>
    + SampleRange
    + Sized
    + Sub<Output = Self>
    + SubAssign<Self>
    + Sum<Self>
    + UpperExp
    + Zero {
    type UnsignedOfEqualWidth;

    const EXPONENT_WIDTH: u32;
    const MANTISSA_WIDTH: u32;
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
    ($t: ident, $width: expr, $u: ident, $from_u32: expr, $from_u64: expr) => {
        //TODO docs
        impl PrimitiveInteger for $t {
            const WIDTH: u32 = $width;

            fn from_u32($u: u32) -> Self {
                $from_u32
            }

            fn from_u64($u: u64) -> Self {
                $from_u64
            }
        }

        impl_named!($t);

        impl FromStrRadix for $t {
            fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
                $t::from_str_radix(src, radix)
            }
        }

        impl CountZeros for $t {
            fn count_zeros(self) -> u32 {
                $t::count_zeros(self)
            }
        }

        impl CountOnes for $t {
            fn count_ones(self) -> u32 {
                $t::count_ones(self)
            }
        }

        impl LeadingZeros for $t {
            fn leading_zeros(self) -> u32 {
                $t::leading_zeros(self)
            }
        }

        impl TrailingZeros for $t {
            fn trailing_zeros(self) -> u32 {
                $t::trailing_zeros(self)
            }
        }

        impl RotateLeft for $t {
            fn rotate_left(&self, n: u32) -> $t {
                $t::rotate_left(*self, n)
            }
        }

        impl RotateRight for $t {
            fn rotate_right(&self, n: u32) -> $t {
                $t::rotate_right(*self, n)
            }
        }

        impl Endian for $t {
            fn swap_bytes(&self) -> $t {
                $t::swap_bytes(*self)
            }

            fn from_be(x: &Self) -> $t {
                $t::from_be(*x)
            }

            fn from_le(x: &Self) -> $t {
                $t::from_le(*x)
            }

            fn to_be(&self) -> $t {
                $t::to_be(*self)
            }

            fn to_le(&self) -> $t {
                $t::to_le(*self)
            }
        }

        impl CheckedAdd<$t> for $t {
            type Output = $t;

            fn checked_add(self, rhs: $t) -> Option<$t> {
                $t::checked_add(self, rhs)
            }
        }

        impl CheckedSub<$t> for $t {
            type Output = $t;

            fn checked_sub(self, rhs: $t) -> Option<$t> {
                $t::checked_sub(self, rhs)
            }
        }

        impl CheckedMul<$t> for $t {
            type Output = $t;

            fn checked_mul(self, rhs: $t) -> Option<$t> {
                $t::checked_mul(self, rhs)
            }
        }

        impl CheckedDiv<$t> for $t {
            type Output = $t;

            fn checked_div(self, rhs: $t) -> Option<$t> {
                $t::checked_div(self, rhs)
            }
        }

        impl CheckedRem<$t> for $t {
            type Output = $t;

            fn checked_rem(self, rhs: $t) -> Option<$t> {
                $t::checked_rem(self, rhs)
            }
        }

        impl CheckedNeg for $t {
            type Output = $t;

            fn checked_neg(self) -> Option<$t> {
                $t::checked_neg(self)
            }
        }

        impl CheckedShl for $t {
            type Output = $t;

            fn checked_shl(self, rhs: u32) -> Option<$t> {
                $t::checked_shl(self, rhs)
            }
        }

        impl CheckedShr for $t {
            type Output = $t;

            fn checked_shr(self, rhs: u32) -> Option<$t> {
                $t::checked_shr(self, rhs)
            }
        }

        impl SaturatingAdd<$t> for $t {
            type Output = $t;

            fn saturating_add(self, rhs: $t) -> $t {
                $t::saturating_add(self, rhs)
            }
        }

        impl SaturatingSub<$t> for $t {
            type Output = $t;

            fn saturating_sub(self, rhs: $t) -> $t {
                $t::saturating_sub(self, rhs)
            }
        }

        impl SaturatingMul<$t> for $t {
            type Output = $t;

            fn saturating_mul(self, rhs: $t) -> $t {
                $t::saturating_mul(self, rhs)
            }
        }

        impl WrappingAdd<$t> for $t {
            type Output = $t;

            fn wrapping_add(self, rhs: $t) -> $t {
                $t::wrapping_add(self, rhs)
            }
        }

        impl WrappingSub<$t> for $t {
            type Output = $t;

            fn wrapping_sub(self, rhs: $t) -> $t {
                $t::wrapping_sub(self, rhs)
            }
        }

        impl WrappingMul<$t> for $t {
            type Output = $t;

            fn wrapping_mul(self, rhs: $t) -> $t {
                $t::wrapping_mul(self, rhs)
            }
        }

        impl WrappingDiv<$t> for $t {
            type Output = $t;

            fn wrapping_div(self, rhs: $t) -> $t {
                $t::wrapping_div(self, rhs)
            }
        }

        impl WrappingRem<$t> for $t {
            type Output = $t;

            fn wrapping_rem(self, rhs: $t) -> $t {
                $t::wrapping_rem(self, rhs)
            }
        }

        impl WrappingNeg for $t {
            type Output = $t;

            fn wrapping_neg(self) -> $t {
                $t::wrapping_neg(self)
            }
        }

        impl WrappingShl for $t {
            type Output = $t;

            fn wrapping_shl(self, rhs: u32) -> $t {
                $t::wrapping_shl(self, rhs)
            }
        }

        impl WrappingShr for $t {
            type Output = $t;

            fn wrapping_shr(self, rhs: u32) -> $t {
                $t::wrapping_shr(self, rhs)
            }
        }

        impl OverflowingAdd<$t> for $t {
            type Output = $t;

            fn overflowing_add(self, rhs: $t) -> ($t, bool) {
                $t::overflowing_add(self, rhs)
            }
        }

        impl OverflowingSub<$t> for $t {
            type Output = $t;

            fn overflowing_sub(self, rhs: $t) -> ($t, bool) {
                $t::overflowing_sub(self, rhs)
            }
        }

        impl OverflowingMul<$t> for $t {
            type Output = $t;

            fn overflowing_mul(self, rhs: $t) -> ($t, bool) {
                $t::overflowing_mul(self, rhs)
            }
        }

        impl OverflowingDiv<$t> for $t {
            type Output = $t;

            fn overflowing_div(self, rhs: $t) -> ($t, bool) {
                $t::overflowing_div(self, rhs)
            }
        }

        impl OverflowingRem<$t> for $t {
            type Output = $t;

            fn overflowing_rem(self, rhs: $t) -> ($t, bool) {
                $t::overflowing_rem(self, rhs)
            }
        }

        impl OverflowingNeg for $t {
            type Output = $t;

            fn overflowing_neg(self) -> ($t, bool) {
                $t::overflowing_neg(self)
            }
        }

        impl OverflowingShl for $t {
            type Output = $t;

            fn overflowing_shl(self, rhs: u32) -> ($t, bool) {
                $t::overflowing_shl(self, rhs)
            }
        }

        impl OverflowingShr for $t {
            type Output = $t;

            fn overflowing_shr(self, rhs: u32) -> ($t, bool) {
                $t::overflowing_shr(self, rhs)
            }
        }

        impl Pow<u32> for $t {
            type Output = $t;

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
            fn increment(&mut self) {
                *self = self.checked_add(1).expect("Cannot increment past the maximum value.");
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
            fn decrement(&mut self) {
                *self = self.checked_sub(1).expect("Cannot decrement past the minimum value.");
            }
        }
    }
}

//TODO docs
macro_rules! unsigned_traits {
    ($t: ident, $width: expr, $u: ident, $from_u32: expr, $from_u64: expr) => {
        integer_traits!($t, $width, $u, $from_u32, $from_u64);

        impl IsPowerOfTwo for $t {
            fn is_power_of_two(&self) -> bool {
                $t::is_power_of_two(*self)
            }
        }

        impl NextPowerOfTwo for $t {
            fn next_power_of_two(&self) -> $t {
                $t::next_power_of_two(*self)
            }
        }

        impl CheckedNextPowerOfTwo for $t {
            fn checked_next_power_of_two(&self) -> Option<$t> {
                $t::checked_next_power_of_two(*self)
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
            fn significant_bits(self) -> u64 {
                (Self::WIDTH - self.leading_zeros()).into()
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
            fn get_bit(&self, index: u64) -> bool {
                index < Self::WIDTH.into() && *self & (Self::ONE << index) != Self::ZERO
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
            fn set_bit(&mut self, index: u64) {
                if index < Self::WIDTH.into() {
                    *self |= Self::ONE << index;
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
            fn clear_bit(&mut self, index: u64) {
                if index < Self::WIDTH.into() {
                    *self &= !(Self::ONE << index);
                }
            }
        }
    }
}

//TODO docs
macro_rules! signed_traits {
    (
        $t: ident,
        $ut: ident,
        $width: expr,
        $u: ident,
        $from_u32: expr,
        $from_u64: expr,
        $i: ident,
        $from_i32: expr,
        $from_i64: expr
    ) => {
        integer_traits!($t, $width, $u, $from_u32, $from_u64);

        //TODO docs
        impl PrimitiveSigned for $t {
            type UnsignedOfEqualWidth = $ut;

            fn from_i32($i: i32) -> Self {
                $from_i32
            }

            fn from_i64($i: i64) -> Self {
                $from_i64
            }
        }

        impl Abs for $t {
            type Output = $t;

            fn abs(self) -> $t {
                $t::abs(self)
            }
        }

        impl CheckedAbs for $t {
            type Output = $t;

            fn abs(self) -> Option<$t> {
                $t::checked_abs(self)
            }
        }

        impl WrappingAbs for $t {
            type Output = $t;

            fn abs(self) -> $t {
                $t::wrapping_abs(self)
            }
        }

        impl OverflowingAbs for $t {
            type Output = $t;

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
            fn significant_bits(self) -> u64 {
                (self.wrapping_abs() as $ut).significant_bits()
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
    }
}

//TODO docs
macro_rules! floating_point_traits {
    ($t: ident, $ut: ident, $exponent_width: expr, $mantissa_width: expr) => {
        //TODO docs
        impl FloatingPoint for $t {
            type UnsignedOfEqualWidth = $ut;

            const EXPONENT_WIDTH: u32 = $exponent_width;
            const MANTISSA_WIDTH: u32 = $mantissa_width;
        }

        impl_named!($t);

        impl Min for $t {
            const MIN: $t = std::$t::MIN;
        }

        impl Max for $t {
            const MAX: $t = std::$t::MAX;
        }
    }
}

//TODO docs
impl PrimitiveUnsigned for u8 {
    type SignedOfEqualWidth = i8;

    fn to_u64(&self) -> u64 {
        (*self).into()
    }
}

impl PrimitiveUnsigned for u16 {
    type SignedOfEqualWidth = i16;

    fn to_u64(&self) -> u64 {
        (*self).into()
    }
}

impl PrimitiveUnsigned for u32 {
    type SignedOfEqualWidth = i32;

    fn to_u64(&self) -> u64 {
        (*self).into()
    }
}

impl PrimitiveUnsigned for u64 {
    type SignedOfEqualWidth = i64;

    fn to_u64(&self) -> u64 {
        *self
    }
}

unsigned_traits!(u8, 8, u, u as u8, u as u8);
unsigned_traits!(u16, 16, u, u as u16, u as u16);
unsigned_traits!(u32, 32, u, u, u as u32);
unsigned_traits!(u64, 64, u, u.into(), u);

signed_traits!(i8, u8, 8, u, u as i8, u as i8, i, i as i8, i as i8);
signed_traits!(i16, u16, 16, u, u as i16, u as i16, i, i as i16, i as i16);
signed_traits!(i32, u32, 32, u, u as i32, u as i32, i, i, i as i32);
signed_traits!(i64, u64, 64, u, u.into(), u as i64, i, i.into(), i);

floating_point_traits!(f32, u32, 8, 23);
floating_point_traits!(f64, u64, 11, 52);

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

    fn lt_abs(&self, other: &Rhs) -> bool {
        match self.partial_cmp_abs(other) {
            Some(Ordering::Less) => true,
            _ => false,
        }
    }

    fn le_abs(&self, other: &Rhs) -> bool {
        match self.partial_cmp_abs(other) {
            Some(Ordering::Less) | Some(Ordering::Equal) => true,
            _ => false,
        }
    }

    fn gt_abs(&self, other: &Rhs) -> bool {
        match self.partial_cmp_abs(other) {
            Some(Ordering::Greater) => true,
            _ => false,
        }
    }

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
pub trait Zero {
    const ZERO: Self;
}

/// Provides the constant 1.
pub trait One {
    const ONE: Self;
}

/// Provides the constant 2.
pub trait Two {
    const TWO: Self;
}

/// Provides the constant -1.
pub trait NegativeOne {
    const NEGATIVE_ONE: Self;
}

/// Implements the constants 0, 1, and 2 for unsigned primitive integers.
macro_rules! impl01_unsigned {
    ($t: ty) => {
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
    }
}

/// Implements the constants 0, 1, 2, and -1 for signed primitive integers.
macro_rules! impl01_signed {
    ($t: ty) => {
        impl01_unsigned!($t);

        /// The constant -1 for signed primitive integers.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl NegativeOne for $t {
            const NEGATIVE_ONE: $t = -1;
        }
    }
}

/// Implements the constants 0, 1, 2, and -1 for primitive floating-point types.
macro_rules! impl01_float {
    ($t: ty) => {
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
    }
}

impl01_unsigned!(u8);
impl01_unsigned!(u16);
impl01_unsigned!(u32);
impl01_unsigned!(u64);
impl01_unsigned!(usize);

impl01_signed!(i8);
impl01_signed!(i16);
impl01_signed!(i32);
impl01_signed!(i64);
impl01_signed!(isize);

impl01_float!(f32);
impl01_float!(f64);

/// Provides a function to get the number of significant bits of `self`.
pub trait SignificantBits {
    /// The number of bits it takes to represent `self`. This is useful when benchmarking functions;
    /// the functions' inputs can be bucketed based on their number of significant bits.
    fn significant_bits(self) -> u64;
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
    fn split_in_half(&self) -> (Self::Half, Self::Half) {
        (self.upper_half(), self.lower_half())
    }
}

/// Implements `JoinHalves` and `SplitInHalf` for unsigned primitive integers.
macro_rules! impl_halves_unsigned {
    ($t: ident, $ht: ident) => {
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
            fn upper_half(&self) -> Self::Half {
                (self >> $ht::WIDTH) as $ht
            }
        }
    }
}

impl_halves_unsigned!(u16, u8);
impl_halves_unsigned!(u32, u16);
impl_halves_unsigned!(u64, u32);

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
