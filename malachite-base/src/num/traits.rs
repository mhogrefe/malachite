use std::cmp::Ordering;
use std::num::ParseIntError;

use round::RoundingMode;

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

pub trait CheckedSubMul<B, C> {
    type Output;

    fn checked_sub_mul(self, b: B, c: C) -> Option<Self::Output>;
}

pub trait SaturatingSubMulAssign<B, C> {
    fn saturating_sub_mul_assign(&mut self, b: B, c: C);
}

pub trait SaturatingSubMul<B, C> {
    type Output;

    fn saturating_sub_mul(self, b: B, c: C) -> Self::Output;
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

//TODO doc and test
pub trait FromU32Slice: Sized {
    fn from_u32_slice(slice: &[u32]) -> Self;

    fn copy_from_u32_slice(out_slice: &mut [Self], in_slice: &[u32]);
}
