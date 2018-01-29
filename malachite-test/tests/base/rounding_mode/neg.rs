use common::LARGE_LIMIT;
use malachite_base::round::RoundingMode;
use malachite_test::common::GenerationMode;
use malachite_test::inputs::base::rounding_modes;

#[test]
fn test_neg() {
    let test = |rm: RoundingMode, out| {
        assert_eq!(-rm, out);
    };
    test(RoundingMode::Down, RoundingMode::Down);
    test(RoundingMode::Up, RoundingMode::Up);
    test(RoundingMode::Floor, RoundingMode::Ceiling);
    test(RoundingMode::Ceiling, RoundingMode::Floor);
    test(RoundingMode::Nearest, RoundingMode::Nearest);
    test(RoundingMode::Exact, RoundingMode::Exact);
}

#[test]
fn neg_properties() {
    // --x == x
    let one_rounding_mode = |rm: RoundingMode| {
        assert_eq!(-(-rm), rm);
    };

    for n in rounding_modes(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_rounding_mode(n);
    }

    for n in rounding_modes(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_rounding_mode(n);
    }
}
