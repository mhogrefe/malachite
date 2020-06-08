use malachite_base_test_util::common::test_eq_helper;

use malachite_base::rounding_mode::RoundingMode;

#[test]
fn test_eq() {
    test_eq_helper::<RoundingMode>(&["Down", "Up", "Floor", "Ceiling", "Nearest", "Exact"]);
}
