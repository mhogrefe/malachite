use common::test_properties;
use malachite_nz::integer::Integer;
use malachite_test::common::{bigint_to_integer, rug_integer_to_integer};
use malachite_test::inputs::base::unsigneds;
use num::BigInt;
use rug;
use std::u32;

#[test]
fn test_from_u32() {
    let test = |u: u32, out| {
        let x = Integer::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        assert_eq!(BigInt::from(u).to_string(), out);

        assert_eq!(rug::Integer::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(u32::MAX, "4294967295");
}

#[test]
fn from_u32_properties() {
    test_properties(unsigneds, |&u: &u32| {
        let n = Integer::from(u);
        assert!(n.is_valid());
        assert_eq!(n.to_u32(), Some(u));

        assert_eq!(bigint_to_integer(&BigInt::from(u)), n);
        assert_eq!(rug_integer_to_integer(&rug::Integer::from(u)), n);
    });
}
