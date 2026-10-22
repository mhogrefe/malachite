// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use itertools::Itertools;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{FractionFreeLuDecomposition, Parity};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::generators::{integer_matrix_gen, integer_matrix_pair_gen_var_4};
use malachite_nz::test_util::integer_matrix::arithmetic::determinant::*;
use malachite_nz::test_util::integer_matrix::arithmetic::fraction_free_lu_decomposition::*;

// Checks the decomposition `(p, l, u, den)` of `a` and returns its rank: `p` is a permutation, `l`
// is lower trapezoidal and `u` is in row echelon form, their diagonals are the pivots, the last
// pivot is `den`, and $PA = LD^{-1}U$ with $D = \operatorname{diag}(p_0, p_0p_1, \ldots,
// p_{r-2}p_{r-1})$. The last identity is checked after multiplying by the product of the elements
// of $D$.
fn verify_decomposition(a: &IntegerMatrix) -> (Vec<u64>, IntegerMatrix, IntegerMatrix, Integer) {
    let result = a.fraction_free_lu_decomposition();
    assert_eq!(a.clone().fraction_free_lu_decomposition(), result);
    assert_eq!(
        integer_matrix_fraction_free_lu_decomposition_naive(a),
        result
    );
    assert_eq!(
        integer_matrix_fraction_free_lu_decomposition_general(a),
        result
    );
    let (p, l, u, den) = &result;
    assert!(l.is_valid());
    assert!(u.is_valid());
    let h = a.height();
    let w = a.width();
    let r = l.width();
    assert_eq!(l.height(), h);
    assert_eq!(u.height(), r);
    assert_eq!(u.width(), w);
    assert!(r <= h.min(w));
    assert_eq!(
        p.iter().copied().sorted().collect_vec(),
        (0..h).collect_vec()
    );
    // The pivots: the first nonzero element of each row of `u`, strictly to the right of the one
    // above, which is also the diagonal element of `l`.
    let mut pivot_columns = Vec::new();
    for k in 0..r {
        let c = u64::exact_from(u.row_iter(k).position(|x| *x != 0u32).unwrap());
        if let Some(&q) = pivot_columns.last() {
            assert!(c > q);
        }
        assert_eq!(l.get(k, k), u.get(k, c));
        for i in 0..k {
            assert_eq!(*l.get(i, k).unwrap(), 0u32);
        }
        pivot_columns.push(c);
    }
    let pivots: Vec<&Integer> = (0..r).map(|k| l.get(k, k).unwrap()).collect();
    assert_eq!(*den, pivots.last().map_or(Integer::ONE, |&x| x.clone()));
    let d: Vec<Integer> = (0..pivots.len())
        .map(|k| {
            if k == 0 {
                pivots[0].clone()
            } else {
                pivots[k - 1] * pivots[k]
            }
        })
        .collect();
    let d_product: Integer = d.iter().product();
    for i in 0..h {
        let row = a.row_iter(p[usize::exact_from(i)]).collect_vec();
        for j in 0..w {
            let mut sum = Integer::ZERO;
            for (k, dk) in d.iter().enumerate() {
                let k = u64::exact_from(k);
                sum += l.get(i, k).unwrap() * u.get(k, j).unwrap() * (&d_product / dk);
            }
            assert_eq!(sum, row[usize::exact_from(j)] * &d_product);
        }
    }
    // For a square matrix of full rank, the denominator is the determinant, up to sign.
    if a.is_square() && r == h {
        assert_eq!(
            den.unsigned_abs_ref(),
            integer_matrix_determinant_naive(a).unsigned_abs_ref()
        );
    }
    result
}

fn test(a: &str, p: &str, l: &str, u: &str, den: &str) {
    let a = IntegerMatrix::from_str(a).unwrap();
    let (p_actual, l_actual, u_actual, den_actual) = verify_decomposition(&a);
    assert_eq!(format!("[{}]", p_actual.iter().join(", ")), p);
    assert_eq!(l_actual.to_string(), l);
    assert_eq!(u_actual.to_string(), u);
    assert_eq!(den_actual.to_string(), den);
}

