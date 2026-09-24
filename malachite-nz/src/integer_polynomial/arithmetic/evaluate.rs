// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::iter::Sum;
use core::mem::replace;
use core::ops::{AddAssign, Mul, MulAssign};
use malachite_base::num::arithmetic::traits::{Parity, PowerOf2, Square};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::polynomial::Evaluate;

// Evaluates a polynomial, given by its coefficients in ascending order, at `x` with Horner's rule.
//
// This is equivalent to `_fmpz_poly_evaluate_horner_fmpz` from `fmpz_poly/evaluate_horner_fmpz.c`,
// FLINT 3.6.0.
#[doc(hidden)]
pub fn evaluate_horner<T>(coefficients: &[T], x: &T) -> T
where
    T: Clone + Zero + PartialEq + for<'a> AddAssign<&'a T> + for<'a> MulAssign<&'a T>,
{
    let Some((leading, rest)) = coefficients.split_last() else {
        return T::ZERO;
    };
    if rest.is_empty() || *x == T::ZERO {
        return coefficients[0].clone();
    }
    let mut value = leading.clone();
    for c in rest.iter().rev() {
        value *= x;
        value += c;
    }
    value
}

// The block structure shared by the divide-and-conquer evaluations, at an integer here and at a
// rational in malachite-q.
//
// Adjacent coefficients are paired into blocks, and blocks are merged like the carries of a binary
// counter: a block covering `2^k` coefficients waits in `partials[k]` until an equal one follows
// it, and the two are merged. Merging equal halves keeps the operands of each multiplication about
// the same size, where Horner's rule multiplies an ever larger accumulator by the same value.
// Whatever blocks remain at the end are merged from the smallest up.
//
// `pair(c_i, c_{i+1})` makes the block of two adjacent coefficients, and `single(c)` the block of
// the last coefficient when the length is odd. `merge(lower, upper, k, upper_len)` combines a block
// `lower` of `2^k` coefficients with the block `upper` of `upper_len` coefficients directly above
// it. There must be at least 2 coefficients.
//
// This is the loop structure of `_fmpz_poly_evaluate_divconquer_fmpz` from
// `fmpz_poly/evaluate_divconquer_fmpz.c`, FLINT 3.6.0, with the arithmetic left to the caller.
#[doc(hidden)]
pub fn divide_and_conquer_blocks<C, T: Clone + Zero>(
    coefficients: &[C],
    pair: impl Fn(&C, &C) -> T,
    single: impl Fn(&C) -> T,
    merge: impl Fn(&T, T, usize, usize) -> T,
) -> T {
    let len = coefficients.len();
    assert!(len >= 2);
    // 2^{h - 1} < len <= 2^h, and h >= 1.
    let h = usize::exact_from((len - 1).significant_bits());
    let mut partials = vec![T::ZERO; h + 1];
    // Absorbs a block of `block_len` coefficients ending just before coefficient `end` into the
    // pending blocks, merging it with as many of them as the carries of `end` call for, and returns
    // where it was left and how many coefficients it then covers.
    let absorb =
        |mut block: T, mut block_len: usize, end: usize, partials: &mut [T]| -> (usize, usize) {
            let carries = usize::exact_from(end.trailing_zeros());
            let mut k = 1;
            while k < carries {
                block = merge(&partials[k], block, k, block_len);
                block_len += usize::power_of_2(u64::exact_from(k));
                k += 1;
            }
            partials[k] = block;
            (k, block_len)
        };
    let mut k = 1;
    let mut top_len = 0;
    for (i, [low, high]) in coefficients.as_chunks::<2>().0.iter().enumerate() {
        (k, top_len) = absorb(pair(low, high), 2, (i + 1) << 1, &mut partials);
    }
    if len.odd() {
        (k, top_len) = absorb(single(&coefficients[len - 1]), 1, len + 1, &mut partials);
    }
    let mut value = replace(&mut partials[k], T::ZERO);
    while k < h {
        if (len - 1).get_bit(u64::exact_from(k)) {
            value = merge(&partials[k], value, k, top_len);
            top_len += usize::power_of_2(u64::exact_from(k));
        }
        k += 1;
    }
    value
}

