use std::cmp::Ordering;

use round::RoundingMode;

/// Checks whether `self` is reduced mod 2<sup>`pow`</sup>.
pub trait ModPowerOfTwoIsReduced {
    fn mod_power_of_two_is_reduced(&self, pow: u64) -> bool;
}

/// Checks whether `self` is reduced mod `m`.
pub trait ModIsReduced<M = Self> {
    fn mod_is_reduced(&self, m: &M) -> bool;
}

/// Returns `Greater`, `Equal`, or `Less`, depending on whether `self` is positive, zero, or
/// negative, respectively.
pub trait Sign {
    fn sign(&self) -> Ordering;
}

/// Replaces `self` with its negative. Assumes that `self` can be negated.
pub trait NegAssign {
    fn neg_assign(&mut self);
}

/// Checked negation. Computes `-self`, returning `None` if there is no valid result.
pub trait CheckedNeg {
    type Output;

    fn checked_neg(self) -> Option<Self::Output>;
}

/// Checked negation. Computes `-self`, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingNeg {
    type Output;

    fn saturating_neg(self) -> Self::Output;
}

/// Checked negation. Replaces `self` with its negative, saturating at the numeric bounds instead of
/// overflowing.
pub trait SaturatingNegAssign {
    fn saturating_neg_assign(&mut self);
}

/// Wrapping (modular) negation. Computes `-self`, wrapping around at the boundary of the type.
pub trait WrappingNeg {
    type Output;

    fn wrapping_neg(self) -> Self::Output;
}

/// Wrapping (modular) negation. Replaces `self` with its negative, wrapping around at the boundary
/// of the type.
pub trait WrappingNegAssign {
    fn wrapping_neg_assign(&mut self);
}

/// Calculates -`self`.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingNeg {
    type Output;

    fn overflowing_neg(self) -> (Self::Output, bool);
}

/// Replaces `self` with its negative.
///
/// Returns a boolean indicating whether an arithmetic overflow would occur. If an overflow would
/// have occurred then the wrapped value is assigned.
pub trait OverflowingNegAssign {
    fn overflowing_neg_assign(&mut self) -> bool;
}

/// Computes the absolute value of `self`. Inputs are assumed to be valid.
pub trait Abs {
    type Output;

    fn abs(self) -> Self::Output;
}

/// Replaces `self` with its absolute value. Assumes that `self` has a representable absolute value.
pub trait AbsAssign {
    fn abs_assign(&mut self);
}

/// Computes the absolute value of `self` and converts to the unsigned equivalent.
pub trait UnsignedAbs {
    type Output;

    fn unsigned_abs(self) -> Self::Output;
}

/// Checked absolute value. Computes `self.abs()`, returning `None` if there is no valid result.
pub trait CheckedAbs {
    type Output;

    fn checked_abs(self) -> Option<Self::Output>;
}

/// Checked absolute value. Computes `self.abs()`, saturating at the numeric bounds instead of
/// overflowing.
pub trait SaturatingAbs {
    type Output;

    fn saturating_abs(self) -> Self::Output;
}

/// Checked absolute value. Replaces `self` with its absolute value, saturating at the numeric
/// bounds instead of overflowing.
pub trait SaturatingAbsAssign {
    fn saturating_abs_assign(&mut self);
}

/// Wrapping (modular) absolute value. Computes `self.abs()`, wrapping around at the boundary of the
/// type.
pub trait WrappingAbs {
    type Output;

    fn wrapping_abs(self) -> Self::Output;
}

/// Wrapping (modular) absolute value. Replaces `self` with its absolute value, wrapping around at
/// the boundary of the type.
pub trait WrappingAbsAssign {
    fn wrapping_abs_assign(&mut self);
}

/// Calculates `self.abs()`.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingAbs {
    type Output;

    fn overflowing_abs(self) -> (Self::Output, bool);
}

/// Replaces `self` with its absolute value.
///
/// Returns a boolean indicating whether an arithmetic overflow would occur. If an overflow would
/// have occurred then the wrapped value is assigned.
pub trait OverflowingAbsAssign {
    fn overflowing_abs_assign(&mut self) -> bool;
}

/// Computes `-self` mod 2<sup>`pow`</sup>. Assumes the input is already reduced mod
/// 2<sup>`pow`</sup>.
pub trait ModPowerOfTwoNeg {
    type Output;

    fn mod_power_of_two_neg(self, pow: u64) -> Self::Output;
}

/// Replaces `self` with `-self` mod 2<sup>`pow`</sup>. Assumes the input is already reduced mod
/// 2<sup>`pow`</sup>.
pub trait ModPowerOfTwoNegAssign {
    fn mod_power_of_two_neg_assign(&mut self, pow: u64);
}

