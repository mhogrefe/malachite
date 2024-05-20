// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::{
    primitive_float_gen_var_12, unsigned_gen_var_1, unsigned_pair_gen_var_2,
    unsigned_pair_gen_var_30, unsigned_signed_pair_gen_var_1, unsigned_signed_pair_gen_var_2,
};
use std::cmp::Ordering::*;
use std::panic::catch_unwind;

#[test]
pub fn test_integer_mantissa_and_exponent() {
    fn test_unsigned<T: PrimitiveUnsigned>(x: T, mantissa: T, exponent: u64) {
        assert_eq!(x.integer_mantissa(), mantissa);
        assert_eq!(x.integer_exponent(), exponent);
        assert_eq!(x.integer_mantissa_and_exponent(), (mantissa, exponent));
    }
    test_unsigned::<u8>(1, 1, 0);
    test_unsigned::<u8>(2, 1, 1);
    test_unsigned::<u8>(3, 3, 0);
    test_unsigned::<u8>(100, 25, 2);
    test_unsigned::<u32>(65536, 1, 16);

    fn test_primitive_float<T: PrimitiveFloat>(x: T, mantissa: u64, exponent: i64) {
        assert_eq!(x.integer_mantissa(), mantissa);
        assert_eq!(x.integer_exponent(), exponent);
        assert_eq!(x.integer_mantissa_and_exponent(), (mantissa, exponent));
    }
    test_primitive_float::<f32>(1.0, 1, 0);
    test_primitive_float::<f32>(core::f32::consts::PI, 13176795, -22);
    test_primitive_float::<f32>(0.1, 13421773, -27);
    test_primitive_float::<f32>(10.0, 5, 1);
    test_primitive_float::<f32>(f32::MIN_POSITIVE_SUBNORMAL, 1, -149);
    test_primitive_float::<f32>(f32::MAX_SUBNORMAL, 0x7fffff, -149);
    test_primitive_float::<f32>(f32::MIN_POSITIVE_NORMAL, 1, -126);
    test_primitive_float::<f32>(f32::MAX_FINITE, 0xffffff, 104);

    test_primitive_float::<f64>(1.0, 1, 0);
    test_primitive_float::<f64>(core::f64::consts::PI, 884279719003555, -48);
    test_primitive_float::<f64>(0.1, 3602879701896397, -55);
    test_primitive_float::<f64>(10.0, 5, 1);
    test_primitive_float::<f64>(f64::MIN_POSITIVE_SUBNORMAL, 1, -1074);
    test_primitive_float::<f64>(f64::MAX_SUBNORMAL, 0xfffffffffffff, -1074);
    test_primitive_float::<f64>(f64::MIN_POSITIVE_NORMAL, 1, -1022);
    test_primitive_float::<f64>(f64::MAX_FINITE, 0x1fffffffffffff, 971);
}

fn integer_mantissa_and_exponent_unsigned_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.integer_mantissa_and_exponent());
}

fn integer_mantissa_and_exponent_primitive_float_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(T::NAN.integer_mantissa_and_exponent());
    assert_panic!(T::INFINITY.integer_mantissa_and_exponent());
    assert_panic!(T::NEGATIVE_INFINITY.integer_mantissa_and_exponent());
    assert_panic!(T::ZERO.integer_mantissa_and_exponent());
    assert_panic!(T::NEGATIVE_ZERO.integer_mantissa_and_exponent());
}

#[test]
pub fn integer_mantissa_and_exponent_fail() {
    apply_fn_to_unsigneds!(integer_mantissa_and_exponent_unsigned_fail_helper);
    apply_fn_to_primitive_floats!(integer_mantissa_and_exponent_primitive_float_fail_helper);
}

