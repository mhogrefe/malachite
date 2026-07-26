// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::float::conversion::string::get_str::get_str;
use malachite_float::float::conversion::string::strtofr::{set_str, strtofr};
use malachite_float::test_util::common::{rug_round_try_from_rounding_mode, to_hex_string};
use malachite_float::test_util::generators::{
    float_signed_unsigned_rounding_mode_quadruple_gen_var_9,
    string_unsigned_unsigned_rounding_mode_quadruple_gen_var_1,
    string_unsigned_unsigned_rounding_mode_quadruple_gen_var_2,
    string_unsigned_unsigned_rounding_mode_quadruple_gen_var_3,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use rug::ops::CompleteRound;
use std::panic::catch_unwind;

// The five rounding modes that never panic.
const INEXACT_MODES: [RoundingMode; 5] = [Floor, Ceiling, Down, Up, Nearest];

// Whether `o` is a direction `rm` is allowed to round a value of the given sign in. `Nearest` may
// go either way, and an exact result is allowed under every mode.
fn ordering_valid_for(o: Ordering, rm: RoundingMode, sign: bool) -> bool {
    match rm {
        Floor => o != Greater,
        Ceiling => o != Less,
        // toward zero
        Down => o == Equal || (o == Less) == sign,
        // away from zero
        Up => o == Equal || (o == Greater) == sign,
        Exact => o == Equal,
        Nearest => true,
    }
}

// Checks every invariant of `strtofr` and `set_str` on one input. Called by both the unit tests and
// the property tests, so that the unit tests get the full check for free.
fn verify_strtofr(s: &str, base: u8, prec: u64, rm: RoundingMode) {
    let (x, o, len) = strtofr(s, base, prec, rm);
    assert!(len <= s.len());
    assert!(x.is_valid());
    // A finite nonzero result always has the requested precision; zeros and the specials carry
    // none.
    if let Some(p) = x.get_prec() {
        assert_eq!(p, prec);
    } else {
        assert!(x.is_nan() || x.is_infinite() || x == 0);
    }
    // `set_str` succeeds exactly when the whole of a nonempty string is consumed, and then agrees.
    let full = set_str(s, base, prec, rm);
    if s.is_empty() || len != s.len() {
        assert!(full.is_none());
    } else {
        let (y, o2) = full.as_ref().unwrap();
        assert_eq!(ComparableFloatRef(y), ComparableFloatRef(&x));
        assert_eq!(*o2, o);
    }
    // Re-parsing just the consumed prefix gives the same value and consumes all of it. The only
    // lookahead is the one character after an exponent marker, and that character is consumed
    // whenever the marker is, so truncating cannot change the reading.
    let (x2, o2, len2) = strtofr(&s[..len], base, prec, rm);
    assert_eq!(ComparableFloat(x2), ComparableFloat(x.clone()));
    assert_eq!(o2, o);
    assert_eq!(len2, len);
    // Nothing was parsed: the result is an exact zero.
    if len == 0 {
        assert_eq!(ComparableFloat(x.clone()), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        return;
    }
    let sign = x.is_sign_positive();
    assert!(ordering_valid_for(o, rm, sign));
    if x.is_nan() {
        assert_eq!(o, Equal);
        return;
    }
    // The directed modes bracket the value, and agreeing modes mean an exact result. This holds
    // even for the infinities, which stand in for values beyond the exponent range.
    let results: Vec<(Float, Ordering)> = INEXACT_MODES
        .iter()
        .map(|&m| {
            let (y, o, _) = strtofr(s, base, prec, m);
            (y, o)
        })
        .collect();
    let (floor, floor_o) = &results[0];
    let (ceiling, ceiling_o) = &results[1];
    assert!(floor <= ceiling);
    for (m, (y, o)) in INEXACT_MODES.iter().zip(&results) {
        assert!(ordering_valid_for(*o, *m, y.is_sign_positive()));
        assert!(floor <= y && y <= ceiling);
    }
    if floor == ceiling {
        // The value is exactly representable, so every mode gives it, exactly.
        assert_eq!(*floor_o, Equal);
        assert_eq!(*ceiling_o, Equal);
        for (y, o) in &results {
            assert_eq!(ComparableFloat(y.clone()), ComparableFloat(floor.clone()));
            assert_eq!(*o, Equal);
        }
        // `Exact` is then accepted and agrees.
        let (y, o, l) = strtofr(s, base, prec, Exact);
        assert_eq!(ComparableFloat(y), ComparableFloat(floor.clone()));
        assert_eq!(o, Equal);
        assert_eq!(l, len);
    } else {
        assert_eq!(*floor_o, Less);
        assert_eq!(*ceiling_o, Greater);
    }
    // `Down` and `Up` are `Floor` and `Ceiling` chosen by the sign.
    let (down, up) = (&results[2].0, &results[3].0);
    if sign {
        assert_eq!(ComparableFloatRef(down), ComparableFloatRef(floor));
        assert_eq!(ComparableFloatRef(up), ComparableFloatRef(ceiling));
    } else {
        assert_eq!(ComparableFloatRef(down), ComparableFloatRef(ceiling));
        assert_eq!(ComparableFloatRef(up), ComparableFloatRef(floor));
    }
    // Cross-check against rug, i.e. MPFR itself, whenever the input is one rug can read. rug's own
    // front end accepts a little more than MPFR's (interior whitespace and underscores), but any
    // such string would stop `strtofr` early and so has already been excluded by the whole-string
    // requirement.
    if (2..=36).contains(&base)
        && full.is_some()
        && let Ok(round) = rug_round_try_from_rounding_mode(rm)
        && let Ok(incomplete) = rug::Float::parse_radix(s, i32::from(base))
    {
        let (rx, ro): (rug::Float, Ordering) =
            incomplete.complete_round(u32::exact_from(prec), round);
        assert_eq!(ComparableFloat(Float::from(&rx)), ComparableFloat(x));
        assert_eq!(ro, o);
    }
}

#[test]
fn test_strtofr() {
    fn test(
        s: &str,
        base: u8,
        prec: u64,
        rm: RoundingMode,
        out: &str,
        out_hex: &str,
        ord: Ordering,
        len: usize,
    ) {
        let (x, o, l) = strtofr(s, base, prec, rm);
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, ord);
        assert_eq!(l, len);
        verify_strtofr(s, base, prec, rm);
    }
    // specials, in every spelling MPFR accepts
    test("nan", 10, 53, Nearest, "NaN", "NaN", Equal, 3);
    test("NaN", 10, 53, Nearest, "NaN", "NaN", Equal, 3);
    test("@nan@", 20, 53, Nearest, "NaN", "NaN", Equal, 5);
    test("nan(_a1)", 10, 53, Nearest, "NaN", "NaN", Equal, 8);
    test("-nan", 10, 53, Nearest, "NaN", "NaN", Equal, 4);
    test("inf", 10, 53, Nearest, "Infinity", "Infinity", Equal, 3);
    test(
        "InFiNiTy", 10, 53, Nearest, "Infinity", "Infinity", Equal, 8,
    );
    test("-inf", 10, 53, Nearest, "-Infinity", "-Infinity", Equal, 4);
    test("@inf@", 62, 53, Nearest, "Infinity", "Infinity", Equal, 5);
    // zeros keep their sign
    test("0", 10, 53, Nearest, "0.0", "0x0.0", Equal, 1);
    test("-0", 10, 53, Nearest, "-0.0", "-0x0.0", Equal, 2);
    test("0.000", 10, 53, Nearest, "0.0", "0x0.0", Equal, 5);
    test("-0.000e100", 10, 53, Nearest, "-0.0", "-0x0.0", Equal, 10);
    // a zero mantissa beats an overflowing exponent
    test(
        "0.@9223372036854775807",
        2,
        53,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
        22,
    );
    // ordinary values
    test("1", 10, 1, Nearest, "1.0", "0x1.0#1", Equal, 1);
    test("1.5", 10, 10, Nearest, "1.5000", "0x1.800#10", Equal, 3);
    test("-1.5", 10, 10, Nearest, "-1.5000", "-0x1.800#10", Equal, 4);
    test(
        "255",
        10,
        53,
        Nearest,
        "255.00000000000000",
        "0xff.000000000000#53",
        Equal,
        3,
    );
    test(
        "3.1415926535897931",
        10,
        53,
        Nearest,
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Greater,
        18,
    );
    // 0.1 is not representable; the modes bracket it
    test("0.1", 10, 4, Floor, "0.0938", "0x0.18#4", Less, 3);
    test("0.1", 10, 4, Ceiling, "0.102", "0x0.1a#4", Greater, 3);
    test("0.1", 10, 4, Down, "0.0938", "0x0.18#4", Less, 3);
    test("0.1", 10, 4, Up, "0.102", "0x0.1a#4", Greater, 3);
    test("0.1", 10, 4, Nearest, "0.102", "0x0.1a#4", Greater, 3);
    test("-0.1", 10, 4, Floor, "-0.102", "-0x0.1a#4", Less, 4);
    test("-0.1", 10, 4, Ceiling, "-0.0938", "-0x0.18#4", Greater, 4);
    // letter digits, case-insensitive up to base 36
    test(
        "ff",
        16,
        53,
        Nearest,
        "255.00000000000000",
        "0xff.000000000000#53",
        Equal,
        2,
    );
    test(
        "FF",
        16,
        53,
        Nearest,
        "255.00000000000000",
        "0xff.000000000000#53",
        Equal,
        2,
    );
    test(
        "zz",
        36,
        53,
        Nearest,
        "1295.0000000000000",
        "0x50f.00000000000#53",
        Equal,
        2,
    );
    // above base 36 the case matters
    test(
        "zz",
        62,
        53,
        Nearest,
        "3843.0000000000000",
        "0xf03.00000000000#53",
        Equal,
        2,
    );
    test(
        "Zz",
        62,
        53,
        Nearest,
        "2231.0000000000000",
        "0x8b7.00000000000#53",
        Equal,
        2,
    );
    // prefixes, and base 0 detection
    test(
        "0x1.8",
        16,
        53,
        Nearest,
        "1.5000000000000000",
        "0x1.8000000000000#53",
        Equal,
        5,
    );
    test(
        "0X1.8",
        0,
        53,
        Nearest,
        "1.5000000000000000",
        "0x1.8000000000000#53",
        Equal,
        5,
    );
    test(
        "0b1.1",
        2,
        53,
        Nearest,
        "1.5000000000000000",
        "0x1.8000000000000#53",
        Equal,
        5,
    );
    test(
        "0b1.1",
        0,
        53,
        Nearest,
        "1.5000000000000000",
        "0x1.8000000000000#53",
        Equal,
        5,
    );
    test(
        "1.5",
        0,
        53,
        Nearest,
        "1.5000000000000000",
        "0x1.8000000000000#53",
        Equal,
        3,
    );
    test("0x", 16, 53, Nearest, "0.0", "0x0.0", Equal, 1);
    // exponents: e and E up to base 10, @ anywhere, p in bases 2 and 16
    test(
        "1e5",
        10,
        53,
        Nearest,
        "100000.00000000000",
        "0x186a0.000000000#53",
        Equal,
        3,
    );
    test(
        "1E-5",
        10,
        53,
        Nearest,
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Greater,
        4,
    );
    test(
        "1@5",
        16,
        53,
        Nearest,
        "1048576.0000000000",
        "0x100000.00000000#53",
        Equal,
        3,
    );
    test(
        "1.8p3",
        16,
        53,
        Nearest,
        "12.000000000000000",
        "0xc.0000000000000#53",
        Equal,
        5,
    );
    test(
        "1.8P-3",
        16,
        53,
        Nearest,
        "0.18750000000000000",
        "0x0.30000000000000#53",
        Equal,
        6,
    );
    test(
        "0x1.8p3",
        0,
        53,
        Nearest,
        "12.000000000000000",
        "0xc.0000000000000#53",
        Equal,
        7,
    );
    // leading whitespace is skipped, trailing is not
    test(
        "  \t 1.5",
        10,
        53,
        Nearest,
        "1.5000000000000000",
        "0x1.8000000000000#53",
        Equal,
        7,
    );
    test(
        "1.5 ",
        10,
        53,
        Nearest,
        "1.5000000000000000",
        "0x1.8000000000000#53",
        Equal,
        3,
    );
    // partial parses stop at the first character that cannot continue
    test(
        "1.5x",
        10,
        53,
        Nearest,
        "1.5000000000000000",
        "0x1.8000000000000#53",
        Equal,
        3,
    );
    test(
        "ffe-2",
        16,
        53,
        Nearest,
        "4094.0000000000000",
        "0xffe.00000000000#53",
        Equal,
        3,
    );
    test(
        "1.0E+25",
        16,
        53,
        Nearest,
        "1.0546875000000000",
        "0x1.0e00000000000#53",
        Equal,
        4,
    );
    test(
        "1e",
        10,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
        1,
    );
    test(
        "1e 5",
        10,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
        1,
    );
    test(
        "1.2.3",
        10,
        53,
        Nearest,
        "1.2000000000000000",
        "0x1.3333333333333#53",
        Less,
        3,
    );
    test("nanx", 10, 53, Nearest, "NaN", "NaN", Equal, 3);
    // nothing parses at all
    test("", 10, 53, Nearest, "0.0", "0x0.0", Equal, 0);
    test(".", 10, 53, Nearest, "0.0", "0x0.0", Equal, 0);
    test("x", 10, 53, Nearest, "0.0", "0x0.0", Equal, 0);
    test("-", 10, 53, Nearest, "0.0", "0x0.0", Equal, 0);
    // overflow and underflow, with the mode deciding
    test(
        "1e1000000000",
        10,
        53,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
        12,
    );
    test(
        "1e1000000000",
        10,
        53,
        Down,
        "2.0985787164673875e323228496",
        "0x7.ffffffffffffcE+268435455#53",
        Less,
        12,
    );
    test(
        "-1e1000000000",
        10,
        53,
        Ceiling,
        "-2.0985787164673875e323228496",
        "-0x7.ffffffffffffcE+268435455#53",
        Greater,
        13,
    );
    test("1e-1000000000", 10, 53, Nearest, "0.0", "0x0.0", Less, 13);
    test(
        "1e-1000000000",
        10,
        53,
        Up,
        "2.3825649048879511e-323228497",
        "0x1.0000000000000E-268435456#53",
        Greater,
        13,
    );
    test(
        "-1e-1000000000",
        10,
        53,
        Floor,
        "-2.3825649048879511e-323228497",
        "-0x1.0000000000000E-268435456#53",
        Less,
        14,
    );
    // the exponent saturates at the i64 bounds
    test(
        "1e9223372036854775807",
        10,
        53,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
        21,
    );
    test(
        "1e-9223372036854775809",
        10,
        53,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
        22,
    );

    // Cases found by the branch-coverage pass; each is the first input reaching the branch named
    // beside it.
    // - trailing zeros stripped from the mantissa
    test(
        "1.10",
        10,
        53,
        Nearest,
        "1.1000000000000001",
        "0x1.199999999999a#53",
        Greater,
        4,
    );
    // - a NaN bracket whose contents are not all alphanumeric or `_`, so it is not consumed
    test("nan(a-b)", 10, 53, Nearest, "NaN", "NaN", Equal, 3);
    // - an unterminated NaN bracket
    test("nan(abc", 10, 53, Nearest, "NaN", "NaN", Equal, 3);
    // - an exponent too long for i64, saturating
    test(
        "1e99999999999999999999",
        10,
        53,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
        22,
    );
    // - the same, negative
    test(
        "1e-99999999999999999999",
        10,
        53,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
        23,
    );
    // - digits filling one limb more than the working precision, so the significand is shifted
    //   right
    test(
        "18446744073709551616",
        10,
        1,
        Nearest,
        "1.8e19",
        "0x1.0E+16#1",
        Equal,
        20,
    );
    // - more digits than the working precision can use: the tail is dropped, and the Ziv loop
    //   retries
    test(
        "340282366920938463463374607431768211456",
        10,
        2,
        Nearest,
        "3.4e38",
        "0x1.0E+32#2",
        Equal,
        39,
    );
    // - a power-of-two base whose digits fill whole limbs, and whose rounding carries into a new
    //   one
    test(
        "ffffffffffffffffffffffffffffffff",
        16,
        3,
        Nearest,
        "3.4e38",
        "0x1.0E+32#3",
        Greater,
        32,
    );
    // - an exponent so large that computing base^exp overflows
    test(
        "1@4510998398579621261",
        35,
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
        21,
    );
    // - the most negative i64 exponent, which cannot be negated
    test(
        "-.G@-9223372036854775808",
        46,
        71,
        Nearest,
        "-0.0",
        "-0x0.0",
        Greater,
        24,
    );
    // - an exponent so small that computing base^-exp underflows
    test(
        ".628@-5787935179462733366",
        9,
        27,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
        25,
    );
    // - a binary exponent that overflows only once the significand's own exponent is added
    test(
        "-010.00101000001100010010101111010011111111111111111111111111111111111\
         111111111111P9223372036854775807",
        2,
        125,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
        102,
    );
    // - a binary exponent marker with no digits after it, so it is not consumed
    test(
        "1.8p",
        16,
        53,
        Nearest,
        "1.5000000000000000",
        "0x1.8000000000000#53",
        Equal,
        3,
    );
}

#[test]
fn test_set_str() {
    fn test(s: &str, base: u8, prec: u64, rm: RoundingMode, out: Option<(&str, &str, Ordering)>) {
        let x = set_str(s, base, prec, rm);
        assert_eq!(
            x.as_ref()
                .map(|(x, o)| (x.to_string(), to_hex_string(x), *o)),
            out.map(|(s, s_hex, o)| (s.to_string(), s_hex.to_string(), o))
        );
        verify_strtofr(s, base, prec, rm);
    }
    // `set_str` is `strtofr` restricted to strings that are consumed in full.
    test(
        "1.5",
        10,
        10,
        Nearest,
        Some(("1.5000", "0x1.800#10", Equal)),
    );
    test(
        "-inf",
        10,
        53,
        Nearest,
        Some(("-Infinity", "-Infinity", Equal)),
    );
    test(
        "0x1.8",
        16,
        53,
        Nearest,
        Some(("1.5000000000000000", "0x1.8000000000000#53", Equal)),
    );
    // Anything left over is a failure, even a single trailing space.
    test("1.5x", 10, 53, Nearest, None);
    test("1.5 ", 10, 53, Nearest, None);
    test("ffe-2", 16, 53, Nearest, None);
    test("1e", 10, 53, Nearest, None);
    // "0x" reads as the digit 0 with the "x" left over.
    test("0x", 16, 53, Nearest, None);
    // The empty string never parses.
    test("", 10, 53, Nearest, None);
    test(".", 10, 53, Nearest, None);
}

#[test]
fn test_strtofr_exact_panics() {
    // `Exact` panics when the value is not exactly representable with the given precision.
    assert_panic!(strtofr("0.1", 10, 10, Exact));
    assert_panic!(strtofr("1.1", 10, 1, Exact));
    // Overflow and underflow are inexact too.
    assert_panic!(strtofr("1e1000000000", 10, 53, Exact));
    assert_panic!(strtofr("1e-1000000000", 10, 53, Exact));
}

#[test]
fn test_strtofr_bad_args_panics() {
    assert_panic!(strtofr("1", 1, 53, Nearest));
    assert_panic!(strtofr("1", 63, 53, Nearest));
    assert_panic!(strtofr("1", 10, 0, Nearest));
    assert_panic!(set_str("1", 10, 0, Nearest));
}

#[test]
fn strtofr_high_precision_dropped_limb() {
    // A regression test for the working buffer in `limbs_set_str`. MPFR allows the digits' value to
    // occupy at most one limb more than the working precision, which holds when the number of
    // digits read is the exact ceiling of ysize_bits / log2(base). It is computed instead from a
    // table of rational upper approximations to 1 / log2(base), and that table's relative slack (up
    // to 3.7e-5) is multiplied by ysize_bits, so the excess grows without bound and no fixed
    // allowance suffices. Base 21 has the largest slack and so reaches each threshold at the lowest
    // precision.
    //
    // rug is deliberately not the oracle here: MPFR gets these very cases wrong (see the note in
    // `limbs_set_str`). Instead the result is rendered back with `get_str`, which is tested
    // separately, and compared against the value the input obviously denotes: n digits of the
    // largest digit is base^n - 1, which rounds to base^n at any of these precisions.
    fn test(prec: u64, digits: usize) {
        let s: String = core::iter::repeat_n('k', digits).collect();
        let (x, _, len) = strtofr(&s, 21, prec, Nearest);
        assert_eq!(len, s.len());
        assert_eq!(x.get_prec(), Some(prec));
        // base^n - 1 is within a rounding error of base^n, i.e. of 0.1 * base^(n + 1).
        let (rendered, exp, _) = get_str(&x, 21, 20, Nearest).unwrap();
        assert_eq!(String::from_utf8(rendered).unwrap(), "10000000000000000000");
        assert_eq!(exp, i64::exact_from(digits) + 1);
    }
    // The least precision at which the value needs one limb more than MPFR allows, so a low limb
    // must be dropped during normalization.
    test(1_561_772, 356_000);
    // The least precision at which it needs two more, which also overruns a buffer sized by MPFR's
    // fixed allowance.
    test(3_340_000, 762_000);
}

#[test]
fn strtofr_properties() {
    string_unsigned_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(s, base, prec, rm)| {
            // This generator only builds strings that parse in full; without checking that, a
            // parser that wrongly rejected one would look like it had legitimately stopped early,
            // and the rug cross-check below would be skipped along with it.
            assert_eq!(strtofr(&s, base, prec, rm).2, s.len(), "{s:?} base {base}");
            verify_strtofr(&s, base, prec, rm);
        },
    );
    // The same invariants over inputs rug also accepts, so that every case exercises the rug
    // cross-check inside `verify_strtofr`.
    string_unsigned_unsigned_rounding_mode_quadruple_gen_var_2().test_properties(
        |(s, base, prec, rm)| {
            assert_eq!(strtofr(&s, base, prec, rm).2, s.len(), "{s:?} base {base}");
            verify_strtofr(&s, base, prec, rm);
        },
    );
    // Mostly-invalid strings, for the parser's rejection paths.
    string_unsigned_unsigned_rounding_mode_quadruple_gen_var_3().test_properties(
        |(s, base, prec, rm)| {
            verify_strtofr(&s, base, prec, rm);
        },
    );
}

