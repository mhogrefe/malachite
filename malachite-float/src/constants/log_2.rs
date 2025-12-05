// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 1999, 2001-2024 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use alloc::vec;
use core::cmp::Ordering;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::CeilingLogBase2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;

// Auxiliary function: Compute the terms from n1 to n2 (excluded) 3 / 4 * sum((-1) ^ n * n! ^ 2 / 2
// ^ n / (2 * n + 1)!, n = n1...n2 - 1).
//
// Numerator is T[0], denominator is Q[0], Compute P[0] only when need_P is non-zero.
//
// Need 1 + ceil(log(n2 - n1) / log(2)) cells in T[], P[], Q[].
//
// This is S from const_log2.c, MPFR 4.2.0.
fn sum(t: &mut [Integer], p: &mut [Integer], q: &mut [Integer], n1: u64, n2: u64, need_p: bool) {
    if n2 == n1 + 1 {
        p[0] = if n1 == 0 {
            const { Integer::const_from_unsigned(3) }
        } else {
            -Integer::from(n1)
        };
        q[0] = ((Integer::from(n1) << 1u32) + Integer::ONE) << 2u32;
        t[0].clone_from(&p[0]);
    } else {
        let m = (n1 >> 1) + (n2 >> 1) + (n1 & 1 & n2);
        sum(t, p, q, n1, m, true);
        let (t_head, t_tail) = t.split_first_mut().unwrap();
        let (p_head, p_tail) = p.split_first_mut().unwrap();
        let (q_head, q_tail) = q.split_first_mut().unwrap();
        sum(t_tail, p_tail, q_tail, m, n2, need_p);
        *t_head *= &q_tail[0];
        t_tail[0] *= &*p_head;
        *t_head += &t_tail[0];
        if need_p {
            *p_head *= &p_tail[0];
        }
        *q_head *= &q_tail[0];
        // remove common trailing zeros if any
        let mut tz = t_head.trailing_zeros().unwrap();
        if tz != 0 {
            let mut qz = q_head.trailing_zeros().unwrap();
            if qz < tz {
                tz = qz;
            }
            if need_p {
                qz = p_head.trailing_zeros().unwrap();
                if qz < tz {
                    tz = qz;
                }
            }
            // now tz = min(val(T), val(Q), val(P))
            if tz != 0 {
                *t_head >>= tz;
                *q_head >>= tz;
                if need_p {
                    *p_head >>= tz;
                }
            }
        }
    }
}

impl Float {
    /// Returns an approximation to the natural logarithm of 2, with the given precision and rounded
    /// using the given [`RoundingMode`]. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than or greater than the exact value of the constant. (Since the
    /// constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// L = \log 2.
    /// $$
    ///
    /// The constant is irrational.
    ///
    /// The output has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (l2, o) = Float::log_2_prec_round(100, Floor);
    /// assert_eq!(l2.to_string(), "0.693147180559945309417232121458");
    /// assert_eq!(o, Less);
    ///
    /// let (l2, o) = Float::log_2_prec_round(100, Ceiling);
    /// assert_eq!(l2.to_string(), "0.693147180559945309417232121459");
    /// assert_eq!(o, Greater);
    /// ```
    ///
    /// This is mpfr_const_log2_internal from const_log2.c, MPFR 4.2.0.
    pub fn log_2_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let mut working_prec = prec + prec.ceiling_log_base_2() + 3;
        let mut increment = Limb::WIDTH;
        loop {
            let big_n = working_prec / 3 + 1;
            // the following are needed for error analysis (see algorithms.tex)
            assert!(working_prec >= 3 && big_n >= 2);
            let lg_big_n = usize::wrapping_from(big_n.ceiling_log_base_2()) + 1;
            let mut scratch = vec![Integer::ZERO; 3 * lg_big_n];
            split_into_chunks_mut!(scratch, lg_big_n, [t, p], q);
            sum(t, p, q, 0, big_n, false);
            let mut t0 = Integer::ZERO;
            let mut q0 = Integer::ZERO;
            swap(&mut t0, &mut t[0]);
            swap(&mut q0, &mut q[0]);
            let log_2 = Self::from_integer_prec(t0, working_prec).0
                / Self::from_integer_prec(q0, working_prec).0;
            if float_can_round(log_2.significand_ref().unwrap(), working_prec - 2, prec, rm) {
                return Self::from_float_prec_round(log_2, prec, rm);
            }
            working_prec += increment;
            increment = working_prec >> 1;
        }
    }

    /// Returns an approximation to the natural logarithm of 2, with the given precision and rounded
    /// to the nearest [`Float`] of that precision. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than or greater than the exact value of the constant.
    /// (Since the constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// L = \log 2.
    /// $$
    ///
    /// The constant is irrational.
    ///
    /// The output has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (l2, o) = Float::log_2_prec(1);
    /// assert_eq!(l2.to_string(), "0.5");
    /// assert_eq!(o, Less);
    ///
    /// let (l2, o) = Float::log_2_prec(10);
    /// assert_eq!(l2.to_string(), "0.693");
    /// assert_eq!(o, Greater);
    ///
    /// let (l2, o) = Float::log_2_prec(100);
    /// assert_eq!(l2.to_string(), "0.693147180559945309417232121458");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn log_2_prec(prec: u64) -> (Self, Ordering) {
        Self::log_2_prec_round(prec, Nearest)
    }
}
