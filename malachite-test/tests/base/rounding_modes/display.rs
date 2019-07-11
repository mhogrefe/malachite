use common::test_properties_no_limit_exhaustive_no_special;
use malachite_base::round::RoundingMode;
use malachite_base::strings::string_is_subset;
use malachite_test::inputs::base::{rounding_modes, ROUNDING_MODE_CHARS};
use std::str::FromStr;

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

#[test]
fn to_string_properties() {
    test_properties_no_limit_exhaustive_no_special(rounding_modes, |&rm| {
        let s = rm.to_string();
        assert_eq!(RoundingMode::from_str(&s), Ok(rm));
        assert!(string_is_subset(&s, ROUNDING_MODE_CHARS));
    });
}
