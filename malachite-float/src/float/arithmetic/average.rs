// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::float::basic::extended::ExtendedFloat;
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{Average, AverageAssign};
use malachite_base::num::basic::traits::{NegativeZero, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Floor, Nearest};

// Computes $(x+y)/2$, rounded to the given precision with the given rounding mode. Exactly one
// rounding is performed, so the result is the correctly-rounded average; neither the intermediate
// sum nor the halving can overflow or underflow when the true average is in range.
fn average_prec_round_helper(x: Float, y: Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    let (ex, ey) = match (x.get_exponent(), y.get_exponent()) {
        (Some(ex), Some(ey)) => (ex, ey),
        // At least one input is NaN, an infinity, or a zero. Halving leaves each of those
        // unchanged, so the sum is already the average, except when a zero is paired with a finite
        // nonzero value.
        _ => {
            return if !x.is_finite() || !y.is_finite() || (x == 0u32 && y == 0u32) {
                x.add_prec_round(y, prec, rm)
            } else if x == 0u32 {
                y.shl_prec_round(-1i64, prec, rm)
            } else {
                x.shl_prec_round(-1i64, prec, rm)
            };
        }
    };
    if ex > Float::MIN_EXPONENT && ey > Float::MIN_EXPONENT {
        // Neither input is in the lowest binade, so halving each is exact. Since $|x/2+y/2| \leq
        // \max(|x|,|y|)$, the sum cannot overflow except when the true average itself rounds out of
        // range, and the addition is the only rounding.
        (x >> 1u32).add_prec_round(y >> 1u32, prec, rm)
    } else {
        // An input is in the lowest binade, where halving would underflow.
        let (big, small, e_big, e_small) = if ex >= ey {
            (x, y, ex, ey)
        } else {
            (y, x, ey, ex)
        };
        let gap = u64::exact_from(i64::from(e_big) - i64::from(e_small));
        let p = max(big.get_prec().unwrap(), small.get_prec().unwrap());
        if gap > max(prec, p) + 2 {
            // `small` lies below the last bit of both `big / 2` and the target grid, so it acts as
            // a pure sticky bit: adding it whole rounds exactly as adding its half would. `big` is
            // not in the lowest binade, since `gap` is positive, so halving it is exact.
            (big >> 1u32).add_prec_round(small, prec, rm)
        } else {
            // The exact sum spans few enough bits to be affordable. Rebasing both exponents keeps
            // the arithmetic inside `ExtendedFloat` small, and there the halving is a free
            // adjustment of the extended exponent, so the final conversion is the only rounding.
            let w = gap + p + 1;
            let mut a = ExtendedFloat::from(big);
            let mut b = ExtendedFloat::from(small);
            let base = b.exp;
            a.exp -= base;
            b.exp -= base;
            let s = a.add_prec_ref_ref(&b, w).0;
            if s.x == 0u32 {
                // The inputs cancel exactly. `Rational` has no signed zero, so the sign follows the
                // rule the addition uses: an exactly zero sum is negative only when rounding toward
                // negative infinity.
                return (
                    if rm == Floor {
                        Float::NEGATIVE_ZERO
                    } else {
                        Float::ZERO
                    },
                    Equal,
                );
            }
            // Round to the target precision while the exponent is still small, which is equivalent
            // to rounding afterward because rounding to a precision is scale- invariant, and keeps
            // the intermediate rational small. Only then is the rebasing undone, together with the
            // halving; the conversion back applies that shift and handles a result that falls
            // outside `Float`'s exponent range, composing the two orderings.
            let (mut t, o) = ExtendedFloat::from_extended_float_prec_round_ref(&s, prec, rm);
            t.exp += base - 1;
            t.into_float_helper(prec, rm, o)
        }
    }
}

