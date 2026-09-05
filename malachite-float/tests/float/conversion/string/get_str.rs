// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, Equal, Greater, Less};
use malachite_base::num::arithmetic::traits::{Abs, CheckedLogBase2, DivRound, Pow, PowerOf2};
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, IntegerMantissaAndExponent};
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::float::conversion::string::get_str::{get_str, get_str_digit_count};
use malachite_float::test_util::common::{parse_hex_string, rug_round_try_from_rounding_mode};
use malachite_float::test_util::generators::{
    float_signed_unsigned_rounding_mode_quadruple_gen_var_9,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_10,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_15,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_16, unsigned_pair_gen_var_51,
};
use malachite_float::{ComparableFloatRef, Float};
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use std::panic::catch_unwind;

// Inverts the `num_to_text` tables: maps an output character to its base value.
fn digit_value(c: u8, large_table: bool) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'A'..=b'Z' => c - b'A' + 10,
        b'a'..=b'z' if large_table => c - b'a' + 36,
        b'a'..=b'z' => c - b'a' + 10,
        _ => panic!("invalid digit character {c}"),
    }
}

// An oracle for the digits that neither the exact reconstruction nor rug can supply: an extreme
// exponent, where `b ^ exp` as an exact integer would have around a billion bits, together with a
// base above 36 or below zero, or a mode rug does not have. Rather than computing that power, we
// bracket it, which is enough because the checks only need a resolution of one unit in the last of
// at most 20 digits.

// The working precision, in bits, of `power_bracket`. The bracket's relative width roughly doubles
// per squaring, so with a 30-bit exponent it is still around `2 ^ -220` -- far finer than the
// resolution these checks need, which is about `2 ^ -119` for 20 digits of base 62.
const POWER_PREC: u64 = 256;
// Retried at this precision when the first bracket cannot decide a comparison, which happens only
// when the two sides agree to more bits than the bracket resolves.
const POWER_PREC_RETRY: u64 = 2048;

// Brackets `b ^ e` as `[lo, lo + err) * 2 ^ shift`, with `lo` of about `prec` bits. Deliberately
// naive -- binary exponentiation with the running value truncated to `prec` bits and the truncation
// error carried alongside it -- so that it shares nothing with the scaling `get_str` does itself.
fn power_bracket(b: u64, e: u64, prec: u64) -> (Natural, Natural, i64) {
    let (mut lo, mut err, mut shift) = (Natural::ONE, Natural::ZERO, 0i64);
    for i in (0..e.significant_bits()).rev() {
        // Squaring `[L, L + E)` gives `[L ^ 2, L ^ 2 + 2LE + E ^ 2)`.
        err = ((&lo * &err) << 1u32) + (&err * &err);
        lo = &lo * &lo;
        shift *= 2;
        if e.get_bit(i) {
            // Multiplying by `b` scales both ends exactly.
            lo *= Natural::from(b);
            err *= Natural::from(b);
        }
        // Dropping the low `j` bits of `lo` loses less than one unit from each end of the bracket.
        let bits = lo.significant_bits();
        if bits > prec {
            let j = bits - prec;
            lo >>= j;
            err = (err >> j) + Natural::TWO;
            shift += i64::exact_from(j);
        }
    }
    (lo, err, shift)
}

// Compares `a * 2 ^ u` with `c * 2 ^ w`. The bit lengths settle it unless they agree, and when they
// agree `u - w` is the difference of the operands' own bit lengths -- so a wildly wrong exponent
// decides the comparison rather than provoking a huge shift.
fn cmp_scaled(a: &Natural, u: i64, c: &Natural, w: i64) -> Ordering {
    if *a == 0u32 || *c == 0u32 {
        return a.cmp(c);
    }
    let la = i64::exact_from(a.significant_bits()) + u;
    let lc = i64::exact_from(c.significant_bits()) + w;
    if la != lc {
        return la.cmp(&lc);
    }
    let d = u - w;
    if d >= 0 {
        (a << u64::exact_from(d)).cmp(c)
    } else {
        a.cmp(&(c << u64::exact_from(-d)))
    }
}

