use malachite_base_test_util::generators::{rounding_mode_gen, rounding_mode_pair_gen};

use malachite_base::rounding_modes::RoundingMode;

#[test]
#[allow(clippy::clone_on_copy)]
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
fn clone_and_clone_from_properties() {
    rounding_mode_gen().test_properties(|rm| {
        assert_eq!(rm.clone(), rm);
    });

    rounding_mode_pair_gen().test_properties(|(x, y)| {
        let mut mut_x = x;
        mut_x.clone_from(&y);
        assert_eq!(mut_x, y);
    });
}
