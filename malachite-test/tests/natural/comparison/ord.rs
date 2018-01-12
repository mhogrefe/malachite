use common::{test_cmp_helper, LARGE_LIMIT};
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_biguint, natural_to_rugint_integer, GenerationMode};
use malachite_test::natural::comparison::ord::select_inputs;
use num::BigUint;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_triples_from_single, random_triples_from_single};
use std::cmp::Ordering;

#[test]
fn test_cmp() {
    let strings = vec![
        "0",
        "1",
        "2",
        "123",
        "999999999999",
        "1000000000000",
        "1000000000001",
    ];
    test_cmp_helper::<Natural>(&strings);
    test_cmp_helper::<BigUint>(&strings);
    test_cmp_helper::<rugint::Integer>(&strings);
}

#[test]
fn cmp_properties() {
    // x.cmp(&y) is equivalent for malachite, num, and rugint.
    // x.cmp(&y) == y.cmp(&x).reverse()
    // x.cmp(&y) == (-y).cmp(-x)
    let two_naturals = |x: Natural, y: Natural| {
        let ord = x.cmp(&y);
        assert_eq!(natural_to_biguint(&x).cmp(&natural_to_biguint(&y)), ord);
        assert_eq!(
            natural_to_rugint_integer(&x).cmp(&natural_to_rugint_integer(&y)),
            ord
        );
        assert_eq!(y.cmp(&x).reverse(), ord);
        assert_eq!((-y).cmp(&(-x)), ord);
    };

    // x == x
    let one_natural = |x: Natural| {
        assert_eq!(x.cmp(&x), Ordering::Equal);
    };

    // x < y && x < z => x < z, x > y && x > z => x > z
    let three_naturals = |x: Natural, y: Natural, z: Natural| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    };

    for (x, y) in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for (x, y) in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
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

    for (x, y, z) in
        random_triples_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(LARGE_LIMIT)
    {
        three_naturals(x, y, z);
    }
}
