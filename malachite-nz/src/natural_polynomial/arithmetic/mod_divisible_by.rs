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

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_div_exact::assert_invertible;
use crate::natural_polynomial::arithmetic::mod_div_rem::{
    assert_reduced, checked_helper as div_rem_checked_helper,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::ModDivisibleBy;

// Whether `q` divides `p` modulo `m`.
//
// This is equivalent to the result of `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, FLINT
// 3.6.0, except that Malachite panics where FLINT assumes the leading coefficient invertible.
fn helper(p: &NaturalPolynomial, q: &NaturalPolynomial, m: &Natural) -> bool {
    assert_reduced(p, q, m);
    if *m == 1u32 || *p == NaturalPolynomial::ZERO {
        return true;
    }
    if *q == NaturalPolynomial::ZERO {
        return false;
    }
    assert_invertible(q, m);
    div_rem_checked_helper(p, q, m).unwrap().1 == NaturalPolynomial::ZERO
}

impl ModDivisibleBy<Self, Natural> for NaturalPolynomial {
    /// Determines whether a [`NaturalPolynomial`] is divisible by another modulo $m$, taking the
    /// first by value, the second by value, and the modulus by value. The coefficients of both must
    /// already be reduced modulo $m$, and the leading coefficient of the second, unless it is 0,
    /// must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (\exists q : a \equiv bq \pmod{m}).
    /// $$
    ///
    /// 0 is divisible by every polynomial, and only 0 is divisible by 0. When $m$ is 1, every
    /// polynomial is divisible by every other.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The
    /// remainder is found as by [`ModRem`](malachite_base::polynomial::ModRem), and compared with
    /// 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert!(p.mod_divisible_by(q, Natural::from(7u32)));
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// assert!(!p.mod_divisible_by(q, Natural::from(7u32)));
    /// ```
    ///
    /// This is equivalent to the result of `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`,
    /// FLINT 3.6.0, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn mod_divisible_by(self, other: Self, m: Natural) -> bool {
        helper(&self, &other, &m)
    }
}

impl ModDivisibleBy<Self, &Natural> for NaturalPolynomial {
    /// Determines whether a [`NaturalPolynomial`] is divisible by another modulo $m$, taking the
    /// first by value, the second by value, and the modulus by reference. The coefficients of both
    /// must already be reduced modulo $m$, and the leading coefficient of the second, unless it is
    /// 0, must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (\exists q : a \equiv bq \pmod{m}).
    /// $$
    ///
    /// 0 is divisible by every polynomial, and only 0 is divisible by 0. When $m$ is 1, every
    /// polynomial is divisible by every other.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The
    /// remainder is found as by [`ModRem`](malachite_base::polynomial::ModRem), and compared with
    /// 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert!(p.mod_divisible_by(q, &Natural::from(7u32)));
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// assert!(!p.mod_divisible_by(q, &Natural::from(7u32)));
    /// ```
    ///
    /// This is equivalent to the result of `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`,
    /// FLINT 3.6.0, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn mod_divisible_by(self, other: Self, m: &Natural) -> bool {
        helper(&self, &other, m)
    }
}

impl ModDivisibleBy<&Self, Natural> for NaturalPolynomial {
    /// Determines whether a [`NaturalPolynomial`] is divisible by another modulo $m$, taking the
    /// first by value, the second by reference, and the modulus by value. The coefficients of both
    /// must already be reduced modulo $m$, and the leading coefficient of the second, unless it is
    /// 0, must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (\exists q : a \equiv bq \pmod{m}).
    /// $$
    ///
    /// 0 is divisible by every polynomial, and only 0 is divisible by 0. When $m$ is 1, every
    /// polynomial is divisible by every other.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The
    /// remainder is found as by [`ModRem`](malachite_base::polynomial::ModRem), and compared with
    /// 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert!(p.mod_divisible_by(&q, Natural::from(7u32)));
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// assert!(!p.mod_divisible_by(&q, Natural::from(7u32)));
    /// ```
    ///
    /// This is equivalent to the result of `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`,
    /// FLINT 3.6.0, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn mod_divisible_by(self, other: &Self, m: Natural) -> bool {
        helper(&self, other, &m)
    }
}

impl ModDivisibleBy<&Self, &Natural> for NaturalPolynomial {
    /// Determines whether a [`NaturalPolynomial`] is divisible by another modulo $m$, taking the
    /// first by value, the second by reference, and the modulus by reference. The coefficients of
    /// both must already be reduced modulo $m$, and the leading coefficient of the second, unless
    /// it is 0, must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (\exists q : a \equiv bq \pmod{m}).
    /// $$
    ///
    /// 0 is divisible by every polynomial, and only 0 is divisible by 0. When $m$ is 1, every
    /// polynomial is divisible by every other.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The
    /// remainder is found as by [`ModRem`](malachite_base::polynomial::ModRem), and compared with
    /// 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert!(p.mod_divisible_by(&q, &Natural::from(7u32)));
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// assert!(!p.mod_divisible_by(&q, &Natural::from(7u32)));
    /// ```
    ///
    /// This is equivalent to the result of `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`,
    /// FLINT 3.6.0, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn mod_divisible_by(self, other: &Self, m: &Natural) -> bool {
        helper(&self, other, m)
    }
}