// Compares `mag * 2 ^ mag_exp`, a nonnegative value, with `n * b ^ t`. `None` when the bracket
// cannot decide it even after the retry, which means the two agree to thousands of bits.
fn cmp_mag_vs(mag: &Natural, mag_exp: i64, n: &Natural, b: u64, t: i64) -> Option<Ordering> {
    for prec in [POWER_PREC, POWER_PREC_RETRY] {
        let (lo, err, k) = power_bracket(b, t.unsigned_abs(), prec);
        let hi = &lo + &err;
        let decided = if t >= 0 {
            // `n * b ^ t` lies in `[n * lo, n * hi) * 2 ^ k`.
            if cmp_scaled(mag, mag_exp, &(n * &lo), k) == Less {
                Some(Less)
            } else if cmp_scaled(mag, mag_exp, &(n * &hi), k) != Less {
                Some(Greater)
            } else {
                None
            }
        } else {
            // Scale both sides by `b ^ -t`, so that the value compared against `n` is the one
            // bracketed: it lies in `[mag * lo, mag * hi) * 2 ^ (mag_exp + k)`.
            if cmp_scaled(&(mag * &hi), mag_exp + k, n, 0) != Greater {
                Some(Less)
            } else if cmp_scaled(&(mag * &lo), mag_exp + k, n, 0) == Greater {
                Some(Greater)
            } else {
                None
            }
        };
        if decided.is_some() {
            return decided;
        }
    }
    None
}

// The half of `verify_get_str` that checks the digits against `x`, for the exponents where building
// either as a `Rational` is unaffordable. `d` is the digit string's value, so the digits denote
// `+-d * b ^ (exp - m_actual)`.
fn verify_get_str_bracketed(
    x: &Float,
    neg: bool,
    d: &Natural,
    b: u64,
    exp: i64,
    m_actual: usize,
    rnd: RoundingMode,
    ord: Ordering,
) {
    let (mag, mag_exp) = x.integer_mantissa_and_exponent();
    let t = exp - i64::exact_from(m_actual);
    // Fold the sign in, so that every check below is on magnitudes: rounding a negative value
    // toward negative infinity raises its magnitude, and rounding one toward zero lowers it.
    let mag_rnd = match (rnd, neg) {
        (Down, _) | (Floor, false) | (Ceiling, true) => Floor,
        (Up, _) | (Ceiling, false) | (Floor, true) => Ceiling,
        (rnd, _) => rnd,
    };
    let cmp = |n: &Natural| cmp_mag_vs(&mag, mag_exp, n, b, t);
    let one = Natural::ONE;
    let ctx = format!("{x} base {b} m {m_actual} {rnd}");
    // The representable value just below the digits is not always `(d - 1) * b ^ t`. When `d` is
    // the smallest m-digit mantissa, `b ^ (m - 1)`, dropping to `d - 1` would leave only `m - 1`
    // digits, so the real predecessor is the largest m-digit mantissa an exponent lower, `(b ^ m -
    // 1) * b ^ (t - 1)` -- a factor of nearly `b` closer. Using `d - 1` there loosens the bound by
    // that factor, which is exactly enough slack to admit an exponent one too large.
    let b_nat = Natural::from(b);
    let m_bits = u64::exact_from(m_actual);
    let smallest = (&b_nat).pow(m_bits - 1);
    let largest = (&b_nat).pow(m_bits) - &one;
    let narrow = *d == smallest;
    let (pred_n, pred_t) = if narrow {
        (largest.clone(), t - 1)
    } else {
        (d - &one, t)
    };
    match mag_rnd {
        // d * b ^ t <= |x| < (d + 1) * b ^ t
        Floor => {
            assert_ne!(cmp(d), Some(Less), "{ctx}: |x| below the digits");
            assert_eq!(
                cmp(&(d + &one)),
                Some(Less),
                "{ctx}: |x| at or above the next digit string"
            );
        }
        // pred < |x| <= d * b ^ t
        Ceiling => {
            assert_ne!(cmp(d), Some(Greater), "{ctx}: |x| above the digits");
            assert_eq!(
                cmp_mag_vs(&mag, mag_exp, &pred_n, b, pred_t),
                Some(Greater),
                "{ctx}: |x| at or below the previous digit string"
            );
        }
        // The midpoints on either side bracket |x|. Below the digits that midpoint is halfway to
        // `pred`, which for the narrow case is a shorter step than half an ulp.
        Nearest => {
            let two_d = d << 1u32;
            let (lo_mid_n, lo_mid_t) = if narrow {
                ((&two_d * &b_nat) - &one, t - 1)
            } else {
                (&two_d - &one, t)
            };
            assert_ne!(
                cmp_mag_vs(&mag, mag_exp + 1, &lo_mid_n, b, lo_mid_t),
                Some(Less),
                "{ctx}: |x| more than halfway toward the previous digit string"
            );
            assert_ne!(
                cmp_mag_vs(&mag, mag_exp + 1, &(&two_d + &one), b, t),
                Some(Greater),
                "{ctx}: |x| more than half an ulp above the digits"
            );
        }
        // Exactness is what the all-modes-agree check above establishes; the bracket cannot prove
        // an equality, only bound a difference.
        Exact => {}
        _ => unreachable!(),
    }
    // The returned ordering is the rounded value compared to x, which on magnitudes is the digits
    // compared to |x|, reversed for a negative value. An undecided bracket means the two agree to
    // thousands of bits, so there is nothing to check.
    if let Some(c) = cmp(d) {
        assert_eq!(ord, if neg { c } else { c.reverse() }, "{ctx}: ordering");
    }
}

