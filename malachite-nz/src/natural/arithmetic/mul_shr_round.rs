// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Large;
use crate::natural::Natural;
use crate::natural::arithmetic::mul::mul_high::{
    limbs_mul_high_same_length, limbs_mul_high_same_length_scratch_len,
};
use crate::platform::Limb;
use alloc::vec;
use core::cmp::{
    Ordering::{self, *},
    min,
};
use malachite_base::num::arithmetic::traits::{
    CheckedLogBase2, ModPowerOf2, MulShrRound, MulShrRoundAssign, Parity, ShrRound,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};

// Enough guard bits between the cut and the bottom of the short product's valid region to absorb
// its error bound of less than `l + 2` ulps: `64 * l - d >= 66` leaves at least 65 bits of guard
// below the half-ulp position, against an error of at most 64 bits' worth.
const MUL_SHR_ROUND_GUARD: u64 = 66;

// Below this many limbs of product, the short-product window's constant costs -- its allocation and
// the `Natural` arithmetic of the determinacy check -- outweigh what it saves, and the full product
// is faster. Measured crossover on this machine: mid-size inputs were 10-20% slower through the
// window at 15-27 limbs and 50% faster by 41.
const MUL_SHR_ROUND_SHORT_THRESHOLD: usize = 30;

// The top `l` limbs of a `Natural` known to have more than `l` limbs.
fn top_limbs(x: &Natural, l: usize) -> &[Limb] {
    match x {
        Natural(Large(xs)) => &xs[xs.len() - l..],
        _ => unreachable!(),
    }
}

// Rounds the product of two `Natural`s, shifted right by `bits`, according to `rm`. The returned
// `Ordering` compares the rounded value to the exact one.
//
// The point of fusing is to avoid the full product when most of it is discarded. Three regimes:
//
// - If the shift is exact -- decidable up front from the factors' trailing zeros, since $\nu_2(xy)
//   = \nu_2(x) + \nu_2(y)$ -- nothing is really discarded, and the result is the full product of
//   the operands with their trailing zeros removed. No short product can help here, because every
//   bit of both operands ends up in the result.
//
// - If the discarded part is small, the full product plus a shift is within a constant of optimal,
//   and that is what happens.
//
// - Otherwise both operands are truncated to their top `l` limbs, sized so that the kept bits plus
//   a guard band fit in the top half of a `2l`-limb window, and Mulders' short product approximates
//   that window from below with error less than `l + 2` ulps. If the guard band is not within the
//   error of carrying -- checked exactly -- the kept bits and the half-ulp bit are exact, and the
//   sticky bits below come from the trailing-zeros identity, so every rounding mode resolves. The
//   window fails to decide only when the product's bits just below the cut are nearly all ones, in
//   which case the exact product is computed after all, Ziv-style.
crate_test_fn! {mul_shr_round_ref_ref(
    x: &Natural,
    y: &Natural,
    bits: u64,
    rm: RoundingMode
) -> (Natural, Ordering) {
    if *x == 0u32 || *y == 0u32 {
        return (Natural::ZERO, Equal);
    }
    let tzx = x.trailing_zeros().unwrap();
    let tzy = y.trailing_zeros().unwrap();
    if tzx + tzy >= bits {
        // The shift is exact. All information survives, so take the full product of the operands
        // with the shift distributed onto their trailing zeros.
        let a = min(tzx, bits);
        let b = min(tzy, bits - a);
        return ((x >> a) * (y >> b), Equal);
    }
    assert!(
        rm != Exact,
        "Product right shift is not exact: {x} * {y} >> {bits}"
    );
    let n = usize::exact_from(x.limb_count());
    let m = usize::exact_from(y.limb_count());
    let total_limb_bits = u64::exact_from(n + m) << Limb::LOG_WIDTH;
    if bits >= total_limb_bits {
        // The product is below 2^bits, so the exact value is in (0, 1).
        return match rm {
            Down | Floor => (Natural::ZERO, Less),
            Up | Ceiling => (Natural::ONE, Greater),
            Nearest => {
                if bits > total_limb_bits
                    || x.significant_bits() + y.significant_bits() < total_limb_bits
                {
                    // The product is below 2^(bits - 1), so the value is below one half.
                    (Natural::ZERO, Less)
                } else {
                    // The product's top bit may or may not reach 2^(bits - 1); this corner is rare
                    // enough that the exact product settles it. The product is at least 2^(bits -
                    // 1) iff it has at least `bits` significant bits, and equals it -- a half,
                    // rounding to the even neighbor, 0 -- iff it is that power of 2.
                    let p = x * y;
                    if p.significant_bits() < bits || p.checked_log_base_2() == Some(bits - 1) {
                        (Natural::ZERO, Less)
                    } else {
                        (Natural::ONE, Greater)
                    }
                }
            }
            Exact => unreachable!(),
        };
    }
    // The kept span, measured from the top of the product's limb frame.
    let d = total_limb_bits - bits;
    let l = usize::exact_from((d + MUL_SHR_ROUND_GUARD).div_ceil(Limb::WIDTH));
    if n + m < MUL_SHR_ROUND_SHORT_THRESHOLD || l >= min(n, m) {
        // Truncation would discard nothing (or the shorter operand is smaller than the window), so
        // the sub-cut triangle is small and the full product is the right tool.
        return (x * y).shr_round(bits, rm);
    }
    // The short-product attempt. The window is the product of the operands' top `l` limbs: `2l`
    // limbs whose top half approximates the top of the full product from below, with error less
    // than `l + 2` ulps of the bottom valid limb (`l` from Mulders' neglected triangle, 2 from the
    // operands' discarded low limbs).
    let xs = top_limbs(x, l);
    let ys = top_limbs(y, l);
    let two_l = l << 1;
    let mut buf = vec![0; two_l + limbs_mul_high_same_length_scratch_len(l)];
    let (out, scratch) = buf.split_at_mut(two_l);
    limbs_mul_high_same_length(out, xs, ys, scratch);
    let s_top = Natural::from_limbs_asc(&out[l..]);
    // The cut, in the coordinates of the window's top half.
    let c = (u64::exact_from(l) << Limb::LOG_WIDTH) - d;
    // The kept bits and the half-ulp bit are exact iff adding the error bound cannot carry past the
    // half-ulp position.
    let band = (&s_top).mod_power_of_2(c - 1);
    // `v < 2^k` is `v.significant_bits() <= k`, with no power materialized.
    if (band + Natural::from(u64::exact_from(l) + 4)).significant_bits() < c {
        let half = s_top.get_bit(c - 1);
        let k = s_top >> c;
        // Whether anything is set below the half-ulp bit, from the valuation identity.
        let below_half = tzx + tzy < bits - 1;
        match rm {
            Down | Floor => (k, Less),
            Up | Ceiling => (k + Natural::ONE, Greater),
            Nearest => {
                if !half {
                    (k, Less)
                } else if below_half || k.odd() {
                    (k + Natural::ONE, Greater)
                } else {
                    (k, Less)
                }
            }
            Exact => unreachable!(),
        }
    } else {
        // The bits just below the cut are nearly all ones, and the window cannot tell whether they
        // carry. Fall back to the exact product.
        (x * y).shr_round(bits, rm)
    }
}}

