// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::add::{
    add_helper, add_helper_with, finish_significand, round_optional, significand_prec,
};
use crate::float::MIN_EXPONENT_I64;
use alloc::borrow::Cow;
use alloc::vec::Vec;
use core::cmp::Ordering::{self, *};
use core::iter::Sum;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, One, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float::sum::{
    FloatSumInput, FloatSumResult, sum_float_significands,
};

// A value rounded to odd at `q` bits, for q >= 2, given the value truncated to q - 1 bits and the
// `Ordering` of the truncation: if the truncation is inexact, its q-th bit is set to 1. Rounding a
// value to odd at q = p + 2 bits and then to p bits, in any rounding mode, gives the same result,
// and the same `Ordering`, as rounding the value to p bits directly.
fn finish_round_to_odd(truncated: BigExponentFloat, o: Ordering, q: u64) -> BigExponentFloat {
    if o == Equal {
        return truncated;
    }
    let BigExponentFloat(Finite { sign, exponent, .. }) = &truncated else {
        unreachable!()
    };
    let bit = BigExponentFloat::from_parts(
        *sign,
        exponent - Integer::from(q) + Integer::ONE,
        Natural::ONE,
    );
    add_helper_with(Cow::Owned(truncated), Cow::Owned(bit), false, None, false).0
}

// The sum x + y rounded to odd at `q` bits, for q >= 2. The addition is rounded, so the operands'
// exponents may be arbitrarily far apart.
fn add_round_to_odd(
    x: BigExponentFloat,
    y: BigExponentFloat,
    q: u64,
    use_kernels: bool,
) -> BigExponentFloat {
    let (truncated, o) = add_helper_with(
        Cow::Owned(x),
        Cow::Owned(y),
        false,
        Some((q - 1, Down)),
        use_kernels,
    );
    finish_round_to_odd(truncated, o, q)
}

// The sum of one or more finite nonzero terms, rounded to `prec` bits with `rm`, where `top` is the
// largest of the terms' exponents. One or two terms are rounded or added directly; more are summed
// by `Float`'s summation kernel, MPFR's `mpfr_sum`, which keeps an accumulator of only about `prec`
// bits however far apart the terms are. The kernel takes `i64` exponents, which are given as
// offsets from `top`; it is only used with exponents in `Float`'s range, so if some term's exponent
// is more than `-Float::MIN_EXPONENT` below `top`, `None` is returned.
fn kernel_sum(
    terms: &[&BigExponentFloat],
    top: &Integer,
    prec: u64,
    rm: RoundingMode,
) -> Option<(BigExponentFloat, Ordering)> {
    match terms {
        [x] => return Some(round_optional((*x).clone(), Some((prec, rm)))),
        [x, y] => {
            return Some(add_helper_with(
                Cow::Borrowed(*x),
                Cow::Borrowed(*y),
                false,
                Some((prec, rm)),
                true,
            ));
        }
        _ => {}
    }
    let mut inputs = Vec::with_capacity(terms.len());
    for x in terms {
        let BigExponentFloat(Finite {
            sign,
            exponent,
            significand,
        }) = x
        else {
            unreachable!()
        };
        let exp = i64::try_from(&(exponent - top))
            .ok()
            .filter(|&exp| exp >= MIN_EXPONENT_I64)?;
        inputs.push(FloatSumInput {
            sign: *sign,
            exp,
            prec: significand_prec(significand),
            significand,
        });
    }
    // The kernel does not take `Exact`; any other mode reports whether the sum is exact.
    Some(
        match sum_float_significands(&inputs, prec, if rm == Exact { Down } else { rm }) {
            FloatSumResult::Zero => (BigExponentFloat::ZERO, Equal),
            FloatSumResult::Regular {
                sign,
                exp,
                significand,
                o,
            } => {
                assert!(rm != Exact || o == Equal, "Inexact BigExponentFloat sum");
                (
                    BigExponentFloat(Finite {
                        sign,
                        exponent: top + Integer::from(exp),
                        significand: finish_significand(significand),
                    }),
                    o,
                )
            }
        },
    )
}

// The exact sum of one or more finite nonzero terms.
fn exact_sum(terms: &[&BigExponentFloat]) -> BigExponentFloat {
    terms.iter().fold(BigExponentFloat::ZERO, |sum, x| {
        add_helper_with(Cow::Owned(sum), Cow::Borrowed(*x), false, None, false).0
    })
}

