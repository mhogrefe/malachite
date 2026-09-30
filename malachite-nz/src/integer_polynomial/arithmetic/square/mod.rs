// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
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
use crate::integer_polynomial::arithmetic::square::classical::square_to_out_classical;
use crate::integer_polynomial::arithmetic::square::karatsuba::square_to_out_karatsuba;
use crate::integer_polynomial::arithmetic::square::kronecker::square_to_out_kronecker;
use crate::integer_polynomial::arithmetic::square::tiny::{
    square_to_out_tiny_1, square_to_out_tiny_2,
};
use crate::integer_polynomial::arithmetic::vec::max_bits::vec_max_bits;
use crate::integer_polynomial::arithmetic::vec::{
    TinyKernel, classical_preferred, fft_preferred, karatsuba_preferred, tiny_kernel,
};
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;

pub mod classical;
pub mod karatsuba;
pub mod kronecker;
pub mod tiny;

// Sets `out` to the coefficients of the square of the polynomial with coefficients `xs`, which is
// nonempty. `out` must have length `2 * xs.len() - 1`.
//
// # Worst-case complexity
// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
//
// $M(n, m) = O(n(m + \log n) \log (nm))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is the largest number of
// significant bits of any element of `xs`.
//
// This is equivalent to `_fmpz_poly_sqr` from `fmpz_poly/sqr.c`, FLINT 3.6.0. FLINT's first choice
// for long inputs, which multiplies polynomials directly with its small-prime FFT
// (`_fmpz_poly_mul_mid_default_mpn_ctx` from `fft_small/fmpz_poly_mul.c`), and
// Schönhage–Strassen, which it chooses for some inputs of medium size, have not been ported yet;
// Kronecker substitution stands in for both. (Its single integer multiplication reaches the port of
// the small-prime FFT's integer multiplication in `natural/arithmetic/mul/fft.rs` when the operands
// are large.)
crate_test_fn! {square_to_out(out: &mut [Integer], xs: &[Integer]) {
    if xs.len() == 1 {
        out[0] = (&xs[0]).square();
        return;
    }
    let bits = vec_max_bits(xs).0;
    let len = u64::exact_from(xs.len());
    if fft_preferred(len, bits, bits, 80, 160)
        && mul_middle_to_out_fft(out, xs, xs, 0, (xs.len() << 1) - 1)
    {
        return;
    }
    match tiny_kernel(bits, bits, len, len < 50 + 3 * bits) {
        Some(TinyKernel::OneWord) => square_to_out_tiny_1(out, xs),
        Some(TinyKernel::TwoWord) => square_to_out_tiny_2(out, xs),
        None if classical_preferred(len, bits, bits) => {
            square_to_out_classical(out, xs);
        }
        None if karatsuba_preferred(len, bits, bits) => {
            square_to_out_karatsuba(out, xs);
        }
        None => {
            // Schönhage–Strassen, which FLINT chooses instead for some inputs of medium size,
            // has not been ported yet, so Kronecker substitution stands in for it.
            square_to_out_kronecker(out, xs);
        }
    }
}}

// This is equivalent to `fmpz_poly_sqr` from `fmpz_poly/sqr.c`, FLINT 3.6.0.
fn square_ref(xs: &[Integer]) -> Vec<Integer> {
    if xs.is_empty() {
        return Vec::new();
    }
    let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
    square_to_out(&mut out, xs);
    out
}

impl Square for IntegerPolynomial {
    type Output = Self;

    /// Squares an [`IntegerPolynomial`], taking it by value.
    ///
    /// $$
    /// f(p) = p^2.
    /// $$
    ///
    /// Squaring takes roughly half the coefficient multiplications of multiplying two different
    /// polynomials of the same length.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the polynomial, and $m$ is
    /// the largest number of significant bits of any of its coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^2-3*x+2").unwrap())
    ///         .square()
    ///         .to_string(),
    ///     "x^4-6*x^3+13*x^2-12*x+4"
    /// );
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("-x+1").unwrap())
    ///         .square()
    ///         .to_string(),
    ///     "x^2-2*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sqr` from `fmpz_poly/sqr.c`, FLINT 3.6.0.
    #[inline]
    fn square(mut self) -> Self {
        self.square_assign();
        self
    }
}

impl Square for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Squares an [`IntegerPolynomial`], taking it by reference.
    ///
    /// $$
    /// f(p) = p^2.
    /// $$
    ///
    /// Squaring takes roughly half the coefficient multiplications of multiplying two different
    /// polynomials of the same length.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the polynomial, and $m$ is
    /// the largest number of significant bits of any of its coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2-3*x+2").unwrap())
    ///         .square()
    ///         .to_string(),
    ///     "x^4-6*x^3+13*x^2-12*x+4"
    /// );
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("-x+1").unwrap())
    ///         .square()
    ///         .to_string(),
    ///     "x^2-2*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sqr` from `fmpz_poly/sqr.c`, FLINT 3.6.0.
    #[inline]
    fn square(self) -> IntegerPolynomial {
        IntegerPolynomial {
            coefficients: square_ref(&self.coefficients),
        }
    }
}

impl SquareAssign for IntegerPolynomial {
    /// Squares an [`IntegerPolynomial`] in place.
    ///
    /// $$
    /// p \gets p^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the polynomial, and $m$ is
    /// the largest number of significant bits of any of its coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SquareAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// p.square_assign();
    /// assert_eq!(p.to_string(), "x^4-6*x^3+13*x^2-12*x+4");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sqr` from `fmpz_poly/sqr.c`, FLINT 3.6.0.
    #[inline]
    fn square_assign(&mut self) {
        // The square of a constant is computed in place.
        if let [c] = self.coefficients.as_mut_slice() {
            c.square_assign();
        } else {
            self.coefficients = square_ref(&self.coefficients);
        }
    }
}
