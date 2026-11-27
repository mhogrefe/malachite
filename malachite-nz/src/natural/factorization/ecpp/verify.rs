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
use crate::natural::factorization::ecpp::point::{EcppPoint, point_mul_reduced};
use crate::natural::factorization::ecpp::{EcppCertificate, EcppStep};
use malachite_base::num::arithmetic::traits::{FloorSqrt, Parity, Pow};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::IsPrime;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::{DivisibleBy, Mod, ModAdd, ModMul, ModSquare};
use malachite_base::num::number_theory::traits::CoprimeWith;

const THREE: Natural = Natural::const_from(3);
const FOUR: Natural = Natural::const_from(4);
const TWENTY_SEVEN: Natural = Natural::const_from(27);

impl EcppStep {
    /// Checks one step of an elliptic curve primality certificate. If it returns `true` and $q$ is
    /// prime, then $n$ is prime.
    ///
    /// The step is valid if
    /// - $n > 3$ and $\gcd(n, 6) = 1$;
    /// - $q > 1$, and $q$ divides $m$ with $m/q > 1$;
    /// - $(\lfloor\sqrt{q}\rfloor - 1)^4 > n$, which implies $q > (n^{1/4} + 1)^2$;
    /// - $\gcd(4a^3 + 27b^2, n) = 1$, so the curve $y^2 = x^3 + ax + b$ is nonsingular modulo every
    ///   prime factor of $n$;
    /// - $P = (x, y)$ lies on the curve modulo $n$; and
    /// - $mP$ is the point at infinity and $(m/q)P$ is not, where every quantity that the
    ///   computation assumes invertible modulo $n$ is coprime to $n$.
    ///
    /// The fields $a$, $b$, $x$, and $y$ are reduced modulo $n$ first, and $D$ is ignored.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the largest number of significant
    /// bits of the step's fields.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural::factorization::ecpp::EcppCertificate;
    ///
    /// let cert = EcppCertificate::from_str(
    ///     "[[18446744073709551629, 4649078064, 89474922, 4837558977367648749, \
    ///     [7685973744855724010, 12593611317027034729]]]",
    /// )
    /// .unwrap();
    /// let mut step = cert.steps[0].clone();
    /// assert!(step.verify());
    ///
    /// // The point is no longer on the curve
    /// step.y += Natural::ONE;
    /// assert!(!step.verify());
    /// ```
    ///
    /// This is equivalent to `ecpp_verify_step` from `ecpp/verify.c`, FLINT 3.7.0-dev, except that
    /// FLINT requires $a$, $b$, $x$, and $y$ to be reduced modulo $n$.
    pub fn verify(&self) -> bool {
        let n = &self.n;
        // q | m, m = k q with k > 1
        if *n <= 3u32 || n.even() || self.q <= 1u32 || !(&self.m).divisible_by(&self.q) {
            return false;
        }
        let k = &self.m / &self.q;
        if k <= 1u32 {
            return false;
        }
        // q > (n^{1/4} + 1)^2, i.e. (sqrt(q) - 1)^4 > n; check via floor roots
        let t = (&self.q).floor_sqrt();
        if t <= 1u32 {
            return false;
        }
        // (floor(sqrt(q)) - 1)^4; need (sqrt(q) - 1)^4 > n, and sqrt(q) - 1 >= floor(sqrt(q)) - 1,
        // so this suffices
        let u = (t - Natural::ONE).pow(4);
        // gcd(6, n) = 1 and the curve is nonsingular: gcd(4a^3 + 27b^2, n) = 1
        if u <= *n || n.divisible_by(THREE) {
            return false;
        }
        let a = (&self.a).mod_op(n);
        let b = (&self.b).mod_op(n);
        let x = (&self.x).mod_op(n);
        let y = (&self.y).mod_op(n);
        let t = (&a).mod_square(n).mod_mul(&a, n).mod_mul(FOUR, n);
        let u = (&b).mod_square(n).mod_mul(TWENTY_SEVEN, n);
        if !t.mod_add(u, n).coprime_with(n) {
            return false;
        }
        // P on the curve
        let t = (&x)
            .mod_square(n)
            .mod_add(&a, n)
            .mod_mul(&x, n)
            .mod_add(&b, n);
        let u = (&y).mod_square(n);
        if t != u {
            return false;
        }
        // a, x, and y are reduced, so the multiplications need not reduce them again; a failed
        // multiplication counts as giving the point at infinity
        let p = EcppPoint::from_affine(x, y);
        let mut acc = Natural::ONE;
        // m P = O and (m / q) P != O
        if !point_mul_reduced(&p, &self.m, &a, &mut acc, n).is_none_or(|r| r.is_zero())
            || point_mul_reduced(&p, &k, &a, &mut acc, n).is_none_or(|r| r.is_zero())
        {
            return false;
        }
        // all case distinctions were legitimate
        acc.coprime_with(n)
    }
}

fn is_word_prime(n: &Natural) -> bool {
    n.significant_bits() <= 64 && u64::exact_from(n).is_prime()
}

impl EcppCertificate {
    /// Checks whether an elliptic curve primality certificate proves that a [`Natural`] is prime.
    ///
    /// The certificate is valid for $n$ if its first step's $n$ is $n$, each later step's $n$ is
    /// the $q$ of the step before it, every step passes [`EcppStep::verify`], and the last step's
    /// $q$ is a prime less than $2^{64}$. A certificate with no steps is valid for $n$ exactly when
    /// $n$ is a prime less than $2^{64}$. A return value of `true` therefore proves that $n$ is
    /// prime; `false` means only that this certificate does not prove it.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(kn^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the largest number of significant bits
    /// of the steps' fields, and $k$ is the number of steps.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural::factorization::ecpp::EcppCertificate;
    ///
    /// let cert = EcppCertificate::from_str(
    ///     "[[68818587741598470525044465741, -523883721310323, 6684315, \
    ///     13709682389390639021241088892, [4543650699581189310901458620, \
    ///     26860660642440259748266721942]], [10295533310683143210451, 73075650452, 33011640000, \
    ///     10256222990173086439768, [7414706640842905801292, 3254141724577753829968]]]",
    /// )
    /// .unwrap();
    /// assert!(cert.verify(&Natural::from_str("68818587741598470525044465741").unwrap()));
    ///
    /// // The certificate is for a different number
    /// assert!(!cert.verify(&Natural::from_str("10295533310683143210451").unwrap()));
    ///
    /// // A certificate with no steps proves the primality of primes less than 2^64
    /// assert!(EcppCertificate::default().verify(&Natural::from(97u32)));
    /// assert!(!EcppCertificate::default().verify(&Natural::from(91u32)));
    /// ```
    ///
    /// This is equivalent to `ecpp_verify` from `ecpp/verify.c`, FLINT 3.7.0-dev.
    pub fn verify(&self, n: &Natural) -> bool {
        let steps = &self.steps;
        let Some(last) = steps.last() else {
            return is_word_prime(n);
        };
        steps[0].n == *n
            && steps.windows(2).all(|w| w[1].n == w[0].q)
            && steps.iter().all(EcppStep::verify)
            && is_word_prime(&last.q)
    }
}
