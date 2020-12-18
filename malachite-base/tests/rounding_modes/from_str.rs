use std::str::FromStr;

use malachite_base::rounding_modes::RoundingMode;
use malachite_base_test_util::generators::common::GenConfig;
use malachite_base_test_util::generators::{string_gen, string_gen_var_2};

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

#[allow(clippy::needless_pass_by_value)]
fn from_str_helper(s: String) {
    let result = RoundingMode::from_str(&s);
    if let Ok(result) = result {
        assert_eq!(result.to_string(), s);
    }
}

#[test]
fn from_str_properties() {
    let mut config = GenConfig::new();
    config.insert("length_mean_n", 128);
    config.insert("length_mean_d", 1);
    string_gen().test_properties_with_config(&config, from_str_helper);
    string_gen_var_2().test_properties_with_config(&config, from_str_helper);
}
