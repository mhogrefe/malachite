use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign, SubMul, UnsignedAbs};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_nz::integer::arithmetic::add_mul_limb::{
    limbs_overflowing_sub_mul_limb, limbs_overflowing_sub_mul_limb_in_place_either,
    limbs_overflowing_sub_mul_limb_in_place_left, limbs_overflowing_sub_mul_limb_in_place_right,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use common::test_properties;
use malachite_test::inputs::base::triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3;
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_unsigned, pairs_of_integers,
    triples_of_integer_integer_and_unsigned,
};

#[test]
fn test_add_mul_limb() {
    let test = |u, v, c: Limb, out| {
        let mut a = Integer::from_str(u).unwrap();
        a.add_mul_assign(Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Integer::from_str(u).unwrap();
        a.add_mul_assign(&Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(u)
            .unwrap()
            .add_mul(Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(u)
            .unwrap()
            .add_mul(&Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&Integer::from_str(u).unwrap()).add_mul(Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&Integer::from_str(u).unwrap()).add_mul(&Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());
    };
    test("0", "0", 0, "0");
    test("0", "0", 123, "0");
    test("123", "0", 5, "123");
    test("-123", "0", 5, "-123");
    test("123", "5", 1, "128");
    test("-123", "5", 1, "-118");
    test("123", "5", 100, "623");
    test("-123", "5", 100, "377");
    test("10", "3", 4, "22");
    test("10", "-3", 4, "-2");
    test("1000000000000", "0", 123, "1000000000000");
    test("1000000000000", "1", 123, "1000000000123");
    test("1000000000000", "123", 1, "1000000000123");
    test("1000000000000", "123", 100, "1000000012300");
    test("1000000000000", "100", 123, "1000000012300");
    test("1000000000000", "65536", 0x1_0000, "1004294967296");
    test("-1000000000000", "-65536", 0x1_0000, "-1004294967296");
    test("-1000000000000", "65536", 0x1_0000, "-995705032704");
    test("1000000000000", "-65536", 0x1_0000, "995705032704");
    test("1000000000000", "1000000000000", 0, "1000000000000");
    test("1000000000000", "1000000000000", 1, "2000000000000");
    test("1000000000000", "1000000000000", 100, "101000000000000");
    test("0", "1000000000000", 100, "100000000000000");
    test("-1", "1000000000000", 100, "99999999999999");
    test("0", "-1000000000000", 100, "-100000000000000");
    test("1", "-1000000000000", 100, "-99999999999999");
    test("1000000000000", "-1000000000000", 1, "0");
    test("-1000000000000", "1000000000000", 1, "0");
    test(
        "1000000000000000000000",
        "-1000000000000",
        1_000_000_000,
        "0",
    );
    test(
        "-1000000000000000000000",
        "1000000000000",
        1_000_000_000,
        "0",
    );
    test(
        "-7671776751",
        "2017407396563853588311730593327576",
        2553057608,
        "5150557302232819461437363666941314271221457",
    );
    test(
        "988103394839786448188181025971304835384864628299991531106503795701781321749236125121128104\
        4599593380367356508124575858077230634769745899974633112009488503450713453445973560100084776\
        97",
        "-24539813271159590488815484507418119636298188543635862241473443846717146553538708695804830\
        4177952083700616439618000388250762682860351639048865404251686367806613013195724423167",
        4026531840,
        "-57586096570152913699974130752738755704962105930805378968462627240302136570569752978141470\
        936858414616334783344053558228980554892937683322206914846327185737550659583"
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_overflowing_sub_mul_limb() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], limb: Limb, result: &[Limb], sign: bool| {
        let (result_alt, sign_alt) = limbs_overflowing_sub_mul_limb(xs_before, ys_before, limb);
        assert_eq!(&result_alt, &result);
        assert_eq!(sign_alt, sign);
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_overflowing_sub_mul_limb_in_place_left(&mut xs, ys_before, limb),
            sign
        );
        assert_eq!(xs, result);
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_overflowing_sub_mul_limb_in_place_right(xs_before, &mut ys, limb),
            sign
        );
        assert_eq!(ys, result);
    };
    test(&[123, 456], &[123], 4, &[4294966927, 455, 0], true);
    test(&[123, 456], &[123], 0xffff_ffff, &[246, 333, 0], true);
    test(&[123], &[123], 1, &[0, 0], true);
    test(&[123], &[123], 4, &[369, 0], false);
    test(
        &[123],
        &[123, 456],
        0xffff_ffff,
        &[4294967050, 4294966962, 455],
        false,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_overflowing_sub_mul_limb_in_place_either() {
    let test = |xs_before: &[Limb],
                ys_before: &[Limb],
                limb: Limb,
                right: bool,
                xs_after: &[Limb],
                ys_after: &[Limb],
                sign| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_overflowing_sub_mul_limb_in_place_either(&mut xs, &mut ys, limb),
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
        0xffff_ffff,
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
        0xffff_ffff,
        true,
        &[123],
        &[4294967050, 4294966962, 455],
        false,
    );
}

#[test]
fn limbs_overflowing_sub_mul_limb_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3,
        |&(ref a, ref b, c)| {
            let (result_limbs, sign) = limbs_overflowing_sub_mul_limb(a, b, c);
            let expected_result = Integer::from(Natural::from_limbs_asc(a))
                .sub_mul(Integer::from(Natural::from_limbs_asc(b)), c);
            assert_eq!(sign, expected_result >= 0);
            assert_eq!(
                Natural::from_owned_limbs_asc(result_limbs),
                expected_result.unsigned_abs()
            );
        },
    );
}

