// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Two;

/// Combines two congruences by the Chinese remainder theorem, returning `None` if the moduli are
/// not coprime. The residues must be already reduced modulo their moduli.
pub trait Crt<M1 = Self, R2 = Self, M2 = Self> {
    type Output;

    fn crt(self, m1: M1, r2: R2, m2: M2) -> Option<Self::Output>;
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

/// Divides a number by another number modulo a third number $m$, returning `None` if no quotient
/// exists. The inputs must be already reduced modulo $m$.
///
/// If the divisor is not invertible modulo $m$, a quotient may exist without being unique; in that
/// case one of the quotients is returned.
pub trait ModDiv<RHS = Self, M = Self> {
    type Output;

    fn mod_div(self, other: RHS, m: M) -> Option<Self::Output>;
}

/// Finds all quotients of a number and another number modulo a third number $m$, returning `None`
/// if no quotient exists. The inputs must be already reduced modulo $m$.
///
/// The quotients form an arithmetic progression: `Some((start, stride, length))` means that the
/// quotients are exactly the numbers $\text{start} + \text{stride} \cdot i$ for $0 \leq i <
/// \text{length}$, where `start` is the smallest quotient.
pub trait ModDivList<RHS = Self, M = Self> {
    type Output;

    #[allow(clippy::type_complexity)]
    fn mod_div_list(self, other: RHS, m: M) -> Option<(Self::Output, Self::Output, Self::Output)>;
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

/// Adds a number and the product of two other numbers, modulo a fourth number $m$. All three inputs
/// must be already reduced modulo $m$.
pub trait ModAddMul<Y = Self, Z = Self, M = Self> {
    type Output;

    fn mod_add_mul(self, y: Y, z: Z, m: M) -> Self::Output;
}

/// Adds the product of two numbers to a number modulo a fourth number $m$, in place. All three
/// inputs must be already reduced modulo $m$.
pub trait ModAddMulAssign<Y = Self, Z = Self, M = Self> {
    fn mod_add_mul_assign(&mut self, y: Y, z: Z, m: M);
}

/// Subtracts the product of two numbers from a number, modulo a fourth number $m$. All three inputs
/// must be already reduced modulo $m$.
pub trait ModSubMul<Y = Self, Z = Self, M = Self> {
    type Output;

    fn mod_sub_mul(self, y: Y, z: Z, m: M) -> Self::Output;
}

/// Subtracts the product of two numbers from a number modulo a fourth number $m$, in place. All
/// three inputs must be already reduced modulo $m$.
pub trait ModSubMulAssign<Y = Self, Z = Self, M = Self> {
    fn mod_sub_mul_assign(&mut self, y: Y, z: Z, m: M);
}

/// Adds a number and the product of two other numbers, shifted left by a number of bits, modulo a
/// fourth number $m$: $x + yz2^b \bmod m$. All three inputs must be already reduced modulo $m$.
pub trait ModAddMulShl<Y = Self, Z = Self, M = Self> {
    type Output;

    fn mod_add_mul_shl(self, y: Y, z: Z, bits: u64, m: M) -> Self::Output;
}

/// Adds the product of two numbers, shifted left by a number of bits, to a number modulo a fourth
/// number $m$, in place. All three inputs must be already reduced modulo $m$.
pub trait ModAddMulShlAssign<Y = Self, Z = Self, M = Self> {
    fn mod_add_mul_shl_assign(&mut self, y: Y, z: Z, bits: u64, m: M);
}

/// Subtracts the product of two numbers, shifted left by a number of bits, from a number, modulo a
/// fourth number $m$: $x - yz2^b \bmod m$. All three inputs must be already reduced modulo $m$.
pub trait ModSubMulShl<Y = Self, Z = Self, M = Self> {
    type Output;

