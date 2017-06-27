use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, native_natural_to_rugint_integer,
                             rugint_integer_to_native_natural};
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::bools::exhaustive_bools;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, log_pairs, random_triples};
use std::str::FromStr;

#[test]
fn test_assign_bit() {
    let test = |u, index, bit, out| {
        let mut n = native::Natural::from_str(u).unwrap();
        n.assign_bit(index, bit);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n.assign_bit(index, bit);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rugint::Integer::from_str(u).unwrap();
        n.set_bit(index as u32, bit);
        assert_eq!(n.to_string(), out);
    };
    test("0", 10, true, "1024");
    test("100", 0, true, "101");
    test("1000000000000", 10, true, "1000000001024");
    test("1000000000000",
         100,
         true,
         "1267650600228229402496703205376");
    test("5", 100, true, "1267650600228229401496703205381");
    test("0", 10, false, "0");
    test("0", 100, false, "0");
    test("1024", 10, false, "0");
    test("101", 0, false, "100");
    test("1000000001024", 10, false, "1000000000000");
    test("1000000001024", 100, false, "1000000001024");
    test("1267650600228229402496703205376",
         100,
         false,
         "1000000000000");
    test("1267650600228229401496703205381", 100, false, "5");
}

#[test]
fn assign_bit_properties() {
    // n.assign_bit(index) is equivalent for malachite-gmp and malachite-native.
    let natural_u64_and_bool = |mut gmp_n: gmp::Natural, index: u64, bit: bool| {
        let mut n = gmp_natural_to_native(&gmp_n);
        let old_n = n.clone();
        gmp_n.assign_bit(index, bit);
        assert!(gmp_n.is_valid());

        n.assign_bit(index, bit);
        assert!(n.is_valid());
        assert_eq!(gmp_natural_to_native(&gmp_n), n);

        let mut rugint_n = native_natural_to_rugint_integer(&old_n);
        rugint_n.set_bit(index as u32, bit);
        assert_eq!(rugint_integer_to_native_natural(&rugint_n), n);
    };

    for ((n, index), bit) in
        log_pairs(exhaustive_pairs(exhaustive_naturals(), exhaustive_u::<u64>()),
                  exhaustive_bools())
                .take(LARGE_LIMIT) {
        natural_u64_and_bool(n, index, bit);
    }

    for (n, index, bit) in random_triples(&EXAMPLE_SEED,
                                          &(|seed| random_naturals(seed, 32)),
                                          &(|seed| {
                                                natural_u32s_geometric(seed, 32).map(|i| i as u64)
                                            }),
                                          &(|seed| random_x(seed)))
                .take(LARGE_LIMIT) {
        natural_u64_and_bool(n, index, bit);
    }
}