/// Computes `-self` mod `m`. Assumes the input is already reduced mod `m`.
pub trait ModNeg<M = Self> {
    type Output;

    fn mod_neg(self, m: M) -> Self::Output;
}

/// Replaces `self` with `-self` mod `m`. Assumes the input is already reduced mod `m`.
pub trait ModNegAssign<M = Self> {
    fn mod_neg_assign(&mut self, m: M);
}

/// Checked addition. Computes `self + other`, returning `None` if there is no valid result.
pub trait CheckedAdd<RHS = Self> {
    type Output;

    fn checked_add(self, other: RHS) -> Option<Self::Output>;
}

/// Saturating addition. Computes `self + other`, saturating at the numeric bounds instead of
/// overflowing.
pub trait SaturatingAdd<RHS = Self> {
    type Output;

    fn saturating_add(self, other: RHS) -> Self::Output;
}

/// Saturating addition. Replaces `self` with `self + other`, saturating at the numeric bounds
/// instead of overflowing.
pub trait SaturatingAddAssign<RHS = Self> {
    fn saturating_add_assign(&mut self, other: RHS);
}

/// Wrapping (modular) addition. Computes `self + other`, wrapping around at the boundary of the
/// type.
pub trait WrappingAdd<RHS = Self> {
    type Output;

    fn wrapping_add(self, other: RHS) -> Self::Output;
}

/// Wrapping (modular) addition. Replaces `self` with `self + other`, wrapping around at the
/// boundary of the type.
pub trait WrappingAddAssign<RHS = Self> {
    fn wrapping_add_assign(&mut self, other: RHS);
}

/// Calculates `self` + `other`.
///
/// Returns a tuple of the sum along with a boolean indicating whether an arithmetic overflow would
/// occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingAdd<RHS = Self> {
    type Output;

    fn overflowing_add(self, other: RHS) -> (Self::Output, bool);
}

/// Replaces `self` with `self` + `other`.
///
/// Returns a boolean indicating whether an arithmetic overflow would occur. If an overflow would
/// have occurred then the wrapped value is assigned.
pub trait OverflowingAddAssign<RHS = Self> {
    fn overflowing_add_assign(&mut self, other: RHS) -> bool;
}

/// Adds two numbers, each composed of two `Self` values. The sum is returned as a pair of `Self`
/// values. The more significant value always comes first. Addition is wrapping, and overflow is not
/// indicated.
pub trait XXAddYYIsZZ: Sized {
    fn xx_add_yy_is_zz(x_1: Self, x_0: Self, y_1: Self, y_0: Self) -> (Self, Self);
}

/// Adds two numbers, each composed of three `Self` values. The sum is returned as a triple of
/// `Self` values. The more significant value always comes first. Addition is wrapping, and overflow
/// is not indicated.
pub trait XXXAddYYYIsZZZ: Sized {
    fn xxx_add_yyy_is_zzz(
        x_2: Self,
        x_1: Self,
        x_0: Self,
        y_2: Self,
        y_1: Self,
        y_0: Self,
    ) -> (Self, Self, Self);
}

/// Adds two numbers, each composed of four `Self` values. The sum is returned as a quadruple of
/// `Self` values. The more significant value always comes first. Addition is wrapping, and overflow
/// is not indicated.
pub trait XXXXAddYYYYIsZZZZ: Sized {
    #[allow(clippy::too_many_arguments)]
    fn xxxx_add_yyyy_is_zzzz(
        x_3: Self,
        x_2: Self,
        x_1: Self,
        x_0: Self,
        y_3: Self,
        y_2: Self,
        y_1: Self,
        y_0: Self,
    ) -> (Self, Self, Self, Self);
}

/// Checked subtraction. Computes `self - other`, returning `None` if there is no valid result.
pub trait CheckedSub<RHS = Self> {
    type Output;

    fn checked_sub(self, other: RHS) -> Option<Self::Output>;
}

/// Saturating subtraction. Computes `self - other`, saturating at the numeric bounds instead of
/// overflowing.
pub trait SaturatingSub<RHS = Self> {
    type Output;

    fn saturating_sub(self, other: RHS) -> Self::Output;
}

/// Saturating subtraction. Replaces `self` with `self - other`, saturating at the numeric bounds
/// instead of overflowing.
pub trait SaturatingSubAssign<RHS = Self> {
    fn saturating_sub_assign(&mut self, other: RHS);
}

/// Wrapping (modular) subtraction. Computes `self - other`, wrapping around at the boundary of the
/// type.
pub trait WrappingSub<RHS = Self> {
    type Output;

    fn wrapping_sub(self, other: RHS) -> Self::Output;
}

/// Wrapping (modular) subtraction. Replaces `self` with `self - other`, wrapping around at the
/// boundary of the type.
pub trait WrappingSubAssign<RHS = Self> {
    fn wrapping_sub_assign(&mut self, other: RHS);
}

