use common::test_properties;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::natural::naturals;
use rug;
use std::str::FromStr;

#[test]
fn test_not() {
    let test = |s, out| {
        let not = !Natural::from_str(s).unwrap();
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        let not = !(&Natural::from_str(s).unwrap());
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        assert_eq!((!rug::Integer::from_str(s).unwrap()).to_string(), out);
    };
    test("0", "-1");
    test("123", "-124");
    test("1000000000000", "-1000000000001");
    test("2147483647", "-2147483648");
}

#[test]
fn not_properties() {
    test_properties(naturals, |x| {
        let not = !x.clone();
        assert!(not.is_valid());

        let rug_not = !natural_to_rug_integer(x);
        assert_eq!(rug_integer_to_integer(&rug_not), not);

        let not_alt = !x;
        assert!(not_alt.is_valid());
        assert_eq!(not_alt, not);

        assert!(not < 0);
        assert_eq!(!Integer::from(x), not);
        assert_ne!(not, *x);
        assert_eq!(!&not, *x);
    });
}
