use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, native_natural_to_num_biguint,
                             num_biguint_to_native_natural, GenerationMode};
use malachite_test::natural::logic::set_bit::{num_set_bit, select_inputs};
use num;
use std::str::FromStr;

#[test]
fn test_set_bit() {
    let test = |u, index, out| {
        let mut n = native::Natural::from_str(u).unwrap();
        n.set_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n.set_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = num::BigUint::from_str(u).unwrap();
        num_set_bit(&mut n, index);
        assert_eq!(n.to_string(), out);
    };
    test("0", 10, "1024");
    test("100", 0, "101");
    test("1000000000000", 10, "1000000001024");
    test("1000000000000", 100, "1267650600228229402496703205376");
    test("5", 100, "1267650600228229401496703205381");
}

#[test]
fn set_bit_properties() {
    // n.set_bit(index) is equivalent for malachite-gmp, malachite-native, and num.
    // n.set_bit(index) is equivalent to n.assign_bit(index, true).
    // n.set_bit(index); n != 0
    // Setting a bit does not decrease n.
    // If n.get_bit(index), setting at index won't do anything.
    // If !n.get_bit(index), setting and then clearing at index won't do anything.
    let natural_and_u64 = |mut gmp_n: gmp::Natural, index: u64| {
        let mut n = gmp_natural_to_native(&gmp_n);
        let old_n = n.clone();
        gmp_n.set_bit(index);
        assert!(gmp_n.is_valid());

        n.set_bit(index);
        assert!(n.is_valid());
        assert_eq!(gmp_natural_to_native(&gmp_n), n);

        let mut n2 = old_n.clone();
        n2.assign_bit(index, true);
        assert_eq!(n2, n);

        let mut num_n = native_natural_to_num_biguint(&old_n);
        num_set_bit(&mut num_n, index);
        assert_eq!(num_biguint_to_native_natural(&num_n), n);

        assert_ne!(n, 0);
        assert!(n >= old_n);
        if old_n.get_bit(index) {
            assert_eq!(n, old_n);
        } else {
            assert_ne!(n, old_n);
            n.clear_bit(index);
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