    fn mod_sub_mul_shl(self, y: Y, z: Z, bits: u64, m: M) -> Self::Output;
}

/// Subtracts the product of two numbers, shifted left by a number of bits, from a number modulo a
/// fourth number $m$, in place. All three inputs must be already reduced modulo $m$.
pub trait ModSubMulShlAssign<Y = Self, Z = Self, M = Self> {
    fn mod_sub_mul_shl_assign(&mut self, y: Y, z: Z, bits: u64, m: M);
}

/// Adds up the numbers in an iterator, modulo another number $m$. Every number must be already
/// reduced modulo $m$.
///
/// Like [`Sum`](core::iter::Sum), this is implemented both for iterators of values and for
/// iterators of references.
pub trait ModSum<A = Self, M = Self>: Sized {
    fn mod_sum<I: Iterator<Item = A>>(xs: I, m: M) -> Self;
}

/// Multiplies together the numbers in an iterator, modulo another number $m$. Every number must be
/// already reduced modulo $m$.
///
/// Like [`Product`](core::iter::Product), this is implemented both for iterators of values and for
/// iterators of references. The product of no numbers is 1 reduced modulo $m$, which is 0 when $m =
/// 1$.
pub trait ModProduct<A = Self, M = Self>: Sized {
    fn mod_product<I: Iterator<Item = A>>(xs: I, m: M) -> Self;
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

/// Divides a number by another number, returning the balanced remainder: the representative of the
/// first number modulo the second that is closest to zero.
///
/// The remainder $r$ satisfies $-|y|/2 < r \leq |y|/2$, so a remainder of exactly $|y|/2$ is
/// positive. It is congruent to $x$ modulo $y$, and those two properties determine it uniquely.
pub trait BalancedMod<RHS = Self> {
    type Output;

    fn balanced_mod(self, other: RHS) -> Self::Output;
}

/// Divides a number by another number, replacing the first number by the balanced remainder: the
/// representative of the first number modulo the second that is closest to zero.
///
/// The remainder $r$ satisfies $-|y|/2 < r \leq |y|/2$, so a remainder of exactly $|y|/2$ is
/// positive.
pub trait BalancedModAssign<RHS = Self> {
    fn balanced_mod_assign(&mut self, other: RHS);
}

/// Divides a number by another number, returning just the remainder. The remainder is always
/// nonnegative.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0 \leq
/// r < |y|$.
pub trait ModEuclidean<RHS = Self> {
    type Output;

    fn mod_euclidean(self, other: RHS) -> Self::Output;
}

/// Divides a number by another number, replacing the first number by the remainder. The remainder
/// is always nonnegative.
///
/// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0 \leq
/// r < |y|$.
pub trait ModEuclideanAssign<RHS = Self> {
    fn mod_euclidean_assign(&mut self, other: RHS);
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

/// Divides a number by another number modulo $2^k$, returning `None` if no quotient exists. The
/// inputs must be already reduced modulo $2^k$.
///
/// If the divisor is even, a quotient may exist without being unique; in that case the smallest
/// quotient is returned.
pub trait ModPowerOf2Div<RHS = Self> {
    type Output;

    fn mod_power_of_2_div(self, other: RHS, pow: u64) -> Option<Self::Output>;
}

/// Finds the multiplicative inverse of a number modulo $2^k$. The input must be already reduced
/// modulo $2^k$.
pub trait ModPowerOf2Inverse {
    type Output;

    fn mod_power_of_2_inverse(self, pow: u64) -> Option<Self::Output>;
}

/// Finds the least square root of a number modulo $2^k$, returning `None` if the number is not a
/// square modulo $2^k$. The input must be already reduced modulo $2^k$.
pub trait ModPowerOf2Sqrt {
    type Output;

    fn mod_power_of_2_sqrt(self, pow: u64) -> Option<Self::Output>;
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

/// Adds a number and the product of two other numbers, modulo $2^k$. All three inputs must be
/// already reduced modulo $2^k$.
pub trait ModPowerOf2AddMul<Y = Self, Z = Self> {
    type Output;

