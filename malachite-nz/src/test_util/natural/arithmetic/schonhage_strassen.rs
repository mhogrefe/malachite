// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Reference implementations for the Schönhage–Strassen code in
// `natural::arithmetic::mul::schonhage_strassen`, which works with residues modulo the generalized
// Fermat number $p = 2^N + 1$, where $N$ is `limbs * Limb::WIDTH`. A residue is `limbs + 1` limbs,
// the top one a signed overflow. The references compute with `Natural`s reduced modulo $p$, using
// $2^N \equiv -1$ and $2^{2N} \equiv 1$.

use crate::integer::Integer;
use crate::natural::Natural;
use crate::platform::{Limb, SignedLimb};
use malachite_base::num::arithmetic::traits::{
    Mod, ModAddAssign, ModMul, ModNeg, ModPowerOf2, ModSub, PowerOf2,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};

// $N$, the number of bits below the overflow limb.
pub fn fermat_bits(limbs: usize) -> u64 {
    u64::exact_from(limbs) << Limb::LOG_WIDTH
}

// $p = 2^N + 1$.
pub fn fermat_modulus(limbs: usize) -> Natural {
    Natural::power_of_2(fermat_bits(limbs)) + Natural::ONE
}

// The value of the residue `r[..=limbs]` modulo $p$, reading the top limb as signed.
pub fn residue_mod(r: &[Limb], limbs: usize) -> Natural {
    let low = Integer::from(Natural::from_limbs_asc(&r[..limbs]));
    let high = Integer::from(SignedLimb::wrapping_from(r[limbs])) << fermat_bits(limbs);
    Natural::exact_from((low + high).mod_op(Integer::from(fermat_modulus(limbs))))
}

// Whether the residue `r[..=limbs]` is normalized: less than $p$, so either its top limb is zero or
// it is $2^N$.
pub fn residue_is_normalized(r: &[Limb], limbs: usize) -> bool {
    r[limbs] == 0 || (r[limbs] == 1 && r[..limbs].iter().all(|&x| x == 0))
}

// The normalized residue of `limbs + 1` limbs that represents `x`, which must be less than $p$.
pub fn normalized_residue(x: &Natural, limbs: usize) -> Vec<Limb> {
    let mut r = x.to_limbs_asc();
    assert!(r.len() <= limbs + 1);
    r.resize(limbs + 1, 0);
    r
}

// $2^e \bmod p$, for any `e`.
pub fn fermat_power_of_2(e: u64, limbs: usize) -> Natural {
    let n = fermat_bits(limbs);
    let e = e.mod_power_of_2(1) + (((e >> 1) % n) << 1);
    // Now e < 2N.
    if e < n {
        Natural::power_of_2(e)
    } else {
        Natural::power_of_2(e - n).mod_neg(fermat_modulus(limbs))
    }
}

// $x 2^e \bmod p$, for any `e`.
pub fn fermat_mul_power_of_2(x: &Natural, e: u64, limbs: usize) -> Natural {
    let p = fermat_modulus(limbs);
    x.mod_mul(fermat_power_of_2(e, limbs), p)
}

// $x 2^{-e} \bmod p$, for any `e`.
pub fn fermat_div_power_of_2(x: &Natural, e: u64, limbs: usize) -> Natural {
    let two_n = fermat_bits(limbs) << 1;
    fermat_mul_power_of_2(x, two_n - e % two_n, limbs)
}

// A square root of 2 modulo $p$: $2^{3N/4} - 2^{N/4}$. $N$ must be divisible by 4.
pub fn fermat_sqrt_2(limbs: usize) -> Natural {
    let n = fermat_bits(limbs);
    assert_eq!(n & 3, 0);
    let p = fermat_modulus(limbs);
    Natural::power_of_2((3 * n) >> 2).mod_sub(Natural::power_of_2(n >> 2), p)
}

// $x \sqrt{2}^e \bmod p$, for any `e`, with $\sqrt{2}$ as in `fermat_sqrt_2`.
pub fn fermat_mul_sqrt_2_power(x: &Natural, e: u64, limbs: usize) -> Natural {
    let y = fermat_mul_power_of_2(x, e >> 1, limbs);
    if e & 1 == 0 {
        y
    } else {
        y.mod_mul(fermat_sqrt_2(limbs), fermat_modulus(limbs))
    }
}

