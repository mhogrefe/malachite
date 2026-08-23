// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::float::conversion::string::get_str::get_str;
use crate::float::conversion::string::strtofr::strtofr;
use crate::test_util::common::rug_round_exact_from_rounding_mode;
use alloc::string::{String, ToString};
use core::cmp::Ordering::Equal;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::string::options::FromSciStringOptions;
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

// Whether `(s, base, prec, rnd)` is a valid input to `strtofr`: every rounding mode is valid except
// `Exact`, which `strtofr` accepts only when the string's value is exactly representable with
// `prec` bits. Exactness is mode-independent, so we detect it by probing with `Down` (any
// non-`Exact` mode returns `Equal` exactly when the value is representable). An unparseable string
// yields an exact zero, so `Exact` is valid for it too. The base is assumed already valid.
pub fn valid_strtofr_quadruple(s: &str, base: u8, prec: u64, rnd: RoundingMode) -> bool {
    rnd != Exact || strtofr(s, base, prec, Down).1 == Equal
}

// The digit characters, indexed by value. Bases up to 36 are parsed case-insensitively; above that,
// the uppercase letters are 10 to 35 and the lowercase ones 36 to 61.
const DIGIT_CHARS_36: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";
const DIGIT_CHARS_62: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

// Whether `(s, options, prec)` is a valid input to `Float::from_sci_string_with_options_prec`:
// every rounding mode is valid except `Exact`, which is accepted only when the string's value is
// exactly representable with `prec` bits. Exactness is mode-independent, so we detect it by probing
// with `Down`. A string that does not parse at all is fine under every mode.
pub fn valid_float_from_sci_string_triple(
    s: &str,
    options: FromSciStringOptions,
    prec: u64,
) -> bool {
    if options.get_rounding_mode() != Exact {
        return true;
    }
    let mut probe = options;
    probe.set_rounding_mode(Down);
    Float::from_sci_string_with_options_prec(s, probe, prec).is_none_or(|(_, o)| o == Equal)
}

// The number of distinct `combo` values for `sci_string_from_parts`. As for
// `strtofr_string_from_parts`, the fields are ordered so that the fastest-varying ones are the most
// distinctive.
pub const SCI_STRING_COMBO_COUNT: u32 = 3 * 3 * 3 * 5;

// Assembles a string that `Float::from_sci_string_with_options` reads completely, so that
// generators can build valid input by construction rather than by filtering.
//
// `combo` selects, from successive fields: whether the value is a number, a NaN, or an infinity; an
// exponent marker; a sign; and where the point goes. Malachite's grammar differs from MPFR's, so
// there are no base prefixes and no `@` or `p` markers, and a base of 15 or more requires an
// explicit sign after the marker, to tell it from the digit `e`. The special values are spelled as
// `Float`'s `Display` writes them, which admits no sign on a NaN and only `-` on an infinity.
pub fn sci_string_from_parts(base: u8, combo: u32, digits: &[u8], exp: i64) -> String {
    // 0 is a number, the main case, and so the one an exhaustive enumeration should reach first; 1
    // is a NaN and 2 an infinity.
    let kind = combo % 3;
    let marker = (combo / 3) % 3;
    let sign = usize::exact_from((combo / 9) % 3);
    let point = usize::exact_from((combo / 27) % 5);
    if kind == 1 {
        return String::from("NaN");
    }
    if kind == 2 {
        return String::from(if sign == 1 { "-Infinity" } else { "Infinity" });
    }
    let mut s = String::new();
    s.push_str(["", "-", "+"][sign]);
    let default_digits = [0];
    let digits = if digits.is_empty() {
        &default_digits[..]
    } else {
        digits
    };
    let chars: &[u8] = if base <= 36 {
        DIGIT_CHARS_36
    } else {
        DIGIT_CHARS_62
    };
    let point = point % (digits.len() + 1);
    for (i, &d) in digits.iter().enumerate() {
        if i == point {
            s.push('.');
        }
        s.push(char::from(chars[usize::from(d % base)]));
    }
    if point == digits.len() {
        // a trailing point, as in "5."
        s.push('.');
    }
    if marker != 0 {
        // `preprocess_sci_string` folds the digits after the point into the exponent by subtracting
        // their count, so an exponent within that count of `i64::MIN` makes the subtraction
        // overflow and the whole string unparseable.
        let exp = exp.max(i64::MIN + i64::exact_from(digits.len()));
        s.push(if marker == 1 { 'e' } else { 'E' });
        // Above base 14 the marker is only recognized when a sign follows it.
        if exp >= 0 && base >= 15 {
            s.push('+');
        }
        s.push_str(&exp.to_string());
    }
    s
}

