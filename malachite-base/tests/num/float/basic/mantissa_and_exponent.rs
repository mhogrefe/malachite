use malachite_base::num::float::PrimitiveFloat;
use std::panic::catch_unwind;

#[allow(clippy::decimal_literal_representation)]
#[test]
pub fn test_raw_mantissa_and_exponent() {
    fn test<T: PrimitiveFloat>(x: T, mantissa: T::UnsignedOfEqualWidth, exponent: u64) {
        let (actual_mantissa, actual_exponent) = x.raw_mantissa_and_exponent();
        assert_eq!(actual_mantissa, mantissa);
        assert_eq!(actual_exponent, exponent);
    }
    test::<f32>(0.0, 0, 0);
    test::<f32>(-0.0, 0, 0);
    test::<f32>(f32::NAN, 0x400000, 255);
    test::<f32>(f32::POSITIVE_INFINITY, 0, 255);
    test::<f32>(f32::NEGATIVE_INFINITY, 0, 255);
    test::<f32>(1.0, 0, 127);
    test::<f32>(core::f32::consts::PI, 4788187, 128);
    test::<f32>(0.1, 5033165, 123);
    test::<f32>(10.0, 2097152, 130);
    test::<f32>(f32::MIN_POSITIVE_SUBNORMAL, 1, 0);
    test::<f32>(f32::MAX_SUBNORMAL, 0x7fffff, 0);
    test::<f32>(f32::MIN_POSITIVE_NORMAL, 0, 1);
    test::<f32>(f32::MAX_FINITE, 0x7fffff, 254);

    test::<f64>(0.0, 0, 0);
    test::<f64>(-0.0, 0, 0);
    test::<f64>(f64::NAN, 0x8000000000000, 2047);
    test::<f64>(f64::POSITIVE_INFINITY, 0, 2047);
    test::<f64>(f64::NEGATIVE_INFINITY, 0, 2047);
    test::<f64>(1.0, 0, 1023);
    test::<f64>(core::f64::consts::PI, 2570638124657944, 1024);
    test::<f64>(0.1, 2702159776422298, 1019);
    test::<f64>(10.0, 1125899906842624, 1026);
    test::<f64>(f64::MIN_POSITIVE_SUBNORMAL, 1, 0);
    test::<f64>(f64::MAX_SUBNORMAL, 0xfffffffffffff, 0);
    test::<f64>(f64::MIN_POSITIVE_NORMAL, 0, 1);
    test::<f64>(f64::MAX_FINITE, 0xfffffffffffff, 2046);
}

#[test]
pub fn test_adjusted_mantissa_and_exponent() {
    fn test<T: PrimitiveFloat>(x: T, mantissa: T::UnsignedOfEqualWidth, exponent: i64) {
        let (actual_mantissa, actual_exponent) = x.adjusted_mantissa_and_exponent();
        assert_eq!(actual_mantissa, mantissa);
        assert_eq!(actual_exponent, exponent);
    }
    test::<f32>(1.0, 1, 0);
    test::<f32>(core::f32::consts::PI, 13176795, -22);
    test::<f32>(0.1, 13421773, -27);
    test::<f32>(10.0, 5, 1);
    test::<f32>(f32::MIN_POSITIVE_SUBNORMAL, 1, -149);
    test::<f32>(f32::MAX_SUBNORMAL, 0x7fffff, -149);
    test::<f32>(f32::MIN_POSITIVE_NORMAL, 1, -126);
    test::<f32>(f32::MAX_FINITE, 0xffffff, 104);

    test::<f64>(1.0, 1, 0);
    test::<f64>(core::f64::consts::PI, 884279719003555, -48);
    test::<f64>(0.1, 3602879701896397, -55);
    test::<f64>(10.0, 5, 1);
    test::<f64>(f64::MIN_POSITIVE_SUBNORMAL, 1, -1074);
    test::<f64>(f64::MAX_SUBNORMAL, 0xfffffffffffff, -1074);
    test::<f64>(f64::MIN_POSITIVE_NORMAL, 1, -1022);
    test::<f64>(f64::MAX_FINITE, 0x1fffffffffffff, 971);
}

fn adjusted_mantissa_and_exponent_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(T::NAN.adjusted_mantissa_and_exponent());
    assert_panic!(T::POSITIVE_INFINITY.adjusted_mantissa_and_exponent());
    assert_panic!(T::NEGATIVE_INFINITY.adjusted_mantissa_and_exponent());
    assert_panic!(T::ZERO.adjusted_mantissa_and_exponent());
    assert_panic!(T::NEGATIVE_ZERO.adjusted_mantissa_and_exponent());
}

#[test]
pub fn adjusted_mantissa_and_exponent_fail() {
    apply_fn_to_primitive_floats!(adjusted_mantissa_and_exponent_fail_helper);
}
