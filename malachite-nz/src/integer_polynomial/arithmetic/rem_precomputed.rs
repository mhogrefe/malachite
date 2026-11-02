// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2013 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use alloc::vec::Vec;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{AddMulAssign, Pow, SubMulAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::arithmetic::traits::{RemPrecomputed, RemPrecomputedAssign};

/// Data precomputed for finding remainders by a fixed polynomial divisor, with [`RemPrecomputed`]
/// and [`RemPrecomputedAssign`].
///
/// For a divisor $b$ with $n$ coefficients and positive leading coefficient $\ell$, the data holds
/// $\ell$ and integer polynomials $P_0, P_1, \ldots, P_{n-1}$, each of degree less than $n - 1$,
/// with $\ell^{j+1} x^{n-1+j} \equiv P_j \pmod b$. Reducing a dividend with fewer than $2n$
/// coefficients then takes one scalar multiple of a $P_j$ for each coefficient from $x^{n-1}$ up,
/// and no divisions. In practice a remainder found this way takes about as long as one found with
/// [`%`](core::ops::Rem), because the $P_j$ have larger coefficients than $b$.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RemPrecomputedData {
    powers: Vec<IntegerPolynomial>,
    leading_coefficient: Integer,
}

impl RemPrecomputedData {
    // Returns the leading coefficient of the divisor the data was precomputed from, made positive.
    #[doc(hidden)]
    #[inline]
    pub const fn leading_coefficient(&self) -> &Integer {
        &self.leading_coefficient
    }

    // Returns the length of the divisor the data was precomputed from.
    #[doc(hidden)]
    #[inline]
    pub fn divisor_len(&self) -> u64 {
        u64::wrapping_from(self.powers.len())
    }
}

// Precomputes the data for dividing by the polynomial with coefficients `ys`, whose leading
// coefficient must be positive.
//
// This is equivalent to `_fmpz_poly_powers_precompute` from `fmpz_poly/powers_precompute.c`, FLINT
// 3.6.0, except that the divisor need not be monic (FLINT's reduction is only correct when it is),
// so the powers are scaled by powers of the leading coefficient to stay integral, and that only the
// powers from $x^{n-1}$ on are kept, since the lower ones are just powers of $x$. The rational
// counterpart, `_fmpq_poly_powers_precompute` from `fmpq_poly/powers_precompute.c`, keeps the same
// powers as rational polynomials.
#[doc(hidden)]
pub fn precompute_rem_powers(ys: &[Integer]) -> RemPrecomputedData {
    let n = ys.len();
    let (ys_init, ys_last) = ys.split_at(n - 1);
    let lead = &ys_last[0];
    assert!(*lead > 0u32);
    let monic = *lead == 1u32;
    let mut power: Vec<Integer> = ys_init.iter().map(|y| -y).collect();
    let mut powers = Vec::with_capacity(n);
    powers.push(IntegerPolynomial::from_coefficients_asc(power.clone()));
    for _ in 1..n {
        let top = power.pop().unwrap();
        power.insert(0, Integer::ZERO);
        if !monic {
            for x in &mut power {
                *x *= lead;
            }
        }
        if top != 0u32 {
            for (x, y) in power.iter_mut().zip(ys_init) {
                x.sub_mul_assign(&top, y);
            }
        }
        powers.push(IntegerPolynomial::from_coefficients_asc(power.clone()));
    }
    RemPrecomputedData {
        powers,
        leading_coefficient: lead.clone(),
    }
}

fn trim(xs: &mut Vec<Integer>) {
    while xs.last() == Some(&Integer::ZERO) {
        xs.pop();
    }
}

