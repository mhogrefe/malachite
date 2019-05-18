use std::str::FromStr;

use malachite_base::round::RoundingMode;

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
