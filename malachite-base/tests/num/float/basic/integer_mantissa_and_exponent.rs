use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::generators::primitive_float_gen_var_12;
use std::panic::catch_unwind;

#[test]
pub fn test_integer_mantissa_and_exponent() {
    fn test<T: PrimitiveFloat>(x: T, mantissa: u64, exponent: i64) {
        let (actual_mantissa, actual_exponent) = x.integer_mantissa_and_exponent();
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

fn integer_mantissa_and_exponent_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(T::NAN.integer_mantissa_and_exponent());
    assert_panic!(T::POSITIVE_INFINITY.integer_mantissa_and_exponent());
    assert_panic!(T::NEGATIVE_INFINITY.integer_mantissa_and_exponent());
    assert_panic!(T::ZERO.integer_mantissa_and_exponent());
    assert_panic!(T::NEGATIVE_ZERO.integer_mantissa_and_exponent());
}

#[test]
pub fn integer_mantissa_and_exponent_fail() {
    apply_fn_to_primitive_floats!(integer_mantissa_and_exponent_fail_helper);
}

fn integer_mantissa_and_exponent_properties_helper<T: PrimitiveFloat>() {
    primitive_float_gen_var_12::<T>().test_properties(|x| {
        let (mantissa, exponent) = x.integer_mantissa_and_exponent();
        assert_eq!(x.integer_mantissa(), mantissa);
        assert_eq!(x.integer_exponent(), exponent);
        assert_eq!(
            NiceFloat(T::from_integer_mantissa_and_exponent(mantissa, exponent).unwrap()),
            NiceFloat(x.abs())
        );

        assert!(exponent >= T::MIN_EXPONENT);
        assert!(exponent <= T::MAX_EXPONENT);
        assert!(mantissa.significant_bits() <= T::MANTISSA_WIDTH + 1);
        assert!(mantissa.odd());
    });
}

#[test]
fn integer_mantissa_and_exponent_properties() {
    apply_fn_to_primitive_floats!(integer_mantissa_and_exponent_properties_helper);
}
