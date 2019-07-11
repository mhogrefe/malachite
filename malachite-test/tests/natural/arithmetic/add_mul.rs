use common::test_properties;
use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::arithmetic::add_mul::{limbs_add_mul, limbs_add_mul_in_place_left};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;
use malachite_test::inputs::base::triples_of_unsigned_vec_var_27;
use malachite_test::inputs::natural::{pairs_of_naturals, triples_of_naturals};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_add_mul_and_limbs_add_mul_in_place_left() {
    let test = |xs_before: &[Limb], ys: &[Limb], zs: &[Limb], result: &[Limb]| {
        assert_eq!(limbs_add_mul(xs_before, ys, zs), result);
        let mut xs = xs_before.to_vec();
        limbs_add_mul_in_place_left(&mut xs, ys, zs);
        assert_eq!(xs, result);
    };
    test(
        &[123, 456],
        &[123, 789],
        &[321, 654],
        &[39606, 334167, 516006],
    );
    test(
        &[123, 456, 789, 987, 654],
        &[123, 789],
        &[321, 654],
        &[39606, 334167, 516795, 987, 654],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_mul_fail_1() {
    limbs_add_mul(&[10, 10], &[], &[10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_mul_fail_2() {
    limbs_add_mul(&[10, 10], &[10, 10], &[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_mul_in_place_left_fail_1() {
    let mut xs = vec![10, 10];
    limbs_add_mul_in_place_left(&mut xs, &[], &[10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_mul_in_place_left_fail_2() {
    let mut xs = vec![10, 10];
    limbs_add_mul_in_place_left(&mut xs, &[10, 10], &[]);
}

#[test]
fn test_add_mul() {
    let test = |u, v, w, out| {
        let mut a = Natural::from_str(u).unwrap();
        a.add_mul_assign(Natural::from_str(v).unwrap(), Natural::from_str(w).unwrap());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Natural::from_str(u).unwrap();
        a.add_mul_assign(
            Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Natural::from_str(u).unwrap();
        a.add_mul_assign(
            &Natural::from_str(v).unwrap(),
            Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Natural::from_str(u).unwrap();
        a.add_mul_assign(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Natural::from_str(u)
            .unwrap()
            .add_mul(Natural::from_str(v).unwrap(), Natural::from_str(w).unwrap());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Natural::from_str(u).unwrap().add_mul(
            Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Natural::from_str(u).unwrap().add_mul(
            &Natural::from_str(v).unwrap(),
            Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Natural::from_str(u).unwrap().add_mul(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&Natural::from_str(u).unwrap()).add_mul(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
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
    test(
        "18446744073583722494",
        "2",
        "4033876984",
        "18446744081651476462",
    );
}

#[test]
fn limbs_add_mul_properties() {
    test_properties(triples_of_unsigned_vec_var_27, |&(ref a, ref b, ref c)| {
        assert_eq!(
            limbs_add_mul(a, b, c),
            Natural::from_limbs_asc(a)
                .add_mul(Natural::from_limbs_asc(b), Natural::from_limbs_asc(c))
                .into_limbs_asc()
        );
    });
}

#[test]
fn limbs_add_mul_in_place_left_properties() {
    test_properties(triples_of_unsigned_vec_var_27, |&(ref a, ref b, ref c)| {
        let a_old = a;
        let mut a = a.to_vec();
        limbs_add_mul_in_place_left(&mut a, b, c);
        assert_eq!(
            a,
            Natural::from_limbs_asc(a_old)
                .add_mul(Natural::from_limbs_asc(b), Natural::from_limbs_asc(c))
                .into_limbs_asc()
        );
    });
}

#[test]
fn add_mul_limb_properties() {
    test_properties(triples_of_naturals, |&(ref a, ref b, ref c)| {
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
        assert_eq!(mut_a, result);

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
    });

    test_properties(pairs_of_naturals, |&(ref a, ref b)| {
        assert_eq!(a.add_mul(&Natural::ZERO, b), *a);
        assert_eq!(a.add_mul(&Natural::ONE, b), a + b);
        assert_eq!(Natural::ZERO.add_mul(a, b), a * b);
        assert_eq!(a.add_mul(b, &Natural::ZERO), *a);
        assert_eq!(a.add_mul(b, &Natural::ONE), a + b);
    });
}
