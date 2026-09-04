// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::{ComparableGaussianIntegerRef, GaussianInteger};
use crate::integer::Integer;
use crate::natural::Natural;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{CheckedSqrt, Parity, Square, UnsignedAbs};
use malachite_base::num::basic::traits::Zero;

// If a + bi = (x + yi)^2 then, with N = sqrt(a^2 + b^2), x^2 = (N + a) / 2 and y^2 = (N - a) / 2,
// and 2xy = b fixes the sign of y once x is taken positive. A nonzero square root is normalized to
// the principal one, with positive real part or, failing that, non-negative imaginary part.
fn checked_sqrt_helper(z: &GaussianInteger) -> Option<GaussianInteger> {
    let a = &z.real;
    let b = &z.imaginary;
    if *b == 0u32 {
        let root = Integer::from(a.unsigned_abs_ref().checked_sqrt()?);
        return Some(if *a >= 0u32 {
            GaussianInteger::from(root)
        } else {
            // sqrt(-n) = sqrt(n) i
            GaussianInteger {
                real: Integer::ZERO,
                imaginary: root,
            }
        });
    } else if *a == 0u32 {
        // (x + xi)^2 = 2x^2 i and (x - xi)^2 = -2x^2 i
        if b.odd() {
            return None;
        }
        let root = Integer::from((b.unsigned_abs_ref() >> 1u64).checked_sqrt()?);
        return Some(GaussianInteger {
            imaginary: if *b > 0u32 { root.clone() } else { -&root },
            real: root,
        });
    }
    let norm: Natural = a.unsigned_abs_ref().square() + b.unsigned_abs_ref().square();
    let n = Integer::from(norm.checked_sqrt()?);
    let x_squared = &n + a;
    if x_squared.odd() {
        return None;
    }
    let x = (x_squared >> 1u64).unsigned_abs().checked_sqrt()?;
    let y = ((n - a) >> 1u64).unsigned_abs().checked_sqrt()?;
    Some(GaussianInteger {
        real: Integer::from(x),
        imaginary: Integer::from_sign_and_abs(*b > 0u32, y),
    })
}

impl CheckedSqrt for GaussianInteger {
    type Output = Self;

    /// Returns the principal square root of a [`GaussianInteger`], or `None` if it is not a perfect
    /// square. The [`GaussianInteger`] is taken by value.
    ///
    /// A nonzero Gaussian integer that is a perfect square has two square roots, each the negative
    /// of the other; the one returned is the principal root, whose real part is positive or, if it
    /// is zero, whose imaginary part is non-negative. That is the root whose argument lies in
    /// $(-\pi/2, \pi/2]$.
    ///
    /// The root is found through the norm: if $a + bi = (x + yi)^2$ then $N = \sqrt{a^2 + b^2}$ is
    /// an integer, $x^2 = (N + a) / 2$, $y^2 = (N - a) / 2$, and $2xy = b$ fixes the sign of $y$
    /// relative to that of $x$.
    ///
    /// $$
    /// f(z) = \begin{cases}
    ///     \operatorname{Some}(\sqrt{z}) & \text{if} \quad \sqrt{z} \in \Z[i], \\\\
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
    /// bits of the real and imaginary parts of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedSqrt;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (2+i)^2 = 3+4i
    /// assert_eq!(
    ///     GaussianInteger::from_str("3+4i")
    ///         .unwrap()
    ///         .checked_sqrt()
    ///         .unwrap()
    ///         .to_string(),
    ///     "2+i"
    /// );
    /// // (1-i)^2 = -2i
    /// assert_eq!(
    ///     GaussianInteger::from_str("-2i")
    ///         .unwrap()
    ///         .checked_sqrt()
    ///         .unwrap()
    ///         .to_string(),
    ///     "1-i"
    /// );
    /// // -4 = (2i)^2, and 2i is the principal root
    /// assert_eq!(
    ///     GaussianInteger::from(-4)
    ///         .checked_sqrt()
    ///         .unwrap()
    ///         .to_string(),
    ///     "2i"
    /// );
    /// assert!(
    ///     GaussianInteger::from_str("2+i")
    ///         .unwrap()
    ///         .checked_sqrt()
    ///         .is_none()
    /// );
    /// ```
    #[inline]
    fn checked_sqrt(self) -> Option<Self> {
        checked_sqrt_helper(&self)
    }
}