// The half of `verify_get_str` that rebuilds the digits and `x` as exact `Rational`s. Affordable
// only for a moderate exponent; `verify_get_str_bracketed` covers the rest.
fn verify_get_str_exactly(
    x: &Float,
    neg: bool,
    d: &Natural,
    b: u64,
    exp: i64,
    m_actual: usize,
    rnd: RoundingMode,
    ord: Ordering,
) {
    let ulp = Rational::from(b).pow(exp - i64::exact_from(m_actual));
    // The step from the digits to the neighbouring representable value is a full ulp except on the
    // side toward zero, and there only when `d` is the smallest m-digit mantissa: dropping to `d -
    // 1` would leave `m - 1` digits, so the neighbour is the largest mantissa an exponent lower,
    // only `ulp / b` away. Allowing a full ulp there is loose enough to admit an exponent one too
    // large. Which side that is depends on the sign.
    let narrow = *d == Natural::from(b).pow(u64::exact_from(m_actual) - 1);
    let toward_zero = if narrow {
        &ulp / Rational::from(b)
    } else {
        ulp.clone()
    };
    let (below, above) = if neg {
        (ulp.clone(), toward_zero)
    } else {
        (toward_zero, ulp.clone())
    };
    let mut v = Rational::from(d.clone()) * &ulp;
    if neg {
        v = -v;
    }
    let x_rat = Rational::exact_from(x);
    // V correctly rounds x to m digits, on the side dictated by the mode (folding the toward/away
    // modes onto floor/ceiling using the sign).
    let eff = match rnd {
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
        rnd => rnd,
    };
    match eff {
        Floor => {
            assert!(v <= x_rat);
            assert!(&x_rat - &v < above);
        }
        Ceiling => {
            assert!(v >= x_rat);
            assert!(&v - &x_rat < below);
        }
        Nearest => {
            let gap = if v >= x_rat { &below } else { &above };
            assert!((&v - &x_rat).abs() * Rational::TWO <= *gap);
        }
        // We only reach here for `Exact` when the value is exactly representable (the inexact case
        // is filtered out above), so the digits must reconstruct `x` precisely.
        Exact => assert_eq!(v, x_rat),
        _ => unreachable!(),
    }
    // The returned ordering is the result compared to x.
    assert_eq!(ord, v.cmp(&x_rat));
    // It is also consistent with the rounding mode and the sign of x: a directed mode can only err
    // to one side, with Down/Up depending on the sign (cf. div.rs).
    match (x_rat >= 0u32, rnd) {
        (_, Floor) | (true, Down) | (false, Up) => assert_ne!(ord, Greater),
        (_, Ceiling) | (true, Up) | (false, Down) => assert_ne!(ord, Less),
        (_, Exact) => assert_eq!(ord, Equal),
        _ => {}
    }
}

