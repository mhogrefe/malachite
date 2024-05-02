// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Two;
use crate::rounding_modes::RoundingMode;
use core::cmp::Ordering;

/// Takes the absolute value of a number. Assumes that the number has a representable absolute
/// number.
pub trait Abs {
    type Output;

    fn abs(self) -> Self::Output;
}

/// Replaces a number with its absolute value. Assumes that the number has a representable absolute
/// number.
pub trait AbsAssign {
    fn abs_assign(&mut self);
}

/// Takes the absolute value of a number and converts to the unsigned equivalent.
pub trait UnsignedAbs {
    type Output;

    fn unsigned_abs(self) -> Self::Output;
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
pub trait AddMul<Y = Self, Z = Self> {
    type Output;

    fn add_mul(self, y: Y, z: Z) -> Self::Output;
}

/// Adds a number and the product of two other numbers, in place.
pub trait AddMulAssign<Y = Self, Z = Self> {
    fn add_mul_assign(&mut self, y: Y, z: Z);
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

pub trait BinomialCoefficient<T = Self> {
    fn binomial_coefficient(n: T, k: T) -> Self;
}

pub trait CheckedBinomialCoefficient<T = Self>: Sized {
    fn checked_binomial_coefficient(n: T, k: T) -> Option<Self>;
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

/// Determines whether two numbers are coprime.
pub trait CoprimeWith<RHS = Self> {
    fn coprime_with(self, other: RHS) -> bool;
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

/// Divides a number by another number in place, returning the remainder. The quotient is rounded
/// towards negative infinity, and the remainder has the same sign as the divisor (second input).
///
/// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
pub trait DivAssignMod<RHS = Self> {
    type ModOutput;

    fn div_assign_mod(&mut self, other: RHS) -> Self::ModOutput;
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

/// Determines whether a number is divisible by $2^k$.
pub trait DivisibleByPowerOf2 {
    fn divisible_by_power_of_2(self, pow: u64) -> bool;
}

/// Determines whether a number is divisible by another number.
pub trait DivisibleBy<RHS = Self> {
    fn divisible_by(self, other: RHS) -> bool;
}

/// Determines whether a number is equivalent to another number modulo $2^k$.
pub trait EqModPowerOf2<RHS = Self> {
    fn eq_mod_power_of_2(self, other: RHS, pow: u64) -> bool;
}

/// Determines whether a number is equivalent to another number modulo $m$.
pub trait EqMod<RHS = Self, M = Self> {
    fn eq_mod(self, other: RHS, m: M) -> bool;
}

/// Computes the GCD (greatest common divisor) of two numbers $a$ and $b$, and also the coefficients
/// $x$ and $y$ in Bézout's identity $ax+by=\gcd(a,b)$.
///
/// The are infinitely many $x$, $y$ that satisfy the identity, so the full specification is more
/// detailed:
///
/// - $f(0, 0) = (0, 0, 0)$.
/// - $f(a, ak) = (a, 1, 0)$ if $a > 0$ and $k \neq 1$.
/// - $f(a, ak) = (-a, -1, 0)$ if $a < 0$ and $k \neq 1$.
/// - $f(bk, b) = (b, 0, 1)$ if $b > 0$.
/// - $f(bk, b) = (-b, 0, -1)$ if $b < 0$.
/// - $f(a, b) = (g, x, y)$ if $a \neq 0$ and $b \neq 0$ and $\gcd(a, b) \neq \min(|a|, |b|)$, where
///   $g = \gcd(a, b) \geq 0$, $ax + by = g$, $x \leq \lfloor b/g \rfloor$, and $y \leq \lfloor a/g
///   \rfloor$.
pub trait ExtendedGcd<RHS = Self> {
    type Gcd;
    type Cofactor;

