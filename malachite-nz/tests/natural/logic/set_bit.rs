use std::str::FromStr;

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_nz_test_util::natural::logic::set_bit::num_set_bit;
use num::BigUint;
use rug;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::logic::bit_access::{limbs_slice_set_bit, limbs_vec_set_bit};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_set_bit() {
    let test = |xs: &[Limb], index: u64, out: &[Limb]| {
        let mut mut_xs = xs.to_vec();
        limbs_slice_set_bit(&mut mut_xs, index);
        assert_eq!(mut_xs, out);
    };
    test(&[0, 1], 0, &[1, 1]);
    test(&[1, 1], 0, &[1, 1]);
    test(&[1, 1], 1, &[3, 1]);
    test(&[3, 1], 33, &[3, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_set_bit_fail() {
    let mut mut_xs = vec![1, 2, 3];
    limbs_slice_set_bit(&mut mut_xs, 100);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_vec_set_bit() {
    let test = |xs: &[Limb], index: u64, out: &[Limb]| {
        let mut mut_xs = xs.to_vec();
        limbs_vec_set_bit(&mut mut_xs, index);
        assert_eq!(mut_xs, out);
    };
    test(&[0, 1], 0, &[1, 1]);
    test(&[1, 1], 0, &[1, 1]);
    test(&[1, 1], 1, &[3, 1]);
    test(&[3, 1], 33, &[3, 3]);
    test(&[3, 3], 100, &[3, 3, 0, 16]);
    test(&[3, 3], 128, &[3, 3, 0, 0, 1]);
    test(&[], 32, &[0, 1]);
}

#[test]
fn test_set_bit() {
    let test = |u, index, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.set_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = BigUint::from_str(u).unwrap();
        num_set_bit(&mut n, index);
        assert_eq!(n.to_string(), out);

        let mut n = rug::Integer::from_str(u).unwrap();
        n.set_bit(u32::exact_from(index), true);
        assert_eq!(n.to_string(), out);
    };
    test("0", 10, "1024");
    test("100", 0, "101");
    test("1000000000000", 10, "1000000001024");
    test("1000000000000", 100, "1267650600228229402496703205376");
    test("5", 100, "1267650600228229401496703205381");
}
