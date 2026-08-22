// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011, 2021 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    BellNumber, ModAddAssign, ModInverse, ModMulPrecomputed, ModPowPrecomputed, ModSquare,
    ModSubAssign, Parity,
};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::factorization::traits::IsPrime;

// The Bell numbers 0 through 25, all of which fit in a u64. This is bell_number_tab from
// arith/bell_number_nmod.c, FLINT 3.6.0, at its 64-bit size, used on every platform.
const BELL_TAB: [u64; 26] = [
    1,
    1,
    2,
    5,
    15,
    52,
    203,
    877,
    4140,
    21147,
    115975,
    678570,
    4213597,
    27644437,
    190899322,
    1382958545,
    10480142147,
    82864869804,
    682076806159,
    5832742205057,
    51724158235372,
    474869816156751,
    4506715738447323,
    44152005855084346,
    445958869294805289,
    4638590332229999353,
];

// Thresholds up to which the entries of the Bell triangle fit in one, two, and three 64-bit words.
// These are the 64-bit MAX_N_1LIMBS/MAX_N_2LIMBS/MAX_N_3LIMBS from arith/bell_number.c, FLINT
// 3.6.0, and were re-verified against the actual bit growth of the triangle.
const MAX_N_1: usize = 25;
const MAX_N_2: usize = 42;
const MAX_N_3: usize = 58;

// This is arith_bell_number_recursive from arith/bell_number.c, FLINT 3.6.0, with the flat
// multi-word array replaced by phase-typed vectors: the Bell triangle is run in u64s while its
// entries fit, then widened to u128s, then to (u128, u64) pairs handled with explicit carries.
fn bell_number_triangle(n: u64) -> Natural {
    // The phases live in fixed-size stack arrays: their lengths are bounded by the compile-time
    // thresholds, the largest is under two kilobytes, and FLINT itself keeps all three widths in a
    // single stack array. Each phase's index range is known at entry, and a phase limit that was
    // not clamped by its threshold means the run ends there.
    let un = usize::exact_from(n);
    let mut t1 = [0u64; MAX_N_1];
    t1[0] = 1;
    let limit_1 = un.min(MAX_N_1);
    for i in 1..limit_1 {
        t1[i] = t1[0];
        for k in (1..=i).rev() {
            // entries below the phase threshold provably fit, so a plain add cannot overflow
            let t = t1[k];
            t1[k - 1] += t;
        }
    }
    // The dispatch sends n below the table size elsewhere, so this return never fires there; it is
    // kept because it makes the function correct for any n, matching FLINT's recursion, which ends
    // each phase the same way.
    if limit_1 == un {
        return Natural::from(t1[0]);
    }
    let mut t2 = [0u128; MAX_N_2];
    for (x, y) in t2.iter_mut().zip(t1.iter()) {
        *x = u128::from(*y);
    }
    let limit_2 = un.min(MAX_N_2);
    for i in limit_1..limit_2 {
        t2[i] = t2[0];
        for k in (1..=i).rev() {
            let t = t2[k];
            t2[k - 1] += t;
        }
    }
    if limit_2 == un {
        return Natural::from(t2[0]);
    }
    let mut t3 = [(0u128, 0u64); MAX_N_3];
    for (x, y) in t3.iter_mut().zip(t2.iter()) {
        *x = (*y, 0);
    }
    for i in limit_2..un {
        t3[i] = t3[0];
        for k in (1..=i).rev() {
            let (lo, carry) = t3[k - 1].0.overflowing_add(t3[k].0);
            t3[k - 1] = (lo, t3[k - 1].1 + t3[k].1 + u64::from(carry));
        }
    }
    let (lo, hi) = t3[0];
    (Natural::from(hi) << 128u64) | Natural::from(lo)
}

// An upper bound on the number of bits of the nth Bell number, from de Bruijn's asymptotic
// expansion. This is arith_bell_number_size from arith/bell_number_size.c, FLINT 3.6.0; FLINT notes
// it is not proven to be an upper bound for all n but suffices below 2^64, and it was checked
// against the exact values through n = 300.
fn bell_number_size(n: u64) -> u64 {
    // The one caller only passes n past the triangle threshold, so this guard never fires; it is
    // FLINT's, and it keeps the logarithms below well-defined for any input.
    if n <= 1 {
        return 0;
    }
    let l = libm::log(n as f64);
    let ll = libm::log(l);
    let u = 1.0 / l;
    (core::f64::consts::LOG2_E
        * n as f64
        * (l - ll - 1.0 + ll * u + u + 0.5 * (ll * u) * (ll * u) + 0.25 * ll * u * u)
        + 2.0) as u64
}

