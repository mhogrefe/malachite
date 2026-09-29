// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModMul, ModMulAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::{ModDerivative, ModDerivativeAssign};

fn assert_reduced(p: &NaturalPolynomial, m: &Natural) {
    assert!(
        p.mod_is_reduced(m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

// Multiplies the coefficient of x^i, for i at least 1, by i mod m, which is kept as a running
// counter so that no index is ever reduced, and moves it to x^(i-1). The products can be zero, so
// the result is trimmed.
fn mod_derivative_ref(p: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
    assert_reduced(p, m);
    let mut i = Natural::ZERO;
    let mut q = NaturalPolynomial {
        coefficients: p
            .coefficients
            .iter()
            .skip(1)
            .map(|c| {
                i += Natural::ONE;
                if i == *m {
                    i = Natural::ZERO;
                }
                c.mod_mul(&i, m)
            })
            .collect(),
    };
    q.trim();
    q
}

fn mod_derivative_in_place(p: &mut NaturalPolynomial, m: &Natural) {
    assert_reduced(p, m);
    if p.coefficients.is_empty() {
        return;
    }
    p.coefficients.remove(0);
    let mut i = Natural::ZERO;
    for c in &mut p.coefficients {
        i += Natural::ONE;
        if i == *m {
            i = Natural::ZERO;
        }
        c.mod_mul_assign(&i, m);
    }
    p.trim();
}

impl ModDerivative<Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the derivative of a [`NaturalPolynomial`] modulo `m`, taking the polynomial by
    /// value and the modulus by value. The coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, m) = p' \bmod m.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by $i$, reduced, and moved to $x^{i-1}$. Since $ia_i$
    /// can be divisible by the modulus even when $a_i$ is not zero, the derivative can lose any
    /// number of degrees. A constant polynomial, including zero, has derivative zero.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(mn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$ is
    /// `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDerivative;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+3*x^2+2*x+5").unwrap();
    /// assert_eq!(p.mod_derivative(Natural::from(6u32)).to_string(), "3*x^2+2");
    ///
    /// // The derivative can lose more than one degree.
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(p.mod_derivative(Natural::from(3u32)).to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_derivative` from `fmpz_mod_poly/derivative.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_derivative(mut self, m: Natural) -> Self {
        mod_derivative_in_place(&mut self, &m);
        self
    }
}

impl ModDerivative<&Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the derivative of a [`NaturalPolynomial`] modulo `m`, taking the polynomial by
    /// value and the modulus by reference. The coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, m) = p' \bmod m.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by $i$, reduced, and moved to $x^{i-1}$. Since $ia_i$
    /// can be divisible by the modulus even when $a_i$ is not zero, the derivative can lose any
    /// number of degrees. A constant polynomial, including zero, has derivative zero.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(mn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$ is
    /// `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDerivative;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+3*x^2+2*x+5").unwrap();
    /// assert_eq!(
    ///     p.mod_derivative(&Natural::from(6u32)).to_string(),
    ///     "3*x^2+2"
    /// );
    ///
    /// // The derivative can lose more than one degree.
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(p.mod_derivative(&Natural::from(3u32)).to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_derivative` from `fmpz_mod_poly/derivative.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_derivative(mut self, m: &Natural) -> Self {
        mod_derivative_in_place(&mut self, m);
        self
    }
}

impl ModDerivative<Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the derivative of a [`NaturalPolynomial`] modulo `m`, taking the polynomial by
    /// reference and the modulus by value. The coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, m) = p' \bmod m.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by $i$, reduced, and moved to $x^{i-1}$. Since $ia_i$
    /// can be divisible by the modulus even when $a_i$ is not zero, the derivative can lose any
    /// number of degrees. A constant polynomial, including zero, has derivative zero.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(mn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$ is
    /// `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDerivative;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+3*x^2+2*x+5").unwrap();
    /// assert_eq!(
    ///     (&p).mod_derivative(Natural::from(6u32)).to_string(),
    ///     "3*x^2+2"
    /// );
    ///
    /// // The derivative can lose more than one degree.
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!((&p).mod_derivative(Natural::from(3u32)).to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_derivative` from `fmpz_mod_poly/derivative.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_derivative(self, m: Natural) -> NaturalPolynomial {
        mod_derivative_ref(self, &m)
    }
}

impl ModDerivative<&Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the derivative of a [`NaturalPolynomial`] modulo `m`, taking the polynomial by
    /// reference and the modulus by reference. The coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, m) = p' \bmod m.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by $i$, reduced, and moved to $x^{i-1}$. Since $ia_i$
    /// can be divisible by the modulus even when $a_i$ is not zero, the derivative can lose any
    /// number of degrees. A constant polynomial, including zero, has derivative zero.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(mn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$ is
    /// `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDerivative;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+3*x^2+2*x+5").unwrap();
    /// assert_eq!(
    ///     (&p).mod_derivative(&Natural::from(6u32)).to_string(),
    ///     "3*x^2+2"
    /// );
    ///
    /// // The derivative can lose more than one degree.
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!((&p).mod_derivative(&Natural::from(3u32)).to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_derivative` from `fmpz_mod_poly/derivative.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_derivative(self, m: &Natural) -> NaturalPolynomial {
        mod_derivative_ref(self, m)
    }
}

impl ModDerivativeAssign<Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its derivative modulo `m`, in place, taking the
    /// modulus by value. The coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets p' \bmod m.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by $i$, reduced, and moved to $x^{i-1}$. Since $ia_i$
    /// can be divisible by the modulus even when $a_i$ is not zero, the derivative can lose any
    /// number of degrees. A constant polynomial, including zero, has derivative zero.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(mn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$ is
    /// `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDerivativeAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+3*x^2+2*x+5").unwrap();
    /// p.mod_derivative_assign(Natural::from(6u32));
    /// assert_eq!(p.to_string(), "3*x^2+2");
    ///
    /// // The derivative can lose more than one degree.
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// p.mod_derivative_assign(Natural::from(3u32));
    /// assert_eq!(p.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_derivative` from `fmpz_mod_poly/derivative.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_derivative_assign(&mut self, m: Natural) {
        mod_derivative_in_place(self, &m);
    }
}

impl ModDerivativeAssign<&Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its derivative modulo `m`, in place, taking the
    /// modulus by reference. The coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets p' \bmod m.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by $i$, reduced, and moved to $x^{i-1}$. Since $ia_i$
    /// can be divisible by the modulus even when $a_i$ is not zero, the derivative can lose any
    /// number of degrees. A constant polynomial, including zero, has derivative zero.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(mn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$ is
    /// `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDerivativeAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+3*x^2+2*x+5").unwrap();
    /// p.mod_derivative_assign(&Natural::from(6u32));
    /// assert_eq!(p.to_string(), "3*x^2+2");
    ///
    /// // The derivative can lose more than one degree.
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// p.mod_derivative_assign(&Natural::from(3u32));
    /// assert_eq!(p.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_derivative` from `fmpz_mod_poly/derivative.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_derivative_assign(&mut self, m: &Natural) {
        mod_derivative_in_place(self, m);
    }
}