/// Calculates `self` - `other`.
///
/// Returns a tuple of the difference along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingSub<RHS = Self> {
    type Output;

    fn overflowing_sub(self, other: RHS) -> (Self::Output, bool);
}

/// Replaces `self` with `self` - `other`.
///
/// Returns a boolean indicating whether an arithmetic overflow would occur. If an overflow would
/// have occurred then the wrapped value is assigned.
pub trait OverflowingSubAssign<RHS = Self> {
    fn overflowing_sub_assign(&mut self, other: RHS) -> bool;
}

/// Subtracts two numbers, each composed of two `Self` values. The difference is returned as a pair
/// of `Self` values. The more significant value always comes first. Subtraction is wrapping, and
/// overflow is not indicated.
pub trait XXSubYYIsZZ: Sized {
    fn xx_sub_yy_is_zz(x_1: Self, x_0: Self, y_1: Self, y_0: Self) -> (Self, Self);
}

/// Subtracts two numbers, each composed of three `Self` values. The difference is returned as a
/// triple of `Self` values. The more significant value always comes first. Subtraction is wrapping,
/// and overflow is not indicated.
pub trait XXXSubYYYIsZZZ: Sized {
    fn xxx_sub_yyy_is_zzz(
        x_2: Self,
        x_1: Self,
        x_0: Self,
        y_2: Self,
        y_1: Self,
        y_0: Self,
    ) -> (Self, Self, Self);
}

/// Computes `self + other` mod 2<sup>`pow`</sup>. Assumes the inputs are already reduced mod
/// 2<sup>`pow`</sup>.
pub trait ModPowerOfTwoAdd<RHS = Self> {
    type Output;

    fn mod_power_of_two_add(self, other: RHS, pow: u64) -> Self::Output;
}

/// Replaces `self` with `self + other` mod 2<sup>`pow`</sup>. Assumes the inputs are already
/// reduced mod 2<sup>`pow`</sup>.
pub trait ModPowerOfTwoAddAssign<RHS = Self> {
    fn mod_power_of_two_add_assign(&mut self, other: RHS, pow: u64);
}

/// Computes `self + other` mod `m`. Assumes the inputs are already reduced mod `m`.
pub trait ModAdd<RHS = Self, M = Self> {
    type Output;

    fn mod_add(self, other: RHS, m: M) -> Self::Output;
}

/// Replaces `self` with `self + other` mod `m`. Assumes the inputs are already reduced mod `m`.
pub trait ModAddAssign<RHS = Self, M = Self> {
    fn mod_add_assign(&mut self, other: RHS, m: M);
}

/// Computes `self - other` mod 2<sup>`pow`</sup>. Assumes the inputs are already reduced mod
/// 2<sup>`pow`</sup>.
pub trait ModPowerOfTwoSub<RHS = Self> {
    type Output;

    fn mod_power_of_two_sub(self, other: RHS, pow: u64) -> Self::Output;
}

/// Replaces `self` with `self - other` mod 2<sup>`pow`</sup>. Assumes the inputs are already
/// reduced mod 2<sup>`pow`</sup>.
pub trait ModPowerOfTwoSubAssign<RHS = Self> {
    fn mod_power_of_two_sub_assign(&mut self, other: RHS, pow: u64);
}

/// Computes `self - other` mod `m`. Assumes the inputs are already reduced mod `m`.
pub trait ModSub<RHS = Self, M = Self> {
    type Output;

    fn mod_sub(self, other: RHS, m: M) -> Self::Output;
}

/// Replaces `self` with `self - other` mod `m`. Assumes the inputs are already reduced mod `m`.
pub trait ModSubAssign<RHS = Self, M = Self> {
    fn mod_sub_assign(&mut self, other: RHS, m: M);
}

/// Checked multiplication. Computes `self * other`, returning `None` if there is no valid result.
pub trait CheckedMul<RHS = Self> {
    type Output;

    fn checked_mul(self, other: RHS) -> Option<Self::Output>;
}

/// Saturating multiplication. Computes `self * other`, saturating at the numeric bounds instead of
/// overflowing.
pub trait SaturatingMul<RHS = Self> {
    type Output;

    fn saturating_mul(self, other: RHS) -> Self::Output;
}

/// Saturating multiplication. Replaces `self` with `self * other`, saturating at the numeric bounds
/// instead of overflowing.
pub trait SaturatingMulAssign<RHS = Self> {
    fn saturating_mul_assign(&mut self, other: RHS);
}

/// Wrapping (modular) multiplication. Computes `self * other`, wrapping around at the boundary of
/// the type.
pub trait WrappingMul<RHS = Self> {
    type Output;

    fn wrapping_mul(self, other: RHS) -> Self::Output;
}

