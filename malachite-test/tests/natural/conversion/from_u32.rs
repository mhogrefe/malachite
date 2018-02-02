use common::LARGE_LIMIT;
use malachite_nz::natural::Natural;
use malachite_test::common::{biguint_to_natural, rug_integer_to_natural, GenerationMode};
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
    // from(u: u32) is valid.
    // from(u: u32) is equivalent for malachite, num, and rug.
    // from(u: u32).to_u32() == Some(u)
    let one_u32 = |u: u32| {
        let n = Natural::from(u);
        let num_n = biguint_to_natural(&BigUint::from(u));
        let rug_n = rug_integer_to_natural(&rug::Integer::from(u));
        assert!(n.is_valid());
        assert_eq!(n.to_u32(), Some(u));
        assert_eq!(n, num_n);
        assert_eq!(n, rug_n);
    };

    for u in unsigneds(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_u32(u);
    }

    for u in unsigneds(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_u32(u);
    }
}
