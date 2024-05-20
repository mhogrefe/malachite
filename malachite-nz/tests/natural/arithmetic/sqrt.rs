// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::arithmetic::traits::{
    CeilingRoot, CeilingSqrt, CeilingSqrtAssign, CheckedRoot, CheckedSqrt, FloorRoot, FloorSqrt,
    FloorSqrtAssign, RootRem, ShrRound, SqrtAssignRem, SqrtRem, Square,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::{ExactFrom, JoinHalves};
use malachite_base::num::logic::traits::BitAccess;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::logic::traits::LeadingZeros;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    large_type_gen_var_2, unsigned_gen, unsigned_pair_gen_var_31, unsigned_vec_gen_var_1,
    unsigned_vec_pair_gen_var_4, unsigned_vec_pair_gen_var_5, unsigned_vec_triple_gen_var_28,
};
use malachite_nz::natural::arithmetic::sqrt::{
    limbs_ceiling_sqrt, limbs_checked_sqrt, limbs_floor_sqrt, limbs_sqrt_helper, limbs_sqrt_rem,
    limbs_sqrt_rem_helper, limbs_sqrt_rem_helper_scratch_len, limbs_sqrt_rem_to_out,
    limbs_sqrt_to_out, sqrt_rem_2_newton,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::{DoubleLimb, Limb};
use malachite_nz::test_util::generators::natural_gen;
use malachite_nz::test_util::natural::arithmetic::sqrt::{
    ceiling_sqrt_binary, checked_sqrt_binary, floor_sqrt_binary, sqrt_rem_binary,
};
use num::BigUint;
use std::panic::catch_unwind;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_sqrt_rem_2_newton() {
    fn test(n_hi: Limb, n_lo: Limb, sqrt: Limb, r_hi: bool, r_lo: Limb) {
        assert_eq!(sqrt_rem_2_newton(n_hi, n_lo), (sqrt, r_hi, r_lo));
        assert_eq!(
            DoubleLimb::from(sqrt)
                .square()
                .checked_add(DoubleLimb::join_halves(Limb::from(r_hi), r_lo))
                .unwrap(),
            DoubleLimb::join_halves(n_hi, n_lo)
        );
    }
    // - no adjustment needed
    test(2000000000, 123, 2930859019, false, 2746357762);
    test(u32::MAX, 123, 4294967295, true, 122);
    // - adjustment needed
    test(1073741825, 0, 2147483648, true, 0);
}

#[test]
fn sqrt_rem_2_newton_fail() {
    assert_panic!(sqrt_rem_2_newton(1, Limb::MAX));
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sqrt_rem_helper() {
    fn test(old_xs: &[Limb], n: usize, out_out: &[Limb], xs_out: &[Limb], r_hi: bool) {
        assert!(old_xs.len() >= n << 1);
        let mut out = vec![0; n];
        let mut xs = old_xs.to_vec();
        let mut scratch = vec![0; limbs_sqrt_rem_helper_scratch_len(n)];
        assert_eq!(
            limbs_sqrt_rem_helper(&mut out, &mut xs, 0, &mut scratch),
            r_hi
        );
        assert_eq!(out, out_out);
        assert_eq!(xs, xs_out);

        let x = Natural::from_limbs_asc(&old_xs[..n << 1]);
        let sqrt = Natural::from_limbs_asc(&out[..n]);
        let mut rem = Natural::from_limbs_asc(&xs[..n]);
        if r_hi {
            rem.set_bit(u64::exact_from(n) << Limb::LOG_WIDTH);
        }
        assert_eq!((&x).sqrt_rem(), (sqrt.clone(), rem.clone()));
        assert_eq!((&sqrt).square() + rem, x);
        assert!((&sqrt).square() <= x);
        assert!((sqrt + Natural::ONE).square() > x);
    }
    // - h2 == 1
    // - !q
    // - r_hi
    // - h1 == h2
    // - r_hi >= 0
    test(
        &[123, 456, 789, 2000000000],
        2,
        &[2012297342, 2930859019],
        &[1917624951, 2990305571, 2377342468, 942810576],
        false,
    );
    // - !r_hi
    test(&[0, 0, 0, 1073741824], 2, &[0, 2147483648], &[0; 4], false);
    // - q
    // - r_hi < 0
    test(
        &[0, 0, 0, 1073741825],
        2,
        &[4294967295, 2147483648],
        &[4294967295, 1, 0, 0],
        false,
    );
    // - h2 > 1
    // - h1 != h2
    test(
        &[0, 0, 0, 0, 0, 1073741824],
        3,
        &[0, 0, 2147483648],
        &[0; 6],
        false,
    );
}

#[test]
fn limbs_sqrt_rem_helper_fail() {
    // - out too short
    assert_panic!({
        let out = &mut [];
        let xs = &mut [1, 2, 3];
        let mut scratch = vec![0; limbs_sqrt_rem_helper_scratch_len(out.len())];
        limbs_sqrt_rem_helper(out, xs, 0, &mut scratch)
    });

    // - xs too short
    assert_panic!({
        let out = &mut [1, 2, 3];
        let xs = &mut [1, 2, 3, 4, Limb::MAX];
        let mut scratch = vec![0; limbs_sqrt_rem_helper_scratch_len(out.len())];
        limbs_sqrt_rem_helper(out, xs, 0, &mut scratch)
    });

    // - (2 * n - 1)th element of xs too small
    assert_panic!({
        let out = &mut [1, 2, 3];
        let xs = &mut [1, 2, 3, 4, 5, 6];
        let mut scratch = vec![0; limbs_sqrt_rem_helper_scratch_len(out.len())];
        limbs_sqrt_rem_helper(out, xs, 0, &mut scratch)
    });
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sqrt_helper() {
    fn test(xs: &[Limb], out_out: &[Limb], has_remainder: bool) {
        let n = xs.len().shr_round(1, Ceiling).0;
        let odd = xs.len().odd();
        let shift = LeadingZeros::leading_zeros(*xs.last().unwrap()) >> 1;
        let mut out = vec![0; n];
        assert_eq!(limbs_sqrt_helper(&mut out, xs, shift, odd), has_remainder);
        assert_eq!(out, out_out);

        let x = Natural::from_limbs_asc(xs);
        let sqrt = Natural::from_limbs_asc(&out);
        let (sqrt_alt, rem) = (&x).sqrt_rem();
        assert_eq!(sqrt, sqrt_alt);
        assert_eq!(has_remainder, rem != 0);
        assert!((&sqrt).square() <= x);
        assert!((sqrt + Natural::ONE).square() > x);
    }
    // - shift != 0 first time
    // - !q
    // - qs_last != 0 <= 1
    // - (*qs_head >> 3) | qs_tail[0].mod_power_of_2(Limb::WIDTH - s) != 0
    // - odd == 1 || shift != 0
    test(
        &[1, 2, 3, 4, 5, 6, 7, 8, 9],
        &[406519538, 874900746, 1431655766, 1, 3],
        true,
    );
    // - q
    test(
        &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        &[1984678003, 1990369149, 2938805076, 123516853, 207243],
        true,
    );
    // - (*qs_head >> 3) | qs_tail[0].mod_power_of_2(Limb::WIDTH - s) == 0
    // - cmp != Less first time
    // - slice_test_zero(&scratch_hi[h1 + 1..h2 + 1])
    // - cmp == Equal
    // - shift != 0 second time
    // - cmp != Less second time
    test(&[0, 0, 0, 0, 0, 0, 0, 0, 1], &[0, 0, 0, 0, 1], false);
    // - !slice_test_zero(&scratch_hi[h1 + 1..h2 + 1])
    test(
        &[0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
        &[4294959104, 4294967295, 32767, 0, 65536],
        true,
    );
    // - cmp != Equal
    // - cmp == Less second time
    test(
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
        &[4095, 0, 4294959104, 4294967295, 32767, 0, 65536],
        true,
    );
    // - shift == 0 first time
    // - odd == 0 && shift == 0
    test(
        &[
            2193991530, 2899278504, 3717617329, 1249076698, 879590153, 4210532297, 3303769392,
            1147691304, 3624392894, 1881877405, 1728780505, 931615955, 1096404509, 1326003135,
            370549396, 1987183422, 851586836, 2796582944, 2985872407, 2598301166, 356639089,
            2071312376, 1106184443, 3682669872, 1019062633, 3091583269, 502719907, 1700144765,
            1387866140, 1704631346, 2770368441, 1350352297,
        ],
        &[
            1761794912, 1811046967, 2629492573, 855368414, 1733978088, 3870361288, 2771154681,
            1755982555, 443586875, 4077786862, 37569597, 1268220478, 2132005861, 2264846737,
            2419409154, 2408260566,
        ],
        true,
    );
    // - qs_last != 0 > 1
    test(
        &[
            4294967295, 4194303, 0, 0, 3758096384, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        ],
        &[
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 65535,
        ],
        true,
    );
    // - shift == 0 second time
    test(
        &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 4294966784, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295,
        ],
        &[
            4278190079, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 65535,
        ],
        true,
    );
    // - cmp == Less first time
    test(
        &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4294967232, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            2147483679, 16777215, 0, 0, 0, 0, 4294967040, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295,
        ],
        &[
            4293920767, 1073741791, 0, 0, 0, 0, 3221217296, 8388607, 0, 0, 0, 0, 4294967168,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        ],
        true,
    );
}

#[test]
fn limbs_sqrt_helper_fail() {
    // - out too short
    assert_panic!({
        let out = &mut [1, 2, 3, 4];
        let xs = &mut [Limb::MAX; 8];
        limbs_sqrt_helper(out, xs, 0, false);
    });
    // - last element of xs is 0
    assert_panic!({
        let out = &mut [1, 2, 3, 4, 5];
        let xs = &mut [10, 10, 10, 10, 10, 10, 10, 10, 10, 0];
        limbs_sqrt_helper(out, xs, 15, false);
    });
    // - shift too high
    assert_panic!({
        let out = &mut [1, 2, 3, 4, 5];
        let xs = &mut [Limb::MAX; 10];
        limbs_sqrt_helper(out, xs, 16, false);
    });
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sqrt_to_out() {
    fn test(xs: &[Limb], out_out: &[Limb]) {
        let xs_len = xs.len();
        let mut out = vec![0; xs_len.shr_round(1, Ceiling).0];
        limbs_sqrt_to_out(&mut out, xs);
        assert_eq!(out, out_out);
        let x = Natural::from_limbs_asc(xs);
        let sqrt = Natural::from_owned_limbs_asc(out);
        assert_eq!((&x).floor_sqrt(), sqrt);
        assert!((&sqrt).square() <= x);
        assert!((sqrt + Natural::ONE).square() > x);
    }
    // - shift != 0
    // - xs_len == 1
    // - xs_len == 1 && shift != 0
    test(&[1], &[1]);
    // - shift == 0
    // - xs_len == 1 && shift == 0
    test(&[4000000000], &[63245]);
    // - xs_len == 2
    // - xs_len == 2 && shift != 0
    test(&[1, 2], &[92681]);
    // - xs_len == 2 && shift == 0
    test(&[1, 4000000000], &[4144860574]);
    // - xs_len > 2
    // - 2 < xs_len <= 8
    // - xs_len.odd() || shift != 0
    // - xs_len > 2 && shift == 0
    // - tn > 1
    test(&[1, 2, 3], &[3144134278, 1]);
    // - xs_len > 2 && shift != 0
    test(&[1, 2, 4000000000], &[2375990371, 63245]);
    // - xs_len > 8
    test(
        &[
            2572912965, 1596092594, 2193991530, 2899278504, 3717617329, 1249076698, 879590153,
            4210532297, 3303769392, 1147691304, 3624392894,
        ],
        &[3491190173, 18317336, 2518787533, 3220458996, 3998374718, 60202],
    );
    // - xs_len.even() && shift == 0
    test(
        &[345016311, 2711392466, 1490697280, 1246394087],
        &[2306404477, 2313703058],
    );
}

#[test]
fn limbs_sqrt_to_out_fail() {
    // - xs empty
    assert_panic!({
        let out = &mut [1, 2, 3];
        let xs = &mut [];
        limbs_sqrt_to_out(out, xs);
    });
    // - out too short
    assert_panic!({
        let out = &mut [1, 2, 3];
        let xs = &mut [Limb::MAX; 8];
        limbs_sqrt_to_out(out, xs);
    });
    // - last element of xs is 0
    assert_panic!({
        let out = &mut [1, 2, 3];
        let xs = &mut [1, 2, 3, 4, 5, 0];
        limbs_sqrt_to_out(out, xs);
    });
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sqrt_rem_to_out() {
    fn test(xs: &[Limb], out_out_sqrt: &[Limb], out_out_rem: &[Limb]) {
        let xs_len = xs.len();
        let mut out_sqrt = vec![0; xs_len.shr_round(1, Ceiling).0];
        let mut out_rem = vec![0; xs_len];
        let rem_len = limbs_sqrt_rem_to_out(&mut out_sqrt, &mut out_rem, xs);
        assert_eq!(out_sqrt, out_out_sqrt);
        assert_eq!(&out_rem[..rem_len], out_out_rem);
        let x = Natural::from_limbs_asc(xs);
        let sqrt = Natural::from_owned_limbs_asc(out_sqrt);
        let rem = Natural::from_limbs_asc(&out_rem[..rem_len]);
        let (sqrt_alt, rem_alt) = (&x).sqrt_rem();
        assert_eq!(sqrt_alt, sqrt);
        assert_eq!(rem_alt, rem);
        assert_eq!((&sqrt).square() + &rem, x);
        assert!((&sqrt).square() <= x);
        assert!((sqrt + Natural::ONE).square() > x);
    }
    // - shift != 0
    // - xs_len == 1
    // - xs_len == 1 && shift != 0
    test(&[1], &[1], &[]);
    // - shift == 0
    // - xs_len == 1 && shift == 0
    test(&[4000000000], &[63245], &[69975]);
    // - xs_len == 2
    // - xs_len == 2 && shift != 0
    test(&[1, 2], &[92681], &[166832]);
    // - xs_len == 2 && shift == 0
    test(&[1, 4000000000], &[4144860574], &[1805423229, 1]);
    // - xs_len > 2
    // - xs_len.odd() || shift != 0
    // - xs_len > 2 && shift != 0 first time
    // - shift >= Limb::WIDTH
    // - xs_len > 2 && shift != 0 second time
    test(&[1, 2, 3], &[3144134278, 1], &[1429311965, 0]);
    // - xs_len > 2 && shift == 0 first time
    // - xs_len > 2 && shift == 0 second time
    test(
        &[1, 2, 4000000000],
        &[2375990371, 63245],
        &[3710546360, 103937],
    );
    // - xs_len.even() && shift == 0
    test(
        &[2977742827, 3919053323, 1548431690, 1948915452],
        &[733991603, 2893186501],
        &[2063111874, 210353161, 1],
    );
    // - shift < Limb::WIDTH
    test(
        &[
            1347797001, 1439220470, 2750411815, 3145460224, 3430380546, 2707019846, 2327263540,
            551116682,
        ],
        &[1077346225, 1488699754, 3604020692, 1538514909],
        &[4064782248, 3993147064, 4166228975, 2172636662, 0],
    );
}

#[test]
fn limbs_sqrt_rem_to_out_fail() {
    // - xs empty
    assert_panic!({
        let out_sqrt = &mut [1, 2, 3];
        let out_rem = &mut [0; 8];
        let xs = &mut [];
        limbs_sqrt_rem_to_out(out_sqrt, out_rem, xs);
    });
    // - out too short
    assert_panic!({
        let out_sqrt = &mut [1, 2, 3];
        let out_rem = &mut [0; 8];
        let xs = &mut [Limb::MAX; 8];
        limbs_sqrt_rem_to_out(out_sqrt, out_rem, xs);
    });
    // - rem too short
    assert_panic!({
        let out_sqrt = &mut [1, 2, 3, 4];
        let out_rem = &mut [0; 7];
        let xs = &mut [Limb::MAX; 8];
        limbs_sqrt_rem_to_out(out_sqrt, out_rem, xs);
    });
    // - last element of xs is 0
    assert_panic!({
        let out_sqrt = &mut [1, 2, 3];
        let out_rem = &mut [0; 6];
        let xs = &mut [1, 2, 3, 4, 5, 0];
        limbs_sqrt_rem_to_out(out_sqrt, out_rem, xs);
    });
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_floor_sqrt() {
    fn test(xs: &[Limb], out: &[Limb]) {
        assert_eq!(limbs_floor_sqrt(xs), out);
    }
    test(&[1, 2, 3], &[3144134278, 1]);
}

#[test]
fn limbs_floor_sqrt_fail() {
    // - xs empty
    assert_panic!(limbs_floor_sqrt(&[]));
    // - last element of xs is 0
    assert_panic!(limbs_floor_sqrt(&[1, 2, 0]));
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_ceiling_sqrt() {
    fn test(xs: &[Limb], out: &[Limb]) {
        assert_eq!(limbs_ceiling_sqrt(xs), out);
    }
    test(&[1, 2, 3], &[3144134279, 1]);
}

#[test]
fn limbs_ceiling_sqrt_fail() {
    // - xs empty
    assert_panic!(limbs_ceiling_sqrt(&[]));
    // - last element of xs is 0
    assert_panic!(limbs_ceiling_sqrt(&[1, 2, 0]));
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_checked_sqrt() {
    fn test(xs: &[Limb], out: Option<&[Limb]>) {
        assert_eq!(limbs_checked_sqrt(xs), out.map(<[Limb]>::to_vec));
    }
    test(&[1, 2, 3], None);
    test(&[0, 0, 1], Some(&[0, 1]));
}

#[test]
fn limbs_checked_sqrt_fail() {
    // - xs empty
    assert_panic!(limbs_checked_sqrt(&[]));
    // - last element of xs is 0
    assert_panic!(limbs_checked_sqrt(&[1, 2, 0]));
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sqrt_rem() {
    fn test(xs: &[Limb], out_sqrt: &[Limb], out_rem: &[Limb]) {
        let (sqrt, rem) = limbs_sqrt_rem(xs);
        assert_eq!(sqrt, out_sqrt);
        assert_eq!(rem, out_rem);
    }
    test(&[1, 2, 3], &[3144134278, 1], &[1429311965, 0]);
}

#[test]
fn limbs_sqrt_rem_fail() {
    // - xs empty
    assert_panic!(limbs_sqrt_rem(&[]));
    // - last element of xs is 0
    assert_panic!(limbs_sqrt_rem(&[1, 2, 0]));
}

#[test]
fn test_floor_sqrt() {
    let test = |s, out| {
        let n = Natural::from_str(s).unwrap();
        assert_eq!(n.clone().floor_sqrt().to_string(), out);
        assert_eq!((&n).floor_sqrt().to_string(), out);
        assert_eq!(floor_sqrt_binary(&n).to_string(), out);

        let mut n = n;
        n.floor_sqrt_assign();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("2", "1");
    test("3", "1");
    test("4", "2");
    test("5", "2");
    test("10", "3");
    test("100", "10");
    test("1000000000", "31622");
    test("152415765279683", "12345677");
    test("152415765279684", "12345678");
    test("152415765279685", "12345678");
    test(
        "10000000000000000000000000000000000000000",
        "100000000000000000000",
    );
    test(
        "100000000000000000000000000000000000000000",
        "316227766016837933199",
    );
}

#[test]
fn test_ceiling_sqrt() {
    let test = |s, out| {
        let n = Natural::from_str(s).unwrap();
        assert_eq!(n.clone().ceiling_sqrt().to_string(), out);
        assert_eq!((&n).ceiling_sqrt().to_string(), out);
        assert_eq!(ceiling_sqrt_binary(&n).to_string(), out);

        let mut n = n;
        n.ceiling_sqrt_assign();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("2", "2");
    test("3", "2");
    test("4", "2");
    test("5", "3");
    test("10", "4");
    test("100", "10");
    test("1000000000", "31623");
    test("152415765279683", "12345678");
    test("152415765279684", "12345678");
    test("152415765279685", "12345679");
    test(
        "10000000000000000000000000000000000000000",
        "100000000000000000000",
    );
    test(
        "100000000000000000000000000000000000000000",
        "316227766016837933200",
    );
}

#[allow(clippy::redundant_closure_for_method_calls)]
#[test]
fn test_checked_sqrt() {
    let test = |s, out: Option<&str>| {
        let n = Natural::from_str(s).unwrap();
        let out = out.map(|s| s.to_string());

        assert_eq!(n.clone().checked_sqrt().map(|x| x.to_string()), out);
        assert_eq!((&n).checked_sqrt().map(|x| x.to_string()), out);
        assert_eq!(checked_sqrt_binary(&n).map(|x| x.to_string()), out);
    };
    test("0", Some("0"));
    test("1", Some("1"));
    test("2", None);
    test("3", None);
    test("4", Some("2"));
    test("5", None);
    test("10", None);
    test("100", Some("10"));
    test("1000000000", None);
    test("152415765279683", None);
    test("152415765279684", Some("12345678"));
    test("152415765279685", None);
    test(
        "10000000000000000000000000000000000000000",
        Some("100000000000000000000"),
    );
    test("100000000000000000000000000000000000000000", None);
}

#[test]
fn test_sqrt_rem() {
    let test = |s, sqrt_out, rem_out| {
        let n = Natural::from_str(s).unwrap();

        let (sqrt, rem) = n.clone().sqrt_rem();
        assert_eq!(sqrt.to_string(), sqrt_out);
        assert_eq!(rem.to_string(), rem_out);

        let (sqrt, rem) = (&n).sqrt_rem();
        assert_eq!(sqrt.to_string(), sqrt_out);
        assert_eq!(rem.to_string(), rem_out);

        let (sqrt, rem) = sqrt_rem_binary(&n);
        assert_eq!(sqrt.to_string(), sqrt_out);
        assert_eq!(rem.to_string(), rem_out);

        let mut n = n;
        assert_eq!(n.sqrt_assign_rem().to_string(), rem_out);
        assert_eq!(n.to_string(), sqrt_out);
    };
    test("0", "0", "0");
    test("1", "1", "0");
    test("2", "1", "1");
    test("3", "1", "2");
    test("4", "2", "0");
    test("5", "2", "1");
    test("10", "3", "1");
    test("100", "10", "0");
    test("1000000000", "31622", "49116");
    test("152415765279683", "12345677", "24691354");
    test("152415765279684", "12345678", "0");
    test("152415765279685", "12345678", "1");
    test(
        "10000000000000000000000000000000000000000",
        "100000000000000000000",
        "0",
    );
    test(
        "100000000000000000000000000000000000000000",
        "316227766016837933199",
        "562477137586013626399",
    );
}

#[test]
fn sqrt_rem_2_newton_properties() {
    unsigned_pair_gen_var_31().test_properties(|(n_hi, n_lo)| {
        let (sqrt, r_hi, r_lo) = sqrt_rem_2_newton(n_hi, n_lo);
        assert_eq!(
            DoubleLimb::from(sqrt)
                .square()
                .checked_add(DoubleLimb::join_halves(Limb::from(r_hi), r_lo))
                .unwrap(),
            DoubleLimb::join_halves(n_hi, n_lo)
        );
    });
}

#[test]
fn limbs_sqrt_rem_helper_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_4().test_properties_with_config(&config, |(mut out, mut xs)| {
        let n = out.len();
        let mut scratch = vec![0; limbs_sqrt_rem_helper_scratch_len(n)];
        let old_xs = xs.clone();
        let r_hi = limbs_sqrt_rem_helper(&mut out, &mut xs, 0, &mut scratch);
        let x = Natural::from_limbs_asc(&old_xs[..n << 1]);
        let sqrt = Natural::from_limbs_asc(&out);
        let mut rem = Natural::from_limbs_asc(&xs[..n]);
        if r_hi {
            rem.set_bit(u64::exact_from(n) << Limb::LOG_WIDTH);
        }
        assert_eq!((&x).sqrt_rem(), (sqrt.clone(), rem.clone()));
        assert_eq!((&sqrt).square() + rem, x);
        assert!((&sqrt).square() <= x);
        assert!((sqrt + Natural::ONE).square() > x);
    });
}

#[test]
fn limbs_sqrt_helper_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    large_type_gen_var_2().test_properties_with_config(&config, |(mut out, xs, shift, odd)| {
        let has_remainder = limbs_sqrt_helper(&mut out, &xs, shift, odd);
        let x = Natural::from_limbs_asc(&xs);
        let sqrt = Natural::from_limbs_asc(&out);
        let (sqrt_alt, rem) = (&x).sqrt_rem();
        assert_eq!(sqrt, sqrt_alt);
        assert_eq!(has_remainder, rem != 0);
        assert!((&sqrt).square() <= x);
        assert!((sqrt + Natural::ONE).square() > x);
    });
}

#[test]
fn limbs_sqrt_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_5().test_properties_with_config(&config, |(mut out, xs)| {
        limbs_sqrt_to_out(&mut out, &xs);
        let xs_len = xs.len();
        let sqrt_len = xs_len.shr_round(1, Ceiling).0;
        let x = Natural::from_limbs_asc(&xs);
        let sqrt = Natural::from_limbs_asc(&out[..sqrt_len]);
        assert_eq!((&x).floor_sqrt(), sqrt);
        assert!((&sqrt).square() <= x);
        assert!((sqrt + Natural::ONE).square() > x);
    });
}

#[test]
fn limbs_sqrt_rem_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_28().test_properties_with_config(
        &config,
        |(mut out_sqrt, mut out_rem, xs)| {
            let rem_len = limbs_sqrt_rem_to_out(&mut out_sqrt, &mut out_rem, &xs);
            let xs_len = xs.len();
            let sqrt_len = xs_len.shr_round(1, Ceiling).0;
            let x = Natural::from_limbs_asc(&xs);
            let sqrt = Natural::from_limbs_asc(&out_sqrt[..sqrt_len]);
            let rem = Natural::from_limbs_asc(&out_rem[..rem_len]);
            let (sqrt_alt, rem_alt) = (&x).sqrt_rem();
            assert_eq!(sqrt_alt, sqrt);
            assert_eq!(rem_alt, rem);
            assert_eq!((&sqrt).square() + &rem, x);
            assert!((&sqrt).square() <= x);
            assert!((sqrt + Natural::ONE).square() > x);
        },
    );
}

#[test]
fn limbs_floor_sqrt_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen_var_1().test_properties_with_config(&config, |xs| {
        let sqrt = Natural::from_owned_limbs_asc(limbs_floor_sqrt(&xs));
        let x = Natural::from_owned_limbs_asc(xs);
        assert_eq!((&x).floor_sqrt(), sqrt);
        assert!((&sqrt).square() <= x);
        assert!((sqrt + Natural::ONE).square() > x);
    });
}

#[test]
fn limbs_ceiling_sqrt_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen_var_1().test_properties_with_config(&config, |xs| {
        let sqrt = Natural::from_owned_limbs_asc(limbs_ceiling_sqrt(&xs));
        let x = Natural::from_owned_limbs_asc(xs);
        assert_eq!((&x).ceiling_sqrt(), sqrt);
        assert!((&sqrt).square() >= x);
        assert!((sqrt - Natural::ONE).square() < x);
    });
}

#[test]
fn limbs_checked_sqrt_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen_var_1().test_properties_with_config(&config, |xs| {
        let sqrt = limbs_checked_sqrt(&xs).map(Natural::from_owned_limbs_asc);
        let x = Natural::from_owned_limbs_asc(xs);
        assert_eq!((&x).checked_sqrt(), sqrt);
        if let Some(sqrt) = sqrt {
            assert_eq!(sqrt.square(), x);
        }
    });
}

#[test]
fn limbs_sqrt_rem_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen_var_1().test_properties_with_config(&config, |xs| {
        let (sqrt, rem) = limbs_sqrt_rem(&xs);
        let sqrt = Natural::from_owned_limbs_asc(sqrt);
        let rem = Natural::from_owned_limbs_asc(rem);
        let x = Natural::from_owned_limbs_asc(xs);
        assert_eq!((&sqrt).square() + &rem, x);
        assert_eq!((&x).sqrt_rem(), (sqrt, rem));
    });
}

#[test]
fn floor_sqrt_properties() {
    natural_gen().test_properties(|n| {
        let sqrt = n.clone().floor_sqrt();
        assert_eq!((&n).floor_sqrt(), sqrt);
        let mut n_alt = n.clone();
        n_alt.floor_sqrt_assign();
        assert_eq!(n_alt, sqrt);
        assert_eq!(floor_sqrt_binary(&n), sqrt);
        assert_eq!((&n).floor_root(2), sqrt);
        assert_eq!(Natural::from(&BigUint::from(&n).sqrt()), sqrt);
        assert_eq!(Natural::exact_from(&rug::Integer::from(&n).sqrt()), sqrt);

        let square = (&sqrt).square();
        let ceiling_sqrt = (&n).ceiling_sqrt();
        if square == n {
            assert_eq!(ceiling_sqrt, sqrt);
        } else {
            assert_eq!(ceiling_sqrt, &sqrt + Natural::ONE);
        }
        assert!(square <= n);
        assert!((sqrt + Natural::ONE).square() > n);
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert_eq!(u.floor_sqrt(), Natural::from(u).floor_sqrt());
    });
}

#[test]
fn ceiling_sqrt_properties() {
    natural_gen().test_properties(|n| {
        let sqrt = n.clone().ceiling_sqrt();
        assert_eq!((&n).ceiling_sqrt(), sqrt);
        let mut n_alt = n.clone();
        n_alt.ceiling_sqrt_assign();
        assert_eq!(n_alt, sqrt);
        assert_eq!(ceiling_sqrt_binary(&n), sqrt);
        assert_eq!((&n).ceiling_root(2), sqrt);
        let square = (&sqrt).square();
        let floor_sqrt = (&n).floor_sqrt();
        if square == n {
            assert_eq!(floor_sqrt, sqrt);
        } else {
            assert_eq!(floor_sqrt, &sqrt - Natural::ONE);
        }
        assert!(square >= n);
        if n != 0 {
            assert!((sqrt - Natural::ONE).square() < n);
        }
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert_eq!(u.ceiling_sqrt(), Natural::from(u).ceiling_sqrt());
    });
}

#[test]
fn checked_sqrt_properties() {
    natural_gen().test_properties(|n| {
        let sqrt = n.clone().checked_sqrt();
        assert_eq!((&n).checked_sqrt(), sqrt);
        assert_eq!(checked_sqrt_binary(&n), sqrt);
        assert_eq!((&n).checked_root(2), sqrt);
        if let Some(sqrt) = sqrt {
            assert_eq!((&sqrt).square(), n);
            assert_eq!((&n).floor_sqrt(), sqrt);
            assert_eq!(n.ceiling_sqrt(), sqrt);
        }
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert_eq!(
            u.checked_sqrt().map(Natural::from),
            Natural::from(u).checked_sqrt()
        );
    });
}

#[test]
fn sqrt_rem_properties() {
    natural_gen().test_properties(|n| {
        let (sqrt, rem) = n.clone().sqrt_rem();
        assert_eq!((&n).sqrt_rem(), (sqrt.clone(), rem.clone()));
        let mut n_alt = n.clone();
        assert_eq!(n_alt.sqrt_assign_rem(), rem);
        assert_eq!(n_alt, sqrt);
        assert_eq!(sqrt_rem_binary(&n), (sqrt.clone(), rem.clone()));
        assert_eq!((&n).root_rem(2), (sqrt.clone(), rem.clone()));
        let (rug_sqrt, rug_rem) = rug::Integer::from(&n).sqrt_rem(rug::Integer::new());
        assert_eq!(Natural::exact_from(&rug_sqrt), sqrt);
        assert_eq!(Natural::exact_from(&rug_rem), rem);

        assert_eq!((&n).floor_sqrt(), sqrt);
        assert!(rem <= &sqrt << 1);
        assert_eq!(sqrt.square() + rem, n);
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        let (sqrt, rem) = u.sqrt_rem();
        assert_eq!(
            (Natural::from(sqrt), Natural::from(rem)),
            Natural::from(u).sqrt_rem()
        );
    });
}