/// Wrapping (modular) multiplication. Replaces `self` with `self * other`, wrapping around at the
/// boundary of the type.
pub trait WrappingMulAssign<RHS = Self> {
    fn wrapping_mul_assign(&mut self, other: RHS);
}

/// Calculates `self` * `other`.
///
/// Returns a tuple of the product along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingMul<RHS = Self> {
    type Output;

    fn overflowing_mul(self, other: RHS) -> (Self::Output, bool);
}

/// Replaces `self` with `self` * `other`.
///
/// Returns a boolean indicating whether an arithmetic overflow would occur. If an overflow would
/// have occurred then the wrapped value is assigned.
pub trait OverflowingMulAssign<RHS = Self> {
    fn overflowing_mul_assign(&mut self, other: RHS) -> bool;
}

/// Multiplies two numbers, returning the product as a pair of `Self` values. The more significant
/// value always comes first.
pub trait XMulYIsZZ: Sized {
    fn x_mul_y_is_zz(x: Self, y: Self) -> (Self, Self);
}

/// Computes `self * other` mod 2<sup>`pow`</sup>. Assumes the inputs are already reduced mod
/// 2<sup>`pow`</sup>.
pub trait ModPowerOfTwoMul<RHS = Self> {
    type Output;

    fn mod_power_of_two_mul(self, other: RHS, pow: u64) -> Self::Output;
}

/// Replaces `self` with `self * other` mod 2<sup>`pow`</sup>. Assumes the inputs are already
/// reduced mod 2<sup>`pow`</sup>.
pub trait ModPowerOfTwoMulAssign<RHS = Self> {
    fn mod_power_of_two_mul_assign(&mut self, other: RHS, pow: u64);
}

/// Computes `self * other` mod `m`. Assumes the inputs are already reduced mod `m`.
pub trait ModMul<RHS = Self, M = Self> {
    type Output;

    fn mod_mul(self, other: RHS, m: M) -> Self::Output;
}

/// Replaces `self` with `self * other` mod `m`. Assumes the inputs are already reduced mod `m`.
pub trait ModMulAssign<RHS = Self, M = Self> {
    fn mod_mul_assign(&mut self, other: RHS, m: M);
}

/// Computes `self * other` mod `m`. Assumes the inputs are already reduced mod `m`. If multiple
/// modular multiplications with the same modulus are necessary, it can be quicker to precompute
/// some piece of data and reuse it in the multiplication calls. This trait provides a method for
/// precomputing the data and a method for using it during multiplication.
pub trait ModMulPrecomputed<RHS = Self, M = Self> {
    type Output;
    type Data;

    fn precompute_mod_mul_data(m: &M) -> Self::Data;

    fn mod_mul_precomputed(self, other: RHS, m: M, data: &Self::Data) -> Self::Output;
}

/// Replaces `self` with `self * other` mod `m`. Assumes the inputs are already reduced mod `m`. If
/// multiple modular multiplications with the same modulus are necessary, it can be quicker to
/// precompute some piece of data and reuse it in the multiplication calls. This trait provides a
/// method for using precomputed data during multiplication. For precomputing the data, use the
/// `precompute_mod_mul_data` function in `ModMulPrecomputed`.
pub trait ModMulPrecomputedAssign<RHS = Self, M = Self>: ModMulPrecomputed<RHS, M> {
    fn mod_mul_precomputed_assign(&mut self, other: RHS, m: M, data: &Self::Data);
}

/// Computes `self + y * z`.
pub trait AddMul<Y = Self, Z = Self> {
    type Output;

    fn add_mul(self, y: Y, z: Z) -> Self::Output;
}

/// Replaces `self` with `self + y * z`.
pub trait AddMulAssign<Y = Self, Z = Self> {
    fn add_mul_assign(&mut self, y: Y, z: Z);
}

/// Computes `self + y * z`, returning `None` if there is no valid result.
pub trait CheckedAddMul<Y = Self, Z = Self> {
    type Output;

    fn checked_add_mul(self, y: Y, z: Z) -> Option<Self::Output>;
}

/// Computes `self + y * z`, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingAddMul<Y = Self, Z = Self> {
    type Output;

    fn saturating_add_mul(self, y: Y, z: Z) -> Self::Output;
}

/// Replaces `self` with `self + y * z`, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingAddMulAssign<Y = Self, Z = Self> {
    fn saturating_add_mul_assign(&mut self, y: Y, z: Z);
}

/// Computes `self + y * z`, wrapping around at the boundary of the type.
pub trait WrappingAddMul<Y = Self, Z = Self> {
    type Output;

    fn wrapping_add_mul(self, y: Y, z: Z) -> Self::Output;
}

/// Replaces `self` with `self + y * z`, wrapping around at the boundary of the type.
pub trait WrappingAddMulAssign<Y = Self, Z = Self> {
    fn wrapping_add_mul_assign(&mut self, y: Y, z: Z);
}

