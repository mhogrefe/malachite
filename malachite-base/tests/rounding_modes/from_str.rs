use std::str::FromStr;

use malachite_base::rounding_modes::RoundingMode;

#[test]
fn test_from_str() {
    let test = |s, out| {
        assert_eq!(RoundingMode::from_str(s), out);
    };
    test("Down", Ok(RoundingMode::Down));
    test("Up", Ok(RoundingMode::Up));
    test("Floor", Ok(RoundingMode::Floor));
    test("Ceiling", Ok(RoundingMode::Ceiling));
    test("Nearest", Ok(RoundingMode::Nearest));
    test("Exact", Ok(RoundingMode::Exact));

    test("", Err("".to_string()));
    test("abc", Err("abc".to_string()));
    test("Uptown", Err("Uptown".to_string()));
}
