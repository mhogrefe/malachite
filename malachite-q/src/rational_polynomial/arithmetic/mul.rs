// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use alloc::borrow::Cow;
use core::mem::take;
use core::ops::{Mul, MulAssign};
use core::ptr;
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign, Square};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::content_chained::integers_content_chained;
use malachite_nz::natural::Natural;

// The factors that the product of canonical $x/a$ and $y/b$, both nonzero, shares with its
// denominator: $\gcd(\operatorname{cont}(x), b)$ and $\gcd(\operatorname{cont}(y), a)$. By Gauss's
// lemma the content of $xy$ is $\operatorname{cont}(x) \operatorname{cont}(y)$, and since
// $\operatorname{cont}(x)$ is coprime to $a$ and $\operatorname{cont}(y)$ to $b$, the product of
// these two is the GCD of that content and $ab$. A GCD with a denominator of 1 is 1 without
// computing it.
//
// This is the GCD computation of `_fmpq_poly_mul` from `fmpq_poly/mul.c`, FLINT 3.6.0.
pub(crate) fn cross_gcds(
    x: &IntegerPolynomial,
    a: &Natural,
    y: &IntegerPolynomial,
    b: &Natural,
) -> (Natural, Natural) {
    let gcd_1 = if *b == 1u32 {
        Natural::ONE
    } else {
        integers_content_chained(x.coefficients_asc(), b)
    };
    let gcd_2 = if *a == 1u32 {
        Natural::ONE
    } else {
        integers_content_chained(y.coefficients_asc(), a)
    };
    (gcd_1, gcd_2)
}

// Divides the numerator `x` and the denominator `b` by `g`, which divides both, in place when they
// are owned and into new values when they are borrowed.
fn div_exact_cows(x: &mut Cow<'_, IntegerPolynomial>, b: &mut Cow<'_, Natural>, g: &Natural) {
    if *g == 1u32 {
        return;
    }
    let g_integer = Integer::from(g);
    match x {
        Cow::Borrowed(x_ref) => *x = Cow::Owned((*x_ref).div_exact(g_integer)),
        Cow::Owned(x_owned) => x_owned.div_exact_assign(g_integer),
    }
    match b {
        Cow::Borrowed(b_ref) => *b = Cow::Owned((*b_ref).div_exact(g)),
        Cow::Owned(b_owned) => b_owned.div_exact_assign(g),
    }
}

// Multiplies two values, each owned or borrowed, using the by-value multiplication wherever a value
// is owned.
fn mul_cows<'a, T>(x: Cow<'a, T>, y: Cow<'a, T>) -> T
where
    T: Clone + Mul<T, Output = T> + Mul<&'a T, Output = T>,
    &'a T: Mul<T, Output = T> + Mul<&'a T, Output = T>,
{
    match (x, y) {
        (Cow::Owned(x), Cow::Owned(y)) => x * y,
        (Cow::Owned(x), Cow::Borrowed(y)) => x * y,
        (Cow::Borrowed(x), Cow::Owned(y)) => x * y,
        (Cow::Borrowed(x), Cow::Borrowed(y)) => x * y,
    }
}

// The product of $x/a$ and $y/b$, canonical and each owned or borrowed. The factors that the
// product would share with its denominator are divided out of the operands before multiplying,
// since each divides one numerator and the other denominator: $\gcd(\operatorname{cont}(x), b)$
// divides $x$ and $b$, and $\gcd(\operatorname{cont}(y), a)$ divides $y$ and $a$. The
// multiplication then works on smaller numbers, and its result is already in lowest terms.
//
// This is equivalent to `_fmpq_poly_mul` from `fmpq_poly/mul.c`, FLINT 3.6.0, with the division
// moved before the multiplication, as a TODO there suggests.
fn mul_cow(
    mut x: Cow<'_, IntegerPolynomial>,
    mut a: Cow<'_, Natural>,
    mut y: Cow<'_, IntegerPolynomial>,
    mut b: Cow<'_, Natural>,
) -> RationalPolynomial {
    if *x == IntegerPolynomial::ZERO || *y == IntegerPolynomial::ZERO {
        return RationalPolynomial::ZERO;
    }
    let (gcd_1, gcd_2) = cross_gcds(&x, &a, &y, &b);
    div_exact_cows(&mut x, &mut b, &gcd_1);
    div_exact_cows(&mut y, &mut a, &gcd_2);
    RationalPolynomial {
        numerator: mul_cows(x, y),
        denominator: mul_cows(a, b),
    }
}

