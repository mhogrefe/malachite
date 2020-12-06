use malachite_base::rounding_modes::RoundingMode;
use malachite_base_test_util::common::test_cmp_helper;

#[test]
fn test_cmp() {
    test_cmp_helper::<RoundingMode>(&["Down", "Up", "Floor", "Ceiling", "Nearest", "Exact"]);
}
