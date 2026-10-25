// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2026 Edgar Costa
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::natural::Natural;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    AddMulAssign, DivExactAssign, ElementaryDivisors, HermiteNormalForm, Mod, ModLeftNullspace,
    ModRank, Pow, SmithNormalForm,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::Factor;

// Multiplies into the elements of `ed`, which are in increasing order of p-adic valuation, the
// powers of the prime `p` in the elementary divisors of `r`, an integer matrix of full row rank, by
// the algorithm of Luebeck: while `r` has a nontrivial left nullspace modulo p, each basis vector
// of that nullspace gives a combination of rows divisible by p, which replaces one of those rows,
// divided by p. The nullity at each level is the number of elementary divisors whose p-adic
// valuation exceeds the level.
//
// This is equivalent to `_fmpz_mat_elementary_divisors_p` from `fmpz_mat/elementary_divisors.c`,
// FLINT 3.6.0.
fn elementary_divisors_p(ed: &mut [Natural], mut r: IntegerMatrix, p: u64) {
    let (h, w) = (r.height, r.width);
    let mut nullities = Vec::new();
    loop {
        let rp = (&r).mod_op(p);
        let rank = usize::exact_from((&rp).mod_rank(p));
        if rank == h {
            break;
        }
        let nullity = h - rank;
        nullities.push(nullity);
        // Row j of n is a vector v with vR ≡ 0 (mod p), with a 1 in a column where the other rows
        // have 0, so that each row of n is assigned a distinct row of R.
        let n = rp.mod_left_nullspace(p);
        let n = n.elements_ref();
        let mut assigned_rows = Vec::with_capacity(nullity);
        for j in 0..nullity {
            let row = (0..h)
                .find(|&i| n[j * h + i] == 1 && (0..nullity).all(|k| k == j || n[k * h + i] == 0))
                .expect(
                    "the left nullspace basis does not have the expected free-variable structure",
                );
            assigned_rows.push(row);
        }
        // Compute all the combinations from the original R, and then replace the rows.
        let mut combinations = Vec::with_capacity(nullity);
        for j in 0..nullity {
            let mut combination = vec![Integer::ZERO; w];
            for (&c, row) in n[j * h..(j + 1) * h].iter().zip(r.elements.chunks_exact(w)) {
                if c != 0 {
                    let c = Integer::from(c);
                    for (x, y) in combination.iter_mut().zip(row) {
                        x.add_mul_assign(&c, y);
                    }
                }
            }
            let p = Integer::from(p);
            for x in &mut combination {
                x.div_exact_assign(&p);
            }
            combinations.push(combination);
        }
        for (row, combination) in assigned_rows.into_iter().zip(combinations) {
            r.elements[row * w..(row + 1) * w].clone_from_slice(&combination);
        }
    }
    // The valuation of the ith elementary divisor is the number of levels whose nullity is at least
    // h - i.
    for (i, x) in ed.iter_mut().enumerate() {
        let level = nullities
            .iter()
            .filter(|&&nullity| nullity >= h - i)
            .count();
        if level != 0 {
            *x *= Natural::from(p).pow(u64::exact_from(level));
        }
    }
}

// The smallest rank of the matrices whose elementary divisors are found by the algorithm of
// Luebeck, which is slower than the Smith normal form for smaller ones.
const LUEBECK_MIN_RANK: usize = 32;

// The first `r` diagonal elements of the Smith normal form of `a`, which has rank `r`: its
// elementary divisors.
//
// This is equivalent to `_fmpz_mat_elementary_divisors_via_snf` from
// `fmpz_mat/elementary_divisors.c`, FLINT 3.6.0.
crate_test_fn! {
elementary_divisors_via_snf(a: &IntegerMatrix, r: usize) -> Vec<Natural> {
    let w = a.width;
    let s = a.smith_normal_form();
    (0..r)
        .map(|i| Natural::exact_from(&s.elements[i * (w + 1)]))
        .collect()
}}