// Evaluates a polynomial, given by its coefficients in ascending order, at `x` by divide and
// conquer: a block of coefficients `c_i, ..., c_{i + l - 1}` has the value `c_i + c_{i + 1} x + ...
// + c_{i + l - 1} x^{l - 1}`, and a block of `2^k` coefficients below one with value `v` merges
// with it as `lower + x^{2^k} v`.
//
// This is equivalent to `fmpz_poly_evaluate_divconquer_fmpz` and
// `_fmpz_poly_evaluate_divconquer_fmpz` from `fmpz_poly/evaluate_divconquer_fmpz.c`, FLINT 3.6.0.
#[doc(hidden)]
pub fn evaluate_divide_and_conquer<T>(coefficients: &[T], x: &T) -> T
where
    T: Clone + Zero + for<'a> AddAssign<&'a T> + for<'a> MulAssign<&'a T>,
    for<'a> &'a T: Mul<&'a T, Output = T> + Square<Output = T>,
{
    match coefficients.len() {
        0 => return T::ZERO,
        1 => return coefficients[0].clone(),
        _ => {}
    }
    let h = usize::exact_from((coefficients.len() - 1).significant_bits());
    // powers[k - 1] is x^{2^k}, for 1 <= k < h; x itself is borrowed rather than stored.
    let mut powers: Vec<T> = Vec::with_capacity(h - 1);
    for k in 1..h {
        let square = if k == 1 {
            x.square()
        } else {
            powers[k - 2].square()
        };
        powers.push(square);
    }
    let power = |k: usize| if k == 0 { x } else { &powers[k - 1] };
    divide_and_conquer_blocks(
        coefficients,
        |low, high| {
            let mut block = high * x;
            block += low;
            block
        },
        T::clone,
        |lower, mut upper, k, _| {
            upper *= power(k);
            upper += lower;
            upper
        },
    )
}

// Divide and conquer only pays once its balanced multiplications are large enough for a
// subquadratic multiplication algorithm, and when does that depends on the size of `x` as much as
// on the number of coefficients. A one-limb `x` makes every step of Horner's rule a cheap
// multiplication by one limb, and divide and conquer takes a long polynomial to catch up; otherwise
// the crossover follows the product of the length and the number of limbs of `x`. The size of the
// coefficients moves the crossover too, but both ways, and not enough to be worth a scan of them.
//
// Tuned on 64-bit Apple Silicon with the `evaluate` tuning level (`-g tune_evaluate`).
pub(crate) const EVALUATE_DIVIDE_AND_CONQUER_ONE_LIMB_THRESHOLD: usize = 1024;
pub(crate) const EVALUATE_DIVIDE_AND_CONQUER_LENGTH_TIMES_LIMBS_THRESHOLD: usize = 256;
pub(crate) const EVALUATE_DIVIDE_AND_CONQUER_MIN_LENGTH: usize = 4;

// Whether divide and conquer beats Horner's rule for a polynomial with `len` coefficients at a
// value of `x_bits` bits.
pub(crate) fn evaluate_use_divide_and_conquer(len: usize, x_bits: u64) -> bool {
    let x_limbs = usize::exact_from(x_bits.div_ceil(Limb::WIDTH));
    if x_limbs <= 1 {
        len >= EVALUATE_DIVIDE_AND_CONQUER_ONE_LIMB_THRESHOLD
    } else {
        len >= EVALUATE_DIVIDE_AND_CONQUER_MIN_LENGTH
            && len.saturating_mul(x_limbs)
                >= EVALUATE_DIVIDE_AND_CONQUER_LENGTH_TIMES_LIMBS_THRESHOLD
    }
}

