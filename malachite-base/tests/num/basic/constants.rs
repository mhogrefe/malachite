// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::named::Named;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, OneHalf, Two, Zero,
};
use malachite_base::num::float::NiceFloat;

macro_rules! test_unsigned_constants {
    ($t: ident) => {
        assert_eq!($t::ZERO, 0);
        assert_eq!($t::ONE, 1);
        assert_eq!($t::TWO, 2);
    };
}

macro_rules! test_signed_constants {
    ($t: ident) => {
        test_unsigned_constants!($t);
        assert_eq!($t::NEGATIVE_ONE, -1);
    };
}

macro_rules! test_float_constants {
    ($t: ident) => {
        assert_eq!($t::ZERO, 0.0);
        assert_eq!($t::ONE, 1.0);
        assert_eq!($t::TWO, 2.0);
        assert_eq!($t::NEGATIVE_ONE, -1.0);
        assert_eq!($t::NEGATIVE_ZERO, -0.0);
        assert_eq!($t::ONE_HALF, 0.5);
    };
}

#[test]
fn test_constants() {
    apply_to_unsigneds!(test_unsigned_constants);
    apply_to_signeds!(test_signed_constants);
    apply_to_primitive_floats!(test_float_constants);
}

#[test]
fn test_width_constants() {
    assert_eq!(u8::WIDTH, 8);
    assert_eq!(u8::LOG_WIDTH, 3);
    assert_eq!(u8::WIDTH_MASK, 0x7);

    assert_eq!(u16::WIDTH, 16);
    assert_eq!(u16::LOG_WIDTH, 4);
    assert_eq!(u16::WIDTH_MASK, 0xf);

    assert_eq!(u32::WIDTH, 32);
    assert_eq!(u32::LOG_WIDTH, 5);
    assert_eq!(u32::WIDTH_MASK, 0x1f);

    assert_eq!(u64::WIDTH, 64);
    assert_eq!(u64::LOG_WIDTH, 6);
    assert_eq!(u64::WIDTH_MASK, 0x3f);

    assert_eq!(u128::WIDTH, 128);
    assert_eq!(u128::LOG_WIDTH, 7);
    assert_eq!(u128::WIDTH_MASK, 0x7f);

    assert_eq!(i8::WIDTH, 8);
    assert_eq!(i8::LOG_WIDTH, 3);
    assert_eq!(i8::WIDTH_MASK, 0x7);

    assert_eq!(i16::WIDTH, 16);
    assert_eq!(i16::LOG_WIDTH, 4);
    assert_eq!(i16::WIDTH_MASK, 0xf);

    assert_eq!(i32::WIDTH, 32);
    assert_eq!(i32::LOG_WIDTH, 5);
    assert_eq!(i32::WIDTH_MASK, 0x1f);

    assert_eq!(i64::WIDTH, 64);
    assert_eq!(i64::LOG_WIDTH, 6);
    assert_eq!(i64::WIDTH_MASK, 0x3f);

    assert_eq!(i128::WIDTH, 128);
    assert_eq!(i128::LOG_WIDTH, 7);
    assert_eq!(i128::WIDTH_MASK, 0x7f);
}

#[test]
fn test_other_float_constants() {
    assert_eq!(f32::WIDTH, 32);
    assert_eq!(f32::EXPONENT_WIDTH, 8);
    assert_eq!(f32::MANTISSA_WIDTH, 23);
    assert_eq!(f32::MIN_NORMAL_EXPONENT, -126);
    assert_eq!(f32::MIN_EXPONENT, -149);
    assert_eq!(f32::MAX_EXPONENT, 127);
    assert_eq!(NiceFloat(f32::MIN_POSITIVE_SUBNORMAL), NiceFloat(1.0e-45));
    assert_eq!(NiceFloat(f32::MAX_SUBNORMAL), NiceFloat(1.1754942e-38));
    assert_eq!(
        NiceFloat(f32::MIN_POSITIVE_NORMAL),
        NiceFloat(1.1754944e-38)
    );
    assert_eq!(NiceFloat(f32::MAX_FINITE), NiceFloat(3.4028235e38));
    assert_eq!(NiceFloat(Infinity::INFINITY), NiceFloat(f32::INFINITY));
    assert_eq!(
        NiceFloat(f32::NEGATIVE_INFINITY),
        NiceFloat(f32::NEG_INFINITY)
    );
    assert_eq!(NiceFloat(NaN::NAN), NiceFloat(f32::NAN));
    assert_eq!(f32::SMALLEST_UNREPRESENTABLE_UINT, 0x1000001);
    assert_eq!(f32::LARGEST_ORDERED_REPRESENTATION, 0xff000001);

    assert_eq!(f64::WIDTH, 64);
    assert_eq!(f64::EXPONENT_WIDTH, 11);
    assert_eq!(f64::MANTISSA_WIDTH, 52);
    assert_eq!(f64::MIN_NORMAL_EXPONENT, -1022);
    assert_eq!(f64::MIN_EXPONENT, -1074);
    assert_eq!(f64::MAX_EXPONENT, 1023);
    assert_eq!(NiceFloat(f64::MIN_POSITIVE_SUBNORMAL), NiceFloat(5.0e-324));
    assert_eq!(
        NiceFloat(f64::MAX_SUBNORMAL),
        NiceFloat(2.225073858507201e-308)
    );
    assert_eq!(
        NiceFloat(f64::MIN_POSITIVE_NORMAL),
        NiceFloat(2.2250738585072014e-308)
    );
    assert_eq!(
        NiceFloat(f64::MAX_FINITE),
        NiceFloat(1.7976931348623157e308)
    );
    assert_eq!(NiceFloat(Infinity::INFINITY), NiceFloat(f64::INFINITY));
    assert_eq!(
        NiceFloat(f64::NEGATIVE_INFINITY),
        NiceFloat(f64::NEG_INFINITY)
    );
    assert_eq!(NiceFloat(NaN::NAN), NiceFloat(f64::NAN));
    assert_eq!(f32::SMALLEST_UNREPRESENTABLE_UINT, 0x1000001);
    assert_eq!(f64::LARGEST_ORDERED_REPRESENTATION, 0xffe0000000000001);
}

#[test]
pub fn test_named() {
    fn test<T: Named>(out: &str) {
        assert_eq!(T::NAME, out);
    }
    test::<u8>("u8");
    test::<u16>("u16");
    test::<u32>("u32");
    test::<u64>("u64");
    test::<u128>("u128");
    test::<usize>("usize");
    test::<i8>("i8");
    test::<i16>("i16");
    test::<i32>("i32");
    test::<i64>("i64");
    test::<i128>("i128");
    test::<isize>("isize");
    test::<f32>("f32");
    test::<f64>("f64");
}
