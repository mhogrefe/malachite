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
// `format_float_str` (below) is the public MPFR-compatible entry point, for callers who want strict
// `mpfr_printf`-style formatting of a single `Float`. Everything else is internal: `format` is the
// multi-conversion engine it delegates to; `format_float` / `float_conversion_spec` / `PrintfSpec`
// are the spec-based core, exposed only under `test_build`. The `dead_code` allow covers those
// test-only entry points, which are unused in a normal build.
//
// Porting status (2026-07-17):
// - DONE — the ENTIRE `%R` float path works, and `format_float_str` is public. It is validated
//   against MPFR (via rug's `get_str` oracle) in tests/conversion/string/format_float.rs, for all
//   of 'e'/'f'/'g' [base 10] and 'a'/'A'/'b' [bases 16/2]; printf has no other float bases. Chain:
//   spec/flag/arg-type parsing, `StringBuffer` + buffer ops, `NumberParts`/`DecimalInfo`,
//   `mpfr_get_str_wrapper`, `floor_log10` (on `Float::unsigned_pow`), `number_parts_init`,
//   `regular_eg` (scientific), `regular_fg` (fixed), `next_base_power_p` + `regular_ab`
//   (hex/binary), `partition_number` (dispatcher), `sprnt_fp` (emitter), `format_float` (a
//   per-conversion entry point), `format` (a format-string frontend over a `&[PrintfArg]` slice),
//   and `format_float_str` (the public single-value entry point).
// - REMAINING: the multi-argument `format` frontend is not yet public (its `PrintfArg` model would
//   need finalizing); `Display` will be a separate `get_str`-based implementation, not built on
//   this. Note the `'` flag divergence below for any future full-string FFI oracle.
// - Deliberate divergences from MPFR:
//   - Malachite zeros are precision-less (unlike MPFR), so `%e`-of-zero with an empty precision
//     falls back to precision 1.
//   - The `'` flag always groups with a comma; MPFR uses the locale's separator, which is EMPTY in
//     the default C locale (where MPFR therefore prints no separators).
//   - A width or precision literal that overflows an `i64` makes `format` return `None` (MPFR sets
//     EOVERFLOW and returns -1), as does any conversion the `PrintfArg` model cannot supply and any
//     internal size overflow (MPFR's -1 returns).
//   - `Exact` rounding is supported (not an MPFR mode): it panics whenever the output does not
//     represent the value exactly, consistent with `get_str`.
//   - MPFR 4.2.2's single-digit rounding bug is FIXED here (with an exactness check): MPFR rounds
//     exact values away under away-rounding modes ("%.0RUa" of 1.5 gives 0xdp-3 = 1.625) and
//     overflows its digit table when the top digit is 0xf ("%.0RUa" of 15 prints garbage), and it
//     misses inexactness below the top significand limb ("%.0RUb" of 2^100 + 1 is not rounded up).
#![allow(dead_code)]

