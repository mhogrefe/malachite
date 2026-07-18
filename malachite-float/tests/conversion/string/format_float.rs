// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, Pow};
use malachite_base::num::basic::traits::{Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{
    self, Ceiling, Down, Exact, Floor, Nearest, Up,
};
use malachite_float::Float;
use malachite_float::conversion::string::format_float::{
    float_conversion_spec, format_float, format_float_str,
};
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::float_string_pair_gen_var_1;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

// Formats `x` with `fmt` using MPFR's own `mpfr_snprintf` (linked in via rug), the reference we
// must match. Returns `None` when MPFR reports an error.
fn mpfr_format(x: &Float, fmt: &str) -> Option<String> {
    unsafe extern "C" {
        fn mpfr_snprintf(
            buf: *mut core::ffi::c_char,
            n: usize,
            template: *const core::ffi::c_char,
            ...
        ) -> core::ffi::c_int;
    }
    let rf = rug::Float::exact_from(x);
    let template = std::ffi::CString::new(fmt).ok()?;
    let call = |buf: &mut [u8]| unsafe {
        mpfr_snprintf(
            buf.as_mut_ptr().cast(),
            buf.len(),
            template.as_ptr(),
            rf.as_raw(),
        )
    };
    let mut buf = vec![0u8; 1 << 12];
    let mut n = call(&mut buf);
    if n < 0 {
        return None;
    }
    if n as usize >= buf.len() {
        buf = vec![0u8; n as usize + 1];
        n = call(&mut buf);
    }
    Some(String::from_utf8_lossy(&buf[..n as usize]).into_owned())
}

// The base of the conversion specifier `conv`: 16 for 'a'/'A', 2 for 'b', 10 otherwise.
const fn conversion_base(conv: u8) -> u64 {
    match conv {
        b'a' | b'A' => 16,
        b'b' => 2,
        _ => 10,
    }
}

// Reads the base-`base` digit string `digits` (0-9, a-f, or A-F) as a `Natural`.
fn digits_to_natural(digits: &str, base: u64) -> Natural {
    let mut d = Natural::ZERO;
    for c in digits.bytes() {
        let v = match c {
            b'0'..=b'9' => c - b'0',
            b'a'..=b'f' => c - b'a' + 10,
            b'A'..=b'F' => c - b'A' + 10,
            _ => panic!("invalid digit {:?} in {digits:?}", c as char),
        };
        d = d * Natural::from(base) + Natural::from(u32::from(v));
    }
    d
}

// Splits `s` at its decimal point, returning the parts before and after it (the second empty if
// there is no point).
fn split_point(s: &str) -> (&str, &str) {
    s.find('.').map_or((s, ""), |i| (&s[..i], &s[i + 1..]))
}

// Validates a single formatted output `out` of the regular (finite, nonzero, or zero) Float `x`
// under conversion `conv` and rounding mode `rm`. The output is parsed back into the value `v` it
// denotes together with the weight `ulp` of its last digit, and `v` is checked to be the
// correctly-rounded-to-that-precision value of `x`, on the side dictated by `rm`. This is the
// formatting analogue of `get_str`'s `verify_get_str`.
//
// Padding, sign, grouping, and the alternate-form flag do not change the denoted value, so they are
// stripped before parsing.
fn verify_regular_output(x: &Float, out: &str, conv: u8, rm: RoundingMode) {
    let base = conversion_base(conv);
    // strip field-width padding (spaces), a leading sign or space slot, and grouping separators,
    // none of which change the denoted value
    let body = out.trim();
    let neg = body.starts_with('-');
    let body = body.trim_start_matches(['+', '-']).replace(',', "");

    // Parse the concatenated mantissa digit string `all` and the base-`ulp_base` exponent `e` of
    // its last digit, so that the denoted value is `all * ulp_base ^ e`.
    let (all, e, ulp_base): (String, i64, u64) = if base == 10 {
        body.find(['e', 'E']).map_or_else(
            || {
                // fixed: "iii.fff"; the last digit has weight 10 ^ -len(fff)
                let (int_part, frac_part) = split_point(&body);
                (
                    format!("{int_part}{frac_part}"),
                    -i64::exact_from(frac_part.len()),
                    10,
                )
            },
            |pos| {
                // scientific: "d.ffff" "e" "+XX"; the point is after the first digit
                let sci_exp: i64 = body[pos + 1..].parse().unwrap();
                let (int_part, frac_part) = split_point(&body[..pos]);
                (
                    format!("{int_part}{frac_part}"),
                    sci_exp - i64::exact_from(frac_part.len()),
                    10,
                )
            },
        )
    } else {
        // "0x" "h.hhhh" "p" "+XX" for base 16, "b.bbbb" "p" "+XX" for base 2; the exponent after
        // 'p' is a binary exponent, and each digit is `bits` bits wide
        let bits = if base == 16 { 4 } else { 1 };
        let body = body
            .strip_prefix("0x")
            .or_else(|| body.strip_prefix("0X"))
            .unwrap_or(&body);
        let pos = body.find(['p', 'P']).unwrap();
        let bin_exp: i64 = body[pos + 1..].parse().unwrap();
        let (int_part, frac_part) = split_point(&body[..pos]);
        (
            format!("{int_part}{frac_part}"),
            bin_exp - bits * i64::exact_from(frac_part.len()),
            2,
        )
    };

    // v = +-all * ulp_base ^ e is the denoted value; ulp is the weight of the last digit.
    let ulp = Rational::from(ulp_base).pow(e);
    let mut v = Rational::from(digits_to_natural(&all, base)) * &ulp;
    if neg {
        v = -v;
    }
    let x_rat = Rational::exact_from(x);
    // fold the toward-zero / away-from-zero modes onto floor / ceiling using the sign
    let eff = match rm {
        Down => {
            if neg {
                Ceiling
            } else {
                Floor
            }
        }
        Up => {
            if neg {
                Floor
            } else {
                Ceiling
            }
        }
        rm => rm,
    };
    match eff {
        Floor => {
            assert!(v <= x_rat, "{out:?}: {v} > {x_rat}");
            assert!(&x_rat - &v < ulp, "{out:?}: {x_rat} - {v} >= {ulp}");
        }
        Ceiling => {
            assert!(v >= x_rat, "{out:?}: {v} < {x_rat}");
            assert!(&v - &x_rat < ulp, "{out:?}: {v} - {x_rat} >= {ulp}");
        }
        Nearest => assert!(
            (&v - &x_rat).abs() * Rational::TWO <= ulp,
            "{out:?}: |{v} - {x_rat}| > {ulp} / 2"
        ),
        // reachable only when the value is exactly representable (format strings never request
        // Exact; the unit test supplies only exactly-representable Exact cases)
        Exact => assert_eq!(v, x_rat, "{out:?}"),
        _ => unreachable!(),
    }
}

// Whether the single-conversion format string `fmt` requests a precision of 0 (an explicit `.` with
// no digits or `.0`). `fmt` has no surrounding literal text, so the first `.` is the precision.
fn precision_is_zero(fmt: &str) -> bool {
    fmt.find('.').is_some_and(|d| {
        let digits: String = fmt[d + 1..]
            .chars()
            .take_while(char::is_ascii_digit)
            .collect();
        digits.is_empty() || digits == "0"
    })
}

// Validates `format_float_str(x, fmt)` (equivalently `format_float` with the parsed spec), where
// `fmt` is a single-conversion `%R` format string. The output must match MPFR's `mpfr_snprintf`
// exactly, and for a regular value it must additionally denote the correctly-rounded value.
fn verify_format(x: &Float, fmt: &str) {
    let out = format_float_str(x, fmt).unwrap();
    assert!(out.is_ascii());
    let bytes = fmt.as_bytes();
    let conv = *bytes.last().unwrap();
    // the optional rounding character sits just before the conversion character
    let rm = match bytes[bytes.len() - 2] {
        b'D' => Floor,
        b'U' => Ceiling,
        b'Y' => Up,
        b'Z' => Down,
        _ => Nearest,
    };

    // The primary oracle: we must produce exactly what MPFR's own `mpfr_snprintf` produces. There
    // are two documented exceptions, both cases where matching MPFR is impossible or wrong:
    // - the `'` (grouping) flag, whose thousands separator MPFR takes from the locale (empty in the
    //   C locale, so its grouping is not portably comparable); ours always groups with `,`.
    // - `%a`/`%A`/`%b` with a precision of 0, where MPFR's single-digit rounding is buggy — it
    //   rounds exactly-representable values away and overflows its digit table on an all-ones digit
    //   — while we round correctly.
    //
    // Both are instead pinned down by `test_format_float_coverage` and by the reconstruction below.
    if !fmt.contains('\'') && !(matches!(conv, b'a' | b'A' | b'b') && precision_is_zero(fmt)) {
        assert_eq!(
            Some(&out),
            mpfr_format(x, fmt).as_ref(),
            "{fmt:?} disagrees with MPFR"
        );
    }

    // Independent value check: NaN/infinity by their fixed strings, a regular value by
    // reconstructing what its digits denote and confirming it is `x` correctly rounded.
    let uppercase = conv.is_ascii_uppercase();
    if x.is_nan() {
        let core = out.trim().trim_start_matches(['+', '-', ' ']);
        assert_eq!(core, if uppercase { "NAN" } else { "nan" });
    } else if x.is_infinite() {
        let trimmed = out.trim();
        assert_eq!(trimmed.starts_with('-'), x.is_sign_negative());
        let core = trimmed.trim_start_matches(['+', '-', ' ']);
        assert_eq!(core, if uppercase { "INF" } else { "inf" });
    } else {
        verify_regular_output(x, &out, conv, rm);
    }
}

#[test]
fn test_format_float() {
    // format_float, exercised directly through a spec built by float_conversion_spec.
    fn test(s_hex: &str, conv: u8, prec: i64, width: i64, rm: RoundingMode, out: &str) {
        let x = parse_hex_string(s_hex);
        let spec = float_conversion_spec(conv, prec, width, rm);
        assert_eq!(format_float(&x, &spec).unwrap(), out);
        if !x.is_nan() && !x.is_infinite() {
            verify_regular_output(&x, out, conv, rm);
        }
    }
    // scientific, fixed, general (base 10)
    test("0x1.8#2", b'e', 5, 0, Nearest, "1.50000e+00");
    test("-0x1.8#2", b'e', 5, 0, Nearest, "-1.50000e+00");
    test("0x4d2.8#12", b'e', 3, 0, Nearest, "1.234e+03");
    test("0x1.8#2", b'f', 3, 0, Nearest, "1.500");
    test("0x4d2.8#12", b'f', 2, 0, Nearest, "1234.50");
    test("0x1.8#2", b'g', 6, 0, Nearest, "1.5");
    test("0x4d2.8#12", b'g', 6, 0, Nearest, "1234.5");
    test("0xf4240.0#20", b'g', 6, 0, Nearest, "1e+06");
    // hexadecimal and binary
    test("0xff.0#8", b'a', -1, 0, Nearest, "0xf.fp+4");
    test("0x1.8#2", b'a', 2, 0, Nearest, "0x1.80p+0");
    test("0xff.0#8", b'A', -1, 0, Nearest, "0XF.FP+4");
    test("0x1.8#2", b'b', -1, 0, Nearest, "1.1p+0");
    test("0x5.0#3", b'b', -1, 0, Nearest, "1.01p+2");
    // specials
    test("NaN", b'e', 5, 0, Nearest, "nan");
    test("NaN", b'E', 5, 0, Nearest, "NAN");
    test("Infinity", b'f', 3, 0, Nearest, "inf");
    test("-Infinity", b'g', 6, 0, Nearest, "-inf");
    test("0x0.0", b'f', 3, 0, Nearest, "0.000");
    test("0x0.0", b'a', -1, 0, Nearest, "0x0p+0");
    // field width (space padding, right justified)
    test("0x1.8#2", b'f', 2, 8, Nearest, "    1.50");
    test("-0x1.8#2", b'e', 1, 12, Nearest, "    -1.5e+00");
    // rounding of 1.5 to no fractional digits, in every mode
    test("0x1.8#2", b'f', 0, 0, Nearest, "2");
    test("0x1.8#2", b'f', 0, 0, Down, "1");
    test("0x1.8#2", b'f', 0, 0, Up, "2");
    test("0x1.8#2", b'f', 0, 0, Floor, "1");
    test("0x1.8#2", b'f', 0, 0, Ceiling, "2");
    test("-0x1.8#2", b'f', 0, 0, Floor, "-2");
    test("-0x1.8#2", b'f', 0, 0, Ceiling, "-1");
    // the single-hex-digit round-away fix: exact values are never rounded away
    test("0xf.0#4", b'a', 0, 0, Up, "0xfp+0");
    test("0x1.8#2", b'a', 0, 0, Up, "0xcp-3");
    // Exact succeeds exactly when the value is representable in the requested digits
    test("0x0.8#1", b'f', 1, 0, Exact, "0.5");
    test("0x1.8#2", b'a', 0, 0, Exact, "0xcp-3");
}

#[test]
fn test_format_float_none() {
    // A precision that overflows the internal size accounting yields None, rather than panicking or
    // wrapping: `%f`'s `prec + (exp + 1)` and `%g`'s threshold arithmetic overflow i64 and are
    // caught here. (`%e`/`%a`/`%b` with such a precision instead ask get_str for ~2^63 digits
    // directly and run out of memory. That diverges from these conversions only at absurd
    // precisions no real format string uses, which exhaust memory regardless -- and where MPFR does
    // worse: `mpfr_snprintf("%.9223372036854775807Rf", ...)` aborts the whole process on the failed
    // allocation, whereas these paths return None or unwind.)
    let x = parse_hex_string("0x1.8#2");
    assert!(format_float(&x, &float_conversion_spec(b'f', i64::MAX, 0, Nearest)).is_none());
    let small = Float::from(0.001);
    assert!(
        format_float(
            &small,
            &float_conversion_spec(b'g', i64::MAX - 1, 0, Nearest)
        )
        .is_none()
    );
}

#[test]
#[should_panic(expected = "Exact rounding was requested")]
fn format_float_exact_fail() {
    // 1.5625 = 0x1.9 needs five significant bits, so it is not representable in a single base-16
    // digit (unlike 1.5 = 0xcp-3, which is), and Exact must panic.
    let x = parse_hex_string("0x1.9#5");
    format_float(&x, &float_conversion_spec(b'a', 0, 0, Exact));
}

#[test]
fn test_format_float_coverage() {
    // One exemplar per branch of the formatting engine, discovered by instrumenting each branch and
    // taking the first input the property test (or a constructed case) reached it with. Each `//
    // covers:` tag names the branches the case is the first to exercise.
    fn t(hex: &str, fmt: &str, expected: &str) {
        let x = parse_hex_string(hex);
        assert_eq!(
            format_float_str(&x, fmt).as_deref(),
            Some(expected),
            "{hex} {fmt}"
        );
        verify_format(&x, fmt);
    }
    // covers: pn_z_fprec
    t("-0x0.0", "% 11RZf", "  -0.000000");
    // covers: pn_z_gtz
    t("-0x0.0", "%# -RZg", "-0.00000");
    // covers: fg_tiny fg_tiny_1 fg_tiny_up
    t("-0x0.000052ac#13", "%#0 '.3RYf", "-0.001");
    // covers: fg_norm_none
    t(
        "-0x0.012fbd84ae832525a6652c17c7b978#110",
        "%#06.16RNF",
        "-0.0046347092561630",
    );
    // covers: fg_norm_strip
    t("-0x0.63b3894b11a0#45", "%RYG", "-0.389459");
    // covers: fg_exp1 fg_exp1_tz
    t("-0x0.ffffffff#32", "%#0+ -'5.2RYF", "-1.00");
    // covers: ab_neg
    t("-0x1.0#1", "%RNa", "-0x1p+0");
    // covers: eg_expos eg_neg eg_prec
    t("-0x1.1f066E+28#20", "%+7.9RDE", "-5.821556636E+33");
    // covers: nbp_near
    t("-0x1.28ece4c9d66640E-54#54", "%02.0RNb", "-1p-216");
    // covers: sp_padz
    t("-0x1.4214586f02#40", "%0+ 12.3RNe", "-001.258e+00");
    // covers: ab_p0_bump nbp_topz
    t(
        "-0x1.5e1dd004680540b1c76362b187d7d0f5f1e24954a0E-15#166",
        "%#+-16.0RNA",
        "-0XB.P-63       ",
    );
    // covers: ab_b2
    t(
        "-0x1.e257b3b713f2ca30eeb6fc5e60804E-11#115",
        "%#0'11RNb",
        "-1.111000100101011110110011101101110001001111110010110010100011000011101110101101101111110\
        001011110011000001000000001p-44",
    );
    // covers: ab_p0_b2
    t("-0x2192f69.48#33", "%#'1.0RZb", "-1.p+25");
    // covers: ab_p0_npb nbp_away
    t(
        "-0x3.b8da4c3f303120fc532ea95736220939e15f80b7fd846c650caE-12#205",
        "%-2.0RDb",
        "-1p-46",
    );
    // covers: fg_p0_dn sp_padl
    t(
        "-0x4.944dE-9#19",
        "% 28.0RNF",
        "                          -0",
    );
    // covers: pn_g_thresh1
    t(
        "-0x5.88ff974cd01a927868bdc45cE-9#99",
        "%#+-'1.0RZG",
        "-8.E-11",
    );
    // covers: fg_tiny_dn
    t(
        "-0x9.9e58ce8c619156cE-41#63",
        "%#+ -'13.3RZf",
        "-0.000       ",
    );
    // covers: bs_full bs_rle fg_ge1_addtz fg_group pn_tot_group sp_group
    t("-0xd.f0c600E+8#25", "%#0'7.13RUg", "-59,874,082,816.00");
    // covers: fg_ge1 fg_ge1_fp fg_ge1_some fg_ge1_strip pn_g pn_g_bigexp pn_g_cap pn_g_fstyle
    // pn_g_thresh pn_pt_right
    t(
        "-0xf4bfbaf113ee.4d8#57",
        "%+ -.18RZg",
        "-269104312292334.302",
    );
    // covers: pn_sneg sp_sign
    t("-Infinity", "%RNa", "-inf");
    // covers: pn_z_alt sp_pt
    t("0x0.0", "%#RNa", "0x0.p+0");
    // covers: pn_z_ftz pn_z_pt sp_ftz
    t("0x0.0", "%.1RNa", "0x0.0p+0");
    // covers: pn_z_eprec
    t("0x0.0", "%0+2RNe", "+0.0e+00");
    // covers: pn_z_exp pn_z_hexpfx pn_zero sp_exp sp_pfx
    t("0x0.0", "%RNa", "0x0p+0");
    // covers: fg_norm fg_norm_addtz fg_norm_some fg_sub1 fg_sub1_pt fl_ge pn_g_defthresh
    // pn_g_smallexp pn_splus sp_flz
    t(
        "0x0.000d0b7140b8f3aea60aad60c1dc3b2ee0d83e2eba33dcfb6f874df52d78#225",
        "%#+ RDG",
        "+0.000199046",
    );
    // covers: fg_tiny_fl
    t(
        "0x0.0340718bd4b60c249f3315527b76cc3238#127",
        "%#0+ -9.1RDF",
        "+0.0     ",
    );
    // covers: fg_p0 fg_p0_up pn_f pn_pad sp_padr
    t("0x0.0e04#10", "%#0 -4.0RYF", " 1. ");
    // covers: ab_pt
    t("0x1.0#1", "%#RNa", "0x1.p+0");
    // covers: ab_exneg ab_p0 ab_p0_b16 nbp_nornd
    t("0x1.0#1", "%.0RNa", "0x8p-3");
    // covers: ab_fp ab_prec sp_fp
    t("0x1.0#1", "%.1RNa", "0x1.0p+0");
    // covers: ab_b16 ab_expos ab_frac ab_hasprec ab_hexpfx ab_noprec ab_strip pn_ab
    t("0x1.0#1", "%RNa", "0x1p+0");
    // covers: fg_tiny_0 fg_tiny_near
    t("0x2.bfddbE-16#22", "%0+ -11.8RNf", "+0.00000000");
    // covers: fg_tiny_ce
    t("0x3.57fae951E-16#34", "%#+-.9RUF", "+0.000000001");
    // covers: pn_f_defprec
    t("0x5.51cc0E-21#20", "%0+ -RUf", "+0.000001");
    // covers: fl_lt
    t("0x5.a714188778E-9#40", "%#0+-'2RZF", "+0.000000");
    // covers: eg_exneg eg_fp eg_hasfrac eg_none eg_noprec eg_pt pn_e
    t(
        "0x6.055703bef650178E-28#63",
        "%#+ 2RUE",
        "+1.1595752615776271306E-33",
    );
    // covers: eg_some eg_strip pn_g_estyle
    t("0x6.f70E+25#13", "%0+ 1.4RNg", "+8.829e+30");
    // covers: fg_ge1_alt
    t(
        "0x69.6eacadfaaf3fdd3aca95dc4c77f67f7bc#138",
        "%# -.9RDG",
        " 105.432322",
    );
    // covers: fg_ge1_none
    t(
        "0xd.06b0f9ea88b7aE+52#55",
        "%-'5.0RUf",
        "5,358,642,337,391,035,738,579,640,570,997,836,747,766,575,780,392,604,692,404,764,672",
    );
    // covers: ab_upper pn_sspace
    t(
        "0xd4b658b9f48.0#41",
        "%#0 -6.24RZA",
        " 0XD.4B658B9F4800000000000000P+40",
    );
    // covers: pn_inf_z
    t("Infinity", "%0RNa", "inf");
    // covers: pn_inf
    t("Infinity", "%RNa", "inf");
    // covers: pn_nan_z pn_pt_zeros
    t("NaN", "%0RNa", "nan");
    // covers: pn_nan pn_pt_left pn_snone
    t("NaN", "%RNa", "nan");
    // covers: fg_ip_roundup sp_iptz
    t("0x9.8#5", "%.0RUf", "10");
    // covers: bs_rgt
    t("0x9.8#5", "%'.0RUf", "10");
    // covers: bs_last
    t("0x270f.8#16", "%'.0RUf", "10,000");
    // covers: eg_addtz
    t("0x0.00008#1", "%#.20Rg", "7.6293945312500000000e-06");
}

#[test]
fn format_float_properties() {
    // format_float_str(x, fmt) is format_float applied to the spec parsed from fmt, so this
    // property exercises the same partition_number / sprnt_fp core as format_float itself, over the
    // full space of conversions, flags, widths, precisions, and rounding characters.
    float_string_pair_gen_var_1().test_properties(|(x, fmt)| {
        let out = format_float_str(&x, &fmt).unwrap();
        // valid format strings with bounded width/precision always succeed and produce ASCII
        assert!(out.is_ascii());
        // every non-NaN negative value (including negative zero and negative infinity) is signed
        if !x.is_nan() && x.is_sign_negative() {
            assert!(out.trim().starts_with('-'), "{fmt:?} -> {out:?}");
        }
        verify_format(&x, &fmt);
    });
}
