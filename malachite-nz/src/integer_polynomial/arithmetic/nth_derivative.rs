// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::natural::Natural;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{DivExactAssign, Factorial};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{NthDerivative, NthDerivativeAssign};

// The falling factorials i(i - 1)...(i - n + 1), for i from n: the first is n!, and each is carried
// to the next by dividing exactly by i - n and multiplying by i, both of which are small.
fn falling_factorials(n: usize) -> impl FnMut(usize) -> Integer {
    let mut f = Integer::from(Natural::factorial(u64::exact_from(n)));
    move |i| {
        if i > n {
            f.div_exact_assign(Integer::from(i - n));
            f *= Integer::from(i);
        }
        f.clone()
    }
}

// The coefficients of the nth derivative, for n at least 1 and less than `xs.len()`: the
// coefficient of x^i, for i at least n, times i(i - 1)...(i - n + 1), moved to x^(i - n). The
// leading coefficient times its falling factorial is nonzero, so the result is normalized.
//
// This is equivalent to `_fmpz_poly_nth_derivative` from `fmpz_poly/nth_derivative.c`, FLINT 3.6.0.
fn nth_derivative_ref(xs: &[Integer], n: usize) -> Vec<Integer> {
    let mut falling = falling_factorials(n);
    xs.iter()
        .enumerate()
        .skip(n)
        .map(|(i, c)| c * falling(i))
        .collect()
}

fn nth_derivative_in_place(xs: &mut Vec<Integer>, n: u64) {
    if n == 0 {
        return;
    }
    if u64::exact_from(xs.len()) <= n {
        xs.clear();
        return;
    }
    let n = usize::exact_from(n);
    xs.drain(..n);
    let mut falling = falling_factorials(n);
    for (j, c) in xs.iter_mut().enumerate() {
        *c *= falling(j + n);
    }
}

impl NthDerivative for IntegerPolynomial {
    type Output = Self;

    /// Computes the $n$th derivative of an [`IntegerPolynomial`], taking it by value.
    ///
    /// $$
    /// f(p, n) = p^{(n)} = \sum_{i=n}^d i^{\underline n}a_ix^{i-n}.
    /// $$
    ///
    /// Here $i^{\underline n} = i(i-1)\cdots(i-n+1)$ is a falling factorial, and $d$ is the degree.
    /// The zeroth derivative is the polynomial itself, and a polynomial of degree less than $n$ has
    /// $n$th derivative zero.
    ///
    /// # Worst-case complexity
    /// $T(b, k) = O(k(b + k \log k))$
    ///
    /// $M(b, k) = O(b + k^2 \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $b$ is the total number of bits of the
    /// coefficients, and $k$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::NthDerivative;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^4-3*x^3+2*x-5").unwrap();
    /// assert_eq!(p.clone().nth_derivative(2).to_string(), "12*x^2-18*x");
    /// assert_eq!(p.clone().nth_derivative(0).to_string(), "x^4-3*x^3+2*x-5");
    /// assert_eq!(p.nth_derivative(5), IntegerPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_nth_derivative` from `fmpz_poly/nth_derivative.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn nth_derivative(mut self, n: u64) -> Self {
        self.nth_derivative_assign(n);
        self
    }
}

impl NthDerivative for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Computes the $n$th derivative of an [`IntegerPolynomial`], taking it by reference.
    ///
    /// $$
    /// f(p, n) = p^{(n)} = \sum_{i=n}^d i^{\underline n}a_ix^{i-n}.
    /// $$
    ///
    /// Here $i^{\underline n} = i(i-1)\cdots(i-n+1)$ is a falling factorial, and $d$ is the degree.
    /// The zeroth derivative is the polynomial itself, and a polynomial of degree less than $n$ has
    /// $n$th derivative zero.
    ///
    /// # Worst-case complexity
    /// $T(b, k) = O(k(b + k \log k))$
    ///
    /// $M(b, k) = O(b + k^2 \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $b$ is the total number of bits of the
    /// coefficients, and $k$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::NthDerivative;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^4-3*x^3+2*x-5").unwrap();
    /// assert_eq!((&p).nth_derivative(2).to_string(), "12*x^2-18*x");
    /// assert_eq!((&p).nth_derivative(0).to_string(), "x^4-3*x^3+2*x-5");
    /// assert_eq!((&p).nth_derivative(5), IntegerPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_nth_derivative` from `fmpz_poly/nth_derivative.c`, FLINT
    /// 3.6.0.
    fn nth_derivative(self, n: u64) -> IntegerPolynomial {
        if n == 0 {
            return self.clone();
        }
        if u64::exact_from(self.coefficients.len()) <= n {
            return IntegerPolynomial {
                coefficients: Vec::new(),
            };
        }
        IntegerPolynomial {
            coefficients: nth_derivative_ref(&self.coefficients, usize::exact_from(n)),
        }
    }
}

impl NthDerivativeAssign for IntegerPolynomial {
    /// Replaces an [`IntegerPolynomial`] with its $n$th derivative.
    ///
    /// $$
    /// p \gets p^{(n)} = \sum_{i=n}^d i^{\underline n}a_ix^{i-n}.
    /// $$
    ///
    /// Here $i^{\underline n} = i(i-1)\cdots(i-n+1)$ is a falling factorial, and $d$ is the degree.
    /// The zeroth derivative is the polynomial itself, and a polynomial of degree less than $n$ has
    /// $n$th derivative zero.
    ///
    /// # Worst-case complexity
    /// $T(b, k) = O(k(b + k \log k))$
    ///
    /// $M(b, k) = O(b + k^2 \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $b$ is the total number of bits of the
    /// coefficients, and $k$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::NthDerivativeAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^4-3*x^3+2*x-5").unwrap();
    /// p.nth_derivative_assign(2);
    /// assert_eq!(p.to_string(), "12*x^2-18*x");
    ///
    /// let mut p = IntegerPolynomial::from_str("x^4-3*x^3+2*x-5").unwrap();
    /// p.nth_derivative_assign(0);
    /// assert_eq!(p.to_string(), "x^4-3*x^3+2*x-5");
    ///
    /// let mut p = IntegerPolynomial::from_str("x^4-3*x^3+2*x-5").unwrap();
    /// p.nth_derivative_assign(5);
    /// assert_eq!(p, IntegerPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_nth_derivative` from `fmpz_poly/nth_derivative.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn nth_derivative_assign(&mut self, n: u64) {
        nth_derivative_in_place(&mut self.coefficients, n);
    }
}
