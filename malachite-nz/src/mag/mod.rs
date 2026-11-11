// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2014, 2015 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::WrappingFrom;

// The number of bits in the mantissa of a `Mag`.
pub const MAG_BITS: u64 = 30;

const MAG_BITS_I64: i64 = MAG_BITS as i64;

const MAG_ONE_HALF: u64 = 1 << (MAG_BITS - 1);

// A nonnegative number m * 2^(e - MAG_BITS), with a mantissa m of exactly `MAG_BITS` bits, or zero,
// with m = e = 0. A nonzero `Mag` with exponent e therefore lies in [2^(e - 1), 2^e). Operations
// round their results up, so that the value is an upper bound for the exact one, unless their names
// end in `_lower`.
//
// The representation is canonical, so the derived equality is numeric equality.
//
// This is FLINT's `mag_t` from `mag.h`, FLINT 3.6.0, without infinity, and with an exponent that
// fits in an `i64`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Mag {
    man: u64,
    exp: i64,
}

// The product of two `MAG_BITS`-bit mantissas, shifted right by `MAG_BITS`.
//
// This is `MAG_FIXMUL` from `mag.h`, FLINT 3.6.0.
const fn fix_mul(x: u64, y: u64) -> u64 {
    (x * y) >> MAG_BITS
}

impl Mag {
    pub const ZERO: Self = Self { man: 0, exp: 0 };

    pub const ONE: Self = Self {
        man: MAG_ONE_HALF,
        exp: 1,
    };

    pub const fn is_zero(self) -> bool {
        self.man == 0
    }

    // The `Mag` with mantissa `man` and exponent `exp`, that is, man * 2^(exp - MAG_BITS). Either
    // both are zero, or `man` has exactly `MAG_BITS` significant bits.
    pub const fn from_mantissa_and_exponent(man: u64, exp: i64) -> Self {
        assert!(
            (man == 0 && exp == 0) || man >> (MAG_BITS - 1) == 1,
            "invalid Mag mantissa and exponent"
        );
        Self { man, exp }
    }

    // The mantissa and exponent of `self`: `self` is man * 2^(exp - MAG_BITS).
    pub const fn mantissa_and_exponent(self) -> (u64, i64) {
        (self.man, self.exp)
    }

    // Shifts the mantissa left by one bit if it has one bit too few.
    //
    // This is `MAG_FAST_ADJUST_ONE_TOO_SMALL` from `mag.h`, FLINT 3.6.0.
    fn adjust_one_too_small(&mut self) {
        let t = u64::from(self.man < MAG_ONE_HALF);
        self.man <<= t;
        self.exp -= i64::wrapping_from(t);
    }

    // Shifts the mantissa right by one bit, rounding up, if it has one bit too many.
    //
    // This is `MAG_FAST_ADJUST_ONE_TOO_LARGE` from `mag.h`, FLINT 3.6.0.
    fn adjust_one_too_large(&mut self) {
        let t = self.man >> MAG_BITS;
        self.man = (self.man >> t) + (t & self.man);
        self.exp += i64::wrapping_from(t);
    }
}

/// An upper bound for `x * y + z`.
pub mod add_mul;
/// The smallest integer that is at least a `Mag`.
pub mod ceiling;
/// Comparison of `Mag`s.
pub mod cmp;
/// An upper bound for the quotient of a `Mag` and a positive integer.
pub mod div;
/// Iterators that generate `Mag`s without repetition.
pub mod exhaustive;
/// An upper bound for a `Natural`.
pub mod from_natural;
/// Parsing a `Mag` from its exact value.
pub mod from_string;
/// Upper and lower bounds for a `u64`.
pub mod from_u64;
/// An upper bound for a product of `Mag`s.
pub mod mul;
#[cfg(feature = "random")]
/// Iterators that generate `Mag`s randomly.
pub mod random;
/// An upper bound for the square root of a `Mag`.
pub mod sqrt;
/// Formatting a `Mag` as its exact value.
pub mod to_string;
