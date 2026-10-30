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
use malachite_base::polynomial::{ModRemovePower, ModRemovePowerAssign};

// The largest power of `q` that divides `p` modulo `m`, divided out of `p`, and its exponent: `p`
// is divided by `q` while the remainder is 0.
//
// This is equivalent to `fmpz_mod_poly_remove` from `fmpz_mod_poly/remove.c`, FLINT 3.6.0, except
// that Malachite panics where FLINT never returns or assumes the leading coefficient invertible.
fn helper(p: &NaturalPolynomial, q: &NaturalPolynomial, m: &Natural) -> (NaturalPolynomial, u64) {
    assert_reduced(p, q, m);
    if *m == 1u32 {
        return (NaturalPolynomial::ZERO, 0);
    }
    assert_invertible(q, m);
    assert!(
        q.coefficients.len() > 1,
        "the factor must not be a constant"
    );
    let mut p = p.clone();
    let mut e = 0;
    while p.coefficients.len() >= q.coefficients.len() {
        let (s, r) = div_rem_checked_helper(&p, q, m).unwrap();
        if r != NaturalPolynomial::ZERO {
            break;
        }
        p = s;
        e += 1;
    }
    (p, e)
}

impl ModRemovePower<Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Removes the largest power of a [`NaturalPolynomial`] that divides another modulo $m$,
    /// returning the reduced polynomial and the exponent of that power, taking the polynomial by
    /// value, the factor by value, and the modulus by value. The coefficients of both must already
    /// be reduced modulo $m$, and the factor must have degree at least 1 and a leading coefficient
    /// that is invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, e), \quad a \equiv b^e q \pmod{m}, \quad b \nmid q.
    /// $$
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $m$
    /// is 1, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by
    /// [`ModDivRem`](malachite_base::polynomial::ModDivRem), while the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+5*x^3+2*x^2+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// let (s, e) = p.mod_remove_power(q, Natural::from(7u32));
    /// assert_eq!(s.to_string(), "x+2");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_remove` from `fmpz_mod_poly/remove.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT never returns or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_remove_power(self, other: Self, m: Natural) -> (Self, u64) {
        helper(&self, &other, &m)
    }
}

impl ModRemovePower<Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Removes the largest power of a [`NaturalPolynomial`] that divides another modulo $m$,
    /// returning the reduced polynomial and the exponent of that power, taking the polynomial by
    /// value, the factor by value, and the modulus by reference. The coefficients of both must
    /// already be reduced modulo $m$, and the factor must have degree at least 1 and a leading
    /// coefficient that is invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, e), \quad a \equiv b^e q \pmod{m}, \quad b \nmid q.
    /// $$
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $m$
    /// is 1, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by
    /// [`ModDivRem`](malachite_base::polynomial::ModDivRem), while the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+5*x^3+2*x^2+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// let (s, e) = p.mod_remove_power(q, &Natural::from(7u32));
    /// assert_eq!(s.to_string(), "x+2");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_remove` from `fmpz_mod_poly/remove.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT never returns or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_remove_power(self, other: Self, m: &Natural) -> (Self, u64) {
        helper(&self, &other, m)
    }
}

impl ModRemovePower<&Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Removes the largest power of a [`NaturalPolynomial`] that divides another modulo $m$,
    /// returning the reduced polynomial and the exponent of that power, taking the polynomial by
    /// value, the factor by reference, and the modulus by value. The coefficients of both must
    /// already be reduced modulo $m$, and the factor must have degree at least 1 and a leading
    /// coefficient that is invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, e), \quad a \equiv b^e q \pmod{m}, \quad b \nmid q.
    /// $$
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $m$
    /// is 1, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by
    /// [`ModDivRem`](malachite_base::polynomial::ModDivRem), while the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+5*x^3+2*x^2+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// let (s, e) = p.mod_remove_power(&q, Natural::from(7u32));
    /// assert_eq!(s.to_string(), "x+2");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_remove` from `fmpz_mod_poly/remove.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT never returns or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_remove_power(self, other: &Self, m: Natural) -> (Self, u64) {
        helper(&self, other, &m)
    }
}

