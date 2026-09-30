// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
//      Copyright © 2014 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::mul_middle::fft::mul_middle_to_out_fft;
use crate::integer_polynomial::arithmetic::square_truncated::classical::*;
use crate::integer_polynomial::arithmetic::square_truncated::karatsuba::*;
use crate::integer_polynomial::arithmetic::square_truncated::kronecker::*;
use crate::integer_polynomial::arithmetic::square_truncated::tiny::{
    square_truncated_to_out_tiny_1, square_truncated_to_out_tiny_2,
};
use crate::integer_polynomial::arithmetic::vec::max_bits::vec_max_bits;
use crate::integer_polynomial::arithmetic::vec::{
    TinyKernel, classical_preferred, fft_preferred, karatsuba_preferred, tiny_kernel,
};
use alloc::vec;
use core::cmp::min;
use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{SquareTruncated, SquareTruncatedAssign};

pub mod classical;
pub mod karatsuba;
pub mod kronecker;
pub mod tiny;

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, which is nonempty. `out.len()` must be positive and at most `2 * xs.len() -
// 1`.
//
// # Worst-case complexity
// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
//
// $M(n, m) = O(n(m + \log n) \log (nm))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `out.len()`, and $m$ is the largest number of
// significant bits of any element of `xs`.
//
// This is equivalent to `_fmpz_poly_sqrlow` from `fmpz_poly/sqrlow.c`, FLINT 3.6.0, where `n` is
// `out.len()`. FLINT's first choice for long inputs, which multiplies polynomials directly with its
// small-prime FFT (`_fmpz_poly_mul_mid_default_mpn_ctx` from `fft_small/fmpz_poly_mul.c`), and
// Schönhage–Strassen, which it chooses for some inputs of medium size, have not been ported yet;
// Kronecker substitution stands in for both. (Its single integer multiplication reaches the port of
// the small-prime FFT's integer multiplication in `natural/arithmetic/mul/fft.rs` when the operands
// are large.)
crate_test_fn! {square_truncated_to_out(out: &mut [Integer], xs: &[Integer]) {
    let n = out.len();
    let xs = &xs[..min(xs.len(), n)];
    if xs.len() == 1 {
        out[0] = (&xs[0]).square();
        return;
    }
    let bits = vec_max_bits(xs).0;
    let len = u64::exact_from(xs.len());
    if fft_preferred(len, bits, bits, 100, 240) && mul_middle_to_out_fft(out, xs, xs, 0, n) {
        return;
    }
    let n = u64::exact_from(n);
    let short_enough = len < 50 + (bits << 1) || (len << 2 >= 3 * n && n < 140 + 6 * bits);
    match tiny_kernel(bits, bits, len, short_enough) {
        Some(TinyKernel::OneWord) => square_truncated_to_out_tiny_1(out, xs),
        Some(TinyKernel::TwoWord) => square_truncated_to_out_tiny_2(out, xs),
        None if classical_preferred(len, bits, bits) => {
            square_truncated_to_out_classical(out, xs);
        }
        None if karatsuba_preferred(len, bits, bits) => {
            square_truncated_to_out_karatsuba(out, xs);
        }
        None => {
            // Schönhage–Strassen, which FLINT chooses instead for some inputs of medium size,
            // has not been ported yet, so Kronecker substitution stands in for it.
            square_truncated_to_out_kronecker(out, xs);
        }
    }
}}

// This is equivalent to `fmpz_poly_sqrlow` from `fmpz_poly/sqrlow.c`, FLINT 3.6.0.
fn square_truncated_ref(xs: &[Integer], len: u64) -> IntegerPolynomial {
    if xs.is_empty() || len == 0 {
        return IntegerPolynomial::ZERO;
    }
    let n = usize::try_from(len)
        .unwrap_or(usize::MAX)
        .min((xs.len() << 1) - 1);
    let mut out = vec![Integer::ZERO; n];
    square_truncated_to_out(&mut out, xs);
    let mut p = IntegerPolynomial { coefficients: out };
    p.trim();
    p
}

impl SquareTruncated for IntegerPolynomial {
    type Output = Self;

    /// Squares an [`IntegerPolynomial`], keeping only the coefficients of $x^i$ for $i$ less than
    /// `len`, taking it by value.
    ///
    /// $$
    /// f(p, n) = p^2 \bmod x^n.
    /// $$
    ///
    /// The polynomial need not already be truncated: this is the square of its image modulo $x^n$,
    /// so only its first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of the polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SquareTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^2-3*x+2").unwrap())
    ///         .square_truncated(3)
    ///         .to_string(),
    ///     "13*x^2-12*x+4"
    /// );
    /// // The cross terms combine with the square of the linear coefficient.
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^2+x-1").unwrap())
    ///         .square_truncated(3)
    ///         .to_string(),
    ///     "-x^2-2*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sqrlow` from `fmpz_poly/sqrlow.c`, FLINT 3.6.0.
    #[inline]
    fn square_truncated(mut self, len: u64) -> Self {
        self.square_truncated_assign(len);
        self
    }
}

impl SquareTruncated for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Squares an [`IntegerPolynomial`], keeping only the coefficients of $x^i$ for $i$ less than
    /// `len`, taking it by reference.
    ///
    /// $$
    /// f(p, n) = p^2 \bmod x^n.
    /// $$
    ///
    /// The polynomial need not already be truncated: this is the square of its image modulo $x^n$,
    /// so only its first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of the polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SquareTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2-3*x+2").unwrap())
    ///         .square_truncated(3)
    ///         .to_string(),
    ///     "13*x^2-12*x+4"
    /// );
    /// // The cross terms combine with the square of the linear coefficient.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2+x-1").unwrap())
    ///         .square_truncated(3)
    ///         .to_string(),
    ///     "-x^2-2*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sqrlow` from `fmpz_poly/sqrlow.c`, FLINT 3.6.0.
    #[inline]
    fn square_truncated(self, len: u64) -> IntegerPolynomial {
        square_truncated_ref(&self.coefficients, len)
    }
}

impl SquareTruncatedAssign for IntegerPolynomial {
    /// Squares an [`IntegerPolynomial`] in place, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`.
    ///
    /// $$
    /// p \gets p^2 \bmod x^n.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of the polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SquareTruncatedAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// p.square_truncated_assign(3);
    /// assert_eq!(p.to_string(), "13*x^2-12*x+4");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sqrlow` from `fmpz_poly/sqrlow.c`, FLINT 3.6.0.
    #[inline]
    fn square_truncated_assign(&mut self, len: u64) {
        // The square of a constant is computed in place.
        if len != 0
            && let [c] = self.coefficients.as_mut_slice()
        {
            c.square_assign();
        } else {
            *self = square_truncated_ref(&self.coefficients, len);
        }
    }
}
