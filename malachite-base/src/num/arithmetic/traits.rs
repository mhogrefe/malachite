// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rounding_modes::RoundingMode;
use core::cmp::Ordering;

// Until the next release, the traits that moved to `num::modular`, `num::number_theory`,
// `num::exponential`, `num::elliptic`, `num::trigonometric`, `num::hyperbolic`, and
// `num::linear_algebra` are also re-exported here, so that paths through this module keep
// resolving. These re-exports will then be removed.
pub use crate::num::elliptic::traits::*;
pub use crate::num::exponential::traits::*;
pub use crate::num::hyperbolic::traits::*;
pub use crate::num::linear_algebra::traits::*;
pub use crate::num::modular::traits::*;
pub use crate::num::number_theory::traits::*;
pub use crate::num::trigonometric::traits::*;

/// Takes the absolute value of a number. Assumes that the number has a representable absolute
/// value.
pub trait Abs {
    type Output;

    fn abs(self) -> Self::Output;
}

/// Replaces a number with its absolute value. Assumes that the number has a representable absolute
/// value.
pub trait AbsAssign {
    fn abs_assign(&mut self);
}

/// Computes the squared absolute value of a number.
///
/// For a real number this is just its square, but for a complex number it is the sum of the squares
/// of its real and imaginary parts. In both cases it equals $|x|^2$; for Gaussian integers and
/// Gaussian rationals this quantity is also called the norm.
pub trait AbsSquared {
    type Output;

    fn abs_squared(self) -> Self::Output;
}

/// Replaces a number with its squared absolute value.
///
/// For a real number this is just squaring in place. For a complex number the result is the purely
/// real value $|x|^2$, embedded in the same type.
pub trait AbsSquaredAssign {
    fn abs_squared_assign(&mut self);
}

/// Computes the complex conjugate of a number.
///
/// For a complex number the sign of the imaginary part is flipped. For a real number, which is its
/// own conjugate, this is the identity; the trivial implementations let generic code use
/// conjugation uniformly, for example when forming Hermitian products.
pub trait Conjugate {
    type Output;

    fn conjugate(self) -> Self::Output;
}

/// Replaces a number with its complex conjugate.
///
/// For a complex number the sign of the imaginary part is flipped. For a real number this does
/// nothing.
pub trait ConjugateAssign {
    fn conjugate_assign(&mut self);
}

/// Multiplies a number by $i$, the imaginary unit.
///
/// For a complex number $a + bi$ this is $-b + ai$, a counterclockwise quarter turn. No type in
/// this crate implements this trait; it exists for complex types downstream, like Gaussian
/// integers.
pub trait MulI {
    type Output;

    fn mul_i(self) -> Self::Output;
}

/// Replaces a number with its product with $i$, the imaginary unit.
///
/// For a complex number $a + bi$ the result is $-b + ai$, a counterclockwise quarter turn. No type
/// in this crate implements this trait; it exists for complex types downstream, like Gaussian
/// integers.
pub trait MulIAssign {
    fn mul_i_assign(&mut self);
}

/// Divides a number by $i$, the imaginary unit.
///
/// For a complex number $a + bi$ this is $b - ai$, a clockwise quarter turn. No type in this crate
/// implements this trait; it exists for complex types downstream, like Gaussian integers.
pub trait DivI {
    type Output;

    fn div_i(self) -> Self::Output;
}

/// Replaces a number with its quotient by $i$, the imaginary unit.
///
/// For a complex number $a + bi$ the result is $b - ai$, a clockwise quarter turn. No type in this
/// crate implements this trait; it exists for complex types downstream, like Gaussian integers.
pub trait DivIAssign {
    fn div_i_assign(&mut self);
}

/// Multiplies a number by $i^k$, a power of the imaginary unit.
///
/// Only $k$ modulo 4 matters: $i^0 = 1$, $i^1 = i$, $i^2 = -1$, and $i^3 = -i$, so the result is
/// the number itself, a counterclockwise quarter turn, a half turn, or a clockwise quarter turn.
/// Since $i^{-k} = i^{3k}$, a negative power is a matter of tripling the exponent. No type in this
/// crate implements this trait; it exists for complex types downstream, like Gaussian integers.
pub trait MulIPow {
    type Output;

    fn mul_i_pow(self, k: u64) -> Self::Output;
}

/// Replaces a number with its product with $i^k$, a power of the imaginary unit.
///
/// Only $k$ modulo 4 matters: $i^0 = 1$, $i^1 = i$, $i^2 = -1$, and $i^3 = -i$, so the result is
/// the number itself, a counterclockwise quarter turn, a half turn, or a clockwise quarter turn.
/// Since $i^{-k} = i^{3k}$, a negative power is a matter of tripling the exponent. No type in this
/// crate implements this trait; it exists for complex types downstream, like Gaussian integers.
pub trait MulIPowAssign {
    fn mul_i_pow_assign(&mut self, k: u64);
}

/// Determines whether a number is a unit of its ring, meaning that it has a multiplicative inverse
/// in the same ring.
///
/// Which elements these are depends on the ring: the only unit of $\mathbb{N}$ is 1, the units of
/// $\mathbb{Z}$ are $\pm 1$, the Gaussian integers have four, $\pm 1$ and $\pm i$, and in a field
/// every nonzero element is a unit.
///
/// The implementations for the primitive integers follow suit, and the ones for the primitive
/// floats count every finite nonzero value.
pub trait IsUnit {
    fn is_unit(&self) -> bool;
}

/// Finds the power of $i$ that brings a number into canonical Gaussian-unit form.
///
/// The Gaussian units are $1$, $i$, $-1$, and $-i$: the units of the Gaussian integers. Multiplying
/// a nonzero complex number by them gives its four Gaussian associates, $x$, $ix$, $-x$, and $-ix$,
/// and the canonical one is the one whose argument lies in $(-\pi/4, \pi/4]$, meaning that its real
/// part $a$ is positive and its imaginary part $b$ satisfies $-a < b \leq a$. This function returns
/// the $k \in \\{0, 1, 2, 3\\}$ such that $x i^k$ is canonical, and 0 for zero;
/// [`CanonicalizeGaussianUnit`] does the multiplication.
///
/// A real number has only the two Gaussian associates $\pm x$ to choose between, so the answer
/// there is 0 for a nonnegative number and 2 for a negative one, $i^2$ being $-1$. That is what the
/// primitive implementations return.
///
/// This is the canonical form of [`CanonicalizeUnit`] only when the ring's units are Gaussian
/// units: for the integers, whose units are $\pm 1$, and the Gaussian integers, whose units are
/// $\pm 1$ and $\pm i$. In a field every nonzero element is a unit, so the canonical associate of a
/// nonzero rational or Gaussian rational is 1, which no power of $i$ reaches.
pub trait CanonicalUnitIPow {
    fn canonical_unit_i_pow(&self) -> u64;
}

/// Brings a number into canonical unit form: replaces it with its canonical associate.
///
/// The associates of a number are the numbers it becomes when multiplied by a unit of its ring, and
/// the canonical associate is the one chosen to represent them all. Each implementation documents
/// its choice. An unsigned number is already canonical, so the unsigned implementations are the
/// identity; the units of the integers are $\pm 1$, so the signed implementations return the
/// absolute value; the units of the Gaussian integers are the powers of $i$, and the canonical
/// associate is the one whose argument lies in $(-\pi/4, \pi/4]$ (see [`CanonicalUnitIPow`]). In a
/// field every nonzero element is a unit, so the canonical associate of a nonzero rational number,
/// a nonzero Gaussian rational, or a finite nonzero floating-point number is 1; zero, the
/// infinities, and NaN are not units, and keep their absolute value. For a polynomial over the
/// integers it is the associate with a positive leading coefficient, and over the rationals it is
/// the monic one.
///
/// For the canonical form under the Gaussian units $\pm 1$ and $\pm i$ alone, which rotates a
/// number by a power of $i$ rather than collapsing a field's units to 1, see
/// [`CanonicalizeGaussianUnit`].
pub trait CanonicalizeUnit {
    type Output;

    fn canonicalize_unit(self) -> Self::Output;
}

/// Replaces a number with its canonical unit form: its canonical associate.
///
/// This is the in-place form of [`CanonicalizeUnit`], and the same per-type behaviour applies.
pub trait CanonicalizeUnitAssign {
    fn canonicalize_unit_assign(&mut self);
}

/// Brings a number into canonical Gaussian-unit form: multiplies it by the power of $i$ that
/// [`CanonicalUnitIPow`] finds.
///
/// The result is the Gaussian associate whose argument lies in $(-\pi/4, \pi/4]$. For a real number
/// that is its absolute value, and zero is its own canonical form. This differs from
/// [`CanonicalizeUnit`] exactly in a field, where every nonzero element is a unit and canonicalizes
/// to 1; here a nonzero number keeps its magnitude and is only rotated by a quarter turn or
/// negated.
pub trait CanonicalizeGaussianUnit {
    type Output;

    fn canonicalize_gaussian_unit(self) -> Self::Output;
}

/// Replaces a number with its canonical Gaussian-unit form.
///
/// This is the in-place form of [`CanonicalizeGaussianUnit`].
pub trait CanonicalizeGaussianUnitAssign {
    fn canonicalize_gaussian_unit_assign(&mut self);
}

/// Brings a value into canonical sign form: negates it if it is negative, so that the result is
/// non-negative.
///
/// A value and its negation are the same up to multiplication by $\pm 1$, and this picks one of the
/// two to represent both. What "negative" means is up to the type: a vector is negative when its
/// pivot, its first nonzero element, is, so a vector of zeros is already canonical.
pub trait CanonicalizeSign {
    type Output;

    fn canonicalize_sign(self) -> Self::Output;
}

