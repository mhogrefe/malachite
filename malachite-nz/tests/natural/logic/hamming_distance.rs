use malachite_base::num::logic::traits::HammingDistance;
use malachite_nz_test_util::natural::logic::hamming_distance::{
    natural_hamming_distance_alt_1, natural_hamming_distance_alt_2, rug_hamming_distance,
};
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::logic::hamming_distance::{
    limbs_hamming_distance, limbs_hamming_distance_limb, limbs_hamming_distance_same_length,
};
use malachite_nz::natural::Natural;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_hamming_distance_limb() {
    let test = |xs, y, out| {
        assert_eq!(limbs_hamming_distance_limb(xs, y), out);
    };
    test(&[2], 3, 1);
    test(&[1, 1, 1], 1, 2);
    test(&[1, 1, 1], 2, 4);
    test(&[1, 2, 3], 0, 4);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_hamming_distance_limb_fail() {
    limbs_hamming_distance_limb(&[], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_hamming_distance_same_length() {
    let test = |xs, ys, out| {
        assert_eq!(limbs_hamming_distance_same_length(xs, ys), out);
    };
    test(&[], &[], 0);
    test(&[2], &[3], 1);
    test(&[1, 1, 1], &[1, 2, 3], 3);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_hamming_distance_limb_same_length_fail() {
    limbs_hamming_distance_same_length(&[1], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_hamming_distance() {
    let test = |xs, ys, out| {
        assert_eq!(limbs_hamming_distance(xs, ys), out);
    };
    test(&[], &[], 0);
    test(&[2], &[3], 1);
    test(&[1, 1, 1], &[1, 2, 3], 3);
    test(&[], &[1, 2, 3], 4);
    test(&[1, 2, 3], &[], 4);
    test(&[1, 1, 1], &[1, 2, 3, 4], 4);
    test(&[1, 2, 3, 4], &[1, 1, 1], 4);
}

#[test]
fn test_hamming_distance() {
    let test = |x, y, out| {
        assert_eq!(
            Natural::from_str(x)
                .unwrap()
                .hamming_distance(&Natural::from_str(y).unwrap()),
            out
        );
        assert_eq!(
            natural_hamming_distance_alt_1(
                &Natural::from_str(x).unwrap(),
                &Natural::from_str(y).unwrap(),
            ),
            out
        );
        assert_eq!(
            natural_hamming_distance_alt_2(
                &Natural::from_str(x).unwrap(),
                &Natural::from_str(y).unwrap(),
            ),
            out
        );
        assert_eq!(
            rug_hamming_distance(
                &rug::Integer::from_str(x).unwrap(),
                &rug::Integer::from_str(y).unwrap(),
            ),
            out
        );
    };
    test("105", "123", 2);
    test("1000000000000", "0", 13);
    test("4294967295", "0", 32);
    test("4294967295", "4294967295", 0);
    test("4294967295", "4294967296", 33);
    test("1000000000000", "1000000000001", 1);
}
