// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::rational_polynomial::RationalPolynomial;
use alloc::vec::Vec;
use core::mem::{swap, take};
use core::ops::{Add, AddAssign};
use core::ptr;
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign, Gcd, Parity};
use malachite_base::num::basic::traits::One;
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::content_chained::integers_content_chained;
use malachite_nz::integer_polynomial::arithmetic::scalar_add_mul::integers_add_mul_scalar_assign;
use malachite_nz::integer_polynomial::arithmetic::scalar_mul::integers_mul_scalar_assign;
use malachite_nz::natural::Natural;

// Adds $y/b$ to $x/a$, where both pairs are canonical, reusing the storage of $x$, and returns the
// canonical sum.
//
// With $g = \gcd(a, b)$, the sum is $(x(b/g) + y(a/g))/(ab/g)$. Any common factor of that numerator
// and denominator divides $g$: a prime dividing $a/g$ but not $b/g$ divides $y(a/g)$ but not
// $x(b/g)$, since it does not divide the content of $x$. So only the GCD of the numerator's content
// and $g$ needs to be divided out, and when $g = 1$ nothing does.
//
// This is equivalent to `_fmpq_poly_add_can` from `fmpq_poly/add.c`, FLINT 3.6.0, where `can` is 1.
fn add_owned_ref(
    x: IntegerPolynomial,
    a: Natural,
    y: &IntegerPolynomial,
    b: &Natural,
) -> RationalPolynomial {
    if a == *b {
        let mut numerator = x + y;
        let mut denominator = a;
        if denominator != 1u32 {
            // A zero sum reduces to 0/1 here, since the content of zero is taken to be 0.
            let g = integers_content_chained(numerator.coefficients_asc(), &denominator);
            if g != 1u32 {
                denominator.div_exact_assign(&g);
                numerator.div_exact_assign(Integer::from(g));
            }
        }
        return RationalPolynomial {
            numerator,
            denominator,
        };
    }
    // When one denominator is 1, their GCD is 1 without computing it.
    let g = if a != 1u32 && *b != 1u32 {
        (&a).gcd(b)
    } else {
        Natural::ONE
    };
    let mut xs = x.into_coefficients_asc();
    if g == 1u32 {
        integers_mul_scalar_assign(&mut xs, &Integer::from(b));
        let denominator = &a * b;
        integers_add_mul_scalar_assign(&mut xs, y.coefficients_asc(), &Integer::from(a));
        return RationalPolynomial {
            numerator: IntegerPolynomial::from_coefficients_asc(xs),
            denominator,
        };
    }
    let a_over_g = Integer::from((&a).div_exact(&g));
    let b_over_g = b.div_exact(&g);
    integers_mul_scalar_assign(&mut xs, &Integer::from(&b_over_g));
    integers_add_mul_scalar_assign(&mut xs, y.coefficients_asc(), &a_over_g);
    // The sum is not zero, since canonical polynomials with different denominators cannot cancel.
    let e = integers_content_chained(&xs, &g);
    let mut numerator = IntegerPolynomial::from_coefficients_asc(xs);
    let denominator = if e == 1u32 {
        a * b_over_g
    } else {
        let denominator = a.div_exact(&e) * b_over_g;
        numerator.div_exact_assign(Integer::from(e));
        denominator
    };
    RationalPolynomial {
        numerator,
        denominator,
    }
}

// Adds two `RationalPolynomial`s taken by value, reusing the storage of the one with the longer
// numerator.
fn add_owned_owned(mut p: RationalPolynomial, mut q: RationalPolynomial) -> RationalPolynomial {
    if q.numerator.len() > p.numerator.len() {
        swap(&mut p, &mut q);
    }
    add_owned_ref(p.numerator, p.denominator, &q.numerator, &q.denominator)
}

impl Add<Self> for RationalPolynomial {
    type Output = Self;

