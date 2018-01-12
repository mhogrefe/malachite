use common::{test_eq_helper, LARGE_LIMIT};
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_biguint, natural_to_rugint_integer, GenerationMode};
use malachite_test::natural::comparison::eq::select_inputs;
use num::BigUint;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_triples_from_single, random_triples_from_single};

#[test]
fn test_eq() {
    let strings = vec!["0", "1", "2", "123", "1000000000000"];
    test_eq_helper::<Natural>(&strings);
    test_eq_helper::<BigUint>(&strings);
    test_eq_helper::<rugint::Integer>(&strings);
}

#[test]
fn eq_properties() {
    // x == y is equivalent for malachite, num, and rugint.
    // (x == y) == (y == x)
    let two_naturals = |x: Natural, y: Natural| {
        let eq = x == y;
        assert_eq!(natural_to_biguint(&x) == natural_to_biguint(&y), eq);
        assert_eq!(
            natural_to_rugint_integer(&x) == natural_to_rugint_integer(&y),
            eq
        );
        assert_eq!(y == x, eq);
    };

    // x == x
    let one_natural = |x: Natural| {
        assert_eq!(x, x);
    };

    // x == y && x == z => x == z
    let three_naturals = |x: Natural, y: Natural, z: Natural| {
        if x == y && x == z {
            assert_eq!(x, z);
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
