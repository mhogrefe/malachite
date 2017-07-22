use common::{LARGE_LIMIT, test_eq_helper};
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

#[test]
fn test_eq() {
    let strings = vec!["0", "1", "-1", "2", "-2", "123", "-123", "1000000000000", "-1000000000000"];
    test_eq_helper::<native::Integer>(&strings);
    test_eq_helper::<gmp::Integer>(&strings);
    test_eq_helper::<num::BigInt>(&strings);
    test_eq_helper::<rugint::Integer>(&strings);
}

#[test]
fn eq_properties() {
    // x == y is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // (x == y) == (y == x)
    let two_integers = |gmp_x: gmp::Integer, gmp_y: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let y = gmp_integer_to_native(&gmp_y);
        let eq = x == y;
        assert_eq!(gmp_x == gmp_y, eq);
        assert_eq!(native_integer_to_num_bigint(&x) == native_integer_to_num_bigint(&y),
                   eq);
        assert_eq!(native_integer_to_rugint(&x) == native_integer_to_rugint(&y),
                   eq);
        assert_eq!(y == x, eq);
    };

    // x == x
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        assert_eq!(x, x);
    };

    // x == y && x == z => x == z
    let three_integers = |gmp_x: gmp::Integer, gmp_y: gmp::Integer, gmp_z: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let y = gmp_integer_to_native(&gmp_y);
        let z = gmp_integer_to_native(&gmp_z);
        if x == y && x == z {
            assert_eq!(x, z);
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
            .take(LARGE_LIMIT) {
        three_integers(x, y, z);
    }
}