// Evaluates a polynomial, given by its coefficients in ascending order, at `x`, choosing between
// Horner's rule and divide and conquer by the length and the size of `x`. At `x = 1` the value is
// the sum of the coefficients, which needs no multiplication at all.
//
// This is equivalent to `_fmpz_poly_evaluate_fmpz` from `fmpz_poly/evaluate_fmpz.c`, FLINT 3.6.0,
// except for the shortcut at 1 and the choice of algorithm, where FLINT switches at 50 coefficients
// whatever the size of `x`.
pub(crate) fn evaluate<T>(coefficients: &[T], x: &T) -> T
where
    T: Clone + Zero + One + PartialEq + for<'a> AddAssign<&'a T> + for<'a> MulAssign<&'a T>,
    T: for<'a> Sum<&'a T>,
    for<'a> &'a T: Mul<&'a T, Output = T> + Square<Output = T> + SignificantBits,
{
    if *x == T::ONE {
        coefficients.iter().sum()
    } else if evaluate_use_divide_and_conquer(coefficients.len(), x.significant_bits()) {
        evaluate_divide_and_conquer(coefficients, x)
    } else {
        evaluate_horner(coefficients, x)
    }
}

// At `x = -1` the value is the alternating sum of the coefficients, which needs no multiplication.
fn evaluate_integer(coefficients: &[Integer], x: &Integer) -> Integer {
    if *x == -1i32 {
        coefficients.iter().step_by(2).sum::<Integer>()
            - coefficients.iter().skip(1).step_by(2).sum::<Integer>()
    } else {
        evaluate(coefficients, x)
    }
}

impl Evaluate<&Integer> for &IntegerPolynomial {
    type Output = Integer;

    /// Evaluates an [`IntegerPolynomial`] at an [`Integer`], taking both by reference.
    ///
    /// $$
    /// f(p, x) = \sum_{i=0}^{n-1} c_i x^i,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Horner's rule is used unless the polynomial is long compared with the size of `x`, in which
    /// case divide and conquer, which pairs off coefficients and merges the pairs so that each
    /// multiplication has operands of about the same size, is faster.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log^2 n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()` times the larger of the
    /// greatest number of bits of any coefficient and the number of bits of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Two, Zero};
    /// use malachite_base::polynomial::Evaluate;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// assert_eq!((&p).evaluate(&Integer::ZERO), 2);
    /// assert_eq!((&p).evaluate(&Integer::ONE), 0);
    /// assert_eq!((&p).evaluate(&Integer::from(-2)), 12);
    /// assert_eq!((&p).evaluate(&Integer::from(10)), 72);
    ///
    /// let q = IntegerPolynomial::from_str("x^100-1").unwrap();
    /// assert_eq!(
    ///     (&q).evaluate(&Integer::TWO).to_string(),
    ///     "1267650600228229401496703205375"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_evaluate_fmpz` from `fmpz_poly/evaluate_fmpz.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn evaluate(self, x: &Integer) -> Integer {
        evaluate_integer(&self.coefficients, x)
    }
}

impl Evaluate<Integer> for &IntegerPolynomial {
    type Output = Integer;

    /// Evaluates an [`IntegerPolynomial`] at an [`Integer`], taking the polynomial by reference and
    /// the value by value.
    ///
    /// $$
    /// f(p, x) = \sum_{i=0}^{n-1} c_i x^i,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Horner's rule is used unless the polynomial is long compared with the size of `x`, in which
    /// case divide and conquer, which pairs off coefficients and merges the pairs so that each
    /// multiplication has operands of about the same size, is faster.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log^2 n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()` times the larger of the
    /// greatest number of bits of any coefficient and the number of bits of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Two, Zero};
    /// use malachite_base::polynomial::Evaluate;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// assert_eq!((&p).evaluate(Integer::ZERO), 2);
    /// assert_eq!((&p).evaluate(Integer::ONE), 0);
    /// assert_eq!((&p).evaluate(Integer::from(-2)), 12);
    /// assert_eq!((&p).evaluate(Integer::from(10)), 72);
    ///
    /// let q = IntegerPolynomial::from_str("x^100-1").unwrap();
    /// assert_eq!(
    ///     (&q).evaluate(Integer::TWO).to_string(),
    ///     "1267650600228229401496703205375"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_evaluate_fmpz` from `fmpz_poly/evaluate_fmpz.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn evaluate(self, x: Integer) -> Integer {
        evaluate_integer(&self.coefficients, &x)
    }
}
