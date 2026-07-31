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

use crate::num::conversion::traits::{ToStringBase, WrappingFrom};
use alloc::string::String;
use alloc::vec::Vec;
use core::cmp::max;

/// A single parsed `printf`-style conversion specification, as GMP's and MPFR's formatted-output
/// functions understand them: `% [flags] [width] [.precision] [type] [rounding] conv`.
///
/// The struct is purely syntactic: it records what was written, and each [`GmpFormatArg`]
/// implementation applies its own library's interpretation. In particular:
/// - `sign` is the *last* `+` or space flag written (or 0 for neither), which is what GMP's own
///   types use, while `plus` and `space` record whether each flag appeared at all, which is what
///   the C conversions need (`+` overrides space in C, regardless of order).
/// - `prec` is [`None`] when no precision was written, and `Some(-1)` for a `.` with no digits,
///   which GMP reads as "all necessary digits" and C and MPFR read as 0.
/// - `type_chr` is the type or length-modifier character (`Z`, `Q`, `R`, `l`, `h`, and so on, or 0
///   for none), with `type_doubled` distinguishing `hh` and `ll`. As in GMP's parser, a later type
///   character overwrites an earlier one.
/// - `rnd_chr` is MPFR's rounding character, only ever set directly after an `R`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GmpConversionSpec {
    pub sign: u8,
    pub plus: bool,
    pub space: bool,
    pub alt: bool,
    pub left: bool,
    pub group: bool,
    pub fill: u8,
    pub width: i64,
    pub prec: Option<i64>,
    pub type_chr: u8,
    pub type_doubled: bool,
    pub rnd_chr: u8,
    pub conv: u8,
}

// Reads a run of decimal digits (the first of which is `c`) from the front of `fmt`, returning the
// value and the unconsumed tail. GMP stores widths and precisions in a C `int`, so values beyond
// `i32::MAX` cannot be expressed in a format string and are rejected.
fn read_digits(c: u8, mut fmt: &[u8]) -> Option<(i64, &[u8])> {
    let mut n = i64::from(c - b'0');
    while let Some((&d, tail)) = fmt.split_first()
        && d.is_ascii_digit()
    {
        n = n.checked_mul(10)?.checked_add(i64::from(d - b'0'))?;
        fmt = tail;
    }
    if n > i64::from(i32::MAX) {
        return None;
    }
    Some((n, fmt))
}

