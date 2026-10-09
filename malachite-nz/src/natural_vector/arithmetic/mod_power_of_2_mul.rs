// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use alloc::vec;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2MulAssign,
};
use malachite_base::num::basic::traits::{One, Zero};

fn assert_reduced(v: &NaturalVector, c: &Natural, pow: u64) {
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {v} has an element >= 2^{pow}"
    );
    assert!(
        c.mod_power_of_2_is_reduced(pow),
        "c must be reduced mod 2^pow, but {c} >= 2^{pow}"
    );
}

impl ModPowerOf2Mul<Natural> for NaturalVector {
    type Output = Self;

    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`] modulo $2^k$, taking the
    /// vector by value and the scalar by value. The elements and the scalar must already be reduced
    /// modulo $2^k$.
    ///
    /// Each product is reduced modulo $2^k$, and the result has the same dimension as the vector.
    ///
    /// $$
    /// f(v, c, k) = cv \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of `self`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_mul(Natural::from(3u32), 3).to_string(),
    ///     "(7, 3, 1)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_mul(mut self, c: Natural, pow: u64) -> Self {
        self.mod_power_of_2_mul_assign(&c, pow);
        self
    }
}

impl ModPowerOf2Mul<&Natural> for NaturalVector {
    type Output = Self;

    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`] modulo $2^k$, taking the
    /// vector by value and the scalar by reference. The elements and the scalar must already be
    /// reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2Mul`] implementation that takes the vector and
    /// the scalar by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of `self`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_mul(&Natural::from(3u32), 3).to_string(),
    ///     "(7, 3, 1)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_mul(mut self, c: &Natural, pow: u64) -> Self {
        self.mod_power_of_2_mul_assign(c, pow);
        self
    }
}

impl ModPowerOf2Mul<Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`] modulo $2^k$, taking the
    /// vector by reference and the scalar by value. The elements and the scalar must already be
    /// reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2Mul`] implementation that takes the vector and
    /// the scalar by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of `self`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_power_of_2_mul(Natural::from(3u32), 3).to_string(),
    ///     "(7, 3, 1)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, c: Natural, pow: u64) -> NaturalVector {
        self.mod_power_of_2_mul(&c, pow)
    }
}

impl ModPowerOf2Mul<&Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`] modulo $2^k$, taking the
    /// vector by reference and the scalar by reference. The elements and the scalar must already be
    /// reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2Mul`] implementation that takes the vector and
    /// the scalar by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of `self`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_power_of_2_mul(&Natural::from(3u32), 3).to_string(),
    ///     "(7, 3, 1)"
    /// );
    /// ```
    fn mod_power_of_2_mul(self, c: &Natural, pow: u64) -> NaturalVector {
        assert_reduced(self, c, pow);
        NaturalVector {
            elements: match *c {
                Natural::ZERO => vec![Natural::ZERO; self.elements.len()],
                Natural::ONE => self.elements.clone(),
                _ => self
                    .elements
                    .iter()
                    .map(|x| x.mod_power_of_2_mul(c, pow))
                    .collect(),
            },
        }
    }
}

impl ModPowerOf2MulAssign<Natural> for NaturalVector {
    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`] modulo $2^k$, in place,
    /// taking the scalar by value. The elements and the scalar must already be reduced modulo
    /// $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2Mul`] implementation that takes the vector and
    /// the scalar by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of `self`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// v.mod_power_of_2_mul_assign(Natural::from(3u32), 3);
    /// assert_eq!(v.to_string(), "(7, 3, 1)");
    /// ```
    #[inline]
    fn mod_power_of_2_mul_assign(&mut self, c: Natural, pow: u64) {
        self.mod_power_of_2_mul_assign(&c, pow);
    }
}

impl ModPowerOf2MulAssign<&Natural> for NaturalVector {
    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`] modulo $2^k$, in place,
    /// taking the scalar by reference. The elements and the scalar must already be reduced modulo
    /// $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2Mul`] implementation that takes the vector and
    /// the scalar by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of `self`, or `c`, is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// v.mod_power_of_2_mul_assign(&Natural::from(3u32), 3);
    /// assert_eq!(v.to_string(), "(7, 3, 1)");
    /// ```
    fn mod_power_of_2_mul_assign(&mut self, c: &Natural, pow: u64) {
        assert_reduced(self, c, pow);
        match *c {
            Natural::ZERO => self.elements.fill(Natural::ZERO),
            Natural::ONE => {}
            _ => {
                for x in &mut self.elements {
                    x.mod_power_of_2_mul_assign(c, pow);
                }
            }
        }
    }
}
