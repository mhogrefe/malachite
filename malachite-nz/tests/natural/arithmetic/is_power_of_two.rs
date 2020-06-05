use std::str::FromStr;

use malachite_base::num::arithmetic::traits::IsPowerOfTwo;
use rug;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::is_power_of_two::limbs_is_power_of_two;
use malachite_nz::natural::Natural;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_is_power_of_two() {
    let test = |xs, out| {
        assert_eq!(limbs_is_power_of_two(xs), out);
    };
    test(&[1], true);
    test(&[2], true);
    test(&[3], false);
    test(&[4], true);
    test(&[256], true);
    test(&[0, 0, 0, 256], true);
    test(&[1, 0, 0, 256], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_is_power_of_two_fail() {
    limbs_is_power_of_two(&[]);
}

#[test]
fn test_is_power_of_two() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().is_power_of_two(), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().is_power_of_two(), out);
    };
    test("0", false);
    test("1", true);
    test("2", true);
    test("3", false);
    test("4", true);
    test("5", false);
    test("6", false);
    test("7", false);
    test("8", true);
    test("1024", true);
    test("1025", false);
    test("1000000000000", false);
    test("1099511627776", true);
}
