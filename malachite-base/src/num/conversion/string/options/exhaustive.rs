// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::bools::exhaustive::exhaustive_bools;
use crate::num::conversion::string::options::{FromSciStringOptions, SciSizeOptions, ToSciOptions};
use crate::num::exhaustive::{
    exhaustive_negative_signeds, primitive_int_increasing_inclusive_range,
};
use crate::num::logic::traits::NotAssign;
use crate::rounding_modes::exhaustive::exhaustive_rounding_modes;
use crate::rounding_modes::RoundingMode;
use crate::tuples::exhaustive::{exhaustive_triples, lex_pairs, lex_quadruples_from_single};
use alloc::boxed::Box;

/// Generates all [`SciSizeOptions`]s.
///
/// This struct is created by [`exhaustive_sci_size_options`]; see its documentation for more.
pub struct ExhaustiveSciSizeOptions {
    i: u64,
    even: bool,
}

impl Iterator for ExhaustiveSciSizeOptions {
    type Item = SciSizeOptions;

    fn next(&mut self) -> Option<SciSizeOptions> {
        let out = if self.even {
            if self.i == 0 {
                SciSizeOptions::Complete
            } else {
                SciSizeOptions::Precision(self.i)
            }
        } else {
            let i = self.i;
            self.i += 1;
            SciSizeOptions::Scale(i)
        };
        self.even.not_assign();
        Some(out)
    }
}

/// Generates all [`SciSizeOptions`]s.
///
/// The output length is $2^{65}$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
pub const fn exhaustive_sci_size_options() -> ExhaustiveSciSizeOptions {
    ExhaustiveSciSizeOptions { i: 0, even: true }
}

/// Generates all [`ToSciOptions`]s.
///
/// This struct is created by [`exhaustive_to_sci_options`]; see its documentation for more.
pub struct ExhaustiveToSciOptions(
    Box<
        dyn Iterator<
            Item = (
                (u8, SciSizeOptions, i64),
                (RoundingMode, (bool, bool, bool, bool)),
            ),
        >,
    >,
);

impl Iterator for ExhaustiveToSciOptions {
    type Item = ToSciOptions;

    fn next(&mut self) -> Option<ToSciOptions> {
        let (
            (base, size_options, neg_exp_threshold),
            (
                rounding_mode,
                (lowercase, e_lowercase, force_exponent_plus_sign, include_trailing_zeros),
            ),
        ) = self.0.next()?;
        Some(ToSciOptions {
            base,
            size_options,
            neg_exp_threshold,
            rounding_mode,
            lowercase,
            e_lowercase,
            force_exponent_plus_sign,
            include_trailing_zeros,
        })
    }
}

/// Generates all [`ToSciOptions`]s.
///
/// The output length is $2^{133}\times 3 \times 5 \times 7 \approx 1.4335 \times 10^{42}$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
pub fn exhaustive_to_sci_options() -> ExhaustiveToSciOptions {
    ExhaustiveToSciOptions(Box::new(lex_pairs(
        exhaustive_triples(
            primitive_int_increasing_inclusive_range(2, 36),
            exhaustive_sci_size_options(),
            exhaustive_negative_signeds(),
        ),
        lex_pairs(
            exhaustive_rounding_modes(),
            lex_quadruples_from_single(exhaustive_bools()),
        ),
    )))
}

/// Generates all [`FromSciStringOptions`]s.
///
/// This struct is created by [`exhaustive_from_sci_string_options`]; see its documentation for
/// more.
pub struct ExhaustiveFromSciStringOptions(Box<dyn Iterator<Item = (u8, RoundingMode)>>);

impl Iterator for ExhaustiveFromSciStringOptions {
    type Item = FromSciStringOptions;

    fn next(&mut self) -> Option<FromSciStringOptions> {
        let (base, rounding_mode) = self.0.next()?;
        Some(FromSciStringOptions {
            base,
            rounding_mode,
        })
    }
}

/// Generates all [`FromSciStringOptions`]s.
///
/// The output length is 210.
///
/// # Complexity per iteration
/// Constant time and additional memory.
pub fn exhaustive_from_sci_string_options() -> ExhaustiveFromSciStringOptions {
    ExhaustiveFromSciStringOptions(Box::new(lex_pairs(
        primitive_int_increasing_inclusive_range(2, 36),
        exhaustive_rounding_modes(),
    )))
}
