// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::gaussian_rational::arithmetic::content_and_primitive_part::{
    scale_up_ref, scale_up_val,
};
use crate::gaussian_rational::{ComparableGaussianRationalRef, GaussianRational};
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{CheckedRoot, DivRound, MulIPow, PowerOf2};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_base::rounding_modes::RoundingMode::Ceiling;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

// z = S / L with S a Gaussian integer and L the LCM of the denominators. The denominator D of a
// root w is determined by L alone: for an odd prime, its exponent in L(w^k) is k times its exponent
// in L(w), so the odd part of D is the kth root of the odd part of L; for 2, whose Gaussian prime 1
// + i has norm 2, the exponents relate through a ceiling, so the power of 2 in D is solved from
// that of L. Then D^k / L is a power of two, and D w is the Gaussian integer kth root of S times
// that power. Dividing by the positive D preserves the principal choice.
fn checked_root_helper(scaled: GaussianInteger, l: Natural, exp: u64) -> Option<GaussianRational> {
    // L is positive
    let l_twos = l.trailing_zeros().unwrap();
    let d_odd = (l >> l_twos).checked_root(exp)?;
    // If (1 + i)^e is the 1 + i part of the root's Gaussian denominator, L(w) has 2^ceil(e / 2) and
    // L(w^k) has 2^ceil(ke / 2), which pins e down to at most one value.
    let e = (l_twos << 1) / exp;
    if (exp * e).div_round(2, Ceiling).0 != l_twos {
        return None;
    }
    let d_twos = e.div_round(2, Ceiling).0;
    let shift = exp * d_twos - l_twos;
    let root = GaussianInteger {
        real: scaled.real << shift,
        imaginary: scaled.imaginary << shift,
    }
    .checked_root(exp)?;
    let d = Integer::from(d_odd << d_twos);
    Some(GaussianRational {
        real: Rational::from_integers_ref(&root.real, &d),
        imaginary: Rational::from_integers(root.imaginary, d),
    })
}

impl CheckedRoot<u64> for GaussianRational {
    type Output = Self;

    /// Returns the principal $n$th root of a [`GaussianRational`], or `None` if it is not a perfect
    /// $n$th power. The [`GaussianRational`] is taken by value.
    ///
    /// A nonzero Gaussian rational has either no $n$th roots or exactly $\gcd(n, 4)$ of them, the
    /// rotations of any one by the units $\zeta$ with $\zeta^n = 1$. The one returned is the
    /// principal root, whose argument lies in $(-\pi/g, \pi/g]$ for $g = \gcd(n, 4)$: the unique
    /// root for odd $n$, the root with positive real part (or zero real part and positive imaginary
    /// part) for $n \equiv 2 \pmod 4$, and the root in canonical unit form for $4 \mid n$.
    ///
    /// The root is found by clearing denominators: with $L$ the LCM of the two denominators, the
    /// denominator $D$ of a root is pinned down by $L$ alone (its odd part is the $n$th root of the
    /// odd part of $L$, and its power of 2 follows from that of $L$), and then $Dz$ scaled by the
    /// power of two $D^n / L$ is a Gaussian integer whose $n$th root, divided by $D$, is the
    /// answer.
    ///
    /// $$
    /// f(z, n) = \begin{cases}
    ///     \operatorname{Some}(\sqrt\[n\]{z}) & \text{if} \quad \sqrt\[n\]{z} \in
    ///     \mathbb{Q}(i), \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm^2)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum number of significant bits
    /// of the numerators and denominators of the real and imaginary parts of `self`, and $m$ is
    /// `exp`.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedRoot;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let root = |s, exp| {
    ///     GaussianRational::from_str(s)
    ///         .unwrap()
    ///         .checked_root(exp)
    ///         .map(|r| r.to_string())
    /// };
    /// // ((2+i)/5)^5 = (-38+41i)/3125
    /// assert_eq!(root("-38/3125+41i/3125", 5), Some("2/5+i/5".to_string()));
    /// // -1/4 = ((1+i)/2)^4, and (1+i)/2 is the principal root of the four
    /// assert_eq!(root("-1/4", 4), Some("1/2+i/2".to_string()));
    /// assert_eq!(root("-1/8", 3), Some("-1/2".to_string()));
    /// assert_eq!(root("1/2", 3), None);
    /// ```
    fn checked_root(self, exp: u64) -> Option<Self> {
        assert_ne!(exp, 0, "Cannot take the 0th root of a Gaussian rational");
        if self == 0u32 {
            return Some(Self::ZERO);
        }
        let (scaled, l) = scale_up_val(self);
        checked_root_helper(scaled, l, exp)
    }
}

impl CheckedRoot<u64> for &GaussianRational {
    type Output = GaussianRational;

