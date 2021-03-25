use malachite_base_test_util::common::test_cmp_helper;
use num::BigUint;
use rug;
#[cfg(feature = "32_bit_limbs")]
use std::cmp::Ordering;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::comparison::ord::{limbs_cmp, limbs_cmp_same_length};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_cmp_same_length() {
    let test = |xs: &[Limb], ys: &[Limb], out| {
        assert_eq!(limbs_cmp_same_length(xs, ys), out);
    };
    test(&[3], &[5], Ordering::Less);
    test(&[3, 0], &[5, 0], Ordering::Less);
    test(&[1, 2], &[2, 1], Ordering::Greater);
    test(&[1, 2, 3], &[1, 2, 3], Ordering::Equal);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_cmp_same_length_fail() {
    limbs_cmp_same_length(&[1], &[2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_cmp() {
    let test = |xs: &[Limb], ys: &[Limb], out| {
        assert_eq!(limbs_cmp(xs, ys), out);
    };
    test(&[3], &[5], Ordering::Less);
    test(&[3, 1], &[5], Ordering::Greater);
    test(&[1, 2], &[2, 1, 3], Ordering::Less);
    test(&[1, 2, 3], &[1, 2, 3], Ordering::Equal);
}

#[test]
fn test_cmp() {
    let strings = vec![
        "0",
        "1",
        "2",
        "123",
        "999999999999",
        "1000000000000",
        "1000000000001",
    ];
    test_cmp_helper::<Natural>(&strings);
    test_cmp_helper::<BigUint>(&strings);
    test_cmp_helper::<rug::Integer>(&strings);
}