    fn extended_gcd(self, other: RHS) -> (Self::Gcd, Self::Cofactor, Self::Cofactor);
}

/// Computes the factorial of a `u64`.
pub trait Factorial {
    fn factorial(n: u64) -> Self;
}

/// Computes the factorial of a `u64`, returning `None` if the result is too large to be
/// represented.
pub trait CheckedFactorial: Sized {
    fn checked_factorial(n: u64) -> Option<Self>;
}

/// Computes the double factorial of a `u64`. The double factorial of a non-negative integer is the
/// product of all the positive integers that are less than or equal to it and have the same parity
/// as it.
pub trait DoubleFactorial {
    fn double_factorial(n: u64) -> Self;
}

/// Computes the double factorial of a `u64`, returning `None` if the result is too large to be
/// represented. The double factorial of a non-negative integer is the product of all the positive
/// integers that are less than or equal to it and have the same parity as it.
pub trait CheckedDoubleFactorial: Sized {
    fn checked_double_factorial(n: u64) -> Option<Self>;
}

/// Computes the $m$-multifactorial of a `u64`. The $m$-multifactorial of a non-negative integer $n$
/// is the product of all integers $k$ such that $0<k\leq n$ and $k\equiv n \pmod m$.
pub trait Multifactorial {
    fn multifactorial(n: u64, m: u64) -> Self;
}

/// Computes the $m$-multifactorial of a `u64`, returning `None` if the result is too large to be
/// represented. The $m$-multifactorial of a non-negative integer $n$ is the product of all integers
/// $k$ such that $0<k\leq n$ and $k\equiv n \pmod m$.
pub trait CheckedMultifactorial: Sized {
    fn checked_multifactorial(n: u64, m: u64) -> Option<Self>;
}

/// Computes the subfactorial of a `u64`. The subfactorial of a non-negative integer $n$ counts the
/// number of derangements of $n$ elements, which are the permutations in which no element is fixed.
pub trait Subfactorial {
    fn subfactorial(n: u64) -> Self;
}

/// Computes the subfactorial of a `u64`, returning `None` if the result is too large to be
/// represented. The subfactorial of a non-negative integer $n$ counts the number of derangements of
/// $n$ elements, which are the permutations in which no element is fixed.
pub trait CheckedSubfactorial: Sized {
    fn checked_subfactorial(n: u64) -> Option<Self>;
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

/// Calculates the GCD (greatest common divisor) of two numbers.
pub trait Gcd<RHS = Self> {
    type Output;

    fn gcd(self, other: RHS) -> Self::Output;
}

/// Replaces a number with the GCD (greatest common divisor) of it and another number.
pub trait GcdAssign<RHS = Self> {
    fn gcd_assign(&mut self, other: RHS);
}

/// Determines whether a number is an integer power of 2.
pub trait IsPowerOf2 {
    fn is_power_of_2(&self) -> bool;
}

/// Calculates the LCM (least common multiple) of two numbers.
pub trait Lcm<RHS = Self> {
    type Output;

    fn lcm(self, other: RHS) -> Self::Output;
}

/// Replaces a number with the LCM (least common multiple) of it and another number.
pub trait LcmAssign<RHS = Self> {
    fn lcm_assign(&mut self, other: RHS);
}

/// Takes the natural logarithm of a number.
pub trait Ln {
    type Output;

    fn ln(self) -> Self::Output;
}

/// Calculates the LCM (least common multiple) of two numbers, returning `None` if the result is not
/// representable.
pub trait CheckedLcm<RHS = Self> {
    type Output;

    fn checked_lcm(self, other: RHS) -> Option<Self::Output>;
}

/// Calculates the Legendre symbol of two numbers. Typically the implementations will be identical
/// to those of [`JacobiSymbol`].
pub trait LegendreSymbol<RHS = Self> {
    fn legendre_symbol(self, other: RHS) -> i8;
}

/// Calculates the Jacobi symbol of two numbers.
pub trait JacobiSymbol<RHS = Self> {
    fn jacobi_symbol(self, other: RHS) -> i8;
}

/// Calculates the Kronecker symbol of two numbers.
pub trait KroneckerSymbol<RHS = Self> {
    fn kronecker_symbol(self, other: RHS) -> i8;
}

/// Calculates the base-$b$ logarithm of a number, or returns `None` if the number is not a perfect
/// power of $b$.
pub trait CheckedLogBase<B = Self> {
    type Output;