/// Replaces a value with its canonical sign form.
///
/// This is the in-place form of [`CanonicalizeSign`].
pub trait CanonicalizeSignAssign {
    fn canonicalize_sign_assign(&mut self);
}

/// Takes the absolute value of a number and converts to the unsigned equivalent.
pub trait UnsignedAbs {
    type Output;

    fn unsigned_abs(self) -> Self::Output;
}

/// Replaces every entry of a value, such as every element of a vector, by its absolute value.
///
/// This is distinct from [`Abs`], which for a vector would be its Euclidean length.
pub trait EntrywiseAbs {
    type Output;

    fn entrywise_abs(self) -> Self::Output;
}

/// Replaces every entry of a value, such as every element of a vector, by its absolute value, in
/// place.
pub trait EntrywiseAbsAssign {
    fn entrywise_abs_assign(&mut self);
}

/// Replaces every entry of a value, such as every element of a vector, by its absolute value,
/// converting to the unsigned equivalent, as [`UnsignedAbs`] does for a single number.
pub trait EntrywiseUnsignedAbs {
    type Output;

    fn entrywise_unsigned_abs(self) -> Self::Output;
}

/// Takes the entrywise maximum of two values, such as two vectors of the same dimension: the value
/// whose every entry is the larger of the corresponding entries.
pub trait EntrywiseMax<RHS = Self> {
    type Output;

    fn entrywise_max(self, other: RHS) -> Self::Output;
}

/// Replaces a value, such as a vector, by its entrywise maximum with another: every entry becomes
/// the larger of itself and the corresponding entry of the other value.
pub trait EntrywiseMaxAssign<RHS = Self> {
    fn entrywise_max_assign(&mut self, other: RHS);
}

/// Takes the entrywise minimum of two values, such as two vectors of the same dimension: the value
/// whose every entry is the smaller of the corresponding entries.
pub trait EntrywiseMin<RHS = Self> {
    type Output;

    fn entrywise_min(self, other: RHS) -> Self::Output;
}

/// Replaces a value, such as a vector, by its entrywise minimum with another: every entry becomes
/// the smaller of itself and the corresponding entry of the other value.
pub trait EntrywiseMinAssign<RHS = Self> {
    fn entrywise_min_assign(&mut self, other: RHS);
}

/// Takes the entrywise product of two values, such as two vectors of the same dimension: the value
/// whose every entry is the product of the corresponding entries. For vectors this is also called
/// the Hadamard or Schur product.
pub trait EntrywiseMul<RHS = Self> {
    type Output;

    fn entrywise_mul(self, other: RHS) -> Self::Output;
}

/// Replaces a value, such as a vector, by its entrywise product with another: every entry is
/// multiplied by the corresponding entry of the other value.
pub trait EntrywiseMulAssign<RHS = Self> {
    fn entrywise_mul_assign(&mut self, other: RHS);
}

/// Divides every entry of a value, such as a vector, by a scalar, rounding each quotient according
/// to a specified rounding mode.
///
/// Unlike [`DivRound`], no [`Ordering`] is returned, since different entries may be rounded in
/// different directions.
pub trait EntrywiseDivRound<RHS> {
    type Output;

    fn entrywise_div_round(self, other: RHS, rm: RoundingMode) -> Self::Output;
}

/// Divides every entry of a value, such as a vector, by a scalar in place, rounding each quotient
/// according to a specified rounding mode.
///
/// Unlike [`DivRoundAssign`], no [`Ordering`] is returned, since different entries may be rounded
/// in different directions.
pub trait EntrywiseDivRoundAssign<RHS> {
    fn entrywise_div_round_assign(&mut self, other: RHS, rm: RoundingMode);
}

/// Left-shifts every entry of a value, such as a vector (multiplying it by a power of 2), rounding
/// each entry according to a specified rounding mode.
///
/// Unlike [`ShlRound`], no [`Ordering`] is returned, since different entries may be rounded in
/// different directions. Rounding might only be necessary if `other` is negative.
pub trait EntrywiseShlRound<RHS> {
    type Output;

    fn entrywise_shl_round(self, other: RHS, rm: RoundingMode) -> Self::Output;
}

/// Left-shifts every entry of a value, such as a vector (multiplying it by a power of 2), in place,
/// rounding each entry according to a specified rounding mode.
///
/// Unlike [`ShlRoundAssign`], no [`Ordering`] is returned, since different entries may be rounded
/// in different directions. Rounding might only be necessary if `other` is negative.
pub trait EntrywiseShlRoundAssign<RHS> {
    fn entrywise_shl_round_assign(&mut self, other: RHS, rm: RoundingMode);
}

/// Right-shifts every entry of a value, such as a vector (dividing it by a power of 2), rounding
/// each entry according to a specified rounding mode.
///
/// Unlike [`ShrRound`], no [`Ordering`] is returned, since different entries may be rounded in
/// different directions. Rounding might only be necessary if `other` is positive.
pub trait EntrywiseShrRound<RHS> {
    type Output;

    fn entrywise_shr_round(self, other: RHS, rm: RoundingMode) -> Self::Output;
}

/// Right-shifts every entry of a value, such as a vector (dividing it by a power of 2), in place,
/// rounding each entry according to a specified rounding mode.
///
/// Unlike [`ShrRoundAssign`], no [`Ordering`] is returned, since different entries may be rounded
/// in different directions. Rounding might only be necessary if `other` is positive.
pub trait EntrywiseShrRoundAssign<RHS> {
    fn entrywise_shr_round_assign(&mut self, other: RHS, rm: RoundingMode);
}

/// Subtracts two numbers and takes the absolute value of the difference.
pub trait AbsDiff<RHS = Self> {
    type Output;

    fn abs_diff(self, other: RHS) -> Self::Output;
}

/// Replaces a number with the absolute value of its difference with another number.
pub trait AbsDiffAssign<RHS = Self> {
    fn abs_diff_assign(&mut self, other: RHS);
}

/// Adds a number and the product of two other numbers.
///
/// Depending on the implementing type, the fused operation may compute the same value as the
/// unfused `self + y * z` more efficiently; or, for types with rounding, it may compute a *more
/// accurate* value -- the product enters the addition exactly, with a single rounding at the end --
/// but *less* efficiently, since the exact product must be computed in full. See each
/// implementation's documentation for which contract it provides.
pub trait AddMul<Y = Self, Z = Self> {
    type Output;

    fn add_mul(self, y: Y, z: Z) -> Self::Output;
}

/// Adds a number and the product of two other numbers, in place.
///
/// Depending on the implementing type, the fused operation may compute the same value as the
/// unfused `*self + y * z` more efficiently; or, for types with rounding, it may compute a *more
/// accurate* value -- the product enters the addition exactly, with a single rounding at the end --
/// but *less* efficiently, since the exact product must be computed in full. See each
/// implementation's documentation for which contract it provides.
pub trait AddMulAssign<Y = Self, Z = Self> {
    fn add_mul_assign(&mut self, y: Y, z: Z);
}

/// Adds a number and the product of two other numbers, shifted left by a number of bits: $x +
/// yz2^k$.
pub trait AddMulShl<Y = Self, Z = Self> {
    type Output;

    fn add_mul_shl(self, y: Y, z: Z, bits: u64) -> Self::Output;
}

/// Adds the product of two numbers, shifted left by a number of bits, to a number in place: $x
/// \gets x + yz2^k$.
pub trait AddMulShlAssign<Y = Self, Z = Self> {
    fn add_mul_shl_assign(&mut self, y: Y, z: Z, bits: u64);
}

/// Adds the products of two pairs of numbers.
pub trait MulAddMul<Y = Self, Z = Self, W = Self> {
    type Output;

    fn mul_add_mul(self, y: Y, z: Z, w: W) -> Self::Output;
}

/// Adds the products of two pairs of numbers, in place.
pub trait MulAddMulAssign<Y = Self, Z = Self, W = Self> {
    fn mul_add_mul_assign(&mut self, y: Y, z: Z, w: W);
}

/// Multiplies two numbers and right-shifts the product (divides it by a power of 2), rounding the
/// result according to a specified rounding mode. An [`Ordering`] is also returned, indicating
/// whether the returned value is less than, equal to, or greater than the exact value.
///
/// The product is computed exactly, as if at unlimited width; only the final shifted result must be
/// representable.
pub trait MulShrRound<RHS = Self, B = u64> {
    type Output;

    fn mul_shr_round(self, other: RHS, bits: B, rm: RoundingMode) -> (Self::Output, Ordering);
}

/// Multiplies two numbers and right-shifts the product (divides it by a power of 2) in place,
/// rounding the result according to a specified rounding mode. An [`Ordering`] is returned,
/// indicating whether the assigned value is less than, equal to, or greater than the exact value.
///
/// The product is computed exactly, as if at unlimited width; only the final shifted result must be
/// representable.
pub trait MulShrRoundAssign<RHS = Self, B = u64> {
    fn mul_shr_round_assign(&mut self, other: RHS, bits: B, rm: RoundingMode) -> Ordering;
}

/// Subtracts the product of one pair of numbers from the product of another.
pub trait MulSubMul<Y = Self, Z = Self, W = Self> {
    type Output;

    fn mul_sub_mul(self, y: Y, z: Z, w: W) -> Self::Output;
}

/// Subtracts the product of one pair of numbers from the product of another, in place.
pub trait MulSubMulAssign<Y = Self, Z = Self, W = Self> {
    fn mul_sub_mul_assign(&mut self, y: Y, z: Z, w: W);
}

/// Calculates the hypotenuse of two numbers, $\sqrt{x^2+y^2}$.
pub trait Hypot<RHS = Self> {
    type Output;

    fn hypot(self, other: RHS) -> Self::Output;
}

/// Replaces a number with the hypotenuse of it and another number.
pub trait HypotAssign<RHS = Self> {
    fn hypot_assign(&mut self, other: RHS);
}