    fn mod_power_of_2_add_mul(self, y: Y, z: Z, pow: u64) -> Self::Output;
}

/// Adds the product of two numbers to a number modulo $2^k$, in place. All three inputs must be
/// already reduced modulo $2^k$.
pub trait ModPowerOf2AddMulAssign<Y = Self, Z = Self> {
    fn mod_power_of_2_add_mul_assign(&mut self, y: Y, z: Z, pow: u64);
}

/// Adds a number and the product of two other numbers, shifted left by a number of bits, modulo
/// $2^k$: $x + yz2^b \bmod 2^k$. All three inputs must be already reduced modulo $2^k$.
pub trait ModPowerOf2AddMulShl<Y = Self, Z = Self> {
    type Output;

    fn mod_power_of_2_add_mul_shl(self, y: Y, z: Z, bits: u64, pow: u64) -> Self::Output;
}

/// Adds the product of two numbers, shifted left by a number of bits, to a number modulo $2^k$, in
/// place. All three inputs must be already reduced modulo $2^k$.
pub trait ModPowerOf2AddMulShlAssign<Y = Self, Z = Self> {
    fn mod_power_of_2_add_mul_shl_assign(&mut self, y: Y, z: Z, bits: u64, pow: u64);
}

/// Subtracts the product of two numbers from a number, modulo $2^k$. All three inputs must be
/// already reduced modulo $2^k$.
pub trait ModPowerOf2SubMul<Y = Self, Z = Self> {
    type Output;

    fn mod_power_of_2_sub_mul(self, y: Y, z: Z, pow: u64) -> Self::Output;
}

/// Subtracts the product of two numbers from a number modulo $2^k$, in place. All three inputs must
/// be already reduced modulo $2^k$.
pub trait ModPowerOf2SubMulAssign<Y = Self, Z = Self> {
    fn mod_power_of_2_sub_mul_assign(&mut self, y: Y, z: Z, pow: u64);
}

/// Subtracts the product of two numbers, shifted left by a number of bits, from a number, modulo
/// $2^k$: $x - yz2^b \bmod 2^k$. All three inputs must be already reduced modulo $2^k$.
pub trait ModPowerOf2SubMulShl<Y = Self, Z = Self> {
    type Output;

    fn mod_power_of_2_sub_mul_shl(self, y: Y, z: Z, bits: u64, pow: u64) -> Self::Output;
}

/// Subtracts the product of two numbers, shifted left by a number of bits, from a number modulo
/// $2^k$, in place. All three inputs must be already reduced modulo $2^k$.
pub trait ModPowerOf2SubMulShlAssign<Y = Self, Z = Self> {
    fn mod_power_of_2_sub_mul_shl_assign(&mut self, y: Y, z: Z, bits: u64, pow: u64);
}

/// Adds up the numbers in an iterator, modulo $2^k$. Every number must be already reduced modulo
/// $2^k$.
///
/// Like [`Sum`](core::iter::Sum), this is implemented both for iterators of values and for
/// iterators of references.
pub trait ModPowerOf2Sum<A = Self>: Sized {
    fn mod_power_of_2_sum<I: Iterator<Item = A>>(xs: I, pow: u64) -> Self;
}

/// Multiplies together the numbers in an iterator, modulo $2^k$. Every number must be already
/// reduced modulo $2^k$.
///
/// Like [`Product`](core::iter::Product), this is implemented both for iterators of values and for
/// iterators of references. The product of no numbers is 1 reduced modulo $2^k$, which is 0 when $k
/// = 0$.
pub trait ModPowerOf2Product<A = Self>: Sized {
    fn mod_power_of_2_product<I: Iterator<Item = A>>(xs: I, pow: u64) -> Self;
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

/// Computes the squared absolute value of a number modulo $2^k$. The input must be already reduced
/// modulo $2^k$.
///
/// For a real number this is its square modulo $2^k$; the trait exists for the types where it is
/// not, such as complex numbers and vectors, for which it is the reduced sum of squares.
pub trait ModPowerOf2AbsSquared {
    type Output;

