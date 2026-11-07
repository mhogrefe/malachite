// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2021 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::series::mul_truncated::classical::product_coefficient_to_out;
use crate::integer_polynomial::series::mul_truncated::mul_truncated_to_out;
use crate::integer_vector::arithmetic::dot_general::DotScratch;
use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::special::traits::{
    RisingFactorialPolynomial, RisingFactorialPolynomialTruncated,
};

const MAX_BASECASE: usize = 16;

// Writes the first `len` coefficients of (a + x)(a + 1 + x)...(a + n - 1 + x) to `res`, or, if
// `reversed` is true, of (1 + ax)(1 + (a + 1)x)...(1 + (a + n - 1)x), computing in `T`, which must
// be wide enough for every coefficient.
fn ogf_basecase<T: PrimitiveUnsigned>(
    res: &mut [Natural],
    a: u64,
    n: u64,
    len: usize,
    reversed: bool,
) where
    Natural: From<T>,
{
    let a = T::exact_from(a);
    let mut v = [T::ZERO; MAX_BASECASE];
    if reversed {
        v[0] = T::ONE;
        v[1] = a;
        // multiply by (1 + (a + i)x)
        for i in 1..n {
            let ui = usize::exact_from(i);
            let factor = a + T::exact_from(i);
            if ui + 1 < len {
                v[ui + 1] = v[ui] * factor;
            }
            for j in (1..=ui.min(len - 1)).rev() {
                v[j] += factor * v[j - 1];
            }
        }
    } else {
        v[0] = a;
        v[1] = T::ONE;
        // multiply by ((a + i) + x)
        for i in 1..n {
            let ui = usize::exact_from(i);
            let factor = a + T::exact_from(i);
            if ui + 1 < len {
                v[ui + 1] = T::ONE;
            }
            for j in (1..=ui.min(len - 1)).rev() {
                v[j] = factor * v[j] + v[j - 1];
            }
            v[0] *= factor;
        }
    }
    for (r, &x) in res[..len].iter_mut().zip(&v) {
        *r = Natural::from(x);
    }
}

// The basecase of `ogf_bsplit` for a product of n factors, the larger of whose constants is at most
// c, truncated to `len` coefficients: `Some(true)` for single words, `Some(false)` for double
// words, and `None` if the product must be split. (c + x)^n has coefficients bounded by max(c,
// n)^n.
fn ogf_basecase_width(n: u64, c: u64, len: usize) -> Option<bool> {
    let cbc = c.max(n).significant_bits();
    if n == 1 || (len <= MAX_BASECASE && n <= u64::WIDTH && n * cbc <= u64::WIDTH) {
        Some(true)
    } else if len <= MAX_BASECASE && n <= u128::WIDTH && n * cbc <= u128::WIDTH {
        Some(false)
    } else {
        None
    }
}

// Writes the first min(len, b - a + 1) coefficients of (a + x)(a + 1 + x)...(b - 1 + x) to `res`,
// or, if `reversed` is true, of (1 + ax)(1 + (a + 1)x)...(1 + (b - 1)x).
//
// This is `stirling_1u_ogf_bsplit` from `arith/stirling1.c`, FLINT 3.6.0, with `final == 0` and
// `which` equal to 1 or 2, and with a basecase in double words as well as single words.
fn ogf_bsplit(res: &mut [Natural], a: u64, b: u64, len: usize, reversed: bool) {
    let len = len.min(usize::exact_from(b - a + 1));
    let n = b - a;
    match ogf_basecase_width(n, b - 1, len) {
        Some(true) => ogf_basecase::<u64>(res, a, n, len, reversed),
        Some(false) => ogf_basecase::<u128>(res, a, n, len, reversed),
        None => {
            let m = a + ((b - a) >> 1);
            let len1 = usize::exact_from(m - a + 1).min(len);
            let len2 = usize::exact_from(b - m + 1).min(len);
            let mut lr = vec![Natural::ZERO; len1 + len2];
            let (l, r) = lr.split_at_mut(len1);
            ogf_bsplit(l, a, m, len, reversed);
            ogf_bsplit(r, m, b, len, reversed);
            mul_truncated_to_out(&mut res[..len.min(len1 + len2 - 1)], r, l);
        }
    }
}

