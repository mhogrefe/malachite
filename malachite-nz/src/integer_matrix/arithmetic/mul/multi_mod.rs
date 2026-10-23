// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011, 2018 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::arithmetic::mul::ProductShape;
use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::integer_vector::arithmetic::multi_crt::multi_balanced_crt_elements;
use crate::natural::{LIMB_HIGH_BIT, WIDTH_MINUS_1};
use crate::natural_vector::arithmetic::multi_crt::multi_crt_elements;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModMul, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::IsPrime;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::unsigned_matrix::UnsignedMatrix;

// The number of bits of the primes, other than the first modulus, that the multimodular algorithm
// works modulo.
//
// This is `NMOD_MAT_OPTIMAL_MODULUS_BITS` from `nmod_mat.h`, FLINT 3.6.0.
const NMOD_MAT_OPTIMAL_MODULUS_BITS: u64 = Limb::WIDTH - 5;

// The primes other than the first modulus are the primes just above this.
pub(crate) const OPTIMAL_MODULUS_MIN: Limb = 1 << NMOD_MAT_OPTIMAL_MODULUS_BITS;

// The smallest prime greater than `n`.
//
// This is equivalent to `n_nextprime` from `ulong_extras/nextprime.c`, FLINT 3.6.0, with `proved =
// 0`.
pub(crate) fn next_prime<T: PrimitiveUnsigned + IsPrime>(n: T) -> T {
    let mut p = n + T::ONE;
    while !p.is_prime() {
        p += T::ONE;
    }
    p
}

// The moduli that the multimodular algorithm works modulo, for products whose elements have at most
// `bits` significant bits, counting a sign bit if they may be negative: a single power of 2 when
// that fits in a word, and otherwise $2^{W-1}$, where $W$ is the limb width, followed by primes
// just above $2^{W-5}$, enough for their product to exceed $2^\text{bits}$.
fn moduli(bits: u64) -> Vec<Limb> {
    if bits < NMOD_MAT_OPTIMAL_MODULUS_BITS || bits <= WIDTH_MINUS_1 {
        return vec![Limb::power_of_2(bits)];
    }
    let count =
        usize::exact_from(1 + (bits - WIDTH_MINUS_1).div_ceil(NMOD_MAT_OPTIMAL_MODULUS_BITS));
    let mut primes = Vec::with_capacity(count);
    primes.push(LIMB_HIGH_BIT);
    let mut p = OPTIMAL_MODULUS_MIN;
    for _ in 1..count {
        p = next_prime(p);
        primes.push(p);
    }
    primes
}

// Returns the elements of the product of the matrices with elements `xs` and `ys`, every element of
// which has at most `bits` significant bits, with the multimodular algorithm: both matrices are
// reduced modulo several word-sized moduli whose product exceeds the range of the product's
// elements, multiplied modulo each modulus, and the product is reconstructed by the Chinese
// remainder theorem. If `sign`, some element may be negative, and the balanced representatives are
// reconstructed.
//
// This is equivalent to `_fmpz_mat_mul_multi_mod` from `fmpz_mat/mul_multi_mod.c`, FLINT 3.6.0,
// with one thread, except that the products modulo each modulus use the public `ModMul` of
// `UnsignedMatrix`, and that a matrix is not recognized as being multiplied by itself.
crate_test_fn! {matrix_mul_multi_mod<C: PolynomialCoefficient>(
    xs: &[C],
    ys: &[C],
    shape: ProductShape,
    sign: bool,
    bits: u64,
) -> Vec<C> {
    let ProductShape { ar: m, br: k, bc: n } = shape;
    if m == 0 || n == 0 || k == 0 {
        return vec![C::ZERO; m * n];
    }
    let primes = moduli(bits + u64::from(sign));
    let a_residues = C::vec_multi_mod(xs, &primes);
    // When a matrix is multiplied by itself, it is reduced only once, as in FLINT.
    let b_residues = if core::ptr::eq(xs, ys) {
        a_residues.clone()
    } else {
        C::vec_multi_mod(ys, &primes)
    };
    let (m_64, k_64, n_64) = (u64::exact_from(m), u64::exact_from(k), u64::exact_from(n));
    let c_residues: Vec<Vec<Limb>> = a_residues
        .into_iter()
        .zip(b_residues)
        .zip(&primes)
        .map(|((a, b), &p)| {
            UnsignedMatrix::from_owned_elements(m_64, k_64, a)
                .mod_mul(UnsignedMatrix::from_owned_elements(k_64, n_64, b), p)
                .into_elements()
        })
        .collect();
    let slices: Vec<&[Limb]> = c_residues.iter().map(Vec::as_slice).collect();
    if sign {
        C::from_integers(multi_balanced_crt_elements(&primes, &slices).unwrap())
    } else {
        C::from_naturals(multi_crt_elements(&primes, &slices).unwrap())
    }
}}

// The product of the matrices with elements `xs` and `ys` with the multimodular algorithm, finding
// the number of bits that the product's elements need.
//
// This is equivalent to `fmpz_mat_mul_multi_mod` from `fmpz_mat/mul_multi_mod.c`, FLINT 3.6.0.
crate_test_fn! {matrix_mul_multi_mod_any<C: PolynomialCoefficient>(
    xs: &[C],
    ys: &[C],
    shape: ProductShape,
) -> Vec<C> {
    let (a_bits, a_negative) = vec_max_bits(xs);
    let (b_bits, b_negative) = vec_max_bits(ys);
    let bits = a_bits + b_bits + u64::exact_from(shape.br).significant_bits();
    matrix_mul_multi_mod(xs, ys, shape, a_negative || b_negative, bits)
}}