impl Float {
    /// Computes the average (arithmetic mean) of two [`Float`]s, rounding the result to the
    /// specified precision and with the specified rounding mode, and taking both [`Float`]s by
    /// value. An [`Ordering`] is also returned, indicating whether the returned value is less than,
    /// equal to, or greater than the exact average.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// $$
    /// f(x,y,p,m) = \frac{x+y}{2},
    /// $$
    ///
    /// rounded to $p$ bits in the direction specified by $m$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::average_prec`] instead. If
    /// you know that your target precision is the maximum of the precisions of the inputs, consider
    /// using [`Float::average_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the average is not exactly representable
    /// with the specified precision.
    ///
    /// # Examples
    /// See [here](super::average#average_prec_round).
    #[inline]
    pub fn average_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        average_prec_round_helper(self, other, prec, rm)
    }

    /// Computes the average (arithmetic mean) of two [`Float`]s, rounding the result to the
    /// specified precision and with the specified rounding mode, and taking the first [`Float`] by
    /// value and the second by reference. An [`Ordering`] is also returned, indicating whether the
    /// returned value is less than, equal to, or greater than the exact average.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// $$
    /// f(x,y,p,m) = \frac{x+y}{2},
    /// $$
    ///
    /// rounded to $p$ bits in the direction specified by $m$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::average_prec_val_ref`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::average_round_val_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the average is not exactly representable
    /// with the specified precision.
    ///
    /// # Examples
    /// See [here](super::average#average_prec_round).
    #[inline]
    pub fn average_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        average_prec_round_helper(self, other.clone(), prec, rm)
    }

    /// Computes the average (arithmetic mean) of two [`Float`]s, rounding the result to the
    /// specified precision and with the specified rounding mode, and taking the first [`Float`] by
    /// reference and the second by value. An [`Ordering`] is also returned, indicating whether the
    /// returned value is less than, equal to, or greater than the exact average.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// $$
    /// f(x,y,p,m) = \frac{x+y}{2},
    /// $$
    ///
    /// rounded to $p$ bits in the direction specified by $m$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::average_prec_ref_val`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::average_round_ref_val`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the average is not exactly representable
    /// with the specified precision.
    ///
    /// # Examples
    /// See [here](super::average#average_prec_round).
    #[inline]
    pub fn average_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        average_prec_round_helper(self.clone(), other, prec, rm)
    }

    /// Computes the average (arithmetic mean) of two [`Float`]s, rounding the result to the
    /// specified precision and with the specified rounding mode, and taking both [`Float`]s by
    /// reference. An [`Ordering`] is also returned, indicating whether the returned value is less
    /// than, equal to, or greater than the exact average.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// $$
    /// f(x,y,p,m) = \frac{x+y}{2},
    /// $$
    ///
    /// rounded to $p$ bits in the direction specified by $m$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::average_prec_ref_ref`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::average_round_ref_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the average is not exactly representable
    /// with the specified precision.
    ///
    /// # Examples
    /// See [here](super::average#average_prec_round).
    #[inline]
    pub fn average_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        average_prec_round_helper(self.clone(), other.clone(), prec, rm)
    }

    /// Computes the average (arithmetic mean) of two [`Float`]s, rounding the result to the nearest
    /// value of the specified precision, and taking both [`Float`]s by value. An [`Ordering`] is
    /// also returned, indicating whether the returned value is less than, equal to, or greater than
    /// the exact average. If a rounding is a tie, the value with fewer 1s in its binary expansion
    /// is chosen.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// $$
    /// f(x,y,p,m) = \frac{x+y}{2},
    /// $$
    ///
    /// rounded to $p$ bits in the direction specified by $m$.
    ///
    /// If you want to specify the rounding mode, consider using [`Float::average_prec_round`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// See [here](super::average#average_prec).
    #[inline]
    pub fn average_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        average_prec_round_helper(self, other, prec, Nearest)
    }

    /// Computes the average (arithmetic mean) of two [`Float`]s, rounding the result to the nearest
    /// value of the specified precision, and taking the first [`Float`] by value and the second by
    /// reference. An [`Ordering`] is also returned, indicating whether the returned value is less
    /// than, equal to, or greater than the exact average. If a rounding is a tie, the value with
    /// fewer 1s in its binary expansion is chosen.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// $$
    /// f(x,y,p,m) = \frac{x+y}{2},
    /// $$
    ///
    /// rounded to $p$ bits in the direction specified by $m$.
    ///
    /// If you want to specify the rounding mode, consider using
    /// [`Float::average_prec_round_val_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// See [here](super::average#average_prec).
    #[inline]
    pub fn average_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        average_prec_round_helper(self, other.clone(), prec, Nearest)
    }

    /// Computes the average (arithmetic mean) of two [`Float`]s, rounding the result to the nearest
    /// value of the specified precision, and taking the first [`Float`] by reference and the second
    /// by value. An [`Ordering`] is also returned, indicating whether the returned value is less
    /// than, equal to, or greater than the exact average. If a rounding is a tie, the value with
    /// fewer 1s in its binary expansion is chosen.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// $$
    /// f(x,y,p,m) = \frac{x+y}{2},
    /// $$
    ///
    /// rounded to $p$ bits in the direction specified by $m$.
    ///
    /// If you want to specify the rounding mode, consider using
    /// [`Float::average_prec_round_ref_val`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// See [here](super::average#average_prec).
    #[inline]
    pub fn average_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        average_prec_round_helper(self.clone(), other, prec, Nearest)
    }

    /// Computes the average (arithmetic mean) of two [`Float`]s, rounding the result to the nearest
    /// value of the specified precision, and taking both [`Float`]s by reference. An [`Ordering`]
    /// is also returned, indicating whether the returned value is less than, equal to, or greater
    /// than the exact average. If a rounding is a tie, the value with fewer 1s in its binary
    /// expansion is chosen.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// $$
    /// f(x,y,p,m) = \frac{x+y}{2},
    /// $$
    ///
    /// rounded to $p$ bits in the direction specified by $m$.
    ///
    /// If you want to specify the rounding mode, consider using
    /// [`Float::average_prec_round_ref_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// See [here](super::average#average_prec).
    #[inline]
    pub fn average_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        average_prec_round_helper(self.clone(), other.clone(), prec, Nearest)
    }

    /// Computes the average (arithmetic mean) of two [`Float`]s, rounding the result with the
    /// specified rounding mode, and taking both [`Float`]s by value. An [`Ordering`] is also
    /// returned, indicating whether the returned value is less than, equal to, or greater than the
    /// exact average.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// $$
    /// f(x,y,p,m) = \frac{x+y}{2},
    /// $$
    ///
    /// rounded to $p$ bits in the direction specified by $m$.
    ///
    /// If you want to specify the output precision, consider using [`Float::average_prec_round`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the average is not exactly representable with the maximum of
    /// the inputs' precisions.
    ///
    /// # Examples
    /// See [here](super::average#average_round).
    #[inline]
    pub fn average_round(self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        average_prec_round_helper(self, other, prec, rm)
    }

    /// Computes the average (arithmetic mean) of two [`Float`]s, rounding the result with the
    /// specified rounding mode, and taking the first [`Float`] by value and the second by
    /// reference. An [`Ordering`] is also returned, indicating whether the returned value is less
    /// than, equal to, or greater than the exact average.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// $$
    /// f(x,y,p,m) = \frac{x+y}{2},
    /// $$
    ///
    /// rounded to $p$ bits in the direction specified by $m$.
    ///
    /// If you want to specify the output precision, consider using
    /// [`Float::average_prec_round_val_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the average is not exactly representable with the maximum of
    /// the inputs' precisions.
    ///
    /// # Examples
    /// See [here](super::average#average_round).
    #[inline]
    pub fn average_round_val_ref(self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        average_prec_round_helper(self, other.clone(), prec, rm)
    }

    /// Computes the average (arithmetic mean) of two [`Float`]s, rounding the result with the
    /// specified rounding mode, and taking the first [`Float`] by reference and the second by
    /// value. An [`Ordering`] is also returned, indicating whether the returned value is less than,
    /// equal to, or greater than the exact average.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// $$
    /// f(x,y,p,m) = \frac{x+y}{2},
    /// $$
    ///
    /// rounded to $p$ bits in the direction specified by $m$.
    ///
    /// If you want to specify the output precision, consider using
    /// [`Float::average_prec_round_ref_val`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the average is not exactly representable with the maximum of
    /// the inputs' precisions.
    ///
    /// # Examples
    /// See [here](super::average#average_round).
    #[inline]
    pub fn average_round_ref_val(&self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        average_prec_round_helper(self.clone(), other, prec, rm)
    }

    /// Computes the average (arithmetic mean) of two [`Float`]s, rounding the result with the
    /// specified rounding mode, and taking both [`Float`]s by reference. An [`Ordering`] is also
    /// returned, indicating whether the returned value is less than, equal to, or greater than the
    /// exact average.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// $$
    /// f(x,y,p,m) = \frac{x+y}{2},
    /// $$
    ///
    /// rounded to $p$ bits in the direction specified by $m$.
    ///
    /// If you want to specify the output precision, consider using
    /// [`Float::average_prec_round_ref_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the average is not exactly representable with the maximum of
    /// the inputs' precisions.
    ///
    /// # Examples
    /// See [here](super::average#average_round).
    #[inline]
    pub fn average_round_ref_ref(&self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        average_prec_round_helper(self.clone(), other.clone(), prec, rm)
    }

    /// Computes the average (arithmetic mean) of two [`Float`]s in place, rounding the result to
    /// the specified precision and with the specified rounding mode, taking the [`Float`] on the
    /// right-hand side by value. An [`Ordering`] is returned, indicating whether the assigned value
    /// is less than, equal to, or greater than the exact average.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the average is not exactly representable
    /// with the specified precision.
    ///
    /// # Examples
    /// See [here](super::average#average_prec_round_assign).
    #[inline]
    pub fn average_prec_round_assign(
        &mut self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (avg, o) = average_prec_round_helper(core::mem::take(self), other, prec, rm);
        *self = avg;
        o
    }

    /// Computes the average (arithmetic mean) of two [`Float`]s in place, rounding the result to
    /// the specified precision and with the specified rounding mode, taking the [`Float`] on the
    /// right-hand side by reference. An [`Ordering`] is returned, indicating whether the assigned
    /// value is less than, equal to, or greater than the exact average.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the average is not exactly representable
    /// with the specified precision.
    ///
    /// # Examples
    /// See [here](super::average#average_prec_round_assign_ref).
    #[inline]
    pub fn average_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (avg, o) = average_prec_round_helper(core::mem::take(self), other.clone(), prec, rm);
        *self = avg;
        o
    }

    /// Computes the average (arithmetic mean) of two [`Float`]s in place, rounding the result to
    /// the nearest value of the specified precision, taking the [`Float`] on the right-hand side by
    /// value. An [`Ordering`] is returned, indicating whether the assigned value is less than,
    /// equal to, or greater than the exact average.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// See [here](super::average#average_prec_assign).
    #[inline]
    pub fn average_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        let (avg, o) = average_prec_round_helper(core::mem::take(self), other, prec, Nearest);
        *self = avg;
        o
    }

    /// Computes the average (arithmetic mean) of two [`Float`]s in place, rounding the result to
    /// the nearest value of the specified precision, taking the [`Float`] on the right-hand side by
    /// reference. An [`Ordering`] is returned, indicating whether the assigned value is less than,
    /// equal to, or greater than the exact average.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// See [here](super::average#average_prec_assign_ref).
    #[inline]
    pub fn average_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        let (avg, o) =
            average_prec_round_helper(core::mem::take(self), other.clone(), prec, Nearest);
        *self = avg;
        o
    }

    /// Computes the average (arithmetic mean) of two [`Float`]s in place, rounding the result with
    /// the specified rounding mode and taking the [`Float`] on the right-hand side by value. An
    /// [`Ordering`] is returned, indicating whether the assigned value is less than, equal to, or
    /// greater than the exact average.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the average is not exactly representable with the maximum of
    /// the inputs' precisions.
    ///
    /// # Examples
    /// See [here](super::average#average_round_assign).
    #[inline]
    pub fn average_round_assign(&mut self, other: Self, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        let (avg, o) = average_prec_round_helper(core::mem::take(self), other, prec, rm);
        *self = avg;
        o
    }

    /// Computes the average (arithmetic mean) of two [`Float`]s in place, rounding the result with
    /// the specified rounding mode and taking the [`Float`] on the right-hand side by reference. An
    /// [`Ordering`] is returned, indicating whether the assigned value is less than, equal to, or
    /// greater than the exact average.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the average is not exactly representable with the maximum of
    /// the inputs' precisions.
    ///
    /// # Examples
    /// See [here](super::average#average_round_assign_ref).
    #[inline]
    pub fn average_round_assign_ref(&mut self, other: &Self, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        let (avg, o) = average_prec_round_helper(core::mem::take(self), other.clone(), prec, rm);
        *self = avg;
        o
    }
}

