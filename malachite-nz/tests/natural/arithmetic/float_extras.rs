// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float_extras::{limbs_float_exp, limbs_get_str_aux};
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    large_type_gen_var_28, unsigned_vec_unsigned_unsigned_triple_gen_var_17,
};

// Runs `limbs_float_exp` on a buffer of length `out_len` and verifies its contract against the
// exact value of `b ^ e`, returning the `(exp, err)` it produced.
//
// Checks: the result is independent of the buffer's initial contents; the result is normalized (the
// most significant bit of the top limb is set); and the enclosure `a * 2 ^ exp <= b ^ e <= (a + 2 ^
// err) * 2 ^ exp` holds, tightening to equality when `err == -1`.
fn verify_limbs_float_exp(out_len: usize, b: u64, e: u64) -> (i64, i32) {
    let mut a = vec![0; out_len];
    let (exp, err) = limbs_float_exp(&mut a, b, i64::exact_from(e));

    // The result must not depend on the buffer's initial contents (the function overwrites it).
    let mut a_alt = vec![Limb::MAX; out_len];
    let (exp_alt, err_alt) = limbs_float_exp(&mut a_alt, b, i64::exact_from(e));
    assert_eq!(exp, exp_alt);
    assert_eq!(err, err_alt);
    assert_eq!(a, a_alt);

    assert!(err >= -2);
    if err == -2 {
        return (exp, err); // overflow: no value is guaranteed
    }
    // Normalized: the top limb's most significant bit is set.
    assert!(a[out_len - 1].get_highest_bit());

    let big_a = Natural::from_limbs_asc(&a);
    let target = Natural::from(b).pow(e);
    let two_err = if err >= 0 {
        Natural::ONE << u64::exact_from(err)
    } else {
        Natural::ZERO
    };
    // Verify `a * 2 ^ exp <= b ^ e <= (a + 2 ^ err) * 2 ^ exp` exactly, scaling by a power of two
    // to stay in the integers regardless of the sign of `exp`.
    if exp >= 0 {
        let s = u64::exact_from(exp);
        let lo = &big_a << s;
        if err == -1 {
            assert_eq!(lo, target);
        } else {
            assert!(lo <= target);
            assert!(target <= (&big_a + &two_err) << s);
        }
    } else {
        let s = u64::exact_from(-exp);
        let scaled_target = &target << s;
        if err == -1 {
            assert_eq!(scaled_target, big_a);
        } else {
            assert!(big_a <= scaled_target);
            assert!(scaled_target <= &big_a + &two_err);
        }
    }
    (exp, err)
}

#[test]
fn test_limbs_float_exp() {
    #[cfg(not(feature = "32_bit_limbs"))]
    fn test(n: usize, b: u64, e: u64, out: &[Limb], exp: i64, err: i32) {
        let mut a = vec![0; n];
        let (actual_exp, actual_err) = limbs_float_exp(&mut a, b, i64::exact_from(e));
        assert_eq!(a.as_slice(), out);
        assert_eq!(actual_exp, exp);
        assert_eq!(actual_err, err);
        verify_limbs_float_exp(n, b, e);
    }

    #[cfg(not(feature = "32_bit_limbs"))]
    {
        // - exact first time (e == 1, so the loop is never entered)
        test(1, 2, 1, &[9223372036854775808], -62, -1);
        // - f_ok first time
        // - add_ok first time
        // - sq_shift first time
        // - sq_a2_no first time
        // - sq_err_no first time
        // - bit0 first time
        test(1, 2, 2, &[9223372036854775808], -61, -1);
        // - sq_norm first time
        test(1, 3, 2, &[10376293541461622784], -60, -1);
        // - bit1 first time
        // - mul_shift first time
        // - mul_err_no first time
        test(1, 2, 3, &[9223372036854775808], -60, -1);
        // - mul_norm first time
        // - mul_ab_no first time
        test(1, 3, 7, &[9849372385059274752], -52, -1);
        // - sq_err first time
        // - inexact first time
        test(1, 17, 16, &[12165297968916717120], 2, 3);
        // - mul_err first time
        test(1, 7, 23, &[13684373670040458171], 1, 3);
        // - mul_ab first time
        test(1, 7, 25, &[10477098591124725787], 7, 4);
        // - sq_a2 first time
        test(1, 17, 32, &[16045593095580712413], 67, 4);
    }
    // The exponent-overflow guards fire only for an astronomically large `e`, far beyond any
    // `mpfr_get_str` call (where `e` is bounded by the Float exponent range). The first guard is
    // reachable directly; `verify_limbs_float_exp` is bypassed because its `b ^ e` oracle would be
    // astronomical.
    // - f_ovf first time
    assert_eq!(limbs_float_exp(&mut [0; 1], 62, 1i64 << 61).1, -2);
    // - add_ovf is shadowed by f_ovf for every findable input (see the fail_on_untested_path marker
    //   in the source); no test reaches it.
}

