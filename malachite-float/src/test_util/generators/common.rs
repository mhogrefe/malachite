// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::conversion::string::get_str::get_str;
use crate::test_util::common::rug_round_exact_from_rounding_mode;
use core::cmp::Ordering::Equal;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, Down, Exact};
use malachite_base::test_util::generators::common::It;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

// Whether `(x, b0, m, rnd)` is a valid input to `get_str`: every rounding mode is valid except
// `Exact`, which `get_str` accepts only when `x` is exactly representable in the chosen digits.
// Exactness is mode-independent, so we detect it by probing with `Down` (any non-`Exact` mode
// returns `Equal` exactly when the value is representable). The base is assumed already valid.
pub fn valid_float_get_str_quadruple(x: &Float, b0: i64, m: usize, rnd: RoundingMode) -> bool {
    rnd != Exact || matches!(get_str(x, b0, m, Down), Some((_, _, Equal)))
}

// The nine Float conversion specifiers, in `format_string_from_parts`'s `combo` order.
const FLOAT_FORMAT_CONV_CHARS: &[u8; 9] = b"aAbeEfFgG";
// The six flag characters, selected by the low six bits of `combo`.
const FLOAT_FORMAT_FLAG_CHARS: &[u8; 6] = b"#0+ -'";
// The five rounding characters, selected (1-indexed) by `combo`.
const FLOAT_FORMAT_RND_CHARS: &[u8; 5] = b"DUYZN";
// The number of distinct `combo` values: 2^6 flag subsets times 9 conversions times 6 rounding
// choices (a rounding character or none).
pub const FLOAT_FORMAT_COMBO_COUNT: u16 = 64 * 9 * 6;

// Assembles a valid single-conversion `%R` printf format string from its parts (see
// `format_float_str`), so that generators can build valid format strings by construction rather
// than by filtering. `combo` (which should be less than `FLOAT_FORMAT_COMBO_COUNT`) selects, via
// its low six bits, a subset of the flag characters, and via the rest a conversion character and an
// optional rounding character; `width` and `prec` are the optional field width and precision. Every
// output parses as a valid Float conversion, so no rounding mode is `Exact` (there is no format
// character for it) and none of the `format_float_str` failure paths can be reached.
pub fn format_string_from_parts(combo: u16, width: Option<u64>, prec: Option<u64>) -> String {
    let flags = combo & 0x3f;
    let selector = combo >> 6; // in 0..54
    let conv = usize::from(selector % 9);
    let rnd = selector / 9; // in 0..6; 0 means no rounding character
    let mut s = vec![b'%'];
    for (i, &c) in FLOAT_FORMAT_FLAG_CHARS.iter().enumerate() {
        if flags & (1 << i) != 0 {
            s.push(c);
        }
    }
    if let Some(w) = width {
        s.extend_from_slice(w.to_string().as_bytes());
    }
    if let Some(p) = prec {
        s.push(b'.');
        s.extend_from_slice(p.to_string().as_bytes());
    }
    s.push(b'R');
    if rnd != 0 {
        s.push(FLOAT_FORMAT_RND_CHARS[usize::from(rnd) - 1]);
    }
    s.push(FLOAT_FORMAT_CONV_CHARS[conv]);
    // `s` is ASCII by construction
    String::from_utf8(s).unwrap()
}

pub fn float_rm(xs: It<Float>) -> It<(rug::Float, Float)> {
    Box::new(xs.map(|x| (rug::Float::exact_from(&x), x)))
}

pub fn float_pair_rm(xs: It<(Float, Float)>) -> It<((rug::Float, rug::Float), (Float, Float))> {
    Box::new(xs.map(|(x, y)| {
        (
            (rug::Float::exact_from(&x), rug::Float::exact_from(&y)),
            (x, y),
        )
    }))
}

pub fn float_natural_pair_rm(
    xs: It<(Float, Natural)>,
) -> It<((rug::Float, rug::Integer), (Float, Natural))> {
    Box::new(xs.map(|(x, y)| {
        (
            (rug::Float::exact_from(&x), rug::Integer::exact_from(&y)),
            (x, y),
        )
    }))
}

pub fn float_integer_pair_rm(
    xs: It<(Float, Integer)>,
) -> It<((rug::Float, rug::Integer), (Float, Integer))> {
    Box::new(xs.map(|(x, y)| {
        (
            (rug::Float::exact_from(&x), rug::Integer::exact_from(&y)),
            (x, y),
        )
    }))
}

pub fn float_rational_pair_rm(
    xs: It<(Float, Rational)>,
) -> It<((rug::Float, rug::Rational), (Float, Rational))> {
    Box::new(xs.map(|(x, y)| {
        (
            (rug::Float::exact_from(&x), rug::Rational::exact_from(&y)),
            (x, y),
        )
    }))
}

pub fn float_primitive_int_pair_rm<T: PrimitiveInt>(
    xs: It<(Float, T)>,
) -> It<((rug::Float, T), (Float, T))> {
    Box::new(xs.map(|(x, y)| ((rug::Float::exact_from(&x), y), (x, y))))
}

pub fn float_primitive_float_pair_rm<T: PrimitiveFloat>(
    xs: It<(Float, T)>,
) -> It<((rug::Float, T), (Float, T))> {
    Box::new(xs.map(|(x, y)| ((rug::Float::exact_from(&x), y), (x, y))))
}

