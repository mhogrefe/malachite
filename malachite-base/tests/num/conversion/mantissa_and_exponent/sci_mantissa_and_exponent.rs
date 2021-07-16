use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::mantissa_and_exponent::{
    from_sci_mantissa_and_exponent_with_rounding, sci_mantissa_and_exponent_with_rounding,
};
use malachite_base::num::conversion::traits::SciMantissaAndExponent;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base_test_util::generators::{
    primitive_float_gen_var_12, primitive_float_signed_pair_gen_var_1,
    primitive_float_signed_pair_gen_var_2, primitive_float_unsigned_pair_gen_var_1,
    primitive_float_unsigned_pair_gen_var_2,
    primitive_float_unsigned_rounding_mode_triple_gen_var_1,
    primitive_float_unsigned_rounding_mode_triple_gen_var_2, unsigned_gen_var_1,
    unsigned_rounding_mode_pair_gen_var_1,
};
use std::cmp::Ordering;
use std::panic::catch_unwind;

#[test]
pub fn test_sci_mantissa_and_exponent() {
    fn test_unsigned<T: PrimitiveUnsigned + SciMantissaAndExponent<U, u64>, U: PrimitiveFloat>(
        x: T,
        mantissa: U,
        exponent: u64,
    ) {
        assert_eq!(NiceFloat(x.sci_mantissa()), NiceFloat(mantissa));
        assert_eq!(SciMantissaAndExponent::<U, u64>::sci_exponent(x), exponent);
        let (actual_mantissa, actual_exponent) = x.sci_mantissa_and_exponent();
        assert_eq!(NiceFloat(actual_mantissa), NiceFloat(mantissa));
        assert_eq!(actual_exponent, exponent);
    }
    test_unsigned::<u8, f32>(1, 1.0, 0);
    test_unsigned::<u8, f32>(2, 1.0, 1);
    test_unsigned::<u8, f32>(3, 1.5, 1);
    test_unsigned::<u8, f32>(100, 1.5625, 6);
    test_unsigned::<u32, f32>(65536, 1.0, 16);

    test_unsigned::<u16, f32>(u16::MAX, 1.9999695, 15);
    test_unsigned::<u32, f32>(u32::MAX, 1.0, 32);
    test_unsigned::<u64, f32>(u64::MAX, 1.0, 64);

    test_unsigned::<u16, f64>(u16::MAX, 1.999969482421875, 15);
    test_unsigned::<u32, f64>(u32::MAX, 1.9999999995343387, 31);
    test_unsigned::<u64, f64>(u64::MAX, 1.0, 64);

    fn test_primitive_float<T: PrimitiveFloat>(x: T, mantissa: T, exponent: i64) {
        assert_eq!(NiceFloat(x.sci_mantissa()), NiceFloat(mantissa));
        assert_eq!(x.sci_exponent(), exponent);
        let (actual_mantissa, actual_exponent) = x.sci_mantissa_and_exponent();
        assert_eq!(NiceFloat(actual_mantissa), NiceFloat(mantissa));
        assert_eq!(actual_exponent, exponent);
    }
    test_primitive_float::<f32>(1.0, 1.0, 0);
    test_primitive_float::<f32>(core::f32::consts::PI, core::f32::consts::FRAC_PI_2, 1);
    test_primitive_float::<f32>(0.1, 1.6, -4);
    test_primitive_float::<f32>(10.0, 1.25, 3);
    test_primitive_float::<f32>(f32::MIN_POSITIVE_SUBNORMAL, 1.0, -149);
    test_primitive_float::<f32>(f32::MAX_SUBNORMAL, 1.9999998, -127);
    test_primitive_float::<f32>(f32::MIN_POSITIVE_NORMAL, 1.0, -126);
    test_primitive_float::<f32>(f32::MAX_FINITE, 1.9999999, 127);

    test_primitive_float::<f64>(1.0, 1.0, 0);
    test_primitive_float::<f64>(core::f64::consts::PI, core::f64::consts::FRAC_PI_2, 1);
    test_primitive_float::<f64>(0.1, 1.6, -4);
    test_primitive_float::<f64>(10.0, 1.25, 3);
    test_primitive_float::<f64>(f64::MIN_POSITIVE_SUBNORMAL, 1.0, -1074);
    test_primitive_float::<f64>(f64::MAX_SUBNORMAL, 1.9999999999999996, -1023);
    test_primitive_float::<f64>(f64::MIN_POSITIVE_NORMAL, 1.0, -1022);
    test_primitive_float::<f64>(f64::MAX_FINITE, 1.9999999999999998, 1023);
}