/// Calculates `self + y * z`.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingAddMul<Y = Self, Z = Self> {
    type Output;

    fn overflowing_add_mul(self, y: Y, z: Z) -> (Self::Output, bool);
}

/// Replaces `self` with `self + y * z`.
///
/// Returns a boolean indicating whether an arithmetic overflow would occur. If an overflow would
/// have occurred then the wrapped value is assigned.
pub trait OverflowingAddMulAssign<Y = Self, Z = Self> {
    fn overflowing_add_mul_assign(&mut self, y: Y, z: Z) -> bool;
}

/// Computes `self - y * z`.
pub trait SubMul<Y = Self, Z = Self> {
    type Output;

    fn sub_mul(self, y: Y, z: Z) -> Self::Output;
}

/// Replaces `self` with `self - y * z`.
pub trait SubMulAssign<Y = Self, Z = Self> {
    fn sub_mul_assign(&mut self, y: Y, z: Z);
}

/// Computes `self - y * z`, returning `None` if there is no valid result.
pub trait CheckedSubMul<Y = Self, Z = Self> {
    type Output;

    fn checked_sub_mul(self, y: Y, z: Z) -> Option<Self::Output>;
}

/// Computes `self - y * z`, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingSubMul<Y = Self, Z = Self> {
    type Output;

    fn saturating_sub_mul(self, y: Y, z: Z) -> Self::Output;
}

/// Replaces `self` with `self - y * z`, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingSubMulAssign<Y = Self, Z = Self> {
    fn saturating_sub_mul_assign(&mut self, y: Y, z: Z);
}

/// Computes `self - y * z`, wrapping around at the boundary of the type.
pub trait WrappingSubMul<Y = Self, Z = Self> {
    type Output;

    fn wrapping_sub_mul(self, y: Y, z: Z) -> Self::Output;
}

/// Replaces `self` with `self - y * z`, wrapping around at the boundary of the type.
pub trait WrappingSubMulAssign<Y = Self, Z = Self> {
    fn wrapping_sub_mul_assign(&mut self, y: Y, z: Z);
}

/// Calculates `self - y * z`.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingSubMul<Y = Self, Z = Self> {
    type Output;

    fn overflowing_sub_mul(self, y: Y, z: Z) -> (Self::Output, bool);
}

/// Replaces `self` with `self - y * z`.
///
/// Returns a boolean indicating whether an arithmetic overflow would occur. If an overflow would
/// have occurred then the wrapped value is assigned.
pub trait OverflowingSubMulAssign<Y = Self, Z = Self> {
    fn overflowing_sub_mul_assign(&mut self, y: Y, z: Z) -> bool;
}

/// Calculates `self` * 2<sup>`other`</sup>, rounding the result according to a specified rounding
/// mode. Rounding might only be necessary if `other` is negative.
pub trait ShlRound<RHS> {
    type Output;

    fn shl_round(self, other: RHS, rm: RoundingMode) -> Self::Output;
}

/// Replaces `self` with `self` * 2<sup>`other`</sup>, rounding the result according to a specified
/// rounding mode. Rounding might only be necessary if `other` is negative.
pub trait ShlRoundAssign<RHS> {
    fn shl_round_assign(&mut self, other: RHS, rm: RoundingMode);
}

/// Calculates `self` / 2<sup>`other`</sup>, rounding the result according to a specified rounding
/// mode. Rounding might only be necessary if `other` is non-negative.
pub trait ShrRound<RHS> {
    type Output;

    fn shr_round(self, other: RHS, rm: RoundingMode) -> Self::Output;
}

/// Replaces `self` with `self` / 2<sup>`other`</sup>, rounding the result according to a specified
/// rounding mode. Rounding might only be necessary if `other` is non-negative.
pub trait ShrRoundAssign<RHS> {
    fn shr_round_assign(&mut self, other: RHS, rm: RoundingMode);
}

/// Computes `self << other`, returning `None` if the result is too large to fit.
pub trait ArithmeticCheckedShl<RHS> {
    type Output;

    fn arithmetic_checked_shl(self, other: RHS) -> Option<Self::Output>;
}

/// Computes `self >> other`, returning `None` if the result is too large to fit.
pub trait ArithmeticCheckedShr<RHS> {
    type Output;

    fn arithmetic_checked_shr(self, other: RHS) -> Option<Self::Output>;
}

/// Computes `self << other` mod 2<sup>`pow`</sup>. Assumes the inputs are already reduced mod
/// 2<sup>`pow`</sup>.
pub trait ModPowerOfTwoShl<RHS> {
    type Output;

    fn mod_power_of_two_shl(self, other: RHS, pow: u64) -> Self::Output;
}

