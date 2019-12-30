extern crate malachite_base;

use std::str::FromStr;

use malachite_base::round::RoundingMode;

#[test]
fn test_rounding_mode_neg() {
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

#[test]
fn test_rounding_mode_from_str() {
    let test = |s, out| {
        assert_eq!(format!("{:?}", RoundingMode::from_str(s)), out);
    };
    test("Down", "Ok(Down)");
    test("Up", "Ok(Up)");
    test("Floor", "Ok(Floor)");
    test("Ceiling", "Ok(Ceiling)");
    test("Nearest", "Ok(Nearest)");
    test("Exact", "Ok(Exact)");
    test("abc", "Err(\"abc\")");
}
