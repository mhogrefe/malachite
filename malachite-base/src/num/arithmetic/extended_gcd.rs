// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009, 2016 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ExtendedGcd;
use crate::num::arithmetic::traits::UnsignedAbs;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;
use crate::rounding_modes::RoundingMode::*;
use core::mem::swap;

fn extended_gcd_signed<
    U: ExtendedGcd<Cofactor = S> + PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>(
    a: S,
    b: S,
) -> (U, S, S) {
    let (gcd, mut x, mut y) = a.unsigned_abs().extended_gcd(b.unsigned_abs());
    if a < S::ZERO {
        x = x.checked_neg().unwrap();
    }
    if b < S::ZERO {
        y = y.checked_neg().unwrap();
    }
    (gcd, x, y)
}

// This is equivalent to `n_xgcd` from `ulong_extras/xgcd.c`, FLINT 2.7.1, with an adjustment to
// find the minimal cofactors.
pub_test! {extended_gcd_unsigned_binary<
    U: WrappingFrom<S> + PrimitiveUnsigned,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    mut a: U,
    mut b: U,
) -> (U, S, S) {
    if a == U::ZERO && b == U::ZERO {
        return (U::ZERO, S::ZERO, S::ZERO);
    } else if a == b || a == U::ZERO {
        return (b, S::ZERO, S::ONE);
    } else if b == U::ZERO {
        return (a, S::ONE, S::ZERO);
    }
    let mut swapped = false;
    if a < b {
        swap(&mut a, &mut b);
        swapped = true;
    }
    let mut u1 = S::ONE;
    let mut v2 = S::ONE;
    let mut u2 = S::ZERO;
    let mut v1 = S::ZERO;
    let mut u3 = a;
    let mut v3 = b;
    let mut d;
    let mut t2;
    let mut t1;
    if (a & b).get_highest_bit() {
        d = u3 - v3;
        t2 = v2;
        t1 = u2;
        u2 = u1 - u2;
        u1 = t1;
        u3 = v3;
        v2 = v1 - v2;
        v1 = t2;
        v3 = d;
    }
    while v3.get_bit(U::WIDTH - 2) {
        d = u3 - v3;
        if d < v3 {
            // quot = 1
            t2 = v2;
            t1 = u2;
            u2 = u1 - u2;
            u1 = t1;
            u3 = v3;
            v2 = v1 - v2;
            v1 = t2;
            v3 = d;
        } else if d < (v3 << 1) {
            // quot = 2
            t1 = u2;
            u2 = u1 - (u2 << 1);
            u1 = t1;
            u3 = v3;
            t2 = v2;
            v2 = v1 - (v2 << 1);
            v1 = t2;
            v3 = d - u3;
        } else {
            // quot = 3
            t1 = u2;
            u2 = u1 - S::wrapping_from(3) * u2;
            u1 = t1;
            u3 = v3;
            t2 = v2;
            v2 = v1 - S::wrapping_from(3) * v2;
            v1 = t2;
            v3 = d - (u3 << 1);
        }
    }
    while v3 != U::ZERO {
        d = u3 - v3;
        // overflow not possible, top 2 bits of v3 not set
        if u3 < (v3 << 2) {
            if d < v3 {
                // quot = 1
                t2 = v2;
                t1 = u2;
                u2 = u1 - u2;
                u1 = t1;
                u3 = v3;
                v2 = v1 - v2;
                v1 = t2;
                v3 = d;
            } else if d < (v3 << 1) {
                // quot = 2
                t1 = u2;
                u2 = u1.wrapping_sub(u2 << 1);
                u1 = t1;
                u3 = v3;
                t2 = v2;
                v2 = v1.wrapping_sub(v2 << 1);
                v1 = t2;
                v3 = d - u3;
            } else {
                // quot = 3
                t1 = u2;
                u2 = u1.wrapping_sub(S::wrapping_from(3).wrapping_mul(u2));
                u1 = t1;
                u3 = v3;
                t2 = v2;
                v2 = v1.wrapping_sub(S::wrapping_from(3).wrapping_mul(v2));
                v1 = t2;
                v3 = d.wrapping_sub(u3 << 1);
            }
        } else {
            let (quot, rem) = u3.div_rem(v3);
            t1 = u2;
            u2 = u1.wrapping_sub(S::wrapping_from(quot).wrapping_mul(u2));
            u1 = t1;
            u3 = v3;
            t2 = v2;
            v2 = v1.wrapping_sub(S::wrapping_from(quot).wrapping_mul(v2));
            v1 = t2;
            v3 = rem;
        }
    }
    // Remarkably, |u1| < x/2, thus comparison with 0 is valid
    if u1 <= S::ZERO {
        u1.wrapping_add_assign(S::wrapping_from(b));
        v1.wrapping_sub_assign(S::wrapping_from(a));
    }
    // The cofactors at this point are not necessarily minimal, so we may need to adjust.
    let gcd = u3;
    let mut x = U::wrapping_from(u1);
    let mut y = U::wrapping_from(v1);
    let two_limit_a = a / gcd;
    let two_limit_b = b / gcd;
    let limit_b = two_limit_b >> 1;
    if x > limit_b {
        let k = (x - limit_b).div_round(two_limit_b, Ceiling).0;
        x.wrapping_sub_assign(two_limit_b.wrapping_mul(k));
        y.wrapping_add_assign(two_limit_a.wrapping_mul(k));
    }
    if swapped {
        swap(&mut x, &mut y);
    }
    (gcd, S::wrapping_from(x), S::wrapping_from(y))
}}

