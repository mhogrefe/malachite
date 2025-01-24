// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::bools::random::{random_bools, RandomBools};
use crate::num::conversion::string::options::{FromSciStringOptions, SciSizeOptions, ToSciOptions};
use crate::num::random::geometric::{
    geometric_random_negative_signeds, geometric_random_unsigneds, GeometricRandomNaturalValues,
    GeometricRandomNegativeSigneds,
};
use crate::num::random::{random_unsigned_inclusive_range, RandomUnsignedInclusiveRange};
use crate::random::Seed;
use crate::rounding_modes::random::{random_rounding_modes, RandomRoundingModes};

/// Generates random [`SciSizeOptions`]s.
///
/// This struct is created by [`random_sci_size_options`]; see its documentation for more.
pub struct RandomSciSizeOptions {
    bs: RandomBools,
    xs: GeometricRandomNaturalValues<u64>,
}

impl Iterator for RandomSciSizeOptions {
    type Item = SciSizeOptions;

    fn next(&mut self) -> Option<SciSizeOptions> {
        let x = self.xs.next().unwrap();
        Some(if self.bs.next().unwrap() {
            if x == 0 {
                SciSizeOptions::Complete
            } else {
                SciSizeOptions::Precision(x)
            }
        } else {
            SciSizeOptions::Scale(x)
        })
    }
}

/// Generates random [`SciSizeOptions`]s.
///
/// The scales and precisions are chosen from a geometric distribution whose mean is the ratio
/// `m_size_numerator / m_size_denominator`.
///
/// # Panics
/// Panics if `m_size_numerator` or `m_size_denominator` are zero, or, if after being reduced to
/// lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// The output length is infinite.
pub fn random_sci_size_options(
    seed: Seed,
    m_size_numerator: u64,
    m_size_denominator: u64,
) -> RandomSciSizeOptions {
    RandomSciSizeOptions {
        bs: random_bools(seed.fork("bs")),
        xs: geometric_random_unsigneds(seed.fork("xs"), m_size_numerator, m_size_denominator),
    }
}

/// Generates random [`ToSciOptions`]s.
///
/// This struct is created by [`random_to_sci_options`]; see its documentation for more.
pub struct RandomToSciOptions {
    us: RandomUnsignedInclusiveRange<u8>,
    rms: RandomRoundingModes,
    sos: RandomSciSizeOptions,
    is: GeometricRandomNegativeSigneds<i64>,
    bs: RandomBools,
}

impl Iterator for RandomToSciOptions {
    type Item = ToSciOptions;

    fn next(&mut self) -> Option<ToSciOptions> {
        Some(ToSciOptions {
            base: self.us.next().unwrap(),
            rounding_mode: self.rms.next().unwrap(),
            size_options: self.sos.next().unwrap(),
            neg_exp_threshold: self.is.next().unwrap(),
            lowercase: self.bs.next().unwrap(),
            e_lowercase: self.bs.next().unwrap(),
            force_exponent_plus_sign: self.bs.next().unwrap(),
            include_trailing_zeros: self.bs.next().unwrap(),
        })
    }
}

/// Generates random [`ToSciOptions`]s.
///
/// The scales, precisions, and the negative of the negative exponenet threshold are chosen from a
/// geometric distribution whose mean is the ratio `m_size_numerator / m_size_denominator`.
///
/// # Panics
/// Panics if `m_size_numerator` or `m_size_denominator` are zero, or, if after being reduced to
/// lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// The output length is infinite.
pub fn random_to_sci_options(
    seed: Seed,
    m_size_numerator: u64,
    m_size_denominator: u64,
) -> RandomToSciOptions {
    RandomToSciOptions {
        us: random_unsigned_inclusive_range(seed.fork("us"), 2, 36),
        rms: random_rounding_modes(seed.fork("rms")),
        sos: random_sci_size_options(seed.fork("sos"), m_size_numerator, m_size_denominator),
        is: geometric_random_negative_signeds(
            seed.fork("is"),
            m_size_numerator,
            m_size_denominator,
        ),
        bs: random_bools(seed.fork("bs")),
    }
}

/// Generates random [`FromSciStringOptions`]s.
///
/// This struct is created by [`random_from_sci_string_options`]; see its documentation for more.
pub struct RandomFromSciStringOptions {
    us: RandomUnsignedInclusiveRange<u8>,
    rms: RandomRoundingModes,
}

impl Iterator for RandomFromSciStringOptions {
    type Item = FromSciStringOptions;

    fn next(&mut self) -> Option<FromSciStringOptions> {
        Some(FromSciStringOptions {
            base: self.us.next().unwrap(),
            rounding_mode: self.rms.next().unwrap(),
        })
    }
}

/// Generates random [`FromSciStringOptions`]s.
///
/// The output length is infinite.
pub fn random_from_sci_string_options(seed: Seed) -> RandomFromSciStringOptions {
    RandomFromSciStringOptions {
        us: random_unsigned_inclusive_range(seed.fork("us"), 2, 36),
        rms: random_rounding_modes(seed.fork("rms")),
    }
}