    /// Returns the principal $n$th root of a [`GaussianRational`], or `None` if it is not a perfect
    /// $n$th power. The [`GaussianRational`] is taken by reference.
    ///
    /// A nonzero Gaussian rational has either no $n$th roots or exactly $\gcd(n, 4)$ of them, the
    /// rotations of any one by the units $\zeta$ with $\zeta^n = 1$. The one returned is the
    /// principal root, whose argument lies in $(-\pi/g, \pi/g]$ for $g = \gcd(n, 4)$: the unique
    /// root for odd $n$, the root with positive real part (or zero real part and positive imaginary
    /// part) for $n \equiv 2 \pmod 4$, and the root in canonical unit form for $4 \mid n$.
    ///
    /// The root is found by clearing denominators: with $L$ the LCM of the two denominators, the
    /// denominator $D$ of a root is pinned down by $L$ alone (its odd part is the $n$th root of the
    /// odd part of $L$, and its power of 2 follows from that of $L$), and then $Dz$ scaled by the
    /// power of two $D^n / L$ is a Gaussian integer whose $n$th root, divided by $D$, is the
    /// answer.
    ///
    /// $$
    /// f(z, n) = \begin{cases}
    ///     \operatorname{Some}(\sqrt\[n\]{z}) & \text{if} \quad \sqrt\[n\]{z} \in
    ///     \mathbb{Q}(i), \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm^2)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum number of significant bits
    /// of the numerators and denominators of the real and imaginary parts of `self`, and $m$ is
    /// `exp`.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedRoot;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let root = |s, exp| {
    ///     (&GaussianRational::from_str(s).unwrap())
    ///         .checked_root(exp)
    ///         .map(|r| r.to_string())
    /// };
    /// // ((2+i)/5)^5 = (-38+41i)/3125
    /// assert_eq!(root("-38/3125+41i/3125", 5), Some("2/5+i/5".to_string()));
    /// // -1/4 = ((1+i)/2)^4, and (1+i)/2 is the principal root of the four
    /// assert_eq!(root("-1/4", 4), Some("1/2+i/2".to_string()));
    /// assert_eq!(root("-1/8", 3), Some("-1/2".to_string()));
    /// assert_eq!(root("1/2", 3), None);
    /// ```
    fn checked_root(self, exp: u64) -> Option<GaussianRational> {
        assert_ne!(exp, 0, "Cannot take the 0th root of a Gaussian rational");
        if *self == 0u32 {
            return Some(GaussianRational::ZERO);
        }
        let (scaled, l) = scale_up_ref(self);
        checked_root_helper(scaled, l, exp)
    }
}

impl GaussianRational {
    /// Returns all the $n$th roots of a [`GaussianRational`]: none if it is not a perfect $n$th
    /// power, one if it is zero, and otherwise $\gcd(n, 4)$ of them, in the canonical order of
    /// [`ComparableGaussianRational`](crate::gaussian_rational::ComparableGaussianRational),
    /// lexicographic by real part and then imaginary part.
    ///
    /// The principal root is the one whose argument lies in $(-\pi/g, \pi/g]$ for $g = \gcd(n, 4)$;
    /// see [`CheckedRoot`](malachite_base::num::arithmetic::traits::CheckedRoot).
    ///
    /// $$
    /// f(z, n) = \\{ w \in \mathbb{Q}(i) : w^n = z \\}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm^2)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum number of significant bits
    /// of the numerators and denominators of the real and imaginary parts of `self`, and $m$ is
    /// `exp`.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let roots = |s, exp| {
    ///     GaussianRational::from_str(s)
    ///         .unwrap()
    ///         .checked_roots(exp)
    ///         .iter()
    ///         .map(ToString::to_string)
    ///         .collect::<Vec<_>>()
    /// };
    /// assert_eq!(
    ///     roots("-1/4", 4),
    ///     ["-1/2-i/2", "-1/2+i/2", "1/2-i/2", "1/2+i/2"]
    /// );
    /// assert_eq!(roots("-1/4", 2), ["-i/2", "i/2"]);
    /// assert_eq!(roots("-1/8", 3), ["-1/2"]);
    /// assert_eq!(roots("1/2", 3), Vec::<String>::new());
    /// assert_eq!(
    ///     GaussianRational::ZERO.checked_roots(7),
    ///     [GaussianRational::ZERO]
    /// );
    /// ```
    pub fn checked_roots(&self, exp: u64) -> Vec<Self> {
        let Some(principal) = self.checked_root(exp) else {
            return Vec::new();
        };
        if principal == 0u32 {
            return vec![principal];
        }
        // g = gcd(exp, 4) roots, each the previous one rotated by 2 pi / g, then sorted
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
        roots.sort_by(|a, b| {
            ComparableGaussianRationalRef(a).cmp(&ComparableGaussianRationalRef(b))
        });
        roots
    }
}
