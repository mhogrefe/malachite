use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, SaturatingFrom,
};
use malachite_base::strings::ToDebugString;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz_test_util::generators::integer_gen;
use std::str::FromStr;

#[test]
fn test_checked_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();

        let on = Natural::checked_from(u.clone());
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = Natural::checked_from(&u);
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
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();

        let n = Natural::exact_from(u.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::exact_from(&u);
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
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();

        let n = Natural::saturating_from(u.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::saturating_from(&u);
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
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();

        assert_eq!(Natural::convertible_from(u.clone()), out);
        assert_eq!(Natural::convertible_from(&u), out);
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

#[test]
fn checked_from_integer_properties() {
    integer_gen().test_properties(|x| {
        let natural_x = Natural::checked_from(x.clone());
        assert!(natural_x.as_ref().map_or(true, Natural::is_valid));

        let natural_x_alt = Natural::checked_from(&x);
        assert!(natural_x_alt.as_ref().map_or(true, Natural::is_valid));
        assert_eq!(natural_x, natural_x_alt);

        assert_eq!(natural_x.is_some(), x >= 0);
        assert_eq!(natural_x.is_some(), Natural::convertible_from(&x));
        if let Some(n) = natural_x {
            assert_eq!(n.to_string(), x.to_string());
            assert_eq!(Natural::exact_from(&x), n);
            assert_eq!(Integer::from(&n), x);
            assert_eq!(Integer::from(n), x);
        }
    });
}

#[test]
fn saturating_from_integer_properties() {
    integer_gen().test_properties(|x| {
        let natural_x = Natural::saturating_from(x.clone());
        assert!(natural_x.is_valid());

        let natural_x_alt = Natural::saturating_from(&x);
        assert!(natural_x_alt.is_valid());
        assert_eq!(natural_x, natural_x_alt);

        assert_eq!(natural_x == 0, x <= 0);
        assert!(natural_x >= x);
        assert_eq!(natural_x == x, Natural::convertible_from(x));
    });
}

#[test]
fn convertible_from_integer_properties() {
    integer_gen().test_properties(|x| {
        let convertible = Natural::convertible_from(x.clone());
        assert_eq!(Natural::convertible_from(&x), convertible);
        assert_eq!(convertible, x >= 0);
    });
}
