// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::test_util::common::rug_float_significant_bits;
use core::cmp::Ordering::{self, *};
use malachite_base::num::conversion::traits::ExactFrom;
use rug::float::Round;
use rug::ops::AssignRound;

// log_{2^pow}(1 + x) = log_2(1 + x) / pow. rug serves as an independent oracle: compute log_2(1 + x)
// once with MPFR's `log2_1p` (correctly rounded, and reporting its own inexactness via a ternary) at
// a working precision generously above the target, then divide by the exact integer `pow` and round
// to `prec`.
//
// A single evaluation -- rather than a Ziv loop that raises the working precision until two
// down/up brackets agree -- is essential for bounded memory. When `x` is an extreme `Float` whose
// exponent is near the `Float` limits, `1 + x` is astronomically close to a power of 2, and forcing
// MPFR to separate brackets around that minuscule deviation would inflate the working precision
// toward the exponent magnitude (up to ~2^30 bits, i.e. gigabytes of MPFR scratch). `log2_1p`
// resolves the correctly-rounded value cheaply and tells us, through its ternary, which side of the
// computed value the true log_2(1 + x) lies on.
//
// That ternary matters in exactly one subtle case. If log_2(1 + x) / pow lands precisely on a
// representable value, a naive evaluation would report the result as exact even though log_2(1 + x)
// was itself inexact (for instance `x = 2^k` makes `1 + x` a hair above `2^k`, so log_2(1 + x) is a
// hair above the integer `k`, and `k / pow` is therefore not the true result). The true value then
// sits infinitesimally to one side of that representable, so nudging the numerator by one ulp toward
// the true log_2(1 + x) and re-dividing recovers the correct directed rounding and ternary. When the
// division is already inexact, the infinitesimal gap between `t` and log_2(1 + x) is far smaller
// than the distance to the next representable, so it cannot change the rounded value or its ternary
// and the first evaluation stands. This reasoning holds for either sign of `pow`, since the nudge is
// applied to the numerator in the direction of the true log_2(1 + x).
pub fn rug_log_base_power_of_2_1_plus_x_prec_round(
    x: &rug::Float,
    pow: i64,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let target_prec = u32::exact_from(prec);
    let wp = u32::exact_from((prec << 1) + 128 + (rug_float_significant_bits(x) << 1));
    // t ~= log_2(1 + x), rounded to nearest; o_num = sign(t - log_2(1 + x)).
    let mut t = rug::Float::with_val(wp, 0);
    let o_num = t.assign_round(x.log2_1p_ref(), Round::Nearest);
    let pow_float = rug::Float::with_val(wp, pow);
    // When log_2(1 + x) is inexact, nudge `t` by one ulp toward its true value before dividing. The
    // nudge is far smaller than one ulp of the result, so for a quotient strictly interior to a
    // rounding cell it changes nothing; but when t / pow lands exactly on a representable or exactly
    // on a tie, it pushes the value to the correct side, so the directed rounding and ternary reflect
    // the true (infinitesimally offset) result rather than a spurious exact/round-to-even outcome.
    // o_num == Greater means `t` overshot, i.e. log_2(1 + x) < t. (For NaN/infinite log_2(1 + x) the
    // ternary is Equal, so no nudge is applied.)
    if o_num == Greater {
        t.next_down();
    } else if o_num == Less {
        t.next_up();
    }
    // l = round_prec(t / pow); o = sign(l - t / pow).
    let mut l = rug::Float::with_val(target_prec, 0);
    let o = l.assign_round(&t / &pow_float, rm);
    if l.is_nan() {
        // x < -1, so the result is NaN. (NaN != NaN, so equality tests against it never succeed.)
        return (l, Equal);
    }
    (l, o)
}

pub fn rug_log_base_power_of_2_1_plus_x_prec(
    x: &rug::Float,
    pow: i64,
    prec: u64,
) -> (rug::Float, Ordering) {
    rug_log_base_power_of_2_1_plus_x_prec_round(x, pow, prec, Round::Nearest)
}

pub fn rug_log_base_power_of_2_1_plus_x_round(
    x: &rug::Float,
    pow: i64,
    rm: Round,
) -> (rug::Float, Ordering) {
    rug_log_base_power_of_2_1_plus_x_prec_round(x, pow, rug_float_significant_bits(x), rm)
}

pub fn rug_log_base_power_of_2_1_plus_x(x: &rug::Float, pow: i64) -> rug::Float {
    rug_log_base_power_of_2_1_plus_x_prec_round(
        x,
        pow,
        rug_float_significant_bits(x),
        Round::Nearest,
    )
    .0
}