use crate::Float;
use crate::conversion::string::get_str::{ceil_mul, get_str, get_str_ndigits};
use core::cmp::Ordering::{Equal, Greater, Less};
use malachite_base::fail_on_untested_path;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, OneHalf};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::string::to_string::digit_to_display_byte_lower;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, LowMask, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{
    self, Ceiling, Down, Exact, Floor, Nearest, Up,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

// All the types described by the `type` field of the format string.
//
// This is `enum arg_t` from `vasprintf.c`, MPFR 4.2.2.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum ArgType {
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
// This is `struct printf_spec` from `vasprintf.c`, MPFR 4.2.2. (Its `size` field, 0 iff snprintf
// with size = 0, is not ported: the count-only mode it selected was dropped.) The struct is `pub`
// under `test_build` so the tests can name it (they build values via `float_conversion_spec` and
// never touch the fields, which stay `pub(crate)`).
pub_crate_test_struct! {
#[derive(Clone, Copy)]
PrintfSpec {
    pub(crate) alt: bool,      // `#` flag
    pub(crate) space: bool,    // space flag
    pub(crate) left: bool,     // `-` flag
    pub(crate) showsign: bool, // `+` flag
    pub(crate) group: bool,    // `'` flag
    pub(crate) width: i64,
    pub(crate) prec: i64,
    pub(crate) arg_type: ArgType,
    pub(crate) rnd_mode: RoundingMode,
    pub(crate) spec: u8,
    pub(crate) pad: u8,
}}

// This is `specinfo_init` from `vasprintf.c`, MPFR 4.2.2.
const fn specinfo_init() -> PrintfSpec {
    PrintfSpec {
        alt: false,
        space: false,
        left: false,
        showsign: false,
        group: false,
        width: 0,
        prec: 0,
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
const fn floating_point_arg_type(at: ArgType) -> bool {
    matches!(
        at,
        ArgType::Mpfr | ArgType::Mpf | ArgType::Long | ArgType::LongDouble
    )
}

// This is `INTEGER_LIKE_ARG_TYPE` from `vasprintf.c`, MPFR 4.2.2.
const fn integer_like_arg_type(at: ArgType) -> bool {
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
            b'0' => {
                specinfo.pad = b'0';
            }
            b'#' => {
                specinfo.alt = true;
            }
            b'+' => {
                specinfo.showsign = true;
            }
            b' ' => {
                specinfo.space = true;
            }
            b'-' => {
                specinfo.left = true;
            }
            // Single UNIX Specification for thousand separator
            b'\'' => {
                specinfo.group = true;
            }
            _ => {
                return format;
            }
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
const fn parse_arg_type<'a>(format: &'a [u8], specinfo: &mut PrintfSpec) -> &'a [u8] {
    let Some((&format_head, mut format_tail)) = format.split_first() else {
        return format;
    };
    specinfo.arg_type = match format_head {
        b'h' => {
            if let Some((b'h', tail)) = format_tail.split_first() {
                format_tail = tail;
                ArgType::Char
            } else {
                ArgType::Short
            }
        }
        b'l' => {
            if let Some((b'l', tail)) = format_tail.split_first() {
                format_tail = tail;
                ArgType::LongLong
            } else {
                ArgType::Long
            }
        }
        b'j' => ArgType::IntMax,
        b'z' => ArgType::Size,
        b't' => ArgType::PtrDiff,
        b'L' => ArgType::LongDouble,
        b'F' => ArgType::Mpf,
        b'Q' => ArgType::Mpq,
        // The 'M' specifier was added in GMP 4.2.0.
        b'M' => ArgType::MpLimb,
        b'N' => ArgType::MpLimbArray,
        b'Z' => ArgType::Mpz,
        // mpfr-specific specifiers
        b'P' => ArgType::MpfrPrec,
        b'R' => ArgType::Mpfr,
        // not a length modifier — leave it for the conversion parser
        _ => return format,
    };
    format_tail
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
    let new_len = b.chars.len() + usize::exact_from(n);
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
    ip_trailing_digits: i32, // additional integral zeros (from rounding up to a power of 10)
    point: u8,               // decimal point character, or '\0'
    fp_leading_zeros: i64,   // additional leading zeros in the fractional part
    fp: Vec<u8>,             // fractional-part digits (was fp_ptr / fp_size)
    fp_trailing_zeros: i64,  // additional trailing zeros in the fractional part
    exp: Vec<u8>,            // exponent part (was exp_ptr / exp_size)
}

// Returns `s` with its trailing '0' characters removed. (This inlines the strip-trailing-zeros
// loops that `vasprintf.c` repeats in `regular_eg`, `regular_fg`, and `regular_ab`.)
fn strip_trailing_zeros(mut s: &[u8]) -> &[u8] {
    while let [rest @ .., b'0'] = s {
        s = rest;
    }
    s
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

// For a real nonzero number `x`, returns the exponent `f` so that `10^f <= |x| < 10^(f + 1)`.
//
// This is `floor_log10` from `vasprintf.c`, MPFR 4.2.2.
fn floor_log10(x: &Float) -> i64 {
    // `y` needs enough precision to represent the exponent exactly and to compare with `x`.
    let prec = x.get_prec().unwrap().max(i64::BITS.into());
    let exp = ceil_mul(i64::from(x.get_exponent().unwrap()), 10, 1) - 1;
    // `y = 10 ^ exp`, rounded up. This is fast: `exp` is an integer (not too large), so the
    // exponentiation reduces to `pow_z` internally.
    let y = Float::power_of_10_of_float_prec_round(Float::from(exp), prec, Up).0;
    if x.lt_abs(&y) { exp - 1 } else { exp }
}

// Initializes a `NumberParts` with the neutral values that `partition_number` sets before filling
// in the parts specific to a number.
const fn number_parts_init() -> NumberParts {
    NumberParts {
        pad_type: PadType::Right,
        pad_size: 0,
        sign: b'\0',
        prefix: b"",
        thousands_sep: b'\0',
        ip: Vec::new(),
        ip_trailing_digits: 0,
        point: b'\0',
        fp_leading_zeros: 0,
        fp: Vec::new(),
        fp_trailing_zeros: 0,
        exp: Vec::new(),
    }
}

// Determines the parts of the string representation of the regular number `p` when `spec.spec` is
// 'e', 'E', 'g', or 'G'. Returns -1 in case of overflow on the sizes, 0 otherwise.
//
// This is `regular_eg` from `vasprintf.c`, MPFR 4.2.2.
fn regular_eg(
    np: &mut NumberParts,
    p: &Float,
    spec: &PrintfSpec,
    dec_info: Option<&DecimalInfo>,
    keep_trailing_zeros: bool,
) -> i32 {
    let uppercase = spec.spec == b'E' || spec.spec == b'G';
    // integral part: one significant digit
    let storage;
    let (str, exp): (&[u8], i64) = match dec_info {
        None => {
            // We keep the trailing zeros, so `mpfr_get_str_wrapper` may be used.
            debug_assert!(keep_trailing_zeros);
            // Number of significant digits: 0 (let get_str decide) if no precision, else one digit
            // before the point plus `spec.prec` after it.
            let nsd = if spec.prec < 0 {
                0
            } else {
                let Some(n) = usize::try_from(spec.prec)
                    .ok()
                    .and_then(|p| p.checked_add(1))
                else {
                    fail_on_untested_path("regular_eg, nsd overflows usize");
                    return -1; // overflow
                };
                n
            };
            storage = mpfr_get_str_wrapper(10, nsd, p, spec);
            (&storage.0, storage.1)
        }
        Some(d) => (&d.str, d.exp),
    };
    // skip the sign character if any
    let digits: &[u8] = if p.is_sign_negative() { &str[1..] } else { str };
    np.ip = vec![digits[0]];

    if spec.prec != 0 {
        // the sign and the first digit have been skipped
        let mut frac = &digits[1..];
        if !keep_trailing_zeros {
            frac = strip_trailing_zeros(frac);
        }
        let str_len = frac.len();
        if str_len != 0 {
            np.fp = frac.to_vec();
            debug_assert!(spec.prec < 0 || i64::exact_from(str_len) <= spec.prec);
            if keep_trailing_zeros && spec.prec > 0 && i64::exact_from(str_len) < spec.prec {
                // add missing trailing zeros
                np.fp_trailing_zeros = spec.prec - i64::exact_from(str_len);
            }
        }
    }

    // decimal point
    if !np.fp.is_empty() || spec.alt {
        np.point = b'.';
    }

    // `exp` is the exponent for the decimal point BEFORE the first digit; we want it AFTER the
    // first digit. No possible overflow because exp < EXP(p) / 3.
    let exp = exp - 1;

    // The exponent part is 'e' or 'E', plus the sign, plus at least two digits, plus only as many
    // more digits as necessary.
    let uexp = exp.unsigned_abs();
    let mut exp_part = Vec::new();
    exp_part.push(if uppercase { b'E' } else { b'e' });
    exp_part.push(if exp >= 0 { b'+' } else { b'-' });
    exp_part.extend_from_slice(format!("{uexp:02}").as_bytes());
    np.exp = exp_part;
    0
}

// Determines the parts of the string representation of the regular number `p` when `spec.spec` is
// 'f', 'F', 'g', or 'G'. `dec_info` is the previously-computed exponent and string, or `None`.
// Returns -1 in case of overflow on the sizes, 0 otherwise.
//
// This is `regular_fg` from `vasprintf.c`, MPFR 4.2.2.
fn regular_fg(
    np: &mut NumberParts,
    p: &Float,
    spec: &PrintfSpec,
    dec_info: Option<&DecimalInfo>,
    keep_trailing_zeros: bool,
) -> i32 {
    // An empty precision field is forbidden here (it means 6, set before the call).
    debug_assert!(spec.prec >= 0);
    if p.get_exponent().unwrap() <= 0 {
        // 0 < |p| < 1; the integral part is usually 0.
        np.ip = vec![b'0'];
        if spec.prec == 0 {
            // The output is "1" or "0", and 0 < |p| < 1 means either is inexact; this branch
            // bypasses `get_str`, so it must reject `Exact` itself to stay consistent with the
            // `get_str`-backed paths.
            assert!(
                spec.rnd_mode != Exact,
                "regular_fg: Exact rounding was requested, but {p} is not exactly representable \
                with 0 fractional digits",
            );
            // either 1 or 0
            let round_up = match spec.rnd_mode {
                Floor => p.is_sign_negative(),
                Ceiling => p.is_sign_positive(),
                Up => true,
                // note that 0.5 rounds to 0 with Nearest (round ties to even)
                Nearest => p.partial_cmp_abs(&Float::ONE_HALF).unwrap() == Greater,
                _ => false,
            };
            if round_up {
                np.ip[0] = b'1';
            }
        } else {
            // exp = position of the most significant decimal digit
            let exp = floor_log10(p);
            debug_assert!(exp < 0);
            if exp < -spec.prec {
                // Only the last digit may be nonzero, and exp < -spec.prec means the printed value
                // (0 or 10^-prec in absolute value) is never |p| itself, so this
                // `get_str`-bypassing branch is always inexact and must reject `Exact`.
                let round_away = match spec.rnd_mode {
                    Up => true,
                    Down => false,
                    Floor => p.is_sign_negative(),
                    Ceiling => p.is_sign_positive(),
                    Exact => panic!(
                        "regular_fg: Exact rounding was requested, but {p} is not exactly \
                        representable with {} fractional digits",
                        spec.prec
                    ),
                    Nearest => {
                        // compare |p| to y = 0.5 * 10^(-spec.prec), increasing the precision of y
                        // until it differs from |p| so that the comparison is decisive
                        let mut e = p.get_prec().unwrap().max(56);
                        loop {
                            e += 8;
                            let y = Float::power_of_10_of_float_prec_round(
                                Float::from(-spec.prec),
                                e,
                                Down,
                            )
                            .0 >> 1u64;
                            let cmp = y.partial_cmp_abs(p).unwrap();
                            if cmp != Equal {
                                break cmp == Less;
                            }
                        }
                    }
                };
                np.fp_leading_zeros = if round_away {
                    // the last output digit is '1'
                    np.fp = vec![b'1'];
                    spec.prec - 1
                } else {
                    // only zeros in the fractional part
                    debug_assert!(spec.spec == b'f' || spec.spec == b'F');
                    spec.prec
                };
            } else {
                // exp >= -spec.prec: the significant digits are the last spec.prec + exp + 1 digits
                // in the fractional part
                let storage;
                let (str, exp): (&[u8], i64) = match dec_info {
                    None => {
                        debug_assert!(keep_trailing_zeros);
                        // no overflow: exp <= -1, so the sum is at most spec.prec
                        debug_assert!(exp <= -1 && spec.prec + (exp + 1) >= 0);
                        let Ok(nsd) = usize::try_from(spec.prec + (exp + 1)) else {
                            fail_on_untested_path("regular_fg, sub-1 nsd overflows usize");
                            return -1;
                        };
                        storage = mpfr_get_str_wrapper(10, nsd, p, spec);
                        (&storage.0, storage.1)
                    }
                    Some(d) => (&d.str, d.exp),
                };
                let digits: &[u8] = if p.is_sign_negative() { &str[1..] } else { str };
                if exp == 1 {
                    // rounded up to 1
                    debug_assert!(digits[0] == b'1');
                    np.ip[0] = b'1';
                    if keep_trailing_zeros {
                        np.fp_leading_zeros = spec.prec;
                    }
                } else {
                    np.fp_leading_zeros = -exp;
                    debug_assert!(exp <= 0);
                    let digits = if keep_trailing_zeros {
                        digits
                    } else {
                        strip_trailing_zeros(digits)
                    };
                    let str_len = digits.len();
                    debug_assert!(str_len > 0);
                    np.fp = digits.to_vec();
                    if keep_trailing_zeros {
                        // add missing trailing zeros so that fp_size + fp_trailing_zeros equals
                        // prec + exp
                        np.fp_trailing_zeros = (spec.prec + exp) - i64::exact_from(str_len);
                        debug_assert!(np.fp_trailing_zeros >= 0);
                    }
                }
            }
        }
        if spec.alt || np.fp_leading_zeros != 0 || !np.fp.is_empty() || np.fp_trailing_zeros != 0 {
            np.point = b'.';
        }
    } else {
        // 1 <= |p|
        let storage;
        let (str, exp): (&[u8], i64) = match dec_info {
            None => {
                // %f case. (The %g case has no use for `floor_log10`, whose power of 10 is computed
                // at the full precision of `p`, so it is only called here.)
                let exp = floor_log10(p);
                debug_assert!(exp >= 0);
                // MPFR computes this sum in `mpfr_uintmax_t` so that it cannot overflow the signed
                // type; use a checked addition instead.
                let n = match spec.prec.checked_add(exp + 1).map(usize::try_from) {
                    Some(Ok(n)) => n,
                    _ => return -1,
                };
                storage = mpfr_get_str_wrapper(10, n, p, spec);
                (&storage.0, storage.1)
            }
            // %g case
            Some(d) => (&d.str, d.exp),
        };
        let digits: &[u8] = if p.is_sign_negative() { &str[1..] } else { str };
        let str_len = digits.len();
        // integral part: `exp` (from get_str) is the number of integral digits
        let ip_size = if exp > i64::exact_from(str_len) {
            // rounding up to the next power of 10 requires an added trailing zero
            np.ip_trailing_digits = i32::exact_from(exp - i64::exact_from(str_len));
            str_len
        } else {
            usize::exact_from(exp)
        };
        np.ip = digits[..ip_size].to_vec();
        if spec.group {
            // MPFR uses the locale's thousands separator here, which is EMPTY in the default C
            // locale (so MPFR prints no separators there); Malachite has no locale machinery, so
            // the `'` flag always groups with a comma.
            np.thousands_sep = b',';
        }
        // fractional part
        let mut frac = &digits[ip_size..];
        if !keep_trailing_zeros {
            frac = strip_trailing_zeros(frac);
        }
        let frac_len = frac.len();
        if frac_len > 0 {
            np.point = b'.';
            np.fp = frac.to_vec();
        }
        if keep_trailing_zeros && i64::exact_from(frac_len) < spec.prec {
            // add missing trailing zeros
            np.point = b'.';
            np.fp_trailing_zeros = spec.prec - i64::exact_from(np.fp.len());
            debug_assert!(np.fp_trailing_zeros >= 0);
        }
        if spec.alt {
            np.point = b'.';
        }
    }
    0
}

// The default precision for the 'f'/'F'/'g'/'G' conversions (as in C, this is 6).
const DEFAULT_DECIMAL_PREC: i64 = 6;

// Whether the rounding mode `rnd` rounds a value with sign `neg` away from zero.
//
// This is `MPFR_IS_LIKE_RNDA` from MPFR 4.2.2.
fn is_like_rnda(rnd: RoundingMode, neg: bool) -> bool {
    rnd == Up || (rnd == Ceiling && !neg) || (rnd == Floor && neg)
}

// Whether the significand `sig` (normalized, so its top bit is set and its bit count is a multiple
// of `Limb::WIDTH`) has any set bit below its top `nbits` bits — that is, whether printing the
// corresponding [`Float`] with a single base-2^`nbits` digit is inexact.
fn one_digit_is_inexact(sig: &Natural, nbits: u64) -> bool {
    sig.trailing_zeros().unwrap() < sig.significant_bits() - nbits
}

// For a real nonzero `x` rounded to a single base-`base` digit, returns whether `x` rounds up to
// the next power of `base`. `base` is 2 or 16.
//
// This is `next_base_power_p` from `vasprintf.c`, MPFR 4.2.2, with a fix for an upstream bug: for
// the non-`Nearest` rounding modes, MPFR only examines the most significant limb for remaining
// bits, so inexactness held entirely in the lower limbs is missed and the value is not rounded up
// (e.g. 2^100 + 1 with "%.0RUb").
fn next_base_power_p(x: &Float, base: i64, rnd: RoundingMode) -> bool {
    // the decimal point is after the first digit in this representation
    let nbits: u64 = if base == 2 { 1 } else { 4 };
    if rnd == Down
        || (rnd == Floor && x.is_sign_positive())
        || (rnd == Ceiling && x.is_sign_negative())
        || x.get_prec().unwrap() <= nbits
    {
        // no rounding when printing x with a single digit
        return false;
    }
    let sig = x.significand_ref().unwrap();
    let xm = sig.limbs().next_back().unwrap();
    // mask of the low (WIDTH - nbits) bits
    let low_mask = Limb::low_mask(Limb::WIDTH - nbits);
    let high_mask = !low_mask;
    if (xm & high_mask) ^ high_mask != 0 {
        // don't round up if some of the first nbits bits are 0
        return false;
    }
    if rnd == Nearest {
        // round up if the rounding bit is 1
        xm.get_bit(Limb::WIDTH - nbits - 1)
    } else {
        // an away-from-zero-like rounding mode: round up if any remaining bit is 1
        one_digit_is_inexact(sig, nbits)
    }
}

// Determines the parts of the string representation of the regular number `p` when `spec.spec` is
// 'a', 'A', or 'b'. Returns -1 in case of overflow on the sizes, 0 otherwise.
//
// This is `regular_ab` from `vasprintf.c`, MPFR 4.2.2.
fn regular_ab(np: &mut NumberParts, p: &Float, spec: &PrintfSpec) -> i32 {
    let uppercase = spec.spec == b'A';
    if spec.spec == b'a' || spec.spec == b'A' {
        np.prefix = if uppercase { b"0X" } else { b"0x" };
    }
    let base: i64 = if spec.spec == b'b' { 2 } else { 16 };
    // the sign-skipped digit string, and the base-two exponent for a point after the first digit
    let (mut digits, exp): (Vec<u8>, i64) = if spec.prec != 0 {
        // one digit before the point plus spec.prec after it (or 0 to let get_str decide)
        let nsd = if spec.prec < 0 {
            0
        } else {
            let Some(n) = usize::try_from(spec.prec)
                .ok()
                .and_then(|p| p.checked_add(1))
            else {
                fail_on_untested_path("regular_ab, nsd overflows usize");
                return -1;
            };
            n
        };
        let (s, e) = mpfr_get_str_wrapper(base, nsd, p, spec);
        let digits = if p.is_sign_negative() {
            s[1..].to_vec()
        } else {
            s
        };
        // base 16: get_str's exponent is base-16 with the point before the first digit; we want
        // base-2 with the point after the first digit
        let exp = if base == 16 { (e - 1) << 2 } else { e - 1 };
        (digits, exp)
    } else {
        let mut e = i64::from(p.get_exponent().unwrap());
        let sig = p.significand_ref().unwrap();
        // A single digit that drops set bits is inexact; this path bypasses `get_str`, so it must
        // reject `Exact` itself.
        assert!(
            spec.rnd_mode != Exact || !one_digit_is_inexact(sig, if base == 2 { 1 } else { 4 }),
            "regular_ab: Exact rounding was requested, but {p} is not exactly representable with \
            a single base-{base} digit",
        );
        let digit_byte = if next_base_power_p(p, base, spec.rnd_mode) {
            b'1'
        } else if base == 2 {
            e -= 1;
            b'1'
        } else {
            // base 16: form the leading digit from the top 4 bits of the top significand limb
            let msl = sig.limbs().next_back().unwrap();
            let rnd_bit = Limb::WIDTH - 5;
            let mut digit = u8::exact_from(msl >> (rnd_bit + 1));
            // Round the digit up only if the value actually has bits below the top nibble. MPFR
            // 4.2.2 omits this exactness check — an upstream bug that rounds exact values away
            // (e.g. "%.0RUa" of 1.5 gives 0xdp-3 = 1.625 instead of 0xcp-3) and overflows its digit
            // table when the top digit is 0xf ("%.0RUa" of 15 prints garbage). With the check, an
            // all-ones nibble with remaining bits always lands in `next_base_power_p` first, so
            // digit <= 15 here.
            if (is_like_rnda(spec.rnd_mode, p.is_sign_negative()) && one_digit_is_inexact(sig, 4))
                || (spec.rnd_mode == Nearest && (msl & (Limb::ONE << rnd_bit)) != 0)
            {
                digit += 1;
            }
            debug_assert!(digit <= 15);
            e -= 4;
            digit_to_display_byte_lower(digit).unwrap()
        };
        (vec![digit_byte], e)
    };
    // all digits in upper case for 'A'
    if uppercase {
        digits.make_ascii_uppercase();
    }
    np.ip = vec![digits[0]];

    if spec.spec == b'b' || spec.prec != 0 {
        // the sign and the first digit have been skipped
        let mut frac = &digits[1..];
        if spec.prec < 0 {
            frac = strip_trailing_zeros(frac);
        }
        let str_len = frac.len();
        if str_len != 0 {
            np.fp = frac.to_vec();
            if spec.prec > 0 && i64::exact_from(str_len) < spec.prec {
                // Unreachable: with an explicit precision, `mpfr_get_str_wrapper` returns exactly
                // `spec.prec + 1` digits, so `str_len` (after the leading digit) equals
                // `spec.prec`. (Unlike the decimal `regular_eg`/`regular_fg`, there is no
                // `%g`-style path here that hands `regular_ab` a shorter cached string.)
                fail_on_untested_path("regular_ab, trailing-zero pad");
                np.fp_trailing_zeros = spec.prec - i64::exact_from(str_len);
            }
        }
    }

    // decimal point
    if !np.fp.is_empty() || spec.alt {
        np.point = b'.';
    }

    // The exponent part is 'p' or 'P', plus the sign, plus at least one digit.
    let uexp = exp.unsigned_abs();
    let mut exp_part = Vec::new();
    exp_part.push(if uppercase { b'P' } else { b'p' });
    exp_part.push(if exp >= 0 { b'+' } else { b'-' });
    exp_part.extend_from_slice(format!("{uexp}").as_bytes());
    np.exp = exp_part;
    0
}

// Determines the different parts of the string representation of `p` according to `spec`, filling
// `np` (all previous information in `np` is lost). Returns the total number of characters to be
// written, or -1 on overflow.
//
// This is `partition_number` from `vasprintf.c`, MPFR 4.2.2.
fn partition_number(np: &mut NumberParts, p: &Float, mut spec: PrintfSpec) -> i64 {
    *np = number_parts_init();
    // left justification means right space padding
    np.pad_type = if spec.left {
        PadType::Right
    } else if spec.pad == b'0' {
        PadType::LeadingZeros
    } else {
        PadType::Left
    };
    let uppercase = matches!(spec.spec, b'A' | b'E' | b'F' | b'G');
    // the sign/space rule is the same for all cases
    np.sign = if p.is_sign_negative() {
        b'-'
    } else if spec.showsign {
        b'+'
    } else if spec.space {
        b' '
    } else {
        b'\0'
    };

    if p.is_nan() {
        if matches!(np.pad_type, PadType::LeadingZeros) {
            // don't want "0000nan"; use left-space padding instead
            np.pad_type = PadType::Left;
        }
        np.ip = if uppercase { b"NAN" } else { b"nan" }.to_vec();
    } else if p.is_infinite() {
        if matches!(np.pad_type, PadType::LeadingZeros) {
            np.pad_type = PadType::Left;
        }
        np.ip = if uppercase { b"INF" } else { b"inf" }.to_vec();
    } else if p.is_zero() {
        // Note: for 'g', zero is displayed 'f'-style with precision spec.prec - 1 and the trailing
        // zeros removed unless the '#' flag is used.
        if spec.spec == b'a' || spec.spec == b'A' {
            np.prefix = if uppercase { b"0X" } else { b"0x" };
        }
        np.ip = vec![b'0'];
        if spec.prec < 0 {
            // empty precision field
            if spec.spec == b'e' || spec.spec == b'E' {
                // Malachite zeros are precision-less (unlike MPFR); fall back to precision 1 when
                // the zero carries none.
                let zprec = p.get_prec().unwrap_or(1);
                spec.prec = i64::exact_from(get_str_ndigits(10, zprec)) - 1;
            } else if matches!(spec.spec, b'f' | b'F' | b'g' | b'G') {
                spec.prec = DEFAULT_DECIMAL_PREC;
            }
        }
        if spec.prec > 0 && ((spec.spec != b'g' && spec.spec != b'G') || spec.alt) {
            np.point = b'.';
            np.fp_trailing_zeros = if spec.spec == b'g' || spec.spec == b'G' {
                spec.prec - 1
            } else {
                spec.prec
            };
            debug_assert!(np.fp_trailing_zeros >= 0);
        } else if spec.alt {
            np.point = b'.';
        }
        if matches!(spec.spec, b'a' | b'A' | b'b' | b'e' | b'E') {
            // exponent part
            np.exp = if spec.spec == b'e' || spec.spec == b'E' {
                if uppercase { b"E+00" } else { b"e+00" }.to_vec()
            } else {
                if uppercase { b"P+0" } else { b"p+0" }.to_vec()
            };
        }
    } else {
        // pure FP (regular number)
        if spec.spec == b'a' || spec.spec == b'A' || spec.spec == b'b' {
            if regular_ab(np, p, &spec) == -1 {
                return -1;
            }
        } else if spec.spec == b'f' || spec.spec == b'F' {
            if spec.prec < 0 {
                spec.prec = DEFAULT_DECIMAL_PREC;
            }
            if regular_fg(np, p, &spec, None, true) == -1 {
                return -1;
            }
        } else if spec.spec == b'e' || spec.spec == b'E' {
            if regular_eg(np, p, &spec, None, true) == -1 {
                return -1;
            }
        } else {
            // %g case, using the C99 rules: with T the threshold below and X the exponent that
            // would be displayed with style 'e' and precision T - 1, if T > X >= -4 the conversion
            // is style 'f'/'F' with precision T - (X + 1), otherwise style 'e'/'E' with precision T
            // - 1.
            let threshold = match spec.prec {
                i64::MIN..0 => DEFAULT_DECIMAL_PREC,
                0 => 1,
                _ => spec.prec,
            };
            debug_assert!(threshold >= 1);
            // Try a smaller threshold for get_str: |p| < 2^EXP(p), so the integer part takes at
            // most ceil(EXP(p) * log10(2)) digits, and with k = PREC(p) - EXP(p), the fractional
            // part in base 10 has at most k digits (if k > 0).
            let exp_p = i64::from(p.get_exponent().unwrap());
            let k = i64::exact_from(p.get_prec().unwrap()) - exp_p;
            let mut e = if exp_p <= 0 {
                k
            } else {
                (exp_p + 2) / 3 + if k <= 0 { 0 } else { k }
            };
            debug_assert!(e >= 1);
            if e > threshold {
                e = threshold;
            }
            // error if e does not fit in a usize (for get_str)
            let Ok(e) = usize::try_from(e) else {
                fail_on_untested_path("partition_number, %g e overflows usize");
                return -1;
            };
            // We need the full significand, so call get_str directly (not the wrapper).
            let (str, dec_exp, _) = get_str(p, 10, e, spec.rnd_mode).unwrap();
            let dec_info = DecimalInfo { exp: dec_exp, str };
            // get_str's significand is in [0.1, 1); we want it in [1, 10).
            let x = dec_info.exp - 1;
            if threshold > x && x >= -4 {
                // x may be as low as -4, so the subtraction can overflow for a threshold within 3
                // of i64::MAX; fail like the other size overflows.
                spec.prec = match threshold.checked_sub(x).and_then(|d| d.checked_sub(1)) {
                    Some(prec) => prec,
                    None => return -1,
                };
                if regular_fg(np, p, &spec, Some(&dec_info), spec.alt) == -1 {
                    return -1;
                }
            } else {
                spec.prec = threshold - 1;
                if regular_eg(np, p, &spec, Some(&dec_info), spec.alt) == -1 {
                    return -1;
                }
            }
        }
    }

    // Compute the number of characters to be written, checking against i64::MAX (MPFR_INTMAX_MAX)
    // via a wider accumulator.
    let mut total: i128 = i128::from(np.sign != b'\0');
    total += np.prefix.len() as i128;
    total += np.ip.len() as i128;
    total += i128::from(np.ip_trailing_digits);
    debug_assert!(np.ip.len() as i128 + i128::from(np.ip_trailing_digits) >= 1);
    if np.thousands_sep != b'\0' {
        total += (np.ip.len() as i128 + i128::from(np.ip_trailing_digits) - 1) / 3;
    }
    if np.point != b'\0' {
        total += 1;
    }
    total += i128::from(np.fp_leading_zeros);
    total += np.fp.len() as i128;
    total += i128::from(np.fp_trailing_zeros);
    total += np.exp.len() as i128;

    if i128::from(spec.width) > total {
        // pad with spaces or zeros depending on np.pad_type
        np.pad_size = spec.width - i64::exact_from(total);
        total = i128::from(spec.width);
    }
    if total > i128::from(i64::MAX) {
        fail_on_untested_path("partition_number, total width overflows i64");
        return -1;
    }
    i64::exact_from(total)
}

// Prints `p` into `buf` according to `spec`. Returns the number of characters written, or -1 if the
// built string is too long.
//
// This is `sprnt_fp` from `vasprintf.c`, MPFR 4.2.2.
fn sprnt_fp(buf: &mut StringBuffer, p: &Float, spec: &PrintfSpec) -> i64 {
    let mut np = number_parts_init();
    let length = partition_number(&mut np, p, *spec);
    if length < 0 {
        return -1;
    }
    // MPFR sizes its buffer from `length` up front; reserve to match.
    if let Ok(len) = usize::try_from(length) {
        buf.chars.reserve(len);
    }
    // right justification padding with left spaces
    if matches!(np.pad_type, PadType::Left) && np.pad_size != 0 {
        buffer_pad(buf, b' ', np.pad_size);
    }
    // sign character (may be '-', '+', ' ', or '\0')
    if np.sign != b'\0' {
        buffer_pad(buf, np.sign, 1);
    }
    // prefix part
    if !np.prefix.is_empty() {
        buffer_cat(buf, np.prefix);
    }
    // right justification padding with leading zeros
    if matches!(np.pad_type, PadType::LeadingZeros) && np.pad_size != 0 {
        buffer_pad(buf, b'0', np.pad_size);
    }
    // integral part (never empty)
    if np.thousands_sep != b'\0' {
        buffer_sandwich(
            buf,
            &np.ip,
            np.ip.len(),
            usize::exact_from(np.ip_trailing_digits),
            np.thousands_sep,
        );
    } else {
        buffer_cat(buf, &np.ip);
        // possible trailing zero in the integral part
        debug_assert!(np.ip_trailing_digits <= 1);
        if np.ip_trailing_digits != 0 {
            buffer_pad(buf, b'0', 1);
        }
    }
    // decimal point
    if np.point != b'\0' {
        buffer_pad(buf, np.point, 1);
    }
    // leading zeros in the fractional part
    if np.fp_leading_zeros != 0 {
        buffer_pad(buf, b'0', np.fp_leading_zeros);
    }
    // significant digits in the fractional part
    if !np.fp.is_empty() {
        buffer_cat(buf, &np.fp);
    }
    // trailing zeros in the fractional part
    if np.fp_trailing_zeros != 0 {
        buffer_pad(buf, b'0', np.fp_trailing_zeros);
    }
    // exponent part
    if !np.exp.is_empty() {
        buffer_cat(buf, &np.exp);
    }
    // left justification padding with right spaces
    if matches!(np.pad_type, PadType::Right) && np.pad_size != 0 {
        buffer_pad(buf, b' ', np.pad_size);
    }
    length
}

// Builds a [`PrintfSpec`] for a single `%R<conv>` conversion with the given precision (negative
// means unset), field width, and rounding mode. The flag fields start out cleared, so callers (e.g.
// a future `Display` wiring, which needs `alt`/`showsign`/`left`/`pad`) may set them on the result
// before calling [`format_float`].
pub_const_crate_test! {float_conversion_spec(
    conv: u8,
    prec: i64,
    width: i64,
    rm: RoundingMode,
) -> PrintfSpec {
    let mut spec = specinfo_init();
    spec.spec = conv;
    spec.prec = prec;
    spec.width = width;
    spec.rnd_mode = rm;
    spec
}}

// Formats the [`Float`] `p` for a single `%R<conv>` conversion described by `spec`, returning the
// formatted string, or `None` on an internal size overflow (where MPFR returns -1). This is the
// core of the `%R` path; [`format`] is the multi-conversion format-string frontend on top of it.
pub_crate_test! {format_float(p: &Float, spec: &PrintfSpec) -> Option<String> {
    let mut buf = buffer_init(0);
    if sprnt_fp(&mut buf, p, spec) < 0 {
        return None;
    }
    Some(String::from_utf8(buf.chars).unwrap())
}}

// An argument supplied to [`format`]. The `%R<conv>` conversions consume a [`Float`]; the `*`
// width/precision fields and the `%d`/`%i` conversions consume an [`Int`]; `%s` consumes a [`Str`].
// This replaces the C `va_list`.
pub(crate) enum PrintfArg<'a> {
    Float(&'a Float),
    Int(i64),
    Str(&'a str),
}

// Whether `c` is one of the floating-point conversion specifiers.
const fn is_float_conversion(c: u8) -> bool {
    matches!(
        c,
        b'a' | b'A' | b'b' | b'e' | b'E' | b'f' | b'F' | b'g' | b'G'
    )
}

// Reads a decimal integer or a `*` field from the front of `fmt`, returning the value and the
// unconsumed tail. A `*` consumes the next `Int` argument. The value is `None` if a literal
// overflows an `i64` (the C `READ_INT` macro sets an overflow flag that makes `mpfr_vasnprintf_aux`
// fail with EOVERFLOW; here the caller bails out likewise).
fn read_int<'a>(fmt: &'a [u8], args: &mut core::slice::Iter<PrintfArg>) -> (Option<i64>, &'a [u8]) {
    if fmt.first() == Some(&b'*') {
        let n = match args.next() {
            Some(PrintfArg::Int(n)) => *n,
            _ => 0,
        };
        (Some(n), &fmt[1..])
    } else {
        let mut n: Option<i64> = Some(0);
        let mut i = 0;
        while i < fmt.len() && fmt[i].is_ascii_digit() {
            n = n
                .and_then(|n| n.checked_mul(10))
                .and_then(|n| n.checked_add(i64::from(fmt[i] - b'0')));
            i += 1;
        }
        (n, &fmt[i..])
    }
}

// Applies the field width and flags of `spec` to the already-rendered `body` (with `sign` prepended
// if present), padding with spaces or leading zeros. `zero_ok` is false for conversions where the
// `0` flag is ignored (e.g. `%s`).
fn pad_to_width(
    out: &mut Vec<u8>,
    sign: Option<u8>,
    body: &[u8],
    spec: &PrintfSpec,
    zero_ok: bool,
) {
    let core_len = body.len() + usize::from(sign.is_some());
    let width = usize::try_from(spec.width).unwrap_or(0);
    let pad = width.saturating_sub(core_len);
    if spec.left {
        if let Some(s) = sign {
            out.push(s);
        }
        out.extend_from_slice(body);
        out.resize(out.len() + pad, b' ');
    } else if zero_ok && spec.pad == b'0' && spec.prec < 0 {
        if let Some(s) = sign {
            out.push(s);
        }
        out.resize(out.len() + pad, b'0');
        out.extend_from_slice(body);
    } else {
        out.resize(out.len() + pad, b' ');
        if let Some(s) = sign {
            out.push(s);
        }
        out.extend_from_slice(body);
    }
}

// Formats a signed integer for the `%d`/`%i` conversions, honoring the precision (minimum digits),
// field width, and the `-`/`0`/`+`/space/`'` flags.
fn format_int(n: i64, spec: &PrintfSpec) -> Vec<u8> {
    let neg = n < 0;
    let mag = n.unsigned_abs();
    let mut digits = format!("{mag}").into_bytes();
    if spec.prec >= 0 {
        if spec.prec == 0 && mag == 0 {
            // precision 0 with value 0 produces no digits
            digits.clear();
        } else if let Ok(want) = usize::try_from(spec.prec)
            && digits.len() < want
        {
            let mut d = vec![b'0'; want - digits.len()];
            d.extend_from_slice(&digits);
            digits = d;
        }
    }
    if spec.group && digits.len() > 3 {
        // The `'` flag; a comma, like the float path (and like it, the padding zeros from the `0`
        // flag are not grouped).
        let len = digits.len();
        let mut grouped = Vec::with_capacity(len + (len - 1) / 3);
        // the number of digits in the leftmost block
        let r = (len - 1) % 3 + 1;
        grouped.extend_from_slice(&digits[..r]);
        for chunk in digits[r..].chunks(3) {
            grouped.push(b',');
            grouped.extend_from_slice(chunk);
        }
        digits = grouped;
    }
    let sign = if neg {
        Some(b'-')
    } else if spec.showsign {
        Some(b'+')
    } else if spec.space {
        Some(b' ')
    } else {
        None
    };
    let mut out = Vec::new();
    pad_to_width(&mut out, sign, &digits, spec, true);
    out
}

// Formats a string for the `%s` conversion, honoring the precision (the maximum length in bytes,
// rounded down to a character boundary so that the output remains valid UTF-8) and field width.
fn format_str(s: &str, spec: &PrintfSpec) -> Vec<u8> {
    let s = if spec.prec >= 0 {
        let mut n = usize::try_from(spec.prec)
            .unwrap_or(usize::MAX)
            .min(s.len());
        while !s.is_char_boundary(n) {
            n -= 1;
        }
        &s[..n]
    } else {
        s
    };
    let mut out = Vec::new();
    pad_to_width(&mut out, None, s.as_bytes(), spec, false);
    out
}

// Interprets an MPFR-style format string, consuming `args` from left to right, and returns the
// result, or `None` on failure (where MPFR's `mpfr_vasnprintf_aux` returns -1): a width or
// precision literal overflowing an `i64`, an internal size overflow, a missing or wrongly-typed
// argument, or a conversion that is valid in MPFR but has no counterpart in the `PrintfArg` model.
// Supports the `%R<conv>` float conversions (all flags, field width, precision, rounding, and bases
// 2/10/16), plus `%d`/`%i`, `%s`, `%%`, and `*` width/precision. Invalid conversion specifications
// are dropped (matching MPFR's "the behavior is undefined" choice of not emitting them). The `%n`
// conversion is intentionally unsupported.
//
// This is the `%R` path of `mpfr_vasnprintf_aux`'s main loop from `vasprintf.c`, MPFR 4.2.2, recast
// onto a Rust argument slice instead of a `va_list` (and without the `gmp_vsnprintf` delegation,
// which has no Malachite analog).
pub(crate) fn format(fmt: &[u8], args: &[PrintfArg]) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    let mut fmt = fmt;
    let mut args = args.iter();
    while let Some(&c) = fmt.first() {
        if c != b'%' {
            out.push(c);
            fmt = &fmt[1..];
            continue;
        }
        // c == '%'
        fmt = &fmt[1..];
        if fmt.first() == Some(&b'%') {
            out.push(b'%');
            fmt = &fmt[1..];
            continue;
        }

        let mut spec = specinfo_init();
        fmt = parse_flags(fmt, &mut spec);

        // field width
        let (w, rest) = read_int(fmt, &mut args);
        fmt = rest;
        spec.width = w?;
        if spec.width < 0 {
            // a negative width (from `*`) means left justification
            spec.left = true;
            spec.width = spec.width.saturating_neg();
        }

        // precision
        if fmt.first() == Some(&b'.') {
            fmt = &fmt[1..];
            let (pr, rest) = read_int(fmt, &mut args);
            fmt = rest;
            let pr = pr?;
            spec.prec = if pr < 0 { -1 } else { pr };
        } else {
            spec.prec = -1;
        }

        fmt = parse_arg_type(fmt, &mut spec);

        // rounding mode (only for the mpfr argument type)
        if spec.arg_type == ArgType::Mpfr {
            spec.rnd_mode = match fmt.first() {
                Some(b'D') => {
                    fmt = &fmt[1..];
                    Floor
                }
                Some(b'U') => {
                    fmt = &fmt[1..];
                    Ceiling
                }
                Some(b'Y') => {
                    fmt = &fmt[1..];
                    Up
                }
                Some(b'Z') => {
                    fmt = &fmt[1..];
                    Down
                }
                Some(b'N') => {
                    fmt = &fmt[1..];
                    Nearest
                }
                Some(b'*') => {
                    fmt = &fmt[1..];
                    // MPFR's rounding-mode enum: 0 = RNDN, 1 = RNDZ, 2 = RNDU, 3 = RNDD, 4 = RNDA
                    match args.next() {
                        Some(PrintfArg::Int(1)) => Down,
                        Some(PrintfArg::Int(2)) => Ceiling,
                        Some(PrintfArg::Int(3)) => Floor,
                        Some(PrintfArg::Int(4)) => Up,
                        _ => Nearest,
                    }
                }
                _ => Nearest,
            };
        }

        spec.spec = match fmt.first() {
            Some(&s) => s,
            None => {
                break;
            }
        };
        if specinfo_is_valid(spec) != 1 {
            // invalid conversion specifier: drop it
            fmt = &fmt[1..];
            continue;
        }
        fmt = &fmt[1..];

        // Every conversion must consume exactly one argument (or fail): a valid conversion that
        // silently consumed nothing would desynchronize the argument stream for every later
        // conversion.
        if spec.arg_type == ArgType::Mpfr && is_float_conversion(spec.spec) {
            match args.next()? {
                PrintfArg::Float(p) => {
                    let mut buf = buffer_init(0);
                    if sprnt_fp(&mut buf, p, &spec) < 0 {
                        return None;
                    }
                    out.extend_from_slice(&buf.chars);
                }
                _ => {
                    return None;
                }
            }
        } else if matches!(spec.spec, b'd' | b'i')
            && matches!(
                spec.arg_type,
                ArgType::None
                    | ArgType::Char
                    | ArgType::Short
                    | ArgType::Long
                    | ArgType::LongLong
                    | ArgType::IntMax
                    | ArgType::Size
                    | ArgType::PtrDiff
            )
        {
            match args.next()? {
                PrintfArg::Int(n) => out.extend_from_slice(&format_int(*n, &spec)),
                _ => {
                    return None;
                }
            }
        } else if spec.spec == b's' && spec.arg_type == ArgType::None {
            match args.next()? {
                PrintfArg::Str(s) => out.extend_from_slice(&format_str(s, &spec)),
                _ => {
                    return None;
                }
            }
        } else {
            // A conversion that is valid in MPFR but has no counterpart in the `PrintfArg` model
            // (e.g. `%Zd`, `%u`, `%x`, `%c`, `%p`, `%ls`, or a float conversion without the `R`
            // prefix): fail rather than desynchronize the argument stream.
            return None;
        }
    }
    Some(out)
}

/// Formats a [`Float`] according to an MPFR-style `printf` format string, for strict compatibility
/// with MPFR's `mpfr_printf` family.
///
/// The format string should contain a single conversion consuming the [`Float`], written
/// `%[flags][width][.precision]R[rounding]conv`, with any surrounding literal text (a literal `%`
/// is written `%%`). The pieces are:
/// - **flags**: any of `-` (left-justify within the field), `+` (always show a sign), space (show a
///   space before a nonnegative value), `#` (alternate form: always print a radix point, and keep
///   trailing zeros for `g`/`G`), `0` (pad the field with leading zeros), and `'` (group the
///   integer part into thousands separated by `,`).
/// - **width**: the minimum field width, as a decimal integer.
/// - **precision**: following a `.`, the number of digits after the radix point (for `e`/`f` and
///   their hexadecimal/binary analogues) or the number of significant digits (for `g`); it defaults
///   to 6.
/// - **`R`**: marks the argument as a [`Float`] (MPFR's length modifier).
/// - **rounding**: an optional MPFR rounding character — `N` (to nearest, the default), `D`
///   (toward $-\infty$), `U` (toward $+\infty$), `Y` (away from zero), or `Z` (toward zero).
/// - **conv**: the conversion — `e`/`E` (scientific), `f`/`F` (fixed-point), `g`/`G` (general),
///   `a`/`A` (hexadecimal significand with a binary exponent), or `b` (binary significand with a
///   binary exponent).
///
/// Returns `None` when the format string is not a single well-formed [`Float`] conversion: for
/// instance if it uses `*` for the width or precision (which would need an integer argument that
/// this single-value entry point does not supply), contains no `%R` conversion or more than one,
/// requests a width or precision that overflows, or would produce an over-long result.
///
/// # Worst-case complexity
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.complexity(), p, w)`, with `p`
/// and `w` the precision and field width requested by the format string.
///
/// # Examples
/// ```
/// use malachite_float::conversion::string::format_float::format_float_str;
/// use malachite_float::Float;
///
/// // fixed-point, scientific, and hexadecimal conversions
/// assert_eq!(
///     format_float_str(&Float::from(1.5), "%.3Rf"),
///     Some("1.500".to_string())
/// );
/// assert_eq!(
///     format_float_str(&Float::from(1.5), "%.5Re"),
///     Some("1.50000e+00".to_string())
/// );
/// assert_eq!(
///     format_float_str(&Float::from(255.0), "%Ra"),
///     Some("0xf.fp+4".to_string())
/// );
///
/// // surrounding literal text is copied; a rounding character overrides the default of nearest
/// assert_eq!(
///     format_float_str(&Float::from(1.5), "x = %Rg!"),
///     Some("x = 1.5!".to_string())
/// );
/// assert_eq!(
///     format_float_str(&Float::from(1.5), "%.0RUf"),
///     Some("2".to_string())
/// );
///
/// // `*` needs an integer argument that this single-value entry point does not provide
/// assert_eq!(format_float_str(&Float::from(1.5), "%*Rf"), None);
/// ```
///
/// A single-value entry point over the port of the `%R` path of `mpfr_vasnprintf_aux` (vasprintf.c,
/// MPFR 4.2.2): `format_float_str(x, fmt)` is `format(fmt, &[PrintfArg::Float(x)])`. The output is
/// valid UTF-8 because every literal run of `fmt` (`%` is ASCII, so it never splits a multi-byte
/// character) and every conversion's output is.
#[inline]
pub fn format_float_str(x: &Float, fmt: &str) -> Option<String> {
    format(fmt.as_bytes(), &[PrintfArg::Float(x)]).map(|v| String::from_utf8(v).unwrap())
}

#[cfg(test)]
mod tests {
    use super::PrintfArg::{Float as F, Int, Str};
    use super::{float_conversion_spec, format, format_float, format_float_str};
    use crate::Float;
    use malachite_base::num::arithmetic::traits::PowerOf2;
    use malachite_base::num::basic::traits::One;
    use malachite_base::rounding_modes::RoundingMode::{self, Nearest, Up};
    use malachite_nz::natural::Natural;

    // Convenience: format one Float with default flags.
    fn fmt_float(p: &Float, conv: u8, prec: i64, rm: RoundingMode) -> String {
        format_float(p, &float_conversion_spec(conv, prec, 0, rm)).unwrap()
    }

    // Convenience: run the format-string frontend and collect the result as a `String`.
    fn fmt(f: &[u8], args: &[super::PrintfArg]) -> String {
        String::from_utf8(format(f, args).unwrap()).unwrap()
    }

    #[test]
    fn format_float_smoke() {
        let e = |p: &Float, prec: i64| fmt_float(p, b'e', prec, Nearest);
        let f = |p: &Float, prec: i64| fmt_float(p, b'f', prec, Nearest);
        let g = |p: &Float, prec: i64| fmt_float(p, b'g', prec, Nearest);
        // scientific
        assert_eq!(e(&Float::from(0.0), 5), "0.00000e+00");
        assert_eq!(e(&Float::from(1.5), 5), "1.50000e+00");
        assert_eq!(e(&Float::from(-1.5), 5), "-1.50000e+00");
        assert_eq!(e(&Float::from(1234.5), 3), "1.234e+03");
        assert_eq!(e(&Float::from(0.001234), 3), "1.234e-03");
        // fixed
        assert_eq!(f(&Float::from(0.0), 3), "0.000");
        assert_eq!(f(&Float::from(1.5), 3), "1.500");
        assert_eq!(f(&Float::from(1234.5), 2), "1234.50");
        assert_eq!(f(&Float::from(0.001234), 6), "0.001234");
        // general
        assert_eq!(g(&Float::from(1.5), 6), "1.5");
        assert_eq!(g(&Float::from(1234.5), 6), "1234.5");
        assert_eq!(g(&Float::from(0.0001), 6), "0.0001");
        assert_eq!(g(&Float::from(1000000.0), 6), "1e+06");
    }

    #[test]
    fn format_float_hex_bin_smoke() {
        let a = |p: &Float, prec: i64| fmt_float(p, b'a', prec, Nearest);
        let up_a = |p: &Float, prec: i64| fmt_float(p, b'A', prec, Nearest);
        let b = |p: &Float, prec: i64| fmt_float(p, b'b', prec, Nearest);
        // hexadecimal ('%a')
        assert_eq!(a(&Float::from(0.0), -1), "0x0p+0");
        assert_eq!(a(&Float::from(1.5), -1), "0x1.8p+0");
        assert_eq!(a(&Float::from(-1.5), -1), "-0x1.8p+0");
        assert_eq!(a(&Float::from(255.0), -1), "0xf.fp+4");
        assert_eq!(a(&Float::from(1.5), 2), "0x1.80p+0");
        assert_eq!(up_a(&Float::from(255.0), -1), "0XF.FP+4");
        // binary ('%b')
        assert_eq!(b(&Float::from(1.5), -1), "1.1p+0");
        assert_eq!(b(&Float::from(5.0), -1), "1.01p+2");
        assert_eq!(b(&Float::from(-0.25), -1), "-1p-2");
    }

    #[test]
    fn format_float_single_digit_rounding() {
        // Exact values must not be rounded away (MPFR 4.2.2 gets these wrong: "%.0RUa" of 15
        // overflows its digit table and prints garbage, and of 1.5 prints 0xdp-3 = 1.625).
        assert_eq!(fmt_float(&Float::from(15.0), b'a', 0, Up), "0xfp+0");
        assert_eq!(fmt_float(&Float::from(1.5), b'a', 0, Up), "0xcp-3");
        assert_eq!(
            fmt_float(&Float::from(-15.0), b'a', 0, RoundingMode::Floor),
            "-0xfp+0"
        );
        // inexact values still round away...
        assert_eq!(fmt_float(&Float::from(12.5), b'a', 0, Up), "0xdp+0");
        // ...including up to the next base power for an all-ones top nibble
        assert_eq!(fmt_float(&Float::from(15.5), b'a', 0, Up), "0x1p+4");
        // Inexactness held entirely below the top significand limb must also round up (MPFR 4.2.2
        // only examines the top limb and misses these).
        let big = Float::from_natural_prec(Natural::power_of_2(100) + Natural::ONE, 101).0;
        assert_eq!(fmt_float(&big, b'b', 0, Up), "1p+101");
        let big_hex =
            Float::from_natural_prec((Natural::from(15u32) << 100u32) + Natural::ONE, 104).0;
        assert_eq!(fmt_float(&big_hex, b'a', 0, Up), "0x1p+104");
        // Nearest is unchanged
        assert_eq!(fmt_float(&Float::from(14.0), b'a', 0, Nearest), "0xep+0");
    }

    #[test]
    fn format_float_exact_mode() {
        // Exact is allowed whenever the output represents the value exactly...
        assert_eq!(
            fmt_float(&Float::from(0.5), b'f', 1, RoundingMode::Exact),
            "0.5"
        );
        assert_eq!(
            fmt_float(&Float::from(1.5), b'a', 0, RoundingMode::Exact),
            "0xcp-3"
        );
    }

    #[test]
    #[should_panic(expected = "Exact rounding was requested")]
    fn format_float_exact_mode_fail_decimal() {
        // ...and panics when it does not (previously this silently printed "0")
        fmt_float(&Float::from(0.6), b'f', 0, RoundingMode::Exact);
    }

    #[test]
    #[should_panic(expected = "Exact rounding was requested")]
    fn format_float_exact_mode_fail_hex() {
        fmt_float(&Float::from(15.5), b'a', 0, RoundingMode::Exact);
    }

    #[test]
    fn format_overflow_and_errors() {
        let x = Float::from(1.5);
        // width/precision literals overflowing an i64 fail like MPFR's EOVERFLOW
        assert!(format(b"%.99999999999999999999Rf", &[F(&x)]).is_none());
        assert!(format(b"%99999999999999999999Rf", &[F(&x)]).is_none());
        // internal size overflows fail instead of wrapping
        assert!(format_float(&x, &float_conversion_spec(b'f', i64::MAX, 0, Nearest)).is_none());
        assert!(
            format_float(
                &Float::from(0.001),
                &float_conversion_spec(b'g', i64::MAX - 1, 0, Nearest)
            )
            .is_none()
        );
        // valid-in-MPFR conversions with no PrintfArg counterpart fail instead of silently
        // desynchronizing the argument stream
        assert!(format(b"%u %Rf", &[Int(5), F(&x)]).is_none());
        assert!(format(b"%e %Rf", &[F(&x), F(&x)]).is_none());
        // missing and wrongly-typed arguments fail
        assert!(format(b"%d %d", &[Int(1)]).is_none());
        assert!(format(b"%d", &[Str("x")]).is_none());
    }

    #[test]
    fn format_float_str_smoke() {
        let x = Float::from(1.5);
        let s = |fmt: &str| format_float_str(&x, fmt).unwrap();
        // a bare spec, a spec with surrounding literal text, flags/width/precision, a rounding char
        assert_eq!(s("%.3Rf"), "1.500");
        assert_eq!(s("x = %Rg!"), "x = 1.5!");
        assert_eq!(s("%+08.2Re"), "+1.50e+00");
        assert_eq!(s("%.0RUf"), "2"); // round up
        assert_eq!(s("%Ra"), "0x1.8p+0");
        // `*` width has no integer argument in the single-value form, so it fails
        assert!(format_float_str(&x, "%*Rf").is_none());
    }

    #[test]
    fn format_frontend_smoke() {
        let x = Float::from(1.5);
        let big = Float::from(1234.5);

        // literals and escaped percent
        assert_eq!(fmt(b"no conversions", &[]), "no conversions");
        assert_eq!(fmt(b"100%% done", &[]), "100% done");

        // a single float conversion with the `R` argument type
        assert_eq!(fmt(b"%.3Rf", &[F(&x)]), "1.500");
        assert_eq!(fmt(b"%.5Re", &[F(&big)]), "1.23450e+03");
        assert_eq!(fmt(b"x = %Rg!", &[F(&big)]), "x = 1234.5!");

        // hexadecimal / binary
        assert_eq!(fmt(b"%Ra", &[F(&x)]), "0x1.8p+0");
        assert_eq!(fmt(b"%Rb", &[F(&x)]), "1.1p+0");

        // mixed float, integer, and string conversions in one call, consumed left to right
        assert_eq!(
            fmt(b"%s = %.2Rf (%d)", &[Str("val"), F(&big), Int(-7)]),
            "val = 1234.50 (-7)"
        );

        // field width via `*`, then precision via `*`, then the float
        assert_eq!(fmt(b"[%*.*Rf]", &[Int(10), Int(2), F(&x)]), "[      1.50]");
        // negative `*` width left-justifies
        assert_eq!(fmt(b"[%*Rf]", &[Int(-12), F(&x)]), "[1.500000    ]");

        // rounding-mode characters (round 1.5 at precision 0)
        assert_eq!(fmt(b"%.0RDf", &[F(&x)]), "1"); // toward -inf
        assert_eq!(fmt(b"%.0RUf", &[F(&x)]), "2"); // toward +inf
    }

    #[test]
    fn format_frontend_int_str() {
        // width, zero-pad, precision, sign flags for `%d`
        assert_eq!(fmt(b"%05d", &[Int(42)]), "00042");
        assert_eq!(fmt(b"%-5d|", &[Int(42)]), "42   |");
        assert_eq!(fmt(b"%+d", &[Int(42)]), "+42");
        assert_eq!(fmt(b"% d", &[Int(42)]), " 42");
        assert_eq!(fmt(b"%.4d", &[Int(-7)]), "-0007");
        assert_eq!(fmt(b"%8.4d", &[Int(-7)]), "   -0007");
        // width and precision (truncation) for `%s`
        assert_eq!(fmt(b"%10s|", &[Str("hi")]), "        hi|");
        assert_eq!(fmt(b"%-10s|", &[Str("hi")]), "hi        |");
        assert_eq!(fmt(b"%.3s", &[Str("truncated")]), "tru");
        // `%s` truncation stops at a character boundary rather than splitting UTF-8
        assert_eq!(fmt(b"%.1s", &[Str("\u{e9}a")]), "");
        assert_eq!(fmt(b"%.2s", &[Str("\u{e9}a")]), "\u{e9}");
        assert_eq!(fmt(b"%.3s", &[Str("\u{e9}a")]), "\u{e9}a");
        // the `'` flag groups `%d` like the float path
        assert_eq!(fmt(b"%'d", &[Int(1234567)]), "1,234,567");
        assert_eq!(fmt(b"%'d", &[Int(123)]), "123");
        assert_eq!(fmt(b"%'.0Rf", &[F(&Float::from(1234567.5))]), "1,234,568");
    }

    // Exemplars for the frontend branches (the `format` main loop, the parsers, and the int/string
    // helpers) that the other tests do not reach. Each `// covers:` tag names a branch this case is
    // the first to exercise, found by the same branch-instrumentation pass as
    // `test_format_float_coverage` (which covers the formatting engine).
    #[test]
    fn format_frontend_coverage() {
        let x = Float::from(1.5);
        // covers: fi_zero (precision 0 of the value 0 prints no digits)
        assert_eq!(fmt(b"%.0d", &[Int(0)]), "");
        // covers: fmt_break (format string ends before a conversion character)
        assert_eq!(fmt(b"%5", &[]), "");
        // covers: fmt_float_bad (a `%R` float conversion given a non-Float argument)
        assert!(format(b"%Rf", &[Int(5)]).is_none());
        // covers: fmt_str_bad (a `%s` conversion given a non-Str argument)
        assert!(format(b"%s", &[Int(5)]).is_none());
        // covers: siv_invalid fmt_invalid (an unknown conversion character is dropped)
        assert_eq!(fmt(b"%y", &[]), "");
        // covers: siv_n (`%n` is rejected, then dropped)
        assert_eq!(fmt(b"%n", &[]), "");
        // covers: siv_p fmt_unsupported (`%p` is valid but has no PrintfArg counterpart)
        assert!(format(b"%p", &[]).is_none());
        // covers: pf_alt (the `#` flag)
        assert_eq!(fmt(b"%#Ra", &[F(&x)]), "0x1.8p+0");
        // covers: fmt_prec_neg (a `*` precision of a negative value means "unset")
        assert_eq!(fmt(b"%.*Rf", &[Int(-1), F(&x)]), "1.500000");
        // the explicit rounding characters and the `*` (argument-supplied) rounding mode
        assert_eq!(fmt(b"%RNf", &[F(&x)]), "1.500000"); // covers: fmt_rn
        assert_eq!(fmt(b"%RYf", &[F(&x)]), "1.500000"); // covers: fmt_ry
        assert_eq!(fmt(b"%RZf", &[F(&x)]), "1.500000"); // covers: fmt_rz
        // covers: fmt_rstar fmt_rstar_n (`*` rounding, argument 0 or unrecognized -> nearest)
        assert_eq!(fmt(b"%R*f", &[Int(0), F(&x)]), "1.500000");
        assert_eq!(fmt(b"%R*f", &[Int(1), F(&x)]), "1.500000"); // covers: fmt_rstar_z
        assert_eq!(fmt(b"%R*f", &[Int(2), F(&x)]), "1.500000"); // covers: fmt_rstar_u
        assert_eq!(fmt(b"%R*f", &[Int(3), F(&x)]), "1.500000"); // covers: fmt_rstar_d
        assert_eq!(fmt(b"%R*f", &[Int(4), F(&x)]), "1.500000"); // covers: fmt_rstar_a
        // the integer length modifiers (all accepted for `%d`)
        assert_eq!(fmt(b"%hd", &[Int(1)]), "1"); // covers: pat_h
        assert_eq!(fmt(b"%hhd", &[Int(1)]), "1"); // covers: pat_hh
        assert_eq!(fmt(b"%ld", &[Int(1)]), "1"); // covers: pat_l
        assert_eq!(fmt(b"%lld", &[Int(1)]), "1"); // covers: pat_ll
        assert_eq!(fmt(b"%jd", &[Int(1)]), "1"); // covers: pat_j
        assert_eq!(fmt(b"%zd", &[Int(1)]), "1"); // covers: pat_z
        assert_eq!(fmt(b"%td", &[Int(1)]), "1"); // covers: pat_t
        // length modifiers with no PrintfArg counterpart: `L` (long double) and `F` (mpf_t) make
        // `%d` invalid (dropped); the GMP integer types make it valid-but-unsupported (fails)
        assert_eq!(fmt(b"%Ld", &[]), ""); // covers: pat_l_double
        assert_eq!(fmt(b"%Fd", &[]), ""); // covers: pat_mpf
        assert!(format(b"%Qd", &[]).is_none()); // covers: pat_mpq
        assert!(format(b"%Md", &[]).is_none()); // covers: pat_mplimb
        assert!(format(b"%Nd", &[]).is_none()); // covers: pat_mplimbarray
        assert!(format(b"%Zd", &[]).is_none()); // covers: pat_mpz
        assert!(format(b"%Pd", &[]).is_none()); // covers: pat_mpfrprec
    }
}
