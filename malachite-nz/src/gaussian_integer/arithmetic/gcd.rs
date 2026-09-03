// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2022 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::gaussian_integer::arithmetic::div_rem::div_rem_approx;
use crate::integer::Integer;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{CanonicalizeUnit, Gcd, GcdAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

// When all four parts have at most this many bits, the whole GCD runs in double precision: the
// products in the quotient estimate stay within a factor of 2 of the inputs, so every remainder is
// computed exactly, and the estimate's rounding error is far too small to stall the descent.
const DOUBLE_GCD_BITS: u64 = 50;

fn fits_double(x: &GaussianInteger) -> bool {
    x.real.significant_bits() <= DOUBLE_GCD_BITS
        && x.imaginary.significant_bits() <= DOUBLE_GCD_BITS
}

// The Euclidean algorithm in double precision, from `_fmpzi_gcd_dddd`: the quotient is the nearest
// Gaussian integer to x / y, evaluated as floor(x conj(y) / N(y) + (1 + i) / 2), and the remainder
// x - qy is exact.
fn gcd_double(mut a: f64, mut b: f64, mut c: f64, mut d: f64) -> GaussianInteger {
    while c != 0.0 || d != 0.0 {
        let t = a * c + b * d;
        let u = b * c - a * d;
        let v = c * c + d * d;
        let w = 0.5 / v;
        let qa = ((2.0 * t + v) * w).floor();
        let qb = ((2.0 * u + v) * w).floor();
        let t = a - (qa * c - qb * d);
        let u = b - (qb * c + qa * d);
        a = c;
        b = d;
        c = t;
        d = u;
    }
    GaussianInteger {
        real: Integer::exact_from(a),
        imaginary: Integer::exact_from(b),
    }
    .canonicalize_unit()
}

// FLINT's `fmpzi_gcd` without its lattice tier: the double-precision kernel once the parts are
// small, and the Euclidean algorithm over approximate divisions until then. FLINT switches to
// `fmpzi_gcd_shortest`, a lattice method, when both operands exceed 30,000 bits; that method is not
// ported yet, so such operands take the Euclidean path.
fn gcd_helper(mut x: GaussianInteger, mut y: GaussianInteger) -> GaussianInteger {
    if x == 0u32 {
        return y.canonicalize_unit();
    } else if y == 0u32 {
        return x.canonicalize_unit();
    }
    loop {
        if fits_double(&x) && fits_double(&y) {
            return gcd_double(
                f64::exact_from(&x.real),
                f64::exact_from(&x.imaginary),
                f64::exact_from(&y.real),
                f64::exact_from(&y.imaginary),
            );
        }
        let r = div_rem_approx(&x, &y).1;
        x = y;
        y = r;
        if y == 0u32 {
            return x.canonicalize_unit();
        }
    }
}

impl Gcd<Self> for GaussianInteger {
    type Output = Self;

    /// Computes the GCD (greatest common divisor) of two [`GaussianInteger`]s, taking both by
    /// value.
    ///
    /// The Gaussian integers are a Euclidean domain, so any two have a GCD, defined up to
    /// multiplication by one of the four units $\pm 1, \pm i$. The one returned is in canonical
    /// unit form (see
    /// [`CanonicalizeUnit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit)): its real
    /// part is positive and its imaginary part lies in $(-\text{real}, \text{real}]$, unless it is
    /// zero. The GCD of 0 and $x$ is the canonical form of $x$; in particular $\gcd(0, 0) = 0$,
    /// which makes sense if we interpret "greatest" to mean "greatest by the divisibility order".
    ///
    /// $$
    /// f(x, y) = \gcd(x, y).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // 3+4i = (2+i)^2 and 5 = (2+i)(2-i)
    /// let x = GaussianInteger::from_str("3+4i").unwrap();
    /// let y = GaussianInteger::from(5);
    /// assert_eq!((x.gcd(y)).to_string(), "2+i");
    /// ```
    #[inline]
    fn gcd(self, other: Self) -> Self {
        gcd_helper(self, other)
    }
}

impl Gcd<&Self> for GaussianInteger {
    type Output = Self;

    /// Computes the GCD (greatest common divisor) of two [`GaussianInteger`]s, taking the first by
    /// value and the second by reference.
    ///
    /// The Gaussian integers are a Euclidean domain, so any two have a GCD, defined up to
    /// multiplication by one of the four units $\pm 1, \pm i$. The one returned is in canonical
    /// unit form (see
    /// [`CanonicalizeUnit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit)): its real
    /// part is positive and its imaginary part lies in $(-\text{real}, \text{real}]$, unless it is
    /// zero. The GCD of 0 and $x$ is the canonical form of $x$; in particular $\gcd(0, 0) = 0$,
    /// which makes sense if we interpret "greatest" to mean "greatest by the divisibility order".
    ///
    /// $$
    /// f(x, y) = \gcd(x, y).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // 3+4i = (2+i)^2 and 5 = (2+i)(2-i)
    /// let x = GaussianInteger::from_str("3+4i").unwrap();
    /// let y = GaussianInteger::from(5);
    /// assert_eq!((x.gcd(&y)).to_string(), "2+i");
    /// ```
    #[inline]
    fn gcd(self, other: &Self) -> Self {
        gcd_helper(self, other.clone())
    }
}

impl Gcd<GaussianInteger> for &GaussianInteger {
    type Output = GaussianInteger;

    /// Computes the GCD (greatest common divisor) of two [`GaussianInteger`]s, taking the first by
    /// reference and the second by value.
    ///
    /// The Gaussian integers are a Euclidean domain, so any two have a GCD, defined up to
    /// multiplication by one of the four units $\pm 1, \pm i$. The one returned is in canonical
    /// unit form (see
    /// [`CanonicalizeUnit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit)): its real
    /// part is positive and its imaginary part lies in $(-\text{real}, \text{real}]$, unless it is
    /// zero. The GCD of 0 and $x$ is the canonical form of $x$; in particular $\gcd(0, 0) = 0$,
    /// which makes sense if we interpret "greatest" to mean "greatest by the divisibility order".
    ///
    /// $$
    /// f(x, y) = \gcd(x, y).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // 3+4i = (2+i)^2 and 5 = (2+i)(2-i)
    /// let x = GaussianInteger::from_str("3+4i").unwrap();
    /// let y = GaussianInteger::from(5);
    /// assert_eq!(((&x).gcd(y)).to_string(), "2+i");
    /// ```
    #[inline]
    fn gcd(self, other: GaussianInteger) -> GaussianInteger {
        gcd_helper(self.clone(), other)
    }
}

impl Gcd<&GaussianInteger> for &GaussianInteger {
    type Output = GaussianInteger;

    /// Computes the GCD (greatest common divisor) of two [`GaussianInteger`]s, taking both by
    /// reference.
    ///
    /// The Gaussian integers are a Euclidean domain, so any two have a GCD, defined up to
    /// multiplication by one of the four units $\pm 1, \pm i$. The one returned is in canonical
    /// unit form (see
    /// [`CanonicalizeUnit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit)): its real
    /// part is positive and its imaginary part lies in $(-\text{real}, \text{real}]$, unless it is
    /// zero. The GCD of 0 and $x$ is the canonical form of $x$; in particular $\gcd(0, 0) = 0$,
    /// which makes sense if we interpret "greatest" to mean "greatest by the divisibility order".
    ///
    /// $$
    /// f(x, y) = \gcd(x, y).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // 3+4i = (2+i)^2 and 5 = (2+i)(2-i)
    /// let x = GaussianInteger::from_str("3+4i").unwrap();
    /// let y = GaussianInteger::from(5);
    /// assert_eq!(((&x).gcd(&y)).to_string(), "2+i");
    /// ```
    #[inline]
    fn gcd(self, other: &GaussianInteger) -> GaussianInteger {
        gcd_helper(self.clone(), other.clone())
    }
}

impl GcdAssign<Self> for GaussianInteger {
    /// Replaces a [`GaussianInteger`] by its GCD (greatest common divisor) with another
    /// [`GaussianInteger`], taking the [`GaussianInteger`] on the right-hand side by value.
    ///
    /// The Gaussian integers are a Euclidean domain, so any two have a GCD, defined up to
    /// multiplication by one of the four units $\pm 1, \pm i$. The one returned is in canonical
    /// unit form (see
    /// [`CanonicalizeUnit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit)): its real
    /// part is positive and its imaginary part lies in $(-\text{real}, \text{real}]$, unless it is
    /// zero. The GCD of 0 and $x$ is the canonical form of $x$; in particular $\gcd(0, 0) = 0$,
    /// which makes sense if we interpret "greatest" to mean "greatest by the divisibility order".
    ///
    /// $$
    /// x \gets \gcd(x, y).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::GcdAssign;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // 3+4i = (2+i)^2 and 5 = (2+i)(2-i)
    /// let mut x = GaussianInteger::from_str("3+4i").unwrap();
    /// x.gcd_assign(GaussianInteger::from(5));
    /// assert_eq!(x.to_string(), "2+i");
    /// ```
    #[inline]
    fn gcd_assign(&mut self, other: Self) {
        *self = gcd_helper(take(self), other);
    }
}

impl GcdAssign<&Self> for GaussianInteger {
    /// Replaces a [`GaussianInteger`] by its GCD (greatest common divisor) with another
    /// [`GaussianInteger`], taking the [`GaussianInteger`] on the right-hand side by reference.
    ///
    /// The Gaussian integers are a Euclidean domain, so any two have a GCD, defined up to
    /// multiplication by one of the four units $\pm 1, \pm i$. The one returned is in canonical
    /// unit form (see
    /// [`CanonicalizeUnit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit)): its real
    /// part is positive and its imaginary part lies in $(-\text{real}, \text{real}]$, unless it is
    /// zero. The GCD of 0 and $x$ is the canonical form of $x$; in particular $\gcd(0, 0) = 0$,
    /// which makes sense if we interpret "greatest" to mean "greatest by the divisibility order".
    ///
    /// $$
    /// x \gets \gcd(x, y).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::GcdAssign;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // 3+4i = (2+i)^2 and 5 = (2+i)(2-i)
    /// let mut x = GaussianInteger::from_str("3+4i").unwrap();
    /// x.gcd_assign(&GaussianInteger::from(5));
    /// assert_eq!(x.to_string(), "2+i");
    /// ```
    #[inline]
    fn gcd_assign(&mut self, other: &Self) {
        *self = gcd_helper(take(self), other.clone());
    }
}