    fn mod_power_of_2_abs_squared(self, pow: u64) -> Self::Output;
}

/// Replaces a number with its squared absolute value modulo $2^k$. The input must be already
/// reduced modulo $2^k$.
pub trait ModPowerOf2AbsSquaredAssign {
    fn mod_power_of_2_abs_squared_assign(&mut self, pow: u64);
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

/// Divides a number by $2^k$, returning the balanced remainder: the representative of the number
/// modulo $2^k$ that is closest to zero.
///
/// The remainder $r$ satisfies $-2^{k-1} < r \leq 2^{k-1}$, so a remainder of exactly $2^{k-1}$ is
/// positive, as with [`BalancedMod`]; for $k = 0$ it is 0. It is congruent to $x$ modulo $2^k$, and
/// those two properties determine it uniquely.
pub trait BalancedModPowerOf2 {
    type Output;

    fn balanced_mod_power_of_2(self, pow: u64) -> Self::Output;
}

/// Divides a number by $2^k$, replacing the number by the balanced remainder: the representative of
/// the number modulo $2^k$ that is closest to zero.
///
/// The remainder $r$ satisfies $-2^{k-1} < r \leq 2^{k-1}$, so a remainder of exactly $2^{k-1}$ is
/// positive; for $k = 0$ it is 0.
pub trait BalancedModPowerOf2Assign {
    fn balanced_mod_power_of_2_assign(&mut self, pow: u64);
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

/// Right-shifts a number (divides it by a power of 2) modulo another number $m$. The number must be
/// already reduced modulo $m$.
pub trait ModShr<RHS, M = Self> {
    type Output;

    fn mod_shr(self, other: RHS, m: M) -> Self::Output;
}

/// Right-shifts a number (divides it by a power of 2) modulo another number $m$, in place. The
/// number must be already reduced modulo $m$.
pub trait ModShrAssign<RHS, M = Self> {
    fn mod_shr_assign(&mut self, other: RHS, m: M);
}

/// Computes a square root of a number modulo another number $m$, returning `None` if no root is
/// found. The input must be already reduced modulo $m$.
///
/// The modulus should be an odd prime: for such moduli a root is found whenever one exists. The
/// behavior for other moduli is deterministic and never hangs, but a root may be missed, and a
/// returned value may fail to be a root.
pub trait ModSqrt<M = Self> {
    type Output;

    fn mod_sqrt(self, m: M) -> Option<Self::Output>;
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

/// Computes the squared absolute value of a number modulo another number $m$. The input must be
/// already reduced modulo $m$.
///
/// For a real number this is its square modulo $m$; the trait exists for the types where it is not,
/// such as complex numbers and vectors, for which it is the reduced sum of squares.
pub trait ModAbsSquared<M = Self> {
    type Output;

    fn mod_abs_squared(self, m: M) -> Self::Output;
}

/// Replaces a number with its squared absolute value modulo another number $m$. The input must be
/// already reduced modulo $m$.
pub trait ModAbsSquaredAssign<M = Self> {
    fn mod_abs_squared_assign(&mut self, m: M);
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

/// Calculates the sum of the elements of a vector modulo $2^k$.
///
/// The elements must already be reduced modulo $2^k$. Unlike [`ModPowerOf2Sum`], which adds vectors
/// to each other, this adds up the elements of one vector, producing a single element.
pub trait ModPowerOf2ElementSum {
    type Output;

    fn to_mod_power_of_2_element_sum(&self, pow: u64) -> Self::Output;

    fn into_mod_power_of_2_element_sum(self, pow: u64) -> Self::Output;
}

/// Calculates the sum of the elements of a vector modulo another number $m$.
///
/// The elements must already be reduced modulo $m$. Unlike [`ModSum`], which adds vectors to each
/// other, this adds up the elements of one vector, producing a single element.
pub trait ModElementSum<M> {
    type Output;