/// Left-shifts a number (multiplies it by a power of 2), returning `None` if the result is not
/// representable.
pub trait ArithmeticCheckedShl<RHS> {
    type Output;

    fn arithmetic_checked_shl(self, other: RHS) -> Option<Self::Output>;
}

/// Right-shifts a number (divides it by a power of 2), returning `None` if the result is not
/// representable.
pub trait ArithmeticCheckedShr<RHS> {
    type Output;

    fn arithmetic_checked_shr(self, other: RHS) -> Option<Self::Output>;
}

/// Computes the average (arithmetic mean) of two numbers, rounding to the nearest integer. Two-way
/// ties are broken by rounding to the even integer.
///
/// The average is computed without overflow: the result is always exact or within a half of the
/// exact value, so it always fits in the same type as the inputs.
pub trait Average<RHS = Self> {
    type Output;

    fn average(self, other: RHS) -> Self::Output;
}

/// Computes the average (arithmetic mean) of two numbers, rounding to the nearest integer and
/// replacing the first number with it. Two-way ties are broken by rounding to the even integer.
///
/// The average is computed without overflow: the result is always exact or within a half of the
/// exact value, so it always fits in the same type as the inputs.
pub trait AverageAssign<RHS = Self> {
    fn average_assign(&mut self, other: RHS);
}

/// Computes the average (arithmetic mean) of two numbers and rounds according to a specified
/// rounding mode. An [`Ordering`] is also returned, indicating whether the returned value is less
/// than, equal to, or greater than the exact value.
///
/// The average is computed without overflow: the result is always exact or within a half of the
/// exact value, so it always fits in the same type as the inputs.
pub trait AverageRound<RHS = Self> {
    type Output;

    fn average_round(self, other: RHS, rm: RoundingMode) -> (Self::Output, Ordering);
}

/// Computes the average (arithmetic mean) of two numbers, rounding according to a specified
/// rounding mode and replacing the first number with it. An [`Ordering`] is returned, indicating
/// whether the assigned value is less than, equal to, or greater than the exact value.
///
/// The average is computed without overflow: the result is always exact or within a half of the
/// exact value, so it always fits in the same type as the inputs.
pub trait AverageRoundAssign<RHS = Self> {
    fn average_round_assign(&mut self, other: RHS, rm: RoundingMode) -> Ordering;
}

/// Takes the ceiling of a number.
pub trait Ceiling {
    type Output;

    fn ceiling(self) -> Self::Output;
}

/// Replaces a number with its ceiling.
pub trait CeilingAssign {
    fn ceiling_assign(&mut self);
}

/// Takes the absolute valie of a number, returning `None` if the result is not representable.
pub trait CheckedAbs {
    type Output;

    fn checked_abs(self) -> Option<Self::Output>;
}

/// Adds two numbers, returning `None` if the result is not representable.
pub trait CheckedAdd<RHS = Self> {
    type Output;

    fn checked_add(self, other: RHS) -> Option<Self::Output>;
}

/// Adds a number and the product of two other numbers, returning `None` if the result is not
/// representable.
pub trait CheckedAddMul<Y = Self, Z = Self> {
    type Output;

    fn checked_add_mul(self, y: Y, z: Z) -> Option<Self::Output>;
}

/// Adds the products of two pairs of numbers, returning `None` if the result is not representable.
pub trait CheckedMulAddMul<Y = Self, Z = Self, W = Self> {
    type Output;

    fn checked_mul_add_mul(self, y: Y, z: Z, w: W) -> Option<Self::Output>;
}

/// Subtracts the product of one pair of numbers from the product of another, returning `None` if
/// the result is not representable.
pub trait CheckedMulSubMul<Y = Self, Z = Self, W = Self> {
    type Output;

    fn checked_mul_sub_mul(self, y: Y, z: Z, w: W) -> Option<Self::Output>;
}

/// Divides two numbers, returning `None` if the result is not representable.
pub trait CheckedDiv<RHS = Self> {
    type Output;

    fn checked_div(self, other: RHS) -> Option<Self::Output>;
}

/// Multiplies two numbers, returning `None` if the result is not representable.
pub trait CheckedMul<RHS = Self> {
    type Output;

    fn checked_mul(self, other: RHS) -> Option<Self::Output>;
}

/// Negates a number, returning `None` if the result is not representable.
pub trait CheckedNeg {
    type Output;

    fn checked_neg(self) -> Option<Self::Output>;
}

/// Finds the smallest integer power of 2 greater than or equal to a number, returning `None` if the
/// result is not representable.
pub trait CheckedNextPowerOf2 {
    type Output;

    fn checked_next_power_of_2(self) -> Option<Self::Output>;
}

/// Raises a number to a power, returning `None` if the result is not representable.
pub trait CheckedPow<RHS> {
    type Output;

    fn checked_pow(self, exp: RHS) -> Option<Self::Output>;
}

/// Squares a number, returning `None` if the result is not representable.
pub trait CheckedSquare {
    type Output;

    fn checked_square(self) -> Option<Self::Output>;
}

/// Subtracts two numbers, returning `None` if the result is not representable.
pub trait CheckedSub<RHS = Self> {
    type Output;

    fn checked_sub(self, other: RHS) -> Option<Self::Output>;
}

/// Subtracts a number by the product of two other numbers, returning `None` if the result is not
/// representable.
pub trait CheckedSubMul<Y = Self, Z = Self> {
    type Output;

    fn checked_sub_mul(self, y: Y, z: Z) -> Option<Self::Output>;
}

/// Combines two congruences by the Chinese remainder theorem, returning the representative of
/// smallest absolute value, or `None` if the moduli are not coprime. The first residue may be
/// negative.
pub trait BalancedCrt<M1 = Self, R2 = Self, M2 = Self> {
    type Output;

    fn balanced_crt(self, m1: M1, r2: R2, m2: M2) -> Option<Self::Output>;
}

/// Divides two numbers, assuming the first exactly divides the second.
///
/// If it doesn't, the `div_exact` function may panic or return a meaningless result.
pub trait DivExact<RHS = Self> {
    type Output;

    fn div_exact(self, other: RHS) -> Self::Output;
}

/// Divides a number by another number in place, assuming the first exactly divides the second.
///
/// If it doesn't, this function may panic or assign a meaningless number to the first number.
pub trait DivExactAssign<RHS = Self> {
    fn div_exact_assign(&mut self, other: RHS);
}

/// Divides two numbers, returning the quotient and remainder. The quotient is rounded towards
/// negative infinity, and the remainder has the same sign as the divisor (second input).
///
/// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
pub trait DivMod<RHS = Self> {
    type DivOutput;
    type ModOutput;

    fn div_mod(self, other: RHS) -> (Self::DivOutput, Self::ModOutput);
}

/// Divides two numbers, returning the quotient and remainder. The quotient is rounded towards
/// negative infinity, and the remainder has the same sign as the divisor (second input).
///
/// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
///
/// If multiple divisions by the same divisor are necessary, it can be quicker to precompute some
/// piece of data based on the divisor and reuse it in the division calls. This trait provides a
/// function for precomputing the data and a function for using it during division.
pub trait DivModPrecomputed<RHS = Self> {
    type DivOutput;
    type ModOutput;
    type Data;

    /// Precomputes some data to use for division.
    fn precompute_div_mod_data(other: &RHS) -> Self::Data;

    fn div_mod_precomputed(
        self,
        other: RHS,
        data: &Self::Data,
    ) -> (Self::DivOutput, Self::ModOutput);
}

/// Divides a number by another number in place, returning the remainder. The quotient is rounded
/// towards negative infinity, and the remainder has the same sign as the divisor (second input).
///
/// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
///
/// If multiple divisions by the same divisor are necessary, it can be quicker to precompute some
/// piece of data based on the divisor and reuse it in the division calls. This trait provides a
/// function for using precomputed data during division. For precomputing the data, use the
/// [`precompute_div_mod_data`](DivModPrecomputed::precompute_div_mod_data) function in
/// [`DivModPrecomputed`].
pub trait DivAssignModPrecomputed<RHS = Self>: DivModPrecomputed<RHS> {
    fn div_assign_mod_precomputed(&mut self, other: RHS, data: &Self::Data) -> Self::ModOutput;
}

/// Divides two numbers, returning just the quotient. The quotient is rounded towards the quotient
/// that makes the remainder nonnegative.
///
/// If the remainder were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
/// \leq r < |y|$.
pub trait DivEuclidean<RHS = Self> {
    type Output;

    fn div_euclidean(self, other: RHS) -> Self::Output;
}

/// Divides a number by another number in place, keeping just the quotient. The quotient is rounded
/// towards the quotient that makes the remainder nonnegative.
///
/// If the remainder were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
/// \leq r < |y|$.
pub trait DivEuclideanAssign<RHS = Self> {
    fn div_euclidean_assign(&mut self, other: RHS);
}

/// Divides two numbers, returning the quotient and remainder. The quotient is rounded towards the
/// quotient that makes the remainder nonnegative, and the remainder is always nonnegative.
///
/// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < |y|$.
pub trait DivModEuclidean<RHS = Self> {
    type DivOutput;
    type ModOutput;

    fn div_mod_euclidean(self, other: RHS) -> (Self::DivOutput, Self::ModOutput);
}

/// Divides a number by another number in place, returning the remainder. The quotient is rounded
/// towards negative infinity, and the remainder has the same sign as the divisor (second input).
///
/// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
pub trait DivAssignMod<RHS = Self> {
    type ModOutput;

    fn div_assign_mod(&mut self, other: RHS) -> Self::ModOutput;
}

/// Divides a number by another number in place, returning the remainder. The quotient is rounded
/// towards the quotient that makes the remainder nonnegative, and the remainder is always
/// nonnegative.
///
/// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < |y|$.
pub trait DivAssignModEuclidean<RHS = Self> {
    type ModOutput;

