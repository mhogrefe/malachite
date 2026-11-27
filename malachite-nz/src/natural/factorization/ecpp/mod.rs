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

use crate::natural::Natural;
use alloc::vec::Vec;

/// One step of an elliptic curve primality certificate.
///
/// The step concerns the elliptic curve $y^2 = x^3 + ax + b$ over $\mathbb{Z}/n\mathbb{Z}$ and a
/// point $P = (x, y)$ on it. It shows that $n$ is prime, provided that $q$ is prime, where $q$ is a
/// factor of $m$ with $q > (n^{1/4} + 1)^2$. The step is valid when $mP$ is the point at infinity
/// and $(m/q)P$ is not, with every inverse taken along the way existing modulo $n$. If $q$ were
/// prime and $n$ had a prime factor $p \leq \sqrt{n}$, the order of $P$ modulo $p$ would be a
/// multiple of $q$, but would be at most $(\sqrt{p} + 1)^2 < q$ by Hasse's theorem, which is
/// impossible. This is the theorem of Goldwasser and Kilian, in the form that Atkin and Morain use.
///
/// The fields $a$, $b$, $x$, and $y$ need not be reduced modulo $n$; [`EcppStep::verify`] reduces
/// them. The discriminant $D$ of the curve's complex multiplication is recorded for information
/// only, and is not checked.
///
/// This is equivalent to `ecpp_step_struct` from `ecpp.h`, FLINT 3.7.0-dev.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct EcppStep {
    /// The number that the step proves prime, provided that $q$ is prime.
    pub n: Natural,
    /// The discriminant $D$ of the curve, or 0 when it is unknown, as it is for a certificate read
    /// from the PARI/GP format.
    pub d: i64,
    /// The coefficient $a$ of the curve $y^2 = x^3 + ax + b$.
    pub a: Natural,
    /// The coefficient $b$ of the curve $y^2 = x^3 + ax + b$.
    pub b: Natural,
    /// The order of the curve modulo $n$, a multiple of $q$.
    pub m: Natural,
    /// The number whose primality the step depends on; the next step proves it prime.
    pub q: Natural,
    /// The $x$-coordinate of the point $P$.
    pub x: Natural,
    /// The $y$-coordinate of the point $P$.
    pub y: Natural,
}

/// An elliptic curve primality certificate: a proof that a number is prime, made by elliptic curve
/// primality proving (ECPP).
///
/// A certificate for $n$ is a chain of [`EcppStep`]s. The first step's $n$ is $n$, each later
/// step's $n$ is the $q$ of the step before it, and the last step's $q$ is less than $2^{64}$, so
/// that its primality can be checked directly. Each step shows that its $n$ is prime provided that
/// its $q$ is, so together they show that $n$ is prime. A certificate for a prime less than
/// $2^{64}$ has no steps.
///
/// Checking a certificate, with [`EcppCertificate::verify`], is much faster than finding one.
///
/// A certificate is written in one of two text formats: FLINT's own, a labelled list of the steps'
/// fields that [`Display`](core::fmt::Display) produces, and the `primecert` format of PARI/GP,
/// which [`EcppCertificate::to_pari_string`] produces. [`FromStr`](core::str::FromStr) reads
/// either.
///
/// This is equivalent to `ecpp_cert_struct` from `ecpp.h`, FLINT 3.7.0-dev.
///
/// # Examples
/// ```
/// use core::str::FromStr;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural::factorization::ecpp::EcppCertificate;
///
/// let cert = EcppCertificate::from_str(
///     "[[18446744073709551629, 4649078064, 89474922, 4837558977367648749, [7685973744855724010, \
///     12593611317027034729]]]",
/// )
/// .unwrap();
/// assert_eq!(cert.steps.len(), 1);
/// assert_eq!(cert.steps[0].n.to_string(), "18446744073709551629");
/// assert_eq!(cert.steps[0].q.to_string(), "206166640403");
/// assert!(cert.verify(&Natural::from_str("18446744073709551629").unwrap()));
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct EcppCertificate {
    /// The steps of the certificate, starting with the step for the number being proven prime.
    pub steps: Vec<EcppStep>,
}

/// Reading an [`EcppCertificate`] from a string, in either of its formats.
pub mod from_string;
crate_test_mod! {
    /// Elliptic curve point arithmetic modulo a number that might not be prime. Internal machinery
    /// for [`EcppStep::verify`], public only under `test_build`.
    point
}
/// Converting an [`EcppCertificate`] to a string, in either of its formats.
pub mod to_string;
/// Checking an [`EcppCertificate`] or an [`EcppStep`].
pub mod verify;
