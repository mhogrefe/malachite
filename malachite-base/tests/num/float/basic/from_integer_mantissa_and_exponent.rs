use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base_test_util::generators::{
    unsigned_signed_pair_gen_var_1, unsigned_signed_pair_gen_var_2,
};

#[test]
pub fn test_from_integer_mantissa_and_exponent() {
    fn test<T: PrimitiveFloat>(mantissa: u64, exponent: i64, x: Option<T>) {
        assert_eq!(
            T::from_integer_mantissa_and_exponent(mantissa, exponent).map(NiceFloat),
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
    test::<f32>(1, 127, Some(1.7014118e38));
    test::<f32>(5, 10000, None);
    test::<f32>(5, -10000, None);
    test::<f32>(u64::MAX, -32, None); // precision too high
    test::<f32>(3, -150, None); // precision too high
    test::<f32>(1, 128, None); // precision too high

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
    test::<f64>(1, 1023, Some(8.98846567431158e307));
    test::<f64>(5, 10000, None);
    test::<f64>(5, -10000, None);
    test::<f64>(u64::MAX, -64, None); // precision too high
    test::<f64>(3, -1075, None); // precision too high
    test::<f64>(1, 1024, None); // precision too high
}

fn from_integer_mantissa_and_exponent_properties_helper<T: PrimitiveFloat>() {
    unsigned_signed_pair_gen_var_1().test_properties(|(mantissa, exponent)| {
        T::from_integer_mantissa_and_exponent(mantissa, exponent);
    });

    unsigned_signed_pair_gen_var_2::<T>().test_properties(|(mantissa, exponent)| {
        let f = T::from_integer_mantissa_and_exponent(mantissa, exponent).unwrap();
        assert!(!f.is_nan());
        assert!(f.is_sign_positive());
        if !f.is_nan() && mantissa.odd() {
            assert_eq!(f.integer_mantissa_and_exponent(), (mantissa, exponent));
        }
    });
}

#[test]
fn from_integer_mantissa_and_exponent_properties() {
    apply_fn_to_primitive_floats!(from_integer_mantissa_and_exponent_properties_helper);
}
