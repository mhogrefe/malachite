// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2026 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use crate::natural::factorization::ecpp::{EcppCertificate, EcppStep};
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::DivMod;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::modular::traits::{Mod, ModMul, ModNeg, ModSquare, ModSub};

const fn is_space(c: u8) -> bool {
    matches!(c, b' ' | b'\n' | b'\t' | b'\r')
}

const fn skip_spaces(s: &[u8], p: &mut usize) {
    while *p < s.len() && is_space(s[*p]) {
        *p += 1;
    }
}

// Reads the decimal integer at s[*p..] (optional sign), and advances *p.
//
// This is equivalent to `_read_int` from `ecpp/cert.c`, FLINT 3.7.0-dev.
fn read_int(s: &[u8], p: &mut usize) -> Option<Integer> {
    skip_spaces(s, p);
    let start = *p;
    let mut q = start;
    if q < s.len() && (s[q] == b'-' || s[q] == b'+') {
        q += 1;
    }
    if q >= s.len() || !s[q].is_ascii_digit() {
        return None;
    }
    while q < s.len() && s[q].is_ascii_digit() {
        q += 1;
    }
    *p = q;
    // fmpz_set_str rejects a leading '+', which Integer::from_str accepts
    if s[start] == b'+' {
        return None;
    }
    Integer::from_str(core::str::from_utf8(&s[start..q]).unwrap()).ok()
}

// Skips whitespace and the expected character; returns None if it is absent.
//
// This is equivalent to `_expect` from `ecpp/cert.c`, FLINT 3.7.0-dev, which returns 0 instead of
// None.
const fn expect(s: &[u8], p: &mut usize, c: u8) -> Option<()> {
    skip_spaces(s, p);
    if *p >= s.len() || s[*p] != c {
        return None;
    }
    *p += 1;
    Some(())
}

fn reduce(x: &Integer, n: &Natural) -> Natural {
    let r = x.unsigned_abs_ref().mod_op(n);
    if *x >= 0u32 { r } else { r.mod_neg(n) }
}

fn find(s: &[u8], from: usize, pat: &[u8]) -> Option<usize> {
    s[from..]
        .windows(pat.len())
        .position(|w| w == pat)
        .map(|i| i + from)
}

// Reads the steps of a certificate in the PARI/GP format, [[N, t, s, a, [x, y]], ...], starting
// just after the opening bracket.
fn read_pari_steps(s: &[u8], mut p: usize, cert: &mut EcppCertificate) -> Option<()> {
    loop {
        skip_spaces(s, &mut p);
        if p < s.len() && s[p] == b']' {
            return Some(());
        }
        if !cert.steps.is_empty() {
            expect(s, &mut p, b',')?;
        }
        expect(s, &mut p, b'[')?;
        let n = read_int(s, &mut p)?;
        expect(s, &mut p, b',')?;
        let t = read_int(s, &mut p)?;
        expect(s, &mut p, b',')?;
        let cofactor = read_int(s, &mut p)?;
        expect(s, &mut p, b',')?;
        let a = read_int(s, &mut p)?;
        expect(s, &mut p, b',')?;
        expect(s, &mut p, b'[')?;
        let x = read_int(s, &mut p)?;
        expect(s, &mut p, b',')?;
        let y = read_int(s, &mut p)?;
        expect(s, &mut p, b']')?;
        expect(s, &mut p, b']')?;
        if n < 3u32 || cofactor <= 0u32 {
            return None;
        }
        let n = Natural::exact_from(n);
        let cofactor = Natural::exact_from(cofactor);
        // m = n + 1 - t, q = m / s (must be exact), b = y^2 - x^3 - a x
        let m = Natural::try_from(Integer::from(&n) + Integer::ONE - t).ok()?;
        let (q, r) = (&m).div_mod(&cofactor);
        if r != 0u32 {
            return None;
        }
        let a = reduce(&a, &n);
        let x = reduce(&x, &n);
        let y = reduce(&y, &n);
        let x3 = (&x).mod_square(&n).mod_mul(&x, &n);
        let ax = (&a).mod_mul(&x, &n);
        let b = (&y).mod_square(&n).mod_sub(x3, &n).mod_sub(ax, &n);
        // D is not part of the format
        cert.steps.push(EcppStep {
            n,
            d: 0,
            a,
            b,
            m,
            q,
            x,
            y,
        });
    }
}

