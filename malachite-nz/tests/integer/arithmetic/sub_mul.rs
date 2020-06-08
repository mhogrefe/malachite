use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{SubMul, SubMulAssign};

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::integer::arithmetic::sub_mul::{
    limbs_overflowing_sub_mul, limbs_overflowing_sub_mul_in_place_left,
    limbs_overflowing_sub_mul_limb, limbs_overflowing_sub_mul_limb_in_place_either,
    limbs_overflowing_sub_mul_limb_in_place_left, limbs_overflowing_sub_mul_limb_in_place_right,
};
use malachite_nz::integer::Integer;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
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
    test(&[123, 456], &[123], 4, &[4_294_966_927, 455, 0], true);
    test(&[123, 456], &[123], u32::MAX, &[246, 333, 0], true);
    test(&[123], &[123], 1, &[0, 0], true);
    test(&[123], &[123], 4, &[369, 0], false);
    test(
        &[123],
        &[123, 456],
        u32::MAX,
        &[4_294_967_050, 4_294_966_962, 455],
        false,
    );
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
        &[4_294_966_927, 455, 0],
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
        &[4_294_967_050, 4_294_966_962, 455],
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
        &[39_360, 333_255, 516_006],
        false,
    );
    test(
        &[123, 456, 789, 987, 654],
        &[123, 789],
        &[321, 654],
        &[4_294_927_936, 4_294_634_040, 4_294_452_078, 986, 654],
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
    let test = |i, j, k, out| {
        let mut a = Integer::from_str(i).unwrap();
        a.sub_mul_assign(Integer::from_str(j).unwrap(), Integer::from_str(k).unwrap());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Integer::from_str(i).unwrap();
        a.sub_mul_assign(
            Integer::from_str(j).unwrap(),
            &Integer::from_str(k).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Integer::from_str(i).unwrap();
        a.sub_mul_assign(
            &Integer::from_str(j).unwrap(),
            Integer::from_str(k).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Integer::from_str(i).unwrap();
        a.sub_mul_assign(
            &Integer::from_str(j).unwrap(),
            &Integer::from_str(k).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(i)
            .unwrap()
            .sub_mul(Integer::from_str(j).unwrap(), Integer::from_str(k).unwrap());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(i).unwrap().sub_mul(
            Integer::from_str(j).unwrap(),
            &Integer::from_str(k).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(i).unwrap().sub_mul(
            &Integer::from_str(j).unwrap(),
            Integer::from_str(k).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(i).unwrap().sub_mul(
            &Integer::from_str(j).unwrap(),
            &Integer::from_str(k).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&Integer::from_str(i).unwrap()).sub_mul(
            &Integer::from_str(j).unwrap(),
            &Integer::from_str(k).unwrap(),
        );
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
}