fn sci_mantissa_and_exponent_fail_helper_unsigned<
    T: PrimitiveUnsigned + SciMantissaAndExponent<U, u64>,
    U: PrimitiveFloat,
>() {
    assert_panic!(SciMantissaAndExponent::<U, u64>::sci_mantissa_and_exponent(
        T::ZERO
    ));
}

fn sci_mantissa_and_exponent_fail_helper_primitive_float<T: PrimitiveFloat>() {
    assert_panic!(T::NAN.sci_mantissa_and_exponent());
    assert_panic!(T::POSITIVE_INFINITY.sci_mantissa_and_exponent());
    assert_panic!(T::NEGATIVE_INFINITY.sci_mantissa_and_exponent());
    assert_panic!(T::ZERO.sci_mantissa_and_exponent());
    assert_panic!(T::NEGATIVE_ZERO.sci_mantissa_and_exponent());
}

#[test]
pub fn sci_mantissa_and_exponent_fail() {
    apply_fn_to_unsigneds_and_primitive_floats!(sci_mantissa_and_exponent_fail_helper_unsigned);
    apply_fn_to_primitive_floats!(sci_mantissa_and_exponent_fail_helper_primitive_float);
}

#[test]
pub fn test_sci_mantissa_and_exponent_with_rounding() {
    fn test<T: PrimitiveUnsigned + SciMantissaAndExponent<U, u64>, U: PrimitiveFloat>(
        x: T,
        rm: RoundingMode,
        out: Option<(U, u64)>,
    ) {
        assert_eq!(
            sci_mantissa_and_exponent_with_rounding::<T, U>(x, rm).map(|(m, e)| (NiceFloat(m), e)),
            out.map(|(m, e)| (NiceFloat(m), e))
        );
    }
    test::<u8, f32>(1, RoundingMode::Floor, Some((1.0, 0)));
    test::<u8, f32>(1, RoundingMode::Down, Some((1.0, 0)));
    test::<u8, f32>(1, RoundingMode::Ceiling, Some((1.0, 0)));
    test::<u8, f32>(1, RoundingMode::Up, Some((1.0, 0)));
    test::<u8, f32>(1, RoundingMode::Nearest, Some((1.0, 0)));
    test::<u8, f32>(1, RoundingMode::Exact, Some((1.0, 0)));

    test::<u8, f32>(2, RoundingMode::Floor, Some((1.0, 1)));
    test::<u8, f32>(3, RoundingMode::Floor, Some((1.5, 1)));
    test::<u8, f32>(100, RoundingMode::Floor, Some((1.5625, 6)));
    test::<u32, f32>(65536, RoundingMode::Floor, Some((1.0, 16)));

    test::<u16, f32>(u16::MAX, RoundingMode::Floor, Some((1.9999695, 15)));
    test::<u16, f32>(u16::MAX, RoundingMode::Down, Some((1.9999695, 15)));
    test::<u16, f32>(u16::MAX, RoundingMode::Ceiling, Some((1.9999695, 15)));
    test::<u16, f32>(u16::MAX, RoundingMode::Up, Some((1.9999695, 15)));
    test::<u16, f32>(u16::MAX, RoundingMode::Nearest, Some((1.9999695, 15)));
    test::<u16, f32>(u16::MAX, RoundingMode::Exact, Some((1.9999695, 15)));

    test::<u32, f32>(u32::MAX, RoundingMode::Floor, Some((1.9999999, 31)));
    test::<u32, f32>(u32::MAX, RoundingMode::Down, Some((1.9999999, 31)));
    test::<u32, f32>(u32::MAX, RoundingMode::Ceiling, Some((1.0, 32)));
    test::<u32, f32>(u32::MAX, RoundingMode::Up, Some((1.0, 32)));
    test::<u32, f32>(u32::MAX, RoundingMode::Nearest, Some((1.0, 32)));
    test::<u32, f32>(u32::MAX, RoundingMode::Exact, None);

    test::<u64, f32>(u64::MAX, RoundingMode::Floor, Some((1.9999999, 63)));
    test::<u64, f32>(u64::MAX, RoundingMode::Down, Some((1.9999999, 63)));
    test::<u64, f32>(u64::MAX, RoundingMode::Ceiling, Some((1.0, 64)));
    test::<u64, f32>(u64::MAX, RoundingMode::Up, Some((1.0, 64)));
    test::<u64, f32>(u64::MAX, RoundingMode::Nearest, Some((1.0, 64)));
    test::<u64, f32>(u64::MAX, RoundingMode::Exact, None);

    test::<u16, f64>(u16::MAX, RoundingMode::Floor, Some((1.999969482421875, 15)));
    test::<u16, f64>(u16::MAX, RoundingMode::Down, Some((1.999969482421875, 15)));
    test::<u16, f64>(
        u16::MAX,
        RoundingMode::Ceiling,
        Some((1.999969482421875, 15)),
    );
    test::<u16, f64>(u16::MAX, RoundingMode::Up, Some((1.999969482421875, 15)));
    test::<u16, f64>(
        u16::MAX,
        RoundingMode::Nearest,
        Some((1.999969482421875, 15)),
    );
    test::<u16, f64>(u16::MAX, RoundingMode::Exact, Some((1.999969482421875, 15)));

    test::<u32, f64>(
        u32::MAX,
        RoundingMode::Floor,
        Some((1.9999999995343387, 31)),
    );
    test::<u32, f64>(u32::MAX, RoundingMode::Down, Some((1.9999999995343387, 31)));
    test::<u32, f64>(
        u32::MAX,
        RoundingMode::Ceiling,
        Some((1.9999999995343387, 31)),
    );
    test::<u32, f64>(u32::MAX, RoundingMode::Up, Some((1.9999999995343387, 31)));
    test::<u32, f64>(
        u32::MAX,
        RoundingMode::Nearest,
        Some((1.9999999995343387, 31)),
    );
    test::<u32, f64>(
        u32::MAX,
        RoundingMode::Exact,
        Some((1.9999999995343387, 31)),
    );

    test::<u64, f64>(
        u64::MAX,
        RoundingMode::Floor,
        Some((1.9999999999999998, 63)),
    );
    test::<u64, f64>(u64::MAX, RoundingMode::Down, Some((1.9999999999999998, 63)));
    test::<u64, f64>(u64::MAX, RoundingMode::Ceiling, Some((1.0, 64)));
    test::<u64, f64>(u64::MAX, RoundingMode::Up, Some((1.0, 64)));
    test::<u64, f64>(u64::MAX, RoundingMode::Nearest, Some((1.0, 64)));
    test::<u64, f64>(u64::MAX, RoundingMode::Exact, None);
}

