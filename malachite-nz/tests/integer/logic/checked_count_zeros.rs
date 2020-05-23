use std::str::FromStr;

use malachite_nz_test_util::integer::logic::checked_count_zeros::{
    integer_checked_count_zeros_alt_1, integer_checked_count_zeros_alt_2,
};

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::integer::logic::checked_count_zeros::limbs_count_zeros_neg;
use malachite_nz::integer::Integer;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_count_zeros_neg() {
    let test = |xs, out| {
        assert_eq!(limbs_count_zeros_neg(xs), out);
    };
    test(&[0, 1, 2], 33);
    test(&[1, 0xffff_ffff], 32);
}

#[test]
fn test_checked_count_zeros() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().checked_count_zeros(), out);
        assert_eq!(
            integer_checked_count_zeros_alt_1(&Integer::from_str(n).unwrap()),
            out
        );
        assert_eq!(
            integer_checked_count_zeros_alt_2(&Integer::from_str(n).unwrap()),
            out
        );
    };
    test("0", None);
    test("105", None);
    test("-105", Some(3));
    test("1000000000000", None);
    test("-1000000000000", Some(24));
    test("4294967295", None);
    test("-4294967295", Some(31));
    test("4294967296", None);
    test("-4294967296", Some(32));
    test("18446744073709551615", None);
    test("-18446744073709551615", Some(63));
    test("18446744073709551616", None);
    test("-18446744073709551616", Some(64));
}