// The product of two `RationalPolynomial`s, multiplying first and dividing the common factors out
// of the product afterwards, as `_fmpq_poly_mul` from `fmpq_poly/mul.c`, FLINT 3.6.0, does.
crate_test_fn! {mul_divide_after(
    p: &RationalPolynomial,
    q: &RationalPolynomial,
) -> RationalPolynomial {
    if *p == RationalPolynomial::ZERO || *q == RationalPolynomial::ZERO {
        return RationalPolynomial::ZERO;
    }
    let (gcd_1, gcd_2) = cross_gcds(&p.numerator, &p.denominator, &q.numerator, &q.denominator);
    let g = gcd_1 * gcd_2;
    let mut numerator = &p.numerator * &q.numerator;
    let mut denominator = &p.denominator * &q.denominator;
    if g != 1u32 {
        numerator.div_exact_assign(Integer::from(&g));
        denominator.div_exact_assign(g);
    }
    RationalPolynomial {
        numerator,
        denominator,
    }
}}

impl Mul<Self> for RationalPolynomial {
    type Output = Self;

    /// Multiplies two [`RationalPolynomial`]s, taking both by value.
    ///
    /// $$
    /// f(p, q) = pq.
    /// $$
    ///
    /// The product is kept in lowest terms. With $p = x/a$ and $q = y/b$, the content of $xy$ is
    /// the product of the contents of $x$ and $y$, and these are coprime to $a$ and $b$
    /// respectively, so the only factor $xy$ can share with $ab$ is $\gcd(\operatorname{cont}(x),
    /// b) \gcd(\operatorname{cont}(y), a)$. Each of the two GCDs is divided out of a numerator and
    /// a denominator before multiplying.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any numerator coefficient or denominator of
    /// either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1/2*x+1/3").unwrap()
    ///         * RationalPolynomial::from_str("x-1/2").unwrap())
    ///     .to_string(),
    ///     "1/2*x^2+1/12*x-1/6"
    /// );
    /// // Factors cancel across the two polynomials.
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("2/3*x").unwrap()
    ///         * RationalPolynomial::from_str("3/4*x+3/2").unwrap())
    ///     .to_string(),
    ///     "1/2*x^2+x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_mul` from `fmpq_poly/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, other: Self) -> Self {
        mul_cow(
            Cow::Owned(self.numerator),
            Cow::Owned(self.denominator),
            Cow::Owned(other.numerator),
            Cow::Owned(other.denominator),
        )
    }
}

impl Mul<&Self> for RationalPolynomial {
    type Output = Self;

    /// Multiplies two [`RationalPolynomial`]s, taking the first by value and the second by
    /// reference.
    ///
    /// $$
    /// f(p, q) = pq.
    /// $$
    ///
    /// The product is kept in lowest terms. With $p = x/a$ and $q = y/b$, the content of $xy$ is
    /// the product of the contents of $x$ and $y$, and these are coprime to $a$ and $b$
    /// respectively, so the only factor $xy$ can share with $ab$ is $\gcd(\operatorname{cont}(x),
    /// b) \gcd(\operatorname{cont}(y), a)$. Each of the two GCDs is divided out of a numerator and
    /// a denominator before multiplying.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any numerator coefficient or denominator of
    /// either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1/2*x+1/3").unwrap()
    ///         * &RationalPolynomial::from_str("x-1/2").unwrap())
    ///         .to_string(),
    ///     "1/2*x^2+1/12*x-1/6"
    /// );
    /// // Factors cancel across the two polynomials.
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("2/3*x").unwrap()
    ///         * &RationalPolynomial::from_str("3/4*x+3/2").unwrap())
    ///         .to_string(),
    ///     "1/2*x^2+x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_mul` from `fmpq_poly/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, other: &Self) -> Self {
        mul_cow(
            Cow::Owned(self.numerator),
            Cow::Owned(self.denominator),
            Cow::Borrowed(&other.numerator),
            Cow::Borrowed(&other.denominator),
        )
    }
}

