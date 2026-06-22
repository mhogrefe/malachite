// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 1999-2024 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Faithful port of the floating-point (`%R...`) formatting path of MPFR's `mpfr_vasprintf`
// (`vasprintf.c`, MPFR 4.2.2): conversion-specification parsing plus the `sprnt_fp` /
// `partition_number` machinery, built on top of `Float::get_str`. The C `char *format` cursor is
// rendered as a `&[u8]` slice that the parser functions advance by returning the unconsumed tail.
//
// WORK IN PROGRESS: translated incrementally; the `dead_code` allow is removed once the public
// frontend is wired in.
#![allow(dead_code)]

use crate::Float;
use crate::conversion::string::get_str::get_str;
use malachite_base::rounding_modes::RoundingMode::{self, Nearest};

// All the types described by the `type` field of the format string.
//
// This is `enum arg_t` from `vasprintf.c`, MPFR 4.2.2.
#[derive(Clone, Copy, Eq, PartialEq)]
enum ArgType {
    None,
    Char,
    Short,
    Long,
    LongLong,
    IntMax,
    Size,
    PtrDiff,
    LongDouble,
    Mpf,
    Mpq,
    MpLimb,
    MpLimbArray,
    Mpz,
    MpfrPrec,
    Mpfr,
    Unsupported,
}

// A single conversion specification of the format string, filled in by the parser. (Adapted, like
// the MPFR original, from the GNU libc structure.) `width` and `prec` use `i64` for MPFR's
// `mpfr_intmax_t`; `prec` is negative when omitted. `spec` and `pad` are single bytes, holding the
// conversion specifier and the padding character.
//
// This is `struct printf_spec` from `vasprintf.c`, MPFR 4.2.2.
#[derive(Clone, Copy)]
struct PrintfSpec {
    alt: bool,      // `#` flag
    space: bool,    // space flag
    left: bool,     // `-` flag
    showsign: bool, // `+` flag
    group: bool,    // `'` flag
    width: i64,
    prec: i64,
    size: usize, // wanted size (0 iff snprintf with size = 0)
    arg_type: ArgType,
    rnd_mode: RoundingMode,
    spec: u8,
    pad: u8,
}

// This is `specinfo_init` from `vasprintf.c`, MPFR 4.2.2.
fn specinfo_init() -> PrintfSpec {
    PrintfSpec {
        alt: false,
        space: false,
        left: false,
        showsign: false,
        group: false,
        width: 0,
        prec: 0,
        size: 1,
        arg_type: ArgType::None,
        rnd_mode: Nearest,
        spec: b'\0',
        pad: b' ',
    }
}

// Note: LONG_ARG is unusual, but is accepted (ISO C99 says "has no effect on a following a, A, e,
// E, f, F, g, or G conversion specifier").
//
// This is `FLOATING_POINT_ARG_TYPE` from `vasprintf.c`, MPFR 4.2.2.
fn floating_point_arg_type(at: ArgType) -> bool {
    matches!(
        at,
        ArgType::Mpfr | ArgType::Mpf | ArgType::Long | ArgType::LongDouble
    )
}

// This is `INTEGER_LIKE_ARG_TYPE` from `vasprintf.c`, MPFR 4.2.2.
fn integer_like_arg_type(at: ArgType) -> bool {
    matches!(
        at,
        ArgType::Short
            | ArgType::Long
            | ArgType::LongLong
            | ArgType::IntMax
            | ArgType::MpfrPrec
            | ArgType::Mpz
            | ArgType::Mpq
            | ArgType::MpLimb
            | ArgType::MpLimbArray
            | ArgType::Char
            | ArgType::Size
            | ArgType::PtrDiff
    )
}

// Returns 1 if `spec` is a valid (supported) conversion, 0 if invalid, and -1 for `n` (which MPFR
// rejects).
//
// This is `specinfo_is_valid` from `vasprintf.c`, MPFR 4.2.2.
fn specinfo_is_valid(spec: PrintfSpec) -> i32 {
    match spec.spec {
        b'n' => -1,
        // 'F': see below
        b'a' | b'A' | b'e' | b'E' | b'f' | b'g' | b'G' => {
            i32::from(spec.arg_type == ArgType::None || floating_point_arg_type(spec.arg_type))
        }
        // 'F' only supports MPFR_ARG, since GMP doesn't support it (it is the mpf_t specifier); 'b'
        // is MPFR-specific.
        b'F' | b'b' => i32::from(spec.arg_type == ArgType::Mpfr),
        b'd' | b'i' | b'o' | b'u' | b'x' | b'X' => {
            i32::from(spec.arg_type == ArgType::None || integer_like_arg_type(spec.arg_type))
        }
        b'c' | b's' => i32::from(spec.arg_type == ArgType::None || spec.arg_type == ArgType::Long),
        b'p' => i32::from(spec.arg_type == ArgType::None),
        _ => 0,
    }
}

