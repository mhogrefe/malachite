// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2021 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::arithmetic::mul::multi_mod::primes_above;
use crate::natural::number_theory::bell_number::divisor_table;
use crate::natural::number_theory::unsigned_stirling_number_of_the_first_kind::{
    second_order_eulerian_numbers, weighted_binomial_sum,
};
use crate::natural::{Natural, WIDTH_MINUS_1};
use crate::natural_polynomial::special::touchard_polynomial::{
    MAX_N_1LIMB, MAX_N_2LIMB, PRIME_BITS, triangular_1, triangular_2,
};
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{DivExact, Parity, Pow, PowerOf2};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::LowMask;
use malachite_base::num::modular::traits::{
    ModAdd, ModAddAssign, ModInverse, ModMulPrecomputed, ModMulPrecomputedAssign, ModNegAssign,
    ModPowPrecomputed, ModSquarePrecomputed, ModSubAssign,
};
use malachite_base::num::number_theory::traits::{Factorial, StirlingNumberOfTheSecondKind};

// An upper bound for the binary logarithm of S(n, k), which is bounded by (1/2) binomial(n, k) k^(n
// - k).
//
// This is `stirling_2_bound_2exp` from `arith/stirling2.c`, FLINT 3.6.0.
fn stirling_2_bound_2exp(n: u64, k: u64) -> i64 {
    // binomial coefficients
    let mut bnk = 1.0;
    let mut bnk_exp = 0i64;
    for j in 1..=k {
        bnk = (bnk * (n + 1 - j) as f64) / j as f64;
        let (mantissa, exp) = libm::frexp(bnk);
        bnk = mantissa;
        bnk_exp += i64::from(exp);
    }
    // (1/2) * binomial(n,k) * k^(n-k); add instead of subtract 1 to ensure an upper bound in
    // floating point
    bnk_exp + ((n - k) as f64 * libm::log(k as f64) * core::f64::consts::LOG2_E) as i64 + 1
}

// S(n, k) = (1/k!) sum_{j odd} sum_{m = 2^p j <= k} (-1)^(k + m) C(k, m) j^n 2^(pn), grouping the
// powers m^n by their odd parts so that each odd j is raised to the nth power once.
//
// This is `stirling_2_powsum` from `arith/stirling2.c`, FLINT 3.6.0.
fn stirling_2_powsum(n: u64, k: u64) -> Natural {
    let max_b = (k + 1) >> 1;
    let umax_b = usize::exact_from(max_b);
    let mut b = vec![Natural::ZERO; umax_b + 1];
    b[0] = Natural::ONE;
    for j in 1..=umax_b {
        b[j] = (&b[j - 1] * Natural::from(k + 1 - j as u64)).div_exact(Natural::from(j));
    }
    let mut s = Integer::ZERO;
    let mut j = 1;
    while j <= k {
        let mut u = Natural::from(j).pow(n);
        let mut m = j;
        loop {
            // Process each m = 2^p * j
            let i = usize::exact_from(if m <= max_b { m } else { k - m });
            if (k + m).odd() {
                s -= Integer::from(&b[i] * &u);
            } else {
                s += Integer::from(&b[i] * &u);
            }
            m <<= 1;
            if m > k {
                break;
            }
            u <<= n;
        }
        j += 2;
    }
    Natural::exact_from(s).div_exact(Natural::factorial(k))
}