/// Replaces `self` with `self << other` mod 2<sup>`pow`</sup>. Assumes the inputs are already
/// reduced mod 2<sup>`pow`</sup>.
pub trait ModPowerOfTwoShlAssign<RHS> {
    fn mod_power_of_two_shl_assign(&mut self, other: RHS, pow: u64);
}

/// Computes `self >> other` mod 2<sup>`pow`</sup>. Assumes the inputs are already reduced mod
/// 2<sup>`pow`</sup>.
pub trait ModPowerOfTwoShr<RHS> {
    type Output;

    fn mod_power_of_two_shr(self, other: RHS, pow: u64) -> Self::Output;
}

/// Replaces `self` with `self >> other` mod 2<sup>`pow`</sup>. Assumes the inputs are already
/// reduced mod 2<sup>`pow`</sup>.
pub trait ModPowerOfTwoShrAssign<RHS> {
    fn mod_power_of_two_shr_assign(&mut self, other: RHS, pow: u64);
}

/// Rounds `self` to a multiple of a power of 2, according to a specified rounding mode.
pub trait RoundToMultipleOfPowerOfTwo<RHS> {
    type Output;

    fn round_to_multiple_of_power_of_two(self, pow: RHS, rm: RoundingMode) -> Self::Output;
}

/// Rounds `self` to a multiple of a power of 2 in place, according to a specified rounding mode.
pub trait RoundToMultipleOfPowerOfTwoAssign<RHS> {
    fn round_to_multiple_of_power_of_two_assign(&mut self, pow: RHS, rm: RoundingMode);
}

/// Computes the quotient and remainder of two numbers. The first is composed of two `Self` values,
/// and the second of a single one. `x_0` must be less than `y`.
pub trait XXDivModYIsQR: Sized {
    fn xx_div_mod_y_is_qr(x_1: Self, x_0: Self, y: Self) -> (Self, Self);
}

/// Checked division. Computes `self / other`, returning `None` if there is no valid result.
pub trait CheckedDiv<RHS = Self> {
    type Output;

    fn checked_div(self, other: RHS) -> Option<Self::Output>;
}

/// Wrapping (modular) division. Computes `self / other`, wrapping around at the boundary of the
/// type.
pub trait WrappingDiv<RHS = Self> {
    type Output;

    fn wrapping_div(self, other: RHS) -> Self::Output;
}

/// Wrapping (modular) division. Replaces `self` with `self / other`, wrapping around at the
/// boundary of the type.
pub trait WrappingDivAssign<RHS = Self> {
    fn wrapping_div_assign(&mut self, other: RHS);
}

/// Calculates `self` / `other`.
///
/// Returns a tuple of the quotient along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingDiv<RHS = Self> {
    type Output;

    fn overflowing_div(self, other: RHS) -> (Self::Output, bool);
}

/// Replaces `self` with `self` / `other`.
///
/// Returns a tuple of the quotient along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingDivAssign<RHS = Self> {
    fn overflowing_div_assign(&mut self, other: RHS) -> bool;
}

/// Calculates `self` mod a power of 2. In other words, returns r, where
/// `self` = q * 2<sup>`other`</sup> + r and 0 <= r < 2<sup>`other`</sup>.
pub trait ModPowerOfTwo {
    type Output;

    fn mod_power_of_two(self, other: u64) -> Self::Output;
}

/// Reduces `self` mod a power of 2. In other words, replaces `self` with r, where
/// `self` = q * 2<sup>`other`</sup> + r and 0 <= r < 2<sup>`other`</sup>.
pub trait ModPowerOfTwoAssign {
    fn mod_power_of_two_assign(&mut self, other: u64);
}

/// Calculates `self` rem a power of 2. In other words, returns r, where
/// `self` = q * 2<sup>`other`</sup> + r, r == 0 or (sgn(r) == sgn(`self`)), and
/// 0 <= |r| < 2<sup>`other`</sup>.
pub trait RemPowerOfTwo {
    type Output;

    fn rem_power_of_two(self, other: u64) -> Self::Output;
}

/// Reduces `self` rem a power of 2. In other words, replaces `self` with r, where
/// `self` = q * 2<sup>`other`</sup> + r, r == 0 or (sgn(r) == sgn(`self`)), and
/// 0 <= |r| < 2<sup>`other`</sup>.
pub trait RemPowerOfTwoAssign {
    fn rem_power_of_two_assign(&mut self, other: u64);
}

/// Calculates `-self` mod a power of 2. In other words, returns r, where
/// `self` = q * 2<sup>`other`</sup> - r and 0 <= r < 2<sup>`other`</sup>.
pub trait NegModPowerOfTwo {
    type Output;

    fn neg_mod_power_of_two(self, other: u64) -> Self::Output;
}

