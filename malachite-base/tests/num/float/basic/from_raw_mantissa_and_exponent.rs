use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base_test_util::generators::unsigned_pair_gen_var_26;
use std::panic::catch_unwind;

pub fn test_from_raw_mantissa_and_exponent() {
    fn test<T: PrimitiveFloat>(mantissa: u64, exponent: u64, x: T) {
        assert_eq!(
            NiceFloat(T::from_raw_mantissa_and_exponent(mantissa, exponent)),
            NiceFloat(x)
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

fn from_raw_mantissa_and_exponent_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(T::from_raw_mantissa_and_exponent(
        u64::power_of_2(T::MANTISSA_WIDTH),
        0
    ));
    assert_panic!(T::from_raw_mantissa_and_exponent(
        0,
        u64::power_of_2(T::EXPONENT_WIDTH)
    ));
}

#[test]
pub fn from_raw_mantissa_and_exponent_fail() {
    apply_fn_to_primitive_floats!(from_raw_mantissa_and_exponent_fail_helper);
}

fn from_raw_mantissa_and_exponent_properties_helper<T: PrimitiveFloat>() {
    unsigned_pair_gen_var_26::<T>().test_properties(|(mantissa, exponent)| {
        let f = T::from_raw_mantissa_and_exponent(mantissa, exponent);
        assert!(f.is_nan() || f.is_sign_positive());
        if !f.is_nan() {
            assert_eq!(f.raw_mantissa_and_exponent(), (mantissa, exponent));
        }
    });
}

#[test]
fn from_raw_mantissa_and_exponent_properties() {
    apply_fn_to_primitive_floats!(from_raw_mantissa_and_exponent_properties_helper);
}
