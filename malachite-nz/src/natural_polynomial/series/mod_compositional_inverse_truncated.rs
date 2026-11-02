// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011, 2023 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::series::mod_compose_truncated::*;
use crate::natural_polynomial::series::mod_div_truncated::*;
use crate::natural_polynomial::series::mod_inverse_truncated::*;
use crate::natural_polynomial::series::mod_mul_truncated::mod_mul_truncated_ref_ref;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::AddMulAssign;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::{
    DivisibleBy, Mod, ModAdd, ModInverse, ModIsReduced, ModMul, ModSub,
};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::series::traits::ModCompositionalInverseTruncated;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::series::mod_compositional_inverse_truncated::*;

pub(crate) fn padded(p: NaturalPolynomial, len: usize) -> Vec<Natural> {
    let mut xs = p.coefficients;
    xs.resize(len, Natural::ZERO);
    xs
}

// The ring of coefficients modulo `m`, which must not fit in two limbs: smaller moduli are handled
// in word arithmetic. The power-series operations call the `Natural` kernels directly.
pub(crate) struct NaturalModRing<'a> {
    pub(crate) m: &'a Natural,
}

impl SeriesReversionRing for NaturalModRing<'_> {
    type C = Natural;

    #[inline]
    fn zero(&self) -> Natural {
        Natural::ZERO
    }

    #[inline]
    fn is_zero(&self, x: &Natural) -> bool {
        *x == 0u32
    }

    #[inline]
    fn add(&self, x: &Natural, y: &Natural) -> Natural {
        x.mod_add(y, self.m)
    }

    #[inline]
    fn sub(&self, x: &Natural, y: &Natural) -> Natural {
        x.mod_sub(y, self.m)
    }

    #[inline]
    fn mul(&self, x: &Natural, y: &Natural) -> Natural {
        x.mod_mul(y, self.m)
    }

    fn inverse(&self, x: &Natural) -> Option<Natural> {
        if *x == 0u32 {
            None
        } else {
            x.mod_inverse(self.m)
        }
    }

    fn div_by_index(&self, x: &Natural, i: u64) -> Option<Natural> {
        let inverse = self.inverse(&Natural::from(i).mod_op(self.m))?;
        Some(x.mod_mul(inverse, self.m))
    }

    #[inline]
    fn mul_by_index(&self, x: &Natural, i: u64) -> Natural {
        x.mod_mul(Natural::from(i).mod_op(self.m), self.m)
    }

    #[inline]
    fn mul_truncated(&self, xs: &[Natural], ys: &[Natural], len: usize) -> Vec<Natural> {
        padded(
            mod_mul_truncated_ref_ref(xs, ys, u64::exact_from(len), self.m),
            len,
        )
    }

    #[inline]
    fn inverse_truncated(&self, xs: &[Natural], len: usize) -> Vec<Natural> {
        padded(
            mod_inverse_truncated_with(
                xs,
                u64::exact_from(len),
                self.m,
                mod_inverse_truncated_dispatch,
            )
            .unwrap(),
            len,
        )
    }

    #[inline]
    fn div_truncated(&self, xs: &[Natural], ys: &[Natural], len: usize) -> Vec<Natural> {
        padded(
            mod_div_truncated_with(
                xs,
                ys,
                u64::exact_from(len),
                self.m,
                mod_div_truncated_dispatch,
            )
            .unwrap(),
            len,
        )
    }

    #[inline]
    fn compose_truncated(&self, xs: &[Natural], ys: &[Natural], len: usize) -> Vec<Natural> {
        padded(
            mod_compose_truncated_with(
                xs,
                ys,
                u64::exact_from(len),
                self.m,
                mod_compose_truncated_dispatch,
            ),
            len,
        )
    }

    // The products are summed unreduced, and only the sum is reduced.
    fn dot_rev(&self, xs: &[Natural], ys: &[Natural], n: usize) -> Natural {
        let mut sum = Natural::ZERO;
        for (x, y) in xs[..n].iter().zip(ys[..n].iter().rev()) {
            sum.add_mul_assign(x, y);
        }
        sum.mod_op(self.m)
    }

    // Measured with a 256-bit modulus, the fast Lagrange algorithm was 1.6 to 2.1 times as fast as
    // Newton's iteration at every length up to 8192. It keeps about $\sqrt n$ powers of length $n$,
    // so beyond that Newton's iteration is used to bound the memory.
    #[inline]
    fn newton_cutoff(&self) -> usize {
        8192
    }

    // Every index $i$ with $2 \leq i < n$ is invertible exactly when no prime factor of $m$ is less
    // than $n$. Since $m$ does not fit in two limbs, it is larger than $n$.
    #[inline]
    fn lagrange_possible(&self, n: usize) -> bool {
        (2..u64::exact_from(n)).all(|i| !self.m.divisible_by(Natural::from(i)))
    }
}