    fn to_mod_element_sum(&self, m: M) -> Self::Output;

    fn into_mod_element_sum(self, m: M) -> Self::Output;
}

/// Calculates the product of the elements of a vector modulo $2^k$.
///
/// The elements must already be reduced modulo $2^k$. Unlike [`ModPowerOf2Product`], which
/// multiplies values from an iterator, this multiplies the elements of one vector, producing a
/// single element.
pub trait ModPowerOf2ElementProduct {
    type Output;

    fn to_mod_power_of_2_element_product(&self, pow: u64) -> Self::Output;

    fn into_mod_power_of_2_element_product(self, pow: u64) -> Self::Output;
}

/// Calculates the product of the elements of a vector modulo another number $m$.
///
/// The elements must already be reduced modulo $m$. Unlike [`ModProduct`], which multiplies values
/// from an iterator, this multiplies the elements of one vector, producing a single element.
pub trait ModElementProduct<M> {
    type Output;

    fn to_mod_element_product(&self, m: M) -> Self::Output;

    fn into_mod_element_product(self, m: M) -> Self::Output;
}

/// Computes the dot product of two vectors modulo $2^k$. The elements must already be reduced
/// modulo $2^k$.
pub trait ModPowerOf2Dot<RHS = Self> {
    type Output;

    fn mod_power_of_2_dot(self, other: RHS, pow: u64) -> Self::Output;
}

/// Computes the dot product of two vectors modulo another number $m$. The elements must already be
/// reduced modulo $m$.
pub trait ModDot<RHS = Self, M = Self> {
    type Output;

    fn mod_dot(self, other: RHS, m: M) -> Self::Output;
}

/// Adds the dot product of two vectors to a number modulo another number $m$. The number and the
/// elements must already be reduced modulo $m$, and the vectors must have the same dimension.
pub trait ModAddDot<V, W = V, M = Self> {
    type Output;

    fn mod_add_dot(self, v: V, w: W, m: M) -> Self::Output;
}

/// Adds the dot product of two vectors to a number in place modulo another number $m$. The number
/// and the elements must already be reduced modulo $m$, and the vectors must have the same
/// dimension.
pub trait ModAddDotAssign<V, W = V, M = Self> {
    fn mod_add_dot_assign(&mut self, v: V, w: W, m: M);
}

/// Adds the dot product of a vector and the reverse of another to a number modulo another number
/// $m$. The number and the elements must already be reduced modulo $m$, and the vectors must have
/// the same dimension.
pub trait ModAddDotReversed<V, W = V, M = Self> {
    type Output;

    fn mod_add_dot_reversed(self, v: V, w: W, m: M) -> Self::Output;
}

/// Adds the dot product of a vector and the reverse of another to a number in place modulo another
/// number $m$. The number and the elements must already be reduced modulo $m$, and the vectors must
/// have the same dimension.
pub trait ModAddDotReversedAssign<V, W = V, M = Self> {
    fn mod_add_dot_reversed_assign(&mut self, v: V, w: W, m: M);
}

/// Subtracts the dot product of two vectors from a number modulo another number $m$. The number and
/// the elements must already be reduced modulo $m$, and the vectors must have the same dimension.
pub trait ModSubDot<V, W = V, M = Self> {
    type Output;

    fn mod_sub_dot(self, v: V, w: W, m: M) -> Self::Output;
}

/// Subtracts the dot product of two vectors from a number in place modulo another number $m$. The
/// number and the elements must already be reduced modulo $m$, and the vectors must have the same
/// dimension.
pub trait ModSubDotAssign<V, W = V, M = Self> {
    fn mod_sub_dot_assign(&mut self, v: V, w: W, m: M);
}

/// Subtracts the dot product of a vector and the reverse of another from a number modulo another
/// number $m$. The number and the elements must already be reduced modulo $m$, and the vectors must
/// have the same dimension.
pub trait ModSubDotReversed<V, W = V, M = Self> {
    type Output;