fn sci_mantissa_and_exponent_with_rounding_fail_helper<T: PrimitiveUnsigned, U: PrimitiveFloat>() {
    assert_panic!(sci_mantissa_and_exponent_with_rounding::<T, U>(
        T::ZERO,
        RoundingMode::Floor
    ));
}

#[test]
pub fn sci_mantissa_and_exponent_with_rounding_fail() {
    apply_fn_to_unsigneds_and_primitive_floats!(
        sci_mantissa_and_exponent_with_rounding_fail_helper
    );
}

#[test]
pub fn test_from_sci_mantissa_and_exponent() {
    fn test_unsigned<T: PrimitiveUnsigned + SciMantissaAndExponent<U, u64>, U: PrimitiveFloat>(
        mantissa: U,
        exponent: u64,
        x: Option<T>,
    ) {
        assert_eq!(T::from_sci_mantissa_and_exponent(mantissa, exponent), x);
    }
    test_unsigned::<u8, f32>(1.0, 0, Some(1));
    test_unsigned::<u8, f32>(1.0, 1, Some(2));
    test_unsigned::<u8, f32>(1.5, 0, Some(2));
    test_unsigned::<u8, f32>(1.5, 1, Some(3));
    test_unsigned::<u8, f32>(1.5626, 6, Some(100));
    test_unsigned::<u32, f32>(1.0, 16, Some(65536));
    test_unsigned::<u32, f32>(1.0, 100, None);
    test_unsigned::<u32, f64>(1.0, 32, None);

    fn test_primitive_float<T: PrimitiveFloat>(mantissa: T, exponent: i64, x: Option<T>) {
        assert_eq!(
            T::from_sci_mantissa_and_exponent(mantissa, exponent).map(NiceFloat),
            x.map(NiceFloat)
        );
    }
    test_primitive_float::<f32>(1.0, 0, Some(1.0));
    test_primitive_float::<f32>(std::f32::consts::FRAC_PI_2, 1, Some(core::f32::consts::PI));
    test_primitive_float::<f32>(1.6, -4, Some(0.1));
    test_primitive_float::<f32>(1.25, 3, Some(10.0));
    test_primitive_float::<f32>(1.0, -149, Some(f32::MIN_POSITIVE_SUBNORMAL));
    test_primitive_float::<f32>(1.9999998, -127, Some(f32::MAX_SUBNORMAL));
    test_primitive_float::<f32>(1.0, -126, Some(f32::MIN_POSITIVE_NORMAL));
    test_primitive_float::<f32>(1.9999999, 127, Some(f32::MAX_FINITE));

    test_primitive_float::<f32>(2.0, 1, None);
    test_primitive_float::<f32>(1.1, -2000, None);
    test_primitive_float::<f32>(1.1, 2000, None);
    test_primitive_float::<f32>(1.999, -149, None); // precision too high

    test_primitive_float::<f64>(1.0, 0, Some(1.0));
    test_primitive_float::<f64>(std::f64::consts::FRAC_PI_2, 1, Some(core::f64::consts::PI));
    test_primitive_float::<f64>(1.6, -4, Some(0.1));
    test_primitive_float::<f64>(1.25, 3, Some(10.0));
    test_primitive_float::<f64>(1.0, -1074, Some(f64::MIN_POSITIVE_SUBNORMAL));
    test_primitive_float::<f64>(1.9999999999999996, -1023, Some(f64::MAX_SUBNORMAL));
    test_primitive_float::<f64>(1.0, -1022, Some(f64::MIN_POSITIVE_NORMAL));
    test_primitive_float::<f64>(1.9999999999999998, 1023, Some(f64::MAX_FINITE));

    test_primitive_float::<f64>(2.0, 1, None);
    test_primitive_float::<f64>(1.1, -2000, None);
    test_primitive_float::<f64>(1.1, 2000, None);
    test_primitive_float::<f64>(1.999, -1074, None); // precision too high
}

