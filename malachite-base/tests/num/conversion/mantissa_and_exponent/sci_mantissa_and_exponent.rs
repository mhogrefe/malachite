// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::mantissa_and_exponent::{
    from_sci_mantissa_and_exponent_round, sci_mantissa_and_exponent_round,
};
use malachite_base::num::conversion::traits::SciMantissaAndExponent;
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    primitive_float_gen_var_12, primitive_float_signed_pair_gen_var_1,
    primitive_float_signed_pair_gen_var_2, primitive_float_unsigned_pair_gen_var_1,
    primitive_float_unsigned_pair_gen_var_2,
    primitive_float_unsigned_rounding_mode_triple_gen_var_1,
    primitive_float_unsigned_rounding_mode_triple_gen_var_2, unsigned_gen_var_1,
    unsigned_rounding_mode_pair_gen_var_1,
};
use std::cmp::Ordering::{self, *};
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
    assert_panic!(T::INFINITY.sci_mantissa_and_exponent());
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
pub fn test_sci_mantissa_and_exponent_round() {
    fn test<T: PrimitiveUnsigned + SciMantissaAndExponent<U, u64>, U: PrimitiveFloat>(
        x: T,
        rm: RoundingMode,
        out: Option<(U, u64, Ordering)>,
    ) {
        assert_eq!(
            sci_mantissa_and_exponent_round::<T, U>(x, rm).map(|(m, e, o)| (NiceFloat(m), e, o)),
            out.map(|(m, e, o)| (NiceFloat(m), e, o))
        );
    }
    test::<u8, f32>(1, Floor, Some((1.0, 0, Equal)));
    test::<u8, f32>(1, Down, Some((1.0, 0, Equal)));
    test::<u8, f32>(1, Ceiling, Some((1.0, 0, Equal)));
    test::<u8, f32>(1, Up, Some((1.0, 0, Equal)));
    test::<u8, f32>(1, Nearest, Some((1.0, 0, Equal)));
    test::<u8, f32>(1, Exact, Some((1.0, 0, Equal)));

    test::<u8, f32>(2, Floor, Some((1.0, 1, Equal)));
    test::<u8, f32>(3, Floor, Some((1.5, 1, Equal)));
    test::<u8, f32>(100, Floor, Some((1.5625, 6, Equal)));
    test::<u32, f32>(65536, Floor, Some((1.0, 16, Equal)));

    test::<u16, f32>(u16::MAX, Floor, Some((1.9999695, 15, Equal)));
    test::<u16, f32>(u16::MAX, Down, Some((1.9999695, 15, Equal)));
    test::<u16, f32>(u16::MAX, Ceiling, Some((1.9999695, 15, Equal)));
    test::<u16, f32>(u16::MAX, Up, Some((1.9999695, 15, Equal)));
    test::<u16, f32>(u16::MAX, Nearest, Some((1.9999695, 15, Equal)));
    test::<u16, f32>(u16::MAX, Exact, Some((1.9999695, 15, Equal)));

    test::<u32, f32>(u32::MAX, Floor, Some((1.9999999, 31, Less)));
    test::<u32, f32>(u32::MAX, Down, Some((1.9999999, 31, Less)));
    test::<u32, f32>(u32::MAX, Ceiling, Some((1.0, 32, Greater)));
    test::<u32, f32>(u32::MAX, Up, Some((1.0, 32, Greater)));
    test::<u32, f32>(u32::MAX, Nearest, Some((1.0, 32, Greater)));
    test::<u32, f32>(u32::MAX, Exact, None);

    test::<u64, f32>(u64::MAX, Floor, Some((1.9999999, 63, Less)));
    test::<u64, f32>(u64::MAX, Down, Some((1.9999999, 63, Less)));
    test::<u64, f32>(u64::MAX, Ceiling, Some((1.0, 64, Greater)));
    test::<u64, f32>(u64::MAX, Up, Some((1.0, 64, Greater)));
    test::<u64, f32>(u64::MAX, Nearest, Some((1.0, 64, Greater)));
    test::<u64, f32>(u64::MAX, Exact, None);

    test::<u16, f64>(u16::MAX, Floor, Some((1.999969482421875, 15, Equal)));
    test::<u16, f64>(u16::MAX, Down, Some((1.999969482421875, 15, Equal)));
    test::<u16, f64>(u16::MAX, Ceiling, Some((1.999969482421875, 15, Equal)));
    test::<u16, f64>(u16::MAX, Up, Some((1.999969482421875, 15, Equal)));
    test::<u16, f64>(u16::MAX, Nearest, Some((1.999969482421875, 15, Equal)));
    test::<u16, f64>(u16::MAX, Exact, Some((1.999969482421875, 15, Equal)));

    test::<u32, f64>(u32::MAX, Floor, Some((1.9999999995343387, 31, Equal)));
    test::<u32, f64>(u32::MAX, Down, Some((1.9999999995343387, 31, Equal)));
    test::<u32, f64>(u32::MAX, Ceiling, Some((1.9999999995343387, 31, Equal)));
    test::<u32, f64>(u32::MAX, Up, Some((1.9999999995343387, 31, Equal)));
    test::<u32, f64>(u32::MAX, Nearest, Some((1.9999999995343387, 31, Equal)));
    test::<u32, f64>(u32::MAX, Exact, Some((1.9999999995343387, 31, Equal)));

    test::<u64, f64>(u64::MAX, Floor, Some((1.9999999999999998, 63, Less)));
    test::<u64, f64>(u64::MAX, Down, Some((1.9999999999999998, 63, Less)));
    test::<u64, f64>(u64::MAX, Ceiling, Some((1.0, 64, Greater)));
    test::<u64, f64>(u64::MAX, Up, Some((1.0, 64, Greater)));
    test::<u64, f64>(u64::MAX, Nearest, Some((1.0, 64, Greater)));
    test::<u64, f64>(u64::MAX, Exact, None);
}

