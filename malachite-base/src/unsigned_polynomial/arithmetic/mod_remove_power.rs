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
use crate::polynomial::{ModRemovePower, ModRemovePowerAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_div_exact::assert_invertible;
use crate::unsigned_polynomial::arithmetic::mod_div_rem::{assert_reduced, mod_div_rem_with};
use crate::unsigned_polynomial::arithmetic::mod_div_truncated::mod_div_truncated_dispatch;

// The largest power of `q` that divides `p` modulo `m`, divided out of `p`, and its exponent: `p`
// is divided by `q` while the remainder is 0.
//
// This is equivalent to `nmod_poly_remove` from `nmod_poly/remove.c`, FLINT 3.6.0, except that
// Malachite panics where FLINT never returns or assumes the leading coefficient invertible.
fn helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> (UnsignedPolynomial<T>, u64) {
    assert_reduced(p, q, m);
    if m == T::ONE {
        return (UnsignedPolynomial::ZERO, 0);
    }
    assert_invertible(q, m);
    assert!(
        q.coefficients.len() > 1,
        "the factor must not be a constant"
    );
    let mut p = p.clone();
    let mut e = 0;
    while p.coefficients.len() >= q.coefficients.len() {
        let (s, r) = mod_div_rem_with(
            &p.coefficients,
            &q.coefficients,
            m,
            mod_div_truncated_dispatch,
        )
        .unwrap();
        if r != UnsignedPolynomial::ZERO {
            break;
        }
        p = s;
        e += 1;
    }
    (p, e)
}

impl<T: PrimitiveUnsigned> ModRemovePower<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Removes the largest power of an [`UnsignedPolynomial`] that divides another modulo $m$,
    /// returning the reduced polynomial and the exponent of that power, taking the polynomial by
    /// value and the factor by value. The coefficients of both must already be reduced modulo $m$,
    /// and the factor must have degree at least 1 and a leading coefficient that is invertible
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, e), \quad a \equiv b^e q \pmod{m}, \quad b \nmid q.
    /// $$
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $m$
    /// is 1, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by [`ModDivRem`](crate::polynomial::ModDivRem),
    /// while the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if, unless $m$ is 1, `other` is 0 or a constant or its leading coefficient is not
    /// invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModRemovePower;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+5*x^3+2*x^2+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let (s, e) = p.mod_remove_power(q, 7);
    /// assert_eq!(s.to_string(), "x+2");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_remove` from `nmod_poly/remove.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT never returns or assumes the leading coefficient invertible.
    #[inline]
    fn mod_remove_power(self, other: Self, m: T) -> (Self, u64) {
        helper(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModRemovePower<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Removes the largest power of an [`UnsignedPolynomial`] that divides another modulo $m$,
    /// returning the reduced polynomial and the exponent of that power, taking the polynomial by
    /// value and the factor by reference. The coefficients of both must already be reduced modulo
    /// $m$, and the factor must have degree at least 1 and a leading coefficient that is invertible
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, e), \quad a \equiv b^e q \pmod{m}, \quad b \nmid q.
    /// $$
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $m$
    /// is 1, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by [`ModDivRem`](crate::polynomial::ModDivRem),
    /// while the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if, unless $m$ is 1, `other` is 0 or a constant or its leading coefficient is not
    /// invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModRemovePower;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+5*x^3+2*x^2+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let (s, e) = p.mod_remove_power(&q, 7);
    /// assert_eq!(s.to_string(), "x+2");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_remove` from `nmod_poly/remove.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT never returns or assumes the leading coefficient invertible.
    #[inline]
    fn mod_remove_power(self, other: &Self, m: T) -> (Self, u64) {
        helper(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModRemovePower<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Removes the largest power of an [`UnsignedPolynomial`] that divides another modulo $m$,
    /// returning the reduced polynomial and the exponent of that power, taking the polynomial by
    /// reference and the factor by value. The coefficients of both must already be reduced modulo
    /// $m$, and the factor must have degree at least 1 and a leading coefficient that is invertible
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, e), \quad a \equiv b^e q \pmod{m}, \quad b \nmid q.
    /// $$
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $m$
    /// is 1, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by [`ModDivRem`](crate::polynomial::ModDivRem),
    /// while the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if, unless $m$ is 1, `other` is 0 or a constant or its leading coefficient is not
    /// invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModRemovePower;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+5*x^3+2*x^2+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let (s, e) = (&p).mod_remove_power(q, 7);
    /// assert_eq!(s.to_string(), "x+2");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_remove` from `nmod_poly/remove.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT never returns or assumes the leading coefficient invertible.
    #[inline]
    fn mod_remove_power(self, other: UnsignedPolynomial<T>, m: T) -> (UnsignedPolynomial<T>, u64) {
        helper(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModRemovePower<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Removes the largest power of an [`UnsignedPolynomial`] that divides another modulo $m$,
    /// returning the reduced polynomial and the exponent of that power, taking the polynomial by
    /// reference and the factor by reference. The coefficients of both must already be reduced
    /// modulo $m$, and the factor must have degree at least 1 and a leading coefficient that is
    /// invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, e), \quad a \equiv b^e q \pmod{m}, \quad b \nmid q.
    /// $$
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $m$
    /// is 1, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by [`ModDivRem`](crate::polynomial::ModDivRem),
    /// while the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if, unless $m$ is 1, `other` is 0 or a constant or its leading coefficient is not
    /// invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModRemovePower;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+5*x^3+2*x^2+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let (s, e) = (&p).mod_remove_power(&q, 7);
    /// assert_eq!(s.to_string(), "x+2");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_remove` from `nmod_poly/remove.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT never returns or assumes the leading coefficient invertible.
    #[inline]
    fn mod_remove_power(self, other: &UnsignedPolynomial<T>, m: T) -> (UnsignedPolynomial<T>, u64) {
        helper(self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModRemovePowerAssign<Self, T> for UnsignedPolynomial<T> {
    /// Replaces an [`UnsignedPolynomial`] with itself divided by the largest power of another that
    /// divides it modulo $m$, returning the exponent of that power, taking the factor by value. The
    /// coefficients of both must already be reduced modulo $m$, and the factor must have degree at
    /// least 1 and a leading coefficient that is invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv b^e q \pmod{m}, \quad b \nmid q,
    /// $$
    /// returning $e$.
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $m$
    /// is 1, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by [`ModDivRem`](crate::polynomial::ModDivRem),
    /// while the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if, unless $m$ is 1, `other` is 0 or a constant or its leading coefficient is not
    /// invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModRemovePowerAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^4+5*x^3+2*x^2+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(p.mod_remove_power_assign(q, 7), 3);
    /// assert_eq!(p.to_string(), "x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_remove` from `nmod_poly/remove.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT never returns or assumes the leading coefficient invertible.
    #[inline]
    fn mod_remove_power_assign(&mut self, other: Self, m: T) -> u64 {
        let (q, e) = helper(self, &other, m);
        *self = q;
        e
    }
}

impl<T: PrimitiveUnsigned> ModRemovePowerAssign<&Self, T> for UnsignedPolynomial<T> {
    /// Replaces an [`UnsignedPolynomial`] with itself divided by the largest power of another that
    /// divides it modulo $m$, returning the exponent of that power, taking the factor by reference.
    /// The coefficients of both must already be reduced modulo $m$, and the factor must have degree
    /// at least 1 and a leading coefficient that is invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv b^e q \pmod{m}, \quad b \nmid q,
    /// $$
    /// returning $e$.
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $m$
    /// is 1, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by [`ModDivRem`](crate::polynomial::ModDivRem),
    /// while the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if, unless $m$ is 1, `other` is 0 or a constant or its leading coefficient is not
    /// invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModRemovePowerAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^4+5*x^3+2*x^2+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(p.mod_remove_power_assign(&q, 7), 3);
    /// assert_eq!(p.to_string(), "x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_remove` from `nmod_poly/remove.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT never returns or assumes the leading coefficient invertible.
    #[inline]
    fn mod_remove_power_assign(&mut self, other: &Self, m: T) -> u64 {
        let (q, e) = helper(self, other, m);
        *self = q;
        e
    }
}
