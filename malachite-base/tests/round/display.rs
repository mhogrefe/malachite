use malachite_base::round::RoundingMode;

#[test]
fn test_rounding_mode_display() {
    let test = |rm: RoundingMode, out| {
        assert_eq!(rm.to_string(), out);
    };
    test(RoundingMode::Down, "Down");
    test(RoundingMode::Up, "Up");
    test(RoundingMode::Floor, "Floor");
    test(RoundingMode::Ceiling, "Ceiling");
    test(RoundingMode::Nearest, "Nearest");
    test(RoundingMode::Exact, "Exact");
}
