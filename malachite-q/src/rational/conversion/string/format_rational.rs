// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational::Rational;
use alloc::string::String;
use malachite_base::strings::gmp_format::{GmpConversionSpec, GmpFormatArg};
use malachite_nz::natural::conversion::string::format_natural::{
    format_gmp_integer_spec, format_gmp_str,
};

/// Formats a [`Rational`] according to a GMP-style `printf` format string, for strict compatibility
/// with GMP's `gmp_printf` family.
///
/// The format string should contain a single conversion consuming the [`Rational`], written
/// `%[flags][width][.precision]Q[conv]`, with any surrounding literal text (a literal `%` is
/// written `%%`). The value is written as the numerator's digits, and, unless the denominator is 1,
/// a slash and the denominator's digits, as with `mpq_get_str`. The pieces are:
/// - **flags**: any of `-` (left-justify within the field), `+` (always show a sign), space (show a
///   space before a nonnegative value), `#` (alternate form: prefix hexadecimal output with `0x` or
///   `0X` and octal output with `0`, on the numerator and on the denominator, unless the respective
///   digits already begin with a zero), and `0` (pad the field with leading zeros). The `'` flag is
///   accepted but, as in GMP, has no effect on GMP types.
/// - **width**: the minimum field width, as a decimal integer; the numerator, slash, and
///   denominator are justified as a unit.
/// - **precision**: following a `.`, a minimum digit count; GMP documents the influence of the
///   precision on a rational as undefined, and this function reproduces what its code does, padding
///   the numerator with leading zeros until the whole string, slash and denominator included,
///   reaches the precision.
/// - **`Q`**: marks the argument as a multiple-precision rational (GMP's type character).
/// - **conv**: the conversion — `d`, `i`, or `u` (decimal; all three are the same, and a negative
///   value keeps its sign under every conversion), `o` (octal), or `x`/`X` (lowercase/uppercase
///   hexadecimal).
///
/// A negative value is written as a `-` followed by the absolute value, and with the `#` flag the
/// sign precedes the numerator's base prefix, as in `-0xff/0x10`.
///
/// Returns [`None`] when the format string is not a single well-formed `%Q` conversion: for
/// instance if it uses `*` for the width or precision (which would need an integer argument that
/// this single-value entry point does not supply), contains no `%Q` conversion or more than one,
/// contains a conversion of any other type, or requests a width or precision beyond `i32::MAX` (the
/// range of the C `int` GMP itself stores them in).
///
/// # Worst-case complexity
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(), p, w)`, with
/// `p` and `w` the precision and field width requested by the format string.
///
/// # Examples
/// ```
/// use malachite_q::rational::conversion::string::format_rational::format_rational_str;
/// use malachite_q::Rational;
///
/// let x = Rational::from_signeds(255, 16);
/// assert_eq!(format_rational_str(&x, "%Qd"), Some("255/16".to_string()));
/// assert_eq!(
///     format_rational_str(&x, "%#Qx"),
///     Some("0xff/0x10".to_string())
/// );
/// assert_eq!(
///     format_rational_str(&x, "%10Qd"),
///     Some("    255/16".to_string())
/// );
/// assert_eq!(format_rational_str(&-x, "%Qd"), Some("-255/16".to_string()));
///
/// // a denominator of 1 is omitted, as with `mpq_get_str`
/// assert_eq!(
///     format_rational_str(&Rational::from(255), "%Qd"),
///     Some("255".to_string())
/// );
///
/// // invalid or unsupported format strings
/// let x = Rational::from_signeds(255, 16);
/// assert_eq!(format_rational_str(&x, "%Zd"), None);
/// assert_eq!(format_rational_str(&x, "%*Qd"), None);
/// assert_eq!(format_rational_str(&x, "%Qd %Qd"), None);
/// ```
///
/// This is `gmp_snprintf` from `printf/snprintf.c`, GMP 6.3.0, where the format string contains a
/// single `%Q` conversion and the buffer is always large enough.
pub fn format_rational_str(x: &Rational, fmt: &str) -> Option<String> {
    let den = x.denominator_ref();
    format_gmp_str(
        *x < 0u32,
        x.numerator_ref(),
        if *den == 1u32 { None } else { Some(den) },
        b'Q',
        fmt,
    )
}

impl GmpFormatArg for Rational {
    /// Formats a [`Rational`] according to a single parsed conversion specification, which must be
    /// a `%Q` integer conversion; see
    /// [`gmp_format`](malachite_base::strings::gmp_format::gmp_format) and [`format_rational_str`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(), p,
    /// w)`, with `p` and `w` the precision and field width of the specification.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::gmp_format;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     gmp_format!("%Qd of them", Rational::from_signeds(255, 16)),
    ///     Some("255/16 of them".to_string())
    /// );
    /// ```
    fn gmp_format(&self, spec: &GmpConversionSpec) -> Option<String> {
        if spec.type_chr != b'Q' {
            return None;
        }
        let den = self.denominator_ref();
        format_gmp_integer_spec(
            *self < 0u32,
            self.numerator_ref(),
            if *den == 1u32 { None } else { Some(den) },
            spec,
        )
    }
}
