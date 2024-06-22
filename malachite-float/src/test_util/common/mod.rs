// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{significand_bits, ComparableFloatRef, Float};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, NegativeZero, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{
    ExactFrom, FromStringBase, RoundingFrom, SciMantissaAndExponent,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use rug::float::{Round, Special};

// Can't have From impl due to orphan rule. We could define an impl in malachite-base where
// RoundingMode is defined, but pulling in rug::float just for that purpose seems overkill.
pub const fn rounding_mode_from_rug_round(rm: Round) -> RoundingMode {
    match rm {
        Round::Nearest => Nearest,
        Round::Zero => Down,
        Round::Up => Ceiling,
        Round::Down => Floor,
        Round::AwayZero => Up,
        _ => panic!(),
    }
}

#[allow(clippy::result_unit_err)]
pub const fn rug_round_try_from_rounding_mode(rm: RoundingMode) -> Result<Round, ()> {
    match rm {
        Floor => Ok(Round::Down),
        Ceiling => Ok(Round::Up),
        Down => Ok(Round::Zero),
        Up => Ok(Round::AwayZero),
        Nearest => Ok(Round::Nearest),
        Exact => Err(()),
    }
}

#[inline]
pub fn rug_round_exact_from_rounding_mode(rm: RoundingMode) -> Round {
    rug_round_try_from_rounding_mode(rm).unwrap()
}

impl From<&rug::Float> for Float {
    fn from(x: &rug::Float) -> Float {
        if x.is_nan() {
            Float::NAN
        } else if x.is_infinite() {
            if x.is_sign_positive() {
                Float::INFINITY
            } else {
                Float::NEGATIVE_INFINITY
            }
        } else if x.is_zero() {
            if x.is_sign_positive() {
                Float::ZERO
            } else {
                Float::NEGATIVE_ZERO
            }
        } else {
            let mut significand = Natural::exact_from(&*x.get_significand().unwrap());
            let precision = u64::from(x.prec());
            if significand.significant_bits() - precision >= Limb::WIDTH {
                // can only happen when 32_bit_limbs is set
                significand >>= Limb::WIDTH;
            }
            let result = Float(Finite {
                sign: x.is_sign_positive(),
                exponent: x.get_exp().unwrap(),
                precision,
                significand,
            });
            assert!(result.is_valid());
            result
        }
    }
}

fn convert_prec(prec: u64) -> Result<u32, ()> {
    u32::try_from(prec).map_err(|_| ())
}

#[allow(clippy::unnecessary_wraps)]
fn special_float(prec: u32, value: Special) -> Result<rug::Float, ()> {
    Ok(rug::Float::with_val_round(prec, value, Round::Zero).0)
}

pub fn rug_float_significant_bits(x: &rug::Float) -> u64 {
    if x.is_normal() {
        u64::from(x.prec())
    } else {
        1
    }
}

impl TryFrom<&Float> for rug::Float {
    type Error = ();

    fn try_from(x: &Float) -> Result<rug::Float, ()> {
        match x {
            float_nan!() => special_float(1, Special::Nan),
            float_infinity!() => special_float(1, Special::Infinity),
            float_negative_infinity!() => special_float(1, Special::NegInfinity),
            float_zero!() => special_float(1, Special::Zero),
            float_negative_zero!() => special_float(1, Special::NegZero),
            Float(Finite {
                sign,
                exponent,
                precision,
                significand,
            }) => {
                let mut f = rug::Float::with_val_round(
                    convert_prec(*precision)?,
                    rug::Integer::from(significand),
                    Round::Zero,
                )
                .0;
                f >>= i32::try_from(
                    i64::exact_from(significand_bits(significand)) - i64::from(*exponent),
                )
                .map_err(|_| ())?;
                if !sign {
                    f = -f;
                }
                Ok(f)
            }
        }
    }
}

pub fn parse_hex_string(s_hex: &str) -> Float {
    let x = Float::from_string_base(16, s_hex).unwrap();
    assert_eq!(format!("{:#x}", ComparableFloatRef(&x)), s_hex);
    x
}

pub fn to_hex_string(x: &Float) -> String {
    format!("{:#x}", ComparableFloatRef(x))
}

