// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011, 2014 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{AddMulAssign, Parity, PowerOf2};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{SplitInHalf, WrappingFrom};
use malachite_base::num::logic::traits::LowMask;
use malachite_nz::natural::Natural;

// The harmonic numbers 0 through 46, whose numerators and denominators fit in a u64. This is
// fmpq_harmonic_ui_tab_num/fmpq_harmonic_ui_tab_den from fmpq/harmonic_ui.c, FLINT 3.6.0, at its
// 64-bit size; the u64 entries are used on every platform, since the computation below uses u64
// accumulators everywhere rather than word-sized ones.
const HARMONIC_TAB_SIZE: u64 = 47;
#[rustfmt::skip]
const HARMONIC_TAB_NUM: [u64; 47] = [
    0, 1, 3, 11, 25, 137, 49, 363, 761, 7129, 7381, 83711, 86021, 1145993, 1171733, 1195757,
    2436559, 42142223, 14274301, 275295799, 55835135, 18858053, 19093197, 444316699, 1347822955,
    34052522467, 34395742267, 312536252003, 315404588903, 9227046511387, 9304682830147,
    290774257297357, 586061125622639, 53676090078349, 54062195834749, 54437269998109,
    54801925434709, 2040798836801833, 2053580969474233, 2066035355155033, 2078178381193813,
    85691034670497533, 12309312989335019, 532145396070491417, 5884182435213075787,
    5914085889685464427, 5943339269060627227,
];
#[rustfmt::skip]
const HARMONIC_TAB_DEN: [u64; 47] = [
    1, 1, 2, 6, 12, 60, 20, 140, 280, 2520, 2520, 27720, 27720, 360360, 360360, 360360, 720720,
    12252240, 4084080, 77597520, 15519504, 5173168, 5173168, 118982864, 356948592, 8923714800,
    8923714800, 80313433200, 80313433200, 2329089562800, 2329089562800, 72201776446800,
    144403552893600, 13127595717600, 13127595717600, 13127595717600, 13127595717600,
    485721041551200, 485721041551200, 485721041551200, 485721041551200, 19914562703599200,
    2844937529085600, 122332313750680800, 1345655451257488800, 1345655451257488800,
    1345655451257488800,
];

// Below this many terms, a subinterval is summed directly rather than split further.
const HARMONIC_DIRECT_THRESHOLD: u64 = 50;

// The sum p/q + t_n/t_d as a pair of u64s, or None if any part of it overflows. The checks are
// staged so that the intermediate u128 sum cannot overflow either: the products are only added once
// both are known to fit in a u64.
fn word_sum(p: u64, q: u64, t_n: u64, t_d: u64) -> Option<(u64, u64)> {
    let (hi, p_td) = (u128::from(p) * u128::from(t_d)).split_in_half();
    if hi != 0 {
        return None;
    }
    let (hi, q_tn) = (u128::from(q) * u128::from(t_n)).split_in_half();
    if hi != 0 {
        return None;
    }
    let (hi, num) = (u128::from(p_td) + u128::from(q_tn)).split_in_half();
    if hi != 0 {
        return None;
    }
    let (hi, den) = (u128::from(q) * u128::from(t_d)).split_in_half();
    if hi != 0 {
        return None;
    }
    Some((num, den))
}

// big_p/big_q += p/q, where the right side need not be reduced.
fn flush(big_p: &mut Natural, big_q: &mut Natural, p: u64, q: u64) {
    *big_p *= Natural::from(q);
    big_p.add_mul_assign(&*big_q, Natural::from(p));
    *big_q *= Natural::from(q);
}