#[test]
pub fn test_from_sci_mantissa_and_exponent_with_rounding() {
    fn test<T: PrimitiveUnsigned, U: PrimitiveFloat>(
        mantissa: U,
        exponent: u64,
        rm: RoundingMode,
        x: Option<T>,
    ) {
        assert_eq!(
            from_sci_mantissa_and_exponent_with_rounding::<T, U>(mantissa, exponent, rm),
            x
        );
    }
    test::<u8, f32>(1.0, 0, RoundingMode::Floor, Some(1));
    test::<u8, f32>(1.0, 0, RoundingMode::Down, Some(1));
    test::<u8, f32>(1.0, 0, RoundingMode::Ceiling, Some(1));
    test::<u8, f32>(1.0, 0, RoundingMode::Up, Some(1));
    test::<u8, f32>(1.0, 0, RoundingMode::Nearest, Some(1));
    test::<u8, f32>(1.0, 0, RoundingMode::Exact, Some(1));

    test::<u8, f32>(1.25, 0, RoundingMode::Floor, Some(1));
    test::<u8, f32>(1.25, 0, RoundingMode::Down, Some(1));
    test::<u8, f32>(1.25, 0, RoundingMode::Ceiling, Some(2));
    test::<u8, f32>(1.25, 0, RoundingMode::Up, Some(2));
    test::<u8, f32>(1.25, 0, RoundingMode::Nearest, Some(1));
    test::<u8, f32>(1.25, 0, RoundingMode::Exact, None);

    test::<u8, f32>(1.5, 0, RoundingMode::Floor, Some(1));
    test::<u8, f32>(1.5, 0, RoundingMode::Down, Some(1));
    test::<u8, f32>(1.5, 0, RoundingMode::Ceiling, Some(2));
    test::<u8, f32>(1.5, 0, RoundingMode::Up, Some(2));
    test::<u8, f32>(1.5, 0, RoundingMode::Nearest, Some(2));
    test::<u8, f32>(1.5, 0, RoundingMode::Exact, None);

    test::<u8, f32>(1.75, 0, RoundingMode::Floor, Some(1));
    test::<u8, f32>(1.75, 0, RoundingMode::Down, Some(1));
    test::<u8, f32>(1.75, 0, RoundingMode::Ceiling, Some(2));
    test::<u8, f32>(1.75, 0, RoundingMode::Up, Some(2));
    test::<u8, f32>(1.75, 0, RoundingMode::Nearest, Some(2));
    test::<u8, f32>(1.75, 0, RoundingMode::Exact, None);

    test::<u8, f32>(1.5, 1, RoundingMode::Floor, Some(3));
    test::<u8, f32>(1.5, 1, RoundingMode::Down, Some(3));
    test::<u8, f32>(1.5, 1, RoundingMode::Ceiling, Some(3));
    test::<u8, f32>(1.5, 1, RoundingMode::Up, Some(3));
    test::<u8, f32>(1.5, 1, RoundingMode::Nearest, Some(3));
    test::<u8, f32>(1.5, 1, RoundingMode::Exact, Some(3));

    test::<u8, f32>(1.0, 100, RoundingMode::Floor, None);
    test::<u8, f32>(1.0, 100, RoundingMode::Down, None);
    test::<u8, f32>(1.0, 100, RoundingMode::Ceiling, None);
    test::<u8, f32>(1.0, 100, RoundingMode::Up, None);
    test::<u8, f32>(1.0, 100, RoundingMode::Nearest, None);
    test::<u8, f32>(1.0, 100, RoundingMode::Exact, None);
}

