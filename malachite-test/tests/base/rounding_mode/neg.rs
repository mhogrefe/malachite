use common::test_properties_no_limit_exhaustive_no_special;
use malachite_base::round::RoundingMode;
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
    test_properties_no_limit_exhaustive_no_special(rounding_modes, |&rm| {
        assert_eq!(-(-rm), rm);
    });
}