// Consumes the leading flag characters of `format`, recording them in `specinfo`, and returns the
// unconsumed tail.
//
// This is `parse_flags` from `vasprintf.c`, MPFR 4.2.2.
fn parse_flags<'a>(mut format: &'a [u8], specinfo: &mut PrintfSpec) -> &'a [u8] {
    while let Some(&c) = format.first() {
        match c {
            b'0' => specinfo.pad = b'0',
            b'#' => specinfo.alt = true,
            b'+' => specinfo.showsign = true,
            b' ' => specinfo.space = true,
            b'-' => specinfo.left = true,
            // Single UNIX Specification for thousand separator
            b'\'' => specinfo.group = true,
            _ => return format,
        }
        format = &format[1..];
    }
    format
}

// Consumes the length-modifier / type prefix of `format`, recording the argument type in
// `specinfo`, and returns the unconsumed tail. `HAVE_LONG_LONG` and the `intmax_t` support are
// assumed present (always true on the platforms Malachite targets).
//
// This is `parse_arg_type` from `vasprintf.c`, MPFR 4.2.2.
fn parse_arg_type<'a>(mut format: &'a [u8], specinfo: &mut PrintfSpec) -> &'a [u8] {
    match format.first() {
        None => {}
        Some(b'h') => {
            format = &format[1..];
            if format.first() == Some(&b'h') {
                format = &format[1..];
                specinfo.arg_type = ArgType::Char;
            } else {
                specinfo.arg_type = ArgType::Short;
            }
        }
        Some(b'l') => {
            format = &format[1..];
            if format.first() == Some(&b'l') {
                format = &format[1..];
                specinfo.arg_type = ArgType::LongLong;
            } else {
                specinfo.arg_type = ArgType::Long;
            }
        }
        Some(b'j') => {
            format = &format[1..];
            specinfo.arg_type = ArgType::IntMax;
        }
        Some(b'z') => {
            format = &format[1..];
            specinfo.arg_type = ArgType::Size;
        }
        Some(b't') => {
            format = &format[1..];
            specinfo.arg_type = ArgType::PtrDiff;
        }
        Some(b'L') => {
            format = &format[1..];
            specinfo.arg_type = ArgType::LongDouble;
        }
        Some(b'F') => {
            format = &format[1..];
            specinfo.arg_type = ArgType::Mpf;
        }
        Some(b'Q') => {
            format = &format[1..];
            specinfo.arg_type = ArgType::Mpq;
        }
        // The 'M' specifier was added in GMP 4.2.0.
        Some(b'M') => {
            format = &format[1..];
            specinfo.arg_type = ArgType::MpLimb;
        }
        Some(b'N') => {
            format = &format[1..];
            specinfo.arg_type = ArgType::MpLimbArray;
        }
        Some(b'Z') => {
            format = &format[1..];
            specinfo.arg_type = ArgType::Mpz;
        }
        // mpfr-specific specifiers
        Some(b'P') => {
            format = &format[1..];
            specinfo.arg_type = ArgType::MpfrPrec;
        }
        Some(b'R') => {
            format = &format[1..];
            specinfo.arg_type = ArgType::Mpfr;
        }
        Some(_) => {}
    }
    format
}

// The growable output buffer. MPFR's `struct string_buffer` tracks a manually-`realloc`-ed C buffer
// (`start`/`curr`/`size`) plus a `len` that becomes -1 on overflow (for the snprintf return value);
// a `Vec<u8>` subsumes all of that (growth is automatic, and a length exceeding `usize::MAX` is not
// a reachable state), so neither `buffer_widen` nor the overflow bookkeeping (`buffer_incr_len`) is
// ported. The size-0 (count-only `snprintf`) mode is likewise dropped: this engine always produces
// the full output.
//
// This is `struct string_buffer` from `vasprintf.c`, MPFR 4.2.2.
struct StringBuffer {
    chars: Vec<u8>,
}

// This is `buffer_init` from `vasprintf.c`, MPFR 4.2.2.
fn buffer_init(s: usize) -> StringBuffer {
    StringBuffer {
        chars: Vec::with_capacity(s),
    }
}

// Concatenates `s` to the buffer `b`. (The caller passes the already-truncated slice, so MPFR's
// separate `len` argument is the slice length.)
//
// This is `buffer_cat` from `vasprintf.c`, MPFR 4.2.2.
fn buffer_cat(b: &mut StringBuffer, s: &[u8]) {
    b.chars.extend_from_slice(s);
}

// Adds `n` copies of the character `c` to the end of the buffer `b`.
//
// This is `buffer_pad` from `vasprintf.c`, MPFR 4.2.2.
fn buffer_pad(b: &mut StringBuffer, c: u8, n: i64) {
    assert!(n > 0);
    let new_len = b.chars.len() + usize::try_from(n).unwrap();
    b.chars.resize(new_len, c);
}