impl CheckedSqrt for &GaussianInteger {
    type Output = GaussianInteger;

    /// Returns the principal square root of a [`GaussianInteger`], or `None` if it is not a perfect
    /// square. The [`GaussianInteger`] is taken by reference.
    ///
    /// A nonzero Gaussian integer that is a perfect square has two square roots, each the negative
    /// of the other; the one returned is the principal root, whose real part is positive or, if it
    /// is zero, whose imaginary part is non-negative. That is the root whose argument lies in
    /// $(-\pi/2, \pi/2]$.
    ///
    /// The root is found through the norm: if $a + bi = (x + yi)^2$ then $N = \sqrt{a^2 + b^2}$ is
    /// an integer, $x^2 = (N + a) / 2$, $y^2 = (N - a) / 2$, and $2xy = b$ fixes the sign of $y$
    /// relative to that of $x$.
    ///
    /// $$
    /// f(z) = \begin{cases}
    ///     \operatorname{Some}(\sqrt{z}) & \text{if} \quad \sqrt{z} \in \Z[i], \\\\
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
    /// bits of the real and imaginary parts of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedSqrt;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (2+i)^2 = 3+4i
    /// assert_eq!(
    ///     (&GaussianInteger::from_str("3+4i").unwrap())
    ///         .checked_sqrt()
    ///         .unwrap()
    ///         .to_string(),
    ///     "2+i"
    /// );
    /// // (1-i)^2 = -2i
    /// assert_eq!(
    ///     (&GaussianInteger::from_str("-2i").unwrap())
    ///         .checked_sqrt()
    ///         .unwrap()
    ///         .to_string(),
    ///     "1-i"
    /// );
    /// // -4 = (2i)^2, and 2i is the principal root
    /// assert_eq!(
    ///     (&GaussianInteger::from(-4))
    ///         .checked_sqrt()
    ///         .unwrap()
    ///         .to_string(),
    ///     "2i"
    /// );
    /// assert!(
    ///     (&GaussianInteger::from_str("2+i").unwrap())
    ///         .checked_sqrt()
    ///         .is_none()
    /// );
    /// ```
    #[inline]
    fn checked_sqrt(self) -> Option<GaussianInteger> {
        checked_sqrt_helper(self)
    }
}

impl GaussianInteger {
    /// Returns all the square roots of a [`GaussianInteger`]: none if it is not a perfect square,
    /// one if it is zero, and otherwise the principal root and its negative, in the canonical order
    /// of [`ComparableGaussianInteger`](crate::gaussian_integer::ComparableGaussianInteger),
    /// lexicographic by real part and then imaginary part.
    ///
    /// The principal root is the one with positive real part or, if that is zero, with non-negative
    /// imaginary part; see [`CheckedSqrt`](malachite_base::num::arithmetic::traits::CheckedSqrt).
    ///
    /// $$
    /// f(z) = \\{ w \in \Z[i] : w^2 = z \\}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let roots = |s| {
    ///     GaussianInteger::from_str(s)
    ///         .unwrap()
    ///         .checked_sqrts()
    ///         .iter()
    ///         .map(ToString::to_string)
    ///         .collect::<Vec<_>>()
    /// };
    /// assert_eq!(roots("3+4i"), ["-2-i", "2+i"]);
    /// assert_eq!(roots("-1"), ["-i", "i"]);
    /// assert_eq!(roots("2+i"), Vec::<String>::new());
    /// assert_eq!(
    ///     GaussianInteger::ZERO.checked_sqrts(),
    ///     [GaussianInteger::ZERO]
    /// );
    /// ```
    pub fn checked_sqrts(&self) -> Vec<Self> {
        match checked_sqrt_helper(self) {
            None => Vec::new(),
            Some(root) if root == 0u32 => vec![root],
            Some(root) => {
                let neg_root = -&root;
                let mut roots = vec![root, neg_root];
                roots.sort_by(|a, b| {
                    ComparableGaussianIntegerRef(a).cmp(&ComparableGaussianIntegerRef(b))
                });
                roots
            }
        }
    }
}
