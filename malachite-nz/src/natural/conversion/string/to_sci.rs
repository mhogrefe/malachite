// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::log_base::log_base_helper_with_pow;
use crate::natural::conversion::string::to_string::BaseFmtWrapper;
use crate::natural::slice_trailing_zeros;
use crate::natural::Natural;
use alloc::string::String;
use core::fmt::{Display, Formatter, Write};
use malachite_base::num::arithmetic::traits::{
    CheckedLogBase2, DivExact, DivRound, DivisibleBy, DivisibleByPowerOf2, FloorLogBase,
    FloorLogBasePowerOf2, Pow, ShrRound,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::string::options::{SciSizeOptions, ToSciOptions};
use malachite_base::num::conversion::string::to_sci::write_exponent;
use malachite_base::num::conversion::string::to_string::{
    digit_to_display_byte_lower, digit_to_display_byte_upper,
};
use malachite_base::num::conversion::traits::{Digits, ExactFrom, ToSci};
use malachite_base::rounding_modes::RoundingMode::*;

fn write_helper<T>(x: &T, f: &mut Formatter, options: ToSciOptions) -> core::fmt::Result
where
    for<'a> BaseFmtWrapper<&'a T>: Display,
{
    let w = BaseFmtWrapper {
        x,
        base: options.get_base(),
    };
    if options.get_lowercase() {
        Display::fmt(&w, f)
    } else {
        write!(f, "{w:#}")
    }
}

impl ToSci for Natural {
    /// Determines whether a [`Natural`] can be converted to a string using
    /// [`to_sci`](`Self::to_sci`) and a particular set of options.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::string::options::ToSciOptions;
    /// use malachite_base::num::conversion::traits::ToSci;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut options = ToSciOptions::default();
    /// assert!(Natural::from(123u8).fmt_sci_valid(options));
    /// assert!(Natural::from(u128::MAX).fmt_sci_valid(options));
    /// // u128::MAX has more than 16 significant digits
    /// options.set_rounding_mode(Exact);
    /// assert!(!Natural::from(u128::MAX).fmt_sci_valid(options));
    /// options.set_precision(50);
    /// assert!(Natural::from(u128::MAX).fmt_sci_valid(options));
    /// ```
    fn fmt_sci_valid(&self, options: ToSciOptions) -> bool {
        if *self == 0u32 || options.get_rounding_mode() != Exact {
            return true;
        }
        match options.get_size_options() {
            SciSizeOptions::Complete | SciSizeOptions::Scale(_) => true,
            SciSizeOptions::Precision(precision) => {
                let n_base = Natural::from(options.get_base());
                let log = self.floor_log_base(&n_base);
                if log < precision {
                    return true;
                }
                let scale = log - precision + 1;
                if let Some(base_log) = options.get_base().checked_log_base_2() {
                    self.divisible_by_power_of_2(base_log * scale)
                } else {
                    self.divisible_by(n_base.pow(scale))
                }
            }
        }
    }

    /// Converts a [`Natural`] to a string using a specified base, possibly formatting the number
    /// using scientific notation.
    ///
    /// See [`ToSciOptions`] for details on the available options. Note that setting
    /// `neg_exp_threshold` has no effect, since there is never a need to use negative exponents
    /// when representing a [`Natural`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `options.rounding_mode` is `Exact`, but the size options are such that the input
    /// must be rounded.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::string::options::ToSciOptions;
    /// use malachite_base::num::conversion::traits::ToSci;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     format!("{}", Natural::from(u128::MAX).to_sci()),
    ///     "3.402823669209385e38"
    /// );
    /// assert_eq!(
    ///     Natural::from(u128::MAX).to_sci().to_string(),
    ///     "3.402823669209385e38"
    /// );
    ///
    /// let n = Natural::from(123456u32);
    /// let mut options = ToSciOptions::default();
    /// assert_eq!(format!("{}", n.to_sci_with_options(options)), "123456");
    /// assert_eq!(n.to_sci_with_options(options).to_string(), "123456");
    ///
    /// options.set_precision(3);
    /// assert_eq!(n.to_sci_with_options(options).to_string(), "1.23e5");
    ///
    /// options.set_rounding_mode(Ceiling);
    /// assert_eq!(n.to_sci_with_options(options).to_string(), "1.24e5");
    ///
    /// options.set_e_uppercase();
    /// assert_eq!(n.to_sci_with_options(options).to_string(), "1.24E5");
    ///
    /// options.set_force_exponent_plus_sign(true);
    /// assert_eq!(n.to_sci_with_options(options).to_string(), "1.24E+5");
    ///
    /// options = ToSciOptions::default();
    /// options.set_base(36);
    /// assert_eq!(n.to_sci_with_options(options).to_string(), "2n9c");
    ///
    /// options.set_uppercase();
    /// assert_eq!(n.to_sci_with_options(options).to_string(), "2N9C");
    ///
    /// options.set_base(2);
    /// options.set_precision(10);
    /// assert_eq!(n.to_sci_with_options(options).to_string(), "1.1110001e16");
    ///
    /// options.set_include_trailing_zeros(true);
    /// assert_eq!(n.to_sci_with_options(options).to_string(), "1.111000100e16");
    /// ```
    fn fmt_sci(&self, f: &mut Formatter, options: ToSciOptions) -> core::fmt::Result {
        match options.get_size_options() {
            SciSizeOptions::Complete | SciSizeOptions::Scale(0) => write_helper(self, f, options),
            SciSizeOptions::Scale(scale) => {
                write_helper(self, f, options)?;
                if options.get_include_trailing_zeros() {
                    f.write_char('.')?;
                    for _ in 0..scale {
                        f.write_char('0')?;
                    }
                }
                Ok(())
            }
            SciSizeOptions::Precision(precision) => {
                let n_base = Natural::from(options.get_base());
                let (base_log, log, power, p) = if *self == 0u32 {
                    // power and p unused
                    (None, 0, Natural::ZERO, 0)
                } else if let Some(base_log) = options.get_base().checked_log_base_2() {
                    // power and p unused
                    (
                        Some(base_log),
                        self.floor_log_base_power_of_2(base_log),
                        Natural::ZERO,
                        0,
                    )
                } else {
                    // We save base^p so that we can save some computation later
                    let (log, _, power, p) = log_base_helper_with_pow(self, &n_base);
                    (None, log, power, p)
                };
                if log < precision {
                    // no exponent
                    write_helper(self, f, options)?;
                    if options.get_include_trailing_zeros() {
                        let extra_zeros = precision - log - 1;
                        if extra_zeros != 0 {
                            f.write_char('.')?;
                            for _ in 0..extra_zeros {
                                f.write_char('0')?;
                            }
                        }
                    }
                    Ok(())
                } else {
                    // exponent
                    let mut e = log;
                    let scale = log - precision + 1;
                    let shifted = if let Some(base_log) = base_log {
                        self.shr_round(base_log * scale, options.get_rounding_mode())
                            .0
                    } else {
                        let n = if precision > log >> 1 {
                            n_base.pow(scale)
                        } else if p >= scale {
                            power.div_exact(n_base.pow(p - scale))
                        } else {
                            // Not sure if this ever happens
                            assert!(p == scale + 1);
                            power * n_base
                        };
                        self.div_round(n, options.get_rounding_mode()).0
                    };
                    let mut chars = shifted.to_digits_desc(&options.get_base());
                    let mut len = chars.len();
                    let p = usize::exact_from(precision);
                    if len > p {
                        // rounded up to a power of the base, need to reduce precision
                        assert_eq!(chars.pop().unwrap(), 0);
                        len -= 1;
                        e += 1;
                    }
                    assert_eq!(len, p);
                    if !options.get_include_trailing_zeros() {
                        chars.truncate(len - slice_trailing_zeros(&chars));
                    }
                    if options.get_lowercase() {
                        for digit in &mut chars {
                            *digit = digit_to_display_byte_lower(*digit).unwrap();
                        }
                    } else {
                        for digit in &mut chars {
                            *digit = digit_to_display_byte_upper(*digit).unwrap();
                        }
                    }
                    len = chars.len();
                    if len != 1 {
                        chars.push(b'0');
                        chars.copy_within(1..len, 2);
                        chars[1] = b'.';
                    }
                    f.write_str(&String::from_utf8(chars).unwrap())?;
                    write_exponent(f, options, e)
                }
            }
        }
    }
}