// For each index, the factor pair the power sieve uses: (1, i) when i must be computed directly,
// and a nontrivial factorization (j, i / j) otherwise. This is divisor_table from
// arith/bell_number_multi_mod.c, FLINT 3.6.0.
fn divisor_table(len: usize) -> Vec<(u32, u32)> {
    let mut tab: Vec<(u32, u32)> = (0..len).map(|i| (1, i as u32)).collect();
    for i in 2..len {
        let mut j = 2;
        while j <= i && i * j < len {
            tab[i * j] = (j as u32, i as u32);
            j += 1;
        }
    }
    tab
}

// The nth Bell number modulo a prime p with p > n, by the Dobinski-style double sum B(n) = sum_{i}
// i^n/i! sum_{j} (-1)^j/j!, computed as n!/i! products so that only (n!)^2 needs inverting at the
// end. This is arith_bell_number_nmod2 from arith/bell_number_multi_mod.c, FLINT 3.6.0, without the
// Montgomery form: plain modular arithmetic replaces the REDC calls, and the three-word accumulator
// becomes a u128 with an explicit carry word.
fn bell_number_mod(n: u64, p: u64, divtab: &[(u32, u32)]) -> u64 {
    let un = usize::exact_from(n);
    // The modulus is fixed for the whole computation, so the precomputed inverses pay for
    // themselves across the ~3n modular multiplications and the n/log n exponentiations below.
    let mul_data = u64::precompute_mod_mul_data(&p);
    let pow_data = u64::precompute_mod_pow_data(&p);
    // facs[i] = n!/i! mod p
    let mut facs = vec![0u64; un + 1];
    facs[un] = 1;
    for i in (0..un).rev() {
        facs[i] = facs[i + 1].mod_mul_precomputed(i as u64 + 1, p, &mul_data);
    }
    // pows[i] = i^n mod p, computed directly only when i is prime. The zeroth entry stays zero,
    // since n is positive here, and the first is one.
    let mut pows = vec![0u64; un + 1];
    pows[1] = 1;
    for i in 2..=un {
        let (a, b) = divtab[i];
        pows[i] = if a == 1 {
            (i as u64).mod_pow_precomputed(n, p, &pow_data)
        } else {
            pows[a as usize].mod_mul_precomputed(pows[b as usize], p, &mul_data)
        };
    }
    // The alternating prefix sum t and the accumulated products, in a 192-bit accumulator: each
    // product is below 2^122, so the u128 low part overflows at most once per addition, and the
    // carries fit comfortably in the high word.
    let mut s_lo = 0u128;
    let mut s_hi = 0u64;
    let mut t = 0u64;
    for i in 0..=un {
        if i.even() {
            t.mod_add_assign(facs[i], p);
        } else {
            t.mod_sub_assign(facs[i], p);
        }
        let u = pows[un - i].mod_mul_precomputed(facs[un - i], p, &mul_data);
        let prod = u128::from(u) * u128::from(t);
        let (lo, carry) = s_lo.overflowing_add(prod);
        s_lo = lo;
        s_hi += u64::from(carry);
    }
    // Reduce hi * 2^128 + lo mod p.
    let r64 = ((1u128 << 64) % u128::from(p)) as u64;
    let r128 = r64.mod_square(p);
    let lo_low = (s_lo % u128::from(p)) as u64;
    let mut s = (s_hi % p).mod_mul_precomputed(r128, p, &mul_data);
    s.mod_add_assign(lo_low, p);
    // Remove (n!)^2. p is prime and exceeds n, so n! is invertible.
    let inv = facs[0].mod_inverse(p).unwrap();
    s.mod_mul_precomputed(inv, p, &mul_data)
        .mod_mul_precomputed(inv, p, &mul_data)
}