// The sum of one or more finite nonzero terms rounded to odd at `q` bits, or, without `use_kernels`
// or if `kernel_sum` cannot be used, the exact sum. Either is as good as the other in
// `sum_helper_with`.
fn cluster_sum(
    terms: &[&BigExponentFloat],
    top: &Integer,
    q: u64,
    use_kernels: bool,
) -> BigExponentFloat {
    if use_kernels && let Some((truncated, o)) = kernel_sum(terms, top, q - 1, Down) {
        finish_round_to_odd(truncated, o, q)
    } else {
        exact_sum(terms)
    }
}

// The sum of the terms, rounded to `prec` bits with `rm`. The special cases are those of FLINT's
// `arf_sum`.
//
// With `use_kernels`, if no term's exponent is too far below the largest one, the sum is computed
// by `kernel_sum`. Otherwise, the finite nonzero terms are taken in order of their lowest set bits
// and grouped into clusters: a term starts a new cluster when its lowest set bit lies more than
// `gap` bits above the highest bit any sum of the current cluster can reach. Everything below the
// current cluster is kept as a running total rounded to odd at `prec + 2` bits. That total is far
// below the current cluster, so if the cluster's exact sum is nonzero, the total affects its
// rounding at `prec + 2` bits only through its sign; and if the cluster's sum is zero, the total is
// the sum so far. Either way, the total rounded to odd is as good as the exact total, and the final
// rounding to `prec` bits is correct.
//
// For the same reason, a cluster's sum may itself be rounded to odd at `prec + 2` bits before the
// total is added to it: the cluster's sum is zero only if its rounded sum is, and otherwise, since
// both the cluster's exact sum and the next multiples of 2^(q - 1) bits around it are multiples of
// the cluster's lowest bit, which lies above the total's magnitude, adding the total to either
// value gives the same result rounded to odd. With `use_kernels`, clusters are summed by
// `kernel_sum` when possible; without it, or when a cluster is too wide for it, exactly.
fn sum_helper_with<'a, I: Iterator<Item = &'a BigExponentFloat>>(
    xs: I,
    prec: u64,
    rm: RoundingMode,
    use_kernels: bool,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    let mut positive_infinity = false;
    let mut negative_infinity = false;
    let mut terms: Vec<&BigExponentFloat> = Vec::new();
    let mut top: Option<&Integer> = None;
    for x in xs {
        match x {
            BigExponentFloat(NaN) => return (BigExponentFloat::NAN, Equal),
            BigExponentFloat(Infinity { sign: true }) => positive_infinity = true,
            BigExponentFloat(Infinity { sign: false }) => negative_infinity = true,
            BigExponentFloat(Zero) => {}
            BigExponentFloat(Finite { exponent, .. }) => {
                if top.is_none_or(|top| exponent > top) {
                    top = Some(exponent);
                }
                terms.push(x);
            }
        }
    }
    match (positive_infinity, negative_infinity) {
        (true, true) => return (BigExponentFloat::NAN, Equal),
        (true, false) => return (BigExponentFloat::INFINITY, Equal),
        (false, true) => return (BigExponentFloat::NEGATIVE_INFINITY, Equal),
        (false, false) => {}
    }
    let Some(top) = top else {
        return (BigExponentFloat::ZERO, Equal);
    };
    if use_kernels && let Some(result) = kernel_sum(&terms, top, prec, rm) {
        return result;
    }
    let low = |x: &BigExponentFloat| {
        let BigExponentFloat(Finite {
            exponent,
            significand,
            ..
        }) = x
        else {
            unreachable!()
        };
        exponent - Integer::from(significand_prec(significand))
    };
    terms.sort_by_cached_key(|x| low(x));
    let q = prec + 2;
    let gap = Integer::from(q + 4 + u64::exact_from(terms.len()).significant_bits());
    let mut total = BigExponentFloat::ZERO;
    let mut start = 0;
    let mut cluster_top = terms[0].get_exponent().unwrap();
    // Any sum of the cluster's terms is less than 2^(cluster_top + count bits) in magnitude; the
    // count is accounted for in `gap`.
    for i in 1..terms.len() {
        let exponent = terms[i].get_exponent().unwrap();
        if low(terms[i]) > cluster_top + &gap {
            let cluster = cluster_sum(&terms[start..i], cluster_top, q, use_kernels);
            total = add_round_to_odd(cluster, total, q, use_kernels);
            start = i;
            cluster_top = exponent;
        } else if exponent > cluster_top {
            cluster_top = exponent;
        }
    }
    let cluster = &terms[start..];
    if use_kernels
        && total == 0u32
        && let Some(result) = kernel_sum(cluster, cluster_top, prec, rm)
    {
        return result;
    }
    let cluster = cluster_sum(cluster, cluster_top, q, use_kernels);
    add_helper_with(
        Cow::Owned(cluster),
        Cow::Owned(total),
        false,
        Some((prec, rm)),
        use_kernels,
    )
}