    fn div_assign_mod_euclidean(&mut self, other: RHS) -> Self::ModOutput;
}

/// Divides two numbers, returning the quotient and remainder. The quotient is rounded towards zero,
/// and the remainder has the same sign as the dividend (first input).
///
/// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
pub trait DivRem<RHS = Self> {
    type DivOutput;
    type RemOutput;

    fn div_rem(self, other: RHS) -> (Self::DivOutput, Self::RemOutput);
}

/// Divides a number by another number in place, returning the remainder. The quotient is rounded
/// towards zero, and the remainder has the same sign as the dividend (first input).
///
/// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
pub trait DivAssignRem<RHS = Self> {
    type RemOutput;

    fn div_assign_rem(&mut self, other: RHS) -> Self::RemOutput;
}

/// Divides a number by another number, returning the ceiling of the quotient and the remainder of
/// the negative of the first number divided by the second.
///
/// The quotient and remainder satisfy $x = qy - r$ and $0 \leq r < y$.
pub trait CeilingDivNegMod<RHS = Self> {
    type DivOutput;
    type ModOutput;

    fn ceiling_div_neg_mod(self, other: RHS) -> (Self::DivOutput, Self::ModOutput);
}

/// Divides a number by another number in place, taking the ceiling of the quotient and returning
/// the remainder of the negative of the first number divided by the second.
///
/// The quotient and remainder satisfy $x = qy - r$ and $0 \leq r < y$.
pub trait CeilingDivAssignNegMod<RHS = Self> {
    type ModOutput;

    fn ceiling_div_assign_neg_mod(&mut self, other: RHS) -> Self::ModOutput;
}

/// Divides a number by another number, returning the quotient and remainder. The quotient is
/// rounded towards positive infinity and the remainder has the opposite sign as the divisor (second
/// input).
///
/// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
pub trait CeilingDivMod<RHS = Self> {
    type DivOutput;
    type ModOutput;

    fn ceiling_div_mod(self, other: RHS) -> (Self::DivOutput, Self::ModOutput);
}

/// Divides a number by another number in place, taking the quotient and returning the remainder.
/// The quotient is rounded towards positive infinity and the remainder has the opposite sign of the
/// divisor (second input).
///
/// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
pub trait CeilingDivAssignMod<RHS = Self> {
    type ModOutput;

    fn ceiling_div_assign_mod(&mut self, other: RHS) -> Self::ModOutput;
}

/// Divides a number by another number and rounds according to a specified rounding mode. An
/// [`Ordering`] is also returned, indicating whether the returned value is less than, equal to, or
/// greater than the exact value.
pub trait DivRound<RHS = Self> {
    type Output;

    fn div_round(self, other: RHS, rm: RoundingMode) -> (Self::Output, Ordering);
}

/// Divides a number by another number in place and rounds according to a specified rounding mode.
/// An [`Ordering`] is returned, indicating whether the assigned value is less than, equal to, or
/// greater than the exact value.
pub trait DivRoundAssign<RHS = Self> {
    fn div_round_assign(&mut self, other: RHS, rm: RoundingMode) -> Ordering;
}

/// Takes the floor of a number.
pub trait Floor {
    type Output;

    fn floor(self) -> Self::Output;
}

/// Replaces a number with its floor.
pub trait FloorAssign {
    fn floor_assign(&mut self);
}

/// Calculates the height of a value: the largest of the magnitudes of the parts it is built from.
///
/// For a rational number $p/q$ in lowest terms this is $\max(|p|, q)$, the measure in which
/// Diophantine approximation bounds are usually stated. For something built out of several such
/// values — a polynomial, or a complex number — it is the largest of their heights.
pub trait Height {
    type Output;

    fn to_height(&self) -> Self::Output;

    fn into_height(self) -> Self::Output;

    fn height_significant_bits(&self) -> u64;
}

/// Lends the height of a value, for the types that already hold it.
///
/// Most heights are one of the magnitudes the value is built from, so they can be lent rather than
/// built: a rational number's height is its numerator or its denominator, and a polynomial's is one
/// of its coefficients. This is separate from [`Height`] because not every height is stored — the
/// coefficients of a polynomial over the rationals share one denominator, so a coefficient's height
/// has to be worked out before it can be compared with the others, and there is nothing to lend.
///
/// It is also not worth implementing where the height is a primitive integer, since there is no
/// clone to avoid.
pub trait HeightRef: Height {
    fn height_ref(&self) -> &Self::Output;
}

/// Calculates the $\ell^1$ norm of a value: the sum of the absolute values of the parts it is built
/// from, such as the elements of a vector.
pub trait L1Norm {
    type Output;

    fn to_l1_norm(&self) -> Self::Output;

    fn into_l1_norm(self) -> Self::Output;

    fn l1_norm_significant_bits(&self) -> u64;
}

/// Calculates the product of the elements of a vector as a value of a wider type, so that the
/// product cannot overflow.
///
/// Unlike [`ElementProduct`], whose output has the type of the elements, this produces a `T` that
/// can hold any product of them: for example, the product of the elements of an
/// [`UnsignedVector<u64>`](crate::unsigned_vector::UnsignedVector) as a `Natural`.
pub trait WideningElementProduct<T> {
    fn to_widening_element_product(&self) -> T;

    fn into_widening_element_product(self) -> T;
}

/// Computes the dot product of two vectors: the sum of the products of their corresponding
/// elements.
///
/// The vectors must have the same dimension. The dot product of two 0-dimensional vectors is 0.
pub trait Dot<RHS = Self> {
    type Output;

    fn dot(self, other: RHS) -> Self::Output;
}

/// Computes the cross product of two 3-dimensional vectors: the vector perpendicular to both whose
/// length is the area of the parallelogram they span.
///
/// $$
/// v \times w = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0).
/// $$
///
/// Both vectors must have dimension 3.
pub trait Cross<RHS = Self> {
    type Output;

    fn cross(self, other: RHS) -> Self::Output;
}

/// Replaces a 3-dimensional vector with its cross product with another.
///
/// $$
/// v \gets v \times w.
/// $$
///
/// Both vectors must have dimension 3.
pub trait CrossAssign<RHS = Self> {
    fn cross_assign(&mut self, other: RHS);
}

/// Computes the generalized cross product of $n - 1$ vectors of dimension $n$: the vector
/// perpendicular to all of them whose length is the $(n - 1)$-dimensional volume of the
/// parallelotope they span, and which follows them in a positively oriented basis when they are
/// linearly independent.
///
/// $n - 1$ vectors of dimension $n$ determine the vector $w$ with
///
/// $$
/// w_i = (-1)^{n - 1 + i} \det A_{(i)},
/// $$
///
/// where $A$ is the $(n - 1) \times n$ matrix whose rows are the vectors and $A_{(i)}$ is $A$
/// without column $i$, so that $w \cdot x = \det \begin{pmatrix} A \\ x \end{pmatrix}$ for every
/// $x$. For $n = 2$ it rotates the vector a quarter turn counterclockwise, $(a, b) \mapsto (-b,
/// a)$; for $n = 3$ it is the cross product $v \times w$. The dimension is determined by the number
/// of vectors, so no vectors give the 1-dimensional vector $(1)$.
pub trait GeneralizedCross: Sized {
    fn generalized_cross(xs: &[Self]) -> Self;
}

/// Adds the dot product of two vectors to a number: $x + \sum_i v_iw_i$.
///
/// The vectors must have the same dimension. Computing the sum and the products together can avoid
/// creating each product as a separate number, and is FLINT's `_fmpz_vec_dot_general` with an
/// initial term.
pub trait AddDot<V, W = V> {
    type Output;

    fn add_dot(self, v: V, w: W) -> Self::Output;
}

/// Adds the dot product of two vectors to a number in place: $x \gets x + \sum_i v_iw_i$.
///
/// The vectors must have the same dimension.
pub trait AddDotAssign<V, W = V> {
    fn add_dot_assign(&mut self, v: V, w: W);
}

/// Subtracts the dot product of two vectors from a number: $x - \sum_i v_iw_i$.
///
/// The vectors must have the same dimension.
pub trait SubDot<V, W = V> {
    type Output;

    fn sub_dot(self, v: V, w: W) -> Self::Output;
}

/// Subtracts the dot product of two vectors from a number in place: $x \gets x - \sum_i v_iw_i$.
///
/// The vectors must have the same dimension.
pub trait SubDotAssign<V, W = V> {
    fn sub_dot_assign(&mut self, v: V, w: W);
}

/// Computes the dot product of a vector and the reverse of another: $\sum_i v_iw_{n-1-i}$, where
/// $n$ is the dimension.
///
/// Reading the second vector backwards pairs the first element of one vector with the last of the
/// other, as in the coefficient of a polynomial product. The vectors must have the same dimension.
pub trait DotReversed<RHS = Self> {
    type Output;

    fn dot_reversed(self, other: RHS) -> Self::Output;
}

/// Adds the dot product of a vector and the reverse of another to a number: $x + \sum_i
/// v_iw_{n-1-i}$.
///
/// The vectors must have the same dimension.
pub trait AddDotReversed<V, W = V> {
    type Output;

    fn add_dot_reversed(self, v: V, w: W) -> Self::Output;
}

/// Adds the dot product of a vector and the reverse of another to a number in place: $x \gets x +
/// \sum_i v_iw_{n-1-i}$.
///
/// The vectors must have the same dimension.
pub trait AddDotReversedAssign<V, W = V> {
    fn add_dot_reversed_assign(&mut self, v: V, w: W);
}

/// Subtracts the dot product of a vector and the reverse of another from a number: $x - \sum_i
/// v_iw_{n-1-i}$.
///
/// The vectors must have the same dimension.
pub trait SubDotReversed<V, W = V> {
    type Output;