impl MulShrRound<Self, u64> for Natural {
    type Output = Self;

    /// Multiplies two [`Natural`]s and right-shifts the product (divides it by a power of 2),
    /// rounding according to a specified rounding mode, taking both [`Natural`]s by value. An
    /// [`Ordering`] is also returned, indicating whether the returned value is less than, equal to,
    /// or greater than the exact value.
    ///
    /// When most of the product is discarded, the product's low portion is never computed: a short
    /// product determines the surviving bits at roughly half the cost of a full multiplication. The
    /// result is always exactly the rounding of $xy/2^k$.
    ///
    /// $f(x, y, k, \mathrm{Down}) = f(x, y, k, \mathrm{Floor}) = \lfloor xy/2^k \rfloor$, and $f(x,
    /// y, k, \mathrm{Up}) = f(x, y, k, \mathrm{Ceiling}) = \lceil xy/2^k \rceil$;
    /// $\mathrm{Nearest}$ rounds to the closer integer, breaking ties toward the even one.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the shift is not exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulShrRound;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(
    ///     Natural::from(100u32).mul_shr_round(Natural::from(200u32), 8, Down),
    ///     (Natural::from(78u32), Less)
    /// );
    /// assert_eq!(
    ///     Natural::from(100u32).mul_shr_round(Natural::from(200u32), 8, Up),
    ///     (Natural::from(79u32), Greater)
    /// );
    /// ```
    #[inline]
    fn mul_shr_round(self, y: Self, bits: u64, rm: RoundingMode) -> (Self, Ordering) {
        mul_shr_round_ref_ref(&self, &y, bits, rm)
    }
}

