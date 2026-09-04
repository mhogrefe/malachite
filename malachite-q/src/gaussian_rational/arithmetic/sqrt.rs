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
use malachite_base::num::arithmetic::traits::CheckedSqrt;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

// z = S / L is a square exactly when S L is a square in the Gaussian integers, since a Gaussian
// integer that is a square of a Gaussian rational is a square of a Gaussian integer; the root
// sqrt(S L) / L is principal because L is positive.
fn checked_sqrt_helper(scaled: GaussianInteger, l: Natural) -> Option<GaussianRational> {
    let l = Integer::from(l);
    let root = GaussianInteger {
        real: scaled.real * &l,
        imaginary: scaled.imaginary * &l,
    }
    .checked_sqrt()?;
    Some(GaussianRational {
        real: Rational::from_integers_ref(&root.real, &l),
        imaginary: Rational::from_integers(root.imaginary, l),
    })
}

impl CheckedSqrt for GaussianRational {
    type Output = Self;

    /// Returns the principal square root of a [`GaussianRational`], or `None` if it is not a
    /// perfect square. The [`GaussianRational`] is taken by value.
    ///
    /// A nonzero Gaussian rational that is a perfect square has two square roots, each the negative
    /// of the other; the one returned is the principal root, whose real part is positive or, if it
    /// is zero, whose imaginary part is non-negative. That is the root whose argument lies in
    /// $(-\pi/2, \pi/2]$.
    ///
    /// The root is found by clearing denominators: with $L$ the LCM of the two denominators and $S
    /// = Lz$ a Gaussian integer, $z$ is a square in $\mathbb{Q}(i)$ exactly when $SL$ is a square
    /// in $\mathbb{Z}[i]$, and then $\sqrt{z} = \sqrt{SL} / L$.
    ///
    /// $$
    /// f(z) = \begin{cases}
    ///     \operatorname{Some}(\sqrt{z}) & \text{if} \quad \sqrt{z} \in \mathbb{Q}(i), \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the numerators and denominators of the real and imaginary parts of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedSqrt;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let root = |s| {
    ///     GaussianRational::from_str(s)
    ///         .unwrap()
    ///         .checked_sqrt()
    ///         .map(|r| r.to_string())
    /// };
    /// // (1+i/2)^2 = 3/4+i
    /// assert_eq!(root("3/4+i"), Some("1+i/2".to_string()));
    /// // -1/4 = (i/2)^2, and i/2 is the principal root
    /// assert_eq!(root("-1/4"), Some("i/2".to_string()));
    /// assert_eq!(root("1/2"), None);
    /// ```
    fn checked_sqrt(self) -> Option<Self> {
        if self == 0u32 {
            return Some(Self::ZERO);
        }
        let (scaled, l) = scale_up_val(self);
        checked_sqrt_helper(scaled, l)
    }
}

impl CheckedSqrt for &GaussianRational {
    type Output = GaussianRational;

    /// Returns the principal square root of a [`GaussianRational`], or `None` if it is not a
    /// perfect square. The [`GaussianRational`] is taken by reference.
    ///
    /// A nonzero Gaussian rational that is a perfect square has two square roots, each the negative
    /// of the other; the one returned is the principal root, whose real part is positive or, if it
    /// is zero, whose imaginary part is non-negative. That is the root whose argument lies in
    /// $(-\pi/2, \pi/2]$.
    ///
    /// The root is found by clearing denominators: with $L$ the LCM of the two denominators and $S
    /// = Lz$ a Gaussian integer, $z$ is a square in $\mathbb{Q}(i)$ exactly when $SL$ is a square
    /// in $\mathbb{Z}[i]$, and then $\sqrt{z} = \sqrt{SL} / L$.
    ///
    /// $$
    /// f(z) = \begin{cases}
    ///     \operatorname{Some}(\sqrt{z}) & \text{if} \quad \sqrt{z} \in \mathbb{Q}(i), \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the numerators and denominators of the real and imaginary parts of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedSqrt;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let root = |s| {
    ///     (&GaussianRational::from_str(s).unwrap())
    ///         .checked_sqrt()
    ///         .map(|r| r.to_string())
    /// };
    /// // (1+i/2)^2 = 3/4+i
    /// assert_eq!(root("3/4+i"), Some("1+i/2".to_string()));
    /// // -1/4 = (i/2)^2, and i/2 is the principal root
    /// assert_eq!(root("-1/4"), Some("i/2".to_string()));
    /// assert_eq!(root("1/2"), None);
    /// ```
    fn checked_sqrt(self) -> Option<GaussianRational> {
        if *self == 0u32 {
            return Some(GaussianRational::ZERO);
        }
        let (scaled, l) = scale_up_ref(self);
        checked_sqrt_helper(scaled, l)
    }
}

impl GaussianRational {
    /// Returns all the square roots of a [`GaussianRational`]: none if it is not a perfect square,
    /// one if it is zero, and otherwise the principal root and its negative, in the canonical order
    /// of [`ComparableGaussianRational`](crate::gaussian_rational::ComparableGaussianRational),
    /// lexicographic by real part and then imaginary part.
    ///
    /// The principal root is the one with positive real part or, if that is zero, with non-negative
    /// imaginary part; see [`CheckedSqrt`](malachite_base::num::arithmetic::traits::CheckedSqrt).
    ///
    /// $$
    /// f(z) = \\{ w \in \mathbb{Q}(i) : w^2 = z \\}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the numerators and denominators of the real and imaginary parts of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let roots = |s| {
    ///     GaussianRational::from_str(s)
    ///         .unwrap()
    ///         .checked_sqrts()
    ///         .iter()
    ///         .map(ToString::to_string)
    ///         .collect::<Vec<_>>()
    /// };
    /// assert_eq!(roots("3/4+i"), ["-1-i/2", "1+i/2"]);
    /// assert_eq!(roots("-1/4"), ["-i/2", "i/2"]);
    /// assert_eq!(roots("1/2"), Vec::<String>::new());
    /// assert_eq!(
    ///     GaussianRational::ZERO.checked_sqrts(),
    ///     [GaussianRational::ZERO]
    /// );
    /// ```
    pub fn checked_sqrts(&self) -> Vec<Self> {
        match self.checked_sqrt() {
            None => Vec::new(),
            Some(root) if root == 0u32 => vec![root],
            Some(root) => {
                let neg_root = -&root;
                let mut roots = vec![root, neg_root];
                roots.sort_by(|a, b| {
                    ComparableGaussianRationalRef(a).cmp(&ComparableGaussianRationalRef(b))
                });
                roots
            }
        }
    }
}
