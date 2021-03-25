use malachite_nz_test_util::natural::logic::xor::{natural_xor_alt_1, natural_xor_alt_2};
use num::BigUint;
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::logic::xor::{
    limbs_xor, limbs_xor_in_place_either, limbs_xor_in_place_left, limbs_xor_limb,
    limbs_xor_limb_in_place, limbs_xor_limb_to_out, limbs_xor_same_length,
    limbs_xor_same_length_in_place_left, limbs_xor_same_length_to_out, limbs_xor_to_out,
};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_xor_limb_and_limbs_xor_limb_in_place() {
    let test = |xs: &[Limb], y: Limb, out: &[Limb]| {
        assert_eq!(limbs_xor_limb(xs, y), out);

        let mut xs = xs.to_vec();
        limbs_xor_limb_in_place(&mut xs, y);
        assert_eq!(xs, out);
    };
    test(&[6, 7], 2, &[4, 7]);
    test(&[100, 101, 102], 10, &[110, 101, 102]);
    test(&[123, 456], 789, &[878, 456]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_limb_fail() {
    limbs_xor_limb(&[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_limb_in_place_fail() {
    limbs_xor_limb_in_place(&mut [], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_xor_limb_to_out() {
    let test = |out_before: &[Limb], xs: &[Limb], y: Limb, out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        limbs_xor_limb_to_out(&mut out, xs, y);
        assert_eq!(out, out_after);
    };
    test(&[10, 10, 10, 10], &[6, 7], 2, &[4, 7, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        &[110, 101, 102, 10],
    );
    test(&[10, 10, 10, 10], &[123, 456], 789, &[878, 456, 10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_limb_to_out_fail_1() {
    limbs_xor_limb_to_out(&mut [], &[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_limb_to_out_fail_2() {
    limbs_xor_limb_to_out(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_xor_same_length_and_limbs_xor_same_length_in_place_left() {
    let test = |xs_before, ys, out| {
        assert_eq!(limbs_xor_same_length(xs_before, ys), out);

        let mut xs = xs_before.to_vec();
        limbs_xor_same_length_in_place_left(&mut xs, ys);
        assert_eq!(xs, out);
    };
    test(&[], &[], vec![]);
    test(&[2], &[3], vec![1]);
    test(&[1, 1, 1], &[1, 2, 3], vec![0, 3, 2]);
    test(&[6, 7], &[1, 2], vec![7, 5]);
    test(&[100, 101, 102], &[102, 101, 100], vec![2, 0, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_same_length_fail_1() {
    limbs_xor_same_length(&[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_same_length_in_place_left_fail() {
    let mut out = vec![6, 7];
    limbs_xor_same_length_in_place_left(&mut out, &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_xor_and_limbs_xor_in_place_left() {
    let test = |xs_before, ys, out| {
        assert_eq!(limbs_xor(xs_before, ys), out);

        let mut xs = xs_before.to_vec();
        limbs_xor_in_place_left(&mut xs, ys);
        assert_eq!(xs, out);
    };
    test(&[], &[], vec![]);
    test(&[2], &[3], vec![1]);
    test(&[1, 1, 1], &[1, 2, 3], vec![0, 3, 2]);
    test(&[6, 7], &[1, 2, 3], vec![7, 5, 3]);
    test(&[1, 2, 3], &[6, 7], vec![7, 5, 3]);
    test(&[100, 101, 102], &[102, 101, 100], vec![2, 0, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_xor_same_length_to_out() {
    let test = |xs, ys, out_before: &[Limb], out_after| {
        let mut out = out_before.to_vec();
        limbs_xor_same_length_to_out(&mut out, xs, ys);
        assert_eq!(out, out_after);
    };
    test(&[], &[], &[0, 0], vec![0, 0]);
    test(&[1, 1, 1], &[1, 2, 3], &[5, 5, 5, 5], vec![0, 3, 2, 5]);
    test(&[6, 7], &[1, 2], &[0, 0], vec![7, 5]);
    test(&[6, 7], &[1, 2], &[10, 10, 10, 10], vec![7, 5, 10, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        vec![2, 0, 2, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_same_length_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    limbs_xor_same_length_to_out(&mut out, &[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_same_length_to_out_fail_2() {
    let mut out = vec![10];
    limbs_xor_same_length_to_out(&mut out, &[6, 7], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_xor_to_out() {
    let test = |xs, ys, out_before: &[Limb], out_after| {
        let mut out = out_before.to_vec();
        limbs_xor_to_out(&mut out, xs, ys);
        assert_eq!(out, out_after);
    };
    test(&[], &[], &[0, 0], vec![0, 0]);
    test(&[1, 1, 1], &[1, 2, 3], &[5, 5, 5, 5], vec![0, 3, 2, 5]);
    test(&[6, 7], &[1, 2, 3], &[10, 10, 10, 10], vec![7, 5, 3, 10]);
    test(&[1, 2, 3], &[6, 7], &[10, 10, 10, 10], vec![7, 5, 3, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        vec![2, 0, 2, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_to_out_fail() {
    let mut out = vec![10, 10];
    limbs_xor_to_out(&mut out, &[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_xor_in_place_either() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], right, xs_after, ys_after| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_xor_in_place_either(&mut xs, &mut ys), right);
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[], &[], false, vec![], vec![]);
    test(&[6, 7], &[1, 2], false, vec![7, 5], vec![1, 2]);
    test(&[6, 7], &[1, 2, 3], true, vec![6, 7], vec![7, 5, 3]);
    test(&[1, 2, 3], &[6, 7], false, vec![7, 5, 3], vec![6, 7]);
    test(&[], &[1, 2, 3], true, vec![], vec![1, 2, 3]);
    test(&[1, 2, 3], &[], false, vec![1, 2, 3], vec![]);
    test(&[1, 1, 1], &[1, 2, 3], false, vec![0, 3, 2], vec![1, 2, 3]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![2, 0, 2],
        vec![102, 101, 100],
    );
}

#[test]
fn test_xor() {
    let test = |u, v, out| {
        let mut n = Natural::from_str(u).unwrap();
        n ^= Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n ^= &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() ^ Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() ^ Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() ^ &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() ^ &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!(
            natural_xor_alt_1(
                &Natural::from_str(u).unwrap(),
                &Natural::from_str(v).unwrap(),
            )
            .to_string(),
            out
        );
        assert_eq!(
            natural_xor_alt_2(
                &Natural::from_str(u).unwrap(),
                &Natural::from_str(v).unwrap(),
            )
            .to_string(),
            out
        );

        let n = BigUint::from_str(u).unwrap() ^ BigUint::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() ^ rug::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "435");
    test("1000000000000", "123", "1000000000123");
    test("123", "1000000000000", "1000000000123");
    test("1000000000001", "123", "1000000000122");
    test("12345678987654321", "0", "12345678987654321");
    test("12345678987654321", "456", "12345678987654521");
    test("12345678987654321", "987654321", "12345678815534080");
    test("1000000000000", "999999999999", "8191");
    test("12345678987654321", "314159265358979", "12035174921130034");
}