    /// Adds two [`RationalPolynomial`]s, taking both by value.
    ///
    /// $$
    /// f(p, q) = p + q.
    /// $$
    ///
    /// The sum is kept in lowest terms. With denominators $a$ and $b$ and $g = \gcd(a, b)$, the
    /// numerators are scaled to the common denominator $ab/g$ and added, and then only a factor of
    /// $g$ can be shared by the new numerator and denominator, so only that is divided out. When
    /// the two polynomials have the same degree, their leading coefficients can cancel, and then
    /// the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerators' coefficients and the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1/2*x+1/3").unwrap()
    ///         + RationalPolynomial::from_str("1/2*x-1/6").unwrap())
    ///     .to_string(),
    ///     "x+1/6"
    /// );
    /// // The leading coefficients cancel.
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1/2*x^2+1/3").unwrap()
    ///         + RationalPolynomial::from_str("-1/2*x^2+1/5*x").unwrap())
    ///     .to_string(),
    ///     "1/5*x+1/3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_add` from `fmpq_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn add(self, other: Self) -> Self {
        add_owned_owned(self, other)
    }
}

impl Add<&Self> for RationalPolynomial {
    type Output = Self;

    /// Adds two [`RationalPolynomial`]s, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(p, q) = p + q.
    /// $$
    ///
    /// The sum is kept in lowest terms. With denominators $a$ and $b$ and $g = \gcd(a, b)$, the
    /// numerators are scaled to the common denominator $ab/g$ and added, and then only a factor of
    /// $g$ can be shared by the new numerator and denominator, so only that is divided out. When
    /// the two polynomials have the same degree, their leading coefficients can cancel, and then
    /// the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerators' coefficients and the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1/2*x+1/3").unwrap()
    ///         + &RationalPolynomial::from_str("1/2*x-1/6").unwrap())
    ///         .to_string(),
    ///     "x+1/6"
    /// );
    /// // The leading coefficients cancel.
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1/2*x^2+1/3").unwrap()
    ///         + &RationalPolynomial::from_str("-1/2*x^2+1/5*x").unwrap())
    ///         .to_string(),
    ///     "1/5*x+1/3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_add` from `fmpq_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn add(self, other: &Self) -> Self {
        add_owned_ref(
            self.numerator,
            self.denominator,
            &other.numerator,
            &other.denominator,
        )
    }
}

impl Add<RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Adds two [`RationalPolynomial`]s, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(p, q) = p + q.
    /// $$
    ///
    /// The sum is kept in lowest terms. With denominators $a$ and $b$ and $g = \gcd(a, b)$, the
    /// numerators are scaled to the common denominator $ab/g$ and added, and then only a factor of
    /// $g$ can be shared by the new numerator and denominator, so only that is divided out. When
    /// the two polynomials have the same degree, their leading coefficients can cancel, and then
    /// the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerators' coefficients and the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x+1/3").unwrap()
    ///         + RationalPolynomial::from_str("1/2*x-1/6").unwrap())
    ///     .to_string(),
    ///     "x+1/6"
    /// );
    /// // The leading coefficients cancel.
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^2+1/3").unwrap()
    ///         + RationalPolynomial::from_str("-1/2*x^2+1/5*x").unwrap())
    ///     .to_string(),
    ///     "1/5*x+1/3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_add` from `fmpq_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn add(self, other: RationalPolynomial) -> RationalPolynomial {
        add_owned_ref(
            other.numerator,
            other.denominator,
            &self.numerator,
            &self.denominator,
        )
    }
}

