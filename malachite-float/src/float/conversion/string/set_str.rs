// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 2004-2024 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::Finite;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{IsPowerOf2, Sign};
use malachite_base::num::basic::traits::{Infinity, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float::set_str::{SetStrResult, limbs_set_str};

// The value is larger in magnitude than any finite `Float` of precision `prec`.
//
// This is `mpfr_overflow` from `exceptions.c`, MPFR 4.3.0.
pub(crate) fn overflow(sign: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    match (sign, rm) {
        (true, Up | Ceiling | Nearest) => (Float::INFINITY, Greater),
        (true, Floor | Down) => (Float::max_finite_value_with_prec(prec), Less),
        (false, Up | Floor | Nearest) => (Float::NEGATIVE_INFINITY, Less),
        (false, Ceiling | Down) => (-Float::max_finite_value_with_prec(prec), Greater),
        (_, Exact) => panic!("Inexact conversion from string to Float"),
    }
}

// The value is smaller in magnitude than the least positive `Float` of precision `prec`. Under
// `Nearest` the result is the least positive value; the caller substitutes `Down` in the cases
// where `mpfr_check_range` does.
//
// This is `mpfr_underflow` from `exceptions.c`, MPFR 4.3.0.
fn underflow(sign: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    match (sign, rm) {
        (true, Up | Ceiling | Nearest) => (Float::min_positive_value_prec(prec), Greater),
        (true, Floor | Down) => (Float::ZERO, Less),
        (false, Up | Floor | Nearest) => (-Float::min_positive_value_prec(prec), Less),
        (false, Ceiling | Down) => (Float::NEGATIVE_ZERO, Greater),
        (_, Exact) => panic!("Inexact conversion from string to Float"),
    }
}

crate_test_fn! {
// Converts a parsed digit string to a `Float` of precision `prec`, correctly rounded with `rm`.
//
// `digits` holds digit values (not characters), most significant first, with leading and trailing
// zeros already stripped; it must be nonempty, so a zero value is the caller's business. `exp_base`
// is the number of digits before the point plus any base-`base` exponent, and `exp_bin` an extra
// binary exponent (the `p` form of the input), zero when there is none.
//
// This is the tail of `parsed_string_to_mpfr` from `strtofr.c`, MPFR 4.3.0, together with the
// `mpfr_check_range` call that follows it.
set_str_helper(
    sign: bool,
    digits: &[u8],
    base: u8,
    exp_base: i64,
    exp_bin: i64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    // `limbs_set_str` rounds the magnitude, so it takes the rounding mode inverted for a negative
    // value, and the direction it returns refers to the magnitude too.
    let mag_rm = if sign { rm } else { -rm };
    let away_from_0 = if sign { Greater } else { Less };
    match limbs_set_str(digits, u64::from(base), exp_base, exp_bin, prec, mag_rm) {
        SetStrResult::Overflow => overflow(sign, prec, rm),
        SetStrResult::Underflow => underflow(sign, prec, if rm == Nearest { Down } else { rm }),
        SetStrResult::Finite(significand, exp, dir) => {
            // `round_helper_raw` rounds under `Exact` as if it were `Down` rather than complaining,
            // so the contract is enforced here, as it is at the other call sites.
            assert!(
                rm != Exact || dir == 0,
                "Inexact conversion from string to Float"
            );
            let o = if sign { dir.sign() } else { dir.sign().reverse() };
            let significand = Natural::from_owned_limbs_asc(significand);
            // mpfr_check_range
            if exp > Float::MAX_EXPONENT_I64 {
                return overflow(sign, prec, rm);
            }
            if exp < Float::MIN_EXPONENT_I64 {
                // Under `Nearest` the result rounds away from zero, to the least positive value,
                // only when it is at least half of it: the exponent must be exactly one below the
                // minimum, and at exactly half (a power of two) the discarded part must have been
                // nonzero.
                let rm = if rm == Nearest
                    && !(exp == Float::MIN_EXPONENT_MINUS_1_I64
                        && (o == away_from_0.reverse() || !significand.is_power_of_2()))
                {
                    Down
                } else {
                    rm
                };
                return underflow(sign, prec, rm);
            }
            (
                Float(Finite {
                    sign,
                    exponent: i32::exact_from(exp),
                    precision: prec,
                    significand,
                }),
                o,
            )
        }
    }
}}