    fn sub_dot_reversed(self, v: V, w: W) -> Self::Output;
}

/// Subtracts the dot product of a vector and the reverse of another from a number in place: $x
/// \gets x - \sum_i v_iw_{n-1-i}$.
///
/// The vectors must have the same dimension.
pub trait SubDotReversedAssign<V, W = V> {
    fn sub_dot_reversed_assign(&mut self, v: V, w: W);
}

/// Determines the sign of the dot product of two vectors, as the ordering of the dot product
/// against 0, without computing the dot product when it can be avoided.
///
/// The vectors must have the same dimension.
pub trait DotSign<RHS = Self> {
    fn dot_sign(self, other: RHS) -> Ordering;
}

/// Calculates the LCM of the elements of a vector.
///
/// The LCM of the elements of the 0-dimensional vector is 1, and the LCM is 0 if any element is 0.
pub trait ElementLcm {
    type Output;

    fn to_element_lcm(&self) -> Self::Output;

    fn into_element_lcm(self) -> Self::Output;
}

/// Calculates the product of the elements of a vector.
///
/// Unlike [`Product`](core::iter::Product), which would multiply vectors together, this multiplies
/// the elements of one vector, producing a single element.
pub trait ElementProduct {
    type Output;

    fn to_element_product(&self) -> Self::Output;

    fn into_element_product(self) -> Self::Output;
}

/// Calculates the sum of the elements of a vector.
///
/// Unlike [`Sum`](core::iter::Sum), which adds vectors to each other, this adds up the elements of
/// one vector, producing a single element.
pub trait ElementSum {
    type Output;

    fn to_element_sum(&self) -> Self::Output;

    fn into_element_sum(self) -> Self::Output;
}

/// Determines whether a number is an integer power of 2.
pub trait IsPowerOf2 {
    fn is_power_of_2(&self) -> bool;
}

/// Computes the content of a value: the unique non-negative rational $c$ such that the value is $c$
/// times an element of the underlying integer lattice with coprime coordinates.
///
/// This applies to an element of a vector space over the rationals with a distinguished integer
/// lattice, like a polynomial, a Gaussian rational, or a vector. For an element of the lattice
/// itself, such as a polynomial or vector with integer entries, the content is the GCD of the
/// coordinates, a non-negative integer. Zero has content 0.
///
/// The content does not depend on any choice of sign or unit: [`PrimitivePart`] and
/// [`CanonicalPrimitivePart`] both divide by the same content.
pub trait Content {
    type Output;

    fn content(self) -> Self::Output;
}

/// Computes the GCD of a given value and the content of another: for a vector, the GCD of the value
/// and the vector's elements.
///
/// This makes it possible to accumulate the common content of several values one at a time,
/// starting from 0. As with [`Content`], the computation can stop as soon as the GCD reaches 1.
pub trait ContentChained<T> {
    type Output;

    fn content_chained(self, x: T) -> Self::Output;
}

/// Computes the primitive part of a value: the element of the underlying integer lattice, with
/// coprime coordinates, that the value is the [`Content`] times.
///
/// $$
/// x = \operatorname{cont}(x) \operatorname{pp}(x).
/// $$
///
/// No sign or unit is chosen, so the primitive part of $-x$ is the negative of the primitive part
/// of $x$. The primitive part of zero is zero. For the representative that is canonical under
/// multiplication by units, see [`CanonicalPrimitivePart`].
pub trait PrimitivePart {
    type Output;

    fn primitive_part(self) -> Self::Output;
}

/// Replaces a value with its primitive part.
///
/// See [`PrimitivePart`].
pub trait PrimitivePartAssign {
    fn primitive_part_assign(&mut self);
}

/// Computes the content and the primitive part of a value together.
///
/// The primitive part is found by dividing by the content, so computing both at once finds the
/// content only once. See [`Content`] and [`PrimitivePart`].
pub trait ContentAndPrimitivePart {
    type Content;
    type PrimitivePart;

    fn content_and_primitive_part(self) -> (Self::Content, Self::PrimitivePart);
}

/// Computes the canonical primitive part of a value: its [`PrimitivePart`], replaced by the
/// canonical associate under multiplication by units, as [`CanonicalizeUnit`] chooses it.
///
/// $$
/// x = u \operatorname{cont}(x) \operatorname{cpp}(x),
/// $$
///
/// where $u$ is a unit. Each implementation documents its choice of associate: for a polynomial
/// with integer or rational coefficients it is the one with a non-negative leading coefficient, for
/// a vector it is the one whose first nonzero element is positive, and for a Gaussian rational it
/// is the one whose argument lies in $(-\pi/4, \pi/4]$. Values with no negative coordinates have
/// only one associate to choose from, so for them this is the same as [`PrimitivePart`]. The
/// canonical primitive part of zero is zero.
pub trait CanonicalPrimitivePart {
    type Output;

    fn canonical_primitive_part(self) -> Self::Output;
}

/// Replaces a value with its canonical primitive part.
///
/// See [`CanonicalPrimitivePart`].
pub trait CanonicalPrimitivePartAssign {
    fn canonical_primitive_part_assign(&mut self);
}

/// Computes the content and the canonical primitive part of a value together.
///
/// The canonical primitive part is found by dividing by the content, so computing both at once
/// finds the content only once. See [`Content`] and [`CanonicalPrimitivePart`].
pub trait ContentAndCanonicalPrimitivePart {
    type Content;
    type CanonicalPrimitivePart;

    fn content_and_canonical_primitive_part(self) -> (Self::Content, Self::CanonicalPrimitivePart);
}

/// Computes the Kronecker product of two matrices: the block matrix whose block in row $i$ and
/// column $j$ is the second matrix multiplied by the element of the first in row $i$ and column
/// $j$.
///
/// The Kronecker product of an $m \times n$ matrix and a $p \times q$ matrix is an $mp \times nq$
/// matrix.
pub trait KroneckerProduct<RHS = Self> {
    type Output;

    fn kronecker_product(self, other: RHS) -> Self::Output;
}

/// Replaces a matrix with its Kronecker product with another matrix: the block matrix whose block
/// in row $i$ and column $j$ is the second matrix multiplied by the element of the first in row $i$
/// and column $j$.
pub trait KroneckerProductAssign<RHS = Self> {
    fn kronecker_product_assign(&mut self, other: RHS);
}

/// Finds the inverse of a square matrix, returning `None` if the matrix is singular.
pub trait Inverse {
    type Output;

    fn inverse(self) -> Option<Self::Output>;
}

/// Replaces a number with its negative. Assumes the result is representable.
pub trait NegAssign {
    fn neg_assign(&mut self);
}

/// Returns the smallest power of 2 greater than or equal to a number. Assumes the result is
/// representable.
pub trait NextPowerOf2 {
    type Output;

    fn next_power_of_2(self) -> Self::Output;
}

/// Replaces a number with the smallest power of 2 greater than or equal it. Assumes the result is
/// representable.
pub trait NextPowerOf2Assign {
    fn next_power_of_2_assign(&mut self);
}

/// Takes the absolute value of a number.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// occurred. If an overflow occurred, then the wrapped number is returned.
pub trait OverflowingAbs {
    type Output;

    fn overflowing_abs(self) -> (Self::Output, bool);
}

/// Replaces a number with its absolute value.
///
/// Returns a boolean indicating whether an arithmetic overflow occurred. If an overflow occurred,
/// then the wrapped number is assigned.
pub trait OverflowingAbsAssign {
    fn overflowing_abs_assign(&mut self) -> bool;
}

/// Adds two numbers.
///
/// Returns a tuple of the sum along with a boolean indicating whether an arithmetic overflow
/// occurred. If an overflow occurred, then the wrapped number is returned.
pub trait OverflowingAdd<RHS = Self> {
    type Output;

    fn overflowing_add(self, other: RHS) -> (Self::Output, bool);
}

/// Adds a number to another number in place.
///
/// Returns a boolean indicating whether an arithmetic overflow occurred. If an overflow occurred,
/// then the wrapped number is assigned.
pub trait OverflowingAddAssign<RHS = Self> {
    fn overflowing_add_assign(&mut self, other: RHS) -> bool;
}

/// Adds a number and the product of two other numbers.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// occurred. If an overflow occurred, then the wrapped number is returned.
pub trait OverflowingAddMul<Y = Self, Z = Self> {
    type Output;

    fn overflowing_add_mul(self, y: Y, z: Z) -> (Self::Output, bool);
}

/// Adds a number and the product of two other numbers, in place.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// occurred. If an overflow occurred, then the wrapped number is returned.
pub trait OverflowingAddMulAssign<Y = Self, Z = Self> {
    fn overflowing_add_mul_assign(&mut self, y: Y, z: Z) -> bool;
}

/// Adds the products of two pairs of numbers.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// occurred. If an overflow occurred, then the wrapped result is returned.
pub trait OverflowingMulAddMul<Y = Self, Z = Self, W = Self> {
    type Output;

    fn overflowing_mul_add_mul(self, y: Y, z: Z, w: W) -> (Self::Output, bool);
}

/// Adds the products of two pairs of numbers, in place.
///
/// Returns a boolean indicating whether an arithmetic overflow occurred. If an overflow occurred,
/// then the wrapped result is assigned.
pub trait OverflowingMulAddMulAssign<Y = Self, Z = Self, W = Self> {
    fn overflowing_mul_add_mul_assign(&mut self, y: Y, z: Z, w: W) -> bool;
}

/// Subtracts the product of one pair of numbers from the product of another.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// occurred. If an overflow occurred, then the wrapped result is returned.
pub trait OverflowingMulSubMul<Y = Self, Z = Self, W = Self> {
    type Output;

    fn overflowing_mul_sub_mul(self, y: Y, z: Z, w: W) -> (Self::Output, bool);
}