impl ModRemovePower<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Removes the largest power of a [`NaturalPolynomial`] that divides another modulo $m$,
    /// returning the reduced polynomial and the exponent of that power, taking the polynomial by
    /// value, the factor by reference, and the modulus by reference. The coefficients of both must
    /// already be reduced modulo $m$, and the factor must have degree at least 1 and a leading
    /// coefficient that is invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, e), \quad a \equiv b^e q \pmod{m}, \quad b \nmid q.
    /// $$
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $m$
    /// is 1, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by
    /// [`ModDivRem`](malachite_base::polynomial::ModDivRem), while the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+5*x^3+2*x^2+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// let (s, e) = p.mod_remove_power(&q, &Natural::from(7u32));
    /// assert_eq!(s.to_string(), "x+2");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_remove` from `fmpz_mod_poly/remove.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT never returns or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_remove_power(self, other: &Self, m: &Natural) -> (Self, u64) {
        helper(&self, other, m)
    }
}

impl ModRemovePower<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Removes the largest power of a [`NaturalPolynomial`] that divides another modulo $m$,
    /// returning the reduced polynomial and the exponent of that power, taking the polynomial by
    /// reference, the factor by value, and the modulus by value. The coefficients of both must
    /// already be reduced modulo $m$, and the factor must have degree at least 1 and a leading
    /// coefficient that is invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, e), \quad a \equiv b^e q \pmod{m}, \quad b \nmid q.
    /// $$
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $m$
    /// is 1, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by
    /// [`ModDivRem`](malachite_base::polynomial::ModDivRem), while the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+5*x^3+2*x^2+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// let (s, e) = (&p).mod_remove_power(q, Natural::from(7u32));
    /// assert_eq!(s.to_string(), "x+2");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_remove` from `fmpz_mod_poly/remove.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT never returns or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_remove_power(self, other: NaturalPolynomial, m: Natural) -> (NaturalPolynomial, u64) {
        helper(self, &other, &m)
    }
}

impl ModRemovePower<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Removes the largest power of a [`NaturalPolynomial`] that divides another modulo $m$,
    /// returning the reduced polynomial and the exponent of that power, taking the polynomial by
    /// reference, the factor by value, and the modulus by reference. The coefficients of both must
    /// already be reduced modulo $m$, and the factor must have degree at least 1 and a leading
    /// coefficient that is invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, e), \quad a \equiv b^e q \pmod{m}, \quad b \nmid q.
    /// $$
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $m$
    /// is 1, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by
    /// [`ModDivRem`](malachite_base::polynomial::ModDivRem), while the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+5*x^3+2*x^2+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// let (s, e) = (&p).mod_remove_power(q, &Natural::from(7u32));
    /// assert_eq!(s.to_string(), "x+2");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_remove` from `fmpz_mod_poly/remove.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT never returns or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_remove_power(self, other: NaturalPolynomial, m: &Natural) -> (NaturalPolynomial, u64) {
        helper(self, &other, m)
    }
}

impl ModRemovePower<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Removes the largest power of a [`NaturalPolynomial`] that divides another modulo $m$,
    /// returning the reduced polynomial and the exponent of that power, taking the polynomial by
    /// reference, the factor by reference, and the modulus by value. The coefficients of both must
    /// already be reduced modulo $m$, and the factor must have degree at least 1 and a leading
    /// coefficient that is invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, e), \quad a \equiv b^e q \pmod{m}, \quad b \nmid q.
    /// $$
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $m$
    /// is 1, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by
    /// [`ModDivRem`](malachite_base::polynomial::ModDivRem), while the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+5*x^3+2*x^2+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// let (s, e) = (&p).mod_remove_power(&q, Natural::from(7u32));
    /// assert_eq!(s.to_string(), "x+2");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_remove` from `fmpz_mod_poly/remove.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT never returns or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_remove_power(self, other: &NaturalPolynomial, m: Natural) -> (NaturalPolynomial, u64) {
        helper(self, other, &m)
    }
}

impl ModRemovePower<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Removes the largest power of a [`NaturalPolynomial`] that divides another modulo $m$,
    /// returning the reduced polynomial and the exponent of that power, taking the polynomial by
    /// reference, the factor by reference, and the modulus by reference. The coefficients of both
    /// must already be reduced modulo $m$, and the factor must have degree at least 1 and a leading
    /// coefficient that is invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, e), \quad a \equiv b^e q \pmod{m}, \quad b \nmid q.
    /// $$
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $m$
    /// is 1, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by
    /// [`ModDivRem`](malachite_base::polynomial::ModDivRem), while the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+5*x^3+2*x^2+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// let (s, e) = (&p).mod_remove_power(&q, &Natural::from(7u32));
    /// assert_eq!(s.to_string(), "x+2");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_remove` from `fmpz_mod_poly/remove.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT never returns or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_remove_power(self, other: &NaturalPolynomial, m: &Natural) -> (NaturalPolynomial, u64) {
        helper(self, other, m)
    }
}

