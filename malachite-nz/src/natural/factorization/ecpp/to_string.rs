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
use crate::natural::factorization::ecpp::EcppCertificate;
use alloc::string::String;
use core::fmt::{Display, Formatter, Result, Write};
use malachite_base::num::basic::traits::One;

impl Display for EcppCertificate {
    /// Writes an [`EcppCertificate`] in FLINT's certificate format.
    ///
    /// The format is a header line, `ecpp certificate, ` followed by the number of steps and `
    /// steps`, and then, for each step, its index in brackets followed by its fields, one per line:
    /// `n`, `D`, `a`, `b`, `m`, `q`, `x`, and `y`. [`FromStr`](core::str::FromStr) reads it back,
    /// giving the same certificate.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of significant bits
    /// of the steps' fields.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::factorization::ecpp::EcppCertificate;
    ///
    /// let cert = EcppCertificate::from_str(
    ///     "[[18446744073709551629, 4649078064, 89474922, 4837558977367648749, \
    ///     [7685973744855724010, 12593611317027034729]]]",
    /// )
    /// .unwrap();
    /// assert_eq!(
    ///     cert.to_string(),
    ///     "ecpp certificate, 1 steps\n\
    ///     [0] n = 18446744073709551629\n    D = 0\n    a = 4837558977367648749\n    \
    ///     b = 5069635411140069722\n    m = 18446744069060473566\n    q = 206166640403\n    \
    ///     x = 7685973744855724010\n    y = 12593611317027034729\n"
    /// );
    ///
    /// assert_eq!(
    ///     EcppCertificate::default().to_string(),
    ///     "ecpp certificate, 0 steps\n"
    /// );
    /// ```
    ///
    /// This is equivalent to `ecpp_cert_get_str` from `ecpp/cert.c`, FLINT 3.7.0-dev, with format
    /// `ECPP_CERT_FORMAT_FLINT`.
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "ecpp certificate, {} steps", self.steps.len())?;
        for (i, s) in self.steps.iter().enumerate() {
            writeln!(
                f,
                "[{}] n = {}\n    D = {}\n    a = {}\n    b = {}\n    m = {}\n    q = {}\n    x = \
                {}\n    y = {}",
                i, s.n, s.d, s.a, s.b, s.m, s.q, s.x, s.y
            )?;
        }
        Ok(())
    }
}

impl EcppCertificate {
    /// Converts an [`EcppCertificate`] to a string in the `primecert` format of PARI/GP.
    ///
    /// The certificate is written as a list of steps, each of the form `[N, t, s, a, [x, y]]`,
    /// where $t = N + 1 - m$ and $s = m / q$. The coefficient $b$ is left out, since it is
    /// determined by the point and $a$, and so is the discriminant $D$. This is the format that
    /// PARI/GP's `primecertisvalid` reads, and [`FromStr`](core::str::FromStr) reads it back,
    /// giving the same certificate except that $D$ is 0 and $b$ is recomputed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of significant bits
    /// of the steps' fields.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::factorization::ecpp::EcppCertificate;
    ///
    /// let cert = EcppCertificate::from_str(
    ///     "ecpp certificate, 1 steps\n\
    ///     [0] n = 18446744073709551629\n    D = -20\n    a = 4837558977367648749\n    \
    ///     b = 5069635411140069722\n    m = 18446744069060473566\n    q = 206166640403\n    \
    ///     x = 7685973744855724010\n    y = 12593611317027034729\n",
    /// )
    /// .unwrap();
    /// assert_eq!(
    ///     cert.to_pari_string(),
    ///     "[[18446744073709551629, 4649078064, 89474922, 4837558977367648749, \
    ///     [7685973744855724010, 12593611317027034729]]]"
    /// );
    ///
    /// assert_eq!(EcppCertificate::default().to_pari_string(), "[]");
    /// ```
    ///
    /// This is equivalent to `ecpp_cert_get_str` from `ecpp/cert.c`, FLINT 3.7.0-dev, with format
    /// `ECPP_CERT_FORMAT_PARI`.
    pub fn to_pari_string(&self) -> String {
        let mut out = String::new();
        out.push('[');
        for (i, s) in self.steps.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            let t = Integer::from(&s.n) + Integer::ONE - Integer::from(&s.m);
            let cofactor = &s.m / &s.q;
            write!(
                out,
                "[{}, {}, {}, {}, [{}, {}]]",
                s.n, t, cofactor, s.a, s.x, s.y
            )
            .unwrap();
        }
        out.push(']');
        out
    }
}
