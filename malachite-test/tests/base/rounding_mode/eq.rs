use common::{test_eq_helper, LARGE_LIMIT};
use malachite_base::round::RoundingMode;
use malachite_test::common::GenerationMode;
use malachite_test::inputs::base::{pairs_of_rounding_modes, rounding_modes,
                                   triples_of_rounding_modes};

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

    for (x, y) in pairs_of_rounding_modes(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_rounding_modes(x, y);
    }

    for (x, y) in pairs_of_rounding_modes(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_rounding_modes(x, y);
    }

    for n in rounding_modes(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_rounding_mode(n);
    }

    for n in rounding_modes(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_rounding_mode(n);
    }

    for (x, y, z) in triples_of_rounding_modes(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        three_rounding_modes(x, y, z);
    }

    for (x, y, z) in triples_of_rounding_modes(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        three_rounding_modes(x, y, z);
    }
}
