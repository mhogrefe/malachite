// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Calculates the AGM (arithmetic-geometric mean) of two numbers.
pub trait Agm<RHS = Self> {
    type Output;

    fn agm(self, other: RHS) -> Self::Output;
}

/// Replaces a number with the AGM (arithmetic-geometric mean) of it and another number.
pub trait AgmAssign<RHS = Self> {
    fn agm_assign(&mut self, other: RHS);
}

/// Calculates the compound function $(1+x)^n$ of a number $x$.
pub trait Compound<N> {
    type Output;

    fn compound(self, n: N) -> Self::Output;
}

/// Replaces a number $x$ with the compound function $(1+x)^n$.
pub trait CompoundAssign<N> {
    fn compound_assign(&mut self, n: N);
}

/// Computes $e^x$, the exponential of a number.
pub trait Exp {
    type Output;

    fn exp(self) -> Self::Output;
}

/// Replaces a number with its exponential, $e^x$.
pub trait ExpAssign {
    fn exp_assign(&mut self);
}

/// Computes $e^x-1$, the exponential of a number, minus one.
pub trait ExpXMinus1 {
    type Output;

    fn exp_x_minus_1(self) -> Self::Output;
}

/// Replaces a number $x$ with $e^x-1$.
pub trait ExpXMinus1Assign {
    fn exp_x_minus_1_assign(&mut self);
}

/// Computes $2^x-1$, two raised to the power of a number, minus one.
pub trait PowerOf2XMinus1 {
    type Output;

    fn power_of_2_x_minus_1(self) -> Self::Output;
}

/// Replaces a number $x$ with $2^x-1$.
pub trait PowerOf2XMinus1Assign {
    fn power_of_2_x_minus_1_assign(&mut self);
}

/// Computes $10^x-1$, ten raised to the power of a number, minus one.
pub trait PowerOf10XMinus1 {
    type Output;

    fn power_of_10_x_minus_1(self) -> Self::Output;
}

/// Replaces a number $x$ with $10^x-1$.
pub trait PowerOf10XMinus1Assign {
    fn power_of_10_x_minus_1_assign(&mut self);
}

/// Takes the natural logarithm of a number.
pub trait Ln {
    type Output;

    fn ln(self) -> Self::Output;
}

/// Replaces a number with its natural logarithm.
pub trait LnAssign {
    fn ln_assign(&mut self);
}

/// Computes $\ln(1+x)$.
pub trait Ln1PlusX {
    type Output;

    fn ln_1_plus_x(self) -> Self::Output;
}

/// Replaces a number $x$ by $\ln(1+x)$.
pub trait Ln1PlusXAssign {
    fn ln_1_plus_x_assign(&mut self);
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

/// Calculates the base-2 logarithm of a number.
pub trait LogBase2 {
    type Output;

    fn log_base_2(self) -> Self::Output;
}

/// Replaces a number with its base-2 logarithm.
pub trait LogBase2Assign {
    fn log_base_2_assign(&mut self);
}

/// Calculates the base-10 logarithm of a number, rounding the (generally irrational) result.
pub trait LogBase10 {
    type Output;

    fn log_base_10(self) -> Self::Output;
}

/// Replaces a number with its base-10 logarithm, rounding the (generally irrational) result.
pub trait LogBase10Assign {
    fn log_base_10_assign(&mut self);
}

/// Computes $\log_2(1+x)$.
pub trait LogBase2Of1PlusX {
    type Output;

    fn log_base_2_1_plus_x(self) -> Self::Output;
}

/// Replaces a number $x$ by $\log_2(1+x)$.
pub trait LogBase2Of1PlusXAssign {
    fn log_base_2_1_plus_x_assign(&mut self);
}

/// Computes $\log_{2^k}(1+x)$.
pub trait LogBasePowerOf2Of1PlusX<POW> {
    type Output;

    fn log_base_power_of_2_1_plus_x(self, pow: POW) -> Self::Output;
}

/// Replaces a number $x$ by $\log_{2^k}(1+x)$.
pub trait LogBasePowerOf2Of1PlusXAssign<POW> {
    fn log_base_power_of_2_1_plus_x_assign(&mut self, pow: POW);
}

/// Computes $\log_b(1+x)$ for an integer base $b$.
pub trait LogBaseOf1PlusX<B = Self> {
    type Output;

    fn log_base_1_plus_x(self, base: B) -> Self::Output;
}

/// Replaces a number $x$ by $\log_b(1+x)$ for an integer base $b$.
pub trait LogBaseOf1PlusXAssign<B = Self> {
    fn log_base_1_plus_x_assign(&mut self, base: B);
}

/// Computes $\log_{10}(1+x)$.
pub trait LogBase10Of1PlusX {
    type Output;

    fn log_base_10_1_plus_x(self) -> Self::Output;
}

/// Replaces a number $x$ by $\log_{10}(1+x)$.
pub trait LogBase10Of1PlusXAssign {
    fn log_base_10_1_plus_x_assign(&mut self);
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

/// Calculates the base-$2^k$ logarithm of a number.
pub trait LogBasePowerOf2<POW> {
    type Output;

    fn log_base_power_of_2(self, pow: POW) -> Self::Output;
}

/// Replaces a number with its base-$2^k$ logarithm.
pub trait LogBasePowerOf2Assign<POW> {
    fn log_base_power_of_2_assign(&mut self, pow: POW);
}

/// Calculates the base-$b$ logarithm of a number, rounding the (generally irrational) result.
pub trait LogBase<B = Self> {
    type Output;

    fn log_base(self, base: B) -> Self::Output;
}

/// Replaces a number with its base-$b$ logarithm, rounding the (generally irrational) result.
pub trait LogBaseAssign<B = Self> {
    fn log_base_assign(&mut self, base: B);
}

/// Raises 10 to a power.
pub trait PowerOf10<POW> {
    fn power_of_10(pow: POW) -> Self;
}

/// Replaces a number with 10 raised to the power of that number.
pub trait PowerOf10Assign {
    fn power_of_10_assign(&mut self);
}