impl ModDivisibleBy<NaturalPolynomial, Natural> for &NaturalPolynomial {
    /// Determines whether a [`NaturalPolynomial`] is divisible by another modulo $m$, taking the
    /// first by reference, the second by value, and the modulus by value. The coefficients of both
    /// must already be reduced modulo $m$, and the leading coefficient of the second, unless it is
    /// 0, must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (\exists q : a \equiv bq \pmod{m}).
    /// $$
    ///
    /// 0 is divisible by every polynomial, and only 0 is divisible by 0. When $m$ is 1, every
    /// polynomial is divisible by every other.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The
    /// remainder is found as by [`ModRem`](malachite_base::polynomial::ModRem), and compared with
    /// 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert!((&p).mod_divisible_by(q, Natural::from(7u32)));
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// assert!(!(&p).mod_divisible_by(q, Natural::from(7u32)));
    /// ```
    ///
    /// This is equivalent to the result of `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`,
    /// FLINT 3.6.0, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn mod_divisible_by(self, other: NaturalPolynomial, m: Natural) -> bool {
        helper(self, &other, &m)
    }
}

impl ModDivisibleBy<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    /// Determines whether a [`NaturalPolynomial`] is divisible by another modulo $m$, taking the
    /// first by reference, the second by value, and the modulus by reference. The coefficients of
    /// both must already be reduced modulo $m$, and the leading coefficient of the second, unless
    /// it is 0, must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (\exists q : a \equiv bq \pmod{m}).
    /// $$
    ///
    /// 0 is divisible by every polynomial, and only 0 is divisible by 0. When $m$ is 1, every
    /// polynomial is divisible by every other.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The
    /// remainder is found as by [`ModRem`](malachite_base::polynomial::ModRem), and compared with
    /// 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert!((&p).mod_divisible_by(q, &Natural::from(7u32)));
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// assert!(!(&p).mod_divisible_by(q, &Natural::from(7u32)));
    /// ```
    ///
    /// This is equivalent to the result of `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`,
    /// FLINT 3.6.0, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn mod_divisible_by(self, other: NaturalPolynomial, m: &Natural) -> bool {
        helper(self, &other, m)
    }
}

impl ModDivisibleBy<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    /// Determines whether a [`NaturalPolynomial`] is divisible by another modulo $m$, taking the
    /// first by reference, the second by reference, and the modulus by value. The coefficients of
    /// both must already be reduced modulo $m$, and the leading coefficient of the second, unless
    /// it is 0, must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (\exists q : a \equiv bq \pmod{m}).
    /// $$
    ///
    /// 0 is divisible by every polynomial, and only 0 is divisible by 0. When $m$ is 1, every
    /// polynomial is divisible by every other.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The
    /// remainder is found as by [`ModRem`](malachite_base::polynomial::ModRem), and compared with
    /// 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert!((&p).mod_divisible_by(&q, Natural::from(7u32)));
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// assert!(!(&p).mod_divisible_by(&q, Natural::from(7u32)));
    /// ```
    ///
    /// This is equivalent to the result of `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`,
    /// FLINT 3.6.0, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn mod_divisible_by(self, other: &NaturalPolynomial, m: Natural) -> bool {
        helper(self, other, &m)
    }
}

impl ModDivisibleBy<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    /// Determines whether a [`NaturalPolynomial`] is divisible by another modulo $m$, taking the
    /// first by reference, the second by reference, and the modulus by reference. The coefficients
    /// of both must already be reduced modulo $m$, and the leading coefficient of the second,
    /// unless it is 0, must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (\exists q : a \equiv bq \pmod{m}).
    /// $$
    ///
    /// 0 is divisible by every polynomial, and only 0 is divisible by 0. When $m$ is 1, every
    /// polynomial is divisible by every other.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The
    /// remainder is found as by [`ModRem`](malachite_base::polynomial::ModRem), and compared with
    /// 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert!((&p).mod_divisible_by(&q, &Natural::from(7u32)));
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// assert!(!(&p).mod_divisible_by(&q, &Natural::from(7u32)));
    /// ```
    ///
    /// This is equivalent to the result of `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`,
    /// FLINT 3.6.0, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn mod_divisible_by(self, other: &NaturalPolynomial, m: &Natural) -> bool {
        helper(self, other, m)
    }
}
