// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use alloc::string::{String, ToString};
use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::conversion::string::options::FromSciStringOptions;
use malachite_base::num::conversion::traits::{FromSciString, FromStringBase};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_q::Rational;

fn reduce_exponent_in_hex_string(s: &str) -> Option<(String, i32)> {
    if let Some(exp_index) = s.find('E') {
        let tail = &s[exp_index + 1..];
        let hash_index = tail.find('#').unwrap();
        let tail = &tail[..hash_index];
        let original_exponent = i32::from_str(tail).unwrap();
        if original_exponent.unsigned_abs() < 20 {
            return None;
        }
        let mut new_s = s[..=exp_index].to_string();
        new_s += "+20";
        new_s += &s[exp_index + hash_index + 1..];
        Some((new_s, (original_exponent << 2) - 80))
    } else {
        None
    }
}

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
            let x = if let Some((alt_s, exp_offset)) = reduce_exponent_in_hex_string(s) {
                let hash_index = alt_s.find('#').unwrap();
                Float::from_rational_prec_round(
                    Rational::from_sci_string_with_options(&alt_s[..hash_index], options).unwrap(),
                    precision,
                    Exact,
                )
                .0 << exp_offset
            } else {
                Float::from_rational_prec_round(
                    Rational::from_sci_string_with_options(&s[..hash_index], options).unwrap(),
                    precision,
                    Exact,
                )
                .0
            };
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
        assert_eq!(base, 16);
        Some(from_hex_string(s))
    }
}
