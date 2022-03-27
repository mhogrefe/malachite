use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base_test_util::generators::rounding_mode_gen;

#[test]
fn test_neg() {
    let test = |mut rm: RoundingMode, out| {
        assert_eq!(-rm, out);
        rm.neg_assign();
        assert_eq!(rm, out);
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
        let mut rm_alt = rm;
        rm_alt.neg_assign();
        assert_eq!(rm_alt, -rm);
    });
}
