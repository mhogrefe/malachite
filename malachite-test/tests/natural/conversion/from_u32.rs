use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_to_native, num_to_native, rugint_to_native};
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::primitive_ints::exhaustive_u;

#[test]
fn test_from_u32() {
    let test = |u: u32, out| {
        let x = native::Natural::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = gmp::Natural::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        assert_eq!(num::BigUint::from(u).to_string(), out);

        assert_eq!(rugint::Integer::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(u32::max_value(), "4294967295");
}

#[test]
fn from_u32_properties() {
    // from(u: u32) is valid.
    // x + y is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // from(u: u32).to_u32() == Some(u)
    let one_u32 = |u: u32| {
        let n = native::Natural::from(u);
        let raw_gmp_n = gmp::Natural::from(u);
        assert!(raw_gmp_n.is_valid());
        let gmp_n = gmp_to_native(&raw_gmp_n);
        let num_n = num_to_native(&num::BigUint::from(u));
        let rugint_n = rugint_to_native(&rugint::Integer::from(u));
        assert!(n.is_valid());
        assert_eq!(n.to_u32(), Some(u));
        assert_eq!(n, gmp_n);
        assert_eq!(n, num_n);
        assert_eq!(n, rugint_n);
    };

    for u in exhaustive_u().take(LARGE_LIMIT) {
        one_u32(u);
    }

    for u in random_x(&EXAMPLE_SEED).take(LARGE_LIMIT) {
        one_u32(u);
    }
}
