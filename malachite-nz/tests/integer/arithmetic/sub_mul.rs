// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CheckedSubMul, SubMul, SubMulAssign, UnsignedAbs};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::test_util::generators::{
    signed_triple_gen, signed_triple_gen_var_2, unsigned_vec_triple_gen_var_39,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10,
};
use malachite_nz::integer::arithmetic::sub_mul::{
    limbs_overflowing_sub_mul, limbs_overflowing_sub_mul_in_place_left,
    limbs_overflowing_sub_mul_limb, limbs_overflowing_sub_mul_limb_in_place_either,
    limbs_overflowing_sub_mul_limb_in_place_left, limbs_overflowing_sub_mul_limb_in_place_right,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::{integer_gen, integer_pair_gen, integer_triple_gen};
use std::str::FromStr;

#[test]
fn test_limbs_overflowing_sub_mul_limb() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], z: Limb, result: &[Limb], sign: bool| {
        let (result_alt, sign_alt) = limbs_overflowing_sub_mul_limb(xs_before, ys_before, z);
        assert_eq!(&result_alt, &result);
        assert_eq!(sign_alt, sign);
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_overflowing_sub_mul_limb_in_place_left(&mut xs, ys_before, z),
            sign
        );
        assert_eq!(xs, result);
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_overflowing_sub_mul_limb_in_place_right(xs_before, &mut ys, z),
            sign
        );
        assert_eq!(ys, result);
    };
    #[cfg(feature = "32_bit_limbs")]
    {
        test(&[123, 456], &[123], 4, &[4294966927, 455, 0], true);
        test(&[123, 456], &[123], u32::MAX, &[246, 333, 0], true);
        test(&[123], &[123], 1, &[0, 0], true);
        test(&[123], &[123], 4, &[369, 0], false);
        test(
            &[123],
            &[123, 456],
            u32::MAX,
            &[4294967050, 4294966962, 455],
            false,
        );
        test(
            &[8191],
            &[0, 4160749568],
            1610678208,
            &[4294959105, 4294967295, 1560344513],
            false,
        );
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test(
            &[18446744073709551615],
            &[0, 18446744056529682432],
            18446742974197923840,
            &[1, 18446744073709551615, 18446742957018055679],
            false,
        );
    }
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_overflowing_sub_mul_limb_in_place_either() {
    let test = |xs_before: &[Limb],
                ys_before: &[Limb],
                z: Limb,
                right: bool,
                xs_after: &[Limb],
                ys_after: &[Limb],
                sign| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_overflowing_sub_mul_limb_in_place_either(&mut xs, &mut ys, z),
            (right, sign)
        );
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(
        &[123, 456],
        &[123],
        4,
        false,
        &[4294966927, 455, 0],
        &[123],
        true,
    );
    test(
        &[123, 456],
        &[123],
        u32::MAX,
        false,
        &[246, 333, 0],
        &[123],
        true,
    );
    test(&[123], &[123], 1, false, &[0, 0], &[123], true);
    test(&[123], &[123], 4, false, &[369, 0], &[123], false);
    test(
        &[123],
        &[123, 456],
        u32::MAX,
        true,
        &[123],
        &[4294967050, 4294966962, 455],
        false,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_overflowing_sub_mul_and_limbs_overflowing_sub_mul_in_place_left() {
    let test = |xs_before: &[Limb], ys: &[Limb], zs: &[Limb], result: &[Limb], sign: bool| {
        let (result_alt, sign_alt) = limbs_overflowing_sub_mul(xs_before, ys, zs);
        assert_eq!(result_alt, result);
        assert_eq!(sign_alt, sign);
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_overflowing_sub_mul_in_place_left(&mut xs, ys, zs),
            sign
        );
        assert_eq!(xs, result);
    };
    test(
        &[123, 456],
        &[123, 789],
        &[321, 654],
        &[39360, 333255, 516006],
        false,
    );
    test(
        &[123, 456, 789, 987, 654],
        &[123, 789],
        &[321, 654],
        &[4294927936, 4294634040, 4294452078, 986, 654],
        true,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_overflowing_sub_mul_fail_1() {
    limbs_overflowing_sub_mul(&[10, 10], &[], &[10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_overflowing_sub_mul_fail_2() {
    limbs_overflowing_sub_mul(&[10, 10], &[10, 10], &[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_overflowing_sub_mul_in_place_left_fail_1() {
    let mut xs = vec![10, 10];
    limbs_overflowing_sub_mul_in_place_left(&mut xs, &[], &[10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_overflowing_sub_mul_in_place_left_fail_2() {
    let mut xs = vec![10, 10];
    limbs_overflowing_sub_mul_in_place_left(&mut xs, &[10, 10], &[]);
}

#[test]
fn test_sub_mul() {
    let test = |r, s, t, out| {
        let u = Integer::from_str(r).unwrap();
        let v = Integer::from_str(s).unwrap();
        let w = Integer::from_str(t).unwrap();

        let mut a = u.clone();
        a.sub_mul_assign(v.clone(), w.clone());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = u.clone();
        a.sub_mul_assign(v.clone(), &w);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = u.clone();
        a.sub_mul_assign(&v, w.clone());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = u.clone();
        a.sub_mul_assign(&v, &w);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = u.clone().sub_mul(v.clone(), w.clone());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = u.clone().sub_mul(v.clone(), &w);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = u.clone().sub_mul(&v, w.clone());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = u.clone().sub_mul(&v, &w);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&u).sub_mul(&v, &w);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());
    };
    test("0", "0", "0", "0");
    test("0", "0", "123", "0");
    test("123", "0", "5", "123");
    test("123", "-5", "1", "128");
    test("123", "-5", "100", "623");
    test("10", "-3", "4", "22");
    test("1000000000000", "0", "123", "1000000000000");
    test("1000000000000", "-1", "123", "1000000000123");
    test("1000000000000", "-123", "1", "1000000000123");
    test("1000000000000", "-123", "100", "1000000012300");
    test("1000000000000", "-100", "123", "1000000012300");
    test("1000000000000", "-65536", "65536", "1004294967296");
    test("1000000000000", "-1000000000000", "0", "1000000000000");
    test("1000000000000", "-1000000000000", "1", "2000000000000");
    test("1000000000000", "-1000000000000", "100", "101000000000000");
    test("0", "-1000000000000", "100", "100000000000000");
    test(
        "1000000000000",
        "-65536",
        "1000000000000",
        "65537000000000000",
    );
    test(
        "1000000000000",
        "-1000000000000",
        "1000000000000",
        "1000000000001000000000000",
    );
    test(
        "0",
        "-1000000000000",
        "1000000000000",
        "1000000000000000000000000",
    );

    test("123", "5", "-1", "128");
    test("123", "5", "-100", "623");
    test("10", "3", "-4", "22");
    test("1000000000000", "1", "-123", "1000000000123");
    test("1000000000000", "123", "-1", "1000000000123");
    test("1000000000000", "123", "-100", "1000000012300");
    test("1000000000000", "100", "-123", "1000000012300");
    test("1000000000000", "65536", "-65536", "1004294967296");
    test("1000000000000", "1000000000000", "-1", "2000000000000");
    test("1000000000000", "1000000000000", "-100", "101000000000000");
    test("0", "1000000000000", "-100", "100000000000000");
    test(
        "1000000000000",
        "65536",
        "-1000000000000",
        "65537000000000000",
    );
    test(
        "1000000000000",
        "1000000000000",
        "-1000000000000",
        "1000000000001000000000000",
    );
    test(
        "0",
        "1000000000000",
        "-1000000000000",
        "1000000000000000000000000",
    );

    test("0", "0", "-123", "0");
    test("123", "0", "-5", "123");
    test("123", "5", "1", "118");
    test("123", "-5", "-1", "118");
    test("123", "5", "100", "-377");
    test("123", "-5", "-100", "-377");
    test("10", "3", "4", "-2");
    test("10", "-3", "-4", "-2");
    test("15", "3", "4", "3");
    test("15", "-3", "-4", "3");
    test("1000000000000", "0", "-123", "1000000000000");
    test("1000000000000", "1", "123", "999999999877");
    test("1000000000000", "-1", "-123", "999999999877");
    test("1000000000000", "123", "1", "999999999877");
    test("1000000000000", "-123", "-1", "999999999877");
    test("1000000000000", "123", "100", "999999987700");
    test("1000000000000", "-123", "-100", "999999987700");
    test("1000000000000", "100", "123", "999999987700");
    test("1000000000000", "-100", "-123", "999999987700");
    test("1000000000000", "65536", "65536", "995705032704");
    test("1000000000000", "-65536", "-65536", "995705032704");
    test("1000000000000", "1000000000000", "0", "1000000000000");
    test("1000000000000", "1000000000000", "1", "0");
    test("1000000000000", "-1000000000000", "-1", "0");
    test("1000000000000", "1000000000000", "100", "-99000000000000");
    test("1000000000000", "-1000000000000", "-100", "-99000000000000");
    test("0", "1000000000000", "100", "-100000000000000");
    test("4294967296", "1", "1", "4294967295");
    test("4294967296", "-1", "-1", "4294967295");
    test("3902609153", "88817093856604", "1", "-88813191247451");
    test("3902609153", "-88817093856604", "-1", "-88813191247451");

    test("-123", "0", "5", "-123");
    test("-123", "5", "1", "-128");
    test("-123", "5", "100", "-623");
    test("-10", "3", "4", "-22");
    test("-1000000000000", "0", "123", "-1000000000000");
    test("-1000000000000", "1", "123", "-1000000000123");
    test("-1000000000000", "123", "1", "-1000000000123");
    test("-1000000000000", "123", "100", "-1000000012300");
    test("-1000000000000", "100", "123", "-1000000012300");
    test("-1000000000000", "65536", "65536", "-1004294967296");
    test("-1000000000000", "1000000000000", "0", "-1000000000000");
    test("-1000000000000", "1000000000000", "1", "-2000000000000");
    test("-1000000000000", "1000000000000", "100", "-101000000000000");
    test(
        "-1000000000000",
        "65536",
        "1000000000000",
        "-65537000000000000",
    );
    test(
        "-1000000000000",
        "1000000000000",
        "1000000000000",
        "-1000000000001000000000000",
    );
    test(
        "0",
        "1000000000000",
        "1000000000000",
        "-1000000000000000000000000",
    );

    test("-123", "-5", "-1", "-128");
    test("-123", "-5", "-100", "-623");
    test("-10", "-3", "-4", "-22");
    test("-1000000000000", "-1", "-123", "-1000000000123");
    test("-1000000000000", "-123", "-1", "-1000000000123");
    test("-1000000000000", "-123", "-100", "-1000000012300");
    test("-1000000000000", "-100", "-123", "-1000000012300");
    test("-1000000000000", "-65536", "-65536", "-1004294967296");
    test("-1000000000000", "-1000000000000", "-1", "-2000000000000");
    test(
        "-1000000000000",
        "-1000000000000",
        "-100",
        "-101000000000000",
    );
    test(
        "-1000000000000",
        "-65536",
        "-1000000000000",
        "-65537000000000000",
    );
    test(
        "-1000000000000",
        "-1000000000000",
        "-1000000000000",
        "-1000000000001000000000000",
    );

    test("-123", "0", "-5", "-123");
    test("-123", "-5", "1", "-118");
    test("-123", "5", "-1", "-118");
    test("-123", "-5", "100", "377");
    test("-123", "5", "-100", "377");
    test("-10", "-3", "4", "2");
    test("-10", "3", "-4", "2");
    test("-15", "-3", "4", "-3");
    test("-15", "3", "-4", "-3");
    test("-1000000000000", "0", "-123", "-1000000000000");
    test("-1000000000000", "-1", "123", "-999999999877");
    test("-1000000000000", "1", "-123", "-999999999877");
    test("-1000000000000", "-123", "1", "-999999999877");
    test("-1000000000000", "123", "-1", "-999999999877");
    test("-1000000000000", "-123", "100", "-999999987700");
    test("-1000000000000", "123", "-100", "-999999987700");
    test("-1000000000000", "-100", "123", "-999999987700");
    test("-1000000000000", "100", "-123", "-999999987700");
    test("-1000000000000", "-65536", "65536", "-995705032704");
    test("-1000000000000", "65536", "-65536", "-995705032704");
    test("-1000000000000", "-1000000000000", "0", "-1000000000000");
    test("-1000000000000", "-1000000000000", "1", "0");
    test("-1000000000000", "1000000000000", "-1", "0");
    test("-1000000000000", "-1000000000000", "100", "99000000000000");
    test("-1000000000000", "1000000000000", "-100", "99000000000000");
    test("-4294967296", "-1", "1", "-4294967295");
    test("-4294967296", "1", "-1", "-4294967295");
    test("-3902609153", "-88817093856604", "1", "88813191247451");
    test("-3902609153", "88817093856604", "-1", "88813191247451");
    test(
        "1000000000000000000000000",
        "1000000000000",
        "1000000000000",
        "0",
    );
    test(
        "18446744073709551615",
        "340282366604025813406317257057592410112",
        "18446742974197923840",
        "-6277101355396255406250174417290596496345921599978512318465",
    );
}

#[test]
fn limbs_overflowing_sub_mul_limb_properties() {
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10().test_properties(|(a, b, c)| {
        let (result, sign) = limbs_overflowing_sub_mul_limb(&a, &b, c);
        let expected_result = Integer::from(Natural::from_owned_limbs_asc(a)).sub_mul(
            Integer::from(Natural::from_owned_limbs_asc(b)),
            Integer::from(c),
        );
        assert_eq!(sign, expected_result >= 0);
        assert_eq!(
            Natural::from_owned_limbs_asc(result),
            expected_result.unsigned_abs()
        );
    });
}

#[test]
fn limbs_overflowing_sub_mul_limb_in_place_left_properties() {
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10().test_properties(|(mut a, b, c)| {
        let a_old = a.clone();
        let sign = limbs_overflowing_sub_mul_limb_in_place_left(&mut a, &b, c);
        let expected_result = Integer::from(Natural::from_owned_limbs_asc(a_old)).sub_mul(
            Integer::from(Natural::from_owned_limbs_asc(b)),
            Integer::from(c),
        );
        assert_eq!(sign, expected_result >= 0);
        assert_eq!(
            Natural::from_owned_limbs_asc(a),
            expected_result.unsigned_abs(),
        );
    });
}

#[test]
fn limbs_overflowing_sub_mul_limb_in_place_right_properties() {
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10().test_properties(|(a, mut b, c)| {
        let b_old = b.clone();
        let sign = limbs_overflowing_sub_mul_limb_in_place_right(&a, &mut b, c);
        let expected_result = Integer::from(Natural::from_owned_limbs_asc(a)).sub_mul(
            Integer::from(Natural::from_owned_limbs_asc(b_old)),
            Integer::from(c),
        );
        assert_eq!(sign, expected_result >= 0);
        assert_eq!(
            Natural::from_owned_limbs_asc(b),
            expected_result.unsigned_abs()
        );
    });
}

#[test]
fn limbs_overflowing_sub_mul_limb_in_place_either_properties() {
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10().test_properties(|(mut a, mut b, c)| {
        let a_old = a.clone();
        let b_old = b.clone();
        let (right, sign) = limbs_overflowing_sub_mul_limb_in_place_either(&mut a, &mut b, c);
        let expected_result = Integer::from(Natural::from_limbs_asc(&a_old)).sub_mul(
            Integer::from(Natural::from_limbs_asc(&b_old)),
            Integer::from(c),
        );
        let result = Natural::from_owned_limbs_asc(if right {
            assert_eq!(a_old, a);
            b
        } else {
            assert_eq!(b_old, b);
            a
        });
        assert_eq!(sign, expected_result >= 0);
        assert_eq!(result, expected_result.unsigned_abs());
    });
}

#[test]
fn limbs_overflowing_sub_mul_properties() {
    unsigned_vec_triple_gen_var_39().test_properties(|(a, b, c)| {
        let (result, sign) = limbs_overflowing_sub_mul(&a, &b, &c);
        let expected_result = Integer::from(Natural::from_owned_limbs_asc(a)).sub_mul(
            Integer::from(Natural::from_owned_limbs_asc(b)),
            Integer::from(Natural::from_owned_limbs_asc(c)),
        );
        if expected_result != 0 {
            assert_eq!(sign, expected_result >= 0);
        }
        assert_eq!(
            Natural::from_owned_limbs_asc(result),
            expected_result.unsigned_abs()
        );
    });
}

#[test]
fn limbs_overflowing_sub_mul_in_place_left_properties() {
    unsigned_vec_triple_gen_var_39().test_properties(|(mut a, b, c)| {
        let a_old = a.clone();
        let sign = limbs_overflowing_sub_mul_in_place_left(&mut a, &b, &c);
        let expected_result = Integer::from(Natural::from_owned_limbs_asc(a_old)).sub_mul(
            Integer::from(Natural::from_owned_limbs_asc(b)),
            Integer::from(Natural::from_owned_limbs_asc(c)),
        );
        if expected_result != 0 {
            assert_eq!(sign, expected_result >= 0);
        }
        assert_eq!(
            Natural::from_owned_limbs_asc(a),
            expected_result.unsigned_abs()
        );
    });
}

#[test]
fn sub_mul_properties() {
    integer_triple_gen().test_properties(|(a, b, c)| {
        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(b.clone(), c.clone());
        assert!(mut_a.is_valid());
        let result = mut_a;

        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(b.clone(), &c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(&b, c.clone());
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(&b, &c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let result_alt = a.clone().sub_mul(b.clone(), c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result = a.clone().sub_mul(b.clone(), &c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result = a.clone().sub_mul(&b, c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result = a.clone().sub_mul(&b, &c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result = (&a).sub_mul(&b, &c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!(&a - &b * &c, result);
        assert_eq!((&a).sub_mul(&c, &b), result);
        assert_eq!((&a).sub_mul(&(-&b), &(-&c)), result);
        assert_eq!((-&a).sub_mul(&(-&b), &c), -&result);
        assert_eq!((-a).sub_mul(b, -c), -result);
    });

    integer_gen().test_properties(|a| {
        assert_eq!((&a).sub_mul(&a, &Integer::ONE), 0);
        assert_eq!((&a).sub_mul(&(-&a), &Integer::NEGATIVE_ONE), 0);
    });

    integer_pair_gen().test_properties(|(a, b)| {
        assert_eq!((&a).sub_mul(&Integer::ZERO, &b), a);
        assert_eq!((&a).sub_mul(&Integer::ONE, &b), &a - &b);
        assert_eq!(Integer::ZERO.sub_mul(&a, &b), -&a * &b);
        assert_eq!((&a).sub_mul(&b, &Integer::ZERO), a);
        assert_eq!((&a).sub_mul(&b, &Integer::ONE), &a - &b);
        assert_eq!((&a * &b).sub_mul(&a, &b), 0);
        assert_eq!((&a * &b).sub_mul(-a, -b), 0);
    });

    signed_triple_gen_var_2::<SignedLimb>().test_properties(|(x, y, z)| {
        assert_eq!(
            x.sub_mul(y, z),
            Integer::from(x).sub_mul(Integer::from(y), Integer::from(z))
        );
    });

    signed_triple_gen::<SignedLimb>().test_properties(|(x, y, z)| {
        let result = Integer::from(x).sub_mul(Integer::from(y), Integer::from(z));
        assert_eq!(
            x.checked_sub_mul(y, z).is_some(),
            SignedLimb::convertible_from(&result)
        );
    });
}
