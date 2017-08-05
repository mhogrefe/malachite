use common::{test_cmp_helper, LARGE_LIMIT};
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_num_bigint,
                             native_integer_to_rugint};
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::tuples::{exhaustive_pairs_from_single, exhaustive_triples_from_single,
                                     random_pairs_from_single, random_triples_from_single};
use std::cmp::Ordering;

#[test]
fn test_ord() {
    let strings = vec![
        "-1000000000001",
        "-1000000000000",
        "-999999999999",
        "-123",
        "-2",
        "-1",
        "0",
        "1",
        "2",
        "123",
        "999999999999",
        "1000000000000",
        "1000000000001",
    ];
    test_cmp_helper::<native::Integer>(&strings);
    test_cmp_helper::<gmp::Integer>(&strings);
    test_cmp_helper::<num::BigInt>(&strings);
    test_cmp_helper::<rugint::Integer>(&strings);
}

#[test]
fn cmp_properties() {
    // x.cmp(&y) is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // x.cmp(&y) == y.cmp(&x).reverse()
    // x.cmp(&y) == (-y).cmp(-x)
    let two_integers = |gmp_x: gmp::Integer, gmp_y: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let y = gmp_integer_to_native(&gmp_y);
        let ord = x.cmp(&y);
        assert_eq!(gmp_x.cmp(&gmp_y), ord);
        assert_eq!(
            native_integer_to_num_bigint(&x).cmp(&native_integer_to_num_bigint(&y)),
            ord
        );
        assert_eq!(
            native_integer_to_rugint(&x).cmp(&native_integer_to_rugint(&y)),
            ord
        );
        assert_eq!(y.cmp(&x).reverse(), ord);
        assert_eq!((-y).cmp(&(-x)), ord);
    };

    // x == x
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        assert_eq!(x.cmp(&x), Ordering::Equal);
    };

    // x < y && x < z => x < z, x > y && x > z => x > z
    let three_integers = |gmp_x: gmp::Integer, gmp_y: gmp::Integer, gmp_z: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let y = gmp_integer_to_native(&gmp_y);
        let z = gmp_integer_to_native(&gmp_z);
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    };

    for (x, y) in exhaustive_pairs_from_single(exhaustive_integers()).take(LARGE_LIMIT) {
        two_integers(x, y);
    }

    for (x, y) in random_pairs_from_single(random_integers(&EXAMPLE_SEED, 32)).take(LARGE_LIMIT) {
        two_integers(x, y);
    }

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for (x, y, z) in exhaustive_triples_from_single(exhaustive_integers()).take(LARGE_LIMIT) {
        three_integers(x, y, z);
    }

    for (x, y, z) in random_triples_from_single(random_integers(&EXAMPLE_SEED, 32))
        .take(LARGE_LIMIT)
    {
        three_integers(x, y, z);
    }
}