// S(n, k) modulo the prime p, which must exceed k, as (1/k!) sum_i (-1)^(k - i) C(k, i) i^n, with
// the binomial coefficients stored only up to the middle and scaled by a common factor. k must be
// positive.
//
// This is `stirling_2_nmod` from `arith/stirling2.c`, FLINT 3.6.0.
fn stirling_2_mod(divtab: &[(u32, u32)], n: u64, k: u64, p: Limb) -> Limb {
    // The modulus is fixed for the whole computation, so the data for multiplying and
    // exponentiating modulo it is computed once.
    let mul_data = Limb::precompute_mod_mul_data(&p);
    let pow_data = Limb::precompute_mod_pow_data(&p);
    let pow_len = usize::exact_from(k + 1);
    // Since k is positive, this is less than `pow_len`.
    let bin_len = usize::exact_from(k >> 1) + 1;
    // compute binomial coefficients + denominator
    let mut t: Vec<Limb> = vec![0; bin_len];
    t[0] = 1;
    for i in 1..bin_len {
        t[i] = t[i - 1].mod_mul_precomputed(Limb::exact_from(k + 1 - i as u64), p, &mul_data);
    }
    let bd = t[bin_len - 1 - usize::exact_from((k + 1) & 1)];
    let mut bden: Limb = 1;
    for i in (0..bin_len).rev() {
        bden.mod_mul_precomputed_assign(Limb::exact_from(i + 1), p, &mul_data);
        t[i].mod_mul_precomputed_assign(bden, p, &mul_data);
    }
    // compute powers
    let mut u: Vec<Limb> = vec![0; pow_len];
    u[0] = Limb::ZERO.mod_pow_precomputed(n, p, &pow_data);
    u[1] = 1;
    for i in 2..pow_len {
        let (a, b) = divtab[i];
        u[i] = if a == 1 {
            Limb::exact_from(i).mod_pow_precomputed(n, p, &pow_data)
        } else {
            u[a as usize].mod_mul_precomputed(u[b as usize], p, &mul_data)
        };
    }
    for i in (1..bin_len).step_by(2) {
        t[i].mod_neg_assign(p);
    }
    let mut s1 = t.iter().zip(&u).fold(0, |s: Limb, (&x, &y)| {
        s.mod_add(x.mod_mul_precomputed(y, p, &mul_data), p)
    });
    let l = pow_len - bin_len;
    let s2 = (0..l).fold(0, |s: Limb, i| {
        s.mod_add(
            u[bin_len + i].mod_mul_precomputed(t[l - 1 - i], p, &mul_data),
            p,
        )
    });
    if k.odd() {
        s1.mod_sub_assign(s2, p);
        s1.mod_neg_assign(p);
    } else {
        s1.mod_add_assign(s2, p);
    }
    let bden = bden
        .mod_square_precomputed(p, &pow_data)
        .mod_mul_precomputed(bd, p, &mul_data)
        .mod_inverse(p)
        .unwrap();
    s1.mod_mul_precomputed(bden, p, &mul_data)
}

// S(n, k) from its residues modulo enough word-sized primes.
//
// This is `stirling_2_multi_mod` from `arith/stirling2.c`, FLINT 3.6.0, with `Natural::multi_crt`
// in place of `tree_crt`.
fn stirling_2_multi_mod(n: u64, k: u64) -> Natural {
    let size = stirling_2_bound_2exp(n, k);
    let num_primes = usize::exact_from((size + PRIME_BITS - 1) / PRIME_BITS);
    // `stirling_2_mod` reads the table only up to index k
    let divtab = divisor_table(usize::exact_from(k) + 1);
    let primes = primes_above(Limb::power_of_2(WIDTH_MINUS_1), num_primes);
    let residues: Vec<Natural> = primes
        .iter()
        .map(|&p| Natural::from(stirling_2_mod(&divtab, n, k, p)))
        .collect();
    let moduli: Vec<Natural> = primes.into_iter().map(Natural::from).collect();
    Natural::multi_crt(&moduli, &residues).unwrap()
}

// S(n, n - d) = sum_{j < d} <<d, j>> C(n + d - 1 - j, 2d), for 0 < d < n.
fn stirling_2_small_difference(n: u64, d: u64) -> Natural {
    let mut weights = second_order_eulerian_numbers(d);
    weights.reverse();
    weighted_binomial_sum(n, &weights)
}

