use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, SaturatingFrom,
};
use malachite_base::strings::ToDebugString;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::str::FromStr;

#[test]
fn test_checked_from_integer() {
    let test = |n, out| {
        let on = Natural::checked_from(Integer::from_str(n).unwrap());
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = Natural::checked_from(&Integer::from_str(n).unwrap());
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test("0", "Some(0)");
    test("123", "Some(123)");
    test("-123", "None");
    test("1000000000000", "Some(1000000000000)");
    test("-1000000000000", "None");
    test("2147483647", "Some(2147483647)");
    test("2147483648", "Some(2147483648)");
    test("-2147483648", "None");
    test("-2147483649", "None");
}

#[test]
fn test_exact_from_integer() {
    let test = |u, out| {
        let n = Natural::exact_from(Integer::from_str(u).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::exact_from(&Integer::from_str(u).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0");
    test("123", "123");
    test("1000000000000", "1000000000000");
    test("2147483647", "2147483647");
    test("2147483648", "2147483648");
}

#[test]
#[should_panic]
fn natural_exact_from_integer_fail_1() {
    Natural::exact_from(Integer::from_str("-123").unwrap());
}

#[test]
#[should_panic]
fn natural_exact_from_integer_fail_2() {
    Natural::exact_from(Integer::from_str("-1000000000000").unwrap());
}

#[test]
#[should_panic]
fn natural_exact_from_integer_fail_3() {
    Natural::exact_from(Integer::from_str("-2147483648").unwrap());
}

#[test]
#[should_panic]
fn natural_exact_from_integer_fail_4() {
    Natural::exact_from(Integer::from_str("-2147483649").unwrap());
}

#[test]
#[should_panic]
fn natural_exact_from_integer_ref_fail_1() {
    Natural::exact_from(&Integer::from_str("-123").unwrap());
}

#[test]
#[should_panic]
fn natural_exact_from_integer_ref_fail_2() {
    Natural::exact_from(&Integer::from_str("-1000000000000").unwrap());
}

#[test]
#[should_panic]
fn natural_exact_from_integer_ref_fail_3() {
    Natural::exact_from(&Integer::from_str("-2147483648").unwrap());
}

#[test]
#[should_panic]
fn natural_exact_from_integer_ref_fail_4() {
    Natural::exact_from(&Integer::from_str("-2147483649").unwrap());
}

#[test]
fn test_saturating_from_integer() {
    let test = |u, out| {
        let n = Natural::saturating_from(Integer::from_str(u).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::saturating_from(&Integer::from_str(u).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0");
    test("123", "123");
    test("-123", "0");
    test("1000000000000", "1000000000000");
    test("-1000000000000", "0");
    test("2147483647", "2147483647");
    test("2147483648", "2147483648");
    test("-2147483648", "0");
    test("-2147483649", "0");
}

#[test]
fn test_convertible_from_integer() {
    let test = |u, out| {
        assert_eq!(
            Natural::convertible_from(Integer::from_str(u).unwrap()),
            out
        );
        assert_eq!(
            Natural::convertible_from(&Integer::from_str(u).unwrap()),
            out
        );
    };
    test("0", true);
    test("123", true);
    test("-123", false);
    test("1000000000000", true);
    test("-1000000000000", false);
    test("2147483647", true);
    test("2147483648", true);
    test("-2147483648", false);
    test("-2147483649", false);
}
