use malachite_base::num::conversion::traits::IsInteger;
use malachite_nz::natural::Natural;
use malachite_nz_test_util::generators::natural_gen;
use std::str::FromStr;

#[test]
fn test_is_integer() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().is_integer(), out);
    };
    test("0", true);
    test("1", true);
    test("100", true);
}

#[test]
fn is_integer_properties() {
    natural_gen().test_properties(|n| {
        assert!(n.is_integer());
    });
}