// The elementary divisors of `a`: the nonzero diagonal elements of its Smith normal form. The rank
// r is found from the Hermite normal form H. When r is at least `min_rank` and each pivot of H fits
// in a `u64`, the pivots are factored, and the powers of each prime factor in the elementary
// divisors are found by the algorithm of Luebeck from the r nonzero rows of H; otherwise they are
// read off from the Smith normal form of H, which is that of `a`.
//
// This is equivalent to `fmpz_mat_elementary_divisors` from `fmpz_mat/elementary_divisors.c`, FLINT
// 3.6.0, except that the algorithm of Luebeck is used only when the rank is at least `min_rank`
// (32, for the public function) and every pivot fits in a `u64`, rather than whenever each pivot
// has at most 128 bits and has no prime factor of more than 64 bits, and that otherwise the Smith
// normal form is found from H rather than from `a`.
fn elementary_divisors_with_luebeck_min_rank(a: &IntegerMatrix, min_rank: usize) -> Vec<Natural> {
    let w = a.width;
    let mut h = a.hermite_normal_form();
    let r = if w == 0 {
        0
    } else {
        h.elements
            .chunks_exact(w)
            .take_while(|row| row.iter().any(|x| *x != 0u32))
            .count()
    };
    if r == 0 {
        return Vec::new();
    }
    // The Smith normal form of H is that of A, and is found with less work.
    if r < min_rank {
        return elementary_divisors_via_snf(&h, r);
    }
    // Factor each pivot, collecting the distinct prime factors.
    let mut primes: Vec<u64> = Vec::new();
    for row in h.elements.chunks_exact(w).take(r) {
        let pivot = row.iter().find(|x| **x != 0u32).unwrap();
        if *pivot == 1u32 {
            continue;
        }
        let Ok(pivot) = u64::try_from(pivot) else {
            return elementary_divisors_via_snf(&h, r);
        };
        for (p, _) in pivot.factor() {
            if !primes.contains(&p) {
                primes.push(p);
            }
        }
    }
    let mut ed = vec![Natural::ONE; r];
    h.elements.truncate(r * w);
    h.height = r;
    for p in primes {
        elementary_divisors_p(&mut ed, h.clone(), p);
    }
    ed
}

// The elementary divisors of `a`, as `ElementaryDivisors` finds them.
fn elementary_divisors_helper(a: &IntegerMatrix) -> Vec<Natural> {
    elementary_divisors_with_luebeck_min_rank(a, LUEBECK_MIN_RANK)
}

// The elementary divisors of `a`, by the algorithm of Luebeck whenever every pivot of the Hermite
// normal form fits in a `u64`, whatever the rank.
crate_test_fn! {
elementary_divisors_luebeck(a: &IntegerMatrix) -> Vec<Natural> {
    elementary_divisors_with_luebeck_min_rank(a, 0)
}}

impl ElementaryDivisors for IntegerMatrix {
    type Output = Vec<Natural>;

