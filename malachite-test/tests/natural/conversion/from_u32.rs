use common::test_properties;
use malachite_nz::natural::Natural;
use malachite_test::common::{biguint_to_natural, rug_integer_to_natural};
use malachite_test::inputs::base::unsigneds;
use num::BigUint;
use rug;
use std::u32;

#[test]
fn test_from_u32() {
    let test = |u: u32, out| {
        let x = Natural::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        assert_eq!(BigUint::from(u).to_string(), out);

        assert_eq!(rug::Integer::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(u32::MAX, "4294967295");
}

#[test]
fn from_u32_properties() {
    test_properties(unsigneds, |&u: &u32| {
        let n = Natural::from(u);
        assert!(n.is_valid());
        assert_eq!(n.to_u32(), Some(u));
        assert_eq!(biguint_to_natural(&BigUint::from(u)), n);
        assert_eq!(rug_integer_to_natural(&rug::Integer::from(u)), n);
    });
}
