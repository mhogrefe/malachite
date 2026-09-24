// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use alloc::borrow::Cow;
use alloc::vec;
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{
    AddMul, AddMulAssign, CoprimeWith, Square, UnsignedAbs,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitIterable, SignificantBits};
use malachite_base::polynomial::Evaluate;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::evaluate::divide_and_conquer_blocks;
use malachite_nz::platform::Limb;

// Evaluating `p(x) = c_0 + c_1 x + ... + c_d x^d` at `x = a / b` in lowest terms, with `b > 0`, is
// evaluating the homogenized polynomial
//
//   N = c_0 b^d + c_1 a b^{d - 1} + ... + c_d a^d,
//
// in integers, and then `p(a / b) = N / b^d`. Nothing but numerators is computed, and the
// denominator is known without computing any fractions. The fraction is also nearly always in
// lowest terms already: a prime dividing `b` divides every term but the last, and does not divide
// `a`, so it divides `N` exactly when it divides `c_d`. So when `c_d` is coprime to `b` no gcd is
// needed at all, and otherwise one is.
//
// FLINT evaluates at a rational by carrying a numerator and a denominator through every step, which
// is the same computation but with denominators that grow faster than they need to (a
// divide-and-conquer merge multiplies in the lower block's denominator as well as the upper's), and
// then reduces the result with a full gcd.

// Makes `N / b^d` a `Rational`, where `b` is positive and `leading` is `c_d`.
fn from_homogeneous(n: Integer, b_power: Integer, leading: &Integer, b: &Integer) -> Rational {
    if leading
        .unsigned_abs_ref()
        .coprime_with(b.unsigned_abs_ref())
    {
        // N / b^d is already in lowest terms. N is not 0 unless b is 1, since by the rational root
        // theorem a root a / b of p has b dividing c_d.
        Rational {
            sign: n >= 0u32,
            numerator: n.unsigned_abs(),
            denominator: b_power.unsigned_abs(),
        }
    } else {
        Rational::from_integers(n, b_power)
    }
}

// Evaluates the homogenized polynomial with Horner's rule, returning N and b^d. There must be at
// least 2 coefficients.
fn homogeneous_horner(f: &[Integer], a: &Integer, b: &Integer) -> (Integer, Integer) {
    let (leading, rest) = f.split_last().unwrap();
    let mut n = leading.clone();
    let mut b_power = b.clone();
    let mut first = true;
    for c in rest.iter().rev() {
        if first {
            first = false;
        } else {
            b_power *= b;
        }
        n *= a;
        n.add_mul_assign(c, &b_power);
    }
    (n, b_power)
}

// Evaluates the homogenized polynomial by divide and conquer, returning N and b^d. There must be at
// least 2 coefficients.
//
// A block of `l` coefficients `c_i, ..., c_{i + l - 1}` holds its own homogenized value `c_i b^{l -
// 1} + c_{i + 1} a b^{l - 2} + ... + c_{i + l - 1} a^{l - 1}`, and a block `L` of `2^k`
// coefficients merges with a block `U` of `l` coefficients above it as `L b^l + a^{2^k} U`.
fn homogeneous_divide_and_conquer(f: &[Integer], a: Integer, b: Integer) -> (Integer, Integer) {
    let len = f.len();
    let h = usize::exact_from((len - 1).significant_bits());
    // a_powers[k] = a^{2^k} and b_powers[k] = b^{2^k}, for 0 <= k < h, in one allocation.
    let mut powers = vec![Integer::ZERO; h << 1];
    let (a_powers, b_powers) = powers.split_at_mut(h);
    a_powers[0] = a;
    b_powers[0] = b;
    for k in 1..h {
        a_powers[k] = (&a_powers[k - 1]).square();
        b_powers[k] = (&b_powers[k - 1]).square();
    }
    let (a_powers, b_powers) = (&*a_powers, &*b_powers);
    // b^l, which for a block of 2^k coefficients is one of the stored powers.
    let b_power = |l: usize| -> Cow<'_, Integer> {
        if l.is_power_of_two() {
            Cow::Borrowed(&b_powers[usize::exact_from(l.trailing_zeros())])
        } else {
            let mut power = Integer::ONE;
            for (k, bit) in l.bits().enumerate() {
                if bit {
                    power *= &b_powers[k];
                }
            }
            Cow::Owned(power)
        }
    };
    let n = divide_and_conquer_blocks(
        f,
        |low, high| (low * &b_powers[0]).add_mul(high, &a_powers[0]),
        Integer::clone,
        |lower, upper, k, upper_len| (upper * &a_powers[k]).add_mul(lower, &*b_power(upper_len)),
    );
    (n, b_power(len - 1).into_owned())
}

