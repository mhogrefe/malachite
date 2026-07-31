// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::conversion::string::format_natural::{format_gmp_integer_spec, format_gmp_str};
use alloc::string::String;
use malachite_base::strings::gmp_format::{GmpConversionSpec, GmpFormatArg};

/// Formats an [`Integer`] according to a GMP-style `printf` format string, for strict compatibility
/// with GMP's `gmp_printf` family.
///
/// The format string should contain a single conversion consuming the [`Integer`], written
/// `%[flags][width][.precision]Z[conv]`, with any surrounding literal text (a literal `%` is
/// written `%%`). The pieces are:
/// - **flags**: any of `-` (left-justify within the field), `+` (always show a sign), space (show a
///   space before a nonnegative value), `#` (alternate form: prefix hexadecimal output with `0x` or
///   `0X` and octal output with `0`, unless the digits already begin with a zero), and `0` (pad the
///   field with leading zeros). The `'` flag is accepted but, as in GMP, has no effect on GMP
///   types.
/// - **width**: the minimum field width, as a decimal integer.
/// - **precision**: following a `.`, the minimum number of digits, the absolute value being padded
///   with leading zeros to reach it; a zero value formatted with a precision of 0 produces no
///   digits at all. By default all necessary digits are printed.
/// - **`Z`**: marks the argument as a multiple-precision integer (GMP's type character).
/// - **conv**: the conversion — `d`, `i`, or `u` (decimal; unlike in C, all three are the same,
///   and a negative value keeps its sign under every conversion), `o` (octal), or `x`/`X`
///   (lowercase/uppercase hexadecimal).
///
/// A negative value is written as a `-` followed by the absolute value's digits, under every
/// conversion; with the `#` flag the sign precedes the base prefix, as in `-0xff`.
///
/// Returns [`None`] when the format string is not a single well-formed `%Z` integer conversion: for
/// instance if it uses `*` for the width or precision (which would need an integer argument that
/// this single-value entry point does not supply), contains no `%Z` conversion or more than one,
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
/// use malachite_nz::integer::conversion::string::format_integer::format_integer_str;
/// use malachite_nz::integer::Integer;
///
/// let x = Integer::from(-255);
/// assert_eq!(format_integer_str(&x, "%Zd"), Some("-255".to_string()));
/// assert_eq!(format_integer_str(&x, "%#Zx"), Some("-0xff".to_string()));
/// assert_eq!(format_integer_str(&x, "%#ZX"), Some("-0XFF".to_string()));
/// assert_eq!(format_integer_str(&x, "%#Zo"), Some("-0377".to_string()));
/// assert_eq!(format_integer_str(&x, "%8Zd"), Some("    -255".to_string()));
/// assert_eq!(
///     format_integer_str(&x, "%08Zd"),
///     Some("-0000255".to_string())
/// );
/// assert_eq!(format_integer_str(&x, "%.6Zd"), Some("-000255".to_string()));
///
/// let x = Integer::from(255);
/// assert_eq!(format_integer_str(&x, "%+Zd"), Some("+255".to_string()));
/// assert_eq!(format_integer_str(&x, "% Zd"), Some(" 255".to_string()));
/// assert_eq!(
///     format_integer_str(&x, "x = %Zd!"),
///     Some("x = 255!".to_string())
/// );
///
/// // invalid or unsupported format strings
/// assert_eq!(format_integer_str(&x, "%d"), None);
/// assert_eq!(format_integer_str(&x, "%*Zd"), None);
/// assert_eq!(format_integer_str(&x, "%Zd %Zd"), None);
/// ```
///
/// This is `gmp_snprintf` from `printf/snprintf.c`, GMP 6.3.0, where the format string contains a
/// single `%Z` integer conversion and the buffer is always large enough.
#[inline]
pub fn format_integer_str(x: &Integer, fmt: &str) -> Option<String> {
    format_gmp_str(*x < 0, x.unsigned_abs_ref(), None, b'Z', fmt)
}

impl GmpFormatArg for Integer {
    /// Formats an [`Integer`] according to a single parsed conversion specification, which must be
    /// a `%Z` integer conversion; see
    /// [`gmp_format`](malachite_base::strings::gmp_format::gmp_format) and [`format_integer_str`].
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     gmp_format!("%+Zd and %#Zx", Integer::from(255), Integer::from(-255)),
    ///     Some("+255 and -0xff".to_string())
    /// );
    /// ```
    fn gmp_format(&self, spec: &GmpConversionSpec) -> Option<String> {
        if spec.type_chr != b'Z' {
            return None;
        }
        format_gmp_integer_spec(*self < 0, self.unsigned_abs_ref(), None, spec)
    }
}