/// Parses a single conversion specification, starting just after the `%`, and returns it along with
/// the unconsumed tail. `star` supplies the value of a `*` field width or precision, from the
/// argument list; it may fail, and a caller with no argument list simply passes `&mut || None`.
///
/// The parser mirrors GMP's single-pass structure, including its quirks: flags may appear after the
/// width, a later `+` or space flag overwrites the `sign` of an earlier one, a later type character
/// overwrites an earlier one, and a `-` flag does not reset the `0` flag's fill character. A
/// negative `*` width means left justification, and a negative `*` precision is treated as 0.
/// Returns [`None`] if a width or precision overflows the range of a C `int` (beyond which GMP
/// cannot express them), if `star` fails, or on a character with no role in a conversion
/// specification.
///
/// # Worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `fmt.len()`.
///
/// # Examples
/// ```
/// use malachite_base::strings::gmp_format::parse_gmp_conversion_spec;
///
/// let (spec, rest) = parse_gmp_conversion_spec(b"+8.3Zd tail", &mut || None).unwrap();
/// assert_eq!(spec.sign, b'+');
/// assert_eq!(spec.width, 8);
/// assert_eq!(spec.prec, Some(3));
/// assert_eq!(spec.type_chr, b'Z');
/// assert_eq!(spec.conv, b'd');
/// assert_eq!(rest, b" tail");
/// ```
///
/// This is the format-parsing loop of `__gmp_doprnt` from `printf/doprnt.c`, GMP 6.3.0, with MPFR's
/// `R` type and rounding characters from `vasprintf.c`, MPFR 4.2.2.
pub fn parse_gmp_conversion_spec<'a>(
    mut fmt: &'a [u8],
    star: &mut dyn FnMut() -> Option<i64>,
) -> Option<(GmpConversionSpec, &'a [u8])> {
    let mut spec = GmpConversionSpec {
        sign: 0,
        plus: false,
        space: false,
        alt: false,
        left: false,
        group: false,
        fill: b' ',
        width: 0,
        prec: None,
        type_chr: 0,
        type_doubled: false,
        rnd_chr: 0,
        conv: 0,
    };
    let mut in_width = true;
    loop {
        let (&c, tail) = fmt.split_first()?;
        fmt = tail;
        match c {
            b'#' => spec.alt = true,
            b'\'' => spec.group = true,
            b'+' => {
                spec.plus = true;
                spec.sign = c;
            }
            b' ' => {
                spec.space = true;
                spec.sign = c;
            }
            b'-' => spec.left = true,
            b'0' => {
                if in_width {
                    // in the width field, `0` is a flag setting the fill
                    spec.fill = b'0';
                } else {
                    spec.prec = Some(0);
                }
            }
            b'1'..=b'9' => {
                let (n, tail) = read_digits(c, fmt)?;
                fmt = tail;
                if in_width {
                    spec.width = n;
                } else {
                    spec.prec = Some(n);
                }
            }
            b'.' => {
                // `.` alone is `Some(-1)`; any following digits overwrite it
                spec.prec = Some(-1);
                in_width = false;
            }
            b'*' => {
                let n = star()?;
                if n.unsigned_abs() > u64::from(u32::wrapping_from(i32::MAX)) {
                    return None;
                }
                if in_width {
                    // a negative width means left justification
                    if n < 0 {
                        spec.left = true;
                        spec.width = -n;
                    } else {
                        spec.width = n;
                    }
                } else {
                    // a negative precision is not allowed
                    spec.prec = Some(max(0, n));
                }
            }
            b'h' | b'l' => {
                spec.type_chr = c;
                spec.type_doubled = false;
                if let Some((&d, tail)) = fmt.split_first()
                    && d == c
                {
                    spec.type_doubled = true;
                    fmt = tail;
                }
            }
            b'j' | b'q' | b't' | b'z' | b'L' | b'Q' | b'M' | b'N' | b'Z' | b'P' => {
                spec.type_chr = c;
                spec.type_doubled = false;
            }
            b'R' => {
                spec.type_chr = c;
                spec.type_doubled = false;
                // MPFR's rounding character comes directly after the `R`; `*`, which in C fetches
                // the mode from the argument list, is not supported
                if let Some((&d, tail)) = fmt.split_first() {
                    match d {
                        b'N' | b'D' | b'U' | b'Y' | b'Z' => {
                            spec.rnd_chr = d;
                            fmt = tail;
                        }
                        b'*' => return None,
                        _ => {}
                    }
                }
            }
            b'F' => {
                if spec.type_chr == b'R' {
                    // after an `R`, `F` is MPFR's uppercase fixed-point conversion
                    spec.conv = c;
                    return Some((spec, fmt));
                }
                // elsewhere it is GMP's `mpf_t` type character
                spec.type_chr = c;
                spec.type_doubled = false;
            }
            b'd' | b'i' | b'u' | b'o' | b'x' | b'X' | b'e' | b'E' | b'f' | b'g' | b'G' | b'a'
            | b'A' | b'b' | b'c' | b's' | b'p' | b'n' | b'm' => {
                spec.conv = c;
                return Some((spec, fmt));
            }
            _ => return None,
        }
    }
}

/// A value that can be consumed by a conversion of a GMP-style format string; see [`gmp_format`].
///
/// Each implementation accepts the conversions its library counterpart would: `Natural` and
/// `Integer` take `%Z` integer conversions, `Rational` takes `%Q`, `Float` takes `%R`, primitive
/// integers take the plain C integer conversions (and `%c`), [`char`] takes `%c`, and strings take
/// `%s`. [`gmp_format`](GmpFormatArg::gmp_format) returns the formatted piece, or [`None`] when the
/// specification does not apply to the value's type.
pub trait GmpFormatArg {
    /// Formats this value according to a single parsed conversion specification, or returns
    /// [`None`] when the specification does not apply to this type.
    fn gmp_format(&self, spec: &GmpConversionSpec) -> Option<String>;

    /// The integer consumed by a `*` field width or precision, when this value is a primitive
    /// integer that fits in an `i64`.
    fn printf_int(&self) -> Option<i64> {
        None
    }
}

// Appends `n` copies of `fill` to `out`.
fn pad(out: &mut Vec<u8>, fill: u8, n: usize) {
    out.resize(out.len() + n, fill);
}