impl Add<&RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Adds two [`RationalPolynomial`]s, taking both by reference.
    ///
    /// $$
    /// f(p, q) = p + q.
    /// $$
    ///
    /// The sum is kept in lowest terms. With denominators $a$ and $b$ and $g = \gcd(a, b)$, the
    /// numerators are scaled to the common denominator $ab/g$ and added, and then only a factor of
    /// $g$ can be shared by the new numerator and denominator, so only that is divided out. When
    /// the two polynomials have the same degree, their leading coefficients can cancel, and then
    /// the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerators' coefficients and the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x+1/3").unwrap()
    ///         + &RationalPolynomial::from_str("1/2*x-1/6").unwrap())
    ///         .to_string(),
    ///     "x+1/6"
    /// );
    /// // The leading coefficients cancel.
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^2+1/3").unwrap()
    ///         + &RationalPolynomial::from_str("-1/2*x^2+1/5*x").unwrap())
    ///         .to_string(),
    ///     "1/5*x+1/3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_add` from `fmpq_poly/add.c`, FLINT 3.6.0.
    fn add(self, other: &RationalPolynomial) -> RationalPolynomial {
        if ptr::eq(self, other) {
            // p + p = 2p. Halving an even denominator keeps the pair canonical, since the content
            // is coprime to the denominator; and doubling the numerator over an odd denominator
            // introduces no common factor.
            return if self.denominator.even() {
                RationalPolynomial {
                    numerator: self.numerator.clone(),
                    denominator: &self.denominator >> 1u32,
                }
            } else {
                RationalPolynomial {
                    numerator: IntegerPolynomial::from_coefficients_asc(
                        self.numerator
                            .coefficients_asc()
                            .iter()
                            .map(|c| c << 1u32)
                            .collect::<Vec<_>>(),
                    ),
                    denominator: self.denominator.clone(),
                }
            };
        }
        add_owned_ref(
            self.numerator.clone(),
            self.denominator.clone(),
            &other.numerator,
            &other.denominator,
        )
    }
}

impl AddAssign<Self> for RationalPolynomial {
    /// Adds a [`RationalPolynomial`] to a [`RationalPolynomial`] in place, taking the second by
    /// value.
    ///
    /// $$
    /// p \gets p + q.
    /// $$
    ///
    /// The sum is kept in lowest terms. With denominators $a$ and $b$ and $g = \gcd(a, b)$, the
    /// numerators are scaled to the common denominator $ab/g$ and added, and then only a factor of
    /// $g$ can be shared by the new numerator and denominator, so only that is divided out. When
    /// the two polynomials have the same degree, their leading coefficients can cancel, and then
    /// the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerators' coefficients and the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x+1/3").unwrap();
    /// p += RationalPolynomial::from_str("1/2*x-1/6").unwrap();
    /// assert_eq!(p.to_string(), "x+1/6");
    ///
    /// // The leading coefficients cancel.
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+1/3").unwrap();
    /// p += RationalPolynomial::from_str("-1/2*x^2+1/5*x").unwrap();
    /// assert_eq!(p.to_string(), "1/5*x+1/3");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_add` from `fmpq_poly/add.c`, FLINT 3.6.0.
    fn add_assign(&mut self, other: Self) {
        *self = add_owned_owned(take(self), other);
    }
}

impl AddAssign<&Self> for RationalPolynomial {
    /// Adds a [`RationalPolynomial`] to a [`RationalPolynomial`] in place, taking the second by
    /// reference.
    ///
    /// $$
    /// p \gets p + q.
    /// $$
    ///
    /// The sum is kept in lowest terms. With denominators $a$ and $b$ and $g = \gcd(a, b)$, the
    /// numerators are scaled to the common denominator $ab/g$ and added, and then only a factor of
    /// $g$ can be shared by the new numerator and denominator, so only that is divided out. When
    /// the two polynomials have the same degree, their leading coefficients can cancel, and then
    /// the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerators' coefficients and the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x+1/3").unwrap();
    /// p += &RationalPolynomial::from_str("1/2*x-1/6").unwrap();
    /// assert_eq!(p.to_string(), "x+1/6");
    ///
    /// // The leading coefficients cancel.
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+1/3").unwrap();
    /// p += &RationalPolynomial::from_str("-1/2*x^2+1/5*x").unwrap();
    /// assert_eq!(p.to_string(), "1/5*x+1/3");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_add` from `fmpq_poly/add.c`, FLINT 3.6.0.
    fn add_assign(&mut self, other: &Self) {
        let p = take(self);
        *self = add_owned_ref(
            p.numerator,
            p.denominator,
            &other.numerator,
            &other.denominator,
        );
    }
}
