use common::test_properties;
use malachite_nz::natural::Natural;
use malachite_test::inputs::natural::naturals;
use std::str::FromStr;

#[test]
fn test_into_integer() {
    let test = |s| {
        let x = Natural::from_str(s).unwrap().into_integer();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), s);

        let x = Natural::from_str(s).unwrap().to_integer();
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
fn to_integer_properties() {
    test_properties(naturals, |x| {
        let integer_x = x.clone().into_integer();
        assert!(integer_x.is_valid());
        assert_eq!(integer_x.to_string(), x.to_string());

        let integer_x_alt = x.to_integer();
        assert!(integer_x_alt.is_valid());
        assert_eq!(integer_x_alt, integer_x);

        assert_eq!(integer_x.to_natural().as_ref(), Some(x));
        assert_eq!(integer_x.into_natural().as_ref(), Some(x));
    });
}