// Splits a `Rational` into a signed numerator and a positive denominator.
fn to_integers(x: &Rational) -> (Integer, Integer) {
    (
        Integer::from_sign_and_abs_ref(*x >= 0u32, x.numerator_ref()),
        Integer::from(x.denominator_ref()),
    )
}

// Evaluates a polynomial at `x` with Horner's rule.
//
// This is equivalent to `fmpz_poly_evaluate_horner_fmpq` from `fmpz_poly/evaluate_horner_fmpq.c`,
// FLINT 3.6.0.
crate_test_fn! {evaluate_integer_polynomial_horner(f: &[Integer], x: &Rational) -> Rational {
    match f.len() {
        0 => Rational::ZERO,
        1 => Rational::from(&f[0]),
        _ => {
            let (a, b) = to_integers(x);
            let (n, b_power) = homogeneous_horner(f, &a, &b);
            from_homogeneous(n, b_power, f.last().unwrap(), &b)
        }
    }
}}

// Evaluates a polynomial at `x` by divide and conquer.
//
// This is equivalent to `fmpz_poly_evaluate_divconquer_fmpq` from
// `fmpz_poly/evaluate_divconquer_fmpq.c`, FLINT 3.6.0.
crate_test_fn! {evaluate_integer_polynomial_divide_and_conquer(
    f: &[Integer],
    x: &Rational,
) -> Rational {
    match f.len() {
        0 => Rational::ZERO,
        1 => Rational::from(&f[0]),
        _ => {
            let (a, b) = to_integers(x);
            let b_copy = b.clone();
            let (n, b_power) = homogeneous_divide_and_conquer(f, a, b);
            from_homogeneous(n, b_power, f.last().unwrap(), &b_copy)
        }
    }
}}

// Whether divide and conquer beats Horner's rule for the homogenized evaluation of a polynomial
// with `len` coefficients, whose leading coefficient has `leading_bits` bits, at `a / b`.
//
// As at an `Integer`, a point that fits in one limb makes Horner's steps cheap, and a larger point
// makes the crossover follow the product of the length and the point's size in limbs. At a
// rational, each Horner step does three multiplications rather than one, so divide and conquer
// catches up sooner. The sizes of the coefficients and of the denominator also matter when the
// point fits in one limb: a small denominator makes the steps cheaper still, while large
// coefficients make them dearer. The leading coefficient stands in for the others, since it is at
// hand.
//
// Tuned on 64-bit Apple Silicon with the `evaluate_rational` tuning level of malachite-q (`-g
// tune_evaluate_rational`). FLINT uses Horner's rule for fewer than 40 coefficients or when the
// denominator has more than `0.003 len^2` bits, and divide and conquer otherwise.
pub(crate) const EVALUATE_RATIONAL_DIVIDE_AND_CONQUER_ONE_LIMB_THRESHOLD: u64 = 16384;
pub(crate) const EVALUATE_RATIONAL_DIVIDE_AND_CONQUER_LENGTH_TIMES_LIMBS_THRESHOLD: u64 = 128;
pub(crate) const EVALUATE_RATIONAL_DIVIDE_AND_CONQUER_MIN_LENGTH: u64 = 4;

fn evaluate_rational_use_divide_and_conquer(
    len: usize,
    leading_bits: u64,
    a_bits: u64,
    b_bits: u64,
) -> bool {
    let len = u64::exact_from(len);
    let point_limbs = max(a_bits, b_bits).div_ceil(Limb::WIDTH);
    if point_limbs <= 1 {
        len.saturating_mul(leading_bits.div_ceil(Limb::WIDTH) + 1)
            .saturating_mul(b_bits)
            >= EVALUATE_RATIONAL_DIVIDE_AND_CONQUER_ONE_LIMB_THRESHOLD
    } else {
        len >= EVALUATE_RATIONAL_DIVIDE_AND_CONQUER_MIN_LENGTH
            && len.saturating_mul(point_limbs)
                >= EVALUATE_RATIONAL_DIVIDE_AND_CONQUER_LENGTH_TIMES_LIMBS_THRESHOLD
    }
}

// Evaluates a polynomial at `a / b`, in lowest terms with `b > 0`. At an integer, the evaluation is
// the one at an `Integer`, with its own shortcuts and tuning.
//
// This is equivalent to `_fmpz_poly_evaluate_fmpq` from `fmpz_poly/evaluate_fmpq.c`, FLINT 3.6.0,
// except for the choice of algorithm.
fn evaluate_integer_polynomial(p: &IntegerPolynomial, a: Integer, b: Integer) -> Rational {
    if b == 1u32 {
        return Rational::from(p.evaluate(a));
    }
    let f = p.coefficients_asc();
    let len = f.len();
    match len {
        0 => return Rational::ZERO,
        1 => return Rational::from(&f[0]),
        _ => {}
    }
    let leading = f.last().unwrap();
    if evaluate_rational_use_divide_and_conquer(
        len,
        leading.significant_bits(),
        a.significant_bits(),
        b.significant_bits(),
    ) {
        let b_copy = b.clone();
        let (n, b_power) = homogeneous_divide_and_conquer(f, a, b);
        from_homogeneous(n, b_power, leading, &b_copy)
    } else {
        let (n, b_power) = homogeneous_horner(f, &a, &b);
        from_homogeneous(n, b_power, leading, &b)
    }
}