#[test]
fn limbs_overflowing_sub_mul_limb_in_place_left_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3,
        |&(ref a, ref b, c)| {
            let a_old = a;
            let mut a = a_old.to_vec();
            let sign = limbs_overflowing_sub_mul_limb_in_place_left(&mut a, b, c);
            let expected_result = Integer::from(Natural::from_limbs_asc(a_old))
                .sub_mul(Integer::from(Natural::from_limbs_asc(b)), c);
            assert_eq!(sign, expected_result >= 0);
            assert_eq!(
                Natural::from_owned_limbs_asc(a),
                expected_result.unsigned_abs()
            );
        },
    );
}

#[test]
fn limbs_overflowing_sub_mul_limb_in_place_right_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3,
        |&(ref a, ref b, c)| {
            let b_old = b;
            let mut b = b_old.to_vec();
            let sign = limbs_overflowing_sub_mul_limb_in_place_right(a, &mut b, c);
            let expected_result = Integer::from(Natural::from_limbs_asc(a))
                .sub_mul(Integer::from(Natural::from_limbs_asc(b_old)), c);
            assert_eq!(sign, expected_result >= 0);
            assert_eq!(
                Natural::from_owned_limbs_asc(b),
                expected_result.unsigned_abs()
            );
        },
    );
}

#[test]
fn limbs_overflowing_sub_mul_limb_in_place_either_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3,
        |&(ref a, ref b, c)| {
            let a_old = a;
            let b_old = b;
            let mut a = a_old.to_vec();
            let mut b = b_old.to_vec();
            let (right, sign) = limbs_overflowing_sub_mul_limb_in_place_either(&mut a, &mut b, c);
            let expected_result = Integer::from(Natural::from_limbs_asc(a_old))
                .sub_mul(Integer::from(Natural::from_limbs_asc(b_old)), c);
            let result = Natural::from_owned_limbs_asc(if right {
                assert_eq!(a_old, &a);
                b
            } else {
                assert_eq!(b_old, &b);
                a
            });
            assert_eq!(sign, expected_result >= 0);
            assert_eq!(result, expected_result.unsigned_abs());
        },
    );
}

#[test]
fn add_mul_limb_properties() {
    test_properties(
        triples_of_integer_integer_and_unsigned,
        |&(ref a, ref b, c): &(Integer, Integer, Limb)| {
            let mut mut_a = a.clone();
            mut_a.add_mul_assign(b.clone(), c);
            assert!(mut_a.is_valid());
            let result = mut_a;

            let mut mut_a = a.clone();
            mut_a.add_mul_assign(b, c);
            assert!(mut_a.is_valid());
            assert_eq!(mut_a, result, "{} {} {}", a, b, c);

            let result_alt = a.add_mul(b.clone(), c);
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = a.add_mul(b, c);
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = a.clone().add_mul(b.clone(), c);
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = a.add_mul(b, c);
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            assert_eq!(a + b * c, result);
            assert_eq!((-a).add_mul(-b, c), -&result);
            assert_eq!(a.add_mul(b, &Integer::from(c)), result);
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n.add_mul(-n, 1 as Limb), 0 as Limb);
    });

    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref n, c): &(Integer, Limb)| {
            assert_eq!(n.add_mul(&Integer::ZERO, c), *n);
            assert_eq!(n.add_mul(&Integer::ONE, c), n + c);
            assert_eq!(n.add_mul(&Integer::NEGATIVE_ONE, c), n - c);
            assert_eq!(Integer::ZERO.add_mul(n, c), n * c);
            assert_eq!((n * c).add_mul(-n, c), 0 as Limb);
        },
    );

    test_properties(pairs_of_integers, |&(ref a, ref b)| {
        assert_eq!(a.add_mul(b, 0 as Limb), *a);
        assert_eq!(a.add_mul(b, 1 as Limb), a + b);
    });
}