// Validates `get_str(x, b0, m, rnd)`. Invalid bases give `None`; special values give their fixed
// strings. For a finite nonzero `x`, the m output digits read as an integer `D` reconstruct the
// value `V = +-D * b ^ (exp - m)`, which must be the correctly-rounded-to-m-digits value of the
// exact `x` (within one ulp, on the side dictated by the rounding mode), and the returned
// `Ordering` must equal `V` compared to `x`. For bases 2..=36 the digit string and exponent are
// also cross-checked against rug (MPFR).
//
// The largest base-`b` exponent for which the value is rebuilt as a `Rational`. This is the same
// bound `float_to_sci_options_valid` uses, and for the same reason.
const MAX_RATIONAL_EXPONENT: u64 = 10_000;

fn verify_get_str(x: &Float, b0: i64, m: usize, rnd: RoundingMode) {
    if !((-36..=-2).contains(&b0) || (2..=62).contains(&b0)) {
        assert!(get_str(x, b0, m, rnd).is_none());
        return;
    }
    // `Exact` is only ever paired with exactly-representable values here (the generators filter out
    // the would-panic cases via `valid_float_get_str_quadruple`; the panic itself is checked in
    // `test_get_str_exact_panics`), so this call never panics.
    let (digits, exp, ord) = get_str(x, b0, m, rnd).unwrap();
    let b = b0.unsigned_abs();
    if x.is_nan() {
        assert_eq!(digits, b"@NaN@");
        assert_eq!(ord, Equal);
        return;
    }
    if x.is_infinite() {
        let expected: &[u8] = if x.is_sign_negative() {
            b"-@Inf@"
        } else {
            b"@Inf@"
        };
        assert_eq!(digits, expected);
        assert_eq!(ord, Equal);
        return;
    }
    if x.is_zero() {
        let mut expected = vec![b'0'; if m == 0 { 1 } else { m }];
        if x.is_sign_negative() {
            expected.insert(0, b'-');
        }
        assert_eq!(digits, expected);
        assert_eq!(ord, Equal);
        return;
    }
    // finite nonzero
    let neg = digits[0] == b'-';
    let digit_bytes = &digits[usize::from(neg)..];
    let m_actual = digit_bytes.len();
    if m != 0 {
        assert_eq!(m_actual, m);
    }
    let large_table = !(2..=36).contains(&b0);
    let mut d = Natural::ZERO;
    for &c in digit_bytes {
        d = d * Natural::from(b) + Natural::from(digit_value(c, large_table));
    }
    // D has exactly m significant base-b digits.
    assert!(d >= Natural::from(b).pow(u64::exact_from(m_actual - 1)));
    assert!(d < Natural::from(b).pow(u64::exact_from(m_actual)));

    // V = +-D * b ^ (exp - m), the value the digits represent; ulp is the weight of the last digit.
    // Both are `Rational`s, which is only affordable for a moderate exponent: `b ^ exp` and the
    // exact value of a `Float` with an extreme exponent are integers of around a billion bits. The
    // rug cross-check below is what covers those, and it is the stronger oracle anyway.
    if exp.unsigned_abs() > MAX_RATIONAL_EXPONENT {
        verify_get_str_bracketed(x, neg, &d, b, exp, m_actual, rnd, ord);
    } else {
        verify_get_str_exactly(x, neg, &d, b, exp, m_actual, rnd, ord);
    }

    // A negative base is base `|base|` written with uppercase letters, so the two calls must agree
    // exactly. This holds at every exponent, and hands the negative bases to whichever oracle
    // covers the positive one -- rug, for the `|base| <= 36` that negative bases are limited to.
    if b0 < 0 {
        let (pos_digits, pos_exp, pos_ord) = get_str(x, -b0, m, rnd).unwrap();
        assert_eq!(digits, pos_digits.to_ascii_uppercase());
        assert_eq!(exp, pos_exp);
        assert_eq!(ord, pos_ord);
    }
    // An exactly representable value has the same digits under every mode, which is what makes
    // `Exact` meaningful. This is what covers `Exact` where rug cannot: rug has no such mode, but
    // it does check the other five, and here they are all the same call.
    if ord == Equal {
        for other in [Floor, Ceiling, Down, Up, Nearest, Exact] {
            assert_eq!(
                get_str(x, b0, m, other).unwrap(),
                (digits.clone(), exp, Equal),
                "{x} base {b0} m {m} disagrees under {other}"
            );
        }
    }

    // Cross-check the digit string and exponent against rug (MPFR), where it applies.
    if (2..=36).contains(&b0)
        && let Ok(round) = rug_round_try_from_rounding_mode(rnd)
    {
        let (rug_neg, rug_mant, rug_exp) = rug::Float::exact_from(x).to_sign_string_exp_round(
            i32::exact_from(b),
            Some(m_actual),
            round,
        );
        assert_eq!(neg, rug_neg);
        assert_eq!(digit_bytes, rug_mant.as_bytes());
        assert_eq!(exp, i64::from(rug_exp.unwrap()));
    }
}

