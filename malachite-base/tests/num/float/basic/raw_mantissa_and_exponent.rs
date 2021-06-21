use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::generators::primitive_float_gen;

#[test]
pub fn test_raw_mantissa_and_exponent() {
    fn test<T: PrimitiveFloat>(x: T, mantissa: u64, exponent: u64) {
        assert_eq!(x.raw_mantissa(), mantissa);
        assert_eq!(x.raw_exponent(), exponent);
        assert_eq!(x.raw_mantissa_and_exponent(), (mantissa, exponent));
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
pub fn test_raw_exponent() {
    fn test<T: PrimitiveFloat>(x: T, exponent: u64) {
        assert_eq!(x.raw_exponent(), exponent);
    }

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

fn raw_mantissa_and_exponent_properties_helper<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|x| {
        let (mantissa, exponent) = x.raw_mantissa_and_exponent();
        assert_eq!(x.raw_mantissa(), mantissa);
        assert_eq!(x.raw_exponent(), exponent);
        assert_eq!(
            NiceFloat(T::from_raw_mantissa_and_exponent(mantissa, exponent)),
            NiceFloat(x.abs())
        );

        assert!(exponent.significant_bits() <= T::EXPONENT_WIDTH);
        assert!(mantissa.significant_bits() <= T::MANTISSA_WIDTH);
    });
}

#[test]
fn raw_mantissa_and_exponent_properties() {
    apply_fn_to_primitive_floats!(raw_mantissa_and_exponent_properties_helper);
}
