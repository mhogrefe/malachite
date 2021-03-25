use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::add_mul::{
    limbs_add_mul, limbs_add_mul_in_place_left, limbs_add_mul_limb,
    limbs_slice_add_mul_limb_same_length_in_place_left,
    limbs_slice_add_mul_limb_same_length_in_place_right, limbs_vec_add_mul_limb_in_place_either,
    limbs_vec_add_mul_limb_in_place_left, limbs_vec_add_mul_limb_in_place_right,
};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_add_mul_limb() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], z: Limb, result: &[Limb]| {
        assert_eq!(limbs_add_mul_limb(xs_before, ys_before, z), result);
        let mut xs = xs_before.to_vec();
        limbs_vec_add_mul_limb_in_place_left(&mut xs, ys_before, z);
        assert_eq!(xs, result);
        let mut ys = ys_before.to_vec();
        limbs_vec_add_mul_limb_in_place_right(xs_before, &mut ys, z);
        assert_eq!(ys, result);
    };
    test(&[123, 456], &[123], 4, &[615, 456]);
    test(&[123, 456], &[123], u32::MAX, &[0, 579]);
    test(&[123], &[0, 123], 4, &[123, 492]);
    test(&[123, 456], &[0, 123], u32::MAX, &[123, 333, 123]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_add_mul_limb_same_length_in_place_left() {
    let test = |xs_before: &[Limb], ys: &[Limb], z: Limb, xs_after: &[Limb], carry: Limb| {
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_slice_add_mul_limb_same_length_in_place_left(&mut xs, ys, z),
            carry
        );
        assert_eq!(xs, xs_after);
    };
    test(&[123], &[123], 4, &[615], 0);
    test(&[123], &[123], u32::MAX, &[0], 123);
    test(&[123, 0], &[0, 123], 4, &[123, 492], 0);
    test(&[123, 456], &[0, 123], u32::MAX, &[123, 333], 123);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_add_mul_limb_same_length_in_place_left_fail() {
    limbs_slice_add_mul_limb_same_length_in_place_left(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_add_mul_limb_same_length_in_place_right() {
    let test = |xs: &[Limb], ys_before: &[Limb], z: Limb, ys_after: &[Limb], carry: Limb| {
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_slice_add_mul_limb_same_length_in_place_right(xs, &mut ys, z),
            carry
        );
        assert_eq!(ys, ys_after);
    };
    test(&[123, 456], &[123, 0], 4, &[615, 456], 0);
    test(&[123, 456], &[123, 0], u32::MAX, &[0, 579], 0);
    test(&[123, 0], &[0, 123], 4, &[123, 492], 0);
    test(&[123, 456], &[0, 123], u32::MAX, &[123, 333], 123);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_add_mul_limb_same_length_in_place_right_fail() {
    limbs_slice_add_mul_limb_same_length_in_place_right(&[10, 10], &mut [10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_vec_add_mul_limb_in_place_either() {
    let test = |xs_before: &[Limb],
                ys_before: &[Limb],
                z: Limb,
                right: bool,
                xs_after: &[Limb],
                ys_after: &[Limb]| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_vec_add_mul_limb_in_place_either(&mut xs, &mut ys, z),
            right
        );
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[123, 456], &[123], 4, false, &[615, 456], &[123]);
    test(
        &[123, 456],
        &[123, 0],
        u32::MAX,
        false,
        &[0, 579],
        &[123, 0],
    );
    test(&[123], &[0, 123], 4, true, &[123], &[123, 492]);
    test(
        &[123, 456],
        &[0, 123],
        u32::MAX,
        false,
        &[123, 333, 123],
        &[0, 123],
    );
}

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