/// Reduces `-self` mod a power of 2. In other words, replaces `self` with r, where
/// `self` = q * 2<sup>`other`</sup> - r and 0 <= r < 2<sup>`other`</sup>.
pub trait NegModPowerOfTwoAssign {
    fn neg_mod_power_of_two_assign(&mut self, other: u64);
}

/// Calculates `self` ceiling-mod a power of 2. In other words, returns r, where
/// `self` = q * 2<sup>`other`</sup> + r and 0 <= -r < 2<sup>`other`</sup>.
pub trait CeilingModPowerOfTwo {
    type Output;

    fn ceiling_mod_power_of_two(self, other: u64) -> Self::Output;
}

/// Reduces `self` ceiling-mod a power of 2. In other words, replaces `self` with r, where
/// `self` = q * 2<sup>`other`</sup> + r and 0 <= -r < 2<sup>`other`</sup>.
pub trait CeilingModPowerOfTwoAssign {
    fn ceiling_mod_power_of_two_assign(&mut self, other: u64);
}

/// Determines whether `self` is even or odd.
pub trait Parity {
    /// Returns whether `self` is even.
    fn even(self) -> bool;

    /// Returns whether `self` is odd.
    fn odd(self) -> bool;
}

/// Determines whether `self` is divisible by 2<sup>pow</sup>.
pub trait DivisibleByPowerOfTwo {
    fn divisible_by_power_of_two(self, pow: u64) -> bool;
}

/// Determines whether `self` is equal to other mod 2<sup>pow</sup>.
pub trait EqModPowerOfTwo<RHS = Self> {
    fn eq_mod_power_of_two(self, other: RHS, pow: u64) -> bool;
}

/// Divides a value by another value, returning the quotient and remainder. The quotient is rounded
/// towards negative infinity, and the remainder has the same sign as the divisor. The quotient and
/// remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
pub trait DivMod<RHS = Self> {
    type DivOutput;
    type ModOutput;

    fn div_mod(self, other: RHS) -> (Self::DivOutput, Self::ModOutput);
}

/// Divides a value by another value in place, returning the remainder. The quotient is rounded
/// towards negative infinity, and the remainder has the same sign as the divisor. The quotient and
/// remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
pub trait DivAssignMod<RHS = Self> {
    type ModOutput;

    fn div_assign_mod(&mut self, other: RHS) -> Self::ModOutput;
}

/// Divides a value by another value, returning the quotient and remainder. The quotient is rounded
/// towards zero and the remainder has the same sign as the dividend. The quotient and remainder
/// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
pub trait DivRem<RHS = Self> {
    type DivOutput;
    type RemOutput;

    fn div_rem(self, other: RHS) -> (Self::DivOutput, Self::RemOutput);
}

/// Divides a value by another value in place, returning the remainder. The quotient is rounded
/// towards zero and the remainder has the same sign as the dividend. The quotient and remainder
/// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
pub trait DivAssignRem<RHS = Self> {
    type RemOutput;

    fn div_assign_rem(&mut self, other: RHS) -> Self::RemOutput;
}

/// Divides a value by another value, returning the ceiling of the quotient and the remainder of the
/// negative of the first value divided by the second. The quotient and remainder satisfy
/// `self` = q * `other` - r and 0 <= r < `other`.
pub trait CeilingDivNegMod<RHS = Self> {
    type DivOutput;
    type ModOutput;

    fn ceiling_div_neg_mod(self, other: RHS) -> (Self::DivOutput, Self::ModOutput);
}

/// Divides a value by another value in place, taking the ceiling of the quotient and returning the
/// remainder of the negative of the first value divided by the second. The quotient and remainder
/// satisfy `self` = q * `other` - r and 0 <= r < `other`.
pub trait CeilingDivAssignNegMod<RHS = Self> {
    type ModOutput;

    fn ceiling_div_assign_neg_mod(&mut self, other: RHS) -> Self::ModOutput;
}

/// Divides a value by another value, returning the quotient and remainder. The quotient is rounded
/// towards positive infinity and the remainder has the opposite sign of the divisor. The quotient
/// and remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
pub trait CeilingDivMod<RHS = Self> {
    type DivOutput;
    type ModOutput;

    fn ceiling_div_mod(self, other: RHS) -> (Self::DivOutput, Self::ModOutput);
}

/// Divides a value by another value in place, taking the quotient and returning the remainder. The
/// quotient is rounded towards positive infinity and the remainder has the opposite sign of the
/// divisor. The quotient and remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
pub trait CeilingDivAssignMod<RHS = Self> {
    type ModOutput;

    fn ceiling_div_assign_mod(&mut self, other: RHS) -> Self::ModOutput;
}

/// Divides a value by another value, returning the quotient and remainder. The quotient is rounded
/// towards negative infinity, and the remainder has the same sign as the divisor. The quotient and
/// remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
pub trait Mod<RHS = Self> {
    type Output;

