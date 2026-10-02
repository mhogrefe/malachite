// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2Square, ModPowerOf2SquareAssign,
};
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::{
    MOD_POWER_OF_2_SQUARE_KARATSUBA_THRESHOLD, add_wrapping_assign, from_coefficients_trimmed,
    karatsuba_wrapping_scratch_len, mask_coefficients, sub_wrapping_assign,
};
use alloc::vec;

// Sets `out` to the square of the polynomial with coefficients `xs`, modulo $2^\text{W}$, by
// schoolbook multiplication: each product of two different coefficients is computed once and
// doubled. `out` must have length `2 * xs.len() - 1`.
pub(crate) fn square_classical_wrapping<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T]) {
    out.fill(T::ZERO);
    for (i, &x) in xs.iter().enumerate() {
        if x != T::ZERO {
            for (o, &y) in out[(i << 1) + 1..].iter_mut().zip(&xs[i + 1..]) {
                o.wrapping_add_assign(x.wrapping_mul(y));
            }
        }
    }
    for o in out.iter_mut() {
        *o = o.wrapping_add(*o);
    }
    for (i, &x) in xs.iter().enumerate() {
        out[i << 1].wrapping_add_assign(x.wrapping_mul(x));
    }
}

// Sets `out` to the square of the polynomial with coefficients `xs`, of nonzero length $n$, modulo
// $2^\text{W}$, by Karatsuba multiplication, falling back to schoolbook multiplication below the
// threshold. `out` must have length $2n - 1$, and `scratch` at least
// `karatsuba_wrapping_scratch_len(n)`.
fn square_karatsuba_scratch_wrapping<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    scratch: &mut [T],
) {
    let n = xs.len();
    if n < MOD_POWER_OF_2_SQUARE_KARATSUBA_THRESHOLD {
        square_classical_wrapping(out, xs);
        return;
    }
    // Write x = x_0 + x^h x_1. Then x^2 = x_0^2 + x^h ((x_0 + x_1)^2 - x_0^2 - x_1^2) + x^{2h}
    // x_1^2.
    let h = n >> 1;
    let c = n - h;
    let two_h = h << 1;
    let (x0, x1) = xs.split_at(h);
    let (sum, scratch) = scratch.split_at_mut(c);
    let (middle, scratch) = scratch.split_at_mut((c << 1) - 1);
    {
        let (low, high) = out.split_at_mut(two_h);
        square_karatsuba_scratch_wrapping(&mut low[..two_h - 1], x0, scratch);
        low[two_h - 1] = T::ZERO;
        square_karatsuba_scratch_wrapping(high, x1, scratch);
    }
    sum.copy_from_slice(x1);
    add_wrapping_assign(sum, x0);
    square_karatsuba_scratch_wrapping(middle, sum, scratch);
    sub_wrapping_assign(middle, &out[..two_h - 1]);
    sub_wrapping_assign(middle, &out[two_h..]);
    add_wrapping_assign(&mut out[h..], middle);
}

// Sets `out` to the square of the polynomial with coefficients `xs`, which is nonempty, modulo
// $2^\text{W}$, by Karatsuba multiplication, falling back to schoolbook multiplication for short
// polynomials. `out` must have length `2 * xs.len() - 1`.
pub(crate) fn square_karatsuba_wrapping<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T]) {
    if xs.len() < MOD_POWER_OF_2_SQUARE_KARATSUBA_THRESHOLD {
        square_classical_wrapping(out, xs);
        return;
    }
    let mut scratch = vec![T::ZERO; karatsuba_wrapping_scratch_len(xs.len())];
    square_karatsuba_scratch_wrapping(out, xs, &mut scratch);
}

fn assert_lengths<T>(out: &[T], xs: &[T]) {
    assert!(!xs.is_empty());
    assert_eq!(out.len(), (xs.len() << 1) - 1);
}

// Sets `out` to the square of the polynomial with coefficients `xs`, nonempty and reduced modulo
// $2^k$, where $k$ is `pow`, by schoolbook multiplication. `out` must have length `2 * xs.len() -
// 1`, and `pow` must be no greater than `T::WIDTH`.
crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_square_to_out_classical<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    pow: u64,
) {
    assert_lengths(out, xs);
    assert!(pow <= T::WIDTH);
    square_classical_wrapping(out, xs);
    mask_coefficients(out, pow);
}}

