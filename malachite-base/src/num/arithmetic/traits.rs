use num::basic::traits::Two;
use rounding_modes::RoundingMode;
use std::cmp::Ordering;

/// Checks whether `self` is reduced mod $2^p$.
pub trait ModPowerOf2IsReduced {
    fn mod_power_of_2_is_reduced(&self, pow: u64) -> bool;
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

/// Computes the absolute value of `self`. The input is assumed to be valid.
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

/// Computes `-self` mod $2^p$. Assumes the input is already reduced mod $2^p$.
pub trait ModPowerOf2Neg {
    type Output;

    fn mod_power_of_2_neg(self, pow: u64) -> Self::Output;
}

/// Replaces `self` with `-self` mod $2^p$. Assumes the input is already reduced mod $2^p$.
pub trait ModPowerOf2NegAssign {
    fn mod_power_of_2_neg_assign(&mut self, pow: u64);
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

/// Computes `self + other` mod $2^p$. Assumes the inputs are already reduced mod $2^p$.
pub trait ModPowerOf2Add<RHS = Self> {
    type Output;

    fn mod_power_of_2_add(self, other: RHS, pow: u64) -> Self::Output;
}

/// Replaces `self` with `self + other` mod $2^p$. Assumes the inputs are already reduced mod $2^p$.
pub trait ModPowerOf2AddAssign<RHS = Self> {
    fn mod_power_of_2_add_assign(&mut self, other: RHS, pow: u64);
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

/// Computes `self - other` mod $2^p$. Assumes the inputs are already reduced mod $2^p$.
pub trait ModPowerOf2Sub<RHS = Self> {
    type Output;

    fn mod_power_of_2_sub(self, other: RHS, pow: u64) -> Self::Output;
}

/// Replaces `self` with `self - other` mod $2^p$. Assumes the inputs are already $2^p$.
pub trait ModPowerOf2SubAssign<RHS = Self> {
    fn mod_power_of_2_sub_assign(&mut self, other: RHS, pow: u64);
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

/// Computes `self * other` mod $2^p$. Assumes the inputs are already reduced mod $2^p$.
pub trait ModPowerOf2Mul<RHS = Self> {
    type Output;

    fn mod_power_of_2_mul(self, other: RHS, pow: u64) -> Self::Output;
}

/// Replaces `self` with `self * other` mod $2^p$. Assumes the inputs are already reduced mod $2^p$.
pub trait ModPowerOf2MulAssign<RHS = Self> {
    fn mod_power_of_2_mul_assign(&mut self, other: RHS, pow: u64);
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

/// Computes `self * other` mod `m`. Assumes the inputs are already reduced mod `m`.
///
/// If multiple modular multiplications with the same modulus are necessary, it can be quicker to
/// precompute some piece of data and reuse it in the multiplication calls. This trait provides a
/// function for precomputing the data and a function for using it during multiplication.
pub trait ModMulPrecomputed<RHS = Self, M = Self> {
    type Output;
    type Data;

    fn precompute_mod_mul_data(m: &M) -> Self::Data;

    fn mod_mul_precomputed(self, other: RHS, m: M, data: &Self::Data) -> Self::Output;
}

/// Replaces `self` with `self * other` mod `m`. Assumes the inputs are already reduced mod `m`.
///
/// If multiple modular multiplications with the same modulus are necessary, it can be quicker to
/// precompute some piece of data and reuse it in the multiplication calls. This trait provides a
/// function for using precomputed data during multiplication. For precomputing the data, use the
/// `precompute_mod_mul_data` function in `ModMulPrecomputed`.
pub trait ModMulPrecomputedAssign<RHS = Self, M = Self>: ModMulPrecomputed<RHS, M> {
    fn mod_mul_precomputed_assign(&mut self, other: RHS, m: M, data: &Self::Data);
}

/// Computes $x + yz$.
pub trait AddMul<Y = Self, Z = Self> {
    type Output;

    fn add_mul(self, y: Y, z: Z) -> Self::Output;
}

/// Replaces $x$ with $x + yz$.
pub trait AddMulAssign<Y = Self, Z = Self> {
    fn add_mul_assign(&mut self, y: Y, z: Z);
}

/// Computes $x + yz$, returning `None` if there is no valid result.
pub trait CheckedAddMul<Y = Self, Z = Self> {
    type Output;

