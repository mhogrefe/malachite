use common::test_properties;
use malachite_base::misc::CheckedFrom;
use malachite_nz::natural::Natural;
use malachite_test::common::biguint_to_natural;
use malachite_test::inputs::base::unsigneds;
use num::BigUint;
use std::u64;

#[test]
fn test_from_u64() {
    let test = |u: u64, out| {
        let x = Natural::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        assert_eq!(BigUint::from(u).to_string(), out);
    };
    test(0u64, "0");
    test(123u64, "123");
    test(1_000_000_000_000u64, "1000000000000");
    test(u64::MAX, "18446744073709551615");
}

#[test]
fn from_u64_properties() {
    test_properties(unsigneds, |&u: &u64| {
        let n = Natural::from(u);
        assert!(n.is_valid());
        assert_eq!(u64::checked_from(&n), Some(u));
        assert_eq!(biguint_to_natural(&BigUint::from(u)), n);
    });
}
