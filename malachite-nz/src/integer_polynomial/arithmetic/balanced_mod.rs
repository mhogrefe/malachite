// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{BalancedMod, BalancedModAssign};

impl BalancedMod<Integer> for IntegerPolynomial {
    type Output = Self;

    /// Reduces every coefficient of an [`IntegerPolynomial`] modulo an [`Integer`] to the
    /// representative closest to zero, taking the polynomial by value and the modulus by value.
    ///
    /// Each coefficient $r_i$ of the result satisfies $-|m|/2 < r_i \leq |m|/2$ and $r_i \equiv p_i
    /// \bmod m$, which determine it uniquely, as with [`BalancedMod`] for [`Integer`]s. A remainder
    /// of exactly $|m|/2$ is positive, and only the magnitude of $m$ matters.
    ///
    /// Reducing can lower the degree, and can even give the zero polynomial: a leading coefficient
    /// that is a multiple of $m$ becomes zero, and a polynomial does not hold trailing zero
    /// coefficients. So $10x^2 + 7x + 5$ modulo $10$ is $-3x + 5$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedMod;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // Each coefficient goes to the representative closest to zero.
    /// let p = IntegerPolynomial::from_str("x^2+27*x-23").unwrap();
    /// assert_eq!(
    ///     p.clone().balanced_mod(Integer::from(10)).to_string(),
    ///     "x^2-3*x-3"
    /// );
    ///
    /// // Half the modulus stays positive, only the modulus's magnitude matters, and reducing the
    /// // leading coefficient to zero lowers the degree.
    /// let p = IntegerPolynomial::from_str("10*x^2+7*x+5").unwrap();
    /// assert_eq!(
    ///     p.clone().balanced_mod(Integer::from(-10)).to_string(),
    ///     "-3*x+5"
    /// );
    /// ```
    #[inline]
    fn balanced_mod(mut self, m: Integer) -> Self {
        self.balanced_mod_assign(m);
        self
    }
}

impl<'a> BalancedMod<&'a Integer> for IntegerPolynomial {
    type Output = Self;

    /// Reduces every coefficient of an [`IntegerPolynomial`] modulo an [`Integer`] to the
    /// representative closest to zero, taking the polynomial by value and the modulus by reference.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`IntegerPolynomial`] that
    /// takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedMod;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // Each coefficient goes to the representative closest to zero.
    /// let p = IntegerPolynomial::from_str("x^2+27*x-23").unwrap();
    /// assert_eq!(
    ///     p.clone().balanced_mod(&Integer::from(10)).to_string(),
    ///     "x^2-3*x-3"
    /// );
    ///
    /// // Half the modulus stays positive, only the modulus's magnitude matters, and reducing the
    /// // leading coefficient to zero lowers the degree.
    /// let p = IntegerPolynomial::from_str("10*x^2+7*x+5").unwrap();
    /// assert_eq!(
    ///     p.clone().balanced_mod(&Integer::from(-10)).to_string(),
    ///     "-3*x+5"
    /// );
    /// ```
    #[inline]
    fn balanced_mod(mut self, m: &'a Integer) -> Self {
        self.balanced_mod_assign(m);
        self
    }
}

impl BalancedMod<Integer> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Reduces every coefficient of an [`IntegerPolynomial`] modulo an [`Integer`] to the
    /// representative closest to zero, taking the polynomial by reference and the modulus by value.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`IntegerPolynomial`] that
    /// takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedMod;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // Each coefficient goes to the representative closest to zero.
    /// let p = IntegerPolynomial::from_str("x^2+27*x-23").unwrap();
    /// assert_eq!(
    ///     (&p).balanced_mod(Integer::from(10)).to_string(),
    ///     "x^2-3*x-3"
    /// );
    ///
    /// // Half the modulus stays positive, only the modulus's magnitude matters, and reducing the
    /// // leading coefficient to zero lowers the degree.
    /// let p = IntegerPolynomial::from_str("10*x^2+7*x+5").unwrap();
    /// assert_eq!((&p).balanced_mod(Integer::from(-10)).to_string(), "-3*x+5");
    /// ```
    #[inline]
    fn balanced_mod(self, m: Integer) -> IntegerPolynomial {
        self.balanced_mod(&m)
    }
}

impl<'a> BalancedMod<&'a Integer> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Reduces every coefficient of an [`IntegerPolynomial`] modulo an [`Integer`] to the
    /// representative closest to zero, taking the polynomial by reference and the modulus by
    /// reference.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`IntegerPolynomial`] that
    /// takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedMod;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // Each coefficient goes to the representative closest to zero.
    /// let p = IntegerPolynomial::from_str("x^2+27*x-23").unwrap();
    /// assert_eq!(
    ///     (&p).balanced_mod(&Integer::from(10)).to_string(),
    ///     "x^2-3*x-3"
    /// );
    ///
    /// // Half the modulus stays positive, only the modulus's magnitude matters, and reducing the
    /// // leading coefficient to zero lowers the degree.
    /// let p = IntegerPolynomial::from_str("10*x^2+7*x+5").unwrap();
    /// assert_eq!((&p).balanced_mod(&Integer::from(-10)).to_string(), "-3*x+5");
    /// ```
    fn balanced_mod(self, m: &'a Integer) -> IntegerPolynomial {
        assert_ne!(*m, 0u32, "division by zero");
        // `from_coefficients_asc` trims, which is what makes the degree fall when the leading
        // coefficient reduces to zero.
        IntegerPolynomial::from_coefficients_asc(
            self.coefficients
                .iter()
                .map(|c| c.balanced_mod(m))
                .collect::<Vec<_>>(),
        )
    }
}

impl BalancedModAssign<Integer> for IntegerPolynomial {
    /// Reduces every coefficient of an [`IntegerPolynomial`] modulo an [`Integer`] to the
    /// representative closest to zero, in place, taking the modulus by value.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`IntegerPolynomial`] that
    /// takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedModAssign;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2+27*x-23").unwrap();
    /// p.balanced_mod_assign(Integer::from(10));
    /// assert_eq!(p.to_string(), "x^2-3*x-3");
    ///
    /// let mut p = IntegerPolynomial::from_str("10*x^2+7*x+5").unwrap();
    /// p.balanced_mod_assign(Integer::from(-10));
    /// assert_eq!(p.to_string(), "-3*x+5");
    /// ```
    #[inline]
    fn balanced_mod_assign(&mut self, m: Integer) {
        self.balanced_mod_assign(&m);
    }
}

impl<'a> BalancedModAssign<&'a Integer> for IntegerPolynomial {
    /// Reduces every coefficient of an [`IntegerPolynomial`] modulo an [`Integer`] to the
    /// representative closest to zero, in place, taking the modulus by reference.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`IntegerPolynomial`] that
    /// takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedModAssign;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2+27*x-23").unwrap();
    /// p.balanced_mod_assign(&Integer::from(10));
    /// assert_eq!(p.to_string(), "x^2-3*x-3");
    ///
    /// let mut p = IntegerPolynomial::from_str("10*x^2+7*x+5").unwrap();
    /// p.balanced_mod_assign(&Integer::from(-10));
    /// assert_eq!(p.to_string(), "-3*x+5");
    /// ```
    fn balanced_mod_assign(&mut self, m: &'a Integer) {
        assert_ne!(*m, 0u32, "division by zero");
        for c in &mut self.coefficients {
            c.balanced_mod_assign(m);
        }
        self.trim();
    }
}
