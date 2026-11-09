// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(doc)]
use malachite_base::num::exponential::traits::{
    Compound, CompoundAssign, Exp, ExpAssign, ExpXMinus1, ExpXMinus1Assign, Ln, Ln1PlusX,
    Ln1PlusXAssign, LnAssign, LogBase, LogBase2, LogBase2Assign, LogBase2Of1PlusX,
    LogBase2Of1PlusXAssign, LogBase10, LogBase10Assign, LogBase10Of1PlusX, LogBase10Of1PlusXAssign,
    LogBaseAssign, LogBaseOf1PlusX, LogBaseOf1PlusXAssign, LogBasePowerOf2, LogBasePowerOf2Assign,
    LogBasePowerOf2Of1PlusX, LogBasePowerOf2Of1PlusXAssign, PowerOf2XMinus1, PowerOf2XMinus1Assign,
};

/// [`Compound`] and [`CompoundAssign`], traits for computing the compound function $(1+x)^n$ for
/// [`Float`](super::Float)s.
pub mod compound;
/// [`Exp`] and [`ExpAssign`], traits for computing $e^x$ for [`Float`](super::Float)s.
pub mod exp;
/// [`ExpXMinus1`] and [`ExpXMinus1Assign`], traits for computing $e^x-1$ for
/// [`Float`](super::Float)s.
pub mod exp_x_minus_1;
/// [`Ln`] and [`LnAssign`], traits for computing the natural logarithm of [`Float`](super::Float)s.
pub mod ln;
/// [`Ln1PlusX`] and [`Ln1PlusXAssign`], traits for computing $\ln(1+x)$ for
/// [`Float`](super::Float)s.
pub mod ln_1_plus_x;
/// [`LogBase`] and [`LogBaseAssign`], traits for computing the base-$b$ logarithm of
/// [`Float`](super::Float)s for an arbitrary integer base $b>1$.
pub mod log_base;
/// [`LogBase10`] and [`LogBase10Assign`], traits for computing the base-10 logarithm of
/// [`Float`](super::Float)s.
pub mod log_base_10;
/// [`LogBase10Of1PlusX`] and [`LogBase10Of1PlusXAssign`], traits for computing $\log_{10}(1+x)$ of
/// [`Float`](super::Float)s.
pub mod log_base_10_1_plus_x;
/// [`LogBaseOf1PlusX`] and [`LogBaseOf1PlusXAssign`], traits for computing $\log_b(1+x)$ of
/// [`Float`](super::Float)s for an arbitrary integer base $b>1$.
pub mod log_base_1_plus_x;
/// [`LogBase2`] and [`LogBase2Assign`], traits for computing the base-2 logarithm of
/// [`Float`](super::Float)s.
pub mod log_base_2;
/// [`LogBase2Of1PlusX`] and [`LogBase2Of1PlusXAssign`], traits for computing $\log_2(1+x)$ for
/// [`Float`](super::Float)s.
pub mod log_base_2_1_plus_x;
/// [`LogBase`] and [`LogBaseAssign`], implemented for a [`Float`](super::Float) base, for computing
/// $\log_b x$ of a [`Float`](super::Float) with an arbitrary [`Float`](super::Float) base.
pub mod log_base_float_base;
/// [`LogBaseOf1PlusX`] and [`LogBaseOf1PlusXAssign`], implemented for a [`Float`](super::Float)
/// base, for computing $\log_b(1+x)$ of a [`Float`](super::Float) with an arbitrary
/// [`Float`](super::Float) base.
pub mod log_base_float_base_1_plus_x;
/// [`LogBasePowerOf2`] and [`LogBasePowerOf2Assign`], traits for computing $\log_{2^k} x$ for
/// [`Float`](super::Float)s.
pub mod log_base_power_of_2;
/// [`LogBasePowerOf2Of1PlusX`] and [`LogBasePowerOf2Of1PlusXAssign`], traits for computing
/// $\log_{2^k}(1+x)$ for [`Float`](super::Float)s.
pub mod log_base_power_of_2_1_plus_x;
/// [`LogBase`] and [`LogBaseAssign`], implemented for a [`Rational`](malachite_q::Rational) base,
/// for computing $\log_b x$ of a [`Float`](super::Float) with an arbitrary rational base $b>1$.
pub mod log_base_rational_base;
/// [`LogBaseOf1PlusX`] and [`LogBaseOf1PlusXAssign`], implemented for a
/// [`Rational`](malachite_q::Rational) base, for computing $\log_b(1+x)$ of a
/// [`Float`](super::Float) with an arbitrary rational base $b>1$.
pub mod log_base_rational_base_1_plus_x;
/// Functions for computing $\log_b x$ of a [`Rational`](malachite_q::Rational) $x$ with an
/// arbitrary [`Float`](super::Float) base, returning a [`Float`](super::Float).
pub mod log_base_rational_float_base;
/// Functions for computing $\log_b x$ of a [`Rational`](malachite_q::Rational) $x$ with an
/// arbitrary [`Rational`](malachite_q::Rational) base $b>1$, returning a [`Float`](super::Float).
pub mod log_base_rational_rational_base;
/// An implementation of [`PowerOf2`](malachite_base::num::arithmetic::traits::PowerOf2) with a
/// [`Float`](super::Float) exponent, computing $2^x$ for [`Float`](super::Float)s.
pub mod power_of_10;
pub mod power_of_10_x_minus_1;
pub mod power_of_2_of_float;
/// Implementations of [`PowerOf2XMinus1`] and [`PowerOf2XMinus1Assign`], traits for computing
/// $2^x-1$.
pub mod power_of_2_x_minus_1;
