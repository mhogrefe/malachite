use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;

#[allow(clippy::decimal_literal_representation)]
pub fn test_from_raw_mantissa_and_exponent() {
    fn test<T: PrimitiveFloat>(mantissa: T::UnsignedOfEqualWidth, exponent: u64, x: T) {
        assert_eq!(
            NiceFloat(T::from_raw_mantissa_and_exponent(mantissa, exponent)),
            x
        );
    }
    test::<f32>(0, 0, 0.0);
    test::<f32>(0x400000, 255, f32::NAN);
    test::<f32>(0, 255, f32::POSITIVE_INFINITY);
    test::<f32>(0, 127, 1.0);
    test::<f32>(4788187, 128, core::f32::consts::PI);
    test::<f32>(5033165, 123, 0.1);
    test::<f32>(2097152, 130, 10.0);
    test::<f32>(1, 0, f32::MIN_POSITIVE_SUBNORMAL);
    test::<f32>(0x7fffff, 0, f32::MAX_SUBNORMAL);
    test::<f32>(0, 1, f32::MIN_POSITIVE_NORMAL);
    test::<f32>(0x7fffff, 254, f32::MAX_FINITE);

    test::<f64>(0, 0, 0.0);
    test::<f64>(0x8000000000000, 2047, f64::NAN);
    test::<f64>(0, 2047, f64::POSITIVE_INFINITY);
    test::<f64>(0, 1023, 1.0);
    test::<f64>(2570638124657944, 1024, core::f64::consts::PI);
    test::<f64>(2702159776422298, 1019, 0.1);
    test::<f64>(1125899906842624, 1026, 10.0);
    test::<f64>(1, 0, f64::MIN_POSITIVE_SUBNORMAL);
    test::<f64>(0xfffffffffffff, 0, f64::MAX_SUBNORMAL);
    test::<f64>(0, 1, f64::MIN_POSITIVE_NORMAL);
    test::<f64>(0xfffffffffffff, 2046, f64::MAX_FINITE);
}

//TODO failure tests

#[test]
pub fn test_from_adjusted_mantissa_and_exponent() {
    fn test<T: PrimitiveFloat>(mantissa: T::UnsignedOfEqualWidth, exponent: i64, x: Option<T>) {
        assert_eq!(
            T::from_adjusted_mantissa_and_exponent(mantissa, exponent).map(NiceFloat),
            x.map(NiceFloat)
        );
    }
    test::<f32>(0, 5, Some(0.0));
    test::<f32>(1, 0, Some(1.0));
    test::<f32>(4, -2, Some(1.0));
    test::<f32>(13176795, -22, Some(core::f32::consts::PI));
    test::<f32>(13421773, -27, Some(0.1));
    test::<f32>(5, 1, Some(10.0));
    test::<f32>(1, -149, Some(f32::MIN_POSITIVE_SUBNORMAL));
    test::<f32>(4, -151, Some(f32::MIN_POSITIVE_SUBNORMAL));
    test::<f32>(0x7fffff, -149, Some(f32::MAX_SUBNORMAL));
    test::<f32>(1, -126, Some(f32::MIN_POSITIVE_NORMAL));
    test::<f32>(0xffffff, 104, Some(f32::MAX_FINITE));
    test::<f32>(5, 10000, None);
    test::<f32>(5, -10000, None);
    test::<f32>(u32::MAX, -32, None); // precision too high
    test::<f32>(3, -150, None); // precision too high

    test::<f64>(0, 5, Some(0.0));
    test::<f64>(1, 0, Some(1.0));
    test::<f64>(4, -2, Some(1.0));
    test::<f64>(884279719003555, -48, Some(core::f64::consts::PI));
    test::<f64>(3602879701896397, -55, Some(0.1));
    test::<f64>(5, 1, Some(10.0));
    test::<f64>(1, -1074, Some(f64::MIN_POSITIVE_SUBNORMAL));
    test::<f64>(4, -1076, Some(f64::MIN_POSITIVE_SUBNORMAL));
    test::<f64>(0xfffffffffffff, -1074, Some(f64::MAX_SUBNORMAL));
    test::<f64>(1, -1022, Some(f64::MIN_POSITIVE_NORMAL));
    test::<f64>(0x1fffffffffffff, 971, Some(f64::MAX_FINITE));
    test::<f64>(5, 10000, None);
    test::<f64>(5, -10000, None);
    test::<f64>(u64::MAX, -64, None); // precision too high
    test::<f64>(3, -1075, None); // precision too high
}