    fn checked_log_base(self, base: B) -> Option<Self::Output>;
}

/// Calculates the floor of the base-$b$ logarithm of a number.
pub trait FloorLogBase<B = Self> {
    type Output;

    fn floor_log_base(self, base: B) -> Self::Output;
}

/// Calculates the ceiling of the base-$b$ logarithm of a number.
pub trait CeilingLogBase<B = Self> {
    type Output;

    fn ceiling_log_base(self, base: B) -> Self::Output;
}

/// Calculates the base-2 logarithm of a number, or returns `None` if the number is not a perfect
/// power of 2.
pub trait CheckedLogBase2 {
    type Output;

    fn checked_log_base_2(self) -> Option<Self::Output>;
}

/// Calculates the floor of the base-2 logarithm of a number.
pub trait FloorLogBase2 {
    type Output;

    fn floor_log_base_2(self) -> Self::Output;
}

/// Calculates the ceiling of the base-2 logarithm of a number.
pub trait CeilingLogBase2 {
    type Output;

    fn ceiling_log_base_2(self) -> Self::Output;
}

/// Calculates the base-$2^k$ logarithm of a number, or returns `None` if the number is not a
/// perfect power of $2^k$.
pub trait CheckedLogBasePowerOf2<POW> {
    type Output;

    fn checked_log_base_power_of_2(self, pow: POW) -> Option<Self::Output>;
}

/// Calculates the floor of the base-$2^k$ logarithm of a number.
pub trait FloorLogBasePowerOf2<POW> {
    type Output;

    fn floor_log_base_power_of_2(self, pow: POW) -> Self::Output;
}

/// Calculates the ceiling of the base-$2^k$ logarithm of a number.
pub trait CeilingLogBasePowerOf2<POW> {
    type Output;

    fn ceiling_log_base_power_of_2(self, pow: POW) -> Self::Output;
}

/// Adds two numbers modulo a third number $m$. The inputs must be already reduced modulo $m$.
pub trait ModAdd<RHS = Self, M = Self> {
    type Output;

    fn mod_add(self, other: RHS, m: M) -> Self::Output;
}

/// Adds two numbers modulo a third number $m$, in place. The inputs must be already reduced modulo
/// $m$.
pub trait ModAddAssign<RHS = Self, M = Self> {
    fn mod_add_assign(&mut self, other: RHS, m: M);
}

/// Finds the multiplicative inverse of a number modulo another number $m$. The input must be
/// already reduced modulo $m$.
pub trait ModInverse<M = Self> {
    type Output;

    fn mod_inverse(self, m: M) -> Option<Self::Output>;
}

/// Checks whether a number is reduced modulo another number $m$.
pub trait ModIsReduced<M = Self> {
    fn mod_is_reduced(&self, m: &M) -> bool;
}

/// Multiplies two numbers modulo a third number $m$. The inputs must be already reduced modulo $m$.
pub trait ModMul<RHS = Self, M = Self> {
    type Output;

    fn mod_mul(self, other: RHS, m: M) -> Self::Output;
}

/// Multiplies two numbers modulo a third number $m$, in place. The inputs must be already reduced
/// modulo $m$.
pub trait ModMulAssign<RHS = Self, M = Self> {
    fn mod_mul_assign(&mut self, other: RHS, m: M);
}

/// Multiplies two numbers modulo a third number $m$. The inputs must be already reduced modulo $m$.
///
/// If multiple modular multiplications with the same modulus are necessary, it can be quicker to
/// precompute some piece of data and reuse it in the multiplication calls. This trait provides a
/// function for precomputing the data and a function for using it during multiplication.
pub trait ModMulPrecomputed<RHS = Self, M = Self> {
    type Output;
    type Data;