    fn checked_add_mul(self, y: Y, z: Z) -> Option<Self::Output>;
}

/// Computes $x + yz$, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingAddMul<Y = Self, Z = Self> {
    type Output;

    fn saturating_add_mul(self, y: Y, z: Z) -> Self::Output;
}

/// Replaces $x$ with $x + yz$, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingAddMulAssign<Y = Self, Z = Self> {
    fn saturating_add_mul_assign(&mut self, y: Y, z: Z);
}

/// Computes $x + yz$, wrapping around at the boundary of the type.
pub trait WrappingAddMul<Y = Self, Z = Self> {
    type Output;

    fn wrapping_add_mul(self, y: Y, z: Z) -> Self::Output;
}

/// Replaces $x$ with $x + yz$, wrapping around at the boundary of the type.
pub trait WrappingAddMulAssign<Y = Self, Z = Self> {
    fn wrapping_add_mul_assign(&mut self, y: Y, z: Z);
}

/// Calculates $x + yz$.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingAddMul<Y = Self, Z = Self> {
    type Output;

    fn overflowing_add_mul(self, y: Y, z: Z) -> (Self::Output, bool);
}

/// Replaces $x$ with $x + yz$.
///
/// Returns a boolean indicating whether an arithmetic overflow would occur. If an overflow would
/// have occurred then the wrapped value is assigned.
pub trait OverflowingAddMulAssign<Y = Self, Z = Self> {
    fn overflowing_add_mul_assign(&mut self, y: Y, z: Z) -> bool;
}

/// Computes $x - yz$.
pub trait SubMul<Y = Self, Z = Self> {
    type Output;

    fn sub_mul(self, y: Y, z: Z) -> Self::Output;
}

/// Replaces $x$ with $x - yz$.
pub trait SubMulAssign<Y = Self, Z = Self> {
    fn sub_mul_assign(&mut self, y: Y, z: Z);
}

/// Computes $x - yz$, returning `None` if there is no valid result.
pub trait CheckedSubMul<Y = Self, Z = Self> {
    type Output;

    fn checked_sub_mul(self, y: Y, z: Z) -> Option<Self::Output>;
}

/// Computes $x - yz$, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingSubMul<Y = Self, Z = Self> {
    type Output;

    fn saturating_sub_mul(self, y: Y, z: Z) -> Self::Output;
}

/// Replaces $x$ with $x - yz$, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingSubMulAssign<Y = Self, Z = Self> {
    fn saturating_sub_mul_assign(&mut self, y: Y, z: Z);
}

/// Computes $x - yz$, wrapping around at the boundary of the type.
pub trait WrappingSubMul<Y = Self, Z = Self> {
    type Output;

    fn wrapping_sub_mul(self, y: Y, z: Z) -> Self::Output;
}

/// Replaces $x$ with $x - yz$, wrapping around at the boundary of the type.
pub trait WrappingSubMulAssign<Y = Self, Z = Self> {
    fn wrapping_sub_mul_assign(&mut self, y: Y, z: Z);
}

/// Calculates $x - yz$.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingSubMul<Y = Self, Z = Self> {
    type Output;

    fn overflowing_sub_mul(self, y: Y, z: Z) -> (Self::Output, bool);
}

/// Replaces $x$ with $x - yz$.
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

/// Computes `self << other` mod $2^p$. Assumes the input is already reduced mod $2^p$.
pub trait ModPowerOf2Shl<RHS> {
    type Output;

    fn mod_power_of_2_shl(self, other: RHS, pow: u64) -> Self::Output;
}

/// Replaces `self` with `self << other` mod $2^p$. Assumes the input is already reduced mod $2^p$.
pub trait ModPowerOf2ShlAssign<RHS> {
    fn mod_power_of_2_shl_assign(&mut self, other: RHS, pow: u64);
}

/// Computes `self >> other` mod $2^p$. Assumes the input is already reduced mod $2^p$.
pub trait ModPowerOf2Shr<RHS> {
    type Output;

    fn mod_power_of_2_shr(self, other: RHS, pow: u64) -> Self::Output;
}

