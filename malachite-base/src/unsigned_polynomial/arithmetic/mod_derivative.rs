// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModIsReduced;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{ModDerivative, ModDerivativeAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;

fn assert_reduced<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, m: T) {
    assert!(
        p.mod_is_reduced(&m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

// Multiplies the coefficient of x^i, for i at least 1, by i mod m, which is kept as a running
// counter so that no index needs converting to `T`, and moves it to x^(i-1). The products can be
// zero, so the result is trimmed.
fn mod_derivative_ref<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    assert_reduced(p, m);
    let mut i = T::ZERO;
    let mut q = UnsignedPolynomial {
        coefficients: p
            .coefficients
            .iter()
            .skip(1)
            .map(|&c| {
                i += T::ONE;
                if i == m {
                    i = T::ZERO;
                }
                c.mod_mul(i, m)
            })
            .collect(),
    };
    q.trim();
    q
}

fn mod_derivative_in_place<T: PrimitiveUnsigned>(p: &mut UnsignedPolynomial<T>, m: T) {
    assert_reduced(p, m);
    if p.coefficients.is_empty() {
        return;
    }
    p.coefficients.remove(0);
    let mut i = T::ZERO;
    for c in &mut p.coefficients {
        i += T::ONE;
        if i == m {
            i = T::ZERO;
        }
        c.mod_mul_assign(i, m);
    }
    p.trim();
}

impl<T: PrimitiveUnsigned> ModDerivative<T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Computes the derivative of an [`UnsignedPolynomial`] modulo `m`, taking the polynomial by
    /// value. The coefficients must already be reduced modulo `m`.
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
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDerivative;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+2*x+5").unwrap();
    /// assert_eq!(p.mod_derivative(6).to_string(), "3*x^2+2");
    ///
    /// // The derivative can lose more than one degree.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(p.mod_derivative(3).to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_derivative` from `nmod_poly/derivative.c`, FLINT 3.6.0.
    #[inline]
    fn mod_derivative(mut self, m: T) -> Self {
        mod_derivative_in_place(&mut self, m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModDerivative<T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the derivative of an [`UnsignedPolynomial`] modulo `m`, taking the polynomial by
    /// reference. The coefficients must already be reduced modulo `m`.
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
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDerivative;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+2*x+5").unwrap();
    /// assert_eq!((&p).mod_derivative(6).to_string(), "3*x^2+2");
    ///
    /// // The derivative can lose more than one degree.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!((&p).mod_derivative(3).to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_derivative` from `nmod_poly/derivative.c`, FLINT 3.6.0.
    #[inline]
    fn mod_derivative(self, m: T) -> UnsignedPolynomial<T> {
        mod_derivative_ref(self, m)
    }
}

impl<T: PrimitiveUnsigned> ModDerivativeAssign<T> for UnsignedPolynomial<T> {
    /// Replaces an [`UnsignedPolynomial`] with its derivative modulo `m`, in place. The
    /// coefficients must already be reduced modulo `m`.
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
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDerivativeAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+2*x+5").unwrap();
    /// p.mod_derivative_assign(6);
    /// assert_eq!(p.to_string(), "3*x^2+2");
    ///
    /// // The derivative can lose more than one degree.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// p.mod_derivative_assign(3);
    /// assert_eq!(p.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_derivative` from `nmod_poly/derivative.c`, FLINT 3.6.0.
    #[inline]
    fn mod_derivative_assign(&mut self, m: T) {
        mod_derivative_in_place(self, m);
    }
}