// This is arith_bell_number_multi_mod from arith/bell_number_multi_mod.c, FLINT 3.6.0, run
// sequentially rather than in parallel, with the CRT recombination through Natural::multi_crt.
fn bell_number_multi_mod(n: u64) -> Natural {
    let size = bell_number_size(n) + 1;
    let prime_bits = 61;
    let num_primes = usize::exact_from(size.div_ceil(prime_bits));
    let divtab = divisor_table(usize::exact_from(n) + 1);
    let mut primes = Vec::with_capacity(num_primes);
    let mut p = 1u64 << prime_bits;
    for _ in 0..num_primes {
        p += 1;
        while !p.is_prime() {
            p += 1;
        }
        primes.push(p);
    }
    let residues: Vec<Natural> = primes
        .iter()
        .map(|&p| Natural::from(bell_number_mod(n, p, &divtab)))
        .collect();
    let moduli: Vec<Natural> = primes.iter().map(|&p| Natural::from(p)).collect();
    // Distinct primes are coprime, so the CRT always succeeds, and the product of the moduli
    // exceeds 2^size, so the residue is the Bell number itself.
    Natural::multi_crt(&moduli, &residues).unwrap()
}

// An iterator over all Bell numbers, produced by the Bell triangle: the growing row is kept as a
// vector of [`Natural`]s, and each pass over it yields the next number. This is
// arith_bell_number_vec_recursive from arith/bell_number_vec_recursive.c, FLINT 3.6.0, exposed one
// element at a time rather than filling a vector.
#[derive(Clone, Debug)]
pub struct BellNumbers {
    row: Vec<Natural>,
    next_index: u64,
}

impl Iterator for BellNumbers {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        let out = match self.next_index {
            // B(0) and B(1) are both 1, and the triangle proper starts afterwards with a
            // single-entry row.
            0 => Natural::ONE,
            1 => {
                self.row.push(Natural::ONE);
                Natural::ONE
            }
            _ => {
                self.row.push(self.row[0].clone());
                for k in (1..self.row.len()).rev() {
                    let t = self.row[k].clone();
                    self.row[k - 1] += t;
                }
                self.row[0].clone()
            }
        };
        self.next_index += 1;
        Some(out)
    }
}

/// Generates all Bell numbers, in order: 1, 1, 2, 5, 15, 52, 203, and so on.
///
/// The iterator runs the Bell triangle, so producing the first $n$ numbers costs $O(n^2)$ additions
/// on values of $\Theta(n \log n)$ bits, and the retained row uses $O(n^2 \log n)$ bits of memory
/// after $n$ steps.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(i^2 \log i)$
///
/// $M(i) = O(i \log i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_nz::natural::arithmetic::bell_number::exhaustive_bell_numbers;
///
/// assert_eq!(
///     exhaustive_bell_numbers()
///         .take(8)
///         .map(|b| b.to_string())
///         .collect_vec(),
///     ["1", "1", "2", "5", "15", "52", "203", "877"]
/// );
/// ```
///
/// This is equivalent to `arith_bell_number_vec_recursive` from
/// `arith/bell_number_vec_recursive.c`, FLINT 3.6.0, as an iterator rather than a vector-filling
/// function.
pub const fn exhaustive_bell_numbers() -> BellNumbers {
    BellNumbers {
        row: Vec::new(),
        next_index: 0,
    }
}

// The Bell numbers 0 through len - 1 modulo a prime, by the Bell triangle in machine words. This is
// arith_bell_number_nmod_vec_recursive from arith/bell_number_nmod_vec_recursive.c, FLINT 3.6.0,
// without the modulus-one and zero-length guards, which the one caller never produces.
fn bell_numbers_mod(len: usize, p: u64) -> Vec<u64> {
    let mut b = vec![0u64; len];
    b[0] = 1;
    if len >= 2 {
        b[1] = 1;
    }
    if len >= 3 {
        let mut t = vec![0u64; len - 1];
        t[0] = 1;
        for i in 1..len - 1 {
            t[i] = t[0];
            for k in (1..=i).rev() {
                let x = t[k];
                t[k - 1].mod_add_assign(x, p);
            }
            b[i + 1] = t[0];
        }
    }
    b
}