pub fn float_t_rounding_mode_triple_rm<T: Clone + 'static>(
    xs: It<(Float, T, RoundingMode)>,
) -> It<((rug::Float, T, rug::float::Round), (Float, T, RoundingMode))> {
    Box::new(xs.filter(|(_, _, rm)| *rm != Exact).map(|(x, p, rm)| {
        (
            (
                rug::Float::exact_from(&x),
                p.clone(),
                rug_round_exact_from_rounding_mode(rm),
            ),
            (x, p, rm),
        )
    }))
}

pub fn float_t_u_triple_rm<T: Clone + 'static, U: Clone + 'static>(
    xs: It<(Float, T, U)>,
) -> It<((rug::Float, T, U), (Float, T, U))> {
    Box::new(xs.map(|(x, p, q)| {
        (
            (rug::Float::exact_from(&x), p.clone(), q.clone()),
            (x, p, q),
        )
    }))
}

pub fn float_t_u_rounding_mode_quadruple_rm<T: Clone + 'static, U: Clone + 'static>(
    xs: It<(Float, T, U, RoundingMode)>,
) -> It<(
    (rug::Float, T, U, rug::float::Round),
    (Float, T, U, RoundingMode),
)> {
    Box::new(
        xs.filter(|(_, _, _, rm)| *rm != Exact)
            .map(|(x, p, q, rm)| {
                (
                    (
                        rug::Float::exact_from(&x),
                        p.clone(),
                        q.clone(),
                        rug_round_exact_from_rounding_mode(rm),
                    ),
                    (x, p, q, rm),
                )
            }),
    )
}

pub fn float_rounding_mode_pair_rm(
    xs: It<(Float, RoundingMode)>,
) -> It<((rug::Float, rug::float::Round), (Float, RoundingMode))> {
    Box::new(xs.filter(|(_, rm)| *rm != Exact).map(|(x, rm)| {
        (
            (
                rug::Float::exact_from(&x),
                rug_round_exact_from_rounding_mode(rm),
            ),
            (x, rm),
        )
    }))
}

pub fn float_float_rounding_mode_triple_rm(
    xs: It<(Float, Float, RoundingMode)>,
) -> It<(
    (rug::Float, rug::Float, rug::float::Round),
    (Float, Float, RoundingMode),
)> {
    Box::new(xs.filter(|(_, _, rm)| *rm != Exact).map(|(x, y, rm)| {
        (
            (
                rug::Float::exact_from(&x),
                rug::Float::exact_from(&y),
                rug_round_exact_from_rounding_mode(rm),
            ),
            (x, y, rm),
        )
    }))
}

pub fn float_float_anything_triple_rm<T: Clone + 'static>(
    xs: It<(Float, Float, T)>,
) -> It<((rug::Float, rug::Float, T), (Float, Float, T))> {
    Box::new(xs.map(|(x, y, z)| {
        (
            (
                rug::Float::exact_from(&x),
                rug::Float::exact_from(&y),
                z.clone(),
            ),
            (x, y, z),
        )
    }))
}

pub fn float_rational_anything_triple_rm<T: Clone + 'static>(
    xs: It<(Float, Rational, T)>,
) -> It<((rug::Float, rug::Rational, T), (Float, Rational, T))> {
    Box::new(xs.map(|(x, y, z)| {
        (
            (
                rug::Float::exact_from(&x),
                rug::Rational::exact_from(&y),
                z.clone(),
            ),
            (x, y, z),
        )
    }))
}

pub fn float_rational_rounding_mode_triple_rm(
    xs: It<(Float, Rational, RoundingMode)>,
) -> It<(
    (rug::Float, rug::Rational, rug::float::Round),
    (Float, Rational, RoundingMode),
)> {
    Box::new(xs.filter(|(_, _, rm)| *rm != Exact).map(|(x, y, rm)| {
        (
            (
                rug::Float::exact_from(&x),
                rug::Rational::exact_from(&y),
                rug_round_exact_from_rounding_mode(rm),
            ),
            (x, y, rm),
        )
    }))
}

pub fn float_float_anything_rounding_mode_quadruple_rm<T: Clone + 'static>(
    xs: It<(Float, Float, T, RoundingMode)>,
) -> It<(
    (rug::Float, rug::Float, T, rug::float::Round),
    (Float, Float, T, RoundingMode),
)> {
    Box::new(
        xs.filter(|(_, _, _, rm)| *rm != Exact)
            .map(|(x, y, z, rm)| {
                (
                    (
                        rug::Float::exact_from(&x),
                        rug::Float::exact_from(&y),
                        z.clone(),
                        rug_round_exact_from_rounding_mode(rm),
                    ),
                    (x, y, z, rm),
                )
            }),
    )
}

pub fn float_rational_anything_rounding_mode_quadruple_rm<T: Clone + 'static>(
    xs: It<(Float, Rational, T, RoundingMode)>,
) -> It<(
    (rug::Float, rug::Rational, T, rug::float::Round),
    (Float, Rational, T, RoundingMode),
)> {
    Box::new(
        xs.filter(|(_, _, _, rm)| *rm != Exact)
            .map(|(x, y, z, rm)| {
                (
                    (
                        rug::Float::exact_from(&x),
                        rug::Rational::exact_from(&y),
                        z.clone(),
                        rug_round_exact_from_rounding_mode(rm),
                    ),
                    (x, y, z, rm),
                )
            }),
    )
}