/// Subtracts the product of one pair of numbers from the product of another, in place.
///
/// Returns a boolean indicating whether an arithmetic overflow occurred. If an overflow occurred,
/// then the wrapped result is assigned.
pub trait OverflowingMulSubMulAssign<Y = Self, Z = Self, W = Self> {
    fn overflowing_mul_sub_mul_assign(&mut self, y: Y, z: Z, w: W) -> bool;
}

/// Divides two numbers.
///
/// Returns a tuple of the sum along with a boolean indicating whether an arithmetic overflow
/// occurred. If an overflow occurred, then the wrapped number is returned.
pub trait OverflowingDiv<RHS = Self> {
    type Output;

    fn overflowing_div(self, other: RHS) -> (Self::Output, bool);
}

/// Divides a number by another number in place.
///
/// Returns a boolean indicating whether an arithmetic overflow occurred. If an overflow occurred,
/// then the wrapped number is assigned.
pub trait OverflowingDivAssign<RHS = Self> {
    fn overflowing_div_assign(&mut self, other: RHS) -> bool;
}

/// Multiplies two numbers.
///
/// Returns a tuple of the sum along with a boolean indicating whether an arithmetic overflow
/// occurred. If an overflow occurred, then the wrapped number is returned.
pub trait OverflowingMul<RHS = Self> {
    type Output;

    fn overflowing_mul(self, other: RHS) -> (Self::Output, bool);
}

/// Multiplies a number by another number in place.
///
/// Returns a boolean indicating whether an arithmetic overflow occurred. If an overflow occurred,
/// then the wrapped number is assigned.
pub trait OverflowingMulAssign<RHS = Self> {
    fn overflowing_mul_assign(&mut self, other: RHS) -> bool;
}

/// Negates a number.
///
/// Returns a tuple of the sum along with a boolean indicating whether an arithmetic overflow
/// occurred. If an overflow occurred, then the wrapped number is returned.
pub trait OverflowingNeg {
    type Output;

    fn overflowing_neg(self) -> (Self::Output, bool);
}

/// Negates a number in place.
///
/// Returns a boolean indicating whether an arithmetic overflow occurred. If an overflow occurred,
/// then the wrapped number is assigned.
pub trait OverflowingNegAssign {
    fn overflowing_neg_assign(&mut self) -> bool;
}

/// Raises a number to a power.
///
/// Returns a tuple of the sum along with a boolean indicating whether an arithmetic overflow
/// occurred. If an overflow occurred, then the wrapped number is returned.
pub trait OverflowingPow<RHS> {
    type Output;

    fn overflowing_pow(self, exp: RHS) -> (Self::Output, bool);
}

/// Raises a number to a power in place.
///
/// Returns a boolean indicating whether an arithmetic overflow occurred. If an overflow occurred,
/// then the wrapped number is assigned.
pub trait OverflowingPowAssign<RHS = Self> {
    fn overflowing_pow_assign(&mut self, exp: RHS) -> bool;
}

/// Squares a number.
///
/// Returns a tuple of the sum along with a boolean indicating whether an arithmetic overflow
/// occurred. If an overflow occurred, then the wrapped number is returned.
pub trait OverflowingSquare {
    type Output;

    fn overflowing_square(self) -> (Self::Output, bool);
}

/// Squares a number in place.
///
/// Returns a boolean indicating whether an arithmetic overflow occurred. If an overflow occurred,
/// then the wrapped number is assigned.
pub trait OverflowingSquareAssign {
    fn overflowing_square_assign(&mut self) -> bool;
}

/// Subtracts two numbers.
///
/// Returns a tuple of the sum along with a boolean indicating whether an arithmetic overflow
/// occurred. If an overflow occurred, then the wrapped number is returned.
pub trait OverflowingSub<RHS = Self> {
    type Output;

    fn overflowing_sub(self, other: RHS) -> (Self::Output, bool);
}

/// Subtracts a number by another number in place.
///
/// Returns a boolean indicating whether an arithmetic overflow occurred. If an overflow occurred,
/// then the wrapped number is assigned.
pub trait OverflowingSubAssign<RHS = Self> {
    fn overflowing_sub_assign(&mut self, other: RHS) -> bool;
}

/// Subtracts a number by the product of two other numbers.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// occurred. If an overflow occurred, then the wrapped number is returned.
pub trait OverflowingSubMul<Y = Self, Z = Self> {
    type Output;

    fn overflowing_sub_mul(self, y: Y, z: Z) -> (Self::Output, bool);
}

/// Subtracts a number by the product of two other numbers, in place.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// occurred. If an overflow occurred, then the wrapped number is returned.
pub trait OverflowingSubMulAssign<Y = Self, Z = Self> {
    fn overflowing_sub_mul_assign(&mut self, y: Y, z: Z) -> bool;
}

/// Determines whether a number is even or odd.
pub trait Parity {
    /// Determines whether a number is even.
    fn even(self) -> bool;

    /// Determines whether a number is odd.
    fn odd(self) -> bool;
}

/// Raises a number to a power. Assumes the result is representable.
pub trait Pow<RHS> {
    type Output;

    fn pow(self, exp: RHS) -> Self::Output;
}

/// Raises a number to a power in place. Assumes the result is representable.
pub trait PowAssign<RHS = Self> {
    fn pow_assign(&mut self, exp: RHS);
}

/// Raises 2 to a power.
pub trait PowerOf2<POW> {
    fn power_of_2(pow: POW) -> Self;
}

/// Replaces a number with 2 raised to the power of that number.
pub trait PowerOf2Assign {
    fn power_of_2_assign(&mut self);
}

/// Finds the reciprocal (multiplicative inverse) of a number.
pub trait Reciprocal {
    type Output;

    fn reciprocal(self) -> Self::Output;
}

/// Replaces a number with its reciprocal (multiplicative inverse).
pub trait ReciprocalAssign {
    fn reciprocal_assign(&mut self);
}

/// Takes the reciprocal of the square root of a number.
pub trait ReciprocalSqrt {
    type Output;

    fn reciprocal_sqrt(self) -> Self::Output;
}

/// Replaces a number with the reciprocal of its square root.
pub trait ReciprocalSqrtAssign {
    fn reciprocal_sqrt_assign(&mut self);
}

/// Finds the floor of the $n$th root of a number.
pub trait FloorRoot<POW> {
    type Output;

    fn floor_root(self, pow: POW) -> Self::Output;
}

/// Replaces a number with the floor of its $n$th root.
pub trait FloorRootAssign<POW> {
    fn floor_root_assign(&mut self, pow: POW);
}

/// Finds the ceiling of the $n$th root of a number.
pub trait CeilingRoot<POW> {
    type Output;

    fn ceiling_root(self, pow: POW) -> Self::Output;
}

/// Replaces a number with the ceiling of its $n$th root.
pub trait CeilingRootAssign<POW> {
    fn ceiling_root_assign(&mut self, pow: POW);
}

/// Finds the $n$th root of a number, returning `None` if it is not a perfect $n$th power.
pub trait CheckedRoot<POW> {
    type Output;

    fn checked_root(self, pow: POW) -> Option<Self::Output>;
}

/// Finds the floor of the $n$th root of a number, returning both the root and the remainder.
pub trait RootRem<POW> {
    type RootOutput;
    type RemOutput;

    fn root_rem(self, exp: POW) -> (Self::RootOutput, Self::RemOutput);
}

/// Replaces a number with the floor of its $n$th root, returning the remainder.
pub trait RootAssignRem<POW> {
    type RemOutput;

    fn root_assign_rem(&mut self, exp: POW) -> Self::RemOutput;
}

/// Takes the $n$th root of a number.
pub trait Root<POW> {
    type Output;

    fn root(self, pow: POW) -> Self::Output;
}

/// Replaces a number with its $n$th root.
pub trait RootAssign<POW> {
    fn root_assign(&mut self, pow: POW);
}

/// Takes the cube root of a number.
pub trait Cbrt {
    type Output;

    fn cbrt(self) -> Self::Output;
}

/// Replaces a number with its cube root.
pub trait CbrtAssign {
    fn cbrt_assign(&mut self);
}

/// Rotates a number left, inserting the leftmost bits into the right end.
pub trait RotateLeft {
    type Output;

    fn rotate_left(self, n: u64) -> Self::Output;
}

/// Rotates a number left, inserting the leftmost bits into the right end, in place.
pub trait RotateLeftAssign {
    fn rotate_left_assign(&mut self, n: u64);
}

/// Rotates a number right, inserting the leftmost bits into the left end.
pub trait RotateRight {
    type Output;

    fn rotate_right(self, n: u64) -> Self::Output;
}

/// Rotates a number right, inserting the leftmost bits into the left end, in place.
pub trait RotateRightAssign {
    fn rotate_right_assign(&mut self, n: u64);
}

/// Rounds a number to a multiple of another number, according to a specified rounding mode. An
/// [`Ordering`] is also returned, indicating whether the returned value is less than, equal to, or
/// greater than the original value.
pub trait RoundToMultiple<RHS = Self> {
    type Output;

    fn round_to_multiple(self, other: RHS, rm: RoundingMode) -> (Self::Output, Ordering);
}

/// Rounds a number to a multiple of another number in place, according to a specified rounding
/// mode. [`Ordering`] is returned, indicating whether the returned value is less than, equal to, or
/// greater than the original value.
pub trait RoundToMultipleAssign<RHS = Self> {
    fn round_to_multiple_assign(&mut self, other: RHS, rm: RoundingMode) -> Ordering;
}

/// Rounds a number to a multiple of $2^k$, according to a specified rounding mode. An [`Ordering`]
/// is also returned, indicating whether the returned value is less than, equal to, or greater than
/// the original value.
pub trait RoundToMultipleOfPowerOf2<RHS> {
    type Output;

