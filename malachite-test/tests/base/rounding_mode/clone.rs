use common::test_properties_no_limit_exhaustive_no_special;
use malachite_base::round::RoundingMode;
use malachite_test::inputs::base::{pairs_of_rounding_modes, rounding_modes};

#[test]
#[allow(unknown_lints, clone_on_copy)]
fn test_clone() {
    let test = |rm: RoundingMode| {
        let cloned = rm.clone();
        assert_eq!(cloned, rm);
    };
    test(RoundingMode::Down);
    test(RoundingMode::Up);
    test(RoundingMode::Floor);
    test(RoundingMode::Ceiling);
    test(RoundingMode::Nearest);
    test(RoundingMode::Exact);
}

#[test]
fn test_clone_from() {
    let test = |mut x: RoundingMode, y: RoundingMode| {
        x.clone_from(&y);
        assert_eq!(x, y);
    };
    test(RoundingMode::Exact, RoundingMode::Floor);
    test(RoundingMode::Up, RoundingMode::Ceiling);
}

#[test]
#[allow(unknown_lints, clone_on_copy)]
fn clone_and_clone_from_properties() {
    test_properties_no_limit_exhaustive_no_special(rounding_modes, |&rm| {
        assert_eq!(rm.clone(), rm);
    });

    test_properties_no_limit_exhaustive_no_special(pairs_of_rounding_modes, |&(x, y)| {
        let mut mut_x = x;
        mut_x.clone_from(&y);
        assert_eq!(mut_x, y);
    });
}
