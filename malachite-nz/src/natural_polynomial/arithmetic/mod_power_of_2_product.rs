// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2MulAssign, ModPowerOf2Product,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;

fn assert_reduced(p: &NaturalPolynomial, pow: u64) {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "every polynomial must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
}

// Multiplies the polynomials together one at a time. Once the partial product is zero it stays
// zero, so the factors after the one that made it zero are not examined; this happens more often
// than for numbers, since modulo a power of 2 or a composite number two nonzero polynomials can
// multiply to zero.
fn mod_power_of_2_product_owned<I: Iterator<Item = NaturalPolynomial>>(
    xs: I,
    pow: u64,
) -> NaturalPolynomial {
    // The constant 1, reduced: the zero polynomial when the modulus is 1.
    let mut product = if pow == 0 {
        NaturalPolynomial::ZERO
    } else {
        NaturalPolynomial::one()
    };
    for p in xs {
        assert_reduced(&p, pow);
        product.mod_power_of_2_mul_assign(p, pow);
        if product == NaturalPolynomial::ZERO {
            break;
        }
    }
    product
}

fn mod_power_of_2_product_ref<'a, I: Iterator<Item = &'a NaturalPolynomial>>(
    xs: I,
    pow: u64,
) -> NaturalPolynomial {
    // The constant 1, reduced: the zero polynomial when the modulus is 1.
    let mut product = if pow == 0 {
        NaturalPolynomial::ZERO
    } else {
        NaturalPolynomial::one()
    };
    for p in xs {
        assert_reduced(p, pow);
        product.mod_power_of_2_mul_assign(p, pow);
        if product == NaturalPolynomial::ZERO {
            break;
        }
    }
    product
}

impl ModPowerOf2Product for NaturalPolynomial {
    /// Multiplies together all the [`NaturalPolynomial`]s in an iterator modulo $2^k$, taking the
    /// polynomials by value. The coefficients must already be reduced.
    ///
    /// The product of no polynomials is the constant 1, reduced: the zero polynomial when $2^k =
    /// 1$.
    ///
    /// $f((p_i)_ {i=0}^{n-1}) = q$, where every coefficient of $q$ is less than $2^k$ and $\prod_
    /// {i=0}^{n-1} p_i \equiv q \mod 2^k$, coefficient by coefficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of coefficients of
    /// all the polynomials.
    ///
    /// # Panics
    /// Panics if any coefficient is greater than or equal to $2^k$. As soon as the partial product
    /// is zero, the remaining polynomials are not examined, so an unreduced coefficient after that
    /// point is not detected.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Product;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let ps = ["x+3", "2*x+1", "x^2+5"].map(|s| NaturalPolynomial::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalPolynomial::mod_power_of_2_product(ps.into_iter(), 3).to_string(),
    ///     "2*x^4+7*x^3+5*x^2+3*x+7"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_product<I: Iterator<Item = Self>>(xs: I, pow: u64) -> Self {
        mod_power_of_2_product_owned(xs, pow)
    }
}

impl<'a> ModPowerOf2Product<&'a Self> for NaturalPolynomial {
    /// Multiplies together all the [`NaturalPolynomial`]s in an iterator modulo $2^k$, taking the
    /// polynomials by reference. The coefficients must already be reduced.
    ///
    /// The product of no polynomials is the constant 1, reduced: the zero polynomial when $2^k =
    /// 1$.
    ///
    /// $f((p_i)_ {i=0}^{n-1}) = q$, where every coefficient of $q$ is less than $2^k$ and $\prod_
    /// {i=0}^{n-1} p_i \equiv q \mod 2^k$, coefficient by coefficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of coefficients of
    /// all the polynomials.
    ///
    /// # Panics
    /// Panics if any coefficient is greater than or equal to $2^k$. As soon as the partial product
    /// is zero, the remaining polynomials are not examined, so an unreduced coefficient after that
    /// point is not detected.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Product;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let ps = ["x+3", "2*x+1", "x^2+5"].map(|s| NaturalPolynomial::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalPolynomial::mod_power_of_2_product(ps.iter(), 3).to_string(),
    ///     "2*x^4+7*x^3+5*x^2+3*x+7"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_product<I: Iterator<Item = &'a Self>>(xs: I, pow: u64) -> Self {
        mod_power_of_2_product_ref(xs, pow)
    }
}
