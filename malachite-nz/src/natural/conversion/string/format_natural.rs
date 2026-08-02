// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1993-2019 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use alloc::string::String;
use alloc::vec::Vec;
use core::cmp::max;
use malachite_base::fail_on_untested_path;
use malachite_base::num::conversion::traits::ToStringBase;
use malachite_base::strings::gmp_format::{
    GmpConversionSpec, GmpFormatArg, parse_gmp_conversion_spec,
};

// Appends the formatted value, the sign of which is `neg` and the absolute value of which is `num`,
// or `num / den` when `den` is present, to `out` according to `spec`. Returns `None` when the total
// width would exceed `i64::MAX`, or would not fit in a `usize`.
//
// This is `__gmp_doprnt_integer` from `printf/doprnti.c`, GMP 6.3.0, together with the
// `mpz_get_str` and `mpq_get_str` calls from the `Z` and `Q` argument cases of `__gmp_doprnt`. As
// there, the digit string of a rational is the numerator's and denominator's digits joined by a
// slash, the field width, precision, and numerator base prefix treat it as a unit, and a `#` prefix
// is inserted separately after the slash. (GMP documents the influence of the precision on a
// rational as undefined; this port reproduces what its code does, padding the numerator so that the
// whole string reaches the precision.)
fn append_integer(
    out: &mut Vec<u8>,
    neg: bool,
    num: &Natural,
    den: Option<&Natural>,
    spec: &GmpConversionSpec,
) -> Option<()> {
    if !matches!(spec.conv, b'd' | b'i' | b'u' | b'o' | b'x' | b'X') {
        return None;
    }
    // GMP semantics of the parsed fields: the later of the `+` and space flags wins, an omitted
    // precision or a `.` with no digits means all necessary digits, and the `0` flag's fill goes
    // between the prefix and the digits unless the field is left-justified.
    let prec = spec.prec.unwrap_or(-1);
    let internal = spec.fill == b'0' && !spec.left;
    let to_base = |x: &Natural| match spec.conv {
        b'o' => x.to_string_base(8),
        b'x' => x.to_string_base(16),
        b'X' => x.to_string_base_upper(16),
        _ => x.to_string_base(10),
    };
    let mut digits = to_base(num);
    if let Some(den) = den {
        digits.push('/');
        digits.push_str(&to_base(den));
    }
    let mut s = digits.as_bytes();
    // `+` or space if wanted, unless the value supplies a `-`
    let sign = if neg { b'-' } else { spec.sign };
    let sign_len = usize::from(sign != 0);
    // if the precision was explicitly 0, print nothing for a 0 value
    if prec == 0 && s.first() == Some(&b'0') {
        s = &s[1..];
    }
    let showbase: &[u8] = if spec.alt {
        match spec.conv {
            b'x' => b"0x",
            b'X' => b"0X",
            b'o' => b"0",
            _ => b"",
        }
    } else {
        b""
    };
    let slash = s.iter().position(|&c| c == b'/');
    // the `#` prefix goes on the denominator too, suppressed like the numerator's when its digits
    // already begin with a zero
    let den_showbase = match slash {
        None => b"" as &[u8],
        Some(i) => {
            if s.get(i + 1) == Some(&b'0') {
                // A canonical denominator is at least 2 and its digits carry no leading zeros, so
                // this suppression, which GMP needs for non-canonical `mpq_t`s, cannot fire here.
                fail_on_untested_path("append_integer, denominator digits beginning with 0");
                b""
            } else {
                showbase
            }
        }
    };
    // the numerator's `#` prefix is suppressed when the digits already begin with a zero
    let showbase = if s.first() == Some(&b'0') {
        b""
    } else {
        showbase
    };
    let zeros = max(0, prec - i64::try_from(s.len()).ok()?);
    // space left over after the actual output length, checked against i64::MAX via a wider
    // accumulator; no justifying if the content exceeds the width
    let core = i128::from(zeros)
        + i128::try_from(sign_len + showbase.len() + den_showbase.len() + s.len()).ok()?;
    let justlen = max(0, i128::from(spec.width) - core);
    if core + justlen > const { i64::MAX as i128 } {
        // The parser caps the width and precision at `i32::MAX`, so the total cannot come close to
        // overflowing an `i64`; this backstop protects a future caller that constructs a spec some
        // other way.
        fail_on_untested_path("append_integer, total width overflows i64");
        return None;
    }
    let zeros = usize::try_from(zeros).ok()?;
    let justlen = usize::try_from(justlen).ok()?;
    if !spec.left && !internal {
        out.resize(out.len() + justlen, spec.fill);
    }
    if sign != 0 {
        out.push(sign);
    }
    out.extend_from_slice(showbase);
    out.resize(out.len() + zeros, b'0');
    if internal {
        out.resize(out.len() + justlen, spec.fill);
    }
    // if there is a prefix on the denominator, print the numerator and slash separately so it can
    // be inserted
    if den_showbase.is_empty() {
        out.extend_from_slice(s);
    } else {
        let i = slash.unwrap() + 1;
        out.extend_from_slice(&s[..i]);
        out.extend_from_slice(den_showbase);
        out.extend_from_slice(&s[i..]);
    }
    if spec.left {
        out.resize(out.len() + justlen, spec.fill);
    }
    Some(())
}