#[test]
fn strtofr_get_str_round_trip_properties() {
    // `get_str` with a digit count of 0 produces the fewest digits that identify the value, so
    // re-reading them at the same precision must give it back exactly. This ties the two MPFR
    // string ports together: `mpfr_get_str` out, `mpfr_strtofr` back in.
    float_signed_unsigned_rounding_mode_quadruple_gen_var_9().test_properties(|(x, b0, _, _)| {
        if !x.is_finite() || x == 0 {
            return;
        }
        let Ok(base) = u8::try_from(b0.unsigned_abs()) else {
            return;
        };
        let prec = x.get_prec().unwrap();
        // The digits are the fewest that identify `x`, but in a base that is not a power of two
        // they are still a rounded rendering of it, so neither this ternary value nor the one from
        // reading them back is `Equal`. What round-trips is the value.
        let (digits, exp, _) = get_str(&x, b0, 0, Nearest).unwrap();
        // `get_str` returns the digits of 0.d1 d2 ... times base^exp, with a leading `-` for a
        // negative value; `strtofr` reads exactly that as `.d1 d2 ...@exp`.
        let digits = String::from_utf8(digits).unwrap();
        let (sign, digits) = digits
            .strip_prefix('-')
            .map_or(("", digits.as_str()), |rest| ("-", rest));
        let s = format!("{sign}.{digits}@{exp}");
        let (y, _, len) = strtofr(&s, base, prec, Nearest);
        assert_eq!(len, s.len());
        assert_eq!(ComparableFloat(y), ComparableFloat(x));
    });
}