// Reduces `xs`, with fewer than twice as many coefficients as the divisor, by folding each
// coefficient from $x^{n-1}$ up into the low part, with a Horner scheme in $\ell$. Returns the low
// part and $k$ such that it is $\ell^k$ times the remainder.
fn rem_by_powers_short(mut xs: Vec<Integer>, data: &RemPrecomputedData) -> (Vec<Integer>, u64) {
    let n = data.powers.len();
    if xs.len() < n {
        return (xs, 0);
    }
    let highs = xs.split_off(n - 1);
    let lead = &data.leading_coefficient;
    let monic = *lead == 1u32;
    for (a, power) in highs.iter().zip(&data.powers) {
        if !monic {
            for x in &mut xs {
                *x *= lead;
            }
        }
        if *a != 0u32 {
            for (x, c) in xs.iter_mut().zip(power.coefficients_asc()) {
                x.add_mul_assign(a, c);
            }
        }
    }
    trim(&mut xs);
    (xs, u64::wrapping_from(highs.len()))
}

// Returns the remainder of the polynomial with coefficients `xs` by the divisor that `data` was
// precomputed from, multiplied by $\ell^k$, together with $k$; when the divisor is monic, $\ell^k$
// is 1. A dividend with at least twice as many coefficients as the divisor, $2n$, is reduced $n$
// coefficients at a time, from the top.
//
// This is equivalent to `_fmpz_poly_rem_powers_precomp` from `fmpz_poly/rem_powers_precomp.c`,
// FLINT 3.6.0, except that the divisor need not be monic and that the long dividend is reduced
// iteratively rather than recursively. `_fmpq_poly_rem_powers_precomp` from
// `fmpq_poly/rem_powers_precomp.c` reduces a long dividend with an ordinary division instead.
#[doc(hidden)]
pub fn rem_by_powers(mut xs: Vec<Integer>, data: &RemPrecomputedData) -> (Vec<Integer>, u64) {
    let n = data.powers.len();
    if n == 1 {
        return (Vec::new(), 0);
    }
    let mut start = 0;
    while xs.len() - start >= n << 1 {
        start += n;
    }
    let (mut rem, mut k) = rem_by_powers_short(xs.split_off(start), data);
    let lead = &data.leading_coefficient;
    let monic = *lead == 1u32;
    while !xs.is_empty() {
        let mut block = xs.split_off(xs.len() - n);
        if !monic && k != 0 {
            let scale = lead.pow(k);
            for x in &mut block {
                *x *= &scale;
            }
        }
        block.append(&mut rem);
        trim(&mut block);
        let (block_rem, block_k) = rem_by_powers_short(block, data);
        rem = block_rem;
        k += block_k;
    }
    (rem, k)
}

fn precompute_helper(q: &IntegerPolynomial) -> RemPrecomputedData {
    let ys = q.coefficients_asc();
    let lead = ys.last().expect("cannot divide by the zero polynomial");
    assert!(
        *lead.unsigned_abs_ref() == 1u32,
        "the leading coefficient of the divisor must be 1 or -1, not {lead}"
    );
    if *lead > 0u32 {
        precompute_rem_powers(ys)
    } else {
        let negated: Vec<Integer> = ys.iter().map(|y| -y).collect();
        precompute_rem_powers(&negated)
    }
}

fn rem_precomputed_helper(
    p: Vec<Integer>,
    q: &IntegerPolynomial,
    data: &RemPrecomputedData,
) -> IntegerPolynomial {
    assert_eq!(
        q.len(),
        data.divisor_len(),
        "the data was not precomputed from the divisor"
    );
    IntegerPolynomial::from_coefficients_asc(rem_by_powers(p, data).0)
}

impl RemPrecomputed<Self> for IntegerPolynomial {
    type Output = Self;
    type Data = RemPrecomputedData;