    /// Computes the elementary divisors of an [`IntegerMatrix`], taking it by value.
    ///
    /// The elementary divisors $d_1 \mid d_2 \mid \cdots \mid d_r$ of a matrix $A$ of rank $r$ are
    /// the nonzero diagonal elements of its Smith normal form (see
    /// [`smith_normal_form`](malachite_base::num::arithmetic::traits::SmithNormalForm)).
    ///
    /// The Hermite normal form $H$ of $A$ is found first. When the rank is at least 32 and every
    /// pivot of $H$ fits in a [`u64`], the pivots are factored, and for each prime factor $p$ the
    /// powers of $p$ in the elementary divisors are found by the algorithm of Luebeck: while the
    /// nonzero rows of $H$ have a nontrivial left nullspace modulo $p$, each basis vector of the
    /// nullspace gives a combination of rows divisible by $p$, which, divided by $p$, replaces one
    /// of them, and the nullity at each level counts the elementary divisors whose $p$-adic
    /// valuation exceeds the level. Otherwise the elementary divisors are read off from the Smith
    /// normal form of $H$, which is that of $A$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^8 (b + \log n)^3 + n^8 (b + \log n)^2 \log (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^4 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of its elements: the bounds
    /// are those of
    /// [`smith_normal_form`](malachite_base::num::arithmetic::traits::SmithNormalForm), and the
    /// algorithm of Luebeck, which uses $O(n \log (nb))$ primes, each with $O(n(b + \log n))$
    /// levels, is within them.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ElementaryDivisors;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[2, 4, 4; -6, 6, 12; 10, -4, -16]").unwrap();
    /// assert_eq!(a.elementary_divisors().to_debug_string(), "[2, 6, 12]");
    ///
    /// // A matrix of rank 2
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(a.elementary_divisors().to_debug_string(), "[1, 3]");
    ///
    /// let a = IntegerMatrix::from_str("[6, 4; 4, 6; 9, 3]").unwrap();
    /// assert_eq!(a.elementary_divisors().to_debug_string(), "[1, 2]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_elementary_divisors` from `fmpz_mat/elementary_divisors.c`,
    /// FLINT 3.6.0, except that the algorithm of Luebeck is used only when the rank is at least 32
    /// and every pivot fits in a [`u64`], rather than whenever each pivot has at most 128 bits and
    /// no prime factor of more than 64 bits, and that otherwise the Smith normal form is found from
    /// the Hermite normal form.
    #[inline]
    fn elementary_divisors(self) -> Vec<Natural> {
        elementary_divisors_helper(&self)
    }
}

impl ElementaryDivisors for &IntegerMatrix {
    type Output = Vec<Natural>;

    /// Computes the elementary divisors of an [`IntegerMatrix`], taking it by reference.
    ///
    /// The elementary divisors $d_1 \mid d_2 \mid \cdots \mid d_r$ of a matrix $A$ of rank $r$ are
    /// the nonzero diagonal elements of its Smith normal form (see
    /// [`smith_normal_form`](malachite_base::num::arithmetic::traits::SmithNormalForm)).
    ///
    /// The Hermite normal form $H$ of $A$ is found first. When the rank is at least 32 and every
    /// pivot of $H$ fits in a [`u64`], the pivots are factored, and for each prime factor $p$ the
    /// powers of $p$ in the elementary divisors are found by the algorithm of Luebeck: while the
    /// nonzero rows of $H$ have a nontrivial left nullspace modulo $p$, each basis vector of the
    /// nullspace gives a combination of rows divisible by $p$, which, divided by $p$, replaces one
    /// of them, and the nullity at each level counts the elementary divisors whose $p$-adic
    /// valuation exceeds the level. Otherwise the elementary divisors are read off from the Smith
    /// normal form of $H$, which is that of $A$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^8 (b + \log n)^3 + n^8 (b + \log n)^2 \log (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^4 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of its elements: the bounds
    /// are those of
    /// [`smith_normal_form`](malachite_base::num::arithmetic::traits::SmithNormalForm), and the
    /// algorithm of Luebeck, which uses $O(n \log (nb))$ primes, each with $O(n(b + \log n))$
    /// levels, is within them.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ElementaryDivisors;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[2, 4, 4; -6, 6, 12; 10, -4, -16]").unwrap();
    /// assert_eq!((&a).elementary_divisors().to_debug_string(), "[2, 6, 12]");
    ///
    /// // A matrix of rank 2
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!((&a).elementary_divisors().to_debug_string(), "[1, 3]");
    ///
    /// let a = IntegerMatrix::from_str("[6, 4; 4, 6; 9, 3]").unwrap();
    /// assert_eq!((&a).elementary_divisors().to_debug_string(), "[1, 2]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_elementary_divisors` from `fmpz_mat/elementary_divisors.c`,
    /// FLINT 3.6.0, except that the algorithm of Luebeck is used only when the rank is at least 32
    /// and every pivot fits in a [`u64`], rather than whenever each pivot has at most 128 bits and
    /// no prime factor of more than 64 bits, and that otherwise the Smith normal form is found from
    /// the Hermite normal form.
    #[inline]
    fn elementary_divisors(self) -> Vec<Natural> {
        elementary_divisors_helper(self)
    }
}
