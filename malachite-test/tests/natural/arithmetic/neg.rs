use common::test_properties;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::natural::naturals;
use rug;
use std::str::FromStr;

#[test]
fn test_neg() {
    let test = |s, out| {
        let neg = -Natural::from_str(s).unwrap();
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        let neg = -&Natural::from_str(s).unwrap();
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        assert_eq!((-rug::Integer::from_str(s).unwrap()).to_string(), out);
    };
    test("0", "0");
    test("123", "-123");
    test("1000000000000", "-1000000000000");
    test("2147483648", "-2147483648");
}

#[test]
fn neg_properties() {
    test_properties(naturals, |x| {
        let neg = -x.clone();
        assert!(neg.is_valid());

        let neg_alt = -x;
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        assert_eq!(rug_integer_to_integer(&(-natural_to_rug_integer(x))), neg);

        assert_eq!(-Integer::from(x), neg);
        assert_eq!(neg == *x, *x == 0);
        assert_eq!(-neg, *x);
    });
}