// The sum of the terms, rounded to `prec` bits with `rm`.
#[inline]
pub(crate) fn sum_helper<'a, I: Iterator<Item = &'a BigExponentFloat>>(
    xs: I,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    sum_helper_with(xs, prec, rm, true)
}

crate_test_fn! {
    // The sum of the terms, rounded to `prec` bits with `rm`, without `Float`'s kernels: the terms
    // are grouped into clusters as in `sum_helper_with`, and each cluster is summed exactly. Used
    // to check and benchmark the kernel path.
    sum_prec_round_generic(
        xs: &[BigExponentFloat],
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        sum_helper_with(xs.iter(), prec, rm, false)
    }
}

impl BigExponentFloat {
    /// Adds a slice of [`BigExponentFloat`]s, rounding the result to the specified precision and
    /// with the specified rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// rounded sum is less than, equal to, or greater than the exact sum. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The sum is rounded only once, at the end, and the significand of no intermediate value spans
    /// the gaps between the terms' exponents, however large they are.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f((x_i)_{i=0}^{n-1},p,m) = \sum_{i=0}^{n-1} x_i+\varepsilon.
    /// $$
    /// - If $\sum_{i=0}^{n-1} x_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\sum_{i=0}^{n-1} x_i$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\sum_{i=0}^{n-1} x_i|\rfloor-p+1}$.
    /// - If $\sum_{i=0}^{n-1} x_i$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |\sum_{i=0}^{n-1} x_i|\rfloor-p}$.
    ///
    /// Special cases:
    /// - The sum of an empty slice is 0.
    /// - If any term is NaN, or if both $\infty$ and $-\infty$ are terms, the sum is NaN.
    /// - Otherwise, if $\infty$ or $-\infty$ is a term, the sum is that infinity.
    /// - If the exact sum of finite terms is zero, the sum is 0.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::sum_prec`]
    /// instead. If you want the exact sum, consider using [`Sum::sum`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is the sum of the
    /// significant bits of the elements of `xs` plus $n$ times `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the sum cannot be represented exactly
    /// with `prec` bits.
    ///
    /// This is FLINT's `arf_sum`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let xs =
    ///     [BigExponentFloat::from(PI), BigExponentFloat::from(E), BigExponentFloat::from(SQRT_2)];
    ///
    /// let (result, o) = BigExponentFloat::sum_prec_round(&xs, 5, Floor);
    /// assert_eq!(format!("{result:?}"), "0x7.4");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::sum_prec_round(&xs, 5, Ceiling);
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::sum_prec_round(&xs, 5, Nearest);
    /// assert_eq!(format!("{result:?}"), "0x7.4");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::sum_prec_round(&xs, 20, Floor);
    /// assert_eq!(format!("{result:?}"), "0x7.462a8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::sum_prec_round(&xs, 20, Ceiling);
    /// assert_eq!(format!("{result:?}"), "0x7.462b");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::sum_prec_round(&xs, 20, Nearest);
    /// assert_eq!(format!("{result:?}"), "0x7.462a8");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sum_prec_round(xs: &[Self], prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        sum_helper(xs.iter(), prec, rm)
    }

    /// Adds a slice of [`BigExponentFloat`]s, rounding the result to the nearest value of the
    /// specified precision. An [`Ordering`] is also returned, indicating whether the rounded sum is
    /// less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The sum is rounded only once, at the end, and the significand of no intermediate value spans
    /// the gaps between the terms' exponents, however large they are.
    ///
    /// $$
    /// f((x_i)_{i=0}^{n-1},p) = \sum_{i=0}^{n-1} x_i+\varepsilon.
    /// $$
    /// - If $\sum_{i=0}^{n-1} x_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\sum_{i=0}^{n-1} x_i$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\sum_{i=0}^{n-1} x_i|\rfloor-p}$.
    ///
    /// Special cases:
    /// - The sum of an empty slice is 0.
    /// - If any term is NaN, or if both $\infty$ and $-\infty$ are terms, the sum is NaN.
    /// - Otherwise, if $\infty$ or $-\infty$ is a term, the sum is that infinity.
    /// - If the exact sum of finite terms is zero, the sum is 0.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::sum_prec_round`] instead. If you want the exact sum, consider using
    /// [`Sum::sum`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is the sum of the
    /// significant bits of the elements of `xs` plus $n$ times `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let xs =
    ///     [BigExponentFloat::from(PI), BigExponentFloat::from(E), BigExponentFloat::from(SQRT_2)];
    ///
    /// let (result, o) = BigExponentFloat::sum_prec(&xs, 5);
    /// assert_eq!(format!("{result:?}"), "0x7.4");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::sum_prec(&xs, 20);
    /// assert_eq!(format!("{result:?}"), "0x7.462a8");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sum_prec(xs: &[Self], prec: u64) -> (Self, Ordering) {
        sum_helper(xs.iter(), prec, Nearest)
    }
}

