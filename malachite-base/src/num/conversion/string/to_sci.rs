// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{CheckedLogBase2, NegAssign, Pow, UnsignedAbs};
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::string::options::{SciSizeOptions, ToSciOptions};
use crate::num::conversion::string::to_string::BaseFmtWrapper;
use crate::num::conversion::string::to_string::{
    digit_to_display_byte_lower, digit_to_display_byte_upper,
};
use crate::num::conversion::traits::{ExactFrom, ToSci};
use crate::rounding_modes::RoundingMode::*;
use crate::slices::slice_trailing_zeros;
use alloc::string::String;
use core::fmt::{Display, Formatter, Write};

/// A `struct` that can be used to format a number in scientific notation.
pub struct SciWrapper<'a, T: ToSci> {
    pub(crate) x: &'a T,
    pub(crate) options: ToSciOptions,
}

impl<T: ToSci> Display for SciWrapper<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        self.x.fmt_sci(f, self.options)
    }
}

#[doc(hidden)]
pub fn write_exponent<T: PrimitiveInt>(
    f: &mut Formatter,
    options: ToSciOptions,
    exp: T,
) -> core::fmt::Result {
    f.write_char(if options.get_e_lowercase() { 'e' } else { 'E' })?;
    if exp > T::ZERO && (options.get_force_exponent_plus_sign() || options.get_base() >= 15) {
        f.write_char('+')?;
    }
    write!(f, "{exp}")
}

fn write_helper<T>(x: T, f: &mut Formatter, options: ToSciOptions) -> core::fmt::Result
where
    BaseFmtWrapper<T>: Display,
{
    let w = BaseFmtWrapper {
        x,
        base: options.base,
    };
    if options.lowercase {
        Display::fmt(&w, f)
    } else {
        write!(f, "{w:#}")
    }
}

fn fmt_sci_valid_unsigned<T: PrimitiveUnsigned>(x: T, options: ToSciOptions) -> bool {
    if x == T::ZERO || options.rounding_mode != Exact {
        return true;
    }
    match options.size_options {
        SciSizeOptions::Complete | SciSizeOptions::Scale(_) => true,
        SciSizeOptions::Precision(precision) => {
            let t_base = T::from(options.base);
            let log = x.floor_log_base(t_base);
            if log < precision {
                return true;
            }
            let neg_scale = log - precision + 1;
            if let Some(base_log) = options.base.checked_log_base_2() {
                x.divisible_by_power_of_2(base_log * neg_scale)
            } else {
                x.divisible_by(Pow::pow(t_base, neg_scale))
            }
        }
    }
}

fn fmt_sci_unsigned<T: PrimitiveUnsigned>(
    mut x: T,
    f: &mut Formatter,
    options: ToSciOptions,
) -> core::fmt::Result
where
    BaseFmtWrapper<T>: Display,
{
    match options.size_options {
        SciSizeOptions::Complete | SciSizeOptions::Scale(0) => write_helper(x, f, options),
        SciSizeOptions::Scale(scale) => {
            write_helper(x, f, options)?;
            if options.include_trailing_zeros {
                f.write_char('.')?;
                for _ in 0..scale {
                    f.write_char('0')?;
                }
            }
            Ok(())
        }
        SciSizeOptions::Precision(precision) => {
            let t_base = T::from(options.base);
            let log = if x == T::ZERO {
                0
            } else {
                x.floor_log_base(t_base)
            };
            if log < precision {
                // no exponent
                write_helper(x, f, options)?;
                if options.include_trailing_zeros {
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
                let neg_scale = log - precision + 1;
                if let Some(base_log) = options.base.checked_log_base_2() {
                    x.shr_round_assign(base_log * neg_scale, options.rounding_mode);
                } else {
                    x.div_round_assign(Pow::pow(t_base, neg_scale), options.rounding_mode);
                }
                let mut chars = x.to_digits_desc(&options.base);
                let mut len = chars.len();
                let p = usize::exact_from(precision);
                if len > p {
                    // rounded up to a power of the base, need to reduce precision
                    chars.pop();
                    len -= 1;
                    e += 1;
                }
                assert_eq!(len, p);
                if !options.include_trailing_zeros {
                    chars.truncate(len - slice_trailing_zeros(&chars));
                }
                if options.lowercase {
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

#[inline]
fn fmt_sci_valid_signed<T: PrimitiveSigned>(x: T, options: ToSciOptions) -> bool
where
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    fmt_sci_valid_unsigned(x.unsigned_abs(), options)
}

fn fmt_sci_signed<T: PrimitiveSigned>(
    x: T,
    f: &mut Formatter,
    mut options: ToSciOptions,
) -> core::fmt::Result
where
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    let abs = x.unsigned_abs();
    if x >= T::ZERO {
        abs.fmt_sci(f, options)
    } else {
        options.rounding_mode.neg_assign();
        f.write_char('-')?;
        abs.fmt_sci(f, options)
    }
}

macro_rules! impl_to_sci_unsigned {
    ($t:ident) => {
        impl ToSci for $t {
            /// Determines whether an unsigned number can be converted to a string using
            /// [`to_sci_with_options`](super::super::traits::ToSci::to_sci_with_options) and a
            /// particular set of options.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::to_sci#fmt_sci_valid).
            #[inline]
            fn fmt_sci_valid(&self, options: ToSciOptions) -> bool {
                fmt_sci_valid_unsigned(*self, options)
            }

            /// Converts an unsigned number to a string using a specified base, possibly formatting
            /// the number using scientific notation.
            ///
            /// See [`ToSciOptions`] for details on the available options. Note that setting
            /// `neg_exp_threshold` has no effect, since there is never a need to use negative
            /// exponents when representing an integer.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `options.rounding_mode` is `Exact`, but the size options are such that the
            /// input must be rounded.
            ///
            /// # Examples
            /// See [here](super::to_sci).
            #[inline]
            fn fmt_sci(&self, f: &mut Formatter, options: ToSciOptions) -> core::fmt::Result {
                fmt_sci_unsigned(*self, f, options)
            }
        }
    };
}
apply_to_unsigneds!(impl_to_sci_unsigned);

macro_rules! impl_to_sci_signed {
    ($t:ident) => {
        impl ToSci for $t {
            /// Determines whether a signed number can be converted to a string using
            /// [`to_sci_with_options`](super::super::traits::ToSci::to_sci_with_options) and a
            /// particular set of options.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::to_sci#fmt_sci_valid).
            #[inline]
            fn fmt_sci_valid(&self, options: ToSciOptions) -> bool {
                fmt_sci_valid_signed(*self, options)
            }

            /// Converts a signed number to a string using a specified base, possibly formatting the
            /// number using scientific notation.
            ///
            /// See [`ToSciOptions`] for details on the available options. Note that setting
            /// `neg_exp_threshold` has no effect, since there is never a need to use negative
            /// exponents when representing an integer.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `options.rounding_mode` is `Exact`, but the size options are such that the
            /// input must be rounded.
            ///
            /// # Examples
            /// See [here](super::to_sci).
            #[inline]
            fn fmt_sci(&self, f: &mut Formatter, options: ToSciOptions) -> core::fmt::Result {
                fmt_sci_signed(*self, f, options)
            }
        }
    };
}
apply_to_signeds!(impl_to_sci_signed);