impl Average<Self> for Float {
    type Output = Self;

    /// Computes the average (arithmetic mean) of two [`Float`]s, taking both [`Float`]s by value.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs, and the result
    /// is rounded to nearest, with ties going to the value whose binary expansion has fewer 1s. If
    /// you want to specify the precision or the rounding mode, consider using
    /// [`Float::average_prec_round`] instead; that form also reports how the returned value
    /// compares with the exact average.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// $$
    /// f(x,y) = \frac{x+y}{2},
    /// $$
    ///
    /// rounded to nearest.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// See [here](super::average#average).
    #[inline]
    fn average(self, other: Self) -> Self {
        let prec = max(self.significant_bits(), other.significant_bits());
        average_prec_round_helper(self, other, prec, Nearest).0
    }
}

impl Average<&Self> for Float {
    type Output = Self;

    /// Computes the average (arithmetic mean) of two [`Float`]s, taking the first [`Float`] by
    /// value and the second by reference.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs, and the result
    /// is rounded to nearest, with ties going to the value whose binary expansion has fewer 1s. If
    /// you want to specify the precision or the rounding mode, consider using
    /// [`Float::average_prec_round_val_ref`] instead; that form also reports how the returned value
    /// compares with the exact average.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// $$
    /// f(x,y) = \frac{x+y}{2},
    /// $$
    ///
    /// rounded to nearest.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// See [here](super::average#average).
    #[inline]
    fn average(self, other: &Self) -> Self {
        let prec = max(self.significant_bits(), other.significant_bits());
        average_prec_round_helper(self, other.clone(), prec, Nearest).0
    }
}

