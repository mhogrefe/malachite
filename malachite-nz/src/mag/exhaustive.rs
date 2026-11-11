// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::mag::{MAG_BITS, Mag};
use core::iter::{Chain, Once, once};
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::exhaustive::{ExhaustiveSigneds, exhaustive_signeds};
use malachite_base::tuples::exhaustive::{ExhaustivePairs, exhaustive_pairs};

// Generates every `MAG_BITS`-bit mantissa, in order of increasing number of significant bits once
// trailing zeros are removed: 2^29, then 3 * 2^28, then 5 * 2^27 and 7 * 2^27, and so on. This is
// how `exhaustive_positive_finite_floats` orders significands, by precision, so that simple values
// come first.
#[derive(Clone, Debug)]
pub struct ExhaustiveMagMantissas {
    // The number of significant bits of the next odd value.
    bits: u64,
    // The next odd value, which has `bits` significant bits.
    odd: u64,
}

impl Iterator for ExhaustiveMagMantissas {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.bits > MAG_BITS {
            return None;
        }
        let man = self.odd << (MAG_BITS - self.bits);
        self.odd += 2;
        if self.odd >> self.bits != 0 {
            self.bits += 1;
            self.odd = u64::power_of_2(self.bits - 1) + 1;
        }
        Some(man)
    }
}

// Generates every `MAG_BITS`-bit mantissa, simplest first; see `ExhaustiveMagMantissas`.
pub const fn exhaustive_mag_mantissas() -> ExhaustiveMagMantissas {
    ExhaustiveMagMantissas { bits: 1, odd: 1 }
}

// Generates all positive `Mag`s.
#[derive(Clone, Debug)]
pub struct ExhaustivePositiveMags(
    ExhaustivePairs<u64, ExhaustiveMagMantissas, i64, ExhaustiveSigneds<i64>>,
);

impl Iterator for ExhaustivePositiveMags {
    type Item = Mag;

    #[inline]
    fn next(&mut self) -> Option<Mag> {
        self.0.next().map(|(man, exp)| Mag { man, exp })
    }
}

// Generates all positive `Mag`s, pairing the mantissas of `exhaustive_mag_mantissas` with the
// exponents of `exhaustive_signeds`, so that small exponents and simple mantissas come first.
pub fn exhaustive_positive_mags() -> ExhaustivePositiveMags {
    ExhaustivePositiveMags(exhaustive_pairs(
        exhaustive_mag_mantissas(),
        exhaustive_signeds(),
    ))
}

// Generates all `Mag`s: zero, followed by `exhaustive_positive_mags`.
pub type ExhaustiveMags = Chain<Once<Mag>, ExhaustivePositiveMags>;

// Generates all `Mag`s: zero, followed by `exhaustive_positive_mags`.
#[inline]
pub fn exhaustive_mags() -> ExhaustiveMags {
    once(Mag::ZERO).chain(exhaustive_positive_mags())
}
