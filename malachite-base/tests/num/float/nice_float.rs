use malachite_base_test_util::common::{test_cmp_helper, test_eq_helper};
use malachite_base_test_util::num::float::nice_float::{FmtRyuString, NiceFloat};

use malachite_base::num::floats::PrimitiveFloat;

#[test]
pub fn test_to_string() {
    fn test<T: PrimitiveFloat + FmtRyuString>(x: T, out: &str) {
        assert_eq!(NiceFloat(x).to_string(), out);
    };
    test::<f32>(f32::NAN, "NaN");
    test::<f32>(f32::POSITIVE_INFINITY, "Infinity");
    test::<f32>(f32::NEGATIVE_INFINITY, "-Infinity");
    test::<f32>(0.0, "0.0");
    test::<f32>(-0.0, "-0.0");
    test::<f32>(1.0, "1.0");
    test::<f32>(-1.0, "-1.0");
    test::<f32>(123.0, "123.0");
    test::<f32>(0.123, "0.123");
    test::<f32>(1000.0, "1000.0");
    test::<f32>(1000000.0, "1000000.0");
    test::<f32>(1.0e20, "1.0e20");
    test::<f32>(f32::MIN_POSITIVE_SUBNORMAL, "1.0e-45");
    test::<f32>(f32::MAX_SUBNORMAL, "1.1754942e-38");
    test::<f32>(f32::MIN_POSITIVE_NORMAL, "1.1754944e-38");
    test::<f32>(f32::MAX_FINITE, "3.4028235e38");
    test::<f32>(2.0f32.sqrt(), "1.4142135");
    test::<f32>(std::f32::consts::E, "2.7182817");
    test::<f32>(std::f32::consts::PI, "3.1415927");

    test::<f64>(f64::NAN, "NaN");
    test::<f64>(f64::POSITIVE_INFINITY, "Infinity");
    test::<f64>(f64::NEGATIVE_INFINITY, "-Infinity");
    test::<f64>(0.0, "0.0");
    test::<f64>(-0.0, "-0.0");
    test::<f64>(1.0, "1.0");
    test::<f64>(-1.0, "-1.0");
    test::<f64>(123.0, "123.0");
    test::<f64>(0.123, "0.123");
    test::<f64>(1000.0, "1000.0");
    test::<f64>(1000000.0, "1000000.0");
    test::<f64>(1.0e100, "1.0e100");
    test::<f64>(f64::MIN_POSITIVE_SUBNORMAL, "5.0e-324");
    test::<f64>(f64::MAX_SUBNORMAL, "2.225073858507201e-308");
    test::<f64>(f64::MIN_POSITIVE_NORMAL, "2.2250738585072014e-308");
    test::<f64>(f64::MAX_FINITE, "1.7976931348623157e308");
    test::<f64>(2.0f64.sqrt(), "1.4142135623730951");
    test::<f64>(std::f64::consts::E, "2.718281828459045");
    test::<f64>(std::f64::consts::PI, "3.141592653589793");
}

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
