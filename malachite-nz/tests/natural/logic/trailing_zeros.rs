use std::str::FromStr;

use malachite_nz_test_util::natural::logic::trailing_zeros::natural_trailing_zeros_alt;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::logic::trailing_zeros::limbs_trailing_zeros;
use malachite_nz::natural::Natural;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_trailing_zeros() {
    let test = |xs, out| {
        assert_eq!(limbs_trailing_zeros(xs), out);
    };
    test(&[4], 2);
    test(&[0, 4], 34);
    test(&[1, 2, 3], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_trailing_zeros_fail_1() {
    limbs_trailing_zeros(&[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_trailing_zeros_fail_2() {
    limbs_trailing_zeros(&[0, 0, 0]);
}

#[test]
fn test_trailing_zeros() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().trailing_zeros(), out);
        assert_eq!(
            natural_trailing_zeros_alt(&Natural::from_str(n).unwrap()),
            out
        );
    };
    test("0", None);
    test("123", Some(0));
    test("1000000000000", Some(12));
    test("4294967295", Some(0));
    test("4294967296", Some(32));
    test("18446744073709551615", Some(0));
    test("18446744073709551616", Some(64));
}
