use malachite_base::num::float::PrimitiveFloat;
use std::panic::catch_unwind;

#[test]
pub fn test_raw_exponent() {
    fn test<T: PrimitiveFloat>(x: T, exponent: u64) {
        assert_eq!(x.raw_exponent(), exponent);
    };

    test::<f32>(0.0, 0);
    test::<f32>(-0.0, 0);
    test::<f32>(f32::NAN, 255);
    test::<f32>(f32::POSITIVE_INFINITY, 255);
    test::<f32>(f32::NEGATIVE_INFINITY, 255);
    test::<f32>(1.0, 127);
    test::<f32>(core::f32::consts::PI, 128);
    test::<f32>(0.1, 123);
    test::<f32>(10.0, 130);
    test::<f32>(f32::MIN_POSITIVE_SUBNORMAL, 0);
    test::<f32>(f32::MAX_SUBNORMAL, 0);
    test::<f32>(f32::MIN_POSITIVE_NORMAL, 1);
    test::<f32>(f32::MAX_FINITE, 254);

    test::<f64>(0.0, 0);
    test::<f64>(-0.0, 0);
    test::<f64>(f64::NAN, 2047);
    test::<f64>(f64::POSITIVE_INFINITY, 2047);
    test::<f64>(f64::NEGATIVE_INFINITY, 2047);
    test::<f64>(1.0, 1023);
    test::<f64>(core::f64::consts::PI, 1024);
    test::<f64>(0.1, 1019);
    test::<f64>(10.0, 1026);
    test::<f64>(f64::MIN_POSITIVE_SUBNORMAL, 0);
    test::<f64>(f64::MAX_SUBNORMAL, 0);
    test::<f64>(f64::MIN_POSITIVE_NORMAL, 1);
    test::<f64>(f64::MAX_FINITE, 2046);
}

#[test]
pub fn test_exponent() {
    fn test<T: PrimitiveFloat>(x: T, exponent: i64) {
        assert_eq!(x.exponent(), exponent);
    };

    test::<f32>(1.0, 0);
    test::<f32>(core::f32::consts::PI, 1);
    test::<f32>(0.1, -4);
    test::<f32>(10.0, 3);
    test::<f32>(f32::MIN_POSITIVE_SUBNORMAL, -127);
    test::<f32>(f32::MAX_SUBNORMAL, -127);
    test::<f32>(f32::MIN_POSITIVE_NORMAL, -126);
    test::<f32>(f32::MAX_FINITE, 127);

    test::<f64>(1.0, 0);
    test::<f64>(core::f64::consts::PI, 1);
    test::<f64>(0.1, -4);
    test::<f64>(10.0, 3);
    test::<f64>(f64::MIN_POSITIVE_SUBNORMAL, -1023);
    test::<f64>(f64::MAX_SUBNORMAL, -1023);
    test::<f64>(f64::MIN_POSITIVE_NORMAL, -1022);
    test::<f64>(f64::MAX_FINITE, 1023);
}

fn exponent_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(T::NAN.exponent());
    assert_panic!(T::POSITIVE_INFINITY.exponent());
    assert_panic!(T::NEGATIVE_INFINITY.exponent());
    assert_panic!(T::ZERO.exponent());
    assert_panic!(T::NEGATIVE_ZERO.exponent());
}

#[test]
pub fn exponent_fail() {
    apply_fn_to_primitive_floats!(exponent_fail_helper);
}
