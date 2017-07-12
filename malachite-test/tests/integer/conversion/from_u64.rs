use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, num_bigint_to_native_integer};
use num;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::primitive_ints::exhaustive_u;

#[test]
fn test_from_u64() {
    let test = |u: u64, out| {
        let x = native::Integer::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = gmp::Integer::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        assert_eq!(num::BigInt::from(u).to_string(), out);
    };
    test(0u64, "0");
    test(123u64, "123");
    test(1000000000000u64, "1000000000000");
    test(u64::max_value(), "18446744073709551615");
}

#[test]
fn from_u64_properties() {
    // from(u: u64) is valid.
    // from(u: u64) is equivalent for malachite-gmp, malachite-native, and num.
    // from(u: u64).to_u64() == Some(u)
    let one_u64 = |u: u64| {
        let n = native::Integer::from(u);
        let raw_gmp_n = gmp::Integer::from(u);
        assert!(raw_gmp_n.is_valid());
        let gmp_n = gmp_integer_to_native(&raw_gmp_n);
        let num_n = num_bigint_to_native_integer(&num::BigInt::from(u));
        assert!(n.is_valid());
        //TODO assert_eq!(n.to_u64(), Some(u));
        assert_eq!(n, gmp_n);
        assert_eq!(n, num_n);
    };

    for u in exhaustive_u().take(LARGE_LIMIT) {
        one_u64(u);
    }

    for u in random_x(&EXAMPLE_SEED).take(LARGE_LIMIT) {
        one_u64(u);
    }
}