use common::test_properties_no_special;
use malachite_base::round::RoundingMode;
use malachite_test::inputs::base::{strings, strings_var_1};
use std::str::FromStr;

#[test]
fn test_from_str() {
    let test = |s, out| {
        assert_eq!(format!("{:?}", RoundingMode::from_str(s)), out);
    };
    test("Down", "Ok(Down)");
    test("Up", "Ok(Up)");
    test("Floor", "Ok(Floor)");
    test("Ceiling", "Ok(Ceiling)");
    test("Nearest", "Ok(Nearest)");
    test("Exact", "Ok(Exact)");

    test("", "Err(\"\")");
    test("abc", "Err(\"abc\")");
    test("Uptown", "Err(\"Uptown\")");
}

fn from_str_helper(s: &str) {
    let result = RoundingMode::from_str(s);
    if let Ok(result) = result {
        assert_eq!(result.to_string(), s);
    }
}

#[test]
fn from_str_properties() {
    test_properties_no_special(strings, |s| from_str_helper(s));
    test_properties_no_special(strings_var_1, |s| from_str_helper(s));
}