// The coefficient of x^(l - 1), where l = min(len, b - a + 1), in (a + x)(a + 1 + x)...(b - 1 + x),
// or, if `reversed` is true, in (1 + ax)(1 + (a + 1)x)...(1 + (b - 1)x).
//
// This is `stirling_1u_ogf_bsplit` from `arith/stirling1.c`, FLINT 3.6.0, with `final == 1`, with
// `_fmpz_poly_mulmid_single` as `product_coefficient_to_out`.
pub(crate) fn ogf_bsplit_final(a: u64, b: u64, len: usize, reversed: bool) -> Natural {
    let len = len.min(usize::exact_from(b - a + 1));
    let n = b - a;
    let mut res = vec![Natural::ZERO; len];
    match ogf_basecase_width(n, b - 1, len) {
        Some(true) => ogf_basecase::<u64>(&mut res, a, n, len, reversed),
        Some(false) => ogf_basecase::<u128>(&mut res, a, n, len, reversed),
        None => {
            let m = a + ((b - a) >> 1);
            let len1 = usize::exact_from(m - a + 1).min(len);
            let len2 = usize::exact_from(b - m + 1).min(len);
            let mut lr = vec![Natural::ZERO; len1 + len2];
            let (l, r) = lr.split_at_mut(len1);
            ogf_bsplit(l, a, m, len, reversed);
            ogf_bsplit(r, m, b, len, reversed);
            let mut out = Natural::ZERO;
            product_coefficient_to_out(&mut out, l, r, len - 1, &mut DotScratch::default());
            return out;
        }
    }
    res.pop().unwrap()
}

// This is `arith_stirling_number_1u_vec` from `arith/stirling1.c`, FLINT 3.6.0.
fn stirling_1u_vec(res: &mut [Natural], n: u64, klen: usize) {
    if klen == 0 {
        return;
    }
    if n >= 1 {
        let len = (klen - 1).min(usize::exact_from(n - 1));
        if len >= 1 {
            ogf_bsplit(&mut res[1..], 1, n, len, false);
        }
    }
    res[0] = Natural::from(u32::from(n == 0));
    for k in n..u64::exact_from(klen) {
        res[usize::exact_from(k)] = Natural::from(u32::from(n == k));
    }
}

impl RisingFactorialPolynomial for NaturalPolynomial {
    /// Computes the $n$th rising factorial polynomial, whose coefficients are the unsigned Stirling
    /// numbers of the first kind $\left[{n \atop k}\right]$, the numbers of permutations of $n$
    /// elements with $k$ cycles.
    ///
    /// $$
    /// x^{(n)} = x(x + 1) \cdots (x + n - 1) = \sum_{k=0}^n \left[{n \atop k}\right] x^k.
    /// $$
    ///
    /// $x^{(n)}$ has degree $n$ and leading coefficient 1, and its value at a natural number $m$ is
    /// the rising factorial of $m$. The product is found by binary splitting, with groups of
    /// factors expanded in single or double words while their coefficients fit.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::polynomial::special::traits::RisingFactorialPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::rising_factorial_polynomial(0).to_string(),
    ///     "1"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::rising_factorial_polynomial(1).to_string(),
    ///     "x"
    /// );
    /// // There are 11 permutations of 4 elements with 2 cycles.
    /// assert_eq!(
    ///     NaturalPolynomial::rising_factorial_polynomial(4).to_string(),
    ///     "x^4+6*x^3+11*x^2+6*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `arith_stirling_number_1u_vec` from `arith/stirling1.c`, FLINT 3.6.0,
    /// with a length of $n + 1$, except that groups of factors are also expanded in double words.
    #[inline]
    fn rising_factorial_polynomial(n: u64) -> Self {
        Self::rising_factorial_polynomial_truncated(n, n + 1)
    }
}

impl RisingFactorialPolynomialTruncated for NaturalPolynomial {
    /// Computes the $n$th rising factorial polynomial truncated to its first `len` coefficients,
    /// which are the unsigned Stirling numbers of the first kind $\left[{n \atop k}\right]$ for $k
    /// < \text{len}$.
    ///
    /// $$
    /// f(n, l) = \sum_{k=0}^{\min(l, n + 1) - 1} \left[{n \atop k}\right] x^k.
    /// $$
    ///
    /// A `len` greater than $n$ gives the whole polynomial. The product $x(x + 1) \cdots (x + n -
    /// 1)$ is found by binary splitting, truncating every partial product, with groups of factors
    /// expanded in single or double words while their coefficients fit.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(nl (\log n)^3 \log\log n)$
    ///
    /// $M(n, l) = O(nl \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `n`, and $l$ is `min(len, n + 1)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::polynomial::special::traits::RisingFactorialPolynomialTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::rising_factorial_polynomial_truncated(5, 3).to_string(),
    ///     "50*x^2+24*x"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::rising_factorial_polynomial_truncated(2, 10).to_string(),
    ///     "x^2+x"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::rising_factorial_polynomial_truncated(5, 0).to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `arith_stirling_number_1u_vec` from `arith/stirling1.c`, FLINT 3.6.0,
    /// except that groups of factors are also expanded in double words.
    fn rising_factorial_polynomial_truncated(n: u64, len: u64) -> Self {
        let klen = usize::exact_from(len);
        let mut row = vec![Natural::ZERO; klen];
        stirling_1u_vec(&mut row, n, klen);
        Self::from_coefficients_asc(row)
    }
}