// The odd-term partial sum of the harmonic series over [a, b) as an unreduced fraction, each odd
// term 1/k weighted by (2^d - 1)/2^(d - 1) for the d with n/2^d < k <= n/2^(d - 1): summing the odd
// terms of H(n) with these weights gives all of H(n), by recursive application of H(n) =
// H(floor(n/2))/2 + H_odd(n). Short ranges are summed directly, with partial sums packed into
// single u64s until they no longer fit; longer ones are split in half and merged, which keeps the
// operands of every multiplication balanced.
fn harmonic_odd(a: u64, b: u64, n: u64, mut d: u32) -> (Natural, Natural) {
    if b - a >= HARMONIC_DIRECT_THRESHOLD {
        let m = a + ((b - a) >> 1);
        let (mut p, mut q) = harmonic_odd(a, m, n, d + u32::from(a == 1));
        let (r, s) = harmonic_odd(m, b, n, d);
        p *= &s;
        p.add_mul_assign(&q, r);
        q *= s;
        return (p, q);
    }
    let mut big_p = Natural::ZERO;
    let mut big_q = Natural::ONE;
    let mut p = 0u64;
    let mut q = 1u64;
    if a == 1 {
        // The leftmost range owns the weight bookkeeping: k descends so that d, which depends only
        // on k, can be advanced incrementally. k must go negative to end the loop; a u64 k would
        // wrap at 1 - 2 and spin nearly forever.
        let mut k = i64::wrapping_from(b - 1 - (b & 1));
        while k > 0 {
            let ku = u64::wrapping_from(k);
            while ku <= (n >> d) {
                d += 1;
            }
            let weight_n = u64::low_mask(u64::from(d));
            let weight_d = ku << (d - 1);
            if let Some(sum) = word_sum(p, q, weight_n, weight_d) {
                (p, q) = sum;
            } else {
                flush(&mut big_p, &mut big_q, p, q);
                p = weight_n;
                q = weight_d;
            }
            k -= 2;
        }
    } else {
        // Every other range has a constant d, so the weight is applied once at the end and the
        // terms are the plain 1/k.
        let mut k = a + u64::from(a.even());
        while k < b {
            if let Some(sum) = word_sum(p, q, 1, k) {
                (p, q) = sum;
            } else {
                flush(&mut big_p, &mut big_q, p, q);
                p = 1;
                q = k;
            }
            k += 2;
        }
    }
    // p is zero only for an empty range, which the callers' split segments never produce; the check
    // keeps the function total.
    if p != 0 {
        flush(&mut big_p, &mut big_q, p, q);
    }
    if a != 1 {
        big_p *= Natural::from(u64::low_mask(u64::from(d)));
        big_q *= Natural::from(u64::power_of_2(u64::from(d - 1)));
    }
    (big_p, big_q)
}

impl Rational {
    /// Computes the $n$th harmonic number:
    ///
    /// $$
    /// H_n = \sum_{k=1}^n \frac{1}{k}.
    /// $$
    ///
    /// $H_0$ is the empty sum, 0.
    ///
    /// The result's numerator and denominator have $\Theta(n \log n)$ bits between them, so even
    /// moderate $n$ produce large outputs: $H_{1000}$ has over 850 digits.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
    ///
    /// # Panics
    /// Panics if $n \geq 2^{63}$.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::harmonic_number(0), 0);
    /// assert_eq!(Rational::harmonic_number(1), 1);
    /// assert_eq!(Rational::harmonic_number(4).to_string(), "25/12");
    /// assert_eq!(
    ///     Rational::harmonic_number(100).to_string(),
    ///     "14466636279520351160221518043104131447711/2788815009188499086581352357412492142272"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_harmonic_ui` from `fmpq/harmonic_ui.c`, FLINT 3.6.0.
    pub fn harmonic_number(n: u64) -> Self {
        if n < HARMONIC_TAB_SIZE {
            // n < 47, so it fits in a usize
            let n = usize::wrapping_from(n);
            Self::from_unsigneds(HARMONIC_TAB_NUM[n], HARMONIC_TAB_DEN[n])
        } else {
            // The weight shifts below need the band index to stay under 64, which fails for n at or
            // above 2^63; FLINT rejects the same range with its signedness check. Such an n would
            // name a sum with more terms than could ever be added anyway.
            assert!(n >> 63 == 0, "harmonic_number: n must be less than 2^63");
            let (p, q) = harmonic_odd(1, n + 1, n, 1);
            Self::from_naturals(p, q)
        }
    }
}