    fn round_to_multiple_of_power_of_2(
        self,
        pow: RHS,
        rm: RoundingMode,
    ) -> (Self::Output, Ordering);
}

/// Rounds a number to a multiple of $2^k$ in place, according to a specified rounding mode. An
/// [`Ordering`] is returned, indicating whether the returned value is less than, equal to, or
/// greater than the original value.
pub trait RoundToMultipleOfPowerOf2Assign<RHS> {
    fn round_to_multiple_of_power_of_2_assign(&mut self, pow: RHS, rm: RoundingMode) -> Ordering;
}

/// Takes the absolute value of a number, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingAbs {
    type Output;

    fn saturating_abs(self) -> Self::Output;
}

/// Replaces a number with its absolute value, saturating at the numeric bounds instead of
/// overflowing.
pub trait SaturatingAbsAssign {
    fn saturating_abs_assign(&mut self);
}

/// Adds two numbers, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingAdd<RHS = Self> {
    type Output;

    fn saturating_add(self, other: RHS) -> Self::Output;
}

/// Add a number to another number in place, saturating at the numeric bounds instead of
/// overflowing.
pub trait SaturatingAddAssign<RHS = Self> {
    fn saturating_add_assign(&mut self, other: RHS);
}

/// Adds a number and the product of two other numbers, saturating at the numeric bounds instead of
/// overflowing.
pub trait SaturatingAddMul<Y = Self, Z = Self> {
    type Output;

    fn saturating_add_mul(self, y: Y, z: Z) -> Self::Output;
}

/// Adds a number and the product of two other numbers in place, saturating at the numeric bounds
/// instead of overflowing.
pub trait SaturatingAddMulAssign<Y = Self, Z = Self> {
    fn saturating_add_mul_assign(&mut self, y: Y, z: Z);
}

/// Adds the products of two pairs of numbers, saturating at the numeric bounds instead of
/// overflowing.
pub trait SaturatingMulAddMul<Y = Self, Z = Self, W = Self> {
    type Output;

    fn saturating_mul_add_mul(self, y: Y, z: Z, w: W) -> Self::Output;
}

/// Adds the products of two pairs of numbers, in place, saturating at the numeric bounds instead of
/// overflowing.
pub trait SaturatingMulAddMulAssign<Y = Self, Z = Self, W = Self> {
    fn saturating_mul_add_mul_assign(&mut self, y: Y, z: Z, w: W);
}

/// Subtracts the product of one pair of numbers from the product of another, saturating at the
/// numeric bounds instead of overflowing.
pub trait SaturatingMulSubMul<Y = Self, Z = Self, W = Self> {
    type Output;

    fn saturating_mul_sub_mul(self, y: Y, z: Z, w: W) -> Self::Output;
}

/// Subtracts the product of one pair of numbers from the product of another, in place, saturating
/// at the numeric bounds instead of overflowing.
pub trait SaturatingMulSubMulAssign<Y = Self, Z = Self, W = Self> {
    fn saturating_mul_sub_mul_assign(&mut self, y: Y, z: Z, w: W);
}

/// Multiplies two numbers, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingMul<RHS = Self> {
    type Output;

    fn saturating_mul(self, other: RHS) -> Self::Output;
}

/// Multiplies a number by another number in place, saturating at the numeric bounds instead of
/// overflowing.
pub trait SaturatingMulAssign<RHS = Self> {
    fn saturating_mul_assign(&mut self, other: RHS);
}

/// Negates a number, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingNeg {
    type Output;

    fn saturating_neg(self) -> Self::Output;
}

/// Negates a number in place, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingNegAssign {
    fn saturating_neg_assign(&mut self);
}

/// Raises a number to a power, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingPow<RHS> {
    type Output;

    fn saturating_pow(self, exp: RHS) -> Self::Output;
}

/// Raises a number to a power in place, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingPowAssign<RHS = Self> {
    fn saturating_pow_assign(&mut self, exp: RHS);
}

/// Squares a number, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingSquare {
    type Output;

    fn saturating_square(self) -> Self::Output;
}

/// Squares a number in place, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingSquareAssign {
    fn saturating_square_assign(&mut self);
}

/// Subtracts two numbers, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingSub<RHS = Self> {
    type Output;

    fn saturating_sub(self, other: RHS) -> Self::Output;
}

/// Subtracts a number by another number in place, saturating at the numeric bounds instead of
/// overflowing.
pub trait SaturatingSubAssign<RHS = Self> {
    fn saturating_sub_assign(&mut self, other: RHS);
}

/// Subtracts a number by the product of two other numbers, saturating at the numeric bounds instead
/// of overflowing.
pub trait SaturatingSubMul<Y = Self, Z = Self> {
    type Output;

    fn saturating_sub_mul(self, y: Y, z: Z) -> Self::Output;
}

/// Subtracts a number by the product of two other numbers in place, saturating at the numeric
/// bounds instead of overflowing.
pub trait SaturatingSubMulAssign<Y = Self, Z = Self> {
    fn saturating_sub_mul_assign(&mut self, y: Y, z: Z);
}

/// Left-shifts a number (multiplies it by a power of 2), rounding the result according to a
/// specified rounding mode. An [`Ordering`] is also returned, indicating whether the returned value
/// is less than, equal to, or greater than the exact value.
///
/// Rounding might only be necessary if `other` is negative.
pub trait ShlRound<RHS> {
    type Output;

    fn shl_round(self, other: RHS, rm: RoundingMode) -> (Self::Output, Ordering);
}

/// Left-shifts a number (multiplies it by a power of 2) in place, rounding the result according to
/// a specified rounding mode. An [`Ordering`] is also returned, indicating whether the assigned
/// value is less than, equal to, or greater than the exact value.
///
/// Rounding might only be necessary if `other` is negative.
pub trait ShlRoundAssign<RHS> {
    fn shl_round_assign(&mut self, other: RHS, rm: RoundingMode) -> Ordering;
}

/// Right-shifts a number (divides it by a power of 2), rounding the result according to a specified
/// rounding mode. An [`Ordering`] is also returned, indicating whether the returned value is less
/// than, equal to, or greater than the exact value.
///
/// Rounding might only be necessary if `other` is positive.
pub trait ShrRound<RHS> {
    type Output;

    fn shr_round(self, other: RHS, rm: RoundingMode) -> (Self::Output, Ordering);
}

/// Right-shifts a number (divides it by a power of 2) in place, rounding the result according to a
/// specified rounding mode. An [`Ordering`] is also returned, indicating whether the assigned value
/// is less than, equal to, or greater than the exact value.
///
/// Rounding might only be necessary if `other` is positive.
pub trait ShrRoundAssign<RHS> {
    fn shr_round_assign(&mut self, other: RHS, rm: RoundingMode) -> Ordering;
}

/// Right-shifts a number (divides it by a power of 2) that is itself an approximation of an exact
/// value, rounding the result according to a specified rounding mode. The approximation's
/// [`Ordering`] against the exact value is passed in, and an [`Ordering`] is returned, indicating
/// whether the returned value is less than, equal to, or greater than the shifted exact value.
///
/// The exact value is taken to lie just above the number if the passed-in [`Ordering`] is `Less`,
/// just below it if `Greater`, and to equal it if `Equal`. This is what is needed to round a value
/// a second time, to a coarser grid, without double-rounding error.
pub trait ShrRoundWithOrdering {
    type Output;

    fn shr_round_with_ordering(
        self,
        bits: u64,
        rm: RoundingMode,
        o: Ordering,
    ) -> (Self::Output, Ordering);
}

/// Right-shifts a number (divides it by a power of 2) in place, where the number is itself an
/// approximation of an exact value, rounding the result according to a specified rounding mode. The
/// approximation's [`Ordering`] against the exact value is passed in, and an [`Ordering`] is
/// returned, indicating whether the assigned value is less than, equal to, or greater than the
/// shifted exact value.
///
/// The exact value is taken to lie just above the number if the passed-in [`Ordering`] is `Less`,
/// just below it if `Greater`, and to equal it if `Equal`.
pub trait ShrRoundWithOrderingAssign {
    fn shr_round_with_ordering_assign(
        &mut self,
        bits: u64,
        rm: RoundingMode,
        o: Ordering,
    ) -> Ordering;
}

/// Returns `Greater`, `Equal`, or `Less`, depending on whether a number is positive, zero, or
/// negative, respectively.
pub trait Sign {
    fn sign(&self) -> Ordering;
}

/// Takes the square root of a number.
pub trait Sqrt {
    type Output;

    fn sqrt(self) -> Self::Output;
}

/// Replaces a number with its square root.
pub trait SqrtAssign {
    fn sqrt_assign(&mut self);
}

/// Finds the floor of the square root of a number.
pub trait FloorSqrt {
    type Output;

    fn floor_sqrt(self) -> Self::Output;
}

/// Replaces a number with the floor of its square root.
pub trait FloorSqrtAssign {
    fn floor_sqrt_assign(&mut self);
}

/// Finds the ceiling of the square root of a number.
pub trait CeilingSqrt {
    type Output;

    fn ceiling_sqrt(self) -> Self::Output;
}

/// Replaces a number with the ceiling of its square root.
pub trait CeilingSqrtAssign {
    fn ceiling_sqrt_assign(&mut self);
}

/// Finds the square root of a number, returning `None` if it is not a perfect square.
pub trait CheckedSqrt {
    type Output;

    fn checked_sqrt(self) -> Option<Self::Output>;
}

/// Finds the floor of the square root of a number, returning both the root and the remainder.
pub trait SqrtRem {
    type SqrtOutput;
    type RemOutput;

    fn sqrt_rem(self) -> (Self::SqrtOutput, Self::RemOutput);
}

/// Replaces a number with the floor of its square root, returning the remainder.
pub trait SqrtAssignRem {
    type RemOutput;

