use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{significand_bits, ComparableFloatRef, Float};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, NegativeZero, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{ExactFrom, FromStringBase};
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::Natural;
use rug::float::{Round, Special};

// Can't have From impl due to orphan rule. We could define an impl in malachite-base where
// RoundingMode is defined, but pulling in rug::float just for that purpose seems overkill.
pub fn rounding_mode_from_rug_round(rm: Round) -> RoundingMode {
    match rm {
        Round::Nearest => RoundingMode::Nearest,
        Round::Zero => RoundingMode::Down,
        Round::Up => RoundingMode::Ceiling,
        Round::Down => RoundingMode::Floor,
        Round::AwayZero => RoundingMode::Up,
        _ => panic!(),
    }
}

#[allow(clippy::result_unit_err)]
pub const fn rug_round_try_from_rounding_mode(rm: RoundingMode) -> Result<Round, ()> {
    match rm {
        RoundingMode::Floor => Ok(Round::Down),
        RoundingMode::Ceiling => Ok(Round::Up),
        RoundingMode::Down => Ok(Round::Zero),
        RoundingMode::Up => Ok(Round::AwayZero),
        RoundingMode::Nearest => Ok(Round::Nearest),
        RoundingMode::Exact => Err(()),
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
            Float(Finite {
                sign: x.is_sign_positive(),
                exponent: i64::from(x.get_exp().unwrap()),
                precision: u64::from(x.prec()),
                significand: Natural::exact_from(&*x.get_significand().unwrap()),
            })
        }
    }
}

fn convert_prec(prec: &u64) -> Result<u32, ()> {
    u32::try_from(*prec).map_err(|_| ())
}

fn special_float(prec: u32, value: Special) -> Result<rug::Float, ()> {
    Ok(rug::Float::with_val_round(prec, value, Round::Zero).0)
}

impl TryFrom<&Float> for rug::Float {
    type Error = ();

    fn try_from(x: &Float) -> Result<rug::Float, ()> {
        match x {
            float_nan!() => special_float(1, Special::Nan),
            float_infinity!() => special_float(1, Special::Infinity),
            float_negative_infinity!() => special_float(1, Special::NegInfinity),
            float_zero!() => special_float(1, Special::Zero),
            float_negative_zero!() => special_float(2, Special::NegZero),
            Float(Finite {
                sign,
                exponent,
                precision,
                significand,
            }) => {
                let mut f = rug::Float::with_val_round(
                    convert_prec(precision)?,
                    rug::Integer::from(significand),
                    Round::Zero,
                )
                .0;
                f >>= i32::try_from(i64::exact_from(significand_bits(significand)) - exponent)
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