    /// Precomputes some data to use for modular multiplication.
    fn precompute_mod_mul_data(m: &M) -> Self::Data;

    fn mod_mul_precomputed(self, other: RHS, m: M, data: &Self::Data) -> Self::Output;
}

/// Multiplies two numbers modulo a third number $m$, in place.The inputs must be already reduced
/// modulo $m$.
///
/// If multiple modular multiplications with the same modulus are necessary, it can be quicker to
/// precompute some piece of data and reuse it in the multiplication calls. This trait provides a
/// function for using precomputed data during multiplication. For precomputing the data, use the
/// [`precompute_mod_mul_data`](ModMulPrecomputed::precompute_mod_mul_data) function in
/// [`ModMulPrecomputed`].
pub trait ModMulPrecomputedAssign<RHS = Self, M = Self>: ModMulPrecomputed<RHS, M> {
    fn mod_mul_precomputed_assign(&mut self, other: RHS, m: M, data: &Self::Data);
}

/// Negates a number modulo another number $m$. The input must be already reduced modulo $m$.
pub trait ModNeg<M = Self> {
    type Output;

    fn mod_neg(self, m: M) -> Self::Output;
}

/// Negates a number modulo another number $m$, in place. The input must be already reduced modulo
/// $m$.
pub trait ModNegAssign<M = Self> {
    fn mod_neg_assign(&mut self, m: M);
}

/// Divides a number by another number, returning just the remainder. The remainder has the same
/// sign as the divisor (second number).
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0 \leq
/// |r| < |y|$.
pub trait Mod<RHS = Self> {
    type Output;

    fn mod_op(self, other: RHS) -> Self::Output;
}

/// Divides a number by another number, replacing the first number by the remainder. The remainder
/// has the same sign as the divisor (second number).
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0 \leq
/// |r| < |y|$.
pub trait ModAssign<RHS = Self> {
    fn mod_assign(&mut self, other: RHS);
}

/// Divides the negative of a number by another number, returning the remainder.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = qy - r$ and $0 \leq
/// r < y$.
pub trait NegMod<RHS = Self> {
    type Output;

    fn neg_mod(self, other: RHS) -> Self::Output;
}

/// Divides the negative of a number by another number, replacing the first number by the remainder.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = qy - r$ and $0 \leq
/// r < y$.
pub trait NegModAssign<RHS = Self> {
    fn neg_mod_assign(&mut self, other: RHS);
}

/// Divides a number by another number, returning just the remainder. The remainder has the opposite
/// sign as the divisor (second number).
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0 \leq
/// |r| < |y|$.
pub trait CeilingMod<RHS = Self> {
    type Output;

    fn ceiling_mod(self, other: RHS) -> Self::Output;
}

/// Divides a number by another number, replacing the first number by the remainder. The remainder
/// has the same sign as the divisor (second number).
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0 \leq
/// |r| < |y|$.
pub trait CeilingModAssign<RHS = Self> {
    fn ceiling_mod_assign(&mut self, other: RHS);
}

/// Raises a number to a power modulo another number $m$. The base must be already reduced modulo
/// $m$.
pub trait ModPow<RHS = Self, M = Self> {
    type Output;

