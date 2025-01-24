// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{Parity, Pow, PowAssign};

impl Pow<u64> for Integer {
    type Output = Integer;

    /// Raises an [`Integer`] to a power, taking the [`Integer`] by value.
    ///
    /// $f(x, n) = x^n$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `exp`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(-3).pow(100).to_string(),
    ///     "515377520732011331036461129765621272702107522001"
    /// );
    /// assert_eq!(
    ///     Integer::from_str("-12345678987654321")
    ///         .unwrap()
    ///         .pow(3)
    ///         .to_string(),
    ///     "-1881676411868862234942354805142998028003108518161"
    /// );
    /// ```
    #[inline]
    fn pow(mut self, exp: u64) -> Integer {
        self.pow_assign(exp);
        self
    }
}

impl Pow<u64> for &Integer {
    type Output = Integer;

    /// Raises an [`Integer`] to a power, taking the [`Integer`] by reference.
    ///
    /// $f(x, n) = x^n$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `exp`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(-3)).pow(100).to_string(),
    ///     "515377520732011331036461129765621272702107522001"
    /// );
    /// assert_eq!(
    ///     (&Integer::from_str("-12345678987654321").unwrap())
    ///         .pow(3)
    ///         .to_string(),
    ///     "-1881676411868862234942354805142998028003108518161"
    /// );
    /// ```
    #[inline]
    fn pow(self, exp: u64) -> Integer {
        Integer {
            sign: exp.even() || self.sign,
            abs: (&self.abs).pow(exp),
        }
    }
}

impl PowAssign<u64> for Integer {
    /// Raises an [`Integer`] to a power in place.
    ///
    /// $x \gets x^n$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `exp`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::PowAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-3);
    /// x.pow_assign(100);
    /// assert_eq!(
    ///     x.to_string(),
    ///     "515377520732011331036461129765621272702107522001"
    /// );
    ///
    /// let mut x = Integer::from_str("-12345678987654321").unwrap();
    /// x.pow_assign(3);
    /// assert_eq!(
    ///     x.to_string(),
    ///     "-1881676411868862234942354805142998028003108518161"
    /// );
    /// ```
    fn pow_assign(&mut self, exp: u64) {
        self.sign = self.sign || exp.even();
        self.abs.pow_assign(exp);
    }
}