macro_rules! impl_extended_gcd {
    ($u:ident, $s:ident) => {
        impl ExtendedGcd<$u> for $u {
            type Gcd = $u;
            type Cofactor = $s;

            /// Computes the GCD (greatest common divisor) of two numbers $a$ and $b$, and also the
            /// coefficients $x$ and $y$ in Bézout's identity $ax+by=\gcd(a,b)$.
            ///
            /// The are infinitely many $x$, $y$ that satisfy the identity for any $a$, $b$, so the
            /// full specification is more detailed:
            ///
            /// - $f(0, 0) = (0, 0, 0)$.
            /// - $f(a, ak) = (a, 1, 0)$ if $a > 0$ and $k \neq 1$.
            /// - $f(bk, b) = (b, 0, 1)$ if $b > 0$.
            /// - $f(a, b) = (g, x, y)$ if $a \neq 0$ and $b \neq 0$ and $\gcd(a, b) \neq \min(a,
            ///   b)$, where $g = \gcd(a, b) \geq 0$, $ax + by = g$, $x \leq \lfloor b/g \rfloor$,
            ///   and $y \leq \lfloor a/g \rfloor$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n^2)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(self.significant_bits(), other.significant_bits())`.
            ///
            /// # Examples
            /// See [here](super::extended_gcd#extended_gcd).
            #[inline]
            fn extended_gcd(self, other: $u) -> ($u, $s, $s) {
                extended_gcd_unsigned_binary(self, other)
            }
        }

        impl ExtendedGcd<$s> for $s {
            type Gcd = $u;
            type Cofactor = $s;

            /// Computes the GCD (greatest common divisor) of two numbers $a$ and $b$, and also the
            /// coefficients $x$ and $y$ in Bézout's identity $ax+by=\gcd(a,b)$.
            ///
            /// The are infinitely many $x$, $y$ that satisfy the identity for any $a$, $b$, so the
            /// full specification is more detailed:
            ///
            /// - $f(0, 0) = (0, 0, 0)$.
            /// - $f(a, ak) = (a, 1, 0)$ if $a > 0$ and $k \neq 1$.
            /// - $f(a, ak) = (-a, -1, 0)$ if $a < 0$ and $k \neq 1$.
            /// - $f(bk, b) = (b, 0, 1)$ if $b > 0$.
            /// - $f(bk, b) = (-b, 0, -1)$ if $b < 0$.
            /// - $f(a, b) = (g, x, y)$ if $a \neq 0$ and $b \neq 0$ and $\gcd(a, b) \neq \min(|a|,
            ///   |b|)$, where $g = \gcd(a, b) \geq 0$, $ax + by = g$, $x \leq \lfloor b/g \rfloor$,
            ///   and $y \leq \lfloor a/g \rfloor$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n^2)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(self.significant_bits(), other.significant_bits())`.
            ///
            /// # Examples
            /// See [here](super::extended_gcd#extended_gcd).
            #[inline]
            fn extended_gcd(self, other: $s) -> ($u, $s, $s) {
                extended_gcd_signed(self, other)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_extended_gcd);
