// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::integer::Integer;
use crate::natural::Natural;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    CanonicalizeUnit, CheckedRoot, CheckedSqrt, ContentAndPrimitivePart, DivRem, Gcd, ModPowerOf2,
    MulIPow, MulIPowAssign, Parity, Pow, PowerOf2, Square,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::TrailingZeros;

fn norm(z: &GaussianInteger) -> Natural {
    z.real.unsigned_abs_ref().square() + z.imaginary.unsigned_abs_ref().square()
}

// The unique kth root of a nonzero z for odd k >= 3, if it exists.
//
// Strip the prime 1 + i, whose multiplicity must be a multiple of k, and split what remains into
// its content C and primitive part Q. If z = w^k, then C is the kth power of the content of the
// corresponding part of w, a Natural root check, and Q is a unit times P^k for the primitive part P
// of w. A primitive Gaussian integer with no factor 1 + i has, for each pair of conjugate split
// primes, only one of the two, so gcd(Q, N(P)) = gcd(u P^k, P conj(P)) is P up to a unit.
// Reassembling gives w up to a unit, and the unit is fixed by comparing the kth power with z: for
// odd k the map from units to their kth powers is a bijection.
fn odd_root(z: &GaussianInteger, k: u64) -> Option<GaussianInteger> {
    let (stripped, one_plus_i_exp) = z.remove_one_plus_i();
    let (w_one_plus_i_exp, r) = one_plus_i_exp.div_rem(k);
    if r != 0 {
        return None;
    }
    let (content, primitive) = stripped.content_and_primitive_part();
    let w_content = Integer::from(content.checked_root(k)?);
    let primitive_norm = norm(&primitive).checked_root(k)?;
    let w_primitive = (&primitive).gcd(GaussianInteger::from(Integer::from(primitive_norm)));
    let mut w = GaussianInteger {
        real: w_primitive.real * &w_content,
        imaginary: w_primitive.imaginary * w_content,
    };
    // multiply by (1 + i)^e = (2i)^(e / 2) (1 + i)^(e mod 2)
    let half = w_one_plus_i_exp >> 1;
    w <<= half;
    w.mul_i_pow_assign(half);
    if w_one_plus_i_exp.odd() {
        // (a + bi)(1 + i) = (a - b) + (a + b)i
        let sum = &w.real + &w.imaginary;
        w.real -= &w.imaginary;
        w.imaginary = sum;
    }
    // z = (i^j w)^k = i^(jk) w^k, so w^k = i^(-jk) z; k is odd, so k is its own inverse mod 4
    let w_pow = (&w).pow(k);
    let j = (0..4).find(|&j| (&w_pow).mul_i_pow(j) == *z)?;
    Some(w.mul_i_pow((j * k).mod_power_of_2(2)))
}

// The principal exp-th root, as described in the documentation of `checked_root`.
fn principal_root(z: &GaussianInteger, exp: u64) -> Option<GaussianInteger> {
    assert_ne!(exp, 0, "Cannot take the 0th root of a Gaussian integer");
    if *z == 0u32 {
        return Some(GaussianInteger::ZERO);
    } else if exp == 1 {
        return Some(z.clone());
    }
    let e = TrailingZeros::trailing_zeros(exp);
    let m = exp >> e;
    // The 2^e-th roots of the odd-part root are rotations of one another, but a rotation of a
    // square need not be a square (i is not a unit square), so following one chain of principal
    // square roots can dead-end where another succeeds: 16 = (1+i)^8, yet 16, 4, 2 stops at 2 while
    // 16, -4, 2i, 1+i reaches the root. So every square root is kept; the candidate set never
    // exceeds four elements.
    let mut candidates = vec![if m == 1 { z.clone() } else { odd_root(z, m)? }];
    for _ in 0..e {
        candidates = candidates
            .into_iter()
            .filter_map(CheckedSqrt::checked_sqrt)
            .flat_map(|w| [-&w, w])
            .collect();
        if candidates.is_empty() {
            return None;
        }
    }
    let candidate = candidates.pop()?;
    Some(match e {
        0 => candidate,
        1 => {
            if (&candidate.real, &candidate.imaginary) > (&Integer::ZERO, &Integer::ZERO) {
                candidate
            } else {
                -candidate
            }
        }
        _ => candidate.canonicalize_unit(),
    })
}

impl CheckedRoot<u64> for GaussianInteger {
    type Output = Self;

    /// Returns the principal $n$th root of a [`GaussianInteger`], or `None` if it is not a perfect
    /// $n$th power. The [`GaussianInteger`] is taken by value.
    ///
    /// A nonzero Gaussian integer has either no $n$th roots or exactly $\gcd(n, 4)$ of them: if $w$
    /// is one, the others are $w\zeta$ for the units $\zeta$ with $\zeta^n = 1$. The one returned
    /// is the principal root, whose argument lies in $(-\pi/g, \pi/g]$ for $g = \gcd(n, 4)$: the
    /// unique root for odd $n$, the root with positive real part (or zero real part and positive
    /// imaginary part) for $n \equiv 2 \pmod 4$, and the root in canonical unit form for $4 \mid
    /// n$.
    ///
    /// Writing $n = 2^e m$ with $m$ odd, the unique $m$th root is found exactly through the norm:
    /// with $N = N(z)^{1/m}$ and $d = \gcd(z, N)$, the quotient $N d / \bar{d}$ is the square of
    /// the root up to a unit, and the unit is fixed by raising to the $m$th power. Square roots are
    /// then taken $e$ times over the candidate set, which never exceeds four roots.
    ///
    /// $$
    /// f(z, n) = \begin{cases}
    ///     \operatorname{Some}(\sqrt\[n\]{z}) & \text{if} \quad \sqrt\[n\]{z} \in \Z[i], \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self`.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedRoot;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let root = |s, exp| {
    ///     GaussianInteger::from_str(s)
    ///         .unwrap()
    ///         .checked_root(exp)
    ///         .map(|r| r.to_string())
    /// };
    /// // (2+i)^5 = -38+41i
    /// assert_eq!(root("-38+41i", 5), Some("2+i".to_string()));
    /// // -4 = (1+i)^4, and 1+i is the principal root of the four
    /// assert_eq!(root("-4", 4), Some("1+i".to_string()));
    /// // the unique cube root of -8 is -2
    /// assert_eq!(root("-8", 3), Some("-2".to_string()));
    /// assert_eq!(root("3+4i", 3), None);
    /// ```
    #[inline]
    fn checked_root(self, exp: u64) -> Option<Self> {
        principal_root(&self, exp)
    }
}