// Applies the field width of `spec` to the already-rendered `body`, left- or right-justifying it.
// Used by the conversions whose zero-fill handling is trivial (`%c`, `%s`): the fill is always a
// space, as in C.
fn justify(body: &[u8], spec: &GmpConversionSpec) -> Option<String> {
    let width = usize::try_from(spec.width).unwrap_or(0);
    let padding = width.saturating_sub(body.len());
    let mut out = Vec::with_capacity(body.len() + padding);
    if !spec.left {
        pad(&mut out, b' ', padding);
    }
    out.extend_from_slice(body);
    if spec.left {
        pad(&mut out, b' ', padding);
    }
    // `body` comes from a `str` or is ASCII
    String::from_utf8(out).ok()
}

// Whether `spec` is an integer conversion with no type character or with a C length modifier, which
// is accepted but has no effect: the value is formatted as passed, and is never truncated the way
// C's `%hd` would truncate an `int` argument.
fn is_c_integer_spec(spec: &GmpConversionSpec) -> bool {
    matches!(spec.conv, b'd' | b'i' | b'u' | b'o' | b'x' | b'X')
        && matches!(
            spec.type_chr,
            0 | b'h' | b'l' | b'j' | b'q' | b't' | b'z' | b'L'
        )
}

// Formats a primitive integer with sign `neg` and absolute-value digits produced by `to_base`,
// following the C `printf` rules: `+` overrides the space flag, the `0` flag is ignored with left
// justification or an explicit precision, a `#` prefix is applied only when the digits do not
// already begin with a zero, and a zero value with a precision of 0 produces no digits. In the C
// locale the `'` flag groups nothing, so it is accepted and ignored.
//
// This is the behavior `gmp_printf` gets by handing the standard conversions to the C library.
fn format_c_integer(
    neg: bool,
    to_base: &dyn Fn(u8, bool) -> String,
    spec: &GmpConversionSpec,
) -> Option<String> {
    if !is_c_integer_spec(spec) {
        return None;
    }
    let digits = match spec.conv {
        b'o' => to_base(8, false),
        b'x' => to_base(16, false),
        b'X' => to_base(16, true),
        _ => to_base(10, false),
    };
    let mut s = digits.as_bytes();
    let sign = if neg {
        b'-'
    } else if spec.plus {
        b'+'
    } else if spec.space {
        b' '
    } else {
        0
    };
    let sign_len = usize::from(sign != 0);
    // C reads a `.` with no digits (`Some(-1)`) as a precision of 0
    let prec = spec.prec.map_or(-1, |p| max(0, p));
    if prec == 0 && s == b"0" {
        s = b"";
    }
    let mut showbase: &[u8] = if spec.alt {
        match spec.conv {
            b'x' => b"0x",
            b'X' => b"0X",
            b'o' => b"0",
            _ => b"",
        }
    } else {
        b""
    };
    if s.first() == Some(&b'0') {
        showbase = b"";
    }
    let zeros = usize::try_from(max(0, prec - i64::try_from(s.len()).ok()?)).ok()?;
    let core = sign_len + showbase.len() + zeros + s.len();
    let width = usize::try_from(spec.width).unwrap_or(0);
    let padding = width.saturating_sub(core);
    // the 0 flag is ignored with left justification or an explicit precision
    let zero_fill = spec.fill == b'0' && !spec.left && spec.prec.is_none();
    let mut out = Vec::with_capacity(core + padding);
    if !spec.left && !zero_fill {
        pad(&mut out, b' ', padding);
    }
    if sign != 0 {
        out.push(sign);
    }
    out.extend_from_slice(showbase);
    if zero_fill {
        pad(&mut out, b'0', padding);
    }
    pad(&mut out, b'0', zeros);
    out.extend_from_slice(s);
    if spec.left {
        pad(&mut out, b' ', padding);
    }
    // ASCII by construction
    String::from_utf8(out).ok()
}

// Formats a primitive integer for a `%c` conversion, as C does: the value is converted to an
// `unsigned char`, keeping its lowest byte.
fn format_c_char_of_int(value: u64, spec: &GmpConversionSpec) -> Option<String> {
    if spec.conv != b'c' || !matches!(spec.type_chr, 0 | b'h' | b'l') {
        return None;
    }
    justify(&[u8::wrapping_from(value)], spec)
}

