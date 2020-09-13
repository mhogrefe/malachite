use std::str::FromStr;

use malachite_base_test_util::generators::rounding_mode_gen;
use malachite_base_test_util::rounding_modes::ROUNDING_MODE_CHARS;

use malachite_base::rounding_modes::RoundingMode;
use malachite_base::strings::string_is_subset;

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
    rounding_mode_gen().test_properties(|rm| {
        let s = rm.to_string();
        assert_eq!(RoundingMode::from_str(&s), Ok(rm));
        assert!(string_is_subset(&s, ROUNDING_MODE_CHARS));
    });
}
