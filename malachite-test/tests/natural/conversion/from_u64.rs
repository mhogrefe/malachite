use common::LARGE_LIMIT;
use malachite_nz::natural::Natural;
use malachite_test::common::{biguint_to_natural, GenerationMode};
use malachite_test::natural::conversion::from_u64::select_inputs;
use num::BigUint;

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
    test(u64::max_value(), "18446744073709551615");
}

#[test]
fn from_u64_properties() {
    // from(u: u64) is valid.
    // from(u: u64) is equivalent for malachite and num.
    // from(u: u64).to_64() == Some(u)
    let one_u64 = |u: u64| {
        let n = Natural::from(u);
        let num_n = biguint_to_natural(&BigUint::from(u));
        assert!(n.is_valid());
        assert_eq!(n.to_u64(), Some(u));
        assert_eq!(n, num_n);
    };

    for u in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_u64(u);
    }

    for u in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_u64(u);
    }
}
