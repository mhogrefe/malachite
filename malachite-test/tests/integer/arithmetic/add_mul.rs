use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign, SubMul, UnsignedAbs};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_nz::integer::arithmetic::add_mul::{
    limbs_overflowing_sub_mul, limbs_overflowing_sub_mul_in_place_left,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use common::test_properties;
use malachite_test::inputs::base::triples_of_unsigned_vec_var_29;
use malachite_test::inputs::integer::{integers, pairs_of_integers, triples_of_integers};

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
fn test_add_mul() {
    let test = |u, v, w, out| {
        let mut a = Integer::from_str(u).unwrap();
        a.add_mul_assign(Integer::from_str(v).unwrap(), Integer::from_str(w).unwrap());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Integer::from_str(u).unwrap();
        a.add_mul_assign(
            Integer::from_str(v).unwrap(),
            &Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Integer::from_str(u).unwrap();
        a.add_mul_assign(
            &Integer::from_str(v).unwrap(),
            Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Integer::from_str(u).unwrap();
        a.add_mul_assign(
            &Integer::from_str(v).unwrap(),
            &Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(u)
            .unwrap()
            .add_mul(Integer::from_str(v).unwrap(), Integer::from_str(w).unwrap());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(u).unwrap().add_mul(
            Integer::from_str(v).unwrap(),
            &Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(u).unwrap().add_mul(
            &Integer::from_str(v).unwrap(),
            Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(u).unwrap().add_mul(
            &Integer::from_str(v).unwrap(),
            &Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&Integer::from_str(u).unwrap()).add_mul(
            &Integer::from_str(v).unwrap(),
            &Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());
    };
    test("0", "0", "0", "0");
    test("0", "0", "123", "0");
    test("123", "0", "5", "123");
    test("123", "5", "1", "128");
    test("123", "5", "100", "623");
    test("10", "3", "4", "22");
    test("1000000000000", "0", "123", "1000000000000");
    test("1000000000000", "1", "123", "1000000000123");
    test("1000000000000", "123", "1", "1000000000123");
    test("1000000000000", "123", "100", "1000000012300");
    test("1000000000000", "100", "123", "1000000012300");
    test("1000000000000", "65536", "65536", "1004294967296");
    test("1000000000000", "1000000000000", "0", "1000000000000");
    test("1000000000000", "1000000000000", "1", "2000000000000");
    test("1000000000000", "1000000000000", "100", "101000000000000");
    test("0", "1000000000000", "100", "100000000000000");
    test(
        "1000000000000",
        "65536",
        "1000000000000",
        "65537000000000000",
    );
    test(
        "1000000000000",
        "1000000000000",
        "1000000000000",
        "1000000000001000000000000",
    );
    test(
        "0",
        "1000000000000",
        "1000000000000",
        "1000000000000000000000000",
    );

    test("123", "-5", "-1", "128");
    test("123", "-5", "-100", "623");
    test("10", "-3", "-4", "22");
    test("1000000000000", "-1", "-123", "1000000000123");
    test("1000000000000", "-123", "-1", "1000000000123");
    test("1000000000000", "-123", "-100", "1000000012300");
    test("1000000000000", "-100", "-123", "1000000012300");
    test("1000000000000", "-65536", "-65536", "1004294967296");
    test("1000000000000", "-1000000000000", "-1", "2000000000000");
    test("1000000000000", "-1000000000000", "-100", "101000000000000");
    test("0", "-1000000000000", "-100", "100000000000000");
    test(
        "1000000000000",
        "-65536",
        "-1000000000000",
        "65537000000000000",
    );
    test(
        "1000000000000",
        "-1000000000000",
        "-1000000000000",
        "1000000000001000000000000",
    );
    test(
        "0",
        "-1000000000000",
        "-1000000000000",
        "1000000000000000000000000",
    );

    test("0", "0", "-123", "0");
    test("123", "0", "-5", "123");
    test("123", "-5", "1", "118");
    test("123", "5", "-1", "118");
    test("123", "-5", "100", "-377");
    test("123", "5", "-100", "-377");
    test("10", "-3", "4", "-2");
    test("10", "3", "-4", "-2");
    test("15", "-3", "4", "3");
    test("15", "3", "-4", "3");
    test("1000000000000", "0", "-123", "1000000000000");
    test("1000000000000", "-1", "123", "999999999877");
    test("1000000000000", "1", "-123", "999999999877");
    test("1000000000000", "-123", "1", "999999999877");
    test("1000000000000", "123", "-1", "999999999877");
    test("1000000000000", "-123", "100", "999999987700");
    test("1000000000000", "123", "-100", "999999987700");
    test("1000000000000", "-100", "123", "999999987700");
    test("1000000000000", "100", "-123", "999999987700");
    test("1000000000000", "-65536", "65536", "995705032704");
    test("1000000000000", "65536", "-65536", "995705032704");
    test("1000000000000", "-1000000000000", "0", "1000000000000");
    test("1000000000000", "-1000000000000", "1", "0");
    test("1000000000000", "1000000000000", "-1", "0");
    test("1000000000000", "-1000000000000", "100", "-99000000000000");
    test("1000000000000", "1000000000000", "-100", "-99000000000000");
    test("0", "-1000000000000", "100", "-100000000000000");
    test("4294967296", "-1", "1", "4294967295");
    test("4294967296", "1", "-1", "4294967295");
    test("3902609153", "-88817093856604", "1", "-88813191247451");
    test("3902609153", "88817093856604", "-1", "-88813191247451");

    test("-123", "0", "5", "-123");
    test("-123", "-5", "1", "-128");
    test("-123", "-5", "100", "-623");
    test("-10", "-3", "4", "-22");
    test("-1000000000000", "0", "123", "-1000000000000");
    test("-1000000000000", "-1", "123", "-1000000000123");
    test("-1000000000000", "-123", "1", "-1000000000123");
    test("-1000000000000", "-123", "100", "-1000000012300");
    test("-1000000000000", "-100", "123", "-1000000012300");
    test("-1000000000000", "-65536", "65536", "-1004294967296");
    test("-1000000000000", "-1000000000000", "0", "-1000000000000");
    test("-1000000000000", "-1000000000000", "1", "-2000000000000");
    test(
        "-1000000000000",
        "-1000000000000",
        "100",
        "-101000000000000",
    );
    test(
        "-1000000000000",
        "-65536",
        "1000000000000",
        "-65537000000000000",
    );
    test(
        "-1000000000000",
        "-1000000000000",
        "1000000000000",
        "-1000000000001000000000000",
    );
    test(
        "0",
        "-1000000000000",
        "1000000000000",
        "-1000000000000000000000000",
    );

    test("-123", "5", "-1", "-128");
    test("-123", "5", "-100", "-623");
    test("-10", "3", "-4", "-22");
    test("-1000000000000", "1", "-123", "-1000000000123");
    test("-1000000000000", "123", "-1", "-1000000000123");
    test("-1000000000000", "123", "-100", "-1000000012300");
    test("-1000000000000", "100", "-123", "-1000000012300");
    test("-1000000000000", "65536", "-65536", "-1004294967296");
    test("-1000000000000", "1000000000000", "-1", "-2000000000000");
    test(
        "-1000000000000",
        "1000000000000",
        "-100",
        "-101000000000000",
    );
    test(
        "-1000000000000",
        "65536",
        "-1000000000000",
        "-65537000000000000",
    );
    test(
        "-1000000000000",
        "1000000000000",
        "-1000000000000",
        "-1000000000001000000000000",
    );

    test("-123", "0", "-5", "-123");
    test("-123", "5", "1", "-118");
    test("-123", "-5", "-1", "-118");
    test("-123", "5", "100", "377");
    test("-123", "-5", "-100", "377");
    test("-10", "3", "4", "2");
    test("-10", "-3", "-4", "2");
    test("-15", "3", "4", "-3");
    test("-15", "-3", "-4", "-3");
    test("-1000000000000", "0", "-123", "-1000000000000");
    test("-1000000000000", "1", "123", "-999999999877");
    test("-1000000000000", "-1", "-123", "-999999999877");
    test("-1000000000000", "123", "1", "-999999999877");
    test("-1000000000000", "-123", "-1", "-999999999877");
    test("-1000000000000", "123", "100", "-999999987700");
    test("-1000000000000", "-123", "-100", "-999999987700");
    test("-1000000000000", "100", "123", "-999999987700");
    test("-1000000000000", "-100", "-123", "-999999987700");
    test("-1000000000000", "65536", "65536", "-995705032704");
    test("-1000000000000", "-65536", "-65536", "-995705032704");
    test("-1000000000000", "1000000000000", "0", "-1000000000000");
    test("-1000000000000", "1000000000000", "1", "0");
    test("-1000000000000", "-1000000000000", "-1", "0");
    test("-1000000000000", "1000000000000", "100", "99000000000000");
    test("-1000000000000", "-1000000000000", "-100", "99000000000000");
    test("-4294967296", "1", "1", "-4294967295");
    test("-4294967296", "-1", "-1", "-4294967295");
    test("-3902609153", "88817093856604", "1", "88813191247451");
    test("-3902609153", "-88817093856604", "-1", "88813191247451");
    test(
        "1000000000000000000000000",
        "-1000000000000",
        "1000000000000",
        "0",
    );
    test(
        "-4",
        "-24227802588",
        "-14313318194700",
        "346780247600420147883596",
    );
}

#[test]
fn limbs_overflowing_sub_mul_properties() {
    test_properties(triples_of_unsigned_vec_var_29, |&(ref a, ref b, ref c)| {
        let (result_limbs, sign) = limbs_overflowing_sub_mul(a, b, c);
        let expected_result = Integer::from(Natural::from_limbs_asc(a)).sub_mul(
            Integer::from(Natural::from_limbs_asc(b)),
            Integer::from(Natural::from_limbs_asc(c)),
        );
        if expected_result != 0 {
            assert_eq!(sign, expected_result >= 0);
        }
        assert_eq!(
            Natural::from_owned_limbs_asc(result_limbs),
            expected_result.unsigned_abs()
        );
    });
}

#[test]
fn limbs_overflowing_sub_mul_in_place_left_properties() {
    test_properties(triples_of_unsigned_vec_var_29, |&(ref a, ref b, ref c)| {
        let a_old = a;
        let mut a = a_old.to_vec();
        let sign = limbs_overflowing_sub_mul_in_place_left(&mut a, b, c);
        let expected_result = Integer::from(Natural::from_limbs_asc(a_old)).sub_mul(
            Integer::from(Natural::from_limbs_asc(b)),
            Integer::from(Natural::from_limbs_asc(c)),
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
fn add_mul_properties() {
    test_properties(triples_of_integers, |&(ref a, ref b, ref c)| {
        let mut mut_a = a.clone();
        mut_a.add_mul_assign(b.clone(), c.clone());
        assert!(mut_a.is_valid());
        let result = mut_a;

        let mut mut_a = a.clone();
        mut_a.add_mul_assign(b.clone(), c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.add_mul_assign(b, c.clone());
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.add_mul_assign(b, c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result, "{} {} {}", a, b, c);

        let result_alt = a.clone().add_mul(b.clone(), c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().add_mul(b.clone(), c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().add_mul(b, c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().add_mul(b, c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.add_mul(b, c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!(a + b * c, result);
        assert_eq!(a.add_mul(c, b), result);
        assert_eq!(a.add_mul(&(-b), &(-c)), result);
        assert_eq!((-a).add_mul(&(-b), c), -&result);
        assert_eq!((-a).add_mul(b, -c), -result);
    });

    test_properties(integers, |a| {
        assert_eq!(a.add_mul(a, &Integer::NEGATIVE_ONE), 0 as Limb);
        assert_eq!(a.add_mul(&(-a), &Integer::ONE), 0 as Limb);
    });

    test_properties(pairs_of_integers, |&(ref a, ref b)| {
        assert_eq!(a.add_mul(&Integer::ZERO, b), *a);
        assert_eq!(a.add_mul(&Integer::ONE, b), a + b);
        assert_eq!(Integer::ZERO.add_mul(a, b), a * b);
        assert_eq!(a.add_mul(b, &Integer::ZERO), *a);
        assert_eq!(a.add_mul(b, &Integer::ONE), a + b);
        assert_eq!((a * b).add_mul(-a, b), 0 as Limb);
        assert_eq!((a * b).add_mul(a, -b), 0 as Limb);
    });
}