    fn mod_sub_dot_reversed(self, v: V, w: W, m: M) -> Self::Output;
}

/// Subtracts the dot product of a vector and the reverse of another from a number in place modulo
/// another number $m$. The number and the elements must already be reduced modulo $m$, and the
/// vectors must have the same dimension.
pub trait ModSubDotReversedAssign<V, W = V, M = Self> {
    fn mod_sub_dot_reversed_assign(&mut self, v: V, w: W, m: M);
}

/// Computes the dot product of a vector and the reverse of another modulo another number $m$. The
/// elements must already be reduced modulo $m$, and the vectors must have the same dimension.
pub trait ModDotReversed<RHS = Self, M = Self> {
    type Output;

    fn mod_dot_reversed(self, other: RHS, m: M) -> Self::Output;
}

/// Adds the dot product of two vectors to a number modulo $2^k$. The number and the elements must
/// already be reduced modulo $2^k$, and the vectors must have the same dimension.
pub trait ModPowerOf2AddDot<V, W = V> {
    type Output;

    fn mod_power_of_2_add_dot(self, v: V, w: W, pow: u64) -> Self::Output;
}

/// Adds the dot product of two vectors to a number in place modulo $2^k$. The number and the
/// elements must already be reduced modulo $2^k$, and the vectors must have the same dimension.
pub trait ModPowerOf2AddDotAssign<V, W = V> {
    fn mod_power_of_2_add_dot_assign(&mut self, v: V, w: W, pow: u64);
}

/// Adds the dot product of a vector and the reverse of another to a number modulo $2^k$. The number
/// and the elements must already be reduced modulo $2^k$, and the vectors must have the same
/// dimension.
pub trait ModPowerOf2AddDotReversed<V, W = V> {
    type Output;

    fn mod_power_of_2_add_dot_reversed(self, v: V, w: W, pow: u64) -> Self::Output;
}

/// Adds the dot product of a vector and the reverse of another to a number in place modulo $2^k$.
/// The number and the elements must already be reduced modulo $2^k$, and the vectors must have the
/// same dimension.
pub trait ModPowerOf2AddDotReversedAssign<V, W = V> {
    fn mod_power_of_2_add_dot_reversed_assign(&mut self, v: V, w: W, pow: u64);
}

/// Subtracts the dot product of two vectors from a number modulo $2^k$. The number and the elements
/// must already be reduced modulo $2^k$, and the vectors must have the same dimension.
pub trait ModPowerOf2SubDot<V, W = V> {
    type Output;

    fn mod_power_of_2_sub_dot(self, v: V, w: W, pow: u64) -> Self::Output;
}

/// Subtracts the dot product of two vectors from a number in place modulo $2^k$. The number and the
/// elements must already be reduced modulo $2^k$, and the vectors must have the same dimension.
pub trait ModPowerOf2SubDotAssign<V, W = V> {
    fn mod_power_of_2_sub_dot_assign(&mut self, v: V, w: W, pow: u64);
}

/// Subtracts the dot product of a vector and the reverse of another from a number modulo $2^k$. The
/// number and the elements must already be reduced modulo $2^k$, and the vectors must have the same
/// dimension.
pub trait ModPowerOf2SubDotReversed<V, W = V> {
    type Output;

    fn mod_power_of_2_sub_dot_reversed(self, v: V, w: W, pow: u64) -> Self::Output;
}

/// Subtracts the dot product of a vector and the reverse of another from a number in place modulo
/// $2^k$. The number and the elements must already be reduced modulo $2^k$, and the vectors must
/// have the same dimension.
pub trait ModPowerOf2SubDotReversedAssign<V, W = V> {
    fn mod_power_of_2_sub_dot_reversed_assign(&mut self, v: V, w: W, pow: u64);
}

/// Computes the dot product of a vector and the reverse of another modulo $2^k$. The elements must
/// already be reduced modulo $2^k$, and the vectors must have the same dimension.
pub trait ModPowerOf2DotReversed<RHS = Self> {
    type Output;

