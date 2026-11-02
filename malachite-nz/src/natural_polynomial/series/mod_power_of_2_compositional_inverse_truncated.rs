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
use crate::natural_polynomial::series::mod_compositional_inverse_truncated::padded;
use crate::natural_polynomial::series::mod_power_of_2_compose_truncated::*;
use crate::natural_polynomial::series::mod_power_of_2_div_truncated::*;
use crate::natural_polynomial::series::mod_power_of_2_inverse_truncated::*;
use crate::natural_polynomial::series::mod_power_of_2_mul_truncated::*;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{AddMulAssign, Parity};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::modular::traits::{
    ModPowerOf2, ModPowerOf2Add, ModPowerOf2Inverse, ModPowerOf2IsReduced, ModPowerOf2Mul,
    ModPowerOf2Sub,
};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::series::traits::ModPowerOf2CompositionalInverseTruncated;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::series::mod_compositional_inverse_truncated::*;

// The ring of coefficients modulo $2^k$, where $2^k$ must not fit in two limbs: smaller powers are
// handled in word arithmetic. The power-series operations call the `Natural` kernels directly.
pub(crate) struct NaturalModPowerOf2Ring {
    pub(crate) pow: u64,
}

impl SeriesReversionRing for NaturalModPowerOf2Ring {
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
        x.mod_power_of_2_add(y, self.pow)
    }

    #[inline]
    fn sub(&self, x: &Natural, y: &Natural) -> Natural {
        x.mod_power_of_2_sub(y, self.pow)
    }

    #[inline]
    fn mul(&self, x: &Natural, y: &Natural) -> Natural {
        x.mod_power_of_2_mul(y, self.pow)
    }

    fn inverse(&self, x: &Natural) -> Option<Natural> {
        if x.even() {
            None
        } else {
            x.mod_power_of_2_inverse(self.pow)
        }
    }

    fn div_by_index(&self, x: &Natural, i: u64) -> Option<Natural> {
        let inverse = self.inverse(&Natural::from(i).mod_power_of_2(self.pow))?;
        Some(x.mod_power_of_2_mul(inverse, self.pow))
    }

    #[inline]
    fn mul_by_index(&self, x: &Natural, i: u64) -> Natural {
        x.mod_power_of_2_mul(Natural::from(i).mod_power_of_2(self.pow), self.pow)
    }

    #[inline]
    fn mul_truncated(&self, xs: &[Natural], ys: &[Natural], len: usize) -> Vec<Natural> {
        padded(
            mod_power_of_2_mul_truncated_ref_ref(xs, ys, u64::exact_from(len), self.pow),
            len,
        )
    }

    #[inline]
    fn inverse_truncated(&self, xs: &[Natural], len: usize) -> Vec<Natural> {
        padded(
            mod_power_of_2_inverse_truncated_with(
                xs,
                u64::exact_from(len),
                self.pow,
                mod_power_of_2_inverse_truncated_dispatch,
            )
            .unwrap(),
            len,
        )
    }

    #[inline]
    fn div_truncated(&self, xs: &[Natural], ys: &[Natural], len: usize) -> Vec<Natural> {
        padded(
            mod_power_of_2_div_truncated_with(
                xs,
                ys,
                u64::exact_from(len),
                self.pow,
                mod_power_of_2_div_truncated_dispatch,
            )
            .unwrap(),
            len,
        )
    }

    #[inline]
    fn compose_truncated(&self, xs: &[Natural], ys: &[Natural], len: usize) -> Vec<Natural> {
        padded(
            mod_power_of_2_compose_truncated_with(
                xs,
                ys,
                u64::exact_from(len),
                self.pow,
                mod_power_of_2_compose_truncated_dispatch,
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
        sum.mod_power_of_2(self.pow)
    }

    // 2 is not invertible, so a division by an index is possible only when there is none.
    #[inline]
    fn lagrange_possible(&self, n: usize) -> bool {
        n <= 2
    }
}

// The compositional inverse of `xs` modulo $2^k$ to `len` terms, computed in `T`, which $2^k$ must
// fit in.
fn mod_power_of_2_compositional_inverse_truncated_word<
    T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>,
>(
    xs: &[Natural],
    len: u64,
    pow: u64,
) -> Option<NaturalPolynomial>
where
    Natural: From<T>,
{
    let xs: Vec<T> = xs.iter().map(T::exact_from).collect();
    Some(NaturalPolynomial {
        coefficients: UnsignedPolynomial::from_coefficients_asc(xs)
            .mod_power_of_2_compositional_inverse_truncated(len, pow)?
            .into_coefficients_asc()
            .into_iter()
            .map(Natural::from)
            .collect(),
    })
}

// The compositional inverse of `p` modulo $2^k$ to `len` terms. When $2^k$ fits in one or two
// limbs, this is done in word arithmetic.
//
// This is equivalent to `gr_poly_revert_series` from `gr_poly/revert_series.c`, FLINT 3.6.0, with
// coefficients modulo $2^k$, except that `None` is returned where FLINT fails.
pub(crate) fn mod_power_of_2_compositional_inverse_truncated_helper(
    p: &NaturalPolynomial,
    len: u64,
    pow: u64,
) -> Option<NaturalPolynomial> {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    if pow == 0 {
        return Some(NaturalPolynomial::ZERO);
    }
    if pow <= Limb::WIDTH {
        mod_power_of_2_compositional_inverse_truncated_word::<Limb>(&p.coefficients, len, pow)
    } else if pow <= DoubleLimb::WIDTH {
        mod_power_of_2_compositional_inverse_truncated_word::<DoubleLimb>(&p.coefficients, len, pow)
    } else {
        compositional_inverse_truncated_helper(
            &NaturalModPowerOf2Ring { pow },
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

impl ModPowerOf2CompositionalInverseTruncated for NaturalPolynomial {
    type Output = Self;

    /// Finds the compositional inverse of a [`NaturalPolynomial`] as a power series modulo $2^k$,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking it by value. Its
    /// coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, n, k) = q, \quad \deg q < n, \quad p(q(x)) \equiv q(p(x)) \equiv x \pmod{x^n, 2^k}.
    /// $$
    ///
    /// Returns `None` unless the constant coefficient is 0 and the coefficient of $x$ is odd;
    /// otherwise the inverse exists and is unique. When the inverse exists and `len` is less than
    /// 2, the result is the zero polynomial, and when $k$ is 0 the result is always `Some` of the
    /// zero polynomial.
    ///
    /// When $2^k$ fits in one or two limbs, the inverse is computed in wrapping word arithmetic, as
    /// for an [`UnsignedPolynomial`]. The Lagrange algorithms divide by every index less than
    /// `len`, which is not possible modulo $2^k$ once 2 is among them, so the inverse is computed
    /// by Newton's iteration, in which each step doubles the number of correct coefficients with a
    /// composition and a division.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2 k \log k \log\log k)$
    ///
    /// $M(n, k) = O(nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::series::traits::ModPowerOf2CompositionalInverseTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+x").unwrap())
    ///         .mod_power_of_2_compositional_inverse_truncated(6, 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^5+3*x^4+2*x^3+7*x^2+x"
    /// );
    /// // The coefficient of x is even.
    /// assert!(
    ///     (NaturalPolynomial::from_str("x^2+2*x").unwrap())
    ///         .mod_power_of_2_compositional_inverse_truncated(5, 3)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_revert_series` from `nmod_poly/revert_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$ and any $k$, except that `None` is returned where FLINT throws
    /// or fails, that the inverse modulo $x^0$ or $x^1$, or modulo 1, is 0, and that Newton's
    /// iteration is used at once rather than after the fast Lagrange algorithm fails.
    #[inline]
    fn mod_power_of_2_compositional_inverse_truncated(self, len: u64, pow: u64) -> Option<Self> {
        mod_power_of_2_compositional_inverse_truncated_helper(&self, len, pow)
    }
}

impl ModPowerOf2CompositionalInverseTruncated for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Finds the compositional inverse of a [`NaturalPolynomial`] as a power series modulo $2^k$,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking it by reference. Its
    /// coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, n, k) = q, \quad \deg q < n, \quad p(q(x)) \equiv q(p(x)) \equiv x \pmod{x^n, 2^k}.
    /// $$
    ///
    /// Returns `None` unless the constant coefficient is 0 and the coefficient of $x$ is odd;
    /// otherwise the inverse exists and is unique. When the inverse exists and `len` is less than
    /// 2, the result is the zero polynomial, and when $k$ is 0 the result is always `Some` of the
    /// zero polynomial.
    ///
    /// When $2^k$ fits in one or two limbs, the inverse is computed in wrapping word arithmetic, as
    /// for an [`UnsignedPolynomial`]. The Lagrange algorithms divide by every index less than
    /// `len`, which is not possible modulo $2^k$ once 2 is among them, so the inverse is computed
    /// by Newton's iteration, in which each step doubles the number of correct coefficients with a
    /// composition and a division.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2 k \log k \log\log k)$
    ///
    /// $M(n, k) = O(nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::series::traits::ModPowerOf2CompositionalInverseTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+x").unwrap())
    ///         .mod_power_of_2_compositional_inverse_truncated(6, 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^5+3*x^4+2*x^3+7*x^2+x"
    /// );
    /// // The coefficient of x is even.
    /// assert!(
    ///     (&NaturalPolynomial::from_str("x^2+2*x").unwrap())
    ///         .mod_power_of_2_compositional_inverse_truncated(5, 3)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_revert_series` from `nmod_poly/revert_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$ and any $k$, except that `None` is returned where FLINT throws
    /// or fails, that the inverse modulo $x^0$ or $x^1$, or modulo 1, is 0, and that Newton's
    /// iteration is used at once rather than after the fast Lagrange algorithm fails.
    #[inline]
    fn mod_power_of_2_compositional_inverse_truncated(
        self,
        len: u64,
        pow: u64,
    ) -> Option<NaturalPolynomial> {
        mod_power_of_2_compositional_inverse_truncated_helper(self, len, pow)
    }
}