    fn mod_pow(self, exp: RHS, m: M) -> Self::Output;
}

/// Raises a number to a power modulo another number $m$, in place. The base must be already reduced
/// modulo $m$.
pub trait ModPowAssign<RHS = Self, M = Self> {
    fn mod_pow_assign(&mut self, exp: RHS, m: M);
}

/// Raises a number to a power modulo another number $m$. The base must be already reduced modulo
/// $m$.
///
/// If multiple modular exponentiations with the same modulus are necessary, it can be quicker to
/// precompute some piece of data and reuse it in the exponentiation calls. This trait provides a
/// function for precomputing the data and a function for using it during exponentiation.
pub trait ModPowPrecomputed<RHS = Self, M = Self>
where
    Self: Sized,
{
    type Output;
    type Data;

    /// Precomputes some data to use for modular exponentiation.
    fn precompute_mod_pow_data(m: &M) -> Self::Data;

    fn mod_pow_precomputed(self, exp: RHS, m: M, data: &Self::Data) -> Self::Output;
}

/// Raises a number to a power modulo another number $m$, in place. The base must be already reduced
/// modulo $m$.
///
/// If multiple modular exponentiations with the same modulus are necessary, it can be quicker to
/// precompute some piece of data and reuse it in the exponentiation calls. This trait provides a
/// function for using precomputed data during exponentiation. For precomputing the data, use the
/// [`precompute_mod_pow_data`](ModPowPrecomputed::precompute_mod_pow_data) function in
/// [`ModPowPrecomputed`].
pub trait ModPowPrecomputedAssign<RHS: Two = Self, M = Self>: ModPowPrecomputed<RHS, M> {
    fn mod_pow_precomputed_assign(&mut self, exp: RHS, m: M, data: &Self::Data);
}

/// Adds two numbers modulo $2^k$. The inputs must be already reduced modulo $2^k$.
pub trait ModPowerOf2Add<RHS = Self> {
    type Output;

    fn mod_power_of_2_add(self, other: RHS, pow: u64) -> Self::Output;
}

/// Adds two numbers modulo $2^k$, in place. The inputs must be already reduced modulo $2^k$.
pub trait ModPowerOf2AddAssign<RHS = Self> {
    fn mod_power_of_2_add_assign(&mut self, other: RHS, pow: u64);
}

/// Finds the multiplicative inverse of a number modulo $2^k$. The input must be already reduced
/// modulo $2^k$.
pub trait ModPowerOf2Inverse {
    type Output;

    fn mod_power_of_2_inverse(self, pow: u64) -> Option<Self::Output>;
}

/// Checks whether a number is reduced modulo $2^k$.
pub trait ModPowerOf2IsReduced {
    fn mod_power_of_2_is_reduced(&self, pow: u64) -> bool;
}

/// Multiplies two numbers modulo $2^k$. The inputs must be already reduced modulo $2^k$.
pub trait ModPowerOf2Mul<RHS = Self> {
    type Output;

    fn mod_power_of_2_mul(self, other: RHS, pow: u64) -> Self::Output;
}

/// Multiplies two numbers modulo $2^k$, in place. The inputs must be already reduced modulo $2^k$.
pub trait ModPowerOf2MulAssign<RHS = Self> {
    fn mod_power_of_2_mul_assign(&mut self, other: RHS, pow: u64);
}

/// Negates a number modulo $2^k$. The input must be already reduced modulo $2^k$.
pub trait ModPowerOf2Neg {
    type Output;

    fn mod_power_of_2_neg(self, pow: u64) -> Self::Output;
}

/// Negates a number modulo $2^k$ in place. The input must be already reduced modulo $2^k$.
pub trait ModPowerOf2NegAssign {
    fn mod_power_of_2_neg_assign(&mut self, pow: u64);
}

/// Raises a number to a power modulo $2^k$. The base must be already reduced modulo $2^k$.
pub trait ModPowerOf2Pow<RHS = Self> {
    type Output;

    fn mod_power_of_2_pow(self, exp: RHS, pow: u64) -> Self::Output;
}

/// Raises a number to a power modulo $2^k$, in place. The base must be already reduced modulo
/// $2^k$.
pub trait ModPowerOf2PowAssign<RHS = Self> {
    fn mod_power_of_2_pow_assign(&mut self, exp: RHS, pow: u64);
}

/// Left-shifts a number (multiplies it by a power of 2) modulo $2^k$. The number must be already
/// reduced modulo $2^k$.
pub trait ModPowerOf2Shl<RHS> {
    type Output;