/// Replaces `self` with `self >> other` mod $2^p$. Assumes the input is already reduced mod $2^p$.
pub trait ModPowerOf2ShrAssign<RHS> {
    fn mod_power_of_2_shr_assign(&mut self, other: RHS, pow: u64);
}

/// Computes `self << other` mod `m`. Assumes the input is already reduced mod `m`.
pub trait ModShl<RHS, M = Self> {
    type Output;

    fn mod_shl(self, other: RHS, m: M) -> Self::Output;
}

/// Replaces `self` with `self << other` mod `m`. Assumes the input is already reduced mod `m`.
pub trait ModShlAssign<RHS, M = Self> {
    fn mod_shl_assign(&mut self, other: RHS, m: M);
}

/// Computes `self >> other` mod `m`. Assumes the input is already reduced mod `m`.
pub trait ModShr<RHS, M = Self> {
    type Output;

    fn mod_shr(self, other: RHS, m: M) -> Self::Output;
}

/// Replaces `self` with `self >> other` mod `m`. Assumes the input is already reduced mod `m`.
pub trait ModShrAssign<RHS, M = Self> {
    fn mod_shr_assign(&mut self, other: RHS, m: M);
}

/// Rounds `self` to a multiple of a power of 2, according to a specified rounding mode.
pub trait RoundToMultipleOfPowerOf2<RHS> {
    type Output;

    fn round_to_multiple_of_power_of_2(self, pow: RHS, rm: RoundingMode) -> Self::Output;
}

/// Rounds `self` to a multiple of a power of 2 in place, according to a specified rounding mode.
pub trait RoundToMultipleOfPowerOf2Assign<RHS> {
    fn round_to_multiple_of_power_of_2_assign(&mut self, pow: RHS, rm: RoundingMode);
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

/// Divides a value by a power of 2, returning just the remainder.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = q2^p + r$ and
/// $0 \leq r < 2^p$.
pub trait ModPowerOf2 {
    type Output;

    fn mod_power_of_2(self, other: u64) -> Self::Output;
}

/// Divides a value by a power of 2, replacing the value by the remainder.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = q2^p + r$ and
/// $0 \leq r < 2^p$.
pub trait ModPowerOf2Assign {
    fn mod_power_of_2_assign(&mut self, other: u64);
}
/// Divides a value by a power of 2, returning just the remainder. The remainder has the same sign
/// as the value.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = q2^p + r$ and
/// $0 \leq |r| < 2^p$.
pub trait RemPowerOf2 {
    type Output;

    fn rem_power_of_2(self, other: u64) -> Self::Output;
}

/// Divides a value by a power of 2, replacing the value by the remainder. The remainder has the
/// same sign as the value.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = q2^p + r$ and
/// $0 \leq |r| < 2^p$.
pub trait RemPowerOf2Assign {
    fn rem_power_of_2_assign(&mut self, other: u64);
}

/// Divides the negative of a value by a power of 2, returning the remainder.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = q2^p - r$ and
/// $0 \leq r < 2^p$.
pub trait NegModPowerOf2 {
    type Output;

    fn neg_mod_power_of_2(self, other: u64) -> Self::Output;
}

/// Divides the negative of a value by a power of 2, replacing the value by the remainder.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = q2^p - r$ and
/// $0 \leq r < 2^p$.
pub trait NegModPowerOf2Assign {
    fn neg_mod_power_of_2_assign(&mut self, other: u64);
}

/// Divides a value by another value, returning just the remainder. The remainder is non-positive.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = q2^p + r$ and
/// $0 \leq -r < 2^p$.
pub trait CeilingModPowerOf2 {
    type Output;

    fn ceiling_mod_power_of_2(self, other: u64) -> Self::Output;
}

/// Divides a value by another value, replacing the value by the remainder. The remainder is
/// non-positive.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = q2^p + r$ and
/// $0 \leq -r < 2^p$.
pub trait CeilingModPowerOf2Assign {
    fn ceiling_mod_power_of_2_assign(&mut self, other: u64);
}

/// Determines whether `self` is even or odd.
pub trait Parity {
    /// Returns whether `self` is even.
    fn even(self) -> bool;