// Forms a string by concatenating the first `len` characters of `str` to `tz` zero(s), inserting
// the character `c` every 3 characters from end to beginning, and concatenates the result to the
// buffer `b`. `c` must not be null and `tz` must be 0 or 1.
//
// This is `buffer_sandwich` from `vasprintf.c`, MPFR 4.2.2.
fn buffer_sandwich(b: &mut StringBuffer, str: &[u8], mut len: usize, tz: usize, c: u8) {
    const STEP: usize = 3;
    assert!(tz == 0 || tz == 1);
    assert!(c != b'\0');
    assert!(len <= str.len());
    let size = len + tz; // number of digits
    assert!(size > 0);
    let q = (size - 1) / STEP; // number of separators c
    let r = ((size - 1) % STEP) + 1; // number of digits in the leftmost block
    let mut str = str;
    // first r significant digits (leftmost block)
    if r <= len {
        buffer_cat(b, &str[..r]);
        str = &str[r..];
        len -= r;
    } else {
        // r > len, and as a consequence: len < STEP, size <= STEP, q == 0, r == size, tz == 1
        buffer_cat(b, &str[..len]);
        b.chars.push(b'0'); // trailing zero
    }
    for _ in 0..q {
        b.chars.push(c);
        if len >= STEP {
            buffer_cat(b, &str[..STEP]);
            len -= STEP;
            str = &str[STEP..];
        } else {
            // last digits (i == q - 1 and STEP - len == 1)
            buffer_cat(b, &str[..len]);
            b.chars.push(b'0'); // trailing zero
        }
    }
}

// MPFR's `string_list` / `init_string_list` / `clear_string_list` / `register_string` (a manual
// list for freeing the temporary digit strings produced while formatting) are not ported: in safe
// Rust those temporaries are owned `Vec`s/`String`s freed by their scope, so no registry is needed.

// Where the padding characters go.
//
// This is `enum pad_t` from `vasprintf.c`, MPFR 4.2.2.
enum PadType {
    Left,         // spaces on the left, for right justification
    LeadingZeros, // '0' padding in the integral part
    Right,        // spaces on the right, for left justification
}

// Details how many characters are needed in each part of a float printout. MPFR's `ip_ptr` and
// `fp_ptr` point into the single `mpfr_get_str` digit string (sometimes both into the same string,
// sometimes a static "0"/"1"), with a `string_list` owning the allocation; that aliasing can't be
// expressed with safe references, so each digit-bearing part is an owned `Vec<u8>` here (the
// shared-allocation optimization is dropped) and MPFR's `*_size` fields collapse into the vectors'
// lengths. The fixed base prefix ("0x", "0X", "0b", "0B") is a `&'static [u8]`.
//
// This is `struct number_parts` from `vasprintf.c`, MPFR 4.2.2.
struct NumberParts {
    pad_type: PadType,
    pad_size: i64,
    sign: u8,                // sign character: '-', '+', ' ', or '\0'
    prefix: &'static [u8],   // prefix part (was prefix_ptr / prefix_size)
    thousands_sep: u8,       // thousands separator (only with style 'f'); '\0' if none
    ip: Vec<u8>,             // integral-part digits (was ip_ptr / ip_size)
    ip_trailing_digits: i32, // additional integral digits (a zero if spec.size != 0)
    point: u8,               // decimal point character, or '\0'
    fp_leading_zeros: i64,   // additional leading zeros in the fractional part
    fp: Vec<u8>,             // fractional-part digits (was fp_ptr / fp_size)
    fp_trailing_zeros: i64,  // additional trailing zeros in the fractional part
    exp: Vec<u8>,            // exponent part (was exp_ptr / exp_size)
}

// Records the result of a `get_str` call so that this expensive function is not called more than
// once for the same number.
//
// This is `struct decimal_info` from `vasprintf.c`, MPFR 4.2.2.
struct DecimalInfo {
    exp: i64,
    str: Vec<u8>,
}

// Returns the base-`base` digits of `op` with `n` significant digits, rounded according to
// `spec.rnd_mode`, together with the base-`base` exponent. (MPFR's size-0 `snprintf` fast path,
// which estimates the printed length from a few significant digits, is not ported: this engine
// always produces the full output. Since `get_str` panics under `Exact` rounding as soon as the
// result is known to be inexact, that behavior carries over for free.)
//
// This is `mpfr_get_str_wrapper` from `vasprintf.c`, MPFR 4.2.2.
fn mpfr_get_str_wrapper(base: i64, n: usize, op: &Float, spec: &PrintfSpec) -> (Vec<u8>, i64) {
    // base is 2, 10, or 16 -- all valid -- so get_str never returns None.
    let (s, exp, _) = get_str(op, base, n, spec.rnd_mode).unwrap();
    (s, exp)
}
