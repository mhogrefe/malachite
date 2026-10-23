// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011, 2019 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

//! Matrices of fractions, the form in which the integer matrix solvers carry rational solutions.
//! malachite-q's `RationalMatrix` holds the same values, but lives in a crate that depends on this
//! one, so these functions work on [`Fraction`]s and malachite-q converts.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::mod_op::integers_mod_word;
use crate::integer_vector::IntegerVector;
use crate::natural::Natural;
use crate::natural::arithmetic::reconstruct::{
    rational_reconstruct_with_bounds, rational_reconstruction_balanced_bound,
};
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::ops::Mul;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{DivExact, DivModPrecomputed, Gcd, LcmAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::comparison::traits::OrdProduct;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;

pub use crate::integer_matrix::arithmetic::can_solve_dixon::can_solve_dixon;
pub use crate::integer_matrix::arithmetic::can_solve_multi_mod::can_solve_multi_mod;
pub use crate::integer_matrix::arithmetic::solve::{can_solve_fflu, solve_nonsingular};
pub use crate::integer_matrix::arithmetic::solve_dixon::solve_dixon;
pub use crate::integer_matrix::arithmetic::solve_multi_mod::solve_multi_mod;

/// The right-hand side $B$ of a linear system $AX = B$, borrowed: the elements of a matrix in
/// row-major order, with its height and width. A vector is borrowed as a matrix with one column, so
/// that systems with either kind of right-hand side are solved without copying it.
#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub struct RightHandSide<'a> {
    pub elements: &'a [Integer],
    pub height: usize,
    pub width: usize,
}

impl<'a> From<&'a IntegerMatrix> for RightHandSide<'a> {
    #[inline]
    fn from(b: &'a IntegerMatrix) -> Self {
        RightHandSide {
            elements: &b.elements,
            height: b.height,
            width: b.width,
        }
    }
}

impl<'a> From<&'a IntegerVector> for RightHandSide<'a> {
    #[inline]
    fn from(b: &'a IntegerVector) -> Self {
        RightHandSide {
            elements: &b.elements,
            height: b.elements.len(),
            width: 1,
        }
    }
}

impl RightHandSide<'_> {
    // A copy of the right-hand side, as a matrix.
    pub(crate) fn to_matrix(self) -> IntegerMatrix {
        IntegerMatrix {
            elements: self.elements.to_vec(),
            height: self.height,
            width: self.width,
        }
    }

    // The right-hand side reduced modulo the word `p`.
    pub(crate) fn mod_word(self, p: Limb) -> UnsignedMatrix<Limb> {
        UnsignedMatrix::from_owned_elements(
            u64::exact_from(self.height),
            u64::exact_from(self.width),
            integers_mod_word(self.elements, p),
        )
    }

    // Whether every element is zero.
    pub(crate) fn is_zero(self) -> bool {
        self.elements.iter().all(|x| *x == 0u32)
    }
}

/// A fraction $(-1)^{s} n/d$ in lowest terms, with $s = 0$ when `sign` is `true`, and $d > 0$; zero
/// has `sign` `true` and denominator 1. This is the representation of a `Rational` in malachite-q.
#[doc(hidden)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fraction {
    pub sign: bool,
    pub numerator: Natural,
    pub denominator: Natural,
}

impl Fraction {
    pub(crate) const ZERO: Self = Self {
        sign: true,
        numerator: Natural::ZERO,
        denominator: Natural::ONE,
    };

    // The fraction n/d with the given sign, reduced to lowest terms. `d` must be positive.
    fn from_sign_and_naturals(sign: bool, n: Natural, d: Natural) -> Self {
        if n == 0u32 {
            return Self::ZERO;
        }
        let g = (&n).gcd(&d);
        if g == 1u32 {
            Self {
                sign,
                numerator: n,
                denominator: d,
            }
        } else {
            Self {
                sign,
                numerator: n.div_exact(&g),
                denominator: d.div_exact(&g),
            }
        }
    }

    // The fraction's numerator, with its sign.
    fn signed_numerator(&self) -> Integer {
        Integer::from_sign_and_abs_ref(self.sign, &self.numerator)
    }
}