#[test]
fn test_get_str() {
    fn test(
        s: &str,
        s_hex: &str,
        b0: i64,
        m: usize,
        rnd: RoundingMode,
        out: &str,
        exp: i64,
        ord: Ordering,
    ) {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let (digits, e, o) = get_str(&x, b0, m, rnd).unwrap();
        assert_eq!(std::str::from_utf8(&digits).unwrap(), out);
        assert_eq!(e, exp);
        assert_eq!(o, ord);
        verify_get_str(&x, b0, m, rnd);
    }
    // Illustrative cases. 1.25 = 0x1.4
    test("1.2", "0x1.4#3", 10, 3, Nearest, "125", 1, Equal);
    test("1.2", "0x1.4#3", 2, 3, Nearest, "101", 1, Equal);
    // 1/3 rounded to 4 base-10 digits: down vs up
    test(
        "0.333333343",
        "0x0.5555558#25",
        10,
        4,
        Floor,
        "3333",
        0,
        Less,
    );
    test(
        "0.333333343",
        "0x0.5555558#25",
        10,
        4,
        Ceiling,
        "3334",
        0,
        Greater,
    );

    // Each of the following inputs first exercises the noted branch(es) of the get_str pipeline
    // (get_str, limbs_get_str, limbs_get_str_power_of_2, limbs_get_str_aux). They were captured by
    // a single branch-recording run over the demo generators; the `// covers:` tags name the
    // branches each input is the first to reach. covers: g_okbase g_nan
    test("NaN", "NaN", 2, 0, Down, "@NaN@", 0, Equal);
    // covers: g_pinf
    test("Infinity", "Infinity", 2, 0, Down, "@Inf@", 0, Equal);
    // covers: g_ninf
    test("-Infinity", "-Infinity", 2, 0, Down, "-@Inf@", 0, Equal);
    // covers: g_pzero g_zm0 g_fpos
    test("0.0", "0x0.0", 2, 0, Down, "0", 0, Equal);
    // covers: g_zmn
    test("0.0", "0x0.0", 2, 1, Down, "0", 0, Equal);
    // covers: g_nzero g_fneg
    test("-0.0", "-0x0.0", 2, 0, Down, "-0", 0, Equal);
    // covers: g_finite g_m0 nd_pow2 g_pos g_pow2 p2_rgt p2_nocarry p2_nb p2_topnz p2_t36
    test("1.0", "0x1.0#1", 2, 0, Down, "1", 1, Equal);
    // covers: g_mn
    test("1.0", "0x1.0#1", 2, 1, Down, "1", 1, Equal);
    // covers: nd_npow2 nd_small g_npow2 g_expnz mgt_g mgt_x1hi mgt_nxle mgt_err0 mgt_lo_no
    // mgt_shift exact_y ax_exact ax_t36 ax_round ax_shr ax_ndr ret_ok
    test("1.0", "0x1.0#1", 3, 0, Down, "10", 1, Equal);
    // covers: g_negO
    test("-1.0", "-0x1.0#1", 2, 0, Down, "-1", 1, Equal);
    // covers: g_negF
    test("-1.0", "-0x1.0#1", 2, 0, Floor, "-1", 1, Equal);
    // covers: g_negC
    test("-1.0", "-0x1.0#1", 2, 0, Ceiling, "-1", 1, Equal);
    // covers: g_expz meq_g meq_nxle meq_nle
    test("2.0", "0x2.0#1", 3, 1, Down, "2", 1, Equal);
    // covers: mgt_x1lo
    test("1.0", "0x1.0#1", 9, 15, Down, "100000000000000", 1, Equal);
    // covers: ax_carry ax_c_j0
    test("0.50", "0x0.8#1", 3, 1, Up, "2", 0, Greater);
    // covers: p2_rle
    test("0.50", "0x0.8#1", 4, 0, Down, "2", 0, Equal);
    // covers: mlt_g mlt_2ngt mlt_n1 mlt_remchk mlt_nonorm exact_n ax_inexact
    test("4.0", "0x4.0#1", 3, 1, Down, "1", 2, Less);
    // covers: ax_noshr
    test(
        "1.0",
        "0x1.0#1",
        9,
        20,
        Down,
        "10000000000000000000",
        1,
        Equal,
    );
    // covers: p2_nonb
    test(
        "0.50",
        "0x0.8#1",
        16,
        16,
        Down,
        "8000000000000000",
        0,
        Equal,
    );
    // covers: p2_carry p2_rpow
    test("1.5", "0x1.8#2", 2, 1, Up, "1", 2, Greater);
    // covers: mgt_norm ax_dr ax_dir ax_trunc
    test("1.5", "0x1.8#2", 3, 0, Down, "111", 1, Less);
    // covers: ax_away ax_awcarry
    test("1.5", "0x1.8#2", 3, 0, Up, "112", 1, Greater);
    // covers: ax_near ax_down
    test("1.5", "0x1.8#2", 3, 0, Nearest, "111", 1, Less);
    // covers: ax_up
    test("1.5", "0x1.8#2", 3, 2, Nearest, "12", 1, Greater);
    // covers: p2_rnpow p2_topz
    test("1.5", "0x1.8#2", 4, 1, Up, "2", 1, Greater);
    // covers: ax_tie ax_evenC
    test("1.5", "0x1.8#2", 6, 1, Nearest, "2", 1, Greater);
    // covers: ax_awnoc
    test("0.75", "0x0.c#2", 3, 1, Up, "1", 1, Greater);
    // covers: mlt_renorm
    test("6.0", "0x6.0#2", 3, 1, Down, "2", 2, Equal);
    // covers: ax_t62
    test(
        "-269104312292334.3027",
        "-0xf4bfbaf113ee.4d8#57",
        43,
        2,
        Down,
        "-N1",
        9,
        Greater,
    );
    // covers: mgt_nxgt mgt_lo (Down, not Exact: this value is inexact in base 7, so Exact would
    // panic. Down takes the same truncating code path, so the branch coverage is unchanged.)
    test(
        "0.000199046277632504184666664672269768242929310652018203552191617720205649",
        "0x0.000d0b7140b8f3aea60aad60c1dc3b2ee0d83e2eba33dcfb6f874df52d78#225",
        7,
        6,
        Down,
        "322631",
        -4,
        Less,
    );
    // covers: mgt_errp
    test(
        "1.1595752615776271305e-33",
        "0x6.055703bef650178E-28#63",
        28,
        2,
        Down,
        "26",
        -22,
        Less,
    );
    // covers: mlt_nn
    test(
        "1.04226364758062811487679885e63",
        "0x2.889a2dba3978ccd56c826E+52#85",
        26,
        11,
        Up,
        "5j89gbd3609",
        45,
        Greater,
    );
    // covers: mlt_2nle mlt_lo
    test(
        "13863336.632654341786855779405528442674244",
        "0xd389a8.a1f5a28ba59ea1aca395f84bcc2#131",
        29,
        2,
        Nearest,
        "jh",
        5,
        Less,
    );
    // covers: ax_tiefail ret_nfail nfail_mgt
    test(
        "1858.9712372",
        "0x742.f8a300#32",
        34,
        6,
        Nearest,
        "1kmx0q",
        3,
        Greater,
    );
    // covers: mlt_lo_no
    test(
        "4.518487544134823141772917e44",
        "0x1.442f82545e664fc1b2dcE+37#79",
        23,
        7,
        Ceiling,
        "c07e6ek",
        33,
        Greater,
    );
    // covers: ax_keep
    test(
        "4.205885e17",
        "0x5.d63bE+14#19",
        39,
        3,
        Nearest,
        "1CQ",
        12,
        Less,
    );
    // covers: meq_nxgt
    test(
        "3387429.4861150337642728810837723",
        "0x33b025.7c7208ec1c8a76dc0e59#102",
        25,
        5,
        Floor,
        "8gjm4",
        5,
        Less,
    );
    // covers: meq_ngt
    test(
        "-1.4772e19",
        "-0xc.d0E+15#11",
        40,
        12,
        Exact,
        "-Z8TcXJ48HaW0",
        12,
        Equal,
    );
    // covers: ax_cprop
    test(
        "-4.9275860538295e71",
        "-0x4.7656eb83f9E+59#43",
        56,
        13,
        Up,
        "-12CZ7Gm3q4fS0",
        42,
        Less,
    );
    // covers: nfail_mle
    test(
        "1.666502136661e30",
        "0x1.508c2421bE+25#37",
        18,
        12,
        Nearest,
        "1478827641f3",
        25,
        Less,
    );
    // covers: ax_evenF
    test(
        "-1.65839",
        "-0x1.a88c#16",
        38,
        14,
        Nearest,
        "-1P0QaVM1O1TQ4S",
        1,
        Greater,
    );
    // covers: ax_fail ret_fail (Down, not Exact: this value is inexact in base 46, so Exact would
    // panic. Down takes the same truncating code path, so the branch coverage is unchanged.)
    test(
        "7.19867e-15",
        "0x2.06b8E-12#16",
        46,
        8,
        Down,
        "6TH73LXN",
        -8,
        Less,
    );

    // Invalid bases (`-36..=-2` and `2..=62` are the only valid ones) give None: the `g_badbase`
    // branch, which the generators never reach since they only emit valid bases.
    let x = parse_hex_string("0x1.0#1");
    assert!(get_str(&x, 100, 0, Nearest).is_none());
    assert!(get_str(&x, 63, 0, Nearest).is_none());
    assert!(get_str(&x, 1, 0, Nearest).is_none());
    assert!(get_str(&x, 0, 0, Nearest).is_none());
    assert!(get_str(&x, -1, 0, Nearest).is_none());
    assert!(get_str(&x, -37, 0, Nearest).is_none());
    verify_get_str(&x, 100, 0, Nearest);
    verify_get_str(&x, -37, 0, Nearest);
}

