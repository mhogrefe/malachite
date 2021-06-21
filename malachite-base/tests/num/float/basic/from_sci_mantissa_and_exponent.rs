use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base_test_util::generators::{
    primitive_float_signed_pair_gen_var_1, primitive_float_signed_pair_gen_var_2,
};

#[test]
pub fn test_from_sci_mantissa_and_exponent() {
    fn test<T: PrimitiveFloat>(mantissa: T, exponent: i64, x: Option<T>) {
        assert_eq!(
            T::from_sci_mantissa_and_exponent(mantissa, exponent).map(NiceFloat),
            x.map(NiceFloat)
        );
    }
    test::<f32>(1.0, 0, Some(1.0));
    test::<f32>(std::f32::consts::FRAC_PI_2, 1, Some(core::f32::consts::PI));
    test::<f32>(1.6, -4, Some(0.1));
    test::<f32>(1.25, 3, Some(10.0));
    test::<f32>(1.0, -149, Some(f32::MIN_POSITIVE_SUBNORMAL));
    test::<f32>(1.9999998, -127, Some(f32::MAX_SUBNORMAL));
    test::<f32>(1.0, -126, Some(f32::MIN_POSITIVE_NORMAL));
    test::<f32>(1.9999999, 127, Some(f32::MAX_FINITE));

    test::<f32>(2.0, 1, None);
    test::<f32>(1.1, -2000, None);
    test::<f32>(1.1, 2000, None);
    test::<f32>(1.999, -149, None); // precision too high

    test::<f64>(1.0, 0, Some(1.0));
    test::<f64>(std::f64::consts::FRAC_PI_2, 1, Some(core::f64::consts::PI));
    test::<f64>(1.6, -4, Some(0.1));
    test::<f64>(1.25, 3, Some(10.0));
    test::<f64>(1.0, -1074, Some(f64::MIN_POSITIVE_SUBNORMAL));
    test::<f64>(1.9999999999999996, -1023, Some(f64::MAX_SUBNORMAL));
    test::<f64>(1.0, -1022, Some(f64::MIN_POSITIVE_NORMAL));
    test::<f64>(1.9999999999999998, 1023, Some(f64::MAX_FINITE));

    test::<f64>(2.0, 1, None);
    test::<f64>(1.1, -2000, None);
    test::<f64>(1.1, 2000, None);
    test::<f64>(1.999, -1074, None); // precision too high
}

fn from_sci_mantissa_and_exponent_properties_helper<T: PrimitiveFloat>() {
    primitive_float_signed_pair_gen_var_1().test_properties(|(mantissa, exponent)| {
        T::from_sci_mantissa_and_exponent(mantissa, exponent);
    });

    primitive_float_signed_pair_gen_var_2::<T>().test_properties(|(mantissa, exponent)| {
        let f = T::from_sci_mantissa_and_exponent(mantissa, exponent).unwrap();
        assert!(!f.is_nan());
        assert!(f.is_sign_positive());
        assert_eq!(f.sci_mantissa_and_exponent(), (mantissa, exponent));
    });
}

#[test]
fn from_sci_mantissa_and_exponent_properties() {
    apply_fn_to_primitive_floats!(from_sci_mantissa_and_exponent_properties_helper);
}