    fn mod_op(self, other: RHS) -> Self::Output;
}

/// Divides a value by another value in place, returning the remainder. The quotient is rounded
/// towards negative infinity, and the remainder has the same sign as the divisor. The quotient and
/// remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
pub trait ModAssign<RHS = Self> {
    fn mod_assign(&mut self, other: RHS);
}

/// Divides a value by another value, returning the ceiling of the quotient and the remainder of the
/// negative of the first value divided by the second. The quotient and remainder satisfy
/// `self` = q * `other` - r and 0 <= r < `other`.
pub trait NegMod<RHS = Self> {
    type Output;

    fn neg_mod(self, other: RHS) -> Self::Output;
}

/// Divides a value by another value in place, taking the ceiling of the quotient and returning the
/// remainder of the negative of the first value divided by the second. The quotient and remainder
/// satisfy `self` = q * `other` - r and 0 <= r < `other`.
pub trait NegModAssign<RHS = Self> {
    fn neg_mod_assign(&mut self, other: RHS);
}

/// Divides a value by another value, returning the quotient and remainder. The quotient is rounded
/// towards positive infinity and the remainder has the opposite sign of the divisor. The quotient
/// and remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
pub trait CeilingMod<RHS = Self> {
    type Output;

    fn ceiling_mod(self, other: RHS) -> Self::Output;
}

/// Divides a value by another value in place, taking the quotient and returning the remainder. The
/// quotient is rounded towards positive infinity and the remainder has the opposite sign of the
/// divisor. The quotient and remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
pub trait CeilingModAssign<RHS = Self> {
    fn ceiling_mod_assign(&mut self, other: RHS);
}

/// Divides a value by another value and rounds according to a specified rounding mode. See the
/// `RoundingMode` documentation for details.
pub trait DivRound<RHS = Self> {
    type Output;

    fn div_round(self, other: RHS, rm: RoundingMode) -> Self::Output;
}

/// Divides a value by another value in place and rounds according to a specified rounding mode. See
/// the `RoundingMode` documentation for details.
pub trait DivRoundAssign<RHS = Self> {
    fn div_round_assign(&mut self, other: RHS, rm: RoundingMode);
}

/// Raises `self` to the power of `exp`, returning `None` if there is no valid result.
pub trait CheckedPow<RHS> {
    type Output;

    fn checked_pow(self, exp: RHS) -> Option<Self::Output>;
}

/// Raises `self` to the power of `exp`, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingPow<RHS> {
    type Output;

    fn saturating_pow(self, exp: RHS) -> Self::Output;
}

/// Wrapping (modular) exponentiation. Raises `self` to the power of `exp`, wrapping around at the
/// boundary of the type.
pub trait WrappingPow<RHS> {
    type Output;

    fn wrapping_pow(self, exp: RHS) -> Self::Output;
}

/// Calculates `self.pow(exp)`.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingPow<RHS> {
    type Output;

    fn overflowing_pow(self, exp: RHS) -> (Self::Output, bool);
}

/// Raises `self` to the power of `exp`.
pub trait Pow<RHS> {
    type Output;

    fn pow(self, exp: RHS) -> Self::Output;
}

/// Returns `true` iff `self` == 2<pow>k</pow> for some integer k.
pub trait IsPowerOfTwo {
    fn is_power_of_two(&self) -> bool;
}

/// Returns the smallest power of two greater than or equal to `self`.
pub trait NextPowerOfTwo {
    type Output;

    fn next_power_of_two(self) -> Self::Output;
}

/// Returns the smallest power of two greater than or equal to `self`. If the next power of two is
/// greater than the type's maximum value, `None` is returned; otherwise the power of two is wrapped
/// in `Some`.
pub trait CheckedNextPowerOfTwo {
    type Output;

    fn checked_next_power_of_two(self) -> Option<Self::Output>;
}

pub trait NextPowerOfTwoAssign {
    fn next_power_of_two_assign(&mut self);
}

pub trait EqMod<RHS = Self, M = Self> {
    fn eq_mod(self, other: RHS, m: M) -> bool;
}

pub trait DivExact<RHS = Self> {
    type Output;

    fn div_exact(self, other: RHS) -> Self::Output;
}

pub trait DivExactAssign<RHS = Self> {
    fn div_exact_assign(&mut self, other: RHS);
}

pub trait DivisibleBy<RHS = Self> {
    fn divisible_by(self, other: RHS) -> bool;
}

pub trait CheckedLogTwo {
    fn checked_log_two(self) -> Option<u64>;
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

/// Calculates 2<sup>pow</sup>.
pub trait PowerOfTwo {
    fn power_of_two(pow: u64) -> Self;
}