    /// Returns whether `self` is odd.
    fn odd(self) -> bool;
}

/// Determines whether `self` is divisible by $2^p$.
pub trait DivisibleByPowerOf2 {
    fn divisible_by_power_of_2(self, pow: u64) -> bool;
}

/// Determines whether `self` is equal to `other` mod $2^p$.
pub trait EqModPowerOf2<RHS = Self> {
    fn eq_mod_power_of_2(self, other: RHS, pow: u64) -> bool;
}

/// Divides a value by another value, returning the quotient and remainder. The quotient is rounded
/// towards negative infinity, and the remainder has the same sign as the divisor.
///
/// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
pub trait DivMod<RHS = Self> {
    type DivOutput;
    type ModOutput;

    fn div_mod(self, other: RHS) -> (Self::DivOutput, Self::ModOutput);
}

/// Divides a value by another value in place, returning the remainder. The quotient is rounded
/// towards negative infinity, and the remainder has the same sign as the divisor.
///
/// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
pub trait DivAssignMod<RHS = Self> {
    type ModOutput;

    fn div_assign_mod(&mut self, other: RHS) -> Self::ModOutput;
}

/// Divides a value by another value, returning the quotient and remainder. The quotient is rounded
/// towards zero and the remainder has the same sign as the dividend.
///
/// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
pub trait DivRem<RHS = Self> {
    type DivOutput;
    type RemOutput;

    fn div_rem(self, other: RHS) -> (Self::DivOutput, Self::RemOutput);
}

/// Divides a value by another value in place, returning the remainder. The quotient is rounded
/// towards zero and the remainder has the same sign as the dividend.
///
/// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
pub trait DivAssignRem<RHS = Self> {
    type RemOutput;

    fn div_assign_rem(&mut self, other: RHS) -> Self::RemOutput;
}

/// Divides a value by another value, returning the ceiling of the quotient and the remainder of the
/// negative of the first value divided by the second.
///
/// The quotient and remainder satisfy $x = qy - r$ and $0 \leq r < y$.
pub trait CeilingDivNegMod<RHS = Self> {
    type DivOutput;
    type ModOutput;

    fn ceiling_div_neg_mod(self, other: RHS) -> (Self::DivOutput, Self::ModOutput);
}

/// Divides a value by another value in place, taking the ceiling of the quotient and returning the
/// remainder of the negative of the first value divided by the second.
///
/// The quotient and remainder satisfy $x = qy - r$ and $0 \leq r < y$.
pub trait CeilingDivAssignNegMod<RHS = Self> {
    type ModOutput;

    fn ceiling_div_assign_neg_mod(&mut self, other: RHS) -> Self::ModOutput;
}

/// Divides a value by another value, returning the quotient and remainder. The quotient is rounded
/// towards positive infinity and the remainder has the opposite sign of the divisor.
///
/// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
pub trait CeilingDivMod<RHS = Self> {
    type DivOutput;
    type ModOutput;

    fn ceiling_div_mod(self, other: RHS) -> (Self::DivOutput, Self::ModOutput);
}

/// Divides a value by another value in place, taking the quotient and returning the remainder. The
/// quotient is rounded towards positive infinity and the remainder has the opposite sign of the
/// divisor.
///
/// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
pub trait CeilingDivAssignMod<RHS = Self> {
    type ModOutput;

    fn ceiling_div_assign_mod(&mut self, other: RHS) -> Self::ModOutput;
}

/// Divides a value by another value, returning just the remainder. The remainder has the same
/// sign as the second value.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and
/// $0 \leq |r| < |y|$.
pub trait Mod<RHS = Self> {
    type Output;

    fn mod_op(self, other: RHS) -> Self::Output;
}

/// Divides a value by another value, replacing the first value by the remainder. The remainder has
/// the same sign as the second value.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and
/// $0 \leq |r| < |y|$.
pub trait ModAssign<RHS = Self> {
    fn mod_assign(&mut self, other: RHS);
}

/// Divides the negative of a value by another value, returning the remainder.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = qy - r$ and
/// $0 \leq r < y$.
pub trait NegMod<RHS = Self> {
    type Output;

    fn neg_mod(self, other: RHS) -> Self::Output;
}

/// Divides the negative of a value by another value, replacing the first value by the remainder.
/// The remainder has the same sign as the second value.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = qy - r$ and
/// $0 \leq r < y$.
pub trait NegModAssign<RHS = Self> {
    fn neg_mod_assign(&mut self, other: RHS);
}

/// Divides a value by another value, returning just the remainder. The remainder has the opposite
/// sign as the second value.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and
/// $0 \leq |r| < |y|$.
pub trait CeilingMod<RHS = Self> {
    type Output;

