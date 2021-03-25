use malachite_nz_test_util::natural::logic::and::{natural_and_alt_1, natural_and_alt_2};
use num::BigUint;
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::logic::and::{
    limbs_and, limbs_and_in_place_either, limbs_and_limb, limbs_and_same_length_to_out,
    limbs_and_to_out, limbs_slice_and_in_place_left, limbs_slice_and_same_length_in_place_left,
    limbs_vec_and_in_place_left,
};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_and_limb() {
    let test = |xs: &[Limb], y: Limb, out: Limb| {
        assert_eq!(limbs_and_limb(xs, y), out);
    };
    test(&[6, 7], 2, 2);
    test(&[100, 101, 102], 10, 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
const fn limbs_and_limb_fail() {
    limbs_and_limb(&[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_and_and_limbs_vec_and_in_place_left() {
    let test = |xs_before, ys, out| {
        assert_eq!(limbs_and(xs_before, ys), out);

        let mut xs = xs_before.to_vec();
        limbs_vec_and_in_place_left(&mut xs, ys);
        assert_eq!(xs, out);
    };
    test(&[], &[], vec![]);
    test(&[2], &[3], vec![2]);
    test(&[1, 1, 1], &[1, 2, 3], vec![1, 0, 1]);
    test(&[6, 7], &[1, 2, 3], vec![0, 2]);
    test(&[1, 2, 3], &[6, 7], vec![0, 2]);
    test(&[100, 101, 102], &[102, 101, 100], vec![100, 101, 100]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_and_same_length_to_out() {
    let test = |xs, ys, out_before: &[Limb], out_after| {
        let mut out = out_before.to_vec();
        limbs_and_same_length_to_out(&mut out, xs, ys);
        assert_eq!(out, out_after);
    };
    test(&[], &[], &[0, 0], vec![0, 0]);
    test(&[1, 1, 1], &[1, 2, 3], &[5, 5, 5, 5], vec![1, 0, 1, 5]);
    test(&[6, 7], &[1, 2], &[0, 0], vec![0, 2]);
    test(&[6, 7], &[1, 2], &[10, 10, 10, 10], vec![0, 2, 10, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        vec![100, 101, 100, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_same_length_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    limbs_and_same_length_to_out(&mut out, &[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_same_length_to_out_fail_2() {
    let mut out = vec![10];
    limbs_and_same_length_to_out(&mut out, &[6, 7], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_and_to_out() {
    let test = |xs, ys, out_before: &[Limb], out_after| {
        let mut out = out_before.to_vec();
        limbs_and_to_out(&mut out, xs, ys);
        assert_eq!(out, out_after);
    };
    test(&[], &[], &[0, 0], vec![0, 0]);
    test(&[1, 1, 1], &[1, 2, 3], &[5, 5, 5, 5], vec![1, 0, 1, 5]);
    test(&[6, 7], &[1, 2, 3], &[10, 10, 10, 10], vec![0, 2, 0, 10]);
    test(&[1, 2, 3], &[6, 7], &[10, 10, 10, 10], vec![0, 2, 0, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        vec![100, 101, 100, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_to_out_fail() {
    let mut out = vec![10, 10];
    limbs_and_to_out(&mut out, &[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_and_same_length_in_place_left() {
    let test = |xs_before: &[Limb], ys, xs_after| {
        let mut xs = xs_before.to_vec();
        limbs_slice_and_same_length_in_place_left(&mut xs, ys);
        assert_eq!(xs, xs_after);
    };
    test(&[], &[], vec![]);
    test(&[6, 7], &[1, 2], vec![0, 2]);
    test(&[1, 1, 1], &[1, 2, 3], vec![1, 0, 1]);
    test(&[100, 101, 102], &[102, 101, 100], vec![100, 101, 100]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_and_same_length_in_place_left_fail() {
    let mut out = vec![6, 7];
    limbs_slice_and_same_length_in_place_left(&mut out, &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_and_in_place_left() {
    let test = |xs_before: &[Limb], ys, truncate_length, xs_after| {
        let mut xs = xs_before.to_vec();
        assert_eq!(limbs_slice_and_in_place_left(&mut xs, ys), truncate_length);
        assert_eq!(xs, xs_after);
    };
    test(&[], &[], None, vec![]);
    test(&[6, 7], &[1, 2], None, vec![0, 2]);
    test(&[6, 7], &[1, 2, 3], None, vec![0, 2]);
    test(&[1, 2, 3], &[6, 7], Some(2), vec![0, 2, 3]);
    test(&[], &[1, 2, 3], None, vec![]);
    test(&[1, 2, 3], &[], Some(0), vec![1, 2, 3]);
    test(&[1, 1, 1], &[1, 2, 3], None, vec![1, 0, 1]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        None,
        vec![100, 101, 100],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_and_in_place_either() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], right, xs_after, ys_after| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_and_in_place_either(&mut xs, &mut ys), right);
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[], &[], false, vec![], vec![]);
    test(&[6, 7], &[1, 2], false, vec![0, 2], vec![1, 2]);
    test(&[6, 7], &[1, 2, 3], false, vec![0, 2], vec![1, 2, 3]);
    test(&[1, 2, 3], &[6, 7], true, vec![1, 2, 3], vec![0, 2]);
    test(&[], &[1, 2, 3], false, vec![], vec![1, 2, 3]);
    test(&[1, 2, 3], &[], true, vec![1, 2, 3], vec![]);
    test(&[1, 1, 1], &[1, 2, 3], false, vec![1, 0, 1], vec![1, 2, 3]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![100, 101, 100],
        vec![102, 101, 100],
    );
}

#[test]
fn test_and() {
    let test = |u, v, out| {
        let mut n = Natural::from_str(u).unwrap();
        n &= Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n &= &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() & Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() & Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() & &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() & &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!(
            natural_and_alt_1(
                &Natural::from_str(u).unwrap(),
                &Natural::from_str(v).unwrap(),
            )
            .to_string(),
            out
        );
        assert_eq!(
            natural_and_alt_2(
                &Natural::from_str(u).unwrap(),
                &Natural::from_str(v).unwrap(),
            )
            .to_string(),
            out
        );

        let n = BigUint::from_str(u).unwrap() & BigUint::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() & rug::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "0");
    test("123", "0", "0");
    test("123", "456", "72");
    test("1000000000000", "123", "0");
    test("123", "1000000000000", "0");
    test("1000000000001", "123", "1");
    test("12345678987654321", "987654321", "579887281");
    test("1000000000000", "999999999999", "999999995904");
    test("12345678987654321", "314159265358979", "312331665941633");
}
