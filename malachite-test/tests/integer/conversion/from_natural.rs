use common::test_properties;
use malachite_base::misc::CheckedFrom;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::inputs::natural::naturals;
use std::str::FromStr;

#[test]
fn test_from_natural() {
    let test = |s| {
        let x = Integer::from(Natural::from_str(s).unwrap());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), s);

        let x = Integer::from(&Natural::from_str(s).unwrap());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), s);
    };
    test("0");
    test("123");
    test("1000000000000");
    test("4294967295");
    test("4294967296");
}

#[test]
fn from_natural_properties() {
    test_properties(naturals, |x| {
        let integer_x = Integer::from(x.clone());
        assert!(integer_x.is_valid());
        assert_eq!(integer_x.to_string(), x.to_string());

        let integer_x_alt = Integer::from(x);
        assert!(integer_x_alt.is_valid());
        assert_eq!(integer_x_alt, integer_x);

        assert_eq!(Natural::checked_from(&integer_x).as_ref(), Some(x));
        assert_eq!(Natural::checked_from(integer_x).as_ref(), Some(x));
    });
}