impl ModRemovePowerAssign<Self, Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with itself divided by the largest power of another that
    /// divides it modulo $m$, returning the exponent of that power, taking the factor by value, and
    /// the modulus by value. The coefficients of both must already be reduced modulo $m$, and the
    /// factor must have degree at least 1 and a leading coefficient that is invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv b^e q \pmod{m}, \quad b \nmid q,
    /// $$
    /// returning $e$.
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $m$
    /// is 1, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by
    /// [`ModDivRem`](malachite_base::polynomial::ModDivRem), while the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^4+5*x^3+2*x^2+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(p.mod_remove_power_assign(q, Natural::from(7u32)), 3);
    /// assert_eq!(p.to_string(), "x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_remove` from `fmpz_mod_poly/remove.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT never returns or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_remove_power_assign(&mut self, other: Self, m: Natural) -> u64 {
        let (q, e) = helper(self, &other, &m);
        *self = q;
        e
    }
}

impl ModRemovePowerAssign<Self, &Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with itself divided by the largest power of another that
    /// divides it modulo $m$, returning the exponent of that power, taking the factor by value, and
    /// the modulus by reference. The coefficients of both must already be reduced modulo $m$, and
    /// the factor must have degree at least 1 and a leading coefficient that is invertible modulo
    /// $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv b^e q \pmod{m}, \quad b \nmid q,
    /// $$
    /// returning $e$.
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $m$
    /// is 1, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by
    /// [`ModDivRem`](malachite_base::polynomial::ModDivRem), while the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^4+5*x^3+2*x^2+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(p.mod_remove_power_assign(q, &Natural::from(7u32)), 3);
    /// assert_eq!(p.to_string(), "x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_remove` from `fmpz_mod_poly/remove.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT never returns or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_remove_power_assign(&mut self, other: Self, m: &Natural) -> u64 {
        let (q, e) = helper(self, &other, m);
        *self = q;
        e
    }
}

impl ModRemovePowerAssign<&Self, Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with itself divided by the largest power of another that
    /// divides it modulo $m$, returning the exponent of that power, taking the factor by reference,
    /// and the modulus by value. The coefficients of both must already be reduced modulo $m$, and
    /// the factor must have degree at least 1 and a leading coefficient that is invertible modulo
    /// $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv b^e q \pmod{m}, \quad b \nmid q,
    /// $$
    /// returning $e$.
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $m$
    /// is 1, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by
    /// [`ModDivRem`](malachite_base::polynomial::ModDivRem), while the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^4+5*x^3+2*x^2+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(p.mod_remove_power_assign(&q, Natural::from(7u32)), 3);
    /// assert_eq!(p.to_string(), "x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_remove` from `fmpz_mod_poly/remove.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT never returns or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_remove_power_assign(&mut self, other: &Self, m: Natural) -> u64 {
        let (q, e) = helper(self, other, &m);
        *self = q;
        e
    }
}

impl ModRemovePowerAssign<&Self, &Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with itself divided by the largest power of another that
    /// divides it modulo $m$, returning the exponent of that power, taking the factor by reference,
    /// and the modulus by reference. The coefficients of both must already be reduced modulo $m$,
    /// and the factor must have degree at least 1 and a leading coefficient that is invertible
    /// modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv b^e q \pmod{m}, \quad b \nmid q,
    /// $$
    /// returning $e$.
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $m$
    /// is 1, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by
    /// [`ModDivRem`](malachite_base::polynomial::ModDivRem), while the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^4+5*x^3+2*x^2+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(p.mod_remove_power_assign(&q, &Natural::from(7u32)), 3);
    /// assert_eq!(p.to_string(), "x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_remove` from `fmpz_mod_poly/remove.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT never returns or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn mod_remove_power_assign(&mut self, other: &Self, m: &Natural) -> u64 {
        let (q, e) = helper(self, other, m);
        *self = q;
        e
    }
}
