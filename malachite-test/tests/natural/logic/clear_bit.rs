use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, GenerationMode};
use malachite_test::natural::logic::clear_bit::select_inputs;
use std::str::FromStr;

#[test]
fn test_clear_bit() {
    let test = |u, index, out| {
        let mut n = native::Natural::from_str(u).unwrap();
        n.clear_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n.clear_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 10, "0");
    test("0", 100, "0");
    test("1024", 10, "0");
    test("101", 0, "100");
    test("1000000001024", 10, "1000000000000");
    test("1000000001024", 100, "1000000001024");
    test("1267650600228229402496703205376", 100, "1000000000000");
    test("1267650600228229401496703205381", 100, "5");
}

#[test]
fn clear_bit_properties() {
    // n.clear_bit(index) is equivalent for malachite-gmp and malachite-native.
    // n.clear_bit(index) is equivalent to n.assign_bit(index, false).
    // If n.get_bit(index), clearing and then setting at index won't do anything.
    // Setting a bit does not increase n.
    // If !n.get_bit(index), clearing at index won't do anything.
    let natural_and_u64 = |mut gmp_n: gmp::Natural, index: u64| {
        let mut n = gmp_natural_to_native(&gmp_n);
        let old_n = n.clone();
        gmp_n.clear_bit(index);
        assert!(gmp_n.is_valid());

        n.clear_bit(index);
        assert!(n.is_valid());
        assert_eq!(gmp_natural_to_native(&gmp_n), n);

        let mut n2 = old_n.clone();
        n2.assign_bit(index, false);
        assert_eq!(n2, n);

        assert!(n <= old_n);
        if old_n.get_bit(index) {
            assert_ne!(n, old_n);
            n.set_bit(index);
            assert_eq!(n, old_n);
        } else {
            assert_eq!(n, old_n);
        }
    };

    for (n, index) in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_u64(n, index);
    }

    for (n, index) in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_u64(n, index);
    }
}
