use malachite_base::named::Named;
use malachite_base::num::basic::traits::{NegativeOne, One, Two, Zero};
use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;

macro_rules! test_common_constants {
    ($t: ident) => {
        assert_eq!(NiceFloat($t::ZERO), 0.0);
        assert_eq!(NiceFloat($t::ONE), 1.0);
        assert_eq!(NiceFloat($t::TWO), 2.0);
        assert_eq!(NiceFloat($t::NEGATIVE_ONE), -1.0);
        assert_eq!(NiceFloat($t::NEGATIVE_ZERO), -0.0);
    };
}

#[allow(clippy::float_cmp)]
#[test]
fn test_constants() {
    apply_to_primitive_floats!(test_common_constants);
}

#[allow(clippy::float_cmp)]
#[test]
fn test_other_constants() {
    assert_eq!(f32::WIDTH, 32);
    assert_eq!(f32::EXPONENT_WIDTH, 8);
    assert_eq!(f32::MANTISSA_WIDTH, 23);
    assert_eq!(f32::MIN_NORMAL_EXPONENT, -126);
    assert_eq!(f32::MIN_EXPONENT, -149);
    assert_eq!(f32::MAX_EXPONENT, 127);
    assert_eq!(NiceFloat(f32::MIN_POSITIVE_SUBNORMAL), 1.0e-45);
    assert_eq!(NiceFloat(f32::MAX_SUBNORMAL), 1.1754942e-38);
    assert_eq!(NiceFloat(f32::MIN_POSITIVE_NORMAL), 1.1754944e-38);
    assert_eq!(NiceFloat(f32::MAX_FINITE), 3.4028235e38);
    assert_eq!(NiceFloat(f32::POSITIVE_INFINITY), std::f32::INFINITY);
    assert_eq!(NiceFloat(f32::NEGATIVE_INFINITY), std::f32::NEG_INFINITY);
    assert_eq!(NiceFloat(f32::NAN), std::f32::NAN);
    assert_eq!(f32::SMALLEST_UNREPRESENTABLE_UINT, 0x1000001);
    assert_eq!(f32::LARGEST_ORDERED_REPRESENTATION, 0xff000001);

    assert_eq!(f64::WIDTH, 64);
    assert_eq!(f64::EXPONENT_WIDTH, 11);
    assert_eq!(f64::MANTISSA_WIDTH, 52);
    assert_eq!(f64::MIN_NORMAL_EXPONENT, -1022);
    assert_eq!(f64::MIN_EXPONENT, -1074);
    assert_eq!(f64::MAX_EXPONENT, 1023);
    assert_eq!(NiceFloat(f64::MIN_POSITIVE_SUBNORMAL), 5.0e-324);
    assert_eq!(NiceFloat(f64::MAX_SUBNORMAL), 2.225073858507201e-308);
    assert_eq!(NiceFloat(f64::MIN_POSITIVE_NORMAL), 2.2250738585072014e-308);
    assert_eq!(NiceFloat(f64::MAX_FINITE), 1.7976931348623157e308);
    assert_eq!(NiceFloat(f64::POSITIVE_INFINITY), std::f64::INFINITY);
    assert_eq!(NiceFloat(f64::NEGATIVE_INFINITY), std::f64::NEG_INFINITY);
    assert_eq!(NiceFloat(f64::NAN), std::f64::NAN);
    assert_eq!(f32::SMALLEST_UNREPRESENTABLE_UINT, 0x1000001);
    assert_eq!(f64::LARGEST_ORDERED_REPRESENTATION, 0xffe0000000000001);
}

#[test]
pub fn test_named() {
    fn test<T: Named>(out: &str) {
        assert_eq!(T::NAME, out);
    }
    test::<f32>("f32");
    test::<f64>("f64");
}
