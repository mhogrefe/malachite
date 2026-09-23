// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{Mod, ModAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec::Vec;
use core::ops::{Rem, RemAssign};

impl<T: PrimitiveUnsigned> Rem<T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Divides every coefficient of a [`UnsignedPolynomial`] by a [`u64`], keeping the remainders,
    /// taking the polynomial by value.
    ///
    /// $p \\% m$ is the polynomial whose $i$th coefficient is $p_i \\% m$, and this is the
    /// remainder of dividing $p$ by the constant polynomial $m$ — under the convention that
    /// applies over the integers, where a remainder is bounded coefficient by coefficient rather
    /// than by degree. Over a field the answer would be different: there a remainder must have
    /// lower degree than the divisor, and a nonzero constant has degree 0, so dividing by one
    /// leaves a remainder of 0. The `T`s are not a field, division by $m$ is not exact, and
    /// bounding the remainder's coefficients is what is left.
    ///
    /// There is a quotient to go with it: the polynomial whose $i$th coefficient is $p_i / m$, for
    /// which $p = mq + r$ holds exactly.
    ///
    /// The result is reduced modulo $m$, which is to say that
    /// [`mod_is_reduced`](crate::num::arithmetic::traits::ModIsReduced::mod_is_reduced) returns
    /// `true` for it.
    ///
    /// Reducing can lower the degree, and can even give the zero polynomial: a leading coefficient
    /// that is a multiple of $m$ becomes zero, and a polynomial does not hold trailing zero
    /// coefficients. So $4x^2 + 3$ modulo $4$ is the constant $3$, not a quadratic with a zero
    /// leading coefficient.
    ///
    /// $$
    /// f(p, m) = q, \\quad \text{where} \\quad q_i = p_i - m \left \lfloor \frac{p_i}{m}
    /// \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // Every coefficient is taken modulo 3.
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u64>::from_str("x^2+4*x+5").unwrap() % 3).to_string(),
    ///     "x^2+x+2"
    /// );
    ///
    /// // Reducing the leading coefficient to zero lowers the degree.
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u64>::from_str("4*x^2+3").unwrap() % 4).to_string(),
    ///     "3"
    /// );
    ///
    /// // Modulo 1 every coefficient is zero, so the whole polynomial is.
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u64>::from_str("x^2+4*x+5").unwrap() % 1).to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_scalar_mod_fmpz` from `fmpz_poly/scalar_mod_fmpz.c`, FLINT
    /// 3.6.0, for a polynomial whose coefficients are all nonnegative.
    #[inline]
    fn rem(mut self, m: T) -> Self {
        self %= m;
        self
    }
}

impl<T: PrimitiveUnsigned> Rem<T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Divides every coefficient of a [`UnsignedPolynomial`] by a [`u64`], keeping the remainders,
    /// taking the polynomial by reference.
    ///
    /// See the documentation for the [`Rem`] implementation on [`UnsignedPolynomial`] for details,
    /// including how reducing can lower the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u64>::from_str("4*x^2+3").unwrap();
    /// assert_eq!((&p % 4).to_string(), "3");
    /// // The polynomial is left alone.
    /// assert_eq!(p.to_string(), "4*x^2+3");
    /// ```
    #[inline]
    fn rem(self, m: T) -> UnsignedPolynomial<T> {
        assert_ne!(m, T::ZERO, "division by zero");
        // `from_coefficients_asc` trims, which is what makes the degree fall when the leading
        // coefficient reduces to zero.
        UnsignedPolynomial::from_coefficients_asc(
            self.coefficients_asc()
                .iter()
                .map(|&c| c % m)
                .collect::<Vec<_>>(),
        )
    }
}

impl<T: PrimitiveUnsigned> RemAssign<T> for UnsignedPolynomial<T> {
    /// Divides every coefficient of a [`UnsignedPolynomial`] by a [`u64`], replacing the polynomial
    /// by the one whose coefficients are the remainders.
    ///
    /// See the documentation for the [`Rem`] implementation on [`UnsignedPolynomial`] for details,
    /// including how reducing can lower the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u64>::from_str("x^2+4*x+5").unwrap();
    /// p %= 3;
    /// assert_eq!(p.to_string(), "x^2+x+2");
    ///
    /// let mut p = UnsignedPolynomial::<u64>::from_str("4*x^2+3").unwrap();
    /// p %= 4;
    /// assert_eq!(p.to_string(), "3");
    /// ```
    fn rem_assign(&mut self, m: T) {
        assert_ne!(m, T::ZERO, "division by zero");
        for c in &mut self.coefficients {
            *c %= m;
        }
        self.trim();
    }
}

impl<T: PrimitiveUnsigned> Mod<T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Divides every coefficient of a [`UnsignedPolynomial`] by a [`u64`], keeping the remainders,
    /// taking the polynomial by value.
    ///
    /// A [`UnsignedPolynomial`]'s coefficients are never negative, so there is nothing for this to
    /// do that `%` does not: the two agree everywhere, and this is the same operation under the
    /// name the mod-family traits use. See the documentation for the [`Rem`] implementation for
    /// details, including how reducing can lower the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("x^2+4*x+5")
    ///         .unwrap()
    ///         .mod_op(3)
    ///         .to_string(),
    ///     "x^2+x+2"
    /// );
    ///
    /// // Reducing the leading coefficient to zero lowers the degree.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("4*x^2+3")
    ///         .unwrap()
    ///         .mod_op(4)
    ///         .to_string(),
    ///     "3"
    /// );
    /// ```
    #[inline]
    fn mod_op(self, m: T) -> Self {
        self % m
    }
}

impl<T: PrimitiveUnsigned> Mod<T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Divides every coefficient of a [`UnsignedPolynomial`] by a [`u64`], keeping the remainders,
    /// taking the polynomial by reference.
    ///
    /// This agrees with `%` everywhere, a [`UnsignedPolynomial`]'s coefficients never being
    /// negative. See the documentation for the [`Rem`] implementation on [`UnsignedPolynomial`] for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u64>::from_str("4*x^2+3").unwrap();
    /// assert_eq!((&p).mod_op(4).to_string(), "3");
    /// // The polynomial is left alone.
    /// assert_eq!(p.to_string(), "4*x^2+3");
    /// ```
    #[inline]
    fn mod_op(self, m: T) -> UnsignedPolynomial<T> {
        self % m
    }
}

impl<T: PrimitiveUnsigned> ModAssign<T> for UnsignedPolynomial<T> {
    /// Divides every coefficient of a [`UnsignedPolynomial`] by a [`u64`], replacing the polynomial
    /// by the one whose coefficients are the remainders.
    ///
    /// This agrees with `%=` everywhere, a [`UnsignedPolynomial`]'s coefficients never being
    /// negative. See the documentation for the [`Rem`] implementation on [`UnsignedPolynomial`] for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u64>::from_str("x^2+4*x+5").unwrap();
    /// p.mod_assign(3);
    /// assert_eq!(p.to_string(), "x^2+x+2");
    /// ```
    #[inline]
    fn mod_assign(&mut self, m: T) {
        *self %= m;
    }
}