impl MulShrRound<&Self, u64> for Natural {
    type Output = Self;

    /// Multiplies two [`Natural`]s and right-shifts the product (divides it by a power of 2),
    /// rounding according to a specified rounding mode, taking the first [`Natural`] by value and
    /// the second by reference. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the exact value.
    ///
    /// See the by-value documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the shift is not exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulShrRound;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(
    ///     Natural::from(100u32).mul_shr_round(&Natural::from(200u32), 8, Nearest),
    ///     (Natural::from(78u32), Less)
    /// );
    /// ```
    #[inline]
    fn mul_shr_round(self, y: &Self, bits: u64, rm: RoundingMode) -> (Self, Ordering) {
        mul_shr_round_ref_ref(&self, y, bits, rm)
    }
}

impl MulShrRound<Natural, u64> for &Natural {
    type Output = Natural;

    /// Multiplies two [`Natural`]s and right-shifts the product (divides it by a power of 2),
    /// rounding according to a specified rounding mode, taking the first [`Natural`] by reference
    /// and the second by value. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the exact value.
    ///
    /// See the by-value documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the shift is not exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulShrRound;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(
    ///     (&Natural::from(100u32)).mul_shr_round(Natural::from(200u32), 8, Ceiling),
    ///     (Natural::from(79u32), Greater)
    /// );
    /// ```
    #[inline]
    fn mul_shr_round(self, y: Natural, bits: u64, rm: RoundingMode) -> (Natural, Ordering) {
        mul_shr_round_ref_ref(self, &y, bits, rm)
    }
}

impl MulShrRound<&Natural, u64> for &Natural {
    type Output = Natural;

    /// Multiplies two [`Natural`]s and right-shifts the product (divides it by a power of 2),
    /// rounding according to a specified rounding mode, taking both [`Natural`]s by reference. An
    /// [`Ordering`] is also returned, indicating whether the returned value is less than, equal to,
    /// or greater than the exact value.
    ///
    /// See the by-value documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the shift is not exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{MulShrRound, Pow};
    /// use malachite_base::num::logic::traits::SignificantBits;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// // the high half of a 2000-bit product, without computing the low half
    /// let x = Natural::from(10u32).pow(300);
    /// let y = Natural::from(3u32).pow(600);
    /// let (hi, o) = (&x).mul_shr_round(&y, 1000, Down);
    /// assert_eq!(hi.significant_bits(), 948);
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    fn mul_shr_round(self, y: &Natural, bits: u64, rm: RoundingMode) -> (Natural, Ordering) {
        mul_shr_round_ref_ref(self, y, bits, rm)
    }
}

impl MulShrRoundAssign<Self, u64> for Natural {
    /// Multiplies two [`Natural`]s and right-shifts the product (divides it by a power of 2) in
    /// place, rounding according to a specified rounding mode, taking the [`Natural`] on the
    /// right-hand side by value. An [`Ordering`] is returned, indicating whether the assigned value
    /// is less than, equal to, or greater than the exact value.
    ///
    /// See the [`MulShrRound`] documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the shift is not exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulShrRoundAssign;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Natural::from(100u32);
    /// assert_eq!(x.mul_shr_round_assign(Natural::from(200u32), 8, Down), Less);
    /// assert_eq!(x, 78);
    /// ```
    #[inline]
    fn mul_shr_round_assign(&mut self, y: Self, bits: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = mul_shr_round_ref_ref(self, &y, bits, rm);
        o
    }
}

impl MulShrRoundAssign<&Self, u64> for Natural {
    /// Multiplies two [`Natural`]s and right-shifts the product (divides it by a power of 2) in
    /// place, rounding according to a specified rounding mode, taking the [`Natural`] on the
    /// right-hand side by reference. An [`Ordering`] is returned, indicating whether the assigned
    /// value is less than, equal to, or greater than the exact value.
    ///
    /// See the [`MulShrRound`] documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the shift is not exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulShrRoundAssign;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Natural::from(100u32);
    /// assert_eq!(
    ///     x.mul_shr_round_assign(&Natural::from(200u32), 8, Up),
    ///     Greater
    /// );
    /// assert_eq!(x, 79);
    /// ```
    #[inline]
    fn mul_shr_round_assign(&mut self, y: &Self, bits: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = mul_shr_round_ref_ref(self, y, bits, rm);
        o
    }
}