    fn sqrt_assign_rem(&mut self) -> Self::RemOutput;
}

/// Squares a number.
pub trait Square {
    type Output;

    fn square(self) -> Self::Output;
}

/// Replaces a number with its square.
pub trait SquareAssign {
    fn square_assign(&mut self);
}

/// Subtracts a number by the product of two other numbers.
///
/// Depending on the implementing type, the fused operation may compute the same value as the
/// unfused `self - y * z` more efficiently; or, for types with rounding, it may compute a *more
/// accurate* value -- the product enters the subtraction exactly, with a single rounding at the end
/// -- but *less* efficiently, since the exact product must be computed in full. See each
/// implementation's documentation for which contract it provides.
pub trait SubMul<Y = Self, Z = Self> {
    type Output;

    fn sub_mul(self, y: Y, z: Z) -> Self::Output;
}

/// Subtracts a number by the product of two other numbers, in place.
///
/// Depending on the implementing type, the fused operation may compute the same value as the
/// unfused `*self - y * z` more efficiently; or, for types with rounding, it may compute a *more
/// accurate* value -- the product enters the subtraction exactly, with a single rounding at the end
/// -- but *less* efficiently, since the exact product must be computed in full. See each
/// implementation's documentation for which contract it provides.
pub trait SubMulAssign<Y = Self, Z = Self> {
    fn sub_mul_assign(&mut self, y: Y, z: Z);
}

/// Subtracts the product of two numbers, shifted left by a number of bits, from a number: $x -
/// yz2^k$.
pub trait SubMulShl<Y = Self, Z = Self> {
    type Output;

    fn sub_mul_shl(self, y: Y, z: Z, bits: u64) -> Self::Output;
}

/// Subtracts the product of two numbers, shifted left by a number of bits, from a number in place:
/// $x \gets x - yz2^k$.
pub trait SubMulShlAssign<Y = Self, Z = Self> {
    fn sub_mul_shl_assign(&mut self, y: Y, z: Z, bits: u64);
}

/// Takes the absolute value of a number, wrapping around at the boundary of the type.
pub trait WrappingAbs {
    type Output;

    fn wrapping_abs(self) -> Self::Output;
}

/// Replaces a number with its absolute value, wrapping around at the boundary of the type.
pub trait WrappingAbsAssign {
    fn wrapping_abs_assign(&mut self);
}

/// Adds two numbers, wrapping around at the boundary of the type.
pub trait WrappingAdd<RHS = Self> {
    type Output;

    fn wrapping_add(self, other: RHS) -> Self::Output;
}

/// Adds a number to another number in place, wrapping around at the boundary of the type.
pub trait WrappingAddAssign<RHS = Self> {
    fn wrapping_add_assign(&mut self, other: RHS);
}

/// Adds a number and the product of two other numbers, wrapping around at the boundary of the type.
pub trait WrappingAddMul<Y = Self, Z = Self> {
    type Output;

    fn wrapping_add_mul(self, y: Y, z: Z) -> Self::Output;
}

/// Adds a number and the product of two other numbers, in place, wrapping around at the boundary of
/// the type.
pub trait WrappingAddMulAssign<Y = Self, Z = Self> {
    fn wrapping_add_mul_assign(&mut self, y: Y, z: Z);
}

/// Adds the products of two pairs of numbers, wrapping around at the boundary of the type.
pub trait WrappingMulAddMul<Y = Self, Z = Self, W = Self> {
    type Output;

    fn wrapping_mul_add_mul(self, y: Y, z: Z, w: W) -> Self::Output;
}

/// Adds the products of two pairs of numbers, in place, wrapping around at the boundary of the
/// type.
pub trait WrappingMulAddMulAssign<Y = Self, Z = Self, W = Self> {
    fn wrapping_mul_add_mul_assign(&mut self, y: Y, z: Z, w: W);
}

/// Subtracts the product of one pair of numbers from the product of another, wrapping around at the
/// boundary of the type.
pub trait WrappingMulSubMul<Y = Self, Z = Self, W = Self> {
    type Output;

    fn wrapping_mul_sub_mul(self, y: Y, z: Z, w: W) -> Self::Output;
}

/// Subtracts the product of one pair of numbers from the product of another, in place, wrapping
/// around at the boundary of the type.
pub trait WrappingMulSubMulAssign<Y = Self, Z = Self, W = Self> {
    fn wrapping_mul_sub_mul_assign(&mut self, y: Y, z: Z, w: W);
}

/// Divides a number by another number, wrapping around at the boundary of the type.
pub trait WrappingDiv<RHS = Self> {
    type Output;

    fn wrapping_div(self, other: RHS) -> Self::Output;
}

/// Divides a number by another number in place, wrapping around at the boundary of the type.
pub trait WrappingDivAssign<RHS = Self> {
    fn wrapping_div_assign(&mut self, other: RHS);
}

/// Multiplies two numbers, wrapping around at the boundary of the type.
pub trait WrappingMul<RHS = Self> {
    type Output;

    fn wrapping_mul(self, other: RHS) -> Self::Output;
}

/// Multiplies a number by another number in place, wrapping around at the boundary of the type.
pub trait WrappingMulAssign<RHS = Self> {
    fn wrapping_mul_assign(&mut self, other: RHS);
}

/// Negates a number, wrapping around at the boundary of the type.
pub trait WrappingNeg {
    type Output;

    fn wrapping_neg(self) -> Self::Output;
}

/// Negates a number in place, wrapping around at the boundary of the type.
pub trait WrappingNegAssign {
    fn wrapping_neg_assign(&mut self);
}

/// Raises a number to a power, wrapping around at the boundary of the type.
pub trait WrappingPow<RHS> {
    type Output;

    fn wrapping_pow(self, exp: RHS) -> Self::Output;
}

/// Raises a number to a power in place, wrapping around at the boundary of the type.
pub trait WrappingPowAssign<RHS = Self> {
    fn wrapping_pow_assign(&mut self, exp: RHS);
}

/// Squares a number, wrapping around at the boundary of the type.
pub trait WrappingSquare {
    type Output;

    fn wrapping_square(self) -> Self::Output;
}

/// Squares a number in place, wrapping around at the boundary of the type.
pub trait WrappingSquareAssign {
    fn wrapping_square_assign(&mut self);
}

/// Subtracts two numbers, wrapping around at the boundary of the type.
pub trait WrappingSub<RHS = Self> {
    type Output;

    fn wrapping_sub(self, other: RHS) -> Self::Output;
}

/// Subtracts a number by another number in place, wrapping around at the boundary of the type.
pub trait WrappingSubAssign<RHS = Self> {
    fn wrapping_sub_assign(&mut self, other: RHS);
}

/// Subtracts a number by the product of two other numbers, wrapping around at the boundary of the
/// type.
pub trait WrappingSubMul<Y = Self, Z = Self> {
    type Output;

    fn wrapping_sub_mul(self, y: Y, z: Z) -> Self::Output;
}

/// Subtracts a number by the product of two other numbers, in place, wrapping around at the
/// boundary of the type.
pub trait WrappingSubMulAssign<Y = Self, Z = Self> {
    fn wrapping_sub_mul_assign(&mut self, y: Y, z: Z);
}

/// Multiplies two numbers, returning the product as a pair of `Self` values.
///
/// The more significant number always comes first.
pub trait XMulYToZZ: Sized {
    fn x_mul_y_to_zz(x: Self, y: Self) -> (Self, Self);
}

/// Adds two numbers, each composed of two `Self` values, returning the sum as a pair of `Self`
/// values.
///
/// The more significant number always comes first. Addition is wrapping, and overflow is not
/// indicated.
pub trait XXAddYYToZZ: Sized {
    fn xx_add_yy_to_zz(x_1: Self, x_0: Self, y_1: Self, y_0: Self) -> (Self, Self);
}

/// Computes the quotient and remainder of two numbers. The first is composed of two `Self` values,
/// and the second of a single one.
///
/// `x_1` must be less than `y`.
pub trait XXDivModYToQR: Sized {
    fn xx_div_mod_y_to_qr(x_1: Self, x_0: Self, y: Self) -> (Self, Self);
}

/// Subtracts two numbers, each composed of two `Self` values, returing the difference as a pair of
/// `Self` values.
///
/// The more significant number always comes first. Subtraction is wrapping, and overflow is not
/// indicated.
pub trait XXSubYYToZZ: Sized {
    fn xx_sub_yy_to_zz(x_1: Self, x_0: Self, y_1: Self, y_0: Self) -> (Self, Self);
}

/// Adds two numbers, each composed of three `Self` values, returning the sum as a triple of `Self`
/// values.
///
/// The more significant number always comes first. Addition is wrapping, and overflow is not
/// indicated.
pub trait XXXAddYYYToZZZ: Sized {
    fn xxx_add_yyy_to_zzz(
        x_2: Self,
        x_1: Self,
        x_0: Self,
        y_2: Self,
        y_1: Self,
        y_0: Self,
    ) -> (Self, Self, Self);
}

/// Subtracts two numbers, each composed of three `Self` values, returing the difference as a triple
/// of `Self` values.
///
/// The more significant number always comes first. Subtraction is wrapping, and overflow is not
/// indicated.
pub trait XXXSubYYYToZZZ: Sized {
    fn xxx_sub_yyy_to_zzz(
        x_2: Self,
        x_1: Self,
        x_0: Self,
        y_2: Self,
        y_1: Self,
        y_0: Self,
    ) -> (Self, Self, Self);
}

/// Adds two numbers, each composed of four `Self` values, returning the sum as a quadruple of
/// `Self` values.
///
/// The more significant number always comes first. Addition is wrapping, and overflow is not
/// indicated.
pub trait XXXXAddYYYYToZZZZ: Sized {
    #[allow(clippy::too_many_arguments)]
    fn xxxx_add_yyyy_to_zzzz(
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