impl Sum<Self> for BigExponentFloat {
    /// Adds the [`BigExponentFloat`]s produced by an iterator, taken by value. The sum is exact.
    ///
    /// $$
    /// f((x_i)_{i=0}^{n-1}) = \sum_{i=0}^{n-1} x_i.
    /// $$
    ///
    /// Special cases:
    /// - The sum of an empty slice is 0.
    /// - If any term is NaN, or if both $\infty$ and $-\infty$ are terms, the sum is NaN.
    /// - Otherwise, if $\infty$ or $-\infty$ is a term, the sum is that infinity.
    /// - If the exact sum of finite terms is zero, the sum is 0.
    ///
    /// When the terms are finite and nonzero, the significand of the sum spans the gaps between
    /// their exponents. To bound its size, use [`BigExponentFloat::sum_prec_round`] or
    /// [`BigExponentFloat::sum_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the significant bits of
    /// the terms plus the difference between the largest exponent and the smallest.
    ///
    /// # Panics
    /// Panics if two terms' exponents differ by $2^{64}$ or more and both are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Sum;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let xs = [
    ///     BigExponentFloat::from(1.5),
    ///     BigExponentFloat::from(0.25),
    ///     BigExponentFloat::from(3.0),
    /// ];
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::sum(xs.into_iter())),
    ///     "0x4.c"
    /// );
    /// ```
    #[inline]
    fn sum<I: Iterator<Item = Self>>(xs: I) -> Self {
        xs.fold(Self::ZERO, |sum, x| {
            add_helper(Cow::Owned(sum), Cow::Owned(x), false, None).0
        })
    }
}

impl<'a> Sum<&'a Self> for BigExponentFloat {
    /// Adds the [`BigExponentFloat`]s produced by an iterator, taken by reference. The sum is
    /// exact.
    ///
    /// $$
    /// f((x_i)_{i=0}^{n-1}) = \sum_{i=0}^{n-1} x_i.
    /// $$
    ///
    /// Special cases:
    /// - The sum of an empty slice is 0.
    /// - If any term is NaN, or if both $\infty$ and $-\infty$ are terms, the sum is NaN.
    /// - Otherwise, if $\infty$ or $-\infty$ is a term, the sum is that infinity.
    /// - If the exact sum of finite terms is zero, the sum is 0.
    ///
    /// When the terms are finite and nonzero, the significand of the sum spans the gaps between
    /// their exponents. To bound its size, use [`BigExponentFloat::sum_prec_round`] or
    /// [`BigExponentFloat::sum_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the significant bits of
    /// the terms plus the difference between the largest exponent and the smallest.
    ///
    /// # Panics
    /// Panics if two terms' exponents differ by $2^{64}$ or more and both are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Sum;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let xs = [
    ///     BigExponentFloat::from(1.5),
    ///     BigExponentFloat::from(0.25),
    ///     BigExponentFloat::from(3.0),
    /// ];
    /// assert_eq!(format!("{:?}", BigExponentFloat::sum(xs.iter())), "0x4.c");
    /// ```
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(xs: I) -> Self {
        xs.fold(Self::ZERO, |sum, x| {
            add_helper(Cow::Owned(sum), Cow::Borrowed(x), false, None).0
        })
    }
}
