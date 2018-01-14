use common::{test_eq_helper, LARGE_LIMIT};
use malachite_base::round::RoundingMode;
use malachite_test::common::GenerationMode;
use malachite_test::base::rounding_mode::eq::select_inputs;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::tuples::{lex_triples, random_triples_from_single};

#[test]
fn test_eq() {
    let strings = vec!["Down", "Up", "Floor", "Ceiling", "Nearest", "Exact"];
    test_eq_helper::<RoundingMode>(&strings);
}

#[test]
fn eq_properties() {
    // (x == y) == (y == x)
    let two_rounding_modes = |x: RoundingMode, y: RoundingMode| {
        assert_eq!(x == y, y == x);
    };

    // rm == rm
    let one_rounding_mode = |rm: RoundingMode| {
        assert_eq!(rm, rm);
    };

    // x == y && x == z => x == z
    let three_rounding_modes = |x: RoundingMode, y: RoundingMode, z: RoundingMode| {
        if x == y && x == z {
            assert_eq!(x, z);
        }
    };

    for (x, y) in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_rounding_modes(x, y);
    }

    for (x, y) in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_rounding_modes(x, y);
    }

    for n in exhaustive_rounding_modes().take(LARGE_LIMIT) {
        one_rounding_mode(n);
    }

    for n in random_rounding_modes(&EXAMPLE_SEED).take(LARGE_LIMIT) {
        one_rounding_mode(n);
    }

    for (x, y, z) in lex_triples(
        exhaustive_rounding_modes(),
        exhaustive_rounding_modes(),
        exhaustive_rounding_modes(),
    ).take(LARGE_LIMIT)
    {
        three_rounding_modes(x, y, z);
    }

    for (x, y, z) in
        random_triples_from_single(random_rounding_modes(&EXAMPLE_SEED)).take(LARGE_LIMIT)
    {
        three_rounding_modes(x, y, z);
    }
}