    fn mod_power_of_2_shl(self, other: RHS, pow: u64) -> Self::Output;
}

/// Left-shifts a number (multiplies it by a power of 2) modulo $2^k$, in place. The number must be
/// already reduced modulo $2^k$.
pub trait ModPowerOf2ShlAssign<RHS> {
    fn mod_power_of_2_shl_assign(&mut self, other: RHS, pow: u64);
}

/// Right-shifts a number (divides it by a power of 2) modulo $2^k$. The number must be already
/// reduced modulo $2^k$.
pub trait ModPowerOf2Shr<RHS> {
    type Output;

    fn mod_power_of_2_shr(self, other: RHS, pow: u64) -> Self::Output;
}

/// Right-shifts a number (divides it by a power of 2) modulo $2^k$, in place. The number must be
/// already reduced modulo $2^k$.
pub trait ModPowerOf2ShrAssign<RHS> {
    fn mod_power_of_2_shr_assign(&mut self, other: RHS, pow: u64);
}

/// Squares a number modulo $2^k$. The input must be already reduced modulo $2^k$.
pub trait ModPowerOf2Square {
    type Output;

    fn mod_power_of_2_square(self, pow: u64) -> Self::Output;
}

/// Squares a number modulo $2^k$ in place. The input must be already reduced modulo $2^k$.
pub trait ModPowerOf2SquareAssign {
    fn mod_power_of_2_square_assign(&mut self, pow: u64);
}

/// Subtracts two numbers modulo $2^k$. The inputs must be already reduced modulo $2^k$.
pub trait ModPowerOf2Sub<RHS = Self> {
    type Output;

    fn mod_power_of_2_sub(self, other: RHS, pow: u64) -> Self::Output;
}

/// Subtracts two numbers modulo $2^k$, in place. The inputs must be already reduced modulo $2^k$.
pub trait ModPowerOf2SubAssign<RHS = Self> {
    fn mod_power_of_2_sub_assign(&mut self, other: RHS, pow: u64);
}

/// Divides a number by $2^k$, returning just the remainder. The remainder is non-negative.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k + r$ and $0
/// \leq r < 2^k$.
pub trait ModPowerOf2 {
    type Output;

    fn mod_power_of_2(self, other: u64) -> Self::Output;
}

/// Divides a number by $2^k$, replacing the number by the remainder. The remainder is non-negative.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k + r$ and $0
/// \leq r < 2^k$.
pub trait ModPowerOf2Assign {
    fn mod_power_of_2_assign(&mut self, other: u64);
}

/// Divides a number by $2^k$, returning just the remainder. The remainder has the same sign as the
/// number.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k + r$ and $0
/// \leq |r| < 2^k$.
pub trait RemPowerOf2 {
    type Output;

    fn rem_power_of_2(self, other: u64) -> Self::Output;
}

/// Divides a number by $2^k$, replacing the number by the remainder. The remainder has the same
/// sign as the number.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k + r$ and $0
/// \leq |r| < 2^k$.
pub trait RemPowerOf2Assign {
    fn rem_power_of_2_assign(&mut self, other: u64);
}

/// Divides the negative of a number by $2^k$, returning the remainder.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k - r$ and $0
/// \leq r < 2^k$.
pub trait NegModPowerOf2 {
    type Output;

    fn neg_mod_power_of_2(self, other: u64) -> Self::Output;
}

/// Divides the negative of a number by $2^k$, replacing the number by the remainder.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k - r$ and $0
/// \leq r < 2^k$.
pub trait NegModPowerOf2Assign {
    fn neg_mod_power_of_2_assign(&mut self, other: u64);
}

/// Divides a number by $2^k$, returning just the remainder. The remainder is non-positive.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k + r$ and $0
/// \leq -r < 2^k$.
pub trait CeilingModPowerOf2 {
    type Output;

    fn ceiling_mod_power_of_2(self, other: u64) -> Self::Output;
}

/// Divides a number by $2^k$, replacing the number by the remainder. The remainder is non-positive.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k + r$ and $0
/// \leq -r < 2^k$.
pub trait CeilingModPowerOf2Assign {
    fn ceiling_mod_power_of_2_assign(&mut self, other: u64);
}

/// Left-shifts a number (multiplies it by a power of 2) modulo another number $m$. The number must
/// be already reduced modulo $m$.
pub trait ModShl<RHS, M = Self> {
    type Output;