fn sci_mantissa_and_exponent_round_fail_helper<T: PrimitiveUnsigned, U: PrimitiveFloat>() {
    assert_panic!(sci_mantissa_and_exponent_round::<T, U>(T::ZERO, Floor));
}

#[test]
pub fn sci_mantissa_and_exponent_round_fail() {
    apply_fn_to_unsigneds_and_primitive_floats!(sci_mantissa_and_exponent_round_fail_helper);
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
pub fn test_from_sci_mantissa_and_exponent_round() {
    fn test<T: PrimitiveUnsigned, U: PrimitiveFloat>(
        mantissa: U,
        exponent: u64,
        rm: RoundingMode,
        xo: Option<(T, Ordering)>,
    ) {
        assert_eq!(
            from_sci_mantissa_and_exponent_round::<T, U>(mantissa, exponent, rm),
            xo
        );
    }
    test::<u8, f32>(1.0, 0, Floor, Some((1, Equal)));
    test::<u8, f32>(1.0, 0, Down, Some((1, Equal)));
    test::<u8, f32>(1.0, 0, Ceiling, Some((1, Equal)));
    test::<u8, f32>(1.0, 0, Up, Some((1, Equal)));
    test::<u8, f32>(1.0, 0, Nearest, Some((1, Equal)));
    test::<u8, f32>(1.0, 0, Exact, Some((1, Equal)));

    test::<u8, f32>(1.25, 0, Floor, Some((1, Less)));
    test::<u8, f32>(1.25, 0, Down, Some((1, Less)));
    test::<u8, f32>(1.25, 0, Ceiling, Some((2, Greater)));
    test::<u8, f32>(1.25, 0, Up, Some((2, Greater)));
    test::<u8, f32>(1.25, 0, Nearest, Some((1, Less)));
    test::<u8, f32>(1.25, 0, Exact, None);

    test::<u8, f32>(1.5, 0, Floor, Some((1, Less)));
    test::<u8, f32>(1.5, 0, Down, Some((1, Less)));
    test::<u8, f32>(1.5, 0, Ceiling, Some((2, Greater)));
    test::<u8, f32>(1.5, 0, Up, Some((2, Greater)));
    test::<u8, f32>(1.5, 0, Nearest, Some((2, Greater)));
    test::<u8, f32>(1.5, 0, Exact, None);

    test::<u8, f32>(1.75, 0, Floor, Some((1, Less)));
    test::<u8, f32>(1.75, 0, Down, Some((1, Less)));
    test::<u8, f32>(1.75, 0, Ceiling, Some((2, Greater)));
    test::<u8, f32>(1.75, 0, Up, Some((2, Greater)));
    test::<u8, f32>(1.75, 0, Nearest, Some((2, Greater)));
    test::<u8, f32>(1.75, 0, Exact, None);

    test::<u8, f32>(1.5, 1, Floor, Some((3, Equal)));
    test::<u8, f32>(1.5, 1, Down, Some((3, Equal)));
    test::<u8, f32>(1.5, 1, Ceiling, Some((3, Equal)));
    test::<u8, f32>(1.5, 1, Up, Some((3, Equal)));
    test::<u8, f32>(1.5, 1, Nearest, Some((3, Equal)));
    test::<u8, f32>(1.5, 1, Exact, Some((3, Equal)));

    test::<u8, f32>(1.0, 100, Floor, None);
    test::<u8, f32>(1.0, 100, Down, None);
    test::<u8, f32>(1.0, 100, Ceiling, None);
    test::<u8, f32>(1.0, 100, Up, None);
    test::<u8, f32>(1.0, 100, Nearest, None);
    test::<u8, f32>(1.0, 100, Exact, None);
}

fn from_sci_mantissa_and_exponent_round_fail_helper<T: PrimitiveUnsigned, U: PrimitiveFloat>() {
    assert_panic!(from_sci_mantissa_and_exponent_round::<T, U>(
        U::ZERO,
        0,
        Floor
    ));
}

#[test]
pub fn from_sci_mantissa_and_exponent_round_fail() {
    apply_fn_to_unsigneds_and_primitive_floats!(from_sci_mantissa_and_exponent_round_fail_helper);
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
            sci_mantissa_and_exponent_round(x, Nearest).map(|(m, e, _)| (NiceFloat(m), e)),
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

fn sci_mantissa_and_exponent_properties_round_helper<T: PrimitiveUnsigned, U: PrimitiveFloat>() {
    unsigned_rounding_mode_pair_gen_var_1::<T>().test_properties(|(x, rm)| {
        if let Some((mantissa, exponent, o)) = sci_mantissa_and_exponent_round::<T, U>(x, rm) {
            assert!(mantissa >= U::ONE);
            assert!(mantissa < U::TWO);
            match rm {
                Floor | Down => assert_ne!(o, Greater),
                Ceiling | Up => assert_ne!(o, Less),
                Exact => assert_eq!(o, Equal),
                _ => {}
            }
            if o == Equal {
                for rm in exhaustive_rounding_modes() {
                    assert_eq!(
                        sci_mantissa_and_exponent_round(x, rm),
                        Some((mantissa, exponent, Equal))
                    );
                }
            } else {
                assert!(sci_mantissa_and_exponent_round::<T, U>(x, Exact).is_none(),);
            }
        }
    });

    unsigned_gen_var_1::<T>().test_properties(|n| {
        let (floor_mantissa, floor_exponent, o_floor) =
            sci_mantissa_and_exponent_round::<T, U>(n, Floor).unwrap();
        assert_ne!(o_floor, Greater);
        assert_eq!(
            sci_mantissa_and_exponent_round::<T, U>(n, Down).unwrap(),
            (floor_mantissa, floor_exponent, o_floor)
        );
        let (ceiling_mantissa, ceiling_exponent, o_ceiling) =
            sci_mantissa_and_exponent_round::<T, U>(n, Ceiling).unwrap();
        assert_ne!(o_ceiling, Less);
        assert_eq!(
            sci_mantissa_and_exponent_round::<T, U>(n, Up).unwrap(),
            (ceiling_mantissa, ceiling_exponent, o_ceiling)
        );
        let (nearest_mantissa, nearest_exponent, o_nearest) =
            sci_mantissa_and_exponent_round::<T, U>(n, Nearest).unwrap();
        if let Some((mantissa, exponent, o)) = sci_mantissa_and_exponent_round::<T, U>(n, Exact) {
            assert_eq!(o, Equal);
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
                (nearest_mantissa, nearest_exponent, o_nearest)
                    == (floor_mantissa, floor_exponent, Less)
                    || (nearest_mantissa, nearest_exponent, o_nearest)
                        == (ceiling_mantissa, ceiling_exponent, Greater)
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
fn sci_mantissa_and_exponent_round_properties() {
    apply_fn_to_unsigneds_and_primitive_floats!(sci_mantissa_and_exponent_properties_round_helper);
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
            from_sci_mantissa_and_exponent_round(m, e, Nearest).map(|p| p.0),
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
        assert_eq!(f.sign(), Greater);
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

fn from_sci_mantissa_and_exponent_properties_round_helper<
    T: PrimitiveUnsigned,
    U: PrimitiveFloat,
>() {
    primitive_float_unsigned_rounding_mode_triple_gen_var_1::<U, u64>().test_properties(
        |(m, e, rm)| {
            let on = from_sci_mantissa_and_exponent_round::<T, U>(m, e, rm);
            if let Some((x, o)) = on {
                assert!(m >= U::ONE && m < U::TWO);
                if o == Equal {
                    for rm in exhaustive_rounding_modes() {
                        assert_eq!(
                            from_sci_mantissa_and_exponent_round::<T, U>(m, e, rm),
                            Some((x, Equal))
                        );
                    }
                } else {
                    assert!(from_sci_mantissa_and_exponent_round::<T, U>(m, e, Exact).is_none());
                }
            }
        },
    );

    primitive_float_unsigned_rounding_mode_triple_gen_var_2::<U>().test_properties(|(m, e, rm)| {
        assert!(m >= U::ONE && m < U::TWO);
        from_sci_mantissa_and_exponent_round::<T, U>(m, e, rm);
    });

    primitive_float_unsigned_pair_gen_var_2::<U>().test_properties(|(m, e)| {
        if let Some(ceiling_n) = from_sci_mantissa_and_exponent_round::<T, U>(m, e, Ceiling) {
            assert_eq!(
                from_sci_mantissa_and_exponent_round::<T, U>(m, e, Up).unwrap(),
                ceiling_n
            );
            let floor_n = from_sci_mantissa_and_exponent_round::<T, U>(m, e, Floor).unwrap();
            assert_eq!(
                from_sci_mantissa_and_exponent_round::<T, U>(m, e, Down).unwrap(),
                floor_n
            );
            let nearest_n = from_sci_mantissa_and_exponent_round::<T, U>(m, e, Nearest).unwrap();
            if let Some(n) = from_sci_mantissa_and_exponent_round::<T, U>(m, e, Exact) {
                assert_eq!(floor_n, n);
                assert_eq!(ceiling_n, n);
                assert_eq!(nearest_n, n);
            } else {
                assert!(nearest_n == floor_n || nearest_n == ceiling_n);
                if floor_n.0 != T::MAX {
                    assert_eq!(ceiling_n.0, floor_n.0 + T::ONE);
                }
            }
        }
    });
}

#[test]
fn from_sci_mantissa_and_exponent_round_properties() {
    apply_fn_to_unsigneds_and_primitive_floats!(
        from_sci_mantissa_and_exponent_properties_round_helper
    );
}