    fn ceiling_mod(self, other: RHS) -> Self::Output;
}

/// Divides a value by another value, replacing the first value by the remainder. The remainder has
/// the same sign as the second value.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and
/// $0 \leq |r| < |y|$.
pub trait CeilingModAssign<RHS = Self> {
    fn ceiling_mod_assign(&mut self, other: RHS);
}

/// Divides a value by another value and rounds according to a specified rounding mode.
pub trait DivRound<RHS = Self> {
    type Output;

    fn div_round(self, other: RHS, rm: RoundingMode) -> Self::Output;
}

/// Divides a value by another value in place and rounds according to a specified rounding mode.
pub trait DivRoundAssign<RHS = Self> {
    fn div_round_assign(&mut self, other: RHS, rm: RoundingMode);
}

/// Divides a value by another value. The first value must be exactly divisible by the second.
///
/// If it isn't, this function may crash or return a meaningless result.
pub trait DivExact<RHS = Self> {
    type Output;

    fn div_exact(self, other: RHS) -> Self::Output;
}

/// Divides a value by another value in place. The value being assigned to must be exactly divisible
/// by the value on the RHS. If it isn't, this function may crash or assign a meaningless value to
/// the first value.
pub trait DivExactAssign<RHS = Self> {
    fn div_exact_assign(&mut self, other: RHS);
}

/// Determines whether `self` is divisible by `other`.
pub trait DivisibleBy<RHS = Self> {
    fn divisible_by(self, other: RHS) -> bool;
}

/// Determines whether `self` is equal to `other` mod `m`.
pub trait EqMod<RHS = Self, M = Self> {
    fn eq_mod(self, other: RHS, m: M) -> bool;
}

/// Rounds `self` to a multiple of other, according to a specified rounding mode.
pub trait RoundToMultiple<RHS = Self> {
    type Output;

    fn round_to_multiple(self, other: RHS, rm: RoundingMode) -> Self::Output;
}

/// Rounds `self` to a multiple of other in place, according to a specified rounding mode.
pub trait RoundToMultipleAssign<RHS = Self> {
    fn round_to_multiple_assign(&mut self, other: RHS, rm: RoundingMode);
}

/// Calculates $2^p$.
pub trait PowerOf2 {
    fn power_of_2(pow: u64) -> Self;
}

/// Determines whether `self` == 2<pow>k</pow> for some integer k.
pub trait IsPowerOf2 {
    fn is_power_of_2(&self) -> bool;
}

/// Returns the smallest power of 2 greater than or equal to `self`. If the next power of 2 is
/// greater than the type's maximum value, panics.
pub trait NextPowerOf2 {
    type Output;

    fn next_power_of_2(self) -> Self::Output;
}

/// Replaces `self` with the smallest power of 2 greater than or equal to `self`. If the next
/// power of 2 is greater than the type's maximum value, panics.
pub trait NextPowerOf2Assign {
    fn next_power_of_2_assign(&mut self);
}

/// Returns the smallest power of 2 greater than or equal to `self`.
///
/// If the next power of 2 is greater than the type's maximum value, `None` is returned.
pub trait CheckedNextPowerOf2 {
    type Output;

    fn checked_next_power_of_2(self) -> Option<Self::Output>;
}

/// Raises `self` to the power of `exp`.
pub trait Pow<RHS> {
    type Output;

    fn pow(self, exp: RHS) -> Self::Output;
}

/// Replaces `self` with `self` ^ `exp`.
pub trait PowAssign<RHS = Self> {
    fn pow_assign(&mut self, exp: RHS);
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

/// Saturating exponentiation. Replaces `self` with `self` ^ `exp`, saturating at the numeric bounds
/// instead of overflowing.
pub trait SaturatingPowAssign<RHS = Self> {
    fn saturating_pow_assign(&mut self, exp: RHS);
}

/// Wrapping (modular) exponentiation. Raises `self` to the power of `exp`, wrapping around at the
/// boundary of the type.
pub trait WrappingPow<RHS> {
    type Output;

