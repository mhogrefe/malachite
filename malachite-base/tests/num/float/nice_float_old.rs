use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base_test_util::common::{test_cmp_helper, test_eq_helper};

const TEST_STRINGS: [&str; 7] = [
    "-Infinity",
    "-5.0e5",
    "-0.0",
    "NaN",
    "0.0",
    "0.123",
    "Infinity",
];

#[test]
pub fn test_eq() {
    test_eq_helper::<NiceFloat<f32>>(&TEST_STRINGS);
    test_eq_helper::<NiceFloat<f64>>(&TEST_STRINGS);
}

#[test]
pub fn test_cmp() {
    test_cmp_helper::<NiceFloat<f32>>(&TEST_STRINGS);
    test_cmp_helper::<NiceFloat<f64>>(&TEST_STRINGS);
}