impl Mul<RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Multiplies two [`RationalPolynomial`]s, taking the first by reference and the second by
    /// value.
    ///
    /// $$
    /// f(p, q) = pq.
    /// $$
    ///
    /// The product is kept in lowest terms. With $p = x/a$ and $q = y/b$, the content of $xy$ is
    /// the product of the contents of $x$ and $y$, and these are coprime to $a$ and $b$
    /// respectively, so the only factor $xy$ can share with $ab$ is $\gcd(\operatorname{cont}(x),
    /// b) \gcd(\operatorname{cont}(y), a)$. Each of the two GCDs is divided out of a numerator and
    /// a denominator before multiplying.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any numerator coefficient or denominator of
    /// either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     ((&RationalPolynomial::from_str("1/2*x+1/3").unwrap())
    ///         * RationalPolynomial::from_str("x-1/2").unwrap())
    ///     .to_string(),
    ///     "1/2*x^2+1/12*x-1/6"
    /// );
    /// // Factors cancel across the two polynomials.
    /// assert_eq!(
    ///     ((&RationalPolynomial::from_str("2/3*x").unwrap())
    ///         * RationalPolynomial::from_str("3/4*x+3/2").unwrap())
    ///     .to_string(),
    ///     "1/2*x^2+x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_mul` from `fmpq_poly/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, other: RationalPolynomial) -> RationalPolynomial {
        other * self
    }
}

impl Mul<&RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Multiplies two [`RationalPolynomial`]s, taking both by reference.
    ///
    /// $$
    /// f(p, q) = pq.
    /// $$
    ///
    /// The product is kept in lowest terms. With $p = x/a$ and $q = y/b$, the content of $xy$ is
    /// the product of the contents of $x$ and $y$, and these are coprime to $a$ and $b$
    /// respectively, so the only factor $xy$ can share with $ab$ is $\gcd(\operatorname{cont}(x),
    /// b) \gcd(\operatorname{cont}(y), a)$. Each of the two GCDs is divided out of a numerator and
    /// a denominator before multiplying.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any numerator coefficient or denominator of
    /// either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     ((&RationalPolynomial::from_str("1/2*x+1/3").unwrap())
    ///         * &RationalPolynomial::from_str("x-1/2").unwrap())
    ///         .to_string(),
    ///     "1/2*x^2+1/12*x-1/6"
    /// );
    /// // Factors cancel across the two polynomials.
    /// assert_eq!(
    ///     ((&RationalPolynomial::from_str("2/3*x").unwrap())
    ///         * &RationalPolynomial::from_str("3/4*x+3/2").unwrap())
    ///         .to_string(),
    ///     "1/2*x^2+x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_mul` from `fmpq_poly/mul.c`, FLINT 3.6.0.
    fn mul(self, other: &RationalPolynomial) -> RationalPolynomial {
        // A product with itself is a square, as in `_fmpq_poly_mul`.
        if ptr::eq(self, other) {
            return self.square();
        }
        mul_cow(
            Cow::Borrowed(&self.numerator),
            Cow::Borrowed(&self.denominator),
            Cow::Borrowed(&other.numerator),
            Cow::Borrowed(&other.denominator),
        )
    }
}

impl MulAssign<Self> for RationalPolynomial {
    /// Multiplies a [`RationalPolynomial`] by another [`RationalPolynomial`] in place, taking the
    /// right-hand side by value.
    ///
    /// $$
    /// p \gets pq.
    /// $$
    ///
    /// The product is kept in lowest terms; see the [`Mul`] implementation for how.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any numerator coefficient or denominator of
    /// either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x+1/3").unwrap();
    /// p *= RationalPolynomial::from_str("x-1/2").unwrap();
    /// assert_eq!(p.to_string(), "1/2*x^2+1/12*x-1/6");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_mul` from `fmpq_poly/mul.c`, FLINT 3.6.0.
    fn mul_assign(&mut self, other: Self) {
        *self = take(self) * other;
    }
}

impl MulAssign<&Self> for RationalPolynomial {
    /// Multiplies a [`RationalPolynomial`] by another [`RationalPolynomial`] in place, taking the
    /// right-hand side by reference.
    ///
    /// $$
    /// p \gets pq.
    /// $$
    ///
    /// The product is kept in lowest terms; see the [`Mul`] implementation for how.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any numerator coefficient or denominator of
    /// either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x+1/3").unwrap();
    /// p *= &RationalPolynomial::from_str("x-1/2").unwrap();
    /// assert_eq!(p.to_string(), "1/2*x^2+1/12*x-1/6");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_mul` from `fmpq_poly/mul.c`, FLINT 3.6.0.
    fn mul_assign(&mut self, other: &Self) {
        *self = take(self) * other;
    }
}