impl CheckedRoot<u64> for &GaussianInteger {
    type Output = GaussianInteger;

    /// Returns the principal $n$th root of a [`GaussianInteger`], or `None` if it is not a perfect
    /// $n$th power. The [`GaussianInteger`] is taken by reference.
    ///
    /// A nonzero Gaussian integer has either no $n$th roots or exactly $\gcd(n, 4)$ of them: if $w$
    /// is one, the others are $w\zeta$ for the units $\zeta$ with $\zeta^n = 1$. The one returned
    /// is the principal root, whose argument lies in $(-\pi/g, \pi/g]$ for $g = \gcd(n, 4)$: the
    /// unique root for odd $n$, the root with positive real part (or zero real part and positive
    /// imaginary part) for $n \equiv 2 \pmod 4$, and the root in canonical unit form for $4 \mid
    /// n$.
    ///
    /// Writing $n = 2^e m$ with $m$ odd, the unique $m$th root is found exactly through the norm:
    /// with $N = N(z)^{1/m}$ and $d = \gcd(z, N)$, the quotient $N d / \bar{d}$ is the square of
    /// the root up to a unit, and the unit is fixed by raising to the $m$th power. Square roots are
    /// then taken $e$ times over the candidate set, which never exceeds four roots.
    ///
    /// $$
    /// f(z, n) = \begin{cases}
    ///     \operatorname{Some}(\sqrt\[n\]{z}) & \text{if} \quad \sqrt\[n\]{z} \in \Z[i], \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self`.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedRoot;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let root = |s, exp| {
    ///     (&GaussianInteger::from_str(s).unwrap())
    ///         .checked_root(exp)
    ///         .map(|r| r.to_string())
    /// };
    /// // (2+i)^5 = -38+41i
    /// assert_eq!(root("-38+41i", 5), Some("2+i".to_string()));
    /// // -4 = (1+i)^4, and 1+i is the principal root of the four
    /// assert_eq!(root("-4", 4), Some("1+i".to_string()));
    /// // the unique cube root of -8 is -2
    /// assert_eq!(root("-8", 3), Some("-2".to_string()));
    /// assert_eq!(root("3+4i", 3), None);
    /// ```
    #[inline]
    fn checked_root(self, exp: u64) -> Option<GaussianInteger> {
        principal_root(self, exp)
    }
}

impl GaussianInteger {
    /// Returns all the $n$th roots of a [`GaussianInteger`]: none if it is not a perfect $n$th
    /// power, one if it is zero, and otherwise $\gcd(n, 4)$ of them, the principal root first and
    /// then its successive rotations by $i$.
    ///
    /// The principal root is the one whose argument lies in $(-\pi/g, \pi/g]$ for $g = \gcd(n, 4)$;
    /// see [`CheckedRoot`](malachite_base::num::arithmetic::traits::CheckedRoot).
    ///
    /// $$
    /// f(z, n) = \\{ w \in \Z[i] : w^n = z \\}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self`.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let roots = |s, exp| {
    ///     GaussianInteger::from_str(s)
    ///         .unwrap()
    ///         .checked_roots(exp)
    ///         .iter()
    ///         .map(ToString::to_string)
    ///         .collect::<Vec<_>>()
    /// };
    /// assert_eq!(roots("-4", 4), ["1+i", "-1+i", "-1-i", "1-i"]);
    /// assert_eq!(roots("-4", 2), ["2i", "-2i"]);
    /// assert_eq!(roots("-8", 3), ["-2"]);
    /// assert_eq!(roots("3+4i", 3), Vec::<String>::new());
    /// assert_eq!(
    ///     GaussianInteger::ZERO.checked_roots(7),
    ///     [GaussianInteger::ZERO]
    /// );
    /// ```
    pub fn checked_roots(&self, exp: u64) -> Vec<Self> {
        let Some(principal) = principal_root(self, exp) else {
            return Vec::new();
        };
        if principal == 0u32 {
            return vec![principal];
        }
        // g = gcd(exp, 4) roots, each the previous one rotated by 2 pi / g
        let g = u64::power_of_2(TrailingZeros::trailing_zeros(exp).min(2));
        let step = 4 / g;
        let mut roots = Vec::with_capacity(usize::exact_from(g));
        let mut root = principal;
        for _ in 1..g {
            let next = (&root).mul_i_pow(step);
            roots.push(root);
            root = next;
        }
        roots.push(root);
        roots
    }
}