    /// Precomputes the data for dividing [`IntegerPolynomial`]s by `other` with
    /// [`rem_precomputed`](RemPrecomputed::rem_precomputed) or
    /// [`rem_precomputed_assign`](RemPrecomputedAssign::rem_precomputed_assign). The leading
    /// coefficient of `other` must be 1 or $-1$, so that every remainder by it has integer
    /// coefficients.
    ///
    /// The data is the powers $x^{n-1}, x^n, \ldots, x^{2n-2}$ reduced modulo `other`, where $n$ is
    /// `other.len()`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, or if its leading coefficient is not 1 or $-1$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::RemPrecomputed;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let q = IntegerPolynomial::from_str("x^2+x-1").unwrap();
    /// let data = IntegerPolynomial::precompute_rem_data(&q);
    ///
    /// let p = IntegerPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(p.rem_precomputed(q.clone(), &data).to_string(), "-2*x+4");
    ///
    /// let p = IntegerPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(p.rem_precomputed(q, &data).to_string(), "4*x");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_powers_precompute` from `fmpz_poly/powers_precompute.c`,
    /// FLINT 3.6.0, except that FLINT gives wrong powers unless `other` is monic.
    #[inline]
    fn precompute_rem_data(other: &Self) -> RemPrecomputedData {
        precompute_helper(other)
    }

    /// Finds the remainder of two [`IntegerPolynomial`]s, using data precomputed from the divisor,
    /// taking both by value. The leading coefficient of the divisor must be 1 or $-1$.
    ///
    /// $$
    /// f(a, b) = r, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The data should be obtained with
    /// [`precompute_rem_data`](RemPrecomputed::precompute_rem_data). It holds the powers $x^{n-1},
    /// \ldots, x^{2n-2}$ reduced modulo the divisor, where $n$ is the divisor's length, so a
    /// dividend with fewer than $2n$ coefficients is reduced by adding a multiple of one power for
    /// each of its coefficients from $x^{n-1}$ up, and a longer one $n$ coefficients at a time,
    /// from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and of the precomputed powers.
    ///
    /// # Panics
    /// Panics if `data` was precomputed from a divisor with a different length. If `data` was not
    /// precomputed from the divisor, the result is meaningless.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::RemPrecomputed;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let q = IntegerPolynomial::from_str("x^2+x-1").unwrap();
    /// let data = IntegerPolynomial::precompute_rem_data(&q);
    ///
    /// let p = IntegerPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(p.rem_precomputed(q.clone(), &data).to_string(), "-2*x+4");
    ///
    /// let p = IntegerPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(p.rem_precomputed(q, &data).to_string(), "4*x");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_rem_powers_precomp` from `fmpz_poly/rem_powers_precomp.c`,
    /// FLINT 3.6.0, except that FLINT's remainder is only correct when the divisor is monic.
    #[inline]
    fn rem_precomputed(self, other: Self, data: &RemPrecomputedData) -> Self {
        rem_precomputed_helper(self.into_coefficients_asc(), &other, data)
    }
}

impl RemPrecomputed<&Self> for IntegerPolynomial {
    type Output = Self;
    type Data = RemPrecomputedData;

    /// Precomputes the data for dividing [`IntegerPolynomial`]s by `other` with
    /// [`rem_precomputed`](RemPrecomputed::rem_precomputed) or
    /// [`rem_precomputed_assign`](RemPrecomputedAssign::rem_precomputed_assign). The leading
    /// coefficient of `other` must be 1 or $-1$, so that every remainder by it has integer
    /// coefficients.
    ///
    /// The data is the powers $x^{n-1}, x^n, \ldots, x^{2n-2}$ reduced modulo `other`, where $n$ is
    /// `other.len()`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, or if its leading coefficient is not 1 or $-1$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::RemPrecomputed;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let q = IntegerPolynomial::from_str("x^2+x-1").unwrap();
    /// let data = IntegerPolynomial::precompute_rem_data(&q);
    ///
    /// let p = IntegerPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(p.rem_precomputed(q.clone(), &data).to_string(), "-2*x+4");
    ///
    /// let p = IntegerPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(p.rem_precomputed(q, &data).to_string(), "4*x");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_powers_precompute` from `fmpz_poly/powers_precompute.c`,
    /// FLINT 3.6.0, except that FLINT gives wrong powers unless `other` is monic.
    #[inline]
    fn precompute_rem_data(other: &&Self) -> RemPrecomputedData {
        precompute_helper(other)
    }

