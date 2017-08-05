use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_rugint,
                             rugint_integer_to_native};
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{log_pairs, random_pairs};
use std::str::FromStr;

#[test]
fn test_flip_bit() {
    let test = |u, index, out| {
        let mut n = native::Integer::from_str(u).unwrap();
        n.flip_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Integer::from_str(u).unwrap();
        n.flip_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rugint::Integer::from_str(u).unwrap();
        n.invert_bit(index as u32);
        assert_eq!(n.to_string(), out);
    };
    test("0", 10, "1024");
    test("1024", 10, "0");
    test("100", 0, "101");
    test("101", 0, "100");
    test("1000000000000", 10, "1000000001024");
    test("1000000001024", 10, "1000000000000");
    test("1000000000000", 100, "1267650600228229402496703205376");
    test("1267650600228229402496703205376", 100, "1000000000000");
    test("5", 100, "1267650600228229401496703205381");
    test("1267650600228229401496703205381", 100, "5");
}

#[test]
fn flip_bit_properties() {
    // n.flip_bit(index) is equivalent for malachite-gmp, malachite-native, and rugint.
    // Flipping a bit once always changes a number.
    // Flipping the same bit twice leaves a number unchanged.
    let integer_and_u64 = |mut gmp_n: gmp::Integer, index: u64| {
        let mut n = gmp_integer_to_native(&gmp_n);
        let old_n = n.clone();
        gmp_n.flip_bit(index);
        assert!(gmp_n.is_valid());

        n.flip_bit(index);
        assert!(n.is_valid());
        assert_ne!(n, old_n);
        assert_eq!(gmp_integer_to_native(&gmp_n), n);

        let mut rugint_n = native_integer_to_rugint(&old_n);
        rugint_n.invert_bit(index as u32);
        assert_eq!(rugint_integer_to_native(&rugint_n), n);

        n.flip_bit(index);
        assert_eq!(n, old_n);
    };

    for (n, index) in log_pairs(exhaustive_integers(), exhaustive_u::<u64>()).take(LARGE_LIMIT) {
        integer_and_u64(n, index);
    }

    for (n, index) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32).map(|i| i as u64)),
    ).take(LARGE_LIMIT)
    {
        integer_and_u64(n, index);
    }
}
