use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::from_native;
use num;
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
    };
    test(0, "0");
    test(123, "123");
    test(u32::max_value(), "4294967295");
}

#[test]
fn from_u32_properties() {
    // from_u32(u) is valid.
    // from_u32(u).to_u32() == Some(u)
    let one_u32 = |u: u32| {
        let n = gmp::Natural::from(u);
        let raw_native_n = native::Natural::from(u);
        assert!(raw_native_n.is_valid());
        let native_n = from_native(&raw_native_n);
        assert!(n.is_valid());
        assert_eq!(n.to_u32(), Some(u));
        assert_eq!(n, native_n);
    };

    for u in exhaustive_u().take(LARGE_LIMIT) {
        one_u32(u);
    }

    for u in random_x(&EXAMPLE_SEED).take(LARGE_LIMIT) {
        one_u32(u);
    }
}