    /// Finds the remainder of two [`IntegerPolynomial`]s, using data precomputed from the divisor,
    /// taking the first by value and the second by reference. The leading coefficient of the
    /// divisor must be 1 or $-1$.
    ///
    /// $$
    /// f(a, b) = r, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The data should be obtained with
    /// [`precompute_rem_data`](RemPrecomputed::precompute_rem_data). It holds the powers $x^{n-1},
    /// \ldots, x^{2n-2}$ reduced modulo the divisor, where $n$ is the divisor's length, so a
    /// dividend with fewer than $2n$ coefficients is reduced by adding a multiple of one power for
    /// each of its coefficients from $x^{n-1}$ up, and a longer one $n$ coefficients at a time,
    /// from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and of the precomputed powers.
    ///
    /// # Panics
    /// Panics if `data` was precomputed from a divisor with a different length. If `data` was not
    /// precomputed from the divisor, the result is meaningless.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::RemPrecomputed;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let q = IntegerPolynomial::from_str("x^2+x-1").unwrap();
    /// let data = IntegerPolynomial::precompute_rem_data(&q);
    ///
    /// let p = IntegerPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(p.rem_precomputed(&q.clone(), &data).to_string(), "-2*x+4");
    ///
    /// let p = IntegerPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(p.rem_precomputed(&q, &data).to_string(), "4*x");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_rem_powers_precomp` from `fmpz_poly/rem_powers_precomp.c`,
    /// FLINT 3.6.0, except that FLINT's remainder is only correct when the divisor is monic.
    #[inline]
    fn rem_precomputed(self, other: &Self, data: &RemPrecomputedData) -> Self {
        rem_precomputed_helper(self.into_coefficients_asc(), other, data)
    }
}

impl RemPrecomputed<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;
    type Data = RemPrecomputedData;

    /// Precomputes the data for dividing [`IntegerPolynomial`]s by `other` with
    /// [`rem_precomputed`](RemPrecomputed::rem_precomputed) or
    /// [`rem_precomputed_assign`](RemPrecomputedAssign::rem_precomputed_assign). The leading
    /// coefficient of `other` must be 1 or $-1$, so that every remainder by it has integer
    /// coefficients.
    ///
    /// The data is the powers $x^{n-1}, x^n, \ldots, x^{2n-2}$ reduced modulo `other`, where $n$ is
    /// `other.len()`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, or if its leading coefficient is not 1 or $-1$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::RemPrecomputed;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let q = IntegerPolynomial::from_str("x^2+x-1").unwrap();
    /// let data = IntegerPolynomial::precompute_rem_data(&q);
    ///
    /// let p = IntegerPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(p.rem_precomputed(q.clone(), &data).to_string(), "-2*x+4");
    ///
    /// let p = IntegerPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(p.rem_precomputed(q, &data).to_string(), "4*x");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_powers_precompute` from `fmpz_poly/powers_precompute.c`,
    /// FLINT 3.6.0, except that FLINT gives wrong powers unless `other` is monic.
    #[inline]
    fn precompute_rem_data(other: &IntegerPolynomial) -> RemPrecomputedData {
        precompute_helper(other)
    }

    /// Finds the remainder of two [`IntegerPolynomial`]s, using data precomputed from the divisor,
    /// taking the first by reference and the second by value. The leading coefficient of the
    /// divisor must be 1 or $-1$.
    ///
    /// $$
    /// f(a, b) = r, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The data should be obtained with
    /// [`precompute_rem_data`](RemPrecomputed::precompute_rem_data). It holds the powers $x^{n-1},
    /// \ldots, x^{2n-2}$ reduced modulo the divisor, where $n$ is the divisor's length, so a
    /// dividend with fewer than $2n$ coefficients is reduced by adding a multiple of one power for
    /// each of its coefficients from $x^{n-1}$ up, and a longer one $n$ coefficients at a time,
    /// from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and of the precomputed powers.
    ///
    /// # Panics
    /// Panics if `data` was precomputed from a divisor with a different length. If `data` was not
    /// precomputed from the divisor, the result is meaningless.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::RemPrecomputed;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let q = IntegerPolynomial::from_str("x^2+x-1").unwrap();
    /// let data = IntegerPolynomial::precompute_rem_data(&q);
    ///
    /// let p = IntegerPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!((&p).rem_precomputed(q.clone(), &data).to_string(), "-2*x+4");
    ///
    /// let p = IntegerPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!((&p).rem_precomputed(q, &data).to_string(), "4*x");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_rem_powers_precomp` from `fmpz_poly/rem_powers_precomp.c`,
    /// FLINT 3.6.0, except that FLINT's remainder is only correct when the divisor is monic.
    #[inline]
    fn rem_precomputed(
        self,
        other: IntegerPolynomial,
        data: &RemPrecomputedData,
    ) -> IntegerPolynomial {
        rem_precomputed_helper(self.coefficients_asc().to_vec(), &other, data)
    }
}

