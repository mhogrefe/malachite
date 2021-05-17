use malachite_base::num::logic::traits::CheckedHammingDistance;
use malachite_nz_test_util::integer::logic::checked_hamming_distance::{
    integer_checked_hamming_distance_alt_1, integer_checked_hamming_distance_alt_2,
    rug_checked_hamming_distance,
};
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::integer::logic::checked_hamming_distance::{
    limbs_hamming_distance_limb_neg, limbs_hamming_distance_neg,
};
use malachite_nz::integer::Integer;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_hamming_distance_limb_neg() {
    let test = |xs, y, out| {
        assert_eq!(limbs_hamming_distance_limb_neg(xs, y), out);
    };
    test(&[2], 2, 0);
    test(&[1, 1, 1], 1, 2);
    test(&[1, 1, 1], 2, 3);
    test(&[1, 2, 3], 3, 4);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_hamming_distance_limb_neg_fail() {
    limbs_hamming_distance_limb_neg(&[], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_hamming_distance_neg() {
    let test = |xs, ys, out| {
        assert_eq!(limbs_hamming_distance_neg(xs, ys), out);
    };
    test(&[2], &[3], 2);
    test(&[1, 1, 1], &[1, 2, 3], 3);
    test(&[1, 1, 1], &[1, 2, 3, 4], 4);
    test(&[1, 2, 3, 4], &[1, 1, 1], 4);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_hamming_distance_neg_fail_1() {
    limbs_hamming_distance_neg(&[0, 0], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_hamming_distance_neg_fail_2() {
    limbs_hamming_distance_neg(&[1, 2, 3], &[0, 0]);
}

#[test]
fn test_checked_hamming_distance() {
    let test = |s, t, out| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        assert_eq!(u.checked_hamming_distance(&v), out);
        assert_eq!(integer_checked_hamming_distance_alt_1(&u, &v,), out);
        assert_eq!(integer_checked_hamming_distance_alt_2(&u, &v,), out);
        assert_eq!(
            rug_checked_hamming_distance(
                &rug::Integer::from_str(s).unwrap(),
                &rug::Integer::from_str(t).unwrap(),
            ),
            out
        );
    };
    test("105", "123", Some(2));
    test("1000000000000", "0", Some(13));
    test("4294967295", "0", Some(32));
    test("4294967295", "4294967295", Some(0));
    test("4294967295", "4294967296", Some(33));
    test("1000000000000", "1000000000001", Some(1));
    test("-105", "-123", Some(2));
    test("-1000000000000", "-1", Some(24));
    test("-4294967295", "-1", Some(31));
    test("-4294967295", "-4294967295", Some(0));
    test("-4294967295", "-4294967296", Some(1));
    test("-1000000000000", "-1000000000001", Some(13));
    test("-105", "123", None);
    test("-1000000000000", "0", None);
    test("-4294967295", "0", None);
    test("-4294967295", "4294967295", None);
    test("-4294967295", "4294967296", None);
    test("-1000000000000", "1000000000001", None);
    test("105", "-123", None);
    test("1000000000000", "-1", None);
    test("4294967295", "-1", None);
    test("4294967295", "-4294967295", None);
    test("4294967295", "-4294967296", None);
    test("1000000000000", "-1000000000001", None);
}