// This is `arith_stirling_number_2` from `arith/stirling2.c`, FLINT 3.6.0, except that the
// exponential generating function is replaced by the second-order Eulerian numbers. Their cost,
// about (n - k)^3 log(n - k), does not grow with n otherwise, and, measured in Malachite, they beat
// the power sum and the multimodular computation when n - k <= 1.4n^(2/3), which keeps their cost
// within O(n^2 log n). The exponential generating function was slower than the multimodular
// computation everywhere it was measured, so the multimodular computation takes its place.
crate_test_fn! {stirling_number_of_the_second_kind(n: u64, k: u64) -> Natural {
    if k >= n {
        Natural::from(u32::from(n == k))
    } else if k <= 1 {
        Natural::from(k)
    } else if k == n - 1 {
        // S(n, n-1) = binomial(n, 2)
        let (a, b) = if n.even() {
            (n >> 1, n - 1)
        } else {
            (n, (n - 1) >> 1)
        };
        Natural::from(a) * Natural::from(b)
    } else if k == 2 {
        // S(n,2) = 2^(n-1)-1
        Natural::low_mask(n - 1)
    } else if n <= const { MAX_N_1LIMB as u64 } {
        let mut c = [0u64; MAX_N_2LIMB + 1];
        triangular_1(&mut c, usize::exact_from(n), usize::exact_from(k) + 1);
        Natural::from(c[usize::exact_from(k)])
    } else if n <= const { MAX_N_2LIMB as u64 } {
        let mut c = [0u128; MAX_N_2LIMB + 1];
        triangular_2(&mut c, usize::exact_from(n), usize::exact_from(k) + 1);
        Natural::from(c[usize::exact_from(k)])
    } else if (n - k) as f64 <= 1.4 * libm::cbrt(n as f64 * n as f64) {
        stirling_2_small_difference(n, n - k)
    } else {
        let low_cutoff = if n < 200 {
            0.9
        } else if n < 3000 {
            (0.95 * libm::exp(-0.00022 * n as f64)).clamp(0.0002, 0.8)
        } else {
            (1500.0 / n as f64).clamp(0.0002, 0.8)
        };
        if k as f64 <= low_cutoff * n as f64 {
            stirling_2_powsum(n, k)
        } else {
            stirling_2_multi_mod(n, k)
        }
    }
}}

crate_test_fn! {stirling_2_by_small_difference(n: u64, k: u64) -> Natural {
    stirling_2_small_difference(n, n - k)
}}

crate_test_fn! {stirling_2_by_powsum(n: u64, k: u64) -> Natural {
    stirling_2_powsum(n, k)
}}

crate_test_fn! {stirling_2_by_multi_mod(n: u64, k: u64) -> Natural {
    stirling_2_multi_mod(n, k)
}}

impl StirlingNumberOfTheSecondKind for Natural {
    /// Computes the Stirling number of the second kind $S(n, k)$: the number of ways to partition a
    /// set of $n$ elements into $k$ nonempty subsets.
    ///
    /// Small cases have closed forms or come from the recurrence $S(n, k) = kS(n - 1, k) + S(n - 1,
    /// k - 1)$ in fixed-width accumulators. When $n - k$ is small, the result is found from the
    /// second-order Eulerian numbers, as $\sum_j \left\langle\!\left\langle {n - k \atop j}
    /// \right\rangle\!\right\rangle \binom{n + (n - k) - 1 - j}{2(n - k)}$, in time that hardly
    /// depends on $n$; otherwise it comes from an alternating sum of powers $i^n$ or from its
    /// residues modulo enough word-sized primes, recombined by the Chinese remainder theorem.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::number_theory::traits::StirlingNumberOfTheSecondKind;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::stirling_number_of_the_second_kind(0, 0), 1);
    /// assert_eq!(Natural::stirling_number_of_the_second_kind(5, 0), 0);
    /// assert_eq!(Natural::stirling_number_of_the_second_kind(3, 5), 0);
    /// assert_eq!(Natural::stirling_number_of_the_second_kind(5, 2), 15);
    /// assert_eq!(Natural::stirling_number_of_the_second_kind(10, 5), 42525);
    /// assert_eq!(
    ///     Natural::stirling_number_of_the_second_kind(42, 8).to_string(),
    ///     "2048320078742103108851269258081470"
    /// );
    /// assert_eq!(
    ///     Natural::stirling_number_of_the_second_kind(u64::MAX, u64::MAX - 2).to_string(),
    ///     "14474011154664524421146179579417084321985095863088265868262062449909771010050"
    /// );
    /// ```
    ///
    /// This is equivalent to `arith_stirling_number_2` from `arith/stirling2.c`, FLINT 3.6.0,
    /// except that the second-order Eulerian numbers replace its exponential generating function.
    #[inline]
    fn stirling_number_of_the_second_kind(n: u64, k: u64) -> Self {
        stirling_number_of_the_second_kind(n, k)
    }
}
