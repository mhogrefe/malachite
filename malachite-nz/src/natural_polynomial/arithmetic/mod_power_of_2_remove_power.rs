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

use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_power_of_2_div_exact::assert_invertible;
use crate::natural_polynomial::arithmetic::mod_power_of_2_div_rem::{
    assert_reduced, checked_helper as dr_helper,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{ModPowerOf2RemovePower, ModPowerOf2RemovePowerAssign};

// The largest power of `q` that divides `p` modulo $2^k$, where $k$ is `pow`, divided out of `p`,
// and its exponent: `p` is divided by `q` while the remainder is 0.
//
// This is equivalent to `fmpz_mod_poly_remove` from `fmpz_mod_poly/remove.c`, FLINT 3.6.0, with the
// modulus $2^k$, except that Malachite panics where FLINT never returns or assumes the leading
// coefficient invertible.
fn helper(p: &NaturalPolynomial, q: &NaturalPolynomial, pow: u64) -> (NaturalPolynomial, u64) {
    assert_reduced(p, q, pow);
    if pow == 0 {
        return (NaturalPolynomial::ZERO, 0);
    }
    assert_invertible(q, pow);
    assert!(
        q.coefficients.len() > 1,
        "the factor must not be a constant"
    );
    let mut p = p.clone();
    let mut e = 0;
    while p.coefficients.len() >= q.coefficients.len() {
        let (s, r) = dr_helper(&p, q, pow).unwrap();
        if r != NaturalPolynomial::ZERO {
            break;
        }
        p = s;
        e += 1;
    }
    (p, e)
}

impl ModPowerOf2RemovePower<Self> for NaturalPolynomial {
    type Output = Self;

    /// Removes the largest power of a [`NaturalPolynomial`] that divides another modulo $2^k$,
    /// returning the reduced polynomial and the exponent of that power, taking the polynomial by
    /// value and the factor by value. The coefficients of both must already be reduced modulo
    /// $2^k$, and the factor must have degree at least 1 and a leading coefficient that is odd.
    ///
    /// $$
    /// f(a, b, k) = (q, e), \quad a \equiv b^e q \pmod{2^k}, \quad b \nmid q.
    /// $$
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $k$
    /// is 0, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by
    /// [`ModPowerOf2DivRem`](malachite_base::polynomial::ModPowerOf2DivRem), while the remainder is
    /// 0.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if,
    /// unless $k$ is 0, `other` is 0 or a constant or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2RemovePower;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+5*x^3+x^2+7*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// let (s, e) = p.mod_power_of_2_remove_power(q, 3);
    /// assert_eq!(s.to_string(), "x+2");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_remove` from `fmpz_mod_poly/remove.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT never returns or assumes
    /// the leading coefficient invertible.
    #[inline]
    fn mod_power_of_2_remove_power(self, other: Self, pow: u64) -> (Self, u64) {
        helper(&self, &other, pow)
    }
}

impl ModPowerOf2RemovePower<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Removes the largest power of a [`NaturalPolynomial`] that divides another modulo $2^k$,
    /// returning the reduced polynomial and the exponent of that power, taking the polynomial by
    /// value and the factor by reference. The coefficients of both must already be reduced modulo
    /// $2^k$, and the factor must have degree at least 1 and a leading coefficient that is odd.
    ///
    /// $$
    /// f(a, b, k) = (q, e), \quad a \equiv b^e q \pmod{2^k}, \quad b \nmid q.
    /// $$
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $k$
    /// is 0, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by
    /// [`ModPowerOf2DivRem`](malachite_base::polynomial::ModPowerOf2DivRem), while the remainder is
    /// 0.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if,
    /// unless $k$ is 0, `other` is 0 or a constant or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2RemovePower;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+5*x^3+x^2+7*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// let (s, e) = p.mod_power_of_2_remove_power(&q, 3);
    /// assert_eq!(s.to_string(), "x+2");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_remove` from `fmpz_mod_poly/remove.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT never returns or assumes
    /// the leading coefficient invertible.
    #[inline]
    fn mod_power_of_2_remove_power(self, other: &Self, pow: u64) -> (Self, u64) {
        helper(&self, other, pow)
    }
}