macro_rules! impl_gmp_format_arg_unsigned {
    ($t:ident) => {
        impl GmpFormatArg for $t {
            /// Formats an unsigned primitive integer according to a single parsed conversion
            /// specification: a plain C integer conversion (`d`, `i`, `u`, `o`, `x`, or `X`, with
            /// any C length modifier accepted but not truncating the value), or `c` (keeping the
            /// value's lowest byte, as C does).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::gmp_format#gmp_format).
            fn gmp_format(&self, spec: &GmpConversionSpec) -> Option<String> {
                if spec.conv == b'c' {
                    return format_c_char_of_int(u64::wrapping_from(*self), spec);
                }
                format_c_integer(
                    false,
                    &|base, upper| {
                        if upper {
                            self.to_string_base_upper(base)
                        } else {
                            self.to_string_base(base)
                        }
                    },
                    spec,
                )
            }

            fn printf_int(&self) -> Option<i64> {
                i64::try_from(*self).ok()
            }
        }
    };
}
apply_to_unsigneds!(impl_gmp_format_arg_unsigned);

macro_rules! impl_gmp_format_arg_signed {
    ($t:ident) => {
        impl GmpFormatArg for $t {
            /// Formats a signed primitive integer according to a single parsed conversion
            /// specification: a plain C integer conversion (`d`, `i`, `u`, `o`, `x`, or `X`, with
            /// any C length modifier accepted but not truncating the value, and a negative value
            /// keeping its sign under every conversion), or `c` (keeping the value's lowest byte,
            /// as C does).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::gmp_format#gmp_format).
            fn gmp_format(&self, spec: &GmpConversionSpec) -> Option<String> {
                if spec.conv == b'c' {
                    return format_c_char_of_int(u64::wrapping_from(self.unsigned_abs()), spec);
                }
                let abs = self.unsigned_abs();
                format_c_integer(
                    *self < 0,
                    &|base, upper| {
                        if upper {
                            abs.to_string_base_upper(base)
                        } else {
                            abs.to_string_base(base)
                        }
                    },
                    spec,
                )
            }

            fn printf_int(&self) -> Option<i64> {
                i64::try_from(*self).ok()
            }
        }
    };
}
apply_to_signeds!(impl_gmp_format_arg_signed);

impl GmpFormatArg for char {
    /// Formats a [`char`] according to a single parsed conversion specification, which must be a
    /// `%c` conversion with no type character.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::gmp_format#gmp_format).
    fn gmp_format(&self, spec: &GmpConversionSpec) -> Option<String> {
        if spec.conv != b'c' || spec.type_chr != 0 {
            return None;
        }
        let mut buf = [0; 4];
        justify(self.encode_utf8(&mut buf).as_bytes(), spec)
    }
}

impl GmpFormatArg for &str {
    /// Formats a string according to a single parsed conversion specification, which must be a `%s`
    /// conversion with no type character. As in C, the precision is the maximum number of bytes
    /// written; if that limit would split a multi-byte character, [`None`] is returned, since the
    /// output could not be a valid string.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.len(), w)`, with `w` the
    /// field width requested by the format string.
    ///
    /// # Examples
    /// See [here](super::gmp_format#gmp_format).
    fn gmp_format(&self, spec: &GmpConversionSpec) -> Option<String> {
        if spec.conv != b's' || spec.type_chr != 0 {
            return None;
        }
        let mut s = *self;
        if let Some(prec) = spec.prec {
            let prec = usize::try_from(max(0, prec)).ok()?;
            if prec < s.len() {
                if !s.is_char_boundary(prec) {
                    return None;
                }
                s = &s[..prec];
            }
        }
        justify(s.as_bytes(), spec)
    }
}

impl GmpFormatArg for String {
    /// Formats a string according to a single parsed conversion specification; see the
    /// [`&str`](GmpFormatArg#impl-GmpFormatArg-for-%26str) implementation.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.len(), w)`, with `w` the
    /// field width requested by the format string.
    ///
    /// # Examples
    /// See [here](super::gmp_format#gmp_format).
    #[inline]
    fn gmp_format(&self, spec: &GmpConversionSpec) -> Option<String> {
        (&**self).gmp_format(spec)
    }
}

