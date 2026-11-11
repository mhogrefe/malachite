// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use malachite_base::comparison::traits::{Max, Min};
use malachite_base::named::Named;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, NegativeOne, One, OneHalf, Two,
    Zero as ZeroTrait,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::SignedLimb;

// The finite power of 2 with sign `sign` and exponent `exponent`: the value is ±2^(exponent - 1).
const fn power_of_2(sign: bool, exponent: SignedLimb) -> BigExponentFloat {
    BigExponentFloat(Finite {
        sign,
        exponent: Integer::const_from_signed(exponent),
        significand: Natural::HIGH_BIT,
    })
}

/// The constant 0.
impl ZeroTrait for BigExponentFloat {
    const ZERO: Self = Self(Zero);
}

/// The constant 1.
impl One for BigExponentFloat {
    const ONE: Self = power_of_2(true, 1);
}

/// The constant 2.
impl Two for BigExponentFloat {
    const TWO: Self = power_of_2(true, 2);
}

/// The constant $-1$.
impl NegativeOne for BigExponentFloat {
    const NEGATIVE_ONE: Self = power_of_2(false, 1);
}

/// The constant 1/2.
impl OneHalf for BigExponentFloat {
    const ONE_HALF: Self = power_of_2(true, 0);
}

/// The constant $\infty$.
impl InfinityTrait for BigExponentFloat {
    const INFINITY: Self = Self(Infinity { sign: true });
}

/// The constant $-\infty$.
impl NegativeInfinity for BigExponentFloat {
    const NEGATIVE_INFINITY: Self = Self(Infinity { sign: false });
}

/// The constant NaN.
impl NaNTrait for BigExponentFloat {
    const NAN: Self = Self(NaN);
}

impl Default for BigExponentFloat {
    /// The default value of a [`BigExponentFloat`], NaN.
    fn default() -> Self {
        Self::NAN
    }
}

/// The lowest value representable by this type, $-\infty$.
impl Min for BigExponentFloat {
    const MIN: Self = Self::NEGATIVE_INFINITY;
}

/// The highest value representable by this type, $\infty$.
impl Max for BigExponentFloat {
    const MAX: Self = Self::INFINITY;
}

// Implements `Named` for `BigExponentFloat`.
impl_named!(BigExponentFloat);
