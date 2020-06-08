use malachite_base::rounding_mode::RoundingMode;

#[test]
fn test_to_string() {
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