impl Average<Float> for &Float {
    type Output = Float;

    /// Computes the average (arithmetic mean) of two [`Float`]s, taking the first [`Float`] by
    /// reference and the second by value.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs, and the result
    /// is rounded to nearest, with ties going to the value whose binary expansion has fewer 1s. If
    /// you want to specify the precision or the rounding mode, consider using
    /// [`Float::average_prec_round_ref_val`] instead; that form also reports how the returned value
    /// compares with the exact average.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// $$
    /// f(x,y) = \frac{x+y}{2},
    /// $$
    ///
    /// rounded to nearest.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// See [here](super::average#average).
    #[inline]
    fn average(self, other: Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        average_prec_round_helper(self.clone(), other, prec, Nearest).0
    }
}

impl Average<&Float> for &Float {
    type Output = Float;

    /// Computes the average (arithmetic mean) of two [`Float`]s, taking both [`Float`]s by
    /// reference.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs, and the result
    /// is rounded to nearest, with ties going to the value whose binary expansion has fewer 1s. If
    /// you want to specify the precision or the rounding mode, consider using
    /// [`Float::average_prec_round_ref_ref`] instead; that form also reports how the returned value
    /// compares with the exact average.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// $$
    /// f(x,y) = \frac{x+y}{2},
    /// $$
    ///
    /// rounded to nearest.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// See [here](super::average#average).
    #[inline]
    fn average(self, other: &Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        average_prec_round_helper(self.clone(), other.clone(), prec, Nearest).0
    }
}