// The characters that can appear in `strtofr` input: digits, the letters used by the special
// spellings and the base prefixes, signs, points, exponent markers, the NaN-suffix brackets, and
// whitespace. Strings drawn from this alphabet exercise the parser's rejection paths without being
// so unlike real input that they are all rejected in the first character.
pub const STRTOFR_STRING_CHARS: &str = "+-.0123456789abfinxzABFINXZ@epP()_ \t";

// The number of distinct `combo` values for `strtofr_string_from_parts`. The fields are ordered so
// that the fastest-varying ones are the most syntactically distinctive, and index 0 of each is the
// least usual choice: exhaustive generators only reach small values, so a low `combo` must still
// vary the shape of the string rather than only its decoration.
pub const STRTOFR_COMBO_COUNT: u32 = 8 * 5 * 4 * 6 * 3 * 4;

// Assembles a valid `strtofr` input string from its parts, so that generators can build valid
// numbers by construction rather than by filtering. `strtofr` always consumes the whole output, so
// `set_str` accepts it too.
//
// `combo` (which should be less than `STRTOFR_COMBO_COUNT`) selects, from successive fields:
// whether the value is a NaN, an infinity, or a number; an exponent marker; a base prefix or
// special spelling; where the point goes; a sign; and leading whitespace. Choices the base does not
// permit fall back to ones it does, so every `(base, combo)` pair yields valid output. `digits`
// supplies the mantissa digits, reduced into range (an empty slice becomes a single zero), and
// `exp` the exponent, used only when the chosen marker is not "none".
//
// When `rug_compatible` is set the output is further restricted to what rug's own parser accepts,
// which is stricter than MPFR's: no `0x` or `0b` prefix, no `p` binary exponent, and the bare `nan`
// and `inf` spellings only in bases up to 10 rather than up to 16.
pub fn strtofr_string_from_parts(
    base: u8,
    combo: u32,
    digits: &[u8],
    exp: i64,
    rug_compatible: bool,
) -> String {
    // 0 is a NaN and 1 an infinity; 2 through 7 are numbers.
    let kind = combo % 8;
    let marker = (combo / 8) % 6;
    let variant = usize::exact_from((combo / 48) % 4);
    let point = usize::exact_from((combo / 192) % 5);
    let sign = ["", "-", "+"][usize::exact_from((combo / 960) % 3)];
    let whitespace = ["", " ", "\t", " \n\t"][usize::exact_from((combo / 2880) % 4)];
    let mut s = String::new();
    s.push_str(whitespace);
    s.push_str(sign);
    // The bare spellings are accepted in bases up to 16 by MPFR but only up to 10 by rug; the `@`
    // forms work in every base.
    let bare_specials_ok = base <= if rug_compatible { 10 } else { 16 };
    if kind == 0 {
        s.push_str(if bare_specials_ok {
            ["nan", "NaN", "@nan@", "nan(_a1)"][variant]
        } else {
            ["@nan@", "@NAN@", "@Nan@", "@nan@(_a1)"][variant]
        });
        return s;
    }
    if kind == 1 {
        s.push_str(if bare_specials_ok {
            ["inf", "INF", "infinity", "@inf@"][variant]
        } else {
            ["@inf@", "@INF@", "@Inf@", "@inf@"][variant]
        });
        return s;
    }
    // A `0x` prefix is recognized in bases 0 and 16, and `0b` in bases 0 and 2; it also fixes the
    // base the digits must satisfy.
    let prefix = if rug_compatible {
        ""
    } else {
        match variant {
            1 if base == 0 || base == 16 => "0x",
            2 if base == 0 || base == 2 => "0b",
            3 if base == 0 || base == 16 => "0X",
            _ => "",
        }
    };
    let effective_base = match prefix {
        "0x" | "0X" => 16,
        "0b" => 2,
        // a base of 0 with no prefix means decimal
        _ if base == 0 => 10,
        _ => base,
    };
    s.push_str(prefix);
    let default_digits = [0];
    let digits = if digits.is_empty() {
        &default_digits[..]
    } else {
        digits
    };
    let chars: &[u8] = if effective_base <= 36 {
        DIGIT_CHARS_36
    } else {
        DIGIT_CHARS_62
    };
    let point = point % (digits.len() + 1);
    for (i, &d) in digits.iter().enumerate() {
        if i == point {
            s.push('.');
        }
        s.push(char::from(chars[usize::from(d % effective_base)]));
    }
    if point == digits.len() {
        // a trailing point, as in "5."
        s.push('.');
    }
    // `@` marks an exponent in every base; `e` and `E` only when the base is at most 10, since
    // above that they are digits, and `p` and `P`, which mark a binary exponent, only in bases 2
    // and 16. A marker the base does not permit becomes `@`.
    let marker = match marker {
        0 => None,
        2 if !rug_compatible && (effective_base == 2 || effective_base == 16) => Some('p'),
        3 if effective_base <= 10 => Some('e'),
        4 if !rug_compatible && (effective_base == 2 || effective_base == 16) => Some('P'),
        5 if effective_base <= 10 => Some('E'),
        _ => Some('@'),
    };
    if let Some(marker) = marker {
        s.push(marker);
        s.push_str(&exp.to_string());
    }
    s
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

// Whether a `%R` format string's output stays short however large the value's exponent is. The `b`,
// `f`, and `F` conversions write out every digit before the point, which for an extreme exponent
// would be hundreds of millions of them; every other conversion positions the point with an
// exponent instead, and `g` and `G` fall back to `e` style exactly when the exponent is large. The
// conversion character is the last character of the string.
pub fn format_string_output_is_bounded(fmt: &str) -> bool {
    !fmt.ends_with(['b', 'f', 'F'])
}

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

// Pairs each `strtofr` input with the same input in the form rug's `parse_radix` and
// `complete_round` take, so that a library comparison does not pay for the conversion inside the
// timed closure.
pub fn string_u_u_rounding_mode_quadruple_rm(
    xs: It<(String, u8, u64, RoundingMode)>,
) -> It<(
    (String, i32, u32, rug::float::Round),
    (String, u8, u64, RoundingMode),
)> {
    Box::new(xs.map(|(s, base, prec, rm)| {
        (
            (
                s.clone(),
                i32::from(base),
                u32::exact_from(prec),
                rug_round_exact_from_rounding_mode(rm),
            ),
            (s, base, prec, rm),
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

#[allow(clippy::type_complexity)]
pub fn float_float_float_float_quadruple_rm(
    xs: It<(Float, Float, Float, Float)>,
) -> It<(
    (rug::Float, rug::Float, rug::Float, rug::Float),
    (Float, Float, Float, Float),
)> {
    Box::new(xs.map(|(a, b, c, d)| {
        (
            (
                rug::Float::exact_from(&a),
                rug::Float::exact_from(&b),
                rug::Float::exact_from(&c),
                rug::Float::exact_from(&d),
            ),
            (a, b, c, d),
        )
    }))
}

#[allow(clippy::type_complexity)]
pub fn float_float_float_float_anything_quintuple_rm<T: Clone + 'static>(
    xs: It<(Float, Float, Float, Float, T)>,
) -> It<(
    (rug::Float, rug::Float, rug::Float, rug::Float, T),
    (Float, Float, Float, Float, T),
)> {
    Box::new(xs.map(|(a, b, c, d, w)| {
        (
            (
                rug::Float::exact_from(&a),
                rug::Float::exact_from(&b),
                rug::Float::exact_from(&c),
                rug::Float::exact_from(&d),
                w.clone(),
            ),
            (a, b, c, d, w),
        )
    }))
}

#[allow(clippy::type_complexity)]
pub fn float_float_float_float_rounding_mode_quintuple_rm(
    xs: It<(Float, Float, Float, Float, RoundingMode)>,
) -> It<(
    (
        rug::Float,
        rug::Float,
        rug::Float,
        rug::Float,
        rug::float::Round,
    ),
    (Float, Float, Float, Float, RoundingMode),
)> {
    Box::new(
        xs.filter(|(_, _, _, _, rm)| *rm != Exact)
            .map(|(a, b, c, d, rm)| {
                (
                    (
                        rug::Float::exact_from(&a),
                        rug::Float::exact_from(&b),
                        rug::Float::exact_from(&c),
                        rug::Float::exact_from(&d),
                        rug_round_exact_from_rounding_mode(rm),
                    ),
                    (a, b, c, d, rm),
                )
            }),
    )
}

#[allow(clippy::type_complexity)]
pub fn float_float_float_float_anything_rounding_mode_sextuple_rm<T: Clone + 'static>(
    xs: It<(Float, Float, Float, Float, T, RoundingMode)>,
) -> It<(
    (
        rug::Float,
        rug::Float,
        rug::Float,
        rug::Float,
        T,
        rug::float::Round,
    ),
    (Float, Float, Float, Float, T, RoundingMode),
)> {
    Box::new(
        xs.filter(|(_, _, _, _, _, rm)| *rm != Exact)
            .map(|(a, b, c, d, w, rm)| {
                (
                    (
                        rug::Float::exact_from(&a),
                        rug::Float::exact_from(&b),
                        rug::Float::exact_from(&c),
                        rug::Float::exact_from(&d),
                        w.clone(),
                        rug_round_exact_from_rounding_mode(rm),
                    ),
                    (a, b, c, d, w, rm),
                )
            }),
    )
}

pub fn float_float_float_triple_rm(
    xs: It<(Float, Float, Float)>,
) -> It<((rug::Float, rug::Float, rug::Float), (Float, Float, Float))> {
    Box::new(xs.map(|(x, y, z)| {
        (
            (
                rug::Float::exact_from(&x),
                rug::Float::exact_from(&y),
                rug::Float::exact_from(&z),
            ),
            (x, y, z),
        )
    }))
}

#[allow(clippy::type_complexity)]
pub fn float_float_float_anything_quadruple_rm<T: Clone + 'static>(
    xs: It<(Float, Float, Float, T)>,
) -> It<(
    (rug::Float, rug::Float, rug::Float, T),
    (Float, Float, Float, T),
)> {
    Box::new(xs.map(|(x, y, z, w)| {
        (
            (
                rug::Float::exact_from(&x),
                rug::Float::exact_from(&y),
                rug::Float::exact_from(&z),
                w.clone(),
            ),
            (x, y, z, w),
        )
    }))
}

#[allow(clippy::type_complexity)]
pub fn float_float_float_rounding_mode_quadruple_rm(
    xs: It<(Float, Float, Float, RoundingMode)>,
) -> It<(
    (rug::Float, rug::Float, rug::Float, rug::float::Round),
    (Float, Float, Float, RoundingMode),
)> {
    Box::new(
        xs.filter(|(_, _, _, rm)| *rm != Exact)
            .map(|(x, y, z, rm)| {
                (
                    (
                        rug::Float::exact_from(&x),
                        rug::Float::exact_from(&y),
                        rug::Float::exact_from(&z),
                        rug_round_exact_from_rounding_mode(rm),
                    ),
                    (x, y, z, rm),
                )
            }),
    )
}

#[allow(clippy::type_complexity)]
pub fn float_float_float_anything_rounding_mode_quintuple_rm<T: Clone + 'static>(
    xs: It<(Float, Float, Float, T, RoundingMode)>,
) -> It<(
    (rug::Float, rug::Float, rug::Float, T, rug::float::Round),
    (Float, Float, Float, T, RoundingMode),
)> {
    Box::new(
        xs.filter(|(_, _, _, _, rm)| *rm != Exact)
            .map(|(x, y, z, w, rm)| {
                (
                    (
                        rug::Float::exact_from(&x),
                        rug::Float::exact_from(&y),
                        rug::Float::exact_from(&z),
                        w.clone(),
                        rug_round_exact_from_rounding_mode(rm),
                    ),
                    (x, y, z, w, rm),
                )
            }),
    )
}

pub fn float_vec_rm(xs: It<Vec<Float>>) -> It<(Vec<rug::Float>, Vec<Float>)> {
    Box::new(xs.map(|xs| (xs.iter().map(rug::Float::exact_from).collect(), xs)))
}

pub fn float_vec_pair_rm(
    ps: It<(Vec<Float>, Vec<Float>)>,
) -> It<((Vec<rug::Float>, Vec<rug::Float>), (Vec<Float>, Vec<Float>))> {
    Box::new(ps.map(|(xs, ys)| {
        (
            (
                xs.iter().map(rug::Float::exact_from).collect(),
                ys.iter().map(rug::Float::exact_from).collect(),
            ),
            (xs, ys),
        )
    }))
}

pub fn float_vec_anything_rounding_mode_triple_rm<T: Clone + 'static>(
    xs: It<(Vec<Float>, T, RoundingMode)>,
) -> It<(
    (Vec<rug::Float>, T, rug::float::Round),
    (Vec<Float>, T, RoundingMode),
)> {
    Box::new(xs.filter(|(_, _, rm)| *rm != Exact).map(|(v, t, rm)| {
        (
            (
                v.iter().map(rug::Float::exact_from).collect(),
                t.clone(),
                rug_round_exact_from_rounding_mode(rm),
            ),
            (v, t, rm),
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
