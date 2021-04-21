use malachite_base::num::arithmetic::traits::{CeilingLogBase2, FloorLogBase2};
use malachite_base::num::basic::traits::Zero;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::log_base_2::{
    limbs_ceiling_log_base_2, limbs_floor_log_base_2,
};
use malachite_nz::natural::Natural;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_floor_log_base_2() {
    let test = |xs, out| {
        assert_eq!(limbs_floor_log_base_2(xs), out);
    };
    test(&[0b1], 0);
    test(&[0b10], 1);
    test(&[0b11], 1);
    test(&[0b100], 2);
    test(&[0, 0b1], 32);
    test(&[0, 0b1101], 35);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_floor_log_base_2_fail() {
    limbs_floor_log_base_2(&[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_ceiling_log_base_2() {
    let test = |xs, out| {
        assert_eq!(limbs_ceiling_log_base_2(xs), out);
    };
    test(&[0b1], 0);
    test(&[0b10], 1);
    test(&[0b11], 2);
    test(&[0b100], 2);
    test(&[0, 0b1], 32);
    test(&[0, 0b1101], 36);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_ceiling_log_base_2_fail() {
    limbs_ceiling_log_base_2(&[]);
}

#[test]
fn test_floor_log_base_2() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().floor_log_base_2(), out);
    };
    test("1", 0);
    test("100", 6);
    test("1000000000000", 39);
    test("4294967295", 31);
    test("4294967296", 32);
    test("18446744073709551615", 63);
    test("18446744073709551616", 64);
}

#[test]
#[should_panic]
fn floor_log_base_2_fail() {
    Natural::ZERO.floor_log_base_2();
}

#[test]
fn test_ceiling_log_base_2() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().ceiling_log_base_2(), out);
    };
    test("1", 0);
    test("100", 7);
    test("1000000000000", 40);
    test("4294967295", 32);
    test("4294967296", 32);
    test("4294967297", 33);
    test("18446744073709551615", 64);
    test("18446744073709551616", 64);
    test("18446744073709551617", 65);
}

#[test]
#[should_panic]
fn ceiling_log_base_2_fail() {
    Natural::ZERO.ceiling_log_base_2();
}