// The compositional inverse of `xs` modulo `m` to `len` terms, computed in `T`, which `m` must fit
// in.
fn mod_compositional_inverse_truncated_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    xs: &[Natural],
    len: u64,
    m: &Natural,
) -> Option<NaturalPolynomial>
where
    Natural: From<T>,
{
    let xs: Vec<T> = xs.iter().map(T::exact_from).collect();
    Some(NaturalPolynomial {
        coefficients: UnsignedPolynomial::from_coefficients_asc(xs)
            .mod_compositional_inverse_truncated(len, T::exact_from(m))?
            .into_coefficients_asc()
            .into_iter()
            .map(Natural::from)
            .collect(),
    })
}

// The compositional inverse of `p` modulo `m` to `len` terms. When `m` fits in one or two limbs,
// this is done in word arithmetic.
//
// This is equivalent to `gr_poly_revert_series` from `gr_poly/revert_series.c`, FLINT 3.6.0, with
// coefficients modulo `m`, except that `None` is returned where FLINT fails.
pub(crate) fn mod_compositional_inverse_truncated_helper(
    p: &NaturalPolynomial,
    len: u64,
    m: &Natural,
) -> Option<NaturalPolynomial> {
    assert_ne!(*m, 0u32);
    assert!(
        p.mod_is_reduced(m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    if *m == 1u32 {
        return Some(NaturalPolynomial::ZERO);
    }
    let bits = m.significant_bits();
    if bits <= Limb::WIDTH {
        mod_compositional_inverse_truncated_word::<Limb>(&p.coefficients, len, m)
    } else if bits <= DoubleLimb::WIDTH {
        mod_compositional_inverse_truncated_word::<DoubleLimb>(&p.coefficients, len, m)
    } else {
        compositional_inverse_truncated_helper(
            &NaturalModRing { m },
            &p.coefficients,
            usize::exact_from(len),
        )
        .map(|coefficients| {
            let mut q = NaturalPolynomial { coefficients };
            q.trim();
            q
        })
    }
}

impl ModCompositionalInverseTruncated<Natural> for NaturalPolynomial {
    type Output = Self;

    /// Finds the compositional inverse of a [`NaturalPolynomial`] as a power series modulo $m$,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking the polynomial by
    /// value and the modulus by value. Its coefficients must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, n, m) = q, \quad \deg q < n, \quad p(q(x)) \equiv q(p(x)) \equiv x \pmod{x^n, m}.
    /// $$
    ///
    /// Returns `None` unless the constant coefficient is 0 and the coefficient of $x$ is invertible
    /// modulo $m$; otherwise the inverse exists and is unique. When the inverse exists and `len` is
    /// less than 2, the result is the zero polynomial, and when $m$ is 1 the result is always
    /// `Some` of the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, the inverse is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`]. Otherwise it is computed by the fast Lagrange algorithm, in which
    /// the coefficient of $x^i$ is that of $x^{i-1}$ in $(x/p)^i$, divided by $i$; when $m$ has a
    /// prime factor less than `len`, so that one of those divisions is not possible, or when `len`
    /// is at least 8192, Newton's iteration is used instead, in which each step doubles the number
    /// of correct coefficients with a composition and a division.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^{3/2} m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::series::traits::ModCompositionalInverseTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+x").unwrap())
    ///         .mod_compositional_inverse_truncated(6, Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "2*x^4+2*x^3+6*x^2+x"
    /// );
    /// // The coefficient of x is not invertible.
    /// assert!(
    ///     (NaturalPolynomial::from_str("x^2+2*x").unwrap())
    ///         .mod_compositional_inverse_truncated(5, Natural::from(10u32))
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `gr_poly_revert_series` from `gr_poly/revert_series.c`, FLINT 3.6.0,
    /// with coefficients modulo $m$ (FLINT's `fmpz_mod_poly` has no series reversion), except that
    /// `None` is returned where FLINT fails, that the inverse modulo $x^0$ or $x^1$, or modulo 1,
    /// is 0, that the cutoff was measured, and that Newton's iteration is chosen at once when a
    /// division by an index is not possible.
    #[inline]
    fn mod_compositional_inverse_truncated(self, len: u64, m: Natural) -> Option<Self> {
        mod_compositional_inverse_truncated_helper(&self, len, &m)
    }
}

impl ModCompositionalInverseTruncated<&Natural> for NaturalPolynomial {
    type Output = Self;

    /// Finds the compositional inverse of a [`NaturalPolynomial`] as a power series modulo $m$,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking the polynomial by
    /// value and the modulus by reference. Its coefficients must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, n, m) = q, \quad \deg q < n, \quad p(q(x)) \equiv q(p(x)) \equiv x \pmod{x^n, m}.
    /// $$
    ///
    /// Returns `None` unless the constant coefficient is 0 and the coefficient of $x$ is invertible
    /// modulo $m$; otherwise the inverse exists and is unique. When the inverse exists and `len` is
    /// less than 2, the result is the zero polynomial, and when $m$ is 1 the result is always
    /// `Some` of the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, the inverse is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`]. Otherwise it is computed by the fast Lagrange algorithm, in which
    /// the coefficient of $x^i$ is that of $x^{i-1}$ in $(x/p)^i$, divided by $i$; when $m$ has a
    /// prime factor less than `len`, so that one of those divisions is not possible, or when `len`
    /// is at least 8192, Newton's iteration is used instead, in which each step doubles the number
    /// of correct coefficients with a composition and a division.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^{3/2} m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::series::traits::ModCompositionalInverseTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+x").unwrap())
    ///         .mod_compositional_inverse_truncated(6, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "2*x^4+2*x^3+6*x^2+x"
    /// );
    /// // The coefficient of x is not invertible.
    /// assert!(
    ///     (NaturalPolynomial::from_str("x^2+2*x").unwrap())
    ///         .mod_compositional_inverse_truncated(5, &Natural::from(10u32))
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `gr_poly_revert_series` from `gr_poly/revert_series.c`, FLINT 3.6.0,
    /// with coefficients modulo $m$ (FLINT's `fmpz_mod_poly` has no series reversion), except that
    /// `None` is returned where FLINT fails, that the inverse modulo $x^0$ or $x^1$, or modulo 1,
    /// is 0, that the cutoff was measured, and that Newton's iteration is chosen at once when a
    /// division by an index is not possible.
    #[inline]
    fn mod_compositional_inverse_truncated(self, len: u64, m: &Natural) -> Option<Self> {
        mod_compositional_inverse_truncated_helper(&self, len, m)
    }
}

impl ModCompositionalInverseTruncated<Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the compositional inverse of a [`NaturalPolynomial`] as a power series modulo $m$,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking the polynomial by
    /// reference and the modulus by value. Its coefficients must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, n, m) = q, \quad \deg q < n, \quad p(q(x)) \equiv q(p(x)) \equiv x \pmod{x^n, m}.
    /// $$
    ///
    /// Returns `None` unless the constant coefficient is 0 and the coefficient of $x$ is invertible
    /// modulo $m$; otherwise the inverse exists and is unique. When the inverse exists and `len` is
    /// less than 2, the result is the zero polynomial, and when $m$ is 1 the result is always
    /// `Some` of the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, the inverse is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`]. Otherwise it is computed by the fast Lagrange algorithm, in which
    /// the coefficient of $x^i$ is that of $x^{i-1}$ in $(x/p)^i$, divided by $i$; when $m$ has a
    /// prime factor less than `len`, so that one of those divisions is not possible, or when `len`
    /// is at least 8192, Newton's iteration is used instead, in which each step doubles the number
    /// of correct coefficients with a composition and a division.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^{3/2} m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::series::traits::ModCompositionalInverseTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+x").unwrap())
    ///         .mod_compositional_inverse_truncated(6, Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "2*x^4+2*x^3+6*x^2+x"
    /// );
    /// // The coefficient of x is not invertible.
    /// assert!(
    ///     (&NaturalPolynomial::from_str("x^2+2*x").unwrap())
    ///         .mod_compositional_inverse_truncated(5, Natural::from(10u32))
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `gr_poly_revert_series` from `gr_poly/revert_series.c`, FLINT 3.6.0,
    /// with coefficients modulo $m$ (FLINT's `fmpz_mod_poly` has no series reversion), except that
    /// `None` is returned where FLINT fails, that the inverse modulo $x^0$ or $x^1$, or modulo 1,
    /// is 0, that the cutoff was measured, and that Newton's iteration is chosen at once when a
    /// division by an index is not possible.
    #[inline]
    fn mod_compositional_inverse_truncated(
        self,
        len: u64,
        m: Natural,
    ) -> Option<NaturalPolynomial> {
        mod_compositional_inverse_truncated_helper(self, len, &m)
    }
}

impl ModCompositionalInverseTruncated<&Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the compositional inverse of a [`NaturalPolynomial`] as a power series modulo $m$,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking the polynomial by
    /// reference and the modulus by reference. Its coefficients must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, n, m) = q, \quad \deg q < n, \quad p(q(x)) \equiv q(p(x)) \equiv x \pmod{x^n, m}.
    /// $$
    ///
    /// Returns `None` unless the constant coefficient is 0 and the coefficient of $x$ is invertible
    /// modulo $m$; otherwise the inverse exists and is unique. When the inverse exists and `len` is
    /// less than 2, the result is the zero polynomial, and when $m$ is 1 the result is always
    /// `Some` of the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, the inverse is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`]. Otherwise it is computed by the fast Lagrange algorithm, in which
    /// the coefficient of $x^i$ is that of $x^{i-1}$ in $(x/p)^i$, divided by $i$; when $m$ has a
    /// prime factor less than `len`, so that one of those divisions is not possible, or when `len`
    /// is at least 8192, Newton's iteration is used instead, in which each step doubles the number
    /// of correct coefficients with a composition and a division.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^{3/2} m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::series::traits::ModCompositionalInverseTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+x").unwrap())
    ///         .mod_compositional_inverse_truncated(6, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "2*x^4+2*x^3+6*x^2+x"
    /// );
    /// // The coefficient of x is not invertible.
    /// assert!(
    ///     (&NaturalPolynomial::from_str("x^2+2*x").unwrap())
    ///         .mod_compositional_inverse_truncated(5, &Natural::from(10u32))
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `gr_poly_revert_series` from `gr_poly/revert_series.c`, FLINT 3.6.0,
    /// with coefficients modulo $m$ (FLINT's `fmpz_mod_poly` has no series reversion), except that
    /// `None` is returned where FLINT fails, that the inverse modulo $x^0$ or $x^1$, or modulo 1,
    /// is 0, that the cutoff was measured, and that Newton's iteration is chosen at once when a
    /// division by an index is not possible.
    #[inline]
    fn mod_compositional_inverse_truncated(
        self,
        len: u64,
        m: &Natural,
    ) -> Option<NaturalPolynomial> {
        mod_compositional_inverse_truncated_helper(self, len, m)
    }
}
