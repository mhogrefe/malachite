// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use malachite_base::num::arithmetic::traits::{
    DivIAssign, ModPowerOf2, MulIAssign, MulIPow, MulIPowAssign, NegAssign,
};

impl MulIPow for GaussianInteger {
    type Output = Self;

    /// Multiplies a [`GaussianInteger`] by $i^k$, taking the [`GaussianInteger`] by value.
    ///
    /// Only $k$ modulo 4 matters: $i^0 = 1$, $i^1 = i$, $i^2 = -1$, and $i^3 = -i$, so the result
    /// is the number itself, a counterclockwise quarter turn, a half turn, or a clockwise quarter
    /// turn. Since $i^{-k} = i^{3k}$, a negative power is a matter of tripling the exponent.
    ///
    /// $$
    /// f(x, k) = i^k x.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulIPow;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("2+3i").unwrap();
    /// assert_eq!(x.clone().mul_i_pow(0).to_string(), "2+3i");
    /// assert_eq!(x.clone().mul_i_pow(1).to_string(), "-3+2i");
    /// assert_eq!(x.clone().mul_i_pow(2).to_string(), "-2-3i");
    /// assert_eq!(x.clone().mul_i_pow(3).to_string(), "3-2i");
    /// assert_eq!(x.mul_i_pow(1000000000001).to_string(), "-3+2i");
    /// ```
    #[inline]
    fn mul_i_pow(mut self, k: u64) -> Self {
        self.mul_i_pow_assign(k);
        self
    }
}

impl MulIPow for &GaussianInteger {
    type Output = GaussianInteger;

    /// Multiplies a [`GaussianInteger`] by $i^k$, taking the [`GaussianInteger`] by reference.
    ///
    /// Only $k$ modulo 4 matters: $i^0 = 1$, $i^1 = i$, $i^2 = -1$, and $i^3 = -i$, so the result
    /// is the number itself, a counterclockwise quarter turn, a half turn, or a clockwise quarter
    /// turn. Since $i^{-k} = i^{3k}$, a negative power is a matter of tripling the exponent.
    ///
    /// $$
    /// f(x, k) = i^k x.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulIPow;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("2+3i").unwrap();
    /// assert_eq!((&x).mul_i_pow(0).to_string(), "2+3i");
    /// assert_eq!((&x).mul_i_pow(1).to_string(), "-3+2i");
    /// assert_eq!((&x).mul_i_pow(2).to_string(), "-2-3i");
    /// assert_eq!((&x).mul_i_pow(3).to_string(), "3-2i");
    /// assert_eq!((&x).mul_i_pow(1000000000001).to_string(), "-3+2i");
    /// ```
    #[inline]
    fn mul_i_pow(self, k: u64) -> GaussianInteger {
        self.clone().mul_i_pow(k)
    }
}

impl MulIPowAssign for GaussianInteger {
    /// Multiplies a [`GaussianInteger`] by $i^k$ in place.
    ///
    /// Only $k$ modulo 4 matters: $i^0 = 1$, $i^1 = i$, $i^2 = -1$, and $i^3 = -i$, so the result
    /// is the number itself, a counterclockwise quarter turn, a half turn, or a clockwise quarter
    /// turn. Since $i^{-k} = i^{3k}$, a negative power is a matter of tripling the exponent.
    ///
    /// $$
    /// x \gets i^k x.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulIPowAssign;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let mut x = GaussianInteger::from_str("2+3i").unwrap();
    /// x.mul_i_pow_assign(1);
    /// assert_eq!(x.to_string(), "-3+2i");
    /// x.mul_i_pow_assign(2);
    /// assert_eq!(x.to_string(), "3-2i");
    /// x.mul_i_pow_assign(1000000000001);
    /// assert_eq!(x.to_string(), "2+3i");
    /// ```
    fn mul_i_pow_assign(&mut self, k: u64) {
        match k.mod_power_of_2(2) {
            0 => {}
            1 => self.mul_i_assign(),
            2 => self.neg_assign(),
            _ => self.div_i_assign(),
        }
    }
}