impl Evaluate<&Rational> for &IntegerPolynomial {
    type Output = Rational;

    /// Evaluates an [`IntegerPolynomial`] at a [`Rational`], taking both by reference.
    ///
    /// $$
    /// f(p, x) = \sum_{i=0}^{n-1} c_i x^i,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The result is in
    /// lowest terms, and the zero polynomial evaluates to 0 everywhere.
    ///
    /// At $x = a/b$, the numerator $\sum_i c_i a^i b^{n-1-i}$ is computed in integers, and the
    /// denominator is $b^{n-1}$; the fraction needs reducing only when the leading coefficient
    /// shares a factor with $b$. At an integer, the evaluation is the one at an [`Integer`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log^2 n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()` times the larger of the
    /// greatest number of bits of any coefficient and the number of bits of the numerator or
    /// denominator of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Evaluate;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    /// use malachite_q::Rational;
    ///
    /// let p = IntegerPolynomial::from_str("4*x^2-3*x+2").unwrap();
    /// assert_eq!(
    ///     (&p).evaluate(&Rational::from_str("1/2").unwrap())
    ///         .to_string(),
    ///     "3/2"
    /// );
    /// assert_eq!(
    ///     (&p).evaluate(&Rational::from_str("-2/3").unwrap())
    ///         .to_string(),
    ///     "52/9"
    /// );
    /// assert_eq!(
    ///     (&p).evaluate(&Rational::from_str("3").unwrap()).to_string(),
    ///     "29"
    /// );
    ///
    /// // The value is in lowest terms: (2x - 1)^2 vanishes at 1/2.
    /// let q = IntegerPolynomial::from_str("4*x^2-4*x+1").unwrap();
    /// assert_eq!(
    ///     (&q).evaluate(&Rational::from_str("1/2").unwrap())
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_evaluate_fmpq` from `fmpz_poly/evaluate_fmpq.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn evaluate(self, x: &Rational) -> Rational {
        let (a, b) = to_integers(x);
        evaluate_integer_polynomial(self, a, b)
    }
}

impl Evaluate<Rational> for &IntegerPolynomial {
    type Output = Rational;

    /// Evaluates an [`IntegerPolynomial`] at a [`Rational`], taking the polynomial by reference and
    /// the value by value.
    ///
    /// $$
    /// f(p, x) = \sum_{i=0}^{n-1} c_i x^i,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The result is in
    /// lowest terms, and the zero polynomial evaluates to 0 everywhere.
    ///
    /// At $x = a/b$, the numerator $\sum_i c_i a^i b^{n-1-i}$ is computed in integers, and the
    /// denominator is $b^{n-1}$; the fraction needs reducing only when the leading coefficient
    /// shares a factor with $b$. At an integer, the evaluation is the one at an [`Integer`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log^2 n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()` times the larger of the
    /// greatest number of bits of any coefficient and the number of bits of the numerator or
    /// denominator of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Evaluate;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    /// use malachite_q::Rational;
    ///
    /// let p = IntegerPolynomial::from_str("4*x^2-3*x+2").unwrap();
    /// assert_eq!(
    ///     (&p).evaluate(Rational::from_str("1/2").unwrap())
    ///         .to_string(),
    ///     "3/2"
    /// );
    /// assert_eq!(
    ///     (&p).evaluate(Rational::from_str("-2/3").unwrap())
    ///         .to_string(),
    ///     "52/9"
    /// );
    /// assert_eq!(
    ///     (&p).evaluate(Rational::from_str("3").unwrap()).to_string(),
    ///     "29"
    /// );
    ///
    /// // The value is in lowest terms: (2x - 1)^2 vanishes at 1/2.
    /// let q = IntegerPolynomial::from_str("4*x^2-4*x+1").unwrap();
    /// assert_eq!(
    ///     (&q).evaluate(Rational::from_str("1/2").unwrap())
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_evaluate_fmpq` from `fmpz_poly/evaluate_fmpq.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn evaluate(self, x: Rational) -> Rational {
        // The numerator and denominator of x are moved rather than cloned.
        let sign = x >= 0u32;
        let (numerator, denominator) = x.into_numerator_and_denominator();
        evaluate_integer_polynomial(
            self,
            Integer::from_sign_and_abs(sign, numerator),
            Integer::from(denominator),
        )
    }
}