// The discrete Fourier transform of `xs` modulo $p$ with the root of unity $2^w$ or, if `sqrt_2`,
// $\sqrt{2}^w$: `out[k]` is the sum of `xs[j]` times the root to the power `j * k`.
pub fn fermat_dft(xs: &[Natural], w: u64, sqrt_2: bool, limbs: usize) -> Vec<Natural> {
    let p = fermat_modulus(limbs);
    let len = u64::exact_from(xs.len());
    (0..len)
        .map(|k| {
            let mut sum = Natural::ZERO;
            for (j, x) in (0..len).zip(xs.iter()) {
                let e = j * k * w;
                let term = if sqrt_2 {
                    fermat_mul_sqrt_2_power(x, e, limbs)
                } else {
                    fermat_mul_power_of_2(x, e, limbs)
                };
                sum.mod_add_assign(term, &p);
            }
            sum
        })
        .collect()
}

// The cyclic convolution of `xs` and `ys`, which have the same length, modulo $p$.
pub fn fermat_cyclic_convolution(xs: &[Natural], ys: &[Natural], limbs: usize) -> Vec<Natural> {
    let p = fermat_modulus(limbs);
    let len = xs.len();
    assert_eq!(ys.len(), len);
    let mut out = vec![Natural::ZERO; len];
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            let k = (i + j) % len;
            out[k].mod_add_assign(x.mod_mul(y, &p), &p);
        }
    }
    out
}

// The product of two residues modulo $p$, as a normalized residue.
pub fn fermat_mul_naive(xs: &[Limb], ys: &[Limb], limbs: usize) -> Vec<Limb> {
    let p = fermat_modulus(limbs);
    normalized_residue(
        &residue_mod(xs, limbs).mod_mul(residue_mod(ys, limbs), p),
        limbs,
    )
}

// The negacyclic convolution of `xs` and `ys`, which have the same length, modulo $2^\text{W}$,
// where W is `Limb::WIDTH`.
pub fn limbs_negacyclic_convolution_naive(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let m = xs.len();
    assert_eq!(ys.len(), m);
    let mut out: Vec<Limb> = vec![0; m];
    for (i, &x) in xs.iter().enumerate() {
        for (j, &y) in ys.iter().enumerate() {
            let product = x.wrapping_mul(y);
            if i + j < m {
                out[i + j] = out[i + j].wrapping_add(product);
            } else {
                out[i + j - m] = out[i + j - m].wrapping_sub(product);
            }
        }
    }
    out
}

// The bits of `xs` from bit `start` (inclusive) to bit `end` (exclusive), as a `Natural`; bits past
// the end of `xs` are zero.
pub fn limbs_bit_field(xs: &[Limb], start: u64, end: u64) -> Natural {
    (Natural::from_limbs_asc(xs) >> start).mod_power_of_2(end - start)
}

// The values of the residues `rs` modulo $p$.
pub fn residues_mod(rs: &[Vec<Limb>], limbs: usize) -> Vec<Natural> {
    rs.iter().map(|r| residue_mod(r, limbs)).collect()
}

// The low `bits` bits of `n`, reversed.
pub const fn revbin(n: usize, bits: u64) -> usize {
    if bits == 0 {
        0
    } else {
        n.reverse_bits() >> (usize::WIDTH - bits)
    }
}

// `count` residues of `limbs + 1` limbs with pseudorandom low limbs, determined by `seed`, and top
// limbs in $[-3, 3]$, for tests that need inputs too large to write out.
pub fn pseudorandom_residues(count: usize, limbs: usize, seed: u64) -> Vec<Vec<Limb>> {
    let mut state = seed;
    let mut next = || {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        state
    };
    (0..count)
        .map(|_| {
            let mut r: Vec<Limb> = (0..limbs)
                .map(|_| Limb::wrapping_from(next() >> 7))
                .collect();
            r.push(Limb::wrapping_from(
                SignedLimb::wrapping_from(next() % 7) - 3,
            ));
            r
        })
        .collect()
}
