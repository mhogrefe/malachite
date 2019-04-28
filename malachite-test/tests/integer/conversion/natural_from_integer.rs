use common::test_properties;
use malachite_base::conversion::CheckedFrom;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_test::inputs::integer::integers;
use std::str::FromStr;

#[test]
fn test_from_integer() {
    let test = |n, out| {
        let on = Natural::checked_from(Integer::from_str(n).unwrap());
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = Natural::checked_from(&Integer::from_str(n).unwrap());
        assert_eq!(format!("{:?}", on), out);
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
fn from_integer_properties() {
    test_properties(integers, |x| {
        let natural_x = Natural::checked_from(x.clone());
        assert!(natural_x.as_ref().map_or(true, |n| n.is_valid()));

        let natural_x = Natural::checked_from(x);
        assert!(natural_x.as_ref().map_or(true, |n| n.is_valid()));

        assert_eq!(natural_x.is_some(), *x >= 0 as Limb);
        if let Some(n) = natural_x {
            assert_eq!(n.to_string(), x.to_string());
            assert_eq!(Integer::from(&n), *x);
            assert_eq!(Integer::from(n), *x);
        }
    });
}