    fn mod_power_of_2_dot_reversed(self, other: RHS, pow: u64) -> Self::Output;
}

/// Computes the Kronecker product of two matrices modulo $2^k$: the block matrix whose block in row
/// $i$ and column $j$ is the second matrix multiplied by the element of the first in row $i$ and
/// column $j$, with every element reduced modulo $2^k$. The elements of both matrices must already
/// be reduced modulo $2^k$.
pub trait ModPowerOf2KroneckerProduct<RHS = Self> {
    type Output;

    fn mod_power_of_2_kronecker_product(self, other: RHS, pow: u64) -> Self::Output;
}

/// Replaces a matrix with its Kronecker product with another matrix modulo $2^k$. The elements of
/// both matrices must already be reduced modulo $2^k$.
pub trait ModPowerOf2KroneckerProductAssign<RHS = Self> {
    fn mod_power_of_2_kronecker_product_assign(&mut self, other: RHS, pow: u64);
}

/// Computes the Kronecker product of two matrices modulo another number $m$: the block matrix whose
/// block in row $i$ and column $j$ is the second matrix multiplied by the element of the first in
/// row $i$ and column $j$, with every element reduced modulo $m$. The elements of both matrices
/// must already be reduced modulo $m$.
pub trait ModKroneckerProduct<RHS = Self, M = Self> {
    type Output;

    fn mod_kronecker_product(self, other: RHS, m: M) -> Self::Output;
}

/// Replaces a matrix with its Kronecker product with another matrix modulo another number $m$. The
/// elements of both matrices must already be reduced modulo $m$.
pub trait ModKroneckerProductAssign<RHS = Self, M = Self> {
    fn mod_kronecker_product_assign(&mut self, other: RHS, m: M);
}

/// Takes the entrywise product of two values, such as two vectors of the same dimension, modulo a
/// number $m$: every entry is the product of the corresponding entries, reduced modulo $m$. The
/// entries must be already reduced modulo $m$.
pub trait ModEntrywiseMul<RHS = Self, M = Self> {
    type Output;

    fn mod_entrywise_mul(self, other: RHS, m: M) -> Self::Output;
}

/// Replaces a value, such as a vector, by its entrywise product with another modulo a number $m$:
/// every entry is multiplied by the corresponding entry of the other value and reduced modulo $m$.
/// The entries must be already reduced modulo $m$.
pub trait ModEntrywiseMulAssign<RHS = Self, M = Self> {
    fn mod_entrywise_mul_assign(&mut self, other: RHS, m: M);
}

/// Computes the cross product of two 3-dimensional vectors modulo $m$. The entries must be already
/// reduced modulo $m$.
///
/// $$
/// v \times w = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \bmod m.
/// $$
pub trait ModCross<RHS = Self, M = Self> {
    type Output;

    fn mod_cross(self, other: RHS, m: M) -> Self::Output;
}

/// Replaces a 3-dimensional vector with its cross product with another, modulo $m$. The entries
/// must be already reduced modulo $m$.
///
/// $$
/// v \gets v \times w \bmod m.
/// $$
pub trait ModCrossAssign<RHS = Self, M = Self> {
    fn mod_cross_assign(&mut self, other: RHS, m: M);
}

/// Computes the generalized cross product of $n - 1$ vectors of dimension $n$ modulo $m$. The
/// entries must be already reduced modulo $m$.
///
/// $n - 1$ vectors of dimension $n$ determine the vector $w$ with
///
/// $$
/// w_i = (-1)^{n - 1 + i} \det A_{(i)} \bmod m,
/// $$
///
/// where $A$ is the $(n - 1) \times n$ matrix whose rows are the vectors and $A_{(i)}$ is $A$
/// without column $i$, so that $w \cdot x = \det \begin{pmatrix} A \\ x \end{pmatrix}$ for every
/// $x$. For $n = 2$ it rotates the vector a quarter turn counterclockwise, $(a, b) \mapsto (-b, a)
/// \bmod m$; for $n = 3$ it is the cross product $v \times w \bmod m$; and no vectors give the
/// 1-dimensional vector $(1 \bmod m)$.
pub trait ModGeneralizedCross<M = Self>: Sized {
    fn mod_generalized_cross(xs: &[Self], m: M) -> Self;
}

/// Finds the multiplicative inverses of all the entries of a value, such as a vector, modulo a
/// number $m$, returning `None` if any entry is not invertible. The entries must be already reduced
/// modulo $m$.
pub trait ModEntrywiseInverse<M = Self> {
    type Output;