impl AverageAssign<Self> for Float {
    /// Computes the average (arithmetic mean) of two [`Float`]s in place, taking the [`Float`] on
    /// the right-hand side by value.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs, and the result
    /// is rounded to nearest.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// See [here](super::average#average_assign).
    #[inline]
    fn average_assign(&mut self, other: Self) {
        let prec = max(self.significant_bits(), other.significant_bits());
        *self = average_prec_round_helper(core::mem::take(self), other, prec, Nearest).0;
    }
}

impl AverageAssign<&Self> for Float {
    /// Computes the average (arithmetic mean) of two [`Float`]s in place, taking the [`Float`] on
    /// the right-hand side by reference.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs, and the result
    /// is rounded to nearest.
    ///
    /// The average is computed as though with unbounded exponent range and rounded exactly once, so
    /// a sum that would overflow, or a halving that would underflow, does not spoil a result that
    /// is itself in range.
    ///
    /// If either input is `NaN`, the result is `NaN`; the average of an infinity and any value
    /// other than the opposite infinity is that infinity, and the average of the two opposite
    /// infinities is `NaN`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// See [here](super::average#average_assign).
    #[inline]
    fn average_assign(&mut self, other: &Self) {
        let prec = max(self.significant_bits(), other.significant_bits());
        *self = average_prec_round_helper(core::mem::take(self), other.clone(), prec, Nearest).0;
    }
}
