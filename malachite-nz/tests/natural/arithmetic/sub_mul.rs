use malachite_base::num::arithmetic::traits::{SubMul, SubMulAssign};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::sub_mul::{
    limbs_sub_mul, limbs_sub_mul_in_place_left, limbs_sub_mul_limb_greater,
    limbs_sub_mul_limb_greater_in_place_left, limbs_sub_mul_limb_greater_in_place_right,
    limbs_sub_mul_limb_same_length_in_place_left, limbs_sub_mul_limb_same_length_in_place_right,
};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_mul_limb_greater() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], limb: Limb, result: &[Limb], borrow| {
        let o_result = limbs_sub_mul_limb_greater(xs_before, ys_before, limb);
        if borrow == 0 {
            assert_eq!(o_result.unwrap(), result);
        } else {
            assert!(o_result.is_none());
        }
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_sub_mul_limb_greater_in_place_left(&mut xs, ys_before, limb),
            borrow
        );
        assert_eq!(xs, result);
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_sub_mul_limb_greater_in_place_right(xs_before, &mut ys, limb),
            borrow
        );
        assert_eq!(ys, result);
    };
    test(&[], &[], 4, &[], 0);
    test(&[123, 456], &[], 4, &[123, 456], 0);
    test(&[123, 456], &[123], 0, &[123, 456], 0);
    test(&[123, 456], &[123], 4, &[4294966927, 455], 0);
    test(&[123, 456], &[123], u32::MAX, &[246, 333], 0);
    test(&[123, 456], &[0, 123], u32::MAX, &[123, 579], 123);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_limb_greater_fail() {
    limbs_sub_mul_limb_greater(&[10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_limb_greater_in_place_left_fail() {
    limbs_sub_mul_limb_greater_in_place_left(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_limb_greater_in_place_right_fail() {
    limbs_sub_mul_limb_greater_in_place_right(&[10], &mut vec![10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_mul_limb_same_length() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], limb: Limb, result: &[Limb], borrow| {
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_sub_mul_limb_same_length_in_place_left(&mut xs, ys_before, limb),
            borrow
        );
        assert_eq!(xs, result);
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_sub_mul_limb_same_length_in_place_right(xs_before, &mut ys, limb),
            borrow
        );
        assert_eq!(ys, result);
    };
    test(&[], &[], 4, &[], 0);
    test(&[123, 456], &[0, 0], 4, &[123, 456], 0);
    test(&[123, 456], &[123, 0], 0, &[123, 456], 0);
    test(&[123, 456], &[123, 0], 4, &[4294966927, 455], 0);
    test(&[123, 456], &[123, 0], u32::MAX, &[246, 333], 0);
    test(&[123, 456], &[0, 123], u32::MAX, &[123, 579], 123);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_limb_same_length_in_place_left_fail() {
    limbs_sub_mul_limb_same_length_in_place_left(&mut [10, 10], &[10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_limb_same_length_in_place_right_fail() {
    limbs_sub_mul_limb_same_length_in_place_right(&[10, 10], &mut [10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_mul_and_limbs_sub_mul_in_place_left() {
    let test = |xs_before: &[Limb], ys: &[Limb], zs: &[Limb], result: Option<Vec<Limb>>| {
        assert_eq!(limbs_sub_mul(xs_before, ys, zs), result);
        let mut xs = xs_before.to_vec();
        let result_alt = if limbs_sub_mul_in_place_left(&mut xs, ys, zs) {
            None
        } else {
            Some(xs)
        };
        assert_eq!(result, result_alt);
    };
    test(&[123, 456, 789], &[123, 789], &[321, 654], None);
    test(
        &[123, 456, 789, 1],
        &[123, 789],
        &[321, 654],
        Some(vec![4294927936, 4294634040, 4294452078, 0]),
    );
    test(
        &[123, 456, 789, 987, 654],
        &[123, 789],
        &[321, 654],
        Some(vec![4294927936, 4294634040, 4294452078, 986, 654]),
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_fail_1() {
    limbs_sub_mul(&[10, 10, 10], &[10], &[10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_fail_2() {
    limbs_sub_mul(&[10, 10, 10], &[10, 10], &[10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_fail_3() {
    limbs_sub_mul(&[10, 10], &[10, 10], &[10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_in_place_left_fail_1() {
    let xs = &mut [10, 10, 10];
    limbs_sub_mul_in_place_left(xs, &[10], &[10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_in_place_left_fail_2() {
    let xs = &mut [10, 10, 10];
    limbs_sub_mul_in_place_left(xs, &[10, 10], &[10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_in_place_left_fail_3() {
    let xs = &mut [10, 10];
    limbs_sub_mul_in_place_left(xs, &[10, 10], &[10, 10]);
}

#[test]
fn test_sub_mul() {
    let test = |u, v, w, out: &str| {
        let mut n = Natural::from_str(u).unwrap();
        n.sub_mul_assign(Natural::from_str(v).unwrap(), Natural::from_str(w).unwrap());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = Natural::from_str(u).unwrap();
        n.sub_mul_assign(
            Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = Natural::from_str(u).unwrap();
        n.sub_mul_assign(
            &Natural::from_str(v).unwrap(),
            Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = Natural::from_str(u).unwrap();
        n.sub_mul_assign(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u)
            .unwrap()
            .sub_mul(Natural::from_str(v).unwrap(), Natural::from_str(w).unwrap());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap().sub_mul(
            Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap().sub_mul(
            &Natural::from_str(v).unwrap(),
            Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap().sub_mul(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = (&Natural::from_str(u).unwrap()).sub_mul(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0", "0");
    test("0", "0", "123", "0");
    test("123", "0", "5", "123");
    test("123", "5", "1", "118");
    test("15", "3", "4", "3");
    test("1000000000000", "0", "123", "1000000000000");
    test("1000000000000", "1", "123", "999999999877");
    test("1000000000000", "123", "1", "999999999877");
    test("1000000000000", "123", "100", "999999987700");
    test("1000000000000", "100", "123", "999999987700");
    test("1000000000000", "65536", "65536", "995705032704");
    test("1000000000000", "1000000000000", "0", "1000000000000");
    test("1000000000000", "1000000000000", "1", "0");
    test("4294967296", "1", "1", "4294967295");
    test(
        "1000000000000000000000000",
        "1000000000000",
        "1000000000000",
        "0",
    );
    test(
        "1000000000001000000000000",
        "1000000000000",
        "1000000000000",
        "1000000000000",
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_fail_1() {
    let mut x = Natural::from_str("123").unwrap();
    x.sub_mul_assign(
        Natural::from_str("5").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_fail_2() {
    let mut x = Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(
        Natural::from_str("1000000000000").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_val_ref_fail_1() {
    let mut x = Natural::from_str("123").unwrap();
    x.sub_mul_assign(
        Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_val_ref_fail_2() {
    let mut x = Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(
        Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_ref_val_fail_1() {
    let mut x = Natural::from_str("123").unwrap();
    x.sub_mul_assign(
        &Natural::from_str("5").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_ref_val_fail_2() {
    let mut x = Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(
        &Natural::from_str("1000000000000").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_ref_ref_fail_1() {
    let mut x = Natural::from_str("123").unwrap();
    x.sub_mul_assign(
        &Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_ref_ref_fail_2() {
    let mut x = Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(
        &Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_fail_1() {
    Natural::from_str("123").unwrap().sub_mul(
        Natural::from_str("5").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_fail_2() {
    Natural::from_str("1000000000000").unwrap().sub_mul(
        Natural::from_str("1000000000000").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_val_ref_fail_1() {
    Natural::from_str("123").unwrap().sub_mul(
        Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_val_ref_fail_2() {
    Natural::from_str("1000000000000").unwrap().sub_mul(
        Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_ref_val_fail_1() {
    Natural::from_str("123").unwrap().sub_mul(
        &Natural::from_str("5").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_ref_val_fail_2() {
    Natural::from_str("1000000000000").unwrap().sub_mul(
        &Natural::from_str("1000000000000").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_ref_ref_fail_1() {
    Natural::from_str("123").unwrap().sub_mul(
        &Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_ref_ref_fail_2() {
    Natural::from_str("1000000000000").unwrap().sub_mul(
        &Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_ref_ref_ref_fail_1() {
    (&Natural::from_str("123").unwrap()).sub_mul(
        &Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_ref_ref_ref_fail_2() {
    (&Natural::from_str("1000000000000").unwrap()).sub_mul(
        &Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}
