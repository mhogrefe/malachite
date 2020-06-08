use std::str::FromStr;

use malachite_base::rounding_mode::RoundingMode;

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::{strings, strings_var_1};

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
