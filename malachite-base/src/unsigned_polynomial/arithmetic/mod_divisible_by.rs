// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
//      Copyright © 2011 William Hart
//
//      Copyright © 2023 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::ModDivisibleBy;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_div_exact::assert_invertible;
use crate::unsigned_polynomial::arithmetic::mod_div_rem::{assert_reduced, mod_div_rem_with};
use crate::unsigned_polynomial::arithmetic::mod_div_truncated::mod_div_truncated_dispatch;

// Whether `q` divides `p` modulo `m`.
//
// This is equivalent to the result of `nmod_poly_divides` from `nmod_poly/divides.c`, FLINT 3.6.0,
// except that Malachite panics where FLINT assumes the leading coefficient invertible.
fn helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> bool {
    assert_reduced(p, q, m);
    if m == T::ONE || *p == UnsignedPolynomial::ZERO {
        return true;
    }
    if *q == UnsignedPolynomial::ZERO {
        return false;
    }
    assert_invertible(q, m);
    mod_div_rem_with(
        &p.coefficients,
        &q.coefficients,
        m,
        mod_div_truncated_dispatch,
    )
    .unwrap()
    .1 == UnsignedPolynomial::ZERO
}

impl<T: PrimitiveUnsigned> ModDivisibleBy<Self, T> for UnsignedPolynomial<T> {
    /// Determines whether an [`UnsignedPolynomial`] is divisible by another modulo $m$, taking the
    /// first by value and the second by value. The coefficients of both must already be reduced
    /// modulo $m$, and the leading coefficient of the second, unless it is 0, must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (\exists q : a \equiv bq \pmod{m}).
    /// $$
    ///
    /// 0 is divisible by every polynomial, and only 0 is divisible by 0. When $m$ is 1, every
    /// polynomial is divisible by every other.
    ///
    /// The remainder is found as by [`ModRem`](crate::polynomial::ModRem), and compared with 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `self` and `other` are nonzero and the leading coefficient of `other` is not
    /// invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivisibleBy;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert!(p.mod_divisible_by(q, 7));
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3").unwrap();
    /// assert!(!p.mod_divisible_by(q, 7));
    /// ```
    ///
    /// This is equivalent to the result of `nmod_poly_divides` from `nmod_poly/divides.c`, FLINT
    /// 3.6.0, except that Malachite panics where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_divisible_by(self, other: Self, m: T) -> bool {
        helper(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModDivisibleBy<&Self, T> for UnsignedPolynomial<T> {
    /// Determines whether an [`UnsignedPolynomial`] is divisible by another modulo $m$, taking the
    /// first by value and the second by reference. The coefficients of both must already be reduced
    /// modulo $m$, and the leading coefficient of the second, unless it is 0, must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (\exists q : a \equiv bq \pmod{m}).
    /// $$
    ///
    /// 0 is divisible by every polynomial, and only 0 is divisible by 0. When $m$ is 1, every
    /// polynomial is divisible by every other.
    ///
    /// The remainder is found as by [`ModRem`](crate::polynomial::ModRem), and compared with 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `self` and `other` are nonzero and the leading coefficient of `other` is not
    /// invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivisibleBy;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert!(p.mod_divisible_by(&q, 7));
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3").unwrap();
    /// assert!(!p.mod_divisible_by(&q, 7));
    /// ```
    ///
    /// This is equivalent to the result of `nmod_poly_divides` from `nmod_poly/divides.c`, FLINT
    /// 3.6.0, except that Malachite panics where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_divisible_by(self, other: &Self, m: T) -> bool {
        helper(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModDivisibleBy<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    /// Determines whether an [`UnsignedPolynomial`] is divisible by another modulo $m$, taking the
    /// first by reference and the second by value. The coefficients of both must already be reduced
    /// modulo $m$, and the leading coefficient of the second, unless it is 0, must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (\exists q : a \equiv bq \pmod{m}).
    /// $$
    ///
    /// 0 is divisible by every polynomial, and only 0 is divisible by 0. When $m$ is 1, every
    /// polynomial is divisible by every other.
    ///
    /// The remainder is found as by [`ModRem`](crate::polynomial::ModRem), and compared with 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `self` and `other` are nonzero and the leading coefficient of `other` is not
    /// invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivisibleBy;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert!((&p).mod_divisible_by(q, 7));
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3").unwrap();
    /// assert!(!(&p).mod_divisible_by(q, 7));
    /// ```
    ///
    /// This is equivalent to the result of `nmod_poly_divides` from `nmod_poly/divides.c`, FLINT
    /// 3.6.0, except that Malachite panics where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_divisible_by(self, other: UnsignedPolynomial<T>, m: T) -> bool {
        helper(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModDivisibleBy<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    /// Determines whether an [`UnsignedPolynomial`] is divisible by another modulo $m$, taking the
    /// first by reference and the second by reference. The coefficients of both must already be
    /// reduced modulo $m$, and the leading coefficient of the second, unless it is 0, must be
    /// invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (\exists q : a \equiv bq \pmod{m}).
    /// $$
    ///
    /// 0 is divisible by every polynomial, and only 0 is divisible by 0. When $m$ is 1, every
    /// polynomial is divisible by every other.
    ///
    /// The remainder is found as by [`ModRem`](crate::polynomial::ModRem), and compared with 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `self` and `other` are nonzero and the leading coefficient of `other` is not
    /// invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivisibleBy;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert!((&p).mod_divisible_by(&q, 7));
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3").unwrap();
    /// assert!(!(&p).mod_divisible_by(&q, 7));
    /// ```
    ///
    /// This is equivalent to the result of `nmod_poly_divides` from `nmod_poly/divides.c`, FLINT
    /// 3.6.0, except that Malachite panics where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_divisible_by(self, other: &UnsignedPolynomial<T>, m: T) -> bool {
        helper(self, other, m)
    }
}