// Formats a value with the sign `neg` and absolute value `num`, or `num / den` when `den` is
// present, for a single parsed conversion, which must be an integer conversion (`d`, `i`, `u`, `o`,
// `x`, or `X`); the caller is responsible for having checked the specification's type character.
// This function is public so that other Malachite crates can call it; it is not part of the public
// API.
//
// This is `__gmp_doprnt_integer` from `printf/doprnti.c`, GMP 6.3.0, together with the
// `mpz_get_str` and `mpq_get_str` calls from the `Z` and `Q` argument cases of `__gmp_doprnt`.
pub fn format_gmp_integer_spec(
    neg: bool,
    num: &Natural,
    den: Option<&Natural>,
    spec: &GmpConversionSpec,
) -> Option<String> {
    let mut out = Vec::new();
    append_integer(&mut out, neg, num, den, spec)?;
    // ASCII by construction
    String::from_utf8(out).ok()
}

// Formats a value with the sign `neg` and absolute value `num`, or `num / den` when `den` is
// present, according to a GMP-style format string containing a single `%Z` (or `%Q`, per
// `type_chr`) conversion; the engine behind [`format_natural_str`], `format_integer_str`, and
// malachite-q's `format_rational_str`. This function is public so that other Malachite crates can
// call it; it is not part of the public API.
//
// This is `__gmp_doprnt` from `printf/doprnt.c`, GMP 6.3.0, restricted to a format string whose
// only conversion is a `%Z` or `%Q` integer conversion.
pub fn format_gmp_str(
    neg: bool,
    num: &Natural,
    den: Option<&Natural>,
    type_chr: u8,
    fmt: &str,
) -> Option<String> {
    let bytes = fmt.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    let mut converted = false;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            if bytes.get(i + 1) == Some(&b'%') {
                out.push(b'%');
                i += 2;
                continue;
            }
            // this single-value entry point has no argument list for a `*` to draw from
            let (spec, rest) = parse_gmp_conversion_spec(&bytes[i + 1..], &mut || None)?;
            if converted || spec.type_chr != type_chr {
                return None;
            }
            converted = true;
            append_integer(&mut out, neg, num, den, &spec)?;
            i = bytes.len() - rest.len();
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    if !converted {
        return None;
    }
    // Literal text is copied byte-for-byte from the input `&str` and everything else appended is
    // ASCII, so the output is valid UTF-8.
    String::from_utf8(out).ok()
}