    fn mod_entrywise_inverse(self, m: M) -> Option<Self::Output>;
}

/// Takes the entrywise product of two values, such as two vectors of the same dimension, modulo
/// $2^k$: every entry is the product of the corresponding entries, reduced modulo $2^k$. The
/// entries must be already reduced modulo $2^k$.
pub trait ModPowerOf2EntrywiseMul<RHS = Self> {
    type Output;

    fn mod_power_of_2_entrywise_mul(self, other: RHS, pow: u64) -> Self::Output;
}

/// Replaces a value, such as a vector, by its entrywise product with another modulo $2^k$: every
/// entry is multiplied by the corresponding entry of the other value and reduced modulo $2^k$. The
/// entries must be already reduced modulo $2^k$.
pub trait ModPowerOf2EntrywiseMulAssign<RHS = Self> {
    fn mod_power_of_2_entrywise_mul_assign(&mut self, other: RHS, pow: u64);
}

/// Computes the cross product of two 3-dimensional vectors modulo $2^k$. The entries must be
/// already reduced modulo $2^k$.
///
/// $$
/// v \times w = (v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0) \bmod 2^k.
/// $$
pub trait ModPowerOf2Cross<RHS = Self> {
    type Output;

    fn mod_power_of_2_cross(self, other: RHS, pow: u64) -> Self::Output;
}

/// Replaces a 3-dimensional vector with its cross product with another, modulo $2^k$. The entries
/// must be already reduced modulo $2^k$.
///
/// $$
/// v \gets v \times w \bmod 2^k.
/// $$
pub trait ModPowerOf2CrossAssign<RHS = Self> {
    fn mod_power_of_2_cross_assign(&mut self, other: RHS, pow: u64);
}

/// Computes the generalized cross product of $n - 1$ vectors of dimension $n$ modulo $2^k$. The
/// entries must be already reduced modulo $2^k$.
///
/// $n - 1$ vectors of dimension $n$ determine the vector $w$ with
///
/// $$
/// w_i = (-1)^{n - 1 + i} \det A_{(i)} \bmod 2^k,
/// $$
///
/// where $A$ is the $(n - 1) \times n$ matrix whose rows are the vectors and $A_{(i)}$ is $A$
/// without column $i$, so that $w \cdot x = \det \begin{pmatrix} A \\ x \end{pmatrix}$ for every
/// $x$. For $n = 2$ it rotates the vector a quarter turn counterclockwise, $(a, b) \mapsto (-b, a)
/// \bmod 2^k$; for $n = 3$ it is the cross product $v \times w \bmod 2^k$; and no vectors give the
/// 1-dimensional vector $(1 \bmod 2^k)$.
pub trait ModPowerOf2GeneralizedCross: Sized {
    fn mod_power_of_2_generalized_cross(xs: &[Self], pow: u64) -> Self;
}

/// Finds the multiplicative inverses of all the entries of a value, such as a vector, modulo $2^k$,
/// returning `None` if any entry is not invertible. The entries must be already reduced modulo
/// $2^k$.
pub trait ModPowerOf2EntrywiseInverse {
    type Output;

    fn mod_power_of_2_entrywise_inverse(self, pow: u64) -> Option<Self::Output>;
}
