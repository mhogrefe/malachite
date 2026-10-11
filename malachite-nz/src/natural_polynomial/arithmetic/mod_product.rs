// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModMulAssign, ModProduct};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;

fn assert_reduced(p: &NaturalPolynomial, m: &Natural) {
    assert!(
        p.mod_is_reduced(m),
        "every polynomial must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

// Multiplies the polynomials together one at a time. Once the partial product is zero it stays
// zero, so the factors after the one that made it zero are not examined; this happens more often
// than for numbers, since modulo a power of 2 or a composite number two nonzero polynomials can
// multiply to zero.
fn mod_product_owned<I: Iterator<Item = NaturalPolynomial>>(
    xs: I,
    m: &Natural,
) -> NaturalPolynomial {
    assert_ne!(*m, 0u32, "m cannot be zero");
    // The constant 1, reduced: the zero polynomial when the modulus is 1.
    let mut product = if *m == 1u32 {
        NaturalPolynomial::ZERO
    } else {
        NaturalPolynomial::one()
    };
    for p in xs {
        assert_reduced(&p, m);
        product.mod_mul_assign(p, m);
        if product == NaturalPolynomial::ZERO {
            break;
        }
    }
    product
}

fn mod_product_ref<'a, I: Iterator<Item = &'a NaturalPolynomial>>(
    xs: I,
    m: &Natural,
) -> NaturalPolynomial {
    assert_ne!(*m, 0u32, "m cannot be zero");
    // The constant 1, reduced: the zero polynomial when the modulus is 1.
    let mut product = if *m == 1u32 {
        NaturalPolynomial::ZERO
    } else {
        NaturalPolynomial::one()
    };
    for p in xs {
        assert_reduced(p, m);
        product.mod_mul_assign(p, m);
        if product == NaturalPolynomial::ZERO {
            break;
        }
    }
    product
}

impl ModProduct<Self, Natural> for NaturalPolynomial {
    /// Multiplies together all the [`NaturalPolynomial`]s in an iterator modulo another number $m$,
    /// taking the polynomials by value and the modulus by value. The coefficients must already be
    /// reduced.
    ///
    /// The product of no polynomials is the constant 1, reduced: the zero polynomial when $m = 1$.
    ///
    /// $f((p_i)_ {i=0}^{n-1}) = q$, where every coefficient of $q$ is less than $m$ and $\prod_
    /// {i=0}^{n-1} p_i \equiv q \mod m$, coefficient by coefficient.
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
    /// Panics if `m` is zero or if any coefficient is greater than or equal to `m`. As soon as the
    /// partial product is zero, the remaining polynomials are not examined, so an unreduced
    /// coefficient after that point is not detected.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModProduct;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let ps = ["x+3", "2*x+1", "x^2+5"].map(|s| NaturalPolynomial::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalPolynomial::mod_product(ps.into_iter(), Natural::from(9u32)).to_string(),
    ///     "2*x^4+7*x^3+4*x^2+8*x+6"
    /// );
    /// ```
    #[inline]
    fn mod_product<I: Iterator<Item = Self>>(xs: I, m: Natural) -> Self {
        mod_product_owned(xs, &m)
    }
}

impl<'b> ModProduct<Self, &'b Natural> for NaturalPolynomial {
    /// Multiplies together all the [`NaturalPolynomial`]s in an iterator modulo another number $m$,
    /// taking the polynomials by value and the modulus by reference. The coefficients must already
    /// be reduced.
    ///
    /// The product of no polynomials is the constant 1, reduced: the zero polynomial when $m = 1$.
    ///
    /// $f((p_i)_ {i=0}^{n-1}) = q$, where every coefficient of $q$ is less than $m$ and $\prod_
    /// {i=0}^{n-1} p_i \equiv q \mod m$, coefficient by coefficient.
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
    /// Panics if `m` is zero or if any coefficient is greater than or equal to `m`. As soon as the
    /// partial product is zero, the remaining polynomials are not examined, so an unreduced
    /// coefficient after that point is not detected.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModProduct;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let ps = ["x+3", "2*x+1", "x^2+5"].map(|s| NaturalPolynomial::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalPolynomial::mod_product(ps.into_iter(), &Natural::from(9u32)).to_string(),
    ///     "2*x^4+7*x^3+4*x^2+8*x+6"
    /// );
    /// ```
    #[inline]
    fn mod_product<I: Iterator<Item = Self>>(xs: I, m: &'b Natural) -> Self {
        mod_product_owned(xs, m)
    }
}

impl<'a> ModProduct<&'a Self, Natural> for NaturalPolynomial {
    /// Multiplies together all the [`NaturalPolynomial`]s in an iterator modulo another number $m$,
    /// taking the polynomials by reference and the modulus by value. The coefficients must already
    /// be reduced.
    ///
    /// The product of no polynomials is the constant 1, reduced: the zero polynomial when $m = 1$.
    ///
    /// $f((p_i)_ {i=0}^{n-1}) = q$, where every coefficient of $q$ is less than $m$ and $\prod_
    /// {i=0}^{n-1} p_i \equiv q \mod m$, coefficient by coefficient.
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
    /// Panics if `m` is zero or if any coefficient is greater than or equal to `m`. As soon as the
    /// partial product is zero, the remaining polynomials are not examined, so an unreduced
    /// coefficient after that point is not detected.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModProduct;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let ps = ["x+3", "2*x+1", "x^2+5"].map(|s| NaturalPolynomial::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalPolynomial::mod_product(ps.iter(), Natural::from(9u32)).to_string(),
    ///     "2*x^4+7*x^3+4*x^2+8*x+6"
    /// );
    /// ```
    #[inline]
    fn mod_product<I: Iterator<Item = &'a Self>>(xs: I, m: Natural) -> Self {
        mod_product_ref(xs, &m)
    }
}

impl<'a, 'b> ModProduct<&'a Self, &'b Natural> for NaturalPolynomial {
    /// Multiplies together all the [`NaturalPolynomial`]s in an iterator modulo another number $m$,
    /// taking the polynomials by reference and the modulus by reference. The coefficients must
    /// already be reduced.
    ///
    /// The product of no polynomials is the constant 1, reduced: the zero polynomial when $m = 1$.
    ///
    /// $f((p_i)_ {i=0}^{n-1}) = q$, where every coefficient of $q$ is less than $m$ and $\prod_
    /// {i=0}^{n-1} p_i \equiv q \mod m$, coefficient by coefficient.
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
    /// Panics if `m` is zero or if any coefficient is greater than or equal to `m`. As soon as the
    /// partial product is zero, the remaining polynomials are not examined, so an unreduced
    /// coefficient after that point is not detected.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModProduct;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let ps = ["x+3", "2*x+1", "x^2+5"].map(|s| NaturalPolynomial::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalPolynomial::mod_product(ps.iter(), &Natural::from(9u32)).to_string(),
    ///     "2*x^4+7*x^3+4*x^2+8*x+6"
    /// );
    /// ```
    #[inline]
    fn mod_product<I: Iterator<Item = &'a Self>>(xs: I, m: &'b Natural) -> Self {
        mod_product_ref(xs, m)
    }
}
