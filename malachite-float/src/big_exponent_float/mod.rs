// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
#[cfg(feature = "random")]
use malachite_base::num::logic::traits::SignificantBits;
#[cfg(feature = "test_build")]
use malachite_base::num::modular::traits::DivisibleByPowerOf2;
#[cfg(feature = "random")]
use malachite_base::num::modular::traits::NegModPowerOf2;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

/// A floating-point number with an arbitrarily large exponent.
///
/// A `BigExponentFloat` is either NaN, $\infty$, $-\infty$, zero, or a finite nonzero number $\pm
/// m2^e$, where $m$ is a positive integer and $e$ is an [`Integer`] of any size. Unlike a
/// [`Float`](crate::Float), it differs in three ways:
/// - Its exponent is unbounded, so no operation overflows or underflows.
/// - It has no stored precision: its significand is exactly as long as its value requires, and
///   each operation takes the precision and rounding mode of its result as arguments.
/// - It has no negative zero.
///
/// This is the counterpart of FLINT's `arf_t`.
///
/// Every value has a single representation. The significand of a finite nonzero value is a
/// [`Natural`] whose most significant bit is the high bit of its top limb, like a
/// [`Float`](crate::Float)'s, and whose lowest limb is nonzero. A finite nonzero value with
/// exponent $e$ lies in $[2^{e-1}, 2^e)$ in absolute value, as for [`Float`](crate::Float).
#[derive(Clone)]
pub struct BigExponentFloat(pub(crate) InnerBigExponentFloat);

#[derive(Clone)]
pub(crate) enum InnerBigExponentFloat {
    NaN,
    Infinity {
        sign: bool,
    },
    Zero,
    Finite {
        sign: bool,
        exponent: Integer,
        significand: Natural,
    },
}

impl BigExponentFloat {
    // The finite nonzero `BigExponentFloat` ±m * 2^(e - b), where b is the number of significant
    // bits of the positive `m`, so that `exponent` is e: the value lies in [2^(e - 1), 2^e) in
    // absolute value. Trailing zeros of `m` are removed, and what is left is aligned to the top of
    // its limbs.
    #[cfg(feature = "random")]
    pub(crate) fn from_parts(sign: bool, exponent: Integer, mut m: Natural) -> Self {
        assert_ne!(m, 0u32);
        let zeros = m.trailing_zeros().unwrap();
        m >>= zeros;
        let bits = m.significant_bits();
        m <<= bits.neg_mod_power_of_2(Limb::LOG_WIDTH);
        Self(InnerBigExponentFloat::Finite {
            sign,
            exponent,
            significand: m,
        })
    }

    // Removes the trailing zero limbs of a significand whose top limb's high bit is set.
    pub(crate) fn trim_significand(significand: &mut Natural) {
        let zero_limbs = significand.trailing_zeros().unwrap() >> Limb::LOG_WIDTH;
        if zero_limbs != 0 {
            *significand >>= zero_limbs << Limb::LOG_WIDTH;
        }
    }

    /// Determines whether a [`BigExponentFloat`] is valid: whether its significand, if it has one,
    /// is a valid [`Natural`], aligned to the top of its limbs, with a nonzero lowest limb.
    ///
    /// This function is only available when the `test_build` feature is enabled.
    #[cfg(feature = "test_build")]
    pub fn is_valid(&self) -> bool {
        match self {
            Self(InnerBigExponentFloat::Finite {
                significand,
                exponent,
                ..
            }) => {
                if !significand.is_valid() || !exponent.is_valid() {
                    return false;
                }
                let bits = significand.significant_bits();
                bits != 0
                    && bits.divisible_by_power_of_2(Limb::LOG_WIDTH)
                    && !significand.divisible_by_power_of_2(Limb::WIDTH)
            }
            _ => true,
        }
    }
}

/// Basic functions for working with [`BigExponentFloat`]s.
pub mod basic;
/// Functions for converting [`BigExponentFloat`]s to and from other types, including strings.
pub mod conversion;
/// Iterators that generate [`BigExponentFloat`]s without repetition.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`BigExponentFloat`]s randomly.
pub mod random;