impl RemPrecomputed<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;
    type Data = RemPrecomputedData;

    /// Precomputes the data for dividing [`IntegerPolynomial`]s by `other` with
    /// [`rem_precomputed`](RemPrecomputed::rem_precomputed) or
    /// [`rem_precomputed_assign`](RemPrecomputedAssign::rem_precomputed_assign). The leading
    /// coefficient of `other` must be 1 or $-1$, so that every remainder by it has integer
    /// coefficients.
    ///
    /// The data is the powers $x^{n-1}, x^n, \ldots, x^{2n-2}$ reduced modulo `other`, where $n$ is
    /// `other.len()`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, or if its leading coefficient is not 1 or $-1$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::RemPrecomputed;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let q = IntegerPolynomial::from_str("x^2+x-1").unwrap();
    /// let data = IntegerPolynomial::precompute_rem_data(&q);
    ///
    /// let p = IntegerPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(p.rem_precomputed(q.clone(), &data).to_string(), "-2*x+4");
    ///
    /// let p = IntegerPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(p.rem_precomputed(q, &data).to_string(), "4*x");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_powers_precompute` from `fmpz_poly/powers_precompute.c`,
    /// FLINT 3.6.0, except that FLINT gives wrong powers unless `other` is monic.
    #[inline]
    fn precompute_rem_data(other: &&IntegerPolynomial) -> RemPrecomputedData {
        precompute_helper(other)
    }

    /// Finds the remainder of two [`IntegerPolynomial`]s, using data precomputed from the divisor,
    /// taking both by reference. The leading coefficient of the divisor must be 1 or $-1$.
    ///
    /// $$
    /// f(a, b) = r, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The data should be obtained with
    /// [`precompute_rem_data`](RemPrecomputed::precompute_rem_data). It holds the powers $x^{n-1},
    /// \ldots, x^{2n-2}$ reduced modulo the divisor, where $n$ is the divisor's length, so a
    /// dividend with fewer than $2n$ coefficients is reduced by adding a multiple of one power for
    /// each of its coefficients from $x^{n-1}$ up, and a longer one $n$ coefficients at a time,
    /// from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and of the precomputed powers.
    ///
    /// # Panics
    /// Panics if `data` was precomputed from a divisor with a different length. If `data` was not
    /// precomputed from the divisor, the result is meaningless.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::RemPrecomputed;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let q = IntegerPolynomial::from_str("x^2+x-1").unwrap();
    /// let data = IntegerPolynomial::precompute_rem_data(&q);
    ///
    /// let p = IntegerPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).rem_precomputed(&q.clone(), &data).to_string(),
    ///     "-2*x+4"
    /// );
    ///
    /// let p = IntegerPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!((&p).rem_precomputed(&q, &data).to_string(), "4*x");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_rem_powers_precomp` from `fmpz_poly/rem_powers_precomp.c`,
    /// FLINT 3.6.0, except that FLINT's remainder is only correct when the divisor is monic.
    #[inline]
    fn rem_precomputed(
        self,
        other: &IntegerPolynomial,
        data: &RemPrecomputedData,
    ) -> IntegerPolynomial {
        rem_precomputed_helper(self.coefficients_asc().to_vec(), other, data)
    }
}