impl GmpFormatArg for Natural {
    /// Formats a [`Natural`] according to a single parsed conversion specification, which must be a
    /// `%Z` integer conversion; see [`gmp_format`](malachite_base::strings::gmp_format::gmp_format)
    /// and [`format_natural_str`].
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     gmp_format!("%Zd pieces", Natural::from(255u32)),
    ///     Some("255 pieces".to_string())
    /// );
    /// ```
    fn gmp_format(&self, spec: &GmpConversionSpec) -> Option<String> {
        if spec.type_chr != b'Z' {
            return None;
        }
        format_gmp_integer_spec(false, self, None, spec)
    }
}

/// Formats a [`Natural`] according to a GMP-style `printf` format string, for strict compatibility
/// with GMP's `gmp_printf` family.
///
/// The format string should contain a single conversion consuming the [`Natural`], written
/// `%[flags][width][.precision]Z[conv]`, with any surrounding literal text (a literal `%` is
/// written `%%`). The pieces are:
/// - **flags**: any of `-` (left-justify within the field), `+` (always show a sign), space (show a
///   space before the value), `#` (alternate form: prefix hexadecimal output with `0x` or `0X` and
///   octal output with `0`, unless the digits already begin with a zero), and `0` (pad the field
///   with leading zeros). The `'` flag is accepted but, as in GMP, has no effect on GMP types.
/// - **width**: the minimum field width, as a decimal integer.
/// - **precision**: following a `.`, the minimum number of digits, the value being padded with
///   leading zeros to reach it; a zero value formatted with a precision of 0 produces no digits at
///   all. By default all necessary digits are printed.
/// - **`Z`**: marks the argument as a multiple-precision integer (GMP's type character).
/// - **conv**: the conversion — `d`, `i`, or `u` (decimal; for a [`Natural`] all three are the
///   same), `o` (octal), or `x`/`X` (lowercase/uppercase hexadecimal).
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
/// use malachite_nz::natural::conversion::string::format_natural::format_natural_str;
/// use malachite_nz::natural::Natural;
///
/// let x = Natural::from(255u32);
/// assert_eq!(format_natural_str(&x, "%Zd"), Some("255".to_string()));
/// assert_eq!(format_natural_str(&x, "%#Zx"), Some("0xff".to_string()));
/// assert_eq!(format_natural_str(&x, "%#ZX"), Some("0XFF".to_string()));
/// assert_eq!(format_natural_str(&x, "%#Zo"), Some("0377".to_string()));
/// assert_eq!(format_natural_str(&x, "%8Zd"), Some("     255".to_string()));
/// assert_eq!(
///     format_natural_str(&x, "%-8Zd|"),
///     Some("255     |".to_string())
/// );
/// assert_eq!(
///     format_natural_str(&x, "%08Zd"),
///     Some("00000255".to_string())
/// );
/// assert_eq!(format_natural_str(&x, "%.6Zd"), Some("000255".to_string()));
/// assert_eq!(format_natural_str(&x, "%+Zd"), Some("+255".to_string()));
/// assert_eq!(
///     format_natural_str(&x, "x = %Zd!"),
///     Some("x = 255!".to_string())
/// );
/// assert_eq!(
///     format_natural_str(&x, "100%% of %Zd"),
///     Some("100% of 255".to_string())
/// );
///
/// // a zero value with an explicit precision of 0 produces no digits
/// assert_eq!(
///     format_natural_str(&Natural::from(0u32), "%.0Zd"),
///     Some("".to_string())
/// );
///
/// // invalid or unsupported format strings
/// assert_eq!(format_natural_str(&x, "%d"), None);
/// assert_eq!(format_natural_str(&x, "%*Zd"), None);
/// assert_eq!(format_natural_str(&x, "%Zd %Zd"), None);
/// ```
///
/// This is `gmp_snprintf` from `printf/snprintf.c`, GMP 6.3.0, where the format string contains a
/// single `%Z` integer conversion and the buffer is always large enough.
#[inline]
pub fn format_natural_str(x: &Natural, fmt: &str) -> Option<String> {
    format_gmp_str(false, x, None, b'Z', fmt)
}
