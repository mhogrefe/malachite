// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    AddMul, DivRound, LcmAssign, ModInverse, ModMul, Square,
};
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;

/// The residues of the elements of `a` modulo `m`, which must be coprime to every denominator: each
/// $n/d$ becomes $n d^{-1} \bmod m$.
pub fn rational_matrix_residues(a: &RationalMatrix, m: &Natural) -> NaturalMatrix {
    let elements: Vec<Natural> = a
        .elements
        .iter()
        .map(|x| {
            let n = x.numerator_ref() % m;
            let n = if *x < 0u32 && n != 0u32 { m - n } else { n };
            n.mod_mul(x.denominator_ref().mod_inverse(m).unwrap(), m)
        })
        .collect();
    NaturalMatrix::from_owned_elements(a.height(), a.width(), elements)
}

/// A modulus from which `RationalMatrix::reconstruct` recovers `a` from its residues: one more than
/// a multiple of the least common multiple of the denominators, so that it is coprime to all of
/// them, and greater than $2(MD)^2$, where $M$ is the largest absolute value of a numerator and $D$
/// the product of the denominators, so that every step of the reconstruction finds a fraction
/// within its bounds.
pub fn rational_matrix_reconstruction_modulus(a: &RationalMatrix) -> Natural {
    let mut lcm = Natural::ONE;
    let mut product = Natural::ONE;
    let mut max = Natural::ONE;
    for x in &a.elements {
        lcm.lcm_assign(x.denominator_ref());
        product *= x.denominator_ref();
        if *x.numerator_ref() > max {
            max = x.numerator_ref().clone();
        }
    }
    let bound = Natural::ONE.add_mul((max * product).square(), Natural::TWO);
    Natural::ONE.add_mul(bound.div_round(&lcm, Ceiling).0, lcm)
}