// Sets `out` to the square of the polynomial with coefficients `xs`, nonempty and reduced modulo
// $2^k$, where $k$ is `pow`, by Karatsuba multiplication. `out` must have length `2 * xs.len() -
// 1`, and `pow` must be no greater than `T::WIDTH`.
crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_square_to_out_karatsuba<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    pow: u64,
) {
    assert_lengths(out, xs);
    assert!(pow <= T::WIDTH);
    square_karatsuba_wrapping(out, xs);
    mask_coefficients(out, pow);
}}

/// Sets `out` to the square of the polynomial with coefficients `xs`, nonempty and reduced modulo
/// $2^k$, where $k$ is `pow`. `out` must have length `2 * xs.len() - 1`, and `pow` must be no
/// greater than `T::WIDTH`.
///
/// This is not part of the public API; it is public so that `malachite-nz` can square
/// `NaturalPolynomial`s with word-sized coefficients modulo $2^k$.
#[doc(hidden)]
pub fn mod_power_of_2_square_to_out<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], pow: u64) {
    assert_lengths(out, xs);
    assert!(pow <= T::WIDTH);
    square_karatsuba_wrapping(out, xs);
    mask_coefficients(out, pow);
}

fn assert_reduced<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, pow: u64) {
    assert!(pow <= T::WIDTH);
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
}

// The square of the polynomial with coefficients `xs`, reduced modulo $2^k$, where $k$ is `pow`,
// modulo $2^k$.
fn mod_power_of_2_square_helper<T: PrimitiveUnsigned>(xs: &[T], pow: u64) -> UnsignedPolynomial<T> {
    if xs.is_empty() {
        return UnsignedPolynomial::ZERO;
    }
    let mut out = vec![T::ZERO; (xs.len() << 1) - 1];
    mod_power_of_2_square_to_out(&mut out, xs, pow);
    from_coefficients_trimmed(out)
}

impl<T: PrimitiveUnsigned> ModPowerOf2Square for UnsignedPolynomial<T> {
    type Output = Self;

    /// Squares an [`UnsignedPolynomial`] modulo $2^k$, taking it by value. Its coefficients must
    /// already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, k) = p^2 \bmod 2^k.
    /// $$
    ///
    /// The leading coefficient of the square can vanish modulo $2^k$, and then the degree of the
    /// square is lower than twice the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Square;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The square is x^4+6*x^3+13*x^2+12*x+4; its coefficients wrap around modulo 8.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_square(3)
    ///         .to_string(),
    ///     "x^4+6*x^3+5*x^2+4*x+4"
    /// );
    /// // The square is 16*x^2+8*x+1, which is 1 modulo 8.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("4*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_square(3)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mul` from `nmod_poly/mul.c`, FLINT 3.6.0, with both factors
    /// equal and the modulus $2^k$.
    fn mod_power_of_2_square(self, pow: u64) -> Self {
        assert_reduced(&self, pow);
        mod_power_of_2_square_helper(&self.coefficients, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Square for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Squares an [`UnsignedPolynomial`] modulo $2^k$, taking it by reference. Its coefficients
    /// must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, k) = p^2 \bmod 2^k.
    /// $$
    ///
    /// The leading coefficient of the square can vanish modulo $2^k$, and then the degree of the
    /// square is lower than twice the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Square;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The square is x^4+6*x^3+13*x^2+12*x+4; its coefficients wrap around modulo 8.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_power_of_2_square(3)
    ///         .to_string(),
    ///     "x^4+6*x^3+5*x^2+4*x+4"
    /// );
    /// // The square is 16*x^2+8*x+1, which is 1 modulo 8.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("4*x+1").unwrap())
    ///         .mod_power_of_2_square(3)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mul` from `nmod_poly/mul.c`, FLINT 3.6.0, with both factors
    /// equal and the modulus $2^k$.
    fn mod_power_of_2_square(self, pow: u64) -> UnsignedPolynomial<T> {
        assert_reduced(self, pow);
        mod_power_of_2_square_helper(&self.coefficients, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2SquareAssign for UnsignedPolynomial<T> {
    /// Squares an [`UnsignedPolynomial`] modulo $2^k$ in place. Its coefficients must already be
    /// reduced modulo $2^k$.
    ///
    /// $$
    /// p \gets p^2 \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SquareAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_square_assign(3);
    /// assert_eq!(p.to_string(), "x^4+6*x^3+5*x^2+4*x+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mul` from `nmod_poly/mul.c`, FLINT 3.6.0, with both factors
    /// equal and the modulus $2^k$.
    fn mod_power_of_2_square_assign(&mut self, pow: u64) {
        assert_reduced(self, pow);
        *self = mod_power_of_2_square_helper(&self.coefficients, pow);
    }
}
