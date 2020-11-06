use malachite_base_test_util::generators::rounding_mode_gen;

use malachite_base::rounding_modes::RoundingMode;

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
    rounding_mode_gen().test_properties(|rm| {
        assert_eq!(-(-rm), rm);
    });
}