#[test]
fn test_fraction_free_lu_decomposition() {
    test("[]_{0,0}", "[]", "[]_{0,0}", "[]_{0,0}", "1");
    test("[]_{2,0}", "[0, 1]", "[]_{2,0}", "[]_{0,0}", "1");
    test("[]_{0,3}", "[]", "[]_{0,0}", "[]_{0,3}", "1");
    test("[0]", "[0]", "[]_{1,0}", "[]_{0,1}", "1");
    test("[5]", "[0]", "[5]", "[5]", "5");
    test("[0, 0; 0, 0]", "[0, 1]", "[]_{2,0}", "[]_{0,2}", "1");
    test(
        "[1, 2; 3, 4]",
        "[0, 1]",
        "[1, 0; 3, -2]",
        "[1, 2; 0, -2]",
        "-2",
    );
    test(
        "[0, 1; 1, 0]",
        "[1, 0]",
        "[1, 0; 0, 1]",
        "[1, 0; 0, 1]",
        "1",
    );
    test("[2, 4; 1, 2]", "[0, 1]", "[2; 1]", "[2, 4]", "2");
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        "[0, 1, 2]",
        "[1, 0, 0; 4, -3, 0; 7, -6, -3]",
        "[1, 2, 3; 0, -3, -6; 0, 0, -3]",
        "-3",
    );
    test(
        "[1, 2, 3; 2, 4, 6; 1, 0, 1]",
        "[0, 2, 1]",
        "[1, 0; 1, -2; 2, 0]",
        "[1, 2, 3; 0, -2, -2]",
        "-2",
    );
    test(
        "[0, 0, 1; 0, 2, 3]",
        "[1, 0]",
        "[2, 0; 0, 2]",
        "[0, 2, 3; 0, 0, 2]",
        "2",
    );
    test("[1; 2; 3]", "[0, 1, 2]", "[1; 2; 3]", "[1]", "1");
    test("[1, 2, 3, 4]", "[0]", "[1]", "[1, 2, 3, 4]", "1");
    test(
        "[2, 1, 1; 1, 1, 0; 0, 0, 0; 4, 3, 2]",
        "[0, 1, 3, 2]",
        "[2, 0, 0; 1, 1, 0; 4, 2, 1; 0, 0, 0]",
        "[2, 1, 1; 0, 1, -1; 0, 0, 1]",
        "1",
    );
    test(
        "[1267650600228229401496703205376, 1; 1, 1267650600228229401496703205376]",
        "[0, 1]",
        "[1267650600228229401496703205376, 0; 1, 1606938044258990275541962092341162602522202993782\
        792835301375]",
        "[1267650600228229401496703205376, 1; 0, 1606938044258990275541962092341162602522202993782\
        792835301375]",
        "1606938044258990275541962092341162602522202993782792835301375",
    );
    test(
        "[1, 0; 0, 1]",
        "[0, 1]",
        "[1, 0; 0, 1]",
        "[1, 0; 0, 1]",
        "1",
    );
    test("[3, -1; -6, 2]", "[0, 1]", "[3; -6]", "[3, -1]", "3");
}

// The next output of the SplitMix64 generator with state `seed`.
const fn splitmix64(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *seed;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

// A matrix with pseudorandom elements in [-2^15, 2^15).
fn pseudorandom_matrix(height: u64, width: u64, seed: &mut u64) -> IntegerMatrix {
    let elements = (0..height * width)
        .map(|_| Integer::from(i16::wrapping_from(splitmix64(seed))))
        .collect();
    IntegerMatrix::from_owned_elements(height, width, elements)
}

// A pseudorandom `height` by `width` matrix with rank at most `rank`: a product of `height` by
// `rank` and `rank` by `width` matrices.
fn pseudorandom_matrix_with_rank(
    height: u64,
    width: u64,
    rank: u64,
    seed: &mut u64,
) -> IntegerMatrix {
    pseudorandom_matrix(height, rank, seed) * pseudorandom_matrix(rank, width, seed)
}

#[test]
fn test_fraction_free_lu_decomposition_large() {
    let mut seed = 0;
    for (h, w, r) in [(12, 12, 12), (12, 12, 7), (8, 15, 5), (15, 9, 9), (10, 10, 0)] {
        let a = pseudorandom_matrix_with_rank(h, w, r, &mut seed);
        assert_eq!(verify_decomposition(&a).1.width(), r);
    }
}

// A matrix with pseudorandom elements of at most `bits` bits, of either sign.
fn pseudorandom_matrix_with_bits(
    height: u64,
    width: u64,
    bits: u64,
    seed: &mut u64,
) -> IntegerMatrix {
    let elements = (0..height * width)
        .map(|_| {
            let x = Integer::from(splitmix64(seed) >> (64 - bits));
            if splitmix64(seed).even() { x } else { -x }
        })
        .collect();
    IntegerMatrix::from_owned_elements(height, width, elements)
}

#[test]
fn test_fraction_free_lu_decomposition_small_and_large_elements() {
    let mut seed = 0;
    // - elements of 30 bits, whose products eventually outgrow a word
    // - elements of 61 and 62 bits, whose first products already outgrow a word
    // - elements of 63 bits, too large for the small-element path from the start
    for bits in [30, 61, 62, 63] {
        for (h, w) in [(4, 4), (6, 5), (3, 7)] {
            verify_decomposition(&pseudorandom_matrix_with_bits(h, w, bits, &mut seed));
        }
    }
}

#[test]
fn fraction_free_lu_decomposition_properties() {
    integer_matrix_gen().test_properties(|a| {
        verify_decomposition(&a);
    });

    integer_matrix_pair_gen_var_4().test_properties(|(a, b)| {
        let r = verify_decomposition(&(&a * &b)).1.width();
        assert!(r <= a.width());
    });
}