impl ModPowerOf2RemovePower<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Removes the largest power of a [`NaturalPolynomial`] that divides another modulo $2^k$,
    /// returning the reduced polynomial and the exponent of that power, taking the polynomial by
    /// reference and the factor by value. The coefficients of both must already be reduced modulo
    /// $2^k$, and the factor must have degree at least 1 and a leading coefficient that is odd.
    ///
    /// $$
    /// f(a, b, k) = (q, e), \quad a \equiv b^e q \pmod{2^k}, \quad b \nmid q.
    /// $$
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $k$
    /// is 0, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by
    /// [`ModPowerOf2DivRem`](malachite_base::polynomial::ModPowerOf2DivRem), while the remainder is
    /// 0.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if,
    /// unless $k$ is 0, `other` is 0 or a constant or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2RemovePower;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+5*x^3+x^2+7*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// let (s, e) = (&p).mod_power_of_2_remove_power(q, 3);
    /// assert_eq!(s.to_string(), "x+2");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_remove` from `fmpz_mod_poly/remove.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT never returns or assumes
    /// the leading coefficient invertible.
    #[inline]
    fn mod_power_of_2_remove_power(
        self,
        other: NaturalPolynomial,
        pow: u64,
    ) -> (NaturalPolynomial, u64) {
        helper(self, &other, pow)
    }
}

impl ModPowerOf2RemovePower<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Removes the largest power of a [`NaturalPolynomial`] that divides another modulo $2^k$,
    /// returning the reduced polynomial and the exponent of that power, taking the polynomial by
    /// reference and the factor by reference. The coefficients of both must already be reduced
    /// modulo $2^k$, and the factor must have degree at least 1 and a leading coefficient that is
    /// odd.
    ///
    /// $$
    /// f(a, b, k) = (q, e), \quad a \equiv b^e q \pmod{2^k}, \quad b \nmid q.
    /// $$
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $k$
    /// is 0, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by
    /// [`ModPowerOf2DivRem`](malachite_base::polynomial::ModPowerOf2DivRem), while the remainder is
    /// 0.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if,
    /// unless $k$ is 0, `other` is 0 or a constant or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2RemovePower;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+5*x^3+x^2+7*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// let (s, e) = (&p).mod_power_of_2_remove_power(&q, 3);
    /// assert_eq!(s.to_string(), "x+2");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_remove` from `fmpz_mod_poly/remove.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT never returns or assumes
    /// the leading coefficient invertible.
    #[inline]
    fn mod_power_of_2_remove_power(
        self,
        other: &NaturalPolynomial,
        pow: u64,
    ) -> (NaturalPolynomial, u64) {
        helper(self, other, pow)
    }
}

impl ModPowerOf2RemovePowerAssign<Self> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with itself divided by the largest power of another that
    /// divides it modulo $2^k$, returning the exponent of that power, taking the factor by value.
    /// The coefficients of both must already be reduced modulo $2^k$, and the factor must have
    /// degree at least 1 and a leading coefficient that is odd.
    ///
    /// $$
    /// a \gets q, \quad a \equiv b^e q \pmod{2^k}, \quad b \nmid q,
    /// $$
    /// returning $e$.
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $k$
    /// is 0, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by
    /// [`ModPowerOf2DivRem`](malachite_base::polynomial::ModPowerOf2DivRem), while the remainder is
    /// 0.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if,
    /// unless $k$ is 0, `other` is 0 or a constant or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2RemovePowerAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^4+5*x^3+x^2+7*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(p.mod_power_of_2_remove_power_assign(q, 3), 3);
    /// assert_eq!(p.to_string(), "x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_remove` from `fmpz_mod_poly/remove.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT never returns or assumes
    /// the leading coefficient invertible.
    #[inline]
    fn mod_power_of_2_remove_power_assign(&mut self, other: Self, pow: u64) -> u64 {
        let (q, e) = helper(self, &other, pow);
        *self = q;
        e
    }
}

impl ModPowerOf2RemovePowerAssign<&Self> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with itself divided by the largest power of another that
    /// divides it modulo $2^k$, returning the exponent of that power, taking the factor by
    /// reference. The coefficients of both must already be reduced modulo $2^k$, and the factor
    /// must have degree at least 1 and a leading coefficient that is odd.
    ///
    /// $$
    /// a \gets q, \quad a \equiv b^e q \pmod{2^k}, \quad b \nmid q,
    /// $$
    /// returning $e$.
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. When $k$
    /// is 0, every polynomial is 0, and so is the result.
    ///
    /// The polynomial is divided by the factor, as by
    /// [`ModPowerOf2DivRem`](malachite_base::polynomial::ModPowerOf2DivRem), while the remainder is
    /// 0.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if,
    /// unless $k$ is 0, `other` is 0 or a constant or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2RemovePowerAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^4+5*x^3+x^2+7*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(p.mod_power_of_2_remove_power_assign(&q, 3), 3);
    /// assert_eq!(p.to_string(), "x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_remove` from `fmpz_mod_poly/remove.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT never returns or assumes
    /// the leading coefficient invertible.
    #[inline]
    fn mod_power_of_2_remove_power_assign(&mut self, other: &Self, pow: u64) -> u64 {
        let (q, e) = helper(self, other, pow);
        *self = q;
        e
    }
}
