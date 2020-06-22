use malachite_base::rounding_modes::RoundingMode;

#[test]
#[allow(unknown_lints, clippy::clone_on_copy)]
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