/// Formats a sequence of values according to a GMP-style `printf` format string, each conversion
/// consuming the next value, as `gmp_printf` and `mpfr_printf` do.
///
/// The format string may contain literal text, `%%` escapes, and any number of conversions, each
/// written `%[flags][width][.precision][type][rounding]conv`. Which conversions a value accepts is
/// up to its [`GmpFormatArg`] implementation: `%Z` integer conversions for `Natural` and `Integer`,
/// `%Q` for `Rational`, `%R` float conversions for `Float`, and the plain C conversions for
/// primitive integers (`d`, `i`, `u`, `o`, `x`, `X`, `c`), [`char`]s (`c`), and strings (`s`). A
/// `*` field width or precision consumes the next value, which must be a primitive integer; a
/// negative `*` width means left justification.
///
/// Returns [`None`] when a conversion specification is malformed or requests a width or precision
/// beyond `i32::MAX` (the range of the C `int` GMP itself stores them in), when a conversion does
/// not apply to the value it would consume, when there are too few values (extra values are
/// permitted, as in C), or when the conversion is one this function does not support: `%n`, `%p`,
/// `%m`, `%F` (GMP's `mpf_t`), `%M` and `%N` (limbs), `%P` (MPFR precisions), and the C float
/// conversions on primitive floats (use a `Float`).
///
/// The [`gmp_format!`](crate::gmp_format) macro wraps this function, building the argument slice.
///
/// # Worst-case complexity
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is the sum over the conversions of
/// `max(x.significant_bits(), p, w)`, with `p` and `w` each conversion's precision and field width,
/// plus `fmt.len()`.
///
/// # Examples
/// ```
/// use malachite_base::strings::gmp_format::gmp_format;
///
/// assert_eq!(
///     gmp_format("%d + %d = %d", &[&2u32, &2u32, &4u32]),
///     Some("2 + 2 = 4".to_string())
/// );
/// assert_eq!(
///     gmp_format("%c%s%c", &[&'(', &"hello", &')']),
///     Some("(hello)".to_string())
/// );
/// assert_eq!(
///     gmp_format("%0*x", &[&8i32, &255u32]),
///     Some("000000ff".to_string())
/// );
/// // 100% literal
/// assert_eq!(gmp_format("100%%", &[]), Some("100%".to_string()));
///
/// // a conversion that does not apply to its value
/// assert_eq!(gmp_format("%s", &[&5u32]), None);
/// // too few values
/// assert_eq!(gmp_format("%d %d", &[&5u32]), None);
/// ```
///
/// This is `gmp_snprintf` from `printf/snprintf.c`, GMP 6.3.0, and `mpfr_snprintf` from
/// `vasprintf.c`, MPFR 4.2.2, where the buffer is always large enough.
pub fn gmp_format(fmt: &str, args: &[&dyn GmpFormatArg]) -> Option<String> {
    let bytes = fmt.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    let mut next = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            if bytes.get(i + 1) == Some(&b'%') {
                out.push(b'%');
                i += 2;
                continue;
            }
            let (spec, rest) = {
                let mut star = || {
                    let arg = args.get(next)?;
                    next += 1;
                    arg.printf_int()
                };
                parse_gmp_conversion_spec(&bytes[i + 1..], &mut star)?
            };
            let arg = args.get(next)?;
            next += 1;
            out.extend_from_slice(arg.gmp_format(&spec)?.as_bytes());
            i = bytes.len() - rest.len();
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    // Literal text is copied byte-for-byte from the input `&str` and every conversion's output is a
    // `String`, so the output is valid UTF-8.
    String::from_utf8(out).ok()
}

/// Formats values according to a GMP-style `printf` format string, as
/// [`gmp_format`](strings::gmp_format::gmp_format) does, taking the values as ordinary arguments:
/// `gmp_format!("%Zd of %d", n, k)`.
///
/// The result is an `Option<String>`; see [`gmp_format`](strings::gmp_format::gmp_format) for the
/// supported conversions and failure conditions.
///
/// # Examples
/// ```
/// use malachite_base::gmp_format;
///
/// assert_eq!(
///     gmp_format!("%d + %d = %d", 2u32, 2u32, 4u32),
///     Some("2 + 2 = 4".to_string())
/// );
/// ```
#[macro_export]
macro_rules! gmp_format {
    ($fmt:expr $(, $args:expr)* $(,)?) => {
        $crate::strings::gmp_format::gmp_format(
            $fmt,
            &[$(&$args as &dyn $crate::strings::gmp_format::GmpFormatArg),*],
        )
    };
}