// This is arith_bell_number_vec_multi_mod from arith/bell_number_vec_multi_mod.c, FLINT 3.6.0, with
// each entry recombined through Natural::multi_crt over just the primes its size needs, in place of
// FLINT's graded combs.
crate_test_fn! {bell_numbers_prefix_multi_mod(len: usize) -> Vec<Natural> {
    let size = bell_number_size(u64::exact_from(len)) + 1;
    let prime_bits = 61;
    let num_primes = usize::exact_from(size.div_ceil(prime_bits));
    let mut primes = Vec::with_capacity(num_primes);
    let mut p = 1u64 << prime_bits;
    for _ in 0..num_primes {
        p += 1;
        while !p.is_prime() {
            p += 1;
        }
        primes.push(p);
    }
    let residue_vecs: Vec<Vec<u64>> = primes.iter().map(|&p| bell_numbers_mod(len, p)).collect();
    let moduli: Vec<Natural> = primes.iter().map(|&p| Natural::from(p)).collect();
    (0..len)
        .map(|k| {
            // Each entry needs only as many primes as its own size demands.
            let num_primes_k =
                usize::exact_from((bell_number_size(u64::exact_from(k)) + 1).div_ceil(prime_bits))
                    .max(1);
            let residues: Vec<Natural> = residue_vecs[..num_primes_k]
                .iter()
                .map(|v| Natural::from(v[k]))
                .collect();
            Natural::multi_crt(&moduli[..num_primes_k], &residues).unwrap()
        })
        .collect()
}}

// Below this many entries, the prefix is produced by the bignum triangle; at or above it, by the
// multimodular batch. This is the threshold in arith/bell_number_vec.c, FLINT 3.6.0.
const PREFIX_MULTI_MOD_THRESHOLD: u64 = 5000;

/// Computes the first `len` Bell numbers: $B_0$ through $B_{\mathrm{len} - 1}$.
///
/// Short prefixes come from the Bell triangle; long ones are computed modulo enough word-sized
/// primes at once and recombined entry by entry, each entry using only as many primes as its size
/// requires.
///
/// # Worst-case complexity
/// $T(n) = O(n^2 \log n)$
///
/// $M(n) = O(n^2 \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::bell_number::bell_numbers_prefix;
///
/// let prefix = bell_numbers_prefix(6);
/// assert_eq!(
///     prefix.iter().map(|b| b.to_string()).collect::<Vec<_>>(),
///     ["1", "1", "2", "5", "15", "52"]
/// );
/// assert!(bell_numbers_prefix(0).is_empty());
/// ```
///
/// This is equivalent to `arith_bell_number_vec` from `arith/bell_number_vec.c`, FLINT 3.6.0.
pub fn bell_numbers_prefix(len: u64) -> Vec<Natural> {
    if len < PREFIX_MULTI_MOD_THRESHOLD {
        exhaustive_bell_numbers()
            .take(usize::exact_from(len))
            .collect()
    } else {
        bell_numbers_prefix_multi_mod(usize::exact_from(len))
    }
}

impl BellNumber for Natural {
    /// Computes the $n$th Bell number: the number of ways to partition a set of $n$ elements.
    ///
    /// The first few Bell numbers are 1, 1, 2, 5, 15, 52, 203, taken from a table; somewhat larger
    /// arguments use the Bell triangle in fixed-width accumulators, and larger ones still are
    /// computed modulo enough word-sized primes and recombined by the Chinese remainder theorem.
    /// The result has $\Theta(n \log n)$ bits, so even moderate $n$ produce large outputs.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BellNumber;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::bell_number(0), 1);
    /// assert_eq!(Natural::bell_number(4), 15);
    /// assert_eq!(Natural::bell_number(10), 115975);
    /// assert_eq!(
    ///     Natural::bell_number(40).to_string(),
    ///     "157450588391204931289324344702531067"
    /// );
    /// ```
    ///
    /// This is equivalent to `arith_bell_number` from `arith/bell_number.c`, FLINT 3.6.0.
    fn bell_number(n: u64) -> Self {
        if n < 26 {
            // n < 26, so it fits in a usize
            Self::from(BELL_TAB[usize::wrapping_from(n)])
        } else if n <= const { MAX_N_3 as u64 } {
            bell_number_triangle(n)
        } else {
            bell_number_multi_mod(n)
        }
    }
}