impl RemPrecomputedAssign<Self> for IntegerPolynomial {
    /// Replaces an [`IntegerPolynomial`] with its remainder by another, using data precomputed from
    /// the divisor, taking the divisor by value. The leading coefficient of the divisor must be 1
    /// or $-1$.
    ///
    /// $$
    /// a \gets r, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The data should be obtained with
    /// [`precompute_rem_data`](RemPrecomputed::precompute_rem_data). It holds the powers $x^{n-1},
    /// \ldots, x^{2n-2}$ reduced modulo the divisor, where $n$ is the divisor's length, so a
    /// dividend with fewer than $2n$ coefficients is reduced by adding a multiple of one power for
    /// each of its coefficients from $x^{n-1}$ up, and a longer one $n$ coefficients at a time,
    /// from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and of the precomputed powers.
    ///
    /// # Panics
    /// Panics if `data` was precomputed from a divisor with a different length. If `data` was not
    /// precomputed from the divisor, the result is meaningless.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::{
    ///     RemPrecomputed, RemPrecomputedAssign,
    /// };
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let q = IntegerPolynomial::from_str("x^2+x-1").unwrap();
    /// let data = IntegerPolynomial::precompute_rem_data(&q);
    ///
    /// let mut p = IntegerPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// p.rem_precomputed_assign(q.clone(), &data);
    /// assert_eq!(p.to_string(), "-2*x+4");
    ///
    /// let mut p = IntegerPolynomial::from_str("x^3+2*x+1").unwrap();
    /// p.rem_precomputed_assign(q, &data);
    /// assert_eq!(p.to_string(), "4*x");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_rem_powers_precomp` from `fmpz_poly/rem_powers_precomp.c`,
    /// FLINT 3.6.0, except that FLINT's remainder is only correct when the divisor is monic.
    #[inline]
    fn rem_precomputed_assign(&mut self, other: Self, data: &RemPrecomputedData) {
        *self = rem_precomputed_helper(take(self).into_coefficients_asc(), &other, data);
    }
}

impl RemPrecomputedAssign<&Self> for IntegerPolynomial {
    /// Replaces an [`IntegerPolynomial`] with its remainder by another, using data precomputed from
    /// the divisor, taking the divisor by reference. The leading coefficient of the divisor must be
    /// 1 or $-1$.
    ///
    /// $$
    /// a \gets r, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The data should be obtained with
    /// [`precompute_rem_data`](RemPrecomputed::precompute_rem_data). It holds the powers $x^{n-1},
    /// \ldots, x^{2n-2}$ reduced modulo the divisor, where $n$ is the divisor's length, so a
    /// dividend with fewer than $2n$ coefficients is reduced by adding a multiple of one power for
    /// each of its coefficients from $x^{n-1}$ up, and a longer one $n$ coefficients at a time,
    /// from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and of the precomputed powers.
    ///
    /// # Panics
    /// Panics if `data` was precomputed from a divisor with a different length. If `data` was not
    /// precomputed from the divisor, the result is meaningless.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::{
    ///     RemPrecomputed, RemPrecomputedAssign,
    /// };
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let q = IntegerPolynomial::from_str("x^2+x-1").unwrap();
    /// let data = IntegerPolynomial::precompute_rem_data(&q);
    ///
    /// let mut p = IntegerPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// p.rem_precomputed_assign(&q.clone(), &data);
    /// assert_eq!(p.to_string(), "-2*x+4");
    ///
    /// let mut p = IntegerPolynomial::from_str("x^3+2*x+1").unwrap();
    /// p.rem_precomputed_assign(&q, &data);
    /// assert_eq!(p.to_string(), "4*x");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_rem_powers_precomp` from `fmpz_poly/rem_powers_precomp.c`,
    /// FLINT 3.6.0, except that FLINT's remainder is only correct when the divisor is monic.
    #[inline]
    fn rem_precomputed_assign(&mut self, other: &Self, data: &RemPrecomputedData) {
        *self = rem_precomputed_helper(take(self).into_coefficients_asc(), other, data);
    }
}