    fn wrapping_pow(self, exp: RHS) -> Self::Output;
}

/// Wrapping (modular) exponentiation. Replaces `self` with `self` ^ `exp`, wrapping around at the
/// boundary of the type.
pub trait WrappingPowAssign<RHS = Self> {
    fn wrapping_pow_assign(&mut self, exp: RHS);
}

/// Calculates `self` ^ `exp`.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingPow<RHS> {
    type Output;

    fn overflowing_pow(self, exp: RHS) -> (Self::Output, bool);
}

/// Replaces `self` with `self` ^ `exp`.
///
/// Returns a boolean indicating whether an arithmetic overflow would occur. If an overflow would
/// have occurred then the wrapped value is assigned.
pub trait OverflowingPowAssign<RHS = Self> {
    fn overflowing_pow_assign(&mut self, exp: RHS) -> bool;
}

/// Computes `self.pow(exp)` mod $2^p$. Assumes the input is already reduced mod $2^p$.
pub trait ModPowerOf2Pow<RHS = Self> {
    type Output;

    fn mod_power_of_2_pow(self, exp: RHS, pow: u64) -> Self::Output;
}

/// Replaces `self` with `self.pow(exp)` mod $2^p$. Assumes the input is already reduced mod $2^p$.
pub trait ModPowerOf2PowAssign<RHS = Self> {
    fn mod_power_of_2_pow_assign(&mut self, exp: RHS, pow: u64);
}

/// Computes `self.pow(exp)` mod `m`. Assumes the input is already reduced mod `m`.
pub trait ModPow<RHS = Self, M = Self> {
    type Output;

    fn mod_pow(self, exp: RHS, m: M) -> Self::Output;
}

/// Replaces `self` with `self.pow(other)` mod `m`. Assumes the inputs are already reduced mod `m`.
pub trait ModPowAssign<RHS = Self, M = Self> {
    fn mod_pow_assign(&mut self, exp: RHS, m: M);
}

/// Computes `self.pow(exp)` mod `m`. Assumes the inputs are already reduced mod `m`. If multiple
/// modular exponentiations with the same modulus are necessary, it can be quicker to precompute
/// some piece of data and reuse it in the exponentiation calls. This trait provides a function for
/// precomputing the data and a function for using it during exponentiation.
pub trait ModPowPrecomputed<RHS = Self, M = Self>
where
    Self: Sized,
{
    type Output;
    type Data;

    fn precompute_mod_pow_data(m: &M) -> Self::Data;

    fn mod_pow_precomputed(self, exp: RHS, m: M, data: &Self::Data) -> Self::Output;
}

/// Replaces `self` with `self.pow(exp)` mod `m`. Assumes the inputs are already reduced mod `m`. If
/// multiple modular exponentiations with the same modulus are necessary, it can be quicker to
/// precompute some piece of data and reuse it in the exponentiation calls. This trait provides a
/// function for using precomputed data during exponentiation. For precomputing the data, use the
/// `precompute_mod_pow_data` function in `ModPowPrecomputed`.
pub trait ModPowPrecomputedAssign<RHS: Two = Self, M = Self>: ModPowPrecomputed<RHS, M> {
    fn mod_pow_precomputed_assign(&mut self, exp: RHS, m: M, data: &Self::Data);
}

/// Squares `self`.
pub trait Square {
    type Output;

    fn square(self) -> Self::Output;
}

/// Replaces `self` with `self` ^ 2.
pub trait SquareAssign {
    fn square_assign(&mut self);
}

/// Squares `self`, returning `None` if there is no valid result.
pub trait CheckedSquare {
    type Output;

    fn checked_square(self) -> Option<Self::Output>;
}

/// Squares `self`, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingSquare {
    type Output;

    fn saturating_square(self) -> Self::Output;
}

/// Saturating squaring. Replaces `self` with `self` ^ 2, saturating at the numeric bounds instead
/// of overflowing.
pub trait SaturatingSquareAssign {
    fn saturating_square_assign(&mut self);
}

/// Wrapping (modular) squaring. Squares `self`, wrapping around at the boundary of the type.
pub trait WrappingSquare {
    type Output;

    fn wrapping_square(self) -> Self::Output;
}