#[test]
fn test_get_str_exact_panics() {
    // `Exact` panics unless the value is exactly representable in the requested digits. 1/3 has no
    // finite base-10 expansion.
    assert_panic!(get_str(&parse_hex_string("0x0.5555558#25"), 10, 4, Exact));
    // 0.5 has no finite expansion in the odd base 3 (it is 0.1111...).
    assert_panic!(get_str(&parse_hex_string("0x0.8#1"), 3, 2, Exact));
    // A value that is dyadic but needs more than the requested 6 base-7 digits.
    assert_panic!(get_str(
        &parse_hex_string("0x0.000d0b7140b8f3aea60aad60c1dc3b2ee0d83e2eba33dcfb6f874df52d78#225"),
        7,
        6,
        Exact
    ));
    // m == 0 picks the round-trip digit count, which is generally too few for an exact base-10
    // representation, so even round-trip mode panics under `Exact`.
    assert_panic!(get_str(&parse_hex_string("0x1.921fb6#24"), 10, 0, Exact));
}

#[test]
fn test_get_str_digit_count() {
    fn test(base: u64, prec: u64, out: usize) {
        assert_eq!(get_str_digit_count(base, prec), out);
    }
    // 17 significant decimal digits distinguish every double-precision (53-bit) value, and 9
    // suffice for single precision (24 bits)
    test(10, 53, 17);
    test(10, 24, 9);
    test(10, 1, 2);
    // power-of-2 bases: 1 + ceil((prec - 1) / k)
    test(2, 1, 1);
    test(2, 10, 10);
    test(2, 53, 53);
    test(16, 1, 1);
    test(16, 53, 14);
    test(32, 100, 21);
    // an odd base, and the largest base
    test(3, 2, 3);
    test(62, 100, 18);
}