    fn mod_shl(self, other: RHS, m: M) -> Self::Output;
}

/// Left-shifts a number (multiplies it by a power of 2) modulo another number $m$, in place. The
/// number must be already reduced modulo $m$.
pub trait ModShlAssign<RHS, M = Self> {
    fn mod_shl_assign(&mut self, other: RHS, m: M);
}

/// Left-shifts a number (divides it by a power of 2) modulo another number $m$. The number must be
/// already reduced modulo $m$.
pub trait ModShr<RHS, M = Self> {
    type Output;

    fn mod_shr(self, other: RHS, m: M) -> Self::Output;
}

/// Left-shifts a number (divides it by a power of 2) modulo another number $m$, in place. The
/// number must be already reduced modulo $m$.
pub trait ModShrAssign<RHS, M = Self> {
    fn mod_shr_assign(&mut self, other: RHS, m: M);
}

/// Squares a number modulo another number $m$. The input must be already reduced modulo $m$.
pub trait ModSquare<M = Self> {
    type Output;

    fn mod_square(self, m: M) -> Self::Output;
}

/// Squares a number modulo another number $m$, in place. The input must be already reduced modulo
/// $m$.
pub trait ModSquareAssign<M = Self> {
    fn mod_square_assign(&mut self, m: M);
}

/// Squares a number modulo another number $m$. The input must be already reduced modulo $m$.
///
/// If multiple modular squarings with the same modulus are necessary, it can be quicker to
/// precompute some piece of data using
/// [`precompute_mod_pow_data`](ModPowPrecomputed::precompute_mod_pow_data) function in
/// [`ModMulPrecomputed`] and reuse it in the squaring calls.
pub trait ModSquarePrecomputed<RHS = Self, M = Self>: ModPowPrecomputed<RHS, M>
where
    Self: Sized,
{
    fn mod_square_precomputed(self, m: M, data: &Self::Data) -> Self::Output;
}

/// Squares a number modulo another number $m$, in place. The input must be already reduced modulo
/// $m$.
///
/// If multiple modular squarings with the same modulus are necessary, it can be quicker to
/// precompute some piece of data using
/// [`precompute_mod_pow_data`](ModPowPrecomputed::precompute_mod_pow_data) function in
/// [`ModMulPrecomputed`] and reuse it in the squaring calls.
pub trait ModSquarePrecomputedAssign<RHS = Self, M = Self>: ModPowPrecomputed<RHS, M> {
    fn mod_square_precomputed_assign(&mut self, m: M, data: &Self::Data);
}

/// Adds two numbers modulo a third number $m$. The inputs must be already reduced modulo $m$.
pub trait ModSub<RHS = Self, M = Self> {
    type Output;

    fn mod_sub(self, other: RHS, m: M) -> Self::Output;
}

/// Adds two numbers modulo a third number $m$, in place. The inputs must be already reduced modulo
/// $m$.
pub trait ModSubAssign<RHS = Self, M = Self> {
    fn mod_sub_assign(&mut self, other: RHS, m: M);
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
/// occured. If an overflow occurred, then the wrapped number is returned.
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

pub trait Primorial {
    fn primorial(n: u64) -> Self;

    fn product_of_first_n_primes(n: u64) -> Self;
}

pub trait CheckedPrimorial: Sized {
    fn checked_primorial(n: u64) -> Option<Self>;

    fn checked_product_of_first_n_primes(n: u64) -> Option<Self>;
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
pub trait SubMul<Y = Self, Z = Self> {
    type Output;

    fn sub_mul(self, y: Y, z: Z) -> Self::Output;
}

/// Subtracts a number by the product of two other numbers, in place.
pub trait SubMulAssign<Y = Self, Z = Self> {
    fn sub_mul_assign(&mut self, y: Y, z: Z);
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