#[allow(clippy::type_repetition_in_bounds)]
pub fn emulate_primitive_float_fn<T: PrimitiveFloat, F: Fn(Float, u64) -> Float>(f: F, x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    let x = Float::from(x);
    let mut result = f(x.clone(), T::MANTISSA_WIDTH + 1);
    if !result.is_normal() {
        return T::exact_from(&result);
    }
    let e = i64::from(<&Float as SciMantissaAndExponent<Float, i32, _>>::sci_exponent(&result));
    if e < T::MIN_NORMAL_EXPONENT {
        if e < T::MIN_EXPONENT {
            return T::rounding_from(&result, Nearest).0;
        }
        result = f(x, T::max_precision_for_sci_exponent(e));
    }
    if result > T::MAX_FINITE {
        T::INFINITY
    } else if result < -T::MAX_FINITE {
        T::NEGATIVE_INFINITY
    } else {
        T::exact_from(&result)
    }
}

#[allow(clippy::type_repetition_in_bounds)]
pub fn emulate_primitive_float_fn_2<T: PrimitiveFloat, F: Fn(Float, Float, u64) -> Float>(
    f: F,
    x: T,
    y: T,
) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    let x = Float::from(x);
    let y = Float::from(y);
    let mut result = f(x.clone(), y.clone(), T::MANTISSA_WIDTH + 1);
    if !result.is_normal() {
        return T::exact_from(&result);
    }
    let e = i64::from(<&Float as SciMantissaAndExponent<Float, i32, _>>::sci_exponent(&result));
    if e < T::MIN_NORMAL_EXPONENT {
        if e < T::MIN_EXPONENT {
            return T::rounding_from(&result, Nearest).0;
        }
        result = f(x, y, T::max_precision_for_sci_exponent(e));
    }
    if result > T::MAX_FINITE {
        T::INFINITY
    } else if result < -T::MAX_FINITE {
        T::NEGATIVE_INFINITY
    } else {
        T::exact_from(&result)
    }
}

pub const ORDERED_FLOAT_STRINGS: [&str; 21] = [
    "-Infinity",
    "-3.1415926535897931",
    "-2.0",
    "-1.4142135623730951",
    "-1.0",
    "-1.0",
    "-1.0",
    "-0.5",
    "-0.33333333333333331",
    "-0.0",
    "NaN",
    "0.0",
    "0.33333333333333331",
    "0.5",
    "1.0",
    "1.0",
    "1.0",
    "1.4142135623730951",
    "2.0",
    "3.1415926535897931",
    "Infinity",
];

pub const ORDERED_FLOAT_HEX_STRINGS: [&str; 21] = [
    "-Infinity",
    "-0x3.243f6a8885a30#53",
    "-0x2.0#1",
    "-0x1.6a09e667f3bcd#53",
    "-0x1.0000000000000000000000000#100",
    "-0x1.0#2",
    "-0x1.0#1",
    "-0x0.8#1",
    "-0x0.55555555555554#53",
    "-0x0.0",
    "NaN",
    "0x0.0",
    "0x0.55555555555554#53",
    "0x0.8#1",
    "0x1.0#1",
    "0x1.0#2",
    "0x1.0000000000000000000000000#100",
    "0x1.6a09e667f3bcd#53",
    "0x2.0#1",
    "0x3.243f6a8885a30#53",
    "Infinity",
];

pub const ORDERED_F32S: [f32; 17] = [
    f32::NEGATIVE_INFINITY,
    -std::f32::consts::PI,
    -2.0,
    -std::f32::consts::SQRT_2,
    -1.0,
    -0.5,
    -1.0 / 3.0,
    -0.0,
    f32::NAN,
    0.0,
    1.0 / 3.0,
    0.5,
    1.0,
    std::f32::consts::SQRT_2,
    2.0,
    std::f32::consts::PI,
    f32::INFINITY,
];

pub const ORDERED_F64S: [f64; 17] = [
    f64::NEGATIVE_INFINITY,
    -std::f64::consts::PI,
    -2.0,
    -std::f64::consts::SQRT_2,
    -1.0,
    -0.5,
    -1.0 / 3.0,
    -0.0,
    f64::NAN,
    0.0,
    1.0 / 3.0,
    0.5,
    1.0,
    std::f64::consts::SQRT_2,
    2.0,
    std::f64::consts::PI,
    f64::INFINITY,
];
