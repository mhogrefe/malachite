// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float::get_str::limbs_get_str_aux;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::large_type_gen_var_28;

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