/// Reconstructs the fractions with residues `residues` modulo `m`, in order, by rational
/// reconstruction with a running denominator: each residue is first multiplied by the product $D$
/// of the denominators found so far, the fraction $p/q$ recovered from $rD \bmod m$ with the
/// balanced bounds gives the fraction $p/(qD)$, and $D$ becomes $qD$. Returns `None` if some step
/// finds no fraction. The residues must be less than `m`, which must be greater than 2; the
/// residues may be `Natural`s or references to them.
///
/// This is `fmpq_mat_set_fmpz_mat_mod_fmpz` from `fmpq_mat/set_fmpz_mat_mod_fmpz.c`, FLINT 3.6.0.
#[doc(hidden)]
pub fn reconstruct_fractions<T: for<'a> Mul<&'a Natural, Output = Natural>>(
    residues: impl Iterator<Item = T>,
    m: &Natural,
) -> Option<Vec<Fraction>> {
    let data = Natural::precompute_div_mod_data(m);
    let bound = rational_reconstruction_balanced_bound(m);
    // The product of the denominators reconstructed so far.
    let mut d = Natural::ONE;
    let mut fractions = Vec::new();
    for r in residues {
        let t = (r * &d).div_mod_precomputed(m, &data).1;
        let (sign, n, x_d) = rational_reconstruct_with_bounds(t, m.clone(), &bound, &bound)?;
        d *= x_d;
        fractions.push(Fraction::from_sign_and_naturals(sign, n, d.clone()));
    }
    Some(fractions)
}

// The iteration of the next stabilization check of a lifting or multimodular solver, after one at
// iteration `i`: $\lfloor 1.4i \rfloor + 1$, with the product computed in double precision as in
// FLINT, where it is `(slong)(i*1.4) + 1`.
pub(crate) fn next_check_iteration(i: u64) -> u64 {
    u64::rounding_from(f64::rounding_from(i, Nearest).0 * 1.4, Down).0 + 1
}

/// Whether the `height`-by-`width` matrix of fractions `x` solves $AX = B$: the columns of $X$ are
/// cleared of denominators, giving an integer matrix $X'$ with column denominators $d_j$, and $AX'$
/// is compared with $B$ times the $d_j$.
///
/// This is equivalent to `_fmpq_mat_check_solution_fmpz_mat` from `fmpq_mat/can_solve_multi_mod.c`,
/// FLINT 3.6.0.
#[doc(hidden)]
pub fn is_solution<'a>(
    x: &[Fraction],
    height: usize,
    width: usize,
    a: &IntegerMatrix,
    b: impl Into<RightHandSide<'a>>,
) -> bool {
    let b = b.into();
    assert_eq!(a.width, height, "the width of A must equal the height of X");
    assert_eq!(a.height, b.height, "the heights of A and B must be equal");
    assert_eq!(width, b.width, "the widths of X and B must be equal");
    // The common denominator of each column, and the cleared numerators.
    let mut denominators = vec![Natural::ONE; width];
    for row in x.chunks_exact(width.max(1)).take(height) {
        for (d, f) in denominators.iter_mut().zip(row) {
            d.lcm_assign(&f.denominator);
        }
    }
    let mut cleared = Vec::with_capacity(height * width);
    for row in x.chunks_exact(width.max(1)).take(height) {
        for (d, f) in denominators.iter().zip(row) {
            cleared.push(if *d == f.denominator {
                f.signed_numerator()
            } else {
                f.signed_numerator() * Integer::from(d.div_exact(&f.denominator))
            });
        }
    }
    let ax = a * IntegerMatrix::from_owned_elements(
        u64::exact_from(height),
        u64::exact_from(width),
        cleared,
    );
    if width == 0 {
        return true;
    }
    ax.elements
        .chunks_exact(width)
        .zip(b.elements.chunks_exact(width))
        .all(|(ax_row, b_row)| {
            ax_row
                .iter()
                .zip(b_row)
                .zip(&denominators)
                .all(|((p, q), d)| Integer::cmp_product(q, &Integer::from(d), p) == Ordering::Equal)
        })
}

/// The `height`-by-`width` matrix of fractions `x` over a common denominator: an integer matrix of
/// numerators and the least common multiple of the denominators, 1 for a matrix with no elements.
///
/// This is equivalent to `fmpq_mat_get_fmpz_mat_matwise` from `fmpq_mat/get_fmpz_mat_matwise.c`,
/// FLINT 3.6.0.
#[doc(hidden)]
pub fn fractions_to_numerators_and_denominator(
    x: &[Fraction],
    height: usize,
    width: usize,
) -> (IntegerMatrix, Natural) {
    let mut lcm = Natural::ONE;
    for f in x {
        lcm.lcm_assign(&f.denominator);
    }
    let elements = x
        .iter()
        .map(|f| {
            if lcm == f.denominator {
                f.signed_numerator()
            } else {
                f.signed_numerator() * Integer::from((&lcm).div_exact(&f.denominator))
            }
        })
        .collect();
    (
        IntegerMatrix {
            elements,
            height,
            width,
        },
        lcm,
    )
}