#[test]
pub fn test_from_integer_mantissa_and_exponent() {
    fn test_unsigned<T: PrimitiveUnsigned>(mantissa: T, exponent: u64, x: Option<T>) {
        assert_eq!(T::from_integer_mantissa_and_exponent(mantissa, exponent), x);
    }
    test_unsigned::<u8>(0, 0, Some(0));
    test_unsigned::<u8>(1, 0, Some(1));
    test_unsigned::<u8>(1, 1, Some(2));
    test_unsigned::<u8>(3, 0, Some(3));
    test_unsigned::<u8>(25, 2, Some(100));
    test_unsigned::<u32>(1, 16, Some(65536));
    test_unsigned::<u32>(1, 100, None);

    fn test_primitive_float<T: PrimitiveFloat>(mantissa: u64, exponent: i64, x: Option<T>) {
        assert_eq!(
            T::from_integer_mantissa_and_exponent(mantissa, exponent).map(NiceFloat),
            x.map(NiceFloat)
        );
    }
    test_primitive_float::<f32>(0, 5, Some(0.0));
    test_primitive_float::<f32>(1, 0, Some(1.0));
    test_primitive_float::<f32>(4, -2, Some(1.0));
    test_primitive_float::<f32>(13176795, -22, Some(core::f32::consts::PI));
    test_primitive_float::<f32>(13421773, -27, Some(0.1));
    test_primitive_float::<f32>(5, 1, Some(10.0));
    test_primitive_float::<f32>(1, -149, Some(f32::MIN_POSITIVE_SUBNORMAL));
    test_primitive_float::<f32>(4, -151, Some(f32::MIN_POSITIVE_SUBNORMAL));
    test_primitive_float::<f32>(0x7fffff, -149, Some(f32::MAX_SUBNORMAL));
    test_primitive_float::<f32>(1, -126, Some(f32::MIN_POSITIVE_NORMAL));
    test_primitive_float::<f32>(0xffffff, 104, Some(f32::MAX_FINITE));
    test_primitive_float::<f32>(1, 127, Some(1.7014118e38));
    test_primitive_float::<f32>(5, 10000, None);
    test_primitive_float::<f32>(5, -10000, None);
    test_primitive_float::<f32>(u64::MAX, -32, None); // precision too high
    test_primitive_float::<f32>(3, -150, None); // precision too high
    test_primitive_float::<f32>(1, 128, None); // precision too high

    test_primitive_float::<f64>(0, 5, Some(0.0));
    test_primitive_float::<f64>(1, 0, Some(1.0));
    test_primitive_float::<f64>(4, -2, Some(1.0));
    test_primitive_float::<f64>(884279719003555, -48, Some(core::f64::consts::PI));
    test_primitive_float::<f64>(3602879701896397, -55, Some(0.1));
    test_primitive_float::<f64>(5, 1, Some(10.0));
    test_primitive_float::<f64>(1, -1074, Some(f64::MIN_POSITIVE_SUBNORMAL));
    test_primitive_float::<f64>(4, -1076, Some(f64::MIN_POSITIVE_SUBNORMAL));
    test_primitive_float::<f64>(0xfffffffffffff, -1074, Some(f64::MAX_SUBNORMAL));
    test_primitive_float::<f64>(1, -1022, Some(f64::MIN_POSITIVE_NORMAL));
    test_primitive_float::<f64>(0x1fffffffffffff, 971, Some(f64::MAX_FINITE));
    test_primitive_float::<f64>(1, 1023, Some(8.98846567431158e307));
    test_primitive_float::<f64>(5, 10000, None);
    test_primitive_float::<f64>(5, -10000, None);
    test_primitive_float::<f64>(u64::MAX, -64, None); // precision too high
    test_primitive_float::<f64>(3, -1075, None); // precision too high
    test_primitive_float::<f64>(1, 1024, None); // precision too high
}

fn integer_mantissa_and_exponent_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen_var_1::<T>().test_properties(|x| {
        let (mantissa, exponent) = x.integer_mantissa_and_exponent();
        assert_eq!(x.integer_mantissa(), mantissa);
        assert_eq!(x.integer_exponent(), exponent);
        assert_eq!(
            T::from_integer_mantissa_and_exponent(mantissa, exponent).unwrap(),
            x
        );

        assert!(exponent < T::WIDTH);
        assert!(mantissa.odd());
    });
}

fn integer_mantissa_and_exponent_properties_helper_primitive_float<T: PrimitiveFloat>() {
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
    apply_fn_to_unsigneds!(integer_mantissa_and_exponent_properties_helper_unsigned);
    apply_fn_to_primitive_floats!(integer_mantissa_and_exponent_properties_helper_primitive_float);
}

fn from_integer_mantissa_and_exponent_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_2::<T, u64>().test_properties(|(mantissa, exponent)| {
        T::from_integer_mantissa_and_exponent(mantissa, exponent);
    });

    unsigned_pair_gen_var_30::<T>().test_properties(|(mantissa, exponent)| {
        let f = T::from_integer_mantissa_and_exponent(mantissa, exponent).unwrap();
        if mantissa.odd() {
            assert_eq!(f.integer_mantissa_and_exponent(), (mantissa, exponent));
        }
    });
}

fn from_integer_mantissa_and_exponent_properties_helper_primitive_float<T: PrimitiveFloat>() {
    unsigned_signed_pair_gen_var_1().test_properties(|(mantissa, exponent)| {
        T::from_integer_mantissa_and_exponent(mantissa, exponent);
    });

    unsigned_signed_pair_gen_var_2::<T>().test_properties(|(mantissa, exponent)| {
        let f = T::from_integer_mantissa_and_exponent(mantissa, exponent).unwrap();
        assert!(!f.is_nan());
        assert_eq!(f.sign(), Greater);
        if !f.is_nan() && mantissa.odd() {
            assert_eq!(f.integer_mantissa_and_exponent(), (mantissa, exponent));
        }
    });
}

#[test]
fn from_integer_mantissa_and_exponent_properties() {
    apply_fn_to_unsigneds!(from_integer_mantissa_and_exponent_properties_helper_unsigned);
    apply_fn_to_primitive_floats!(
        from_integer_mantissa_and_exponent_properties_helper_primitive_float
    );
}