fn from_sci_mantissa_and_exponent_with_rounding_fail_helper<
    T: PrimitiveUnsigned,
    U: PrimitiveFloat,
>() {
    assert_panic!(from_sci_mantissa_and_exponent_with_rounding::<T, U>(
        U::ZERO,
        0,
        RoundingMode::Floor
    ));
}

#[test]
pub fn from_sci_mantissa_and_exponent_with_rounding_fail() {
    apply_fn_to_unsigneds_and_primitive_floats!(
        from_sci_mantissa_and_exponent_with_rounding_fail_helper
    );
}

fn sci_mantissa_and_exponent_properties_helper_unsigned<
    T: PrimitiveUnsigned + SciMantissaAndExponent<U, u64>,
    U: PrimitiveFloat,
>() {
    unsigned_gen_var_1::<T>().test_properties(|x| {
        let (mantissa, exponent) = SciMantissaAndExponent::<U, u64>::sci_mantissa_and_exponent(x);
        assert_eq!(NiceFloat(x.sci_mantissa()), NiceFloat(mantissa));
        assert_eq!(SciMantissaAndExponent::<U, u64>::sci_exponent(x), exponent);
        assert_eq!(
            sci_mantissa_and_exponent_with_rounding(x, RoundingMode::Nearest)
                .map(|(m, e)| (NiceFloat(m), e)),
            Some((NiceFloat(mantissa), exponent))
        );

        assert!(exponent <= T::WIDTH);
        assert!(mantissa >= U::ONE);
        assert!(mantissa < U::TWO);
    });
}

