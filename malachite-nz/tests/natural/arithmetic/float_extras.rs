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
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float_extras::limbs_float_exp;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::unsigned_vec_unsigned_unsigned_triple_gen_var_17;

// Runs `limbs_float_exp` on a buffer of length `out_len` and verifies its contract against the exact
// value of `b ^ e`, returning the `(exp, err)` it produced.
//
// Checks: the result is independent of the buffer's initial contents; the result is normalized (the
// most significant bit of the top limb is set); and the enclosure
// `a * 2 ^ exp <= b ^ e <= (a + 2 ^ err) * 2 ^ exp` holds, tightening to equality when `err == -1`.
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
    // Verify `a * 2 ^ exp <= b ^ e <= (a + 2 ^ err) * 2 ^ exp` exactly, scaling by a power of two to
    // stay in the integers regardless of the sign of `exp`.
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
