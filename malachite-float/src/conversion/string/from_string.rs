use crate::Float;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::conversion::string::options::FromSciStringOptions;
use malachite_base::num::conversion::traits::{FromSciString, FromStringBase};
use malachite_base::rounding_modes::RoundingMode;
use malachite_q::Rational;
use std::str::FromStr;

fn from_hex_string(s: &str) -> Float {
    match s {
        "NaN" => Float::NAN,
        "Infinity" => Float::INFINITY,
        "-Infinity" => Float::NEGATIVE_INFINITY,
        "0x0.0" => Float::ZERO,
        "-0x0.0" => Float::NEGATIVE_ZERO,
        s => {
            let (s, sign) = if let Some(s) = s.strip_prefix('-') {
                (s, false)
            } else {
                (s, true)
            };
            let s = s.strip_prefix("0x").unwrap();
            let hash_index = s.find('#').unwrap();
            let precision = u64::from_str(&s[hash_index + 1..]).unwrap();
            let mut options = FromSciStringOptions::default();
            options.set_base(16);
            let x = Float::from_rational_prec_round(
                Rational::from_sci_string_with_options(&s[..hash_index], options).unwrap(),
                precision,
                RoundingMode::Exact,
            )
            .0;
            if sign {
                x
            } else {
                -x
            }
        }
    }
}

impl FromStringBase for Float {
    fn from_string_base(base: u8, s: &str) -> Option<Self> {
        if base != 16 {
            panic!();
        }
        Some(from_hex_string(s))
    }
}