/// Wrapping (modular) squaring. Replaces `self` with `self` ^ 2, wrapping around at the boundary of
/// the type.
pub trait WrappingSquareAssign {
    fn wrapping_square_assign(&mut self);
}

/// Calculates `self` ^ `exp`.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingSquare {
    type Output;

    fn overflowing_square(self) -> (Self::Output, bool);
}

/// Replaces `self` with `self` ^ 2.
///
/// Returns a boolean indicating whether an arithmetic overflow would occur. If an overflow would
/// have occurred then the wrapped value is assigned.
pub trait OverflowingSquareAssign {
    fn overflowing_square_assign(&mut self) -> bool;
}

/// Computes `self.square()` mod $2^p$. Assumes the input is already reduced mod $2^p$.
pub trait ModPowerOf2Square {
    type Output;

    fn mod_power_of_2_square(self, pow: u64) -> Self::Output;
}

/// Replaces `self` with `self.square()` mod $2^p$. Assumes the input is already reduced mod $2^p$.
pub trait ModPowerOf2SquareAssign {
    fn mod_power_of_2_square_assign(&mut self, pow: u64);
}

/// Computes `self.square()` mod `m`. Assumes the input is already reduced mod `m`.
pub trait ModSquare<M = Self> {
    type Output;

    fn mod_square(self, m: M) -> Self::Output;
}

/// Replaces `self` with `self.square()` mod `m`. Assumes the input is already reduced  mod `m`.
pub trait ModSquareAssign<M = Self> {
    fn mod_square_assign(&mut self, m: M);
}

/// Computes `self.square()` mod `m`. Assumes the input is already reduced mod `m`. If multiple
/// modular squarings with the same modulus are necessary, it can be quicker to precompute some
/// piece of data using `precompute_mod_pow_data` and reuse it in the squaring calls.
pub trait ModSquarePrecomputed<RHS = Self, M = Self>: ModPowPrecomputed<RHS, M>
where
    Self: Sized,
{
    /// Computes `self.square()` mod `m`. Assumes the input is already reduced mod `m`. Some
    /// precomputed data is provided. The precomputed data should be obtained using
    /// `precompute_mod_pow_data`.
    fn mod_square_precomputed(self, m: M, data: &Self::Data) -> Self::Output;
}

/// Replaces `self` with `self.square()` mod `m`. Assumes the input is already reduced mod `m`. If
/// multiple modular squarings with the same modulus are necessary, it can be quicker to precompute
/// some piece of data using `precompute_mod_pow_data` and reuse it in the squaring calls.
pub trait ModSquarePrecomputedAssign<RHS = Self, M = Self>: ModPowPrecomputed<RHS, M> {
    /// Replaces `self` with `self.square()` mod `m`. Assumes the input is already reduced mod `m`.
    /// Some precomputed data is provided. The precomputed data should be obtained using
    /// `precompute_mod_pow_data`.
    fn mod_square_precomputed_assign(&mut self, m: M, data: &Self::Data);
}

/// Provides a function to get the base-2 logarithm of `self`, or return `None` if `self` is not a
/// power of 2.
pub trait CheckedLogBase2 {
    fn checked_log_base_2(self) -> Option<u64>;
}

/// Provides a function to get the floor of the base-2 logarithm of `self`.
pub trait FloorLogBase2 {
    fn floor_log_base_2(self) -> u64;
}

/// Provides a function to get the ceiling of the base-2 logarithm of `self`.
pub trait CeilingLogBase2 {
    fn ceiling_log_base_2(self) -> u64;
}

/// Provides a function to get the base-`p` logarithm of `self`. where `p` is a power of 2, or
/// return `None` if the result is not exact.
pub trait CheckedLogBasePowerOf2 {
    fn checked_log_base_power_of_2(self, pow: u64) -> Option<u64>;
}

/// Provides a function to get the floor of the base-`p` logarithm of `self`, where `p` is a power
/// of 2.
pub trait FloorLogBasePowerOf2 {
    fn floor_log_base_power_of_2(self, pow: u64) -> u64;
}

/// Provides a function to get the ceiling of the base-`p` logarithm of `self`, where `p` is a
/// power of 2.
pub trait CeilingLogBasePowerOf2 {
    fn ceiling_log_base_power_of_2(self, pow: u64) -> u64;
}