#[test]
fn limbs_float_exp_properties() {
    unsigned_vec_unsigned_unsigned_triple_gen_var_17().test_properties(|(out, b, e)| {
        verify_limbs_float_exp(out.len(), b, e);
    });
}

// Inverts the `num_to_text` tables: maps an output character back to its base-`b0` digit value.
// `large_table` selects the table `limbs_get_str_aux` used (uppercase letters are 10..=35 there and
// lowercase 36..=61, versus lowercase 10..=35 for the small table).
fn digit_value(ch: u8, large_table: bool) -> u8 {
    match ch {
        b'0'..=b'9' => ch - b'0',
        b'A'..=b'Z' => ch - b'A' + 10,
        b'a'..=b'z' if large_table => ch - b'a' + 36,
        b'a'..=b'z' => ch - b'a' + 10,
        _ => panic!("invalid digit character {ch}"),
    }
}

// Runs `limbs_get_str_aux` and validates its output, returning the `(dir, exp)` it produced.
//
// Checks: the result is independent of the output buffer's initial contents and transforms `r`
// identically; an exact input (`e < 0`) is always roundable. On success the m output digits, read
// as an integer `D`, reconstruct `V = D * b0 ^ exp`, which must be the
// correctly-rounded-to-m-digits value of the exact center `C = r * 2 ^ -neg_f`: `V` brackets `C` to
// within one ulp (`b0 ^ exp`, scaled by `2 ^ neg_f`) in the direction `dir`, and `dir` is
// consistent with the rounding mode.
fn verify_limbs_get_str_aux(
    r: &[Limb],
    neg_f: u64,
    e: i64,
    b0: i64,
    m: usize,
    rnd: RoundingMode,
) -> (i8, i64) {
    let mut r0 = r.to_vec();
    let mut s0 = vec![0; m];
    let (dir, exp) = limbs_get_str_aux(&mut s0, &mut r0, neg_f, e, b0, m, rnd);

    // The result must not depend on the output buffer's initial contents, and `r` must be
    // transformed identically.
    let mut r1 = r.to_vec();
    let mut s1 = vec![u8::MAX; m];
    let (dir_alt, exp_alt) = limbs_get_str_aux(&mut s1, &mut r1, neg_f, e, b0, m, rnd);
    assert_eq!(dir, dir_alt);
    assert_eq!(exp, exp_alt);
    assert_eq!(r0, r1);

    // An exact input never hits the "error too large" failure (MPFR_ROUND_FAILED = 3), since the
    // roundability test is skipped. The round-to-nearest tie failure (-3) can still occur, when
    // rounding the non-integer Y to an integer is inexact.
    if e < 0 {
        assert_ne!(dir, 3);
    }
    if dir.unsigned_abs() == 3 {
        return (dir, exp); // rounding not possible; the digits are not meaningful
    }
    // On success the digit string is fully written, hence buffer-independent.
    assert_eq!(s0, s1);

    assert!(exp >= 0);
    let b_u64 = u64::exact_from(b0);
    let large_table = !(2..=36).contains(&b0);
    // Read the m base-b0 digits (most significant first) as an integer D.
    let mut d = Natural::ZERO;
    for &ch in &s0 {
        d = d * Natural::from(b_u64) + Natural::from(digit_value(ch, large_table));
    }
    // D has exactly m significant digits: b0 ^ (m - 1) <= D < b0 ^ m.
    assert!(d >= Natural::from(b_u64).pow(u64::exact_from(m - 1)));
    assert!(d < Natural::from(b_u64).pow(u64::exact_from(m)));

    // V = D * b0 ^ exp is the value the output represents, and ulp = b0 ^ exp is the weight of its
    // last digit. Scaling everything by 2 ^ |f| turns the exact center C = r * 2 ^ f into the
    // integer r, keeping the comparisons exact. Check that V is the correctly-rounded-to-m-digits
    // value of C.
    //
    // Direction is checked by rounding *mode*, not by `dir`'s sign: when the internal integer
    // rounding crosses a power of b0 (e.g. Nearest sends 48.6 to 49 = "100" base 7), the function
    // truncates the trailing zero and reports dir = -1 even though V > C. The mode still pins the
    // side (the significand is positive), and the digits are correct.
    let ulp = Natural::from(b_u64).pow(u64::exact_from(exp));
    let den = neg_f;
    let r_nat = Natural::from_limbs_asc(r);
    let u_scaled = &ulp << den;
    let v = (d * &ulp) << den;
    match rnd {
        Floor | Down | Exact => {
            // truncation toward zero: V <= C < V + ulp
            assert!(v <= r_nat);
            assert!(r_nat < &v + &u_scaled);
            assert!(dir <= 0);
        }
        Ceiling | Up => {
            // rounding away from zero: V - ulp < C <= V
            assert!(r_nat <= v);
            assert!(v < &r_nat + &u_scaled);
            assert!(dir >= 0);
        }
        Nearest => {
            // the nearest m-digit value: 2 * |C - V| <= ulp
            let dist2 = if r_nat >= v { &r_nat - &v } else { &v - &r_nat } << 1;
            assert!(dist2 <= u_scaled);
        }
    }
    // dir reports exactness iff the value is represented exactly.
    assert_eq!(dir == 0, v == r_nat);
    (dir, exp)
}