// Reads the steps of a certificate in FLINT's format: "ecpp certificate, k steps" and then the
// labelled steps. Like FLINT, this looks for the labels rather than parsing the lines.
fn read_flint_steps(s: &[u8], p: usize, cert: &mut EcppCertificate) -> Option<()> {
    const LABELS: [&[u8]; 8] = [b"n =", b"D =", b"a =", b"b =", b"m =", b"q =", b"x =", b"y ="];
    let mut pos = find(s, p, b"[0]");
    while let Some(mut p) = pos {
        let mut fields: [Integer; 8] = Default::default();
        for (field, label) in fields.iter_mut().zip(LABELS) {
            let mut q = find(s, p, label)? + 3;
            *field = read_int(s, &mut q)?;
            p = q;
        }
        let [n, d, a, b, m, q, x, y] = fields;
        cert.steps.push(EcppStep {
            n: Natural::try_from(n).ok()?,
            d: i64::try_from(&d).ok()?,
            a: Natural::try_from(a).ok()?,
            b: Natural::try_from(b).ok()?,
            m: Natural::try_from(m).ok()?,
            q: Natural::try_from(q).ok()?,
            x: Natural::try_from(x).ok()?,
            y: Natural::try_from(y).ok()?,
        });
        pos = find(s, p, b"\n[");
    }
    Some(())
}

impl FromStr for EcppCertificate {
    type Err = ();

    /// Reads an [`EcppCertificate`] from a string, in either FLINT's certificate format or the
    /// `primecert` format of PARI/GP.
    ///
    /// The format is recognized by the string's first character other than whitespace: an opening
    /// bracket begins the PARI/GP format, which [`EcppCertificate::to_pari_string`] writes, and
    /// anything else is taken to be FLINT's format, which [`Display`](core::fmt::Display) writes. A
    /// certificate read from the PARI/GP format has $D = 0$, and its $b$ is computed from the point
    /// and $a$; $a$, $x$, and $y$ are reduced modulo $N$, and may be negative in the string. Text
    /// after the end of the certificate is ignored.
    ///
    /// Reading a string does not check that it proves anything; that is what
    /// [`EcppCertificate::verify`] does.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `s.len()`.
    ///
    /// # Errors
    /// Returns `Err(())` if `s` is not a certificate with at least one step. A certificate in
    /// FLINT's format must have nonnegative fields, apart from $D$, which must fit in an [`i64`].
    /// In the PARI/GP format, $N$ must be at least 3, $s$ must be positive and divide $m = N + 1 -
    /// t$, and $m$ must be nonnegative.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::factorization::ecpp::EcppCertificate;
    ///
    /// let flint = "ecpp certificate, 1 steps\n\
    ///     [0] n = 18446744073709551629\n    D = -20\n    a = 4837558977367648749\n    \
    ///     b = 5069635411140069722\n    m = 18446744069060473566\n    q = 206166640403\n    \
    ///     x = 7685973744855724010\n    y = 12593611317027034729\n";
    /// let cert = EcppCertificate::from_str(flint).unwrap();
    /// assert_eq!(cert.steps[0].d, -20);
    /// assert_eq!(cert.to_string(), flint);
    ///
    /// let cert = EcppCertificate::from_str(
    ///     "[[18446744073709551629, 4649078064, 89474922, 4837558977367648749, \
    ///     [7685973744855724010, 12593611317027034729]]]",
    /// )
    /// .unwrap();
    /// assert_eq!(cert.steps[0].d, 0);
    /// assert_eq!(cert.steps[0].b.to_string(), "5069635411140069722");
    ///
    /// assert!(EcppCertificate::from_str("").is_err());
    /// assert!(EcppCertificate::from_str("[]").is_err());
    /// assert!(EcppCertificate::from_str("[[18446744073709551629, 4649078064]]").is_err());
    /// ```
    ///
    /// This is equivalent to `ecpp_cert_set_str` from `ecpp/cert.c`, FLINT 3.7.0-dev, except that
    /// FLINT reads the fields of its own format as signed integers, accepting negative values, and
    /// accepts a $D$ that does not fit in 64 bits.
    fn from_str(s: &str) -> Result<Self, ()> {
        let s = s.as_bytes();
        let mut p = 0;
        let mut cert = Self::default();
        skip_spaces(s, &mut p);
        if p < s.len() && s[p] == b'[' {
            read_pari_steps(s, p + 1, &mut cert)
        } else {
            read_flint_steps(s, p, &mut cert)
        }
        .ok_or(())?;
        if cert.steps.is_empty() {
            Err(())
        } else {
            Ok(cert)
        }
    }
}
