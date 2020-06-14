use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwo, ModPowerOfTwoAssign, ModPowerOfTwoIsReduced, NegModPowerOfTwo,
    NegModPowerOfTwoAssign, RemPowerOfTwo, RemPowerOfTwoAssign,
};

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::mod_power_of_two::{
    limbs_mod_power_of_two, limbs_neg_mod_power_of_two, limbs_neg_mod_power_of_two_in_place,
    limbs_slice_mod_power_of_two_in_place, limbs_vec_mod_power_of_two_in_place,
};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_two_and_limbs_vec_mod_power_of_two_in_place() {
    let test = |xs: &[Limb], pow: u64, out: &[Limb]| {
        assert_eq!(limbs_mod_power_of_two(xs, pow), out);

        let mut xs = xs.to_vec();
        limbs_vec_mod_power_of_two_in_place(&mut xs, pow);
        assert_eq!(xs, out);
    };
    test(&[], 0, &[]);
    test(&[], 5, &[]);
    test(&[], 100, &[]);
    test(&[6, 7], 2, &[2]);
    test(&[100, 101, 102], 10, &[100]);
    test(&[123, 456], 0, &[]);
    test(&[123, 456], 1, &[1]);
    test(&[123, 456], 10, &[123]);
    test(&[123, 456], 33, &[123, 0]);
    test(&[123, 456], 40, &[123, 200]);
    test(&[123, 456], 100, &[123, 456]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_mod_power_of_two_in_place() {
    let test = |xs: &[Limb], pow: u64, out: &[Limb]| {
        let mut xs = xs.to_vec();
        limbs_slice_mod_power_of_two_in_place(&mut xs, pow);
        assert_eq!(xs, out);
    };
    test(&[], 0, &[]);
    test(&[], 5, &[]);
    test(&[], 100, &[]);
    test(&[6, 7], 2, &[2, 0]);
    test(&[100, 101, 102], 10, &[100, 0, 0]);
    test(&[123, 456], 0, &[0, 0]);
    test(&[123, 456], 1, &[1, 0]);
    test(&[123, 456], 10, &[123, 0]);
    test(&[123, 456], 33, &[123, 0]);
    test(&[123, 456], 40, &[123, 200]);
    test(&[123, 456], 100, &[123, 456]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_mod_power_of_two_and_limbs_neg_mod_power_of_two_in_place() {
    let test = |xs: &[Limb], pow: u64, out: &[Limb]| {
        assert_eq!(limbs_neg_mod_power_of_two(xs, pow), out);

        let mut xs = xs.to_vec();
        limbs_neg_mod_power_of_two_in_place(&mut xs, pow);
        assert_eq!(xs, out);
    };
    test(&[], 0, &[]);
    test(&[], 5, &[0]);
    test(&[], 100, &[0, 0, 0, 0]);
    test(&[6, 7], 2, &[2]);
    test(&[100, 101, 102], 10, &[924]);
    test(&[123, 456], 0, &[]);
    test(&[123, 456], 1, &[1]);
    test(&[123, 456], 10, &[901]);
    test(&[123, 456], 33, &[4_294_967_173, 1]);
    test(&[123, 456], 40, &[4_294_967_173, 55]);
    test(
        &[123, 456],
        100,
        &[4_294_967_173, 4_294_966_839, u32::MAX, 15],
    );
}

#[test]
fn test_mod_power_of_two_and_rem_power_of_two() {
    let test = |u, v: u64, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.mod_power_of_two_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_power_of_two_is_reduced(v));

        let n = Natural::from_str(u).unwrap().mod_power_of_two(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).mod_power_of_two(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n.rem_power_of_two_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().rem_power_of_two(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).rem_power_of_two(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 0, "0");
    test("260", 8, "4");
    test("1611", 4, "11");
    test("123", 100, "123");
    test("1000000000000", 0, "0");
    test("1000000000000", 12, "0");
    test("1000000000001", 12, "1");
    test("999999999999", 12, "4095");
    test("1000000000000", 15, "4096");
    test("1000000000000", 100, "1000000000000");
    test("1000000000000000000000000", 40, "1020608380928");
    test("1000000000000000000000000", 64, "2003764205206896640");
    test("4294967295", 31, "2147483647");
    test("4294967295", 32, "4294967295");
    test("4294967295", 33, "4294967295");
    test("4294967296", 31, "0");
    test("4294967296", 32, "0");
    test("4294967296", 33, "4294967296");
    test("4294967297", 31, "1");
    test("4294967297", 32, "1");
    test("4294967297", 33, "4294967297");
}

#[test]
fn test_neg_mod_power_of_two() {
    let test = |u, v: u64, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.neg_mod_power_of_two_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_power_of_two_is_reduced(v));

        let n = Natural::from_str(u).unwrap().neg_mod_power_of_two(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).neg_mod_power_of_two(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };

    test("0", 0, "0");
    test("260", 8, "252");
    test("1611", 4, "5");
    test("123", 100, "1267650600228229401496703205253");
    test("1000000000000", 0, "0");
    test("1000000000000", 12, "0");
    test("1000000000001", 12, "4095");
    test("999999999999", 12, "1");
    test("1000000000000", 15, "28672");
    test("1000000000000", 100, "1267650600228229400496703205376");
    test("1000000000000000000000000", 40, "78903246848");
    test("1000000000000000000000000", 64, "16442979868502654976");
    test("4294967295", 31, "1");
    test("4294967295", 32, "1");
    test("4294967295", 33, "4294967297");
    test("4294967296", 31, "0");
    test("4294967296", 32, "0");
    test("4294967296", 33, "4294967296");
    test("4294967297", 31, "2147483647");
    test("4294967297", 32, "4294967295");
    test("4294967297", 33, "4294967295");
}