#[test]
fn test_limbs_get_str_aux() {
    fn test(
        r: &[Limb],
        neg_f: u64,
        e: i64,
        b0: i64,
        m: usize,
        rnd: RoundingMode,
        out: &str,
        dir: i8,
        exp: i64,
    ) {
        let mut r_mut = r.to_vec();
        let mut s = vec![0; m];
        let (actual_dir, actual_exp) = limbs_get_str_aux(&mut s, &mut r_mut, neg_f, e, b0, m, rnd);
        // The digit string is only meaningful when rounding succeeded.
        if actual_dir.unsigned_abs() != 3 {
            assert_eq!(std::str::from_utf8(&s).unwrap(), out);
        }
        assert_eq!(actual_dir, dir);
        assert_eq!(actual_exp, exp);
        verify_limbs_get_str_aux(r, neg_f, e, b0, m, rnd);
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        const HB: Limb = 1 << 63; // 9223372036854775808
        // exact integer (neg_f = 0), exact m-digit conversion
        test(
            &[HB],
            0,
            -1,
            3,
            40,
            Down,
            "2021110011022210012102010021220101220222",
            0,
            0,
        );
        // exact value with one superfluous digit: truncation and round-away differ in the last
        // digit
        test(
            &[HB],
            0,
            -1,
            5,
            27,
            Down,
            "110433240130442243431031121",
            -1,
            1,
        );
        test(&[HB], 0, -1, 5, 27, Up, "110433240130442243431031122", 1, 1);
        // error too large to determine the nearest integer: MPFR_ROUND_FAILED
        test(&[HB], 0, 1, 3, 39, Down, "", 3, 0);
        // rounding an all-ones mantissa up carries into a fresh power of two, with neg_f a multiple
        // of the limb width so the bit of weight 0 sits at the bottom of a limb (j0 == 0)
        test(
            &[Limb::MAX; 6],
            192,
            -1,
            38,
            37,
            Up,
            "8G552YH5EZUNX7RIBJ7W32K0RYL3GVW0J07PU",
            1,
            0,
        );
    }
}

#[test]
fn limbs_get_str_aux_properties() {
    large_type_gen_var_28().test_properties(|(r, neg_f, e, b0, m, rnd)| {
        verify_limbs_get_str_aux(&r, neg_f, e, b0, m, rnd);
    });
}
