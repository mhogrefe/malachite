// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivisibleBy, EqMod, Mod, UnsignedAbs};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    signed_triple_gen, unsigned_vec_triple_gen_var_36,
    unsigned_vec_unsigned_unsigned_triple_gen_var_5,
    unsigned_vec_unsigned_unsigned_triple_gen_var_7,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6,
};
use malachite_nz::integer::arithmetic::eq_mod::{
    limbs_eq_neg_limb_mod_limb, limbs_pos_eq_neg_limb_mod, limbs_pos_eq_neg_limb_mod_ref,
    limbs_pos_eq_neg_mod, limbs_pos_eq_neg_mod_limb, limbs_pos_eq_neg_mod_ref,
    limbs_pos_limb_eq_neg_limb_mod,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::{
    integer_integer_natural_triple_gen, integer_integer_natural_triple_gen_var_1,
    integer_integer_natural_triple_gen_var_2, integer_natural_pair_gen, integer_pair_gen,
    natural_triple_gen, unsigned_vec_triple_gen_var_37, unsigned_vec_triple_gen_var_38,
    unsigned_vec_unsigned_unsigned_triple_gen_var_6,
    unsigned_vec_unsigned_unsigned_vec_triple_gen_var_2,
    unsigned_vec_unsigned_unsigned_vec_triple_gen_var_3,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_7,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_8,
};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_neg_limb_mod_limb() {
    let test = |xs: &[Limb], y: Limb, m: Limb, equal: bool| {
        assert_eq!(limbs_eq_neg_limb_mod_limb(xs, y, m), equal);
    };
    test(&[6, 7], 4, 2, true);
    test(&[7, 7], 4, 2, false);
    test(&[6, 7], 3, 2, false);
    test(&[7, 7], 3, 2, true);
    test(&[2, 2], 6, 13, true);
    test(&[100, 101, 102], 1232, 10, true);
    test(&[100, 101, 102], 1233, 10, false);
    test(&[123, 456], 153, 789, true);
    test(&[123, 456], 1000, 789, false);
    test(&[u32::MAX, u32::MAX], 101, 2, true);
    test(&[u32::MAX, u32::MAX], 100, 2, false);
    test(&[u32::MAX, u32::MAX], 111, 3, true);
    test(&[u32::MAX, u32::MAX], 110, 3, false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_neg_limb_mod_limb_fail() {
    limbs_eq_neg_limb_mod_limb(&[10], 10, 15);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_limb_eq_neg_limb_mod() {
    let test = |x: Limb, y: Limb, ms: &[Limb], equal: bool| {
        assert_eq!(limbs_pos_limb_eq_neg_limb_mod(x, y, ms), equal);
        let x = Integer::from(x);
        let y = -Natural::from(y);
        let m = Natural::from_limbs_asc(ms);
        assert_eq!((&x).eq_mod(&y, &m), equal);
        let m = Integer::from(m);
        assert_eq!(
            x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m),
            equal
        );
        assert_eq!((x - y).divisible_by(m), equal);
    };
    test(1, 1, &[1, 1], false);
    test(1, 1, &[2, 1], false);
    test(1, 1, &[1, 0, 1], false);
    test(u32::MAX, u32::MAX, &[u32::MAX - 1, 1], true);
    test(u32::MAX, u32::MAX, &[u32::MAX - 1, 1, 2], false);
    test(u32::MAX, u32::MAX, &[u32::MAX - 1, 2], false);
    test(0xabcddbca, 0x641fefdf, &[0xfedcba9, 1], true);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_eq_neg_limb_mod() {
    let test = |xs: &[Limb], y: Limb, ms: &[Limb], equal: bool| {
        let mut mut_ms = ms.to_vec();
        assert_eq!(limbs_pos_eq_neg_limb_mod(xs, y, &mut mut_ms), equal);
        assert_eq!(limbs_pos_eq_neg_limb_mod_ref(xs, y, ms), equal);
        let x = Integer::from(Natural::from_limbs_asc(xs));
        let y = -Natural::from(y);
        let m = Natural::from_limbs_asc(ms);
        assert_eq!((&x).eq_mod(&y, &m), equal);
        let m = Integer::from(m);
        assert_eq!(
            x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m),
            equal
        );
        assert_eq!((x - y).divisible_by(m), equal);
    };
    // - !xs[0].wrapping_neg().eq_mod_power_of_2(y, u64::from(twos))
    test(&[1, 2], 2, &[2, 1], false);
    // - xs[0].wrapping_neg().eq_mod_power_of_2(y, u64::from(twos))
    // - m_len == 2 && m_0 != 0
    // - m_1 < 1 << twos
    // - x_len < BMOD_1_TO_MOD_1_THRESHOLD
    test(&[2, 2], 2, &[2, 1], true);
    // - m_1 >= 1 << twos
    test(&[0, 1], 1, &[1, 1], true);
    // - m_len > 2 || m_0 == 0
    test(&[0, 1], 1, &[1, 0, 1], false);
    // - x_len >= BMOD_1_TO_MOD_1_THRESHOLD
    // - y < m_0
    test(
        &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1,
        ],
        2,
        &[2, 1],
        false,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_limb_mod_fail_1() {
    limbs_pos_eq_neg_limb_mod(&[1], 1, &mut [0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_limb_mod_fail_2() {
    limbs_pos_eq_neg_limb_mod(&[1, 1], 1, &mut [1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_limb_mod_fail_3() {
    limbs_pos_eq_neg_limb_mod(&[1, 0], 1, &mut [0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_limb_mod_fail_4() {
    limbs_pos_eq_neg_limb_mod(&[1, 1], 0, &mut [0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_limb_mod_fail_5() {
    limbs_pos_eq_neg_limb_mod(&[1, 1], 1, &mut [1, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_limb_mod_ref_fail_1() {
    limbs_pos_eq_neg_limb_mod_ref(&[1], 1, &[0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_limb_mod_ref_fail_2() {
    limbs_pos_eq_neg_limb_mod_ref(&[1, 1], 1, &[1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_limb_mod_ref_fail_3() {
    limbs_pos_eq_neg_limb_mod_ref(&[1, 0], 1, &[0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_limb_mod_ref_fail_4() {
    limbs_pos_eq_neg_limb_mod_ref(&[1, 1], 0, &[0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_limb_mod_ref_fail_5() {
    limbs_pos_eq_neg_limb_mod_ref(&[1, 1], 1, &[1, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_eq_neg_mod_limb() {
    let test = |xs: &[Limb], ys: &[Limb], m: Limb, equal: bool| {
        assert_eq!(limbs_pos_eq_neg_mod_limb(xs, ys, m), equal);
        let x = Integer::from(Natural::from_limbs_asc(xs));
        let y = -Natural::from_limbs_asc(ys);
        let m = Natural::from(m);
        assert_eq!((&x).eq_mod(&y, &m), equal);
        let m = Integer::from(m);
        assert_eq!(
            x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m),
            equal
        );
        assert_eq!((x - y).divisible_by(m), equal);
    };
    // - xs[0].wrapping_neg().eq_mod_power_of_2(ys[0], u64::from(m.trailing_zeros())) in
    //   limbs_pos_eq_mod_neg_limb_greater
    test(&[0, 1], &[0, 1], 1, true);
    test(&[0, 1], &[0, 1], 2, true);
    test(&[0, 1], &[6, 1], 2, true);
    // - !xs[0].wrapping_neg().eq_mod_power_of_2(ys[0], u64::from(m.trailing_zeros())) in
    //   limbs_pos_eq_mod_neg_limb_greater
    test(&[0, 1], &[7, 1], 2, false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_limb_fail_1() {
    limbs_pos_eq_neg_mod_limb(&[1], &[3, 4], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_limb_fail_2() {
    limbs_pos_eq_neg_mod_limb(&[1, 1], &[4], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_limb_fail_3() {
    limbs_pos_eq_neg_mod_limb(&[1, 0], &[3, 4], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_limb_fail_4() {
    limbs_pos_eq_neg_mod_limb(&[1, 1], &[3, 0], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_limb_fail_5() {
    limbs_pos_eq_neg_mod_limb(&[1, 1], &[3, 4], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_eq_neg_mod() {
    let test = |xs: &[Limb], ys: &[Limb], ms: &[Limb], equal: bool| {
        let mut mut_ms = ms.to_vec();
        assert_eq!(limbs_pos_eq_neg_mod(xs, ys, &mut mut_ms), equal);
        assert_eq!(limbs_pos_eq_neg_mod_ref(xs, ys, ms), equal);
        let x = Integer::from(Natural::from_limbs_asc(xs));
        let y = -Natural::from_limbs_asc(ys);
        let m = Natural::from_limbs_asc(ms);
        assert_eq!((&x).eq_mod(&y, &m), equal);
        let m = Integer::from(m);
        assert_eq!(
            x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m),
            equal
        );
        assert_eq!((x - y).divisible_by(m), equal);
    };
    // - !xs[0].wrapping_neg().eq_mod_power_of_2(ys[0], u64::from(ms[0].trailing_zeros())) in
    //   limbs_pos_eq_neg_mod_greater
    test(&[1, 2], &[3, 4], &[0, 1], false);
    test(&[0, 0, 1], &[0, 1], &[1, 1], true);
    // - xs[0].wrapping_neg().eq_mod_power_of_2(ys[0], u64::from(ms[0].trailing_zeros())) in
    //   limbs_pos_eq_neg_mod_greater
    test(
        &[
            936369948, 322455623, 3632895046, 978349680, 17000327, 2833388987, 2719643819,
            4166701038,
        ],
        &[
            2342728269, 2320695303, 2977562202, 4108534583, 1505907268, 3739165110, 101046064,
            1901445664,
        ],
        &[602975281, 3649288173, 1789153785, 3864060421, 3382875975, 610141130],
        false,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_fail_1() {
    limbs_pos_eq_neg_mod(&[1], &[1, 0, 3], &mut [0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_fail_2() {
    limbs_pos_eq_neg_mod(&[1, 1, 1], &[1], &mut [0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_fail_3() {
    limbs_pos_eq_neg_mod(&[1, 1, 1], &[1, 0, 3], &mut [7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_fail_4() {
    limbs_pos_eq_neg_mod(&[1, 1, 0], &[1, 0, 3], &mut [0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_fail_5() {
    limbs_pos_eq_neg_mod(&[1, 1, 1], &[1, 0, 0], &mut [0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_fail_6() {
    limbs_pos_eq_neg_mod(&[1, 1, 1], &[1, 0, 3], &mut [7, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_ref_fail_1() {
    limbs_pos_eq_neg_mod_ref(&[1], &[1, 0, 3], &[0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_ref_fail_2() {
    limbs_pos_eq_neg_mod_ref(&[1, 1, 1], &[1], &[0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_ref_fail_3() {
    limbs_pos_eq_neg_mod_ref(&[1, 1, 1], &[1, 0, 3], &[7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_ref_fail_4() {
    limbs_pos_eq_neg_mod_ref(&[1, 1, 0], &[1, 0, 3], &[0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_ref_fail_5() {
    limbs_pos_eq_neg_mod_ref(&[1, 1, 1], &[1, 0, 0], &[0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_ref_fail_6() {
    limbs_pos_eq_neg_mod_ref(&[1, 1, 1], &[1, 0, 3], &[7, 0]);
}

#[test]
fn test_eq_mod() {
    let test = |r, s, t, out| {
        let u = Integer::from_str(r).unwrap();
        let v = Integer::from_str(s).unwrap();
        let w = Natural::from_str(t).unwrap();

        assert_eq!(u.clone().eq_mod(v.clone(), w.clone()), out);
        assert_eq!(u.clone().eq_mod(v.clone(), &w), out);
        assert_eq!(u.clone().eq_mod(&v, w.clone()), out);
        assert_eq!(u.clone().eq_mod(&v, &w), out);
        assert_eq!((&u).eq_mod(v.clone(), w.clone()), out);
        assert_eq!((&u).eq_mod(v.clone(), &w), out);
        assert_eq!((&u).eq_mod(&v, w.clone()), out);
        assert_eq!((&u).eq_mod(&v, &w), out);
        assert_eq!(v.eq_mod(u, w), out);
        assert_eq!(
            rug::Integer::from_str(r).unwrap().is_congruent(
                &rug::Integer::from_str(s).unwrap(),
                &rug::Integer::from_str(t).unwrap()
            ),
            out
        );
    };
    test("0", "0", "0", true);
    test("0", "1", "0", false);
    test("57", "57", "0", true);
    test("57", "58", "0", false);
    test("1000000000000", "57", "0", false);
    test("0", "256", "256", true);
    test("0", "256", "512", false);
    test("13", "23", "10", true);
    test("13", "24", "10", false);
    test("13", "21", "1", true);
    test("13", "21", "2", true);
    test("13", "21", "4", true);
    test("13", "21", "8", true);
    test("13", "21", "16", false);
    test("13", "21", "3", false);
    test("1000000000001", "1", "4096", true);
    test("1000000000001", "1", "8192", false);
    test("12345678987654321", "321", "1000", true);
    test("12345678987654321", "322", "1000", false);
    test("1234", "1234", "1000000000000", true);
    test("1234", "1235", "1000000000000", false);
    test("1000000001234", "1000000002234", "1000", true);
    test("1000000001234", "1000000002235", "1000", false);
    test("1000000001234", "1234", "1000000000000", true);
    test("1000000001234", "1235", "1000000000000", false);
    test("1000000001234", "5000000001234", "1000000000000", true);
    test("1000000001234", "5000000001235", "1000000000000", false);

    test("0", "-1", "0", false);
    test("57", "-57", "0", false);
    test("57", "-58", "0", false);
    test("1000000000000", "-57", "0", false);
    test("0", "-256", "256", true);
    test("0", "-256", "512", false);
    test("13", "-27", "10", true);
    test("13", "-28", "10", false);
    test("29", "-27", "1", true);
    test("29", "-27", "2", true);
    test("29", "-27", "4", true);
    test("29", "-27", "8", true);
    test("29", "-27", "16", false);
    test("29", "-27", "3", false);
    test("999999999999", "-1", "4096", true);
    test("999999999999", "-1", "8192", false);
    test("12345678987654321", "-679", "1000", true);
    test("12345678987654321", "-680", "1000", false);
    test("1000000001234", "-999999999766", "1000", true);
    test("1000000001234", "-999999999767", "1000", false);
    test("1000000001234", "-999999998766", "1000000000000", true);
    test("1000000001234", "-999999998767", "1000000000000", false);

    test("-1", "0", "0", false);
    test("-57", "57", "0", false);
    test("-57", "58", "0", false);
    test("-1000000000000", "57", "0", false);
    test("-256", "0", "256", true);
    test("-256", "0", "512", false);
    test("-13", "27", "10", true);
    test("-13", "28", "10", false);
    test("-29", "27", "1", true);
    test("-29", "27", "2", true);
    test("-29", "27", "4", true);
    test("-29", "27", "8", true);
    test("-29", "27", "16", false);
    test("-29", "27", "3", false);
    test("-999999999999", "1", "4096", true);
    test("-999999999999", "1", "8192", false);
    test("-12345678987654321", "679", "1000", true);
    test("-12345678987654321", "680", "1000", false);
    test("-1000000001234", "999999999766", "1000", true);
    test("-1000000001234", "999999999767", "1000", false);
    test("-1000000001234", "999999998766", "1000000000000", true);
    test("-1000000001234", "999999998767", "1000000000000", false);

    test("-57", "-57", "0", true);
    test("-57", "-58", "0", false);
    test("-1000000000000", "-57", "0", false);
    test("-13", "-23", "10", true);
    test("-13", "-24", "10", false);
    test("-13", "-21", "1", true);
    test("-13", "-21", "2", true);
    test("-13", "-21", "4", true);
    test("-13", "-21", "8", true);
    test("-13", "-21", "16", false);
    test("-13", "-21", "3", false);
    test("-1000000000001", "-1", "4096", true);
    test("-1000000000001", "-1", "8192", false);
    test("-12345678987654321", "-321", "1000", true);
    test("-12345678987654321", "-322", "1000", false);
    test("-1234", "-1234", "1000000000000", true);
    test("-1234", "-1235", "1000000000000", false);
    test("-1000000001234", "-1000000002234", "1000", true);
    test("-1000000001234", "-1000000002235", "1000", false);
    test("-1000000001234", "-1234", "1000000000000", true);
    test("-1000000001234", "-1235", "1000000000000", false);
    test("-1000000001234", "-5000000001234", "1000000000000", true);
    test("-1000000001234", "-5000000001235", "1000000000000", false);
}

#[test]
fn limbs_eq_neg_limb_mod_limb_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_unsigned_triple_gen_var_7().test_properties_with_config(
        &config,
        |(xs, y, m)| {
            let equal = limbs_eq_neg_limb_mod_limb(&xs, y, m);
            assert_eq!(
                (-Natural::from_owned_limbs_asc(xs)).eq_mod(Integer::from(y), Natural::from(m)),
                equal
            );
        },
    );
}

#[test]
fn limbs_pos_limb_eq_neg_limb_mod_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_unsigned_triple_gen_var_5().test_properties_with_config(
        &config,
        |(ms, x, y)| {
            let equal = limbs_pos_limb_eq_neg_limb_mod(x, y, &ms);
            let x = Integer::from(x);
            let y = -Natural::from(y);
            let m = Natural::from_owned_limbs_asc(ms);
            assert_eq!((&x).eq_mod(&y, &m), equal);
            let m = Integer::from(m);
            assert_eq!(
                x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m),
                equal
            );
            assert_eq!((x - y).divisible_by(m), equal);
        },
    );

    unsigned_vec_unsigned_unsigned_triple_gen_var_6().test_properties_with_config(
        &config,
        |(ms, x, y)| {
            assert!(!limbs_pos_limb_eq_neg_limb_mod(x, y, &ms));
            let x = Integer::from(x);
            let y = -Natural::from(y);
            let m = Natural::from_owned_limbs_asc(ms);
            assert!(!(&x).eq_mod(&y, &m));
            let m = Integer::from(m);
            assert!(x != y && (m == 0 || (&x).mod_op(&m) != (&y).mod_op(&m)));
            assert!(!(x - y).divisible_by(m));
        },
    );
}

#[test]
fn limbs_pos_eq_neg_limb_mod_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6().test_properties_with_config(
        &config,
        |(xs, mut ms, y)| {
            let equal = limbs_pos_eq_neg_limb_mod_ref(&xs, y, &ms);
            let m = Natural::from_limbs_asc(&ms);
            assert_eq!(limbs_pos_eq_neg_limb_mod(&xs, y, &mut ms), equal);
            let x = Integer::from(Natural::from_owned_limbs_asc(xs));
            let y = -Natural::from(y);

            assert_eq!((&x).eq_mod(&y, &m), equal);
            let m = Integer::from(m);
            assert_eq!(
                x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m),
                equal
            );
            assert_eq!((x - y).divisible_by(m), equal);
        },
    );

    unsigned_vec_unsigned_unsigned_vec_triple_gen_var_2().test_properties_with_config(
        &config,
        |(xs, y, mut ms)| {
            assert!(limbs_pos_eq_neg_limb_mod_ref(&xs, y, &ms));
            let m = Natural::from_limbs_asc(&ms);
            assert!(limbs_pos_eq_neg_limb_mod(&xs, y, &mut ms));
            let x = Integer::from(Natural::from_owned_limbs_asc(xs));
            let y = -Natural::from(y);
            assert!((&x).eq_mod(&y, &m));
            let m = Integer::from(m);
            assert!(x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m));
            assert!((x - y).divisible_by(m));
        },
    );

    unsigned_vec_unsigned_unsigned_vec_triple_gen_var_3().test_properties_with_config(
        &config,
        |(xs, y, mut ms)| {
            assert!(!limbs_pos_eq_neg_limb_mod_ref(&xs, y, &ms));
            let m = Natural::from_limbs_asc(&ms);
            assert!(!limbs_pos_eq_neg_limb_mod(&xs, y, &mut ms));
            let x = Integer::from(Natural::from_owned_limbs_asc(xs));
            let y = -Natural::from(y);
            assert!(!(&x).eq_mod(&y, &m));
            let m = Integer::from(m);
            assert!(x != y && (m == 0 || (&x).mod_op(&m) != (&y).mod_op(&m)));
            assert!(!(x - y).divisible_by(m));
        },
    );
}

#[test]
fn limbs_pos_eq_neg_mod_limb_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6().test_properties_with_config(
        &config,
        |(xs, ys, m)| {
            let equal = limbs_pos_eq_neg_mod_limb(&xs, &ys, m);
            let x = Integer::from(Natural::from_owned_limbs_asc(xs));
            let y = -Natural::from_owned_limbs_asc(ys);
            let m = Natural::from(m);
            assert_eq!((&x).eq_mod(&y, &m), equal);
            let m = Integer::from(m);
            assert_eq!(
                x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m),
                equal
            );
            assert_eq!((x - y).divisible_by(m), equal);
        },
    );

    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_7().test_properties_with_config(
        &config,
        |(xs, ys, m)| {
            assert!(limbs_pos_eq_neg_mod_limb(&xs, &ys, m));
            let x = Integer::from(Natural::from_owned_limbs_asc(xs));
            let y = -Natural::from_owned_limbs_asc(ys);
            let m = Natural::from(m);
            assert!((&x).eq_mod(&y, &m));
            let m = Integer::from(m);
            assert!(x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m));
            assert!((x - y).divisible_by(m));
        },
    );

    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_8().test_properties_with_config(
        &config,
        |(xs, ys, m)| {
            assert!(!limbs_pos_eq_neg_mod_limb(&xs, &ys, m));
            let x = Integer::from(Natural::from_owned_limbs_asc(xs));
            let y = -Natural::from_owned_limbs_asc(ys);
            let m = Natural::from(m);
            assert!(!(&x).eq_mod(&y, &m));
            let m = Integer::from(m);
            assert!(x != y && (m == 0 || (&x).mod_op(&m) != (&y).mod_op(&m)));
            assert!(!(x - y).divisible_by(m));
        },
    );
}

#[test]
fn limbs_pos_eq_neg_mod_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_36().test_properties_with_config(&config, |(xs, ys, mut ms)| {
        let equal = limbs_pos_eq_neg_mod_ref(&xs, &ys, &ms);
        let m = Natural::from_limbs_asc(&ms);
        assert_eq!(limbs_pos_eq_neg_mod(&xs, &ys, &mut ms), equal);
        let x = Integer::from(Natural::from_owned_limbs_asc(xs));
        let y = -Natural::from_owned_limbs_asc(ys);
        assert_eq!((&x).eq_mod(&y, &m), equal);
        let m = Integer::from(m);
        assert_eq!(
            x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m),
            equal
        );
        assert_eq!((x - y).divisible_by(m), equal);
    });

    unsigned_vec_triple_gen_var_37().test_properties_with_config(&config, |(xs, ys, mut ms)| {
        assert!(limbs_pos_eq_neg_mod_ref(&xs, &ys, &ms));
        let m = Natural::from_limbs_asc(&ms);
        assert!(limbs_pos_eq_neg_mod(&xs, &ys, &mut ms));
        let x = Integer::from(Natural::from_owned_limbs_asc(xs));
        let y = -Natural::from_owned_limbs_asc(ys);
        assert!((&x).eq_mod(&y, &m));
        let m = Integer::from(m);
        assert!(x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m));
        assert!((x - y).divisible_by(m));
    });

    unsigned_vec_triple_gen_var_38().test_properties_with_config(&config, |(xs, ys, mut ms)| {
        assert!(!limbs_pos_eq_neg_mod_ref(&xs, &ys, &ms));
        let m = Natural::from_limbs_asc(&ms);
        assert!(!limbs_pos_eq_neg_mod(&xs, &ys, &mut ms));
        let x = Integer::from(Natural::from_owned_limbs_asc(xs));
        let y = -Natural::from_owned_limbs_asc(ys);
        assert!(!(&x).eq_mod(&y, &m));
        let m = Integer::from(m);
        assert!(x != y && (m == 0 || (&x).mod_op(&m) != (&y).mod_op(&m)));
        assert!(!(x - y).divisible_by(m));
    });
}

#[test]
fn eq_mod_properties() {
    integer_integer_natural_triple_gen().test_properties(|(x, y, m)| {
        let equal = (&x).eq_mod(&y, &m);
        assert_eq!((&y).eq_mod(&x, &m), equal);

        assert_eq!((&x).eq_mod(&y, m.clone()), equal);
        assert_eq!((&x).eq_mod(y.clone(), &m), equal);
        assert_eq!((&x).eq_mod(y.clone(), m.clone()), equal);
        assert_eq!(x.clone().eq_mod(&y, &m), equal);
        assert_eq!(x.clone().eq_mod(&y, m.clone()), equal);
        assert_eq!(x.clone().eq_mod(y.clone(), &m), equal);
        assert_eq!(x.clone().eq_mod(y.clone(), m.clone()), equal);

        assert_eq!((-&x).eq_mod(-&y, &m), equal);
        assert_eq!((&x - &y).divisible_by(Integer::from(&m)), equal);
        assert_eq!((&y - &x).divisible_by(Integer::from(&m)), equal);
        assert_eq!(
            rug::Integer::from(&x).is_congruent(&rug::Integer::from(&y), &rug::Integer::from(&m)),
            equal
        );
    });

    integer_integer_natural_triple_gen_var_1().test_properties(|(ref x, ref y, ref m)| {
        assert!(x.eq_mod(y, m));
        assert!(y.eq_mod(x, m));
        assert!(rug::Integer::from(x).is_congruent(&rug::Integer::from(y), &rug::Integer::from(m)));
    });

    integer_integer_natural_triple_gen_var_2().test_properties(|(ref x, ref y, ref m)| {
        assert!(!x.eq_mod(y, m));
        assert!(!y.eq_mod(x, m));
        assert!(
            !rug::Integer::from(x).is_congruent(&rug::Integer::from(y),
            &rug::Integer::from(m))
        );
    });

    integer_pair_gen().test_properties(|(x, y)| {
        assert!((&x).eq_mod(&y, Natural::ONE));
        assert_eq!((&x).eq_mod(&y, Natural::ZERO), x == y);
    });

    integer_natural_pair_gen().test_properties(|(x, m)| {
        assert_eq!(
            (&x).eq_mod(Integer::ZERO, &m),
            (&x).divisible_by(Integer::from(&m))
        );
        assert!((&x).eq_mod(&x, m));
    });

    natural_triple_gen().test_properties(|(x, y, m)| {
        assert_eq!(
            Integer::from(&x).eq_mod(Integer::from(&y), &m),
            x.eq_mod(y, m)
        );
    });

    signed_triple_gen::<SignedLimb>().test_properties(|(x, y, m)| {
        assert_eq!(
            Integer::from(x).eq_mod(Integer::from(y), Integer::from(m).unsigned_abs()),
            x.eq_mod(y, m)
        );
    });
}