#[test]
fn get_str_digit_count_fail() {
    assert_panic!(get_str_digit_count(0, 10));
    assert_panic!(get_str_digit_count(1, 10));
    assert_panic!(get_str_digit_count(63, 10));
    assert_panic!(get_str_digit_count(10, 0));
}

#[test]
fn get_str_digit_count_properties() {
    unsigned_pair_gen_var_51().test_properties(|(base, prec)| {
        let m = u64::exact_from(get_str_digit_count(base, prec));
        if let Some(k) = base.checked_log_base_2() {
            // for a power-of-2 base, the count is 1 + ceil((prec - 1) / k)
            assert_eq!(m, 1 + (prec - 1).div_round(k, Ceiling).0);
        } else if prec <= 1_000_000 {
            // Otherwise, m - 1 = ceil(prec * log(2) / log(base)), the smallest e with base ^ e >= 2
            // ^ prec; m >= 2, since the ceiling is positive.
            let b = Natural::from(base);
            let p2 = Natural::power_of_2(prec);
            assert!((&b).pow(m - 1) >= p2);
            assert!((&b).pow(m - 2) < p2);
        }
    });
    // The defining round-trip property: printing any Float to get_str_digit_count(base, prec)
    // digits with rounding to nearest, then rounding the digits' value back to prec bits, again to
    // nearest, recovers the Float exactly. The check reconstructs the digits' value as an exact
    // Rational, so it is gated to moderate exponents.
    float_signed_unsigned_rounding_mode_quadruple_gen_var_9().test_properties(|(x, b0, _, _)| {
        let Some(prec) = x.get_prec() else {
            return;
        };
        if x.get_exponent().unwrap().unsigned_abs() > 10_000 {
            return;
        }
        let b = b0.unsigned_abs();
        let m = get_str_digit_count(b, prec);
        let (digits, exp, _) = get_str(&x, i64::exact_from(b), m, Nearest).unwrap();
        let neg = digits[0] == b'-';
        let digit_bytes = &digits[usize::from(neg)..];
        let mut d = Natural::ZERO;
        for &c in digit_bytes {
            d = d * Natural::from(b) + Natural::from(digit_value(c, b > 36));
        }
        let mut v = Rational::from(d) * Rational::from(b).pow(exp - i64::exact_from(m));
        if neg {
            v = -v;
        }
        let x_back = Float::from_rational_prec_round(v, prec, Nearest).0;
        assert_eq!(
            ComparableFloatRef(&x_back),
            ComparableFloatRef(&x),
            "{x} base {b} m {m}"
        );
    });
}

#[test]
fn get_str_properties() {
    float_signed_unsigned_rounding_mode_quadruple_gen_var_9().test_properties(|(x, b0, m, rnd)| {
        verify_get_str(&x, b0, m, rnd);
    });
    // The same property over rug-acceptable inputs only (base 2..=36, non-Exact), so every case
    // also exercises the rug cross-check inside `verify_get_str`.
    float_signed_unsigned_rounding_mode_quadruple_gen_var_10().test_properties(
        |(x, b0, m, rnd)| {
            verify_get_str(&x, b0, m, rnd);
        },
    );
    // The same two again over extreme `Float`s. Scaling by a power of the base as large as the
    // exponent is the part of `get_str` that works hardest there, and rug's exponent range is
    // `Float`'s, so the cross-check stays a faithful oracle.
    float_signed_unsigned_rounding_mode_quadruple_gen_var_15().test_properties(
        |(x, b0, m, rnd)| {
            verify_get_str(&x, b0, m, rnd);
        },
    );
    float_signed_unsigned_rounding_mode_quadruple_gen_var_16().test_properties(
        |(x, b0, m, rnd)| {
            verify_get_str(&x, b0, m, rnd);
        },
    );
}
