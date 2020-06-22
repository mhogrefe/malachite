use std::str::FromStr;

use malachite_base::rounding_modes::RoundingMode;
use malachite_base::strings::string_is_subset;

use malachite_test::common::test_properties_no_limit_exhaustive_no_special;
use malachite_test::inputs::base::{rounding_modes, ROUNDING_MODE_CHARS};

#[test]
fn to_string_properties() {
    test_properties_no_limit_exhaustive_no_special(rounding_modes, |&rm| {
        let s = rm.to_string();
        assert_eq!(RoundingMode::from_str(&s), Ok(rm));
        assert!(string_is_subset(&s, ROUNDING_MODE_CHARS));
    });
}
