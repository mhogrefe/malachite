// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::alloc::string::ToString;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{ComparableFloat, ComparableFloatRef, Float};
use alloc::string::String;
use core::fmt::{Debug, Display, Formatter, LowerHex, Result, Write};
use malachite_base::num::arithmetic::traits::{
    Abs, ModPowerOf2, RoundToMultipleOfPowerOf2, ShrRound,
};
use malachite_base::num::conversion::string::options::ToSciOptions;
use malachite_base::num::conversion::traits::{ExactFrom, ToSci, WrappingFrom};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_q::Rational;

fn replace_exponent_in_hex_string(s: &str, new_exponent: i32) -> String {
    let exp_index = s.find('E').unwrap_or_else(|| panic!("{s}"));
    let mut new_s = s[..exp_index].to_string();
    if new_exponent > 0 {
        write!(new_s, "E+{new_exponent}").unwrap();
    } else {
        write!(new_s, "E{new_exponent}").unwrap();
    }
    new_s
}

impl Display for Float {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            float_nan!() => write!(f, "NaN"),
            float_infinity!() => write!(f, "Infinity"),
            float_negative_infinity!() => write!(f, "-Infinity"),
            float_zero!() => write!(f, "0.0"),
            float_negative_zero!() => write!(f, "-0.0"),
            _ => {
                let exp = self.get_exponent().unwrap();
                if exp.unsigned_abs() > 10000 {
                    // The current slow implementation would take forever to try to convert a Float
                    // with a very large or small exponent to a decimal string. Best to
                    // short-circuit it for now.
                    if *self < 0u32 {
                        write!(f, "-")?;
                    }
                    return write!(f, "{}", if exp >= 0 { "too_big" } else { "too_small" });
                }
                let mut lower = self.clone();
                let mut higher = self.clone();
                lower.decrement();
                higher.increment();
                let self_q = Rational::exact_from(self);
                let lower_q = Rational::exact_from(lower);
                let higher_q = Rational::exact_from(higher);
                let mut options = ToSciOptions::default();
                for precision in 1.. {
                    options.set_precision(precision);
                    let s = self_q.to_sci_with_options(options).to_string();
                    let s_lower = lower_q.to_sci_with_options(options).to_string();
                    let s_higher = higher_q.to_sci_with_options(options).to_string();
                    if s != s_lower && s != s_higher {
                        return if s.contains('.') {
                            write!(f, "{s}")
                        } else if let Some(i) = s.find('e') {
                            write!(f, "{}.0e{}", &s[..i], &s[i + 1..])
                        } else {
                            write!(f, "{s}.0")
                        };
                    }
                }
                panic!();
            }
        }
    }
}

impl Debug for Float {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}

impl LowerHex for Float {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            float_zero!() => f.write_str(if f.alternate() { "0x0.0" } else { "0.0" }),
            float_negative_zero!() => f.write_str(if f.alternate() { "-0x0.0" } else { "-0.0" }),
            Float(Finite {
                exponent,
                precision,
                ..
            }) => {
                if self.is_sign_negative() {
                    f.write_char('-')?;
                }
                let mut options = ToSciOptions::default();
                options.set_base(16);
                let m = u64::from(exponent.mod_power_of_2(2));
                let mut p = precision.saturating_sub(m).shr_round(2, Ceiling).0;
                if m != 0 {
                    p += 1;
                }
                options.set_precision(p);
                options.set_e_uppercase();
                options.set_include_trailing_zeros(true);
                if f.alternate() {
                    f.write_str("0x")?;
                }
                let pr = precision.round_to_multiple_of_power_of_2(5, Up).0;
                let s = if u64::from(exponent.unsigned_abs()) > (pr << 2) {
                    let new_exponent = if *exponent > 0 {
                        i32::exact_from(pr << 1)
                    } else {
                        -i32::exact_from(pr << 1)
                    } + i32::wrapping_from(exponent.mod_power_of_2(2));
                    let mut s = Rational::exact_from(self >> (exponent - new_exponent))
                        .abs()
                        .to_sci_with_options(options)
                        .to_string();
                    s = replace_exponent_in_hex_string(&s, (exponent - 1).shr_round(2, Floor).0);
                    s
                } else {
                    Rational::exact_from(self)
                        .abs()
                        .to_sci_with_options(options)
                        .to_string()
                };
                if s.contains('.') {
                    write!(f, "{s}")
                } else if let Some(i) = s.find('E') {
                    write!(f, "{}.0E{}", &s[..i], &s[i + 1..])
                } else {
                    write!(f, "{s}.0")
                }
            }
            _ => Display::fmt(&self, f),
        }
    }
}

impl Display for ComparableFloat {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(&ComparableFloatRef(&self.0), f)
    }
}

impl Debug for ComparableFloat {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(&ComparableFloatRef(&self.0), f)
    }
}

impl LowerHex for ComparableFloat {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        LowerHex::fmt(&ComparableFloatRef(&self.0), f)
    }
}

impl Display for ComparableFloatRef<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let x @ Float(Finite { precision, .. }) = &self.0 {
            write!(f, "{x}")?;
            f.write_char('#')?;
            write!(f, "{precision}")
        } else {
            Display::fmt(&self.0, f)
        }
    }
}

impl LowerHex for ComparableFloatRef<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let x @ Float(Finite { precision, .. }) = &self.0 {
            if f.alternate() {
                write!(f, "{x:#x}")?;
            } else {
                write!(f, "{x:x}")?;
            }
            f.write_char('#')?;
            write!(f, "{precision}")
        } else {
            LowerHex::fmt(&self.0, f)
        }
    }
}

impl Debug for ComparableFloatRef<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}
