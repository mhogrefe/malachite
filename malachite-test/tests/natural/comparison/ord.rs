use common::{LARGE_LIMIT, test_cmp_helper};
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, native_natural_to_num_biguint,
                             native_natural_to_rugint_integer};
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs_from_single, exhaustive_triples_from_single,
                                     random_pairs_from_single, random_triples_from_single};
use std::cmp::Ordering;

#[test]
fn test_cmp() {
    let strings = vec!["0", "1", "2", "123", "999999999999", "1000000000000", "1000000000001"];
    test_cmp_helper::<native::Natural>(&strings);
    test_cmp_helper::<gmp::Natural>(&strings);
    test_cmp_helper::<num::BigUint>(&strings);
    test_cmp_helper::<rugint::Integer>(&strings);
}

#[test]
fn cmp_properties() {
    // x.cmp(&y) is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // x.cmp(&y) == y.cmp(&x).reverse()
    let two_naturals = |gmp_x: gmp::Natural, gmp_y: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        let y = gmp_natural_to_native(&gmp_y);
        let ord = x.cmp(&y);
        assert_eq!(gmp_x.cmp(&gmp_y), ord);
        assert_eq!(native_natural_to_num_biguint(&x).cmp(&native_natural_to_num_biguint(&y)),
                   ord);
        assert_eq!(native_natural_to_rugint_integer(&x).cmp(&native_natural_to_rugint_integer(&y)),
                   ord);
        assert_eq!(y.cmp(&x).reverse(), ord);
    };

    // x == x
    let one_natural = |gmp_x: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        assert_eq!(x.cmp(&x), Ordering::Equal);
    };

    // x < y && x < z => x < z, x > y && x > z => x > z
    let three_naturals = |gmp_x: gmp::Natural, gmp_y: gmp::Natural, gmp_z: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        let y = gmp_natural_to_native(&gmp_y);
        let z = gmp_natural_to_native(&gmp_z);
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    };

    for (x, y) in exhaustive_pairs_from_single(exhaustive_naturals()).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for (x, y) in random_pairs_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for n in exhaustive_naturals().take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in random_naturals(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for (x, y, z) in exhaustive_triples_from_single(exhaustive_naturals()).take(LARGE_LIMIT) {
        three_naturals(x, y, z);
    }

    for (x, y, z) in random_triples_from_single(random_naturals(&EXAMPLE_SEED, 32))
            .take(LARGE_LIMIT) {
        three_naturals(x, y, z);
    }
}