fn sci_mantissa_and_exponent_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_gen_var_12::<T>().test_properties(|x| {
        let (mantissa, exponent) = x.sci_mantissa_and_exponent();
        assert_eq!(NiceFloat(x.sci_mantissa()), NiceFloat(mantissa));
        assert_eq!(x.sci_exponent(), exponent);
        assert_eq!(
            NiceFloat(T::from_sci_mantissa_and_exponent(mantissa, exponent).unwrap()),
            NiceFloat(x.abs())
        );

        assert!(exponent >= T::MIN_EXPONENT);
        assert!(exponent <= T::MAX_EXPONENT);
        assert!(mantissa >= T::ONE);
        assert!(mantissa < T::TWO);

        let precision = x.precision();
        assert_eq!(mantissa.precision(), precision);
        assert!(precision <= T::max_precision_for_sci_exponent(exponent));
    });
}

#[test]
fn sci_mantissa_and_exponent_properties() {
    apply_fn_to_unsigneds_and_primitive_floats!(
        sci_mantissa_and_exponent_properties_helper_unsigned
    );
    apply_fn_to_primitive_floats!(sci_mantissa_and_exponent_properties_helper_primitive_float);
}

fn sci_mantissa_and_exponent_properties_with_rounding_helper<
    T: PrimitiveUnsigned,
    U: PrimitiveFloat,
>() {
    unsigned_rounding_mode_pair_gen_var_1::<T>().test_properties(|(x, rm)| {
        if let Some((mantissa, exponent)) = sci_mantissa_and_exponent_with_rounding::<T, U>(x, rm) {
            assert!(mantissa >= U::ONE);
            assert!(mantissa < U::TWO);
            if rm == RoundingMode::Exact {
                assert_eq!(
                    from_sci_mantissa_and_exponent_with_rounding(mantissa, exponent, rm),
                    Some(x)
                );
            }
        }
    });

    unsigned_gen_var_1::<T>().test_properties(|n| {
        let (floor_mantissa, floor_exponent) =
            sci_mantissa_and_exponent_with_rounding::<T, U>(n, RoundingMode::Floor).unwrap();
        assert_eq!(
            sci_mantissa_and_exponent_with_rounding::<T, U>(n, RoundingMode::Down).unwrap(),
            (floor_mantissa, floor_exponent)
        );
        let (ceiling_mantissa, ceiling_exponent) =
            sci_mantissa_and_exponent_with_rounding::<T, U>(n, RoundingMode::Ceiling).unwrap();
        assert_eq!(
            sci_mantissa_and_exponent_with_rounding::<T, U>(n, RoundingMode::Up).unwrap(),
            (ceiling_mantissa, ceiling_exponent)
        );
        let (nearest_mantissa, nearest_exponent) =
            sci_mantissa_and_exponent_with_rounding::<T, U>(n, RoundingMode::Nearest).unwrap();
        if let Some((mantissa, exponent)) =
            sci_mantissa_and_exponent_with_rounding::<T, U>(n, RoundingMode::Exact)
        {
            assert_eq!(floor_mantissa, mantissa);
            assert_eq!(ceiling_mantissa, mantissa);
            assert_eq!(nearest_mantissa, mantissa);
            assert_eq!(floor_exponent, exponent);
            assert_eq!(ceiling_exponent, exponent);
            assert_eq!(nearest_exponent, exponent);
        } else {
            assert_ne!(
                (floor_mantissa, floor_exponent),
                (ceiling_mantissa, ceiling_exponent)
            );
            assert!(
                (nearest_mantissa, nearest_exponent) == (floor_mantissa, floor_exponent)
                    || (nearest_mantissa, nearest_exponent) == (ceiling_mantissa, ceiling_exponent)
            );
            if ceiling_mantissa == U::ONE {
                assert_eq!(floor_mantissa, U::TWO.next_lower());
                assert_eq!(floor_exponent, ceiling_exponent - 1);
            } else {
                assert_eq!(floor_mantissa, ceiling_mantissa.next_lower());
                assert_eq!(floor_exponent, ceiling_exponent);
            }
        }
    });
}

#[test]
fn sci_mantissa_and_exponent_with_rounding_properties() {
    apply_fn_to_unsigneds_and_primitive_floats!(
        sci_mantissa_and_exponent_properties_with_rounding_helper
    );
}

fn from_sci_mantissa_and_exponent_properties_helper_unsigned<
    T: PrimitiveUnsigned + SciMantissaAndExponent<U, u64>,
    U: PrimitiveFloat,
>() {
    primitive_float_unsigned_pair_gen_var_1::<U, u64>().test_properties(|(m, e)| {
        T::from_sci_mantissa_and_exponent(m, e);
    });

    primitive_float_unsigned_pair_gen_var_2::<U>().test_properties(|(m, e)| {
        assert!(m >= U::ONE && m < U::TWO);
        let on = T::from_sci_mantissa_and_exponent(m, e);
        assert_eq!(
            from_sci_mantissa_and_exponent_with_rounding(m, e, RoundingMode::Nearest),
            on
        );
    });
}

fn from_sci_mantissa_and_exponent_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_signed_pair_gen_var_1().test_properties(|(mantissa, exponent)| {
        T::from_sci_mantissa_and_exponent(mantissa, exponent);
    });

    primitive_float_signed_pair_gen_var_2::<T>().test_properties(|(mantissa, exponent)| {
        let f = T::from_sci_mantissa_and_exponent(mantissa, exponent).unwrap();
        assert!(!f.is_nan());
        assert_eq!(f.sign(), Ordering::Greater);
        assert_eq!(f.sci_mantissa_and_exponent(), (mantissa, exponent));
    });
}

#[test]
fn from_sci_mantissa_and_exponent_properties() {
    apply_fn_to_unsigneds_and_primitive_floats!(
        from_sci_mantissa_and_exponent_properties_helper_unsigned
    );
    apply_fn_to_primitive_floats!(from_sci_mantissa_and_exponent_properties_helper_primitive_float);
}

fn from_sci_mantissa_and_exponent_properties_with_rounding_helper<
    T: PrimitiveUnsigned,
    U: PrimitiveFloat,
>() {
    primitive_float_unsigned_rounding_mode_triple_gen_var_1::<U, u64>().test_properties(
        |(m, e, rm)| {
            let on = from_sci_mantissa_and_exponent_with_rounding::<T, U>(m, e, rm);
            if on.is_some() {
                assert!(m >= U::ONE && m < U::TWO);
            }
        },
    );

    primitive_float_unsigned_rounding_mode_triple_gen_var_2::<U>().test_properties(|(m, e, rm)| {
        assert!(m >= U::ONE && m < U::TWO);
        from_sci_mantissa_and_exponent_with_rounding::<T, U>(m, e, rm);
    });

    primitive_float_unsigned_pair_gen_var_2::<U>().test_properties(|(m, e)| {
        if let Some(ceiling_n) =
            from_sci_mantissa_and_exponent_with_rounding::<T, U>(m, e, RoundingMode::Ceiling)
        {
            assert_eq!(
                from_sci_mantissa_and_exponent_with_rounding::<T, U>(m, e, RoundingMode::Up)
                    .unwrap(),
                ceiling_n
            );
            let floor_n =
                from_sci_mantissa_and_exponent_with_rounding::<T, U>(m, e, RoundingMode::Floor)
                    .unwrap();
            assert_eq!(
                from_sci_mantissa_and_exponent_with_rounding::<T, U>(m, e, RoundingMode::Down)
                    .unwrap(),
                floor_n
            );
            let nearest_n =
                from_sci_mantissa_and_exponent_with_rounding::<T, U>(m, e, RoundingMode::Nearest)
                    .unwrap();
            if let Some(n) =
                from_sci_mantissa_and_exponent_with_rounding::<T, U>(m, e, RoundingMode::Exact)
            {
                assert_eq!(floor_n, n);
                assert_eq!(ceiling_n, n);
                assert_eq!(nearest_n, n);
            } else {
                assert!(nearest_n == floor_n || nearest_n == ceiling_n);
                if floor_n != T::MAX {
                    assert_eq!(ceiling_n, floor_n + T::ONE);
                }
            }
        }
    });
}

#[test]
fn from_sci_mantissa_and_exponent_with_rounding_properties() {
    apply_fn_to_unsigneds_and_primitive_floats!(
        from_sci_mantissa_and_exponent_properties_with_rounding_helper
    );
}
