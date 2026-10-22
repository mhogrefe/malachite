// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModLuDecomposition, ModMul};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_natural_pair_gen_var_5;
use malachite_nz::test_util::natural_matrix::arithmetic::mod_lu_decomposition::*;
use malachite_nz::test_util::natural_matrix::arithmetic::mod_rank::*;

// `a` with its rows permuted: row `i` of the result is row `p[i]` of `a`.
fn permute_rows(a: &NaturalMatrix, p: &[u64]) -> NaturalMatrix {
    let mut elements = Vec::new();
    for &i in p {
        elements.extend(a.row_iter(i).cloned());
    }
    NaturalMatrix::from_owned_elements(a.height(), a.width(), elements)
}

// The permutation, L, and U of a packed LU decomposition.
fn unpack(
    (perm, rank, packed): (Vec<usize>, usize, NaturalMatrix),
) -> (Vec<u64>, NaturalMatrix, NaturalMatrix) {
    let rank = u64::exact_from(rank);
    let (h, w) = (packed.height(), packed.width());
    let mut l = Vec::new();
    for i in 0..h {
        for c in 0..rank {
            l.push(match i.cmp(&c) {
                core::cmp::Ordering::Greater => packed.get(i, c).unwrap().clone(),
                core::cmp::Ordering::Equal => Natural::ONE,
                core::cmp::Ordering::Less => Natural::ZERO,
            });
        }
    }
    let mut u = Vec::new();
    for i in 0..rank {
        for j in 0..w {
            u.push(if j >= i {
                packed.get(i, j).unwrap().clone()
            } else {
                Natural::ZERO
            });
        }
    }
    (
        perm.into_iter().map(u64::exact_from).collect(),
        NaturalMatrix::from_owned_elements(h, rank, l),
        NaturalMatrix::from_owned_elements(rank, w, u),
    )
}

// Checks that `(p, l, u)` is an LU decomposition of `a` modulo `m` as `mod_lu_decomposition`
// describes, that it matches the decompositions computed by the large-modulus algorithms, and that
// the forms of the call agree.
fn verify_lu(a: &NaturalMatrix, m: &Natural) -> (Vec<u64>, NaturalMatrix, NaturalMatrix) {
    let (p, l, u) = a.mod_lu_decomposition(m);
    assert_eq!(
        a.clone().mod_lu_decomposition(m),
        (p.clone(), l.clone(), u.clone())
    );
    assert_eq!(
        a.clone().mod_lu_decomposition(m.clone()),
        (p.clone(), l.clone(), u.clone())
    );
    assert_eq!(
        a.mod_lu_decomposition(m.clone()),
        (p.clone(), l.clone(), u.clone())
    );
    assert!(l.is_valid());
    assert!(u.is_valid());
    let (h, w) = (a.height(), a.width());
    let r = l.width();
    assert_eq!(l.height(), h);
    assert_eq!(u.height(), r);
    assert_eq!(u.width(), w);
    assert!(l.mod_is_reduced(m));
    assert!(u.mod_is_reduced(m));
    let mut sorted = p.clone();
    sorted.sort_unstable();
    assert_eq!(sorted, (0..h).collect::<Vec<_>>());
    for i in 0..h {
        for j in 0..r {
            let x = l.get(i, j).unwrap();
            if i == j {
                assert_eq!(*x, 1u32);
            } else if j > i {
                assert_eq!(*x, 0u32);
            }
        }
    }
    let mut pivots = Vec::new();
    for i in 0..r {
        let pivot = (0..w).find(|&j| *u.get(i, j).unwrap() != 0u32).unwrap();
        if let Some(&q) = pivots.last() {
            assert!(pivot > q);
        }
        pivots.push(pivot);
    }
    assert_eq!(permute_rows(a, &p), (&l).mod_mul(&u, m));
    assert_eq!(r, natural_matrix_mod_rank_naive(a, m));
    // The large-modulus algorithms give the same result, even when the modulus fits in a limb.
    let packed = natural_matrix_mod_lu_packed(a, m, false).unwrap();
    assert_eq!(unpack(packed.clone()), (p.clone(), l.clone(), u.clone()));
    assert_eq!(
        natural_matrix_mod_lu_packed_classical(a, m, false),
        Some(packed.clone())
    );
    assert_eq!(
        natural_matrix_mod_lu_packed_recursive(a, m, false),
        Some(packed.clone())
    );
    let checked = natural_matrix_mod_lu_packed(a, m, true).unwrap();
    // With the rank check, the decomposition of a matrix with at least as many rows as columns
    // succeeds only if no column is skipped before the rank reaches its maximum, which means that
    // every pivot is on the diagonal. (The rank check is only meaningful for such matrices; as in
    // FLINT, the recursive algorithm may report a wider matrix of full rank as rank-deficient.)
    if h >= w {
        if r == w
            && pivots
                .iter()
                .enumerate()
                .all(|(i, &j)| j == u64::exact_from(i))
        {
            assert_eq!(checked, packed);
        } else {
            assert_eq!(checked.1, 0);
        }
    }
    (p, l, u)
}

fn test(a: &str, m: &str, p: &[u64], l: &str, u: &str) {
    let a = NaturalMatrix::from_str(a).unwrap();
    let m = Natural::from_str(m).unwrap();
    let (p_out, l_out, u_out) = verify_lu(&a, &m);
    assert_eq!(p_out, p);
    assert_eq!(l_out.to_string(), l);
    assert_eq!(u_out.to_string(), u);
}

#[test]
fn test_mod_lu_decomposition() {
    test(
        "[]_{0,0}",
        "170141183460469231731687303715884105727",
        &[],
        "[]_{0,0}",
        "[]_{0,0}",
    );
    test(
        "[]_{2,0}",
        "170141183460469231731687303715884105727",
        &[0, 1],
        "[]_{2,0}",
        "[]_{0,0}",
    );
    test(
        "[]_{0,3}",
        "170141183460469231731687303715884105727",
        &[],
        "[]_{0,0}",
        "[]_{0,3}",
    );
    test(
        "[3]",
        "170141183460469231731687303715884105727",
        &[0],
        "[1]",
        "[3]",
    );
    test(
        "[0]",
        "170141183460469231731687303715884105727",
        &[0],
        "[]_{1,0}",
        "[]_{0,1}",
    );
    test(
        "[0, 1; 1, 0]",
        "170141183460469231731687303715884105727",
        &[1, 0],
        "[1, 0; 0, 1]",
        "[1, 0; 0, 1]",
    );
    test(
        "[2, 1; 4, 3]",
        "36893488147419103363",
        &[0, 1],
        "[1, 0; 2, 1]",
        "[2, 1; 0, 1]",
    );
    test(
        "[1, 2, 3; 2, 4, 6; 1, 0, 1]",
        "170141183460469231731687303715884105727",
        &[0, 2, 1],
        "[1, 0; 1, 1; 2, 0]",
        "[1, 2, 3; 0, 170141183460469231731687303715884105725, \
        170141183460469231731687303715884105725]",
    );
    test(
        "[0, 0, 1; 0, 2, 3]",
        "36893488147419103363",
        &[1, 0],
        "[1, 0; 0, 1]",
        "[0, 2, 3; 0, 0, 1]",
    );
    test(
        "[1; 2; 3]",
        "170141183460469231731687303715884105727",
        &[0, 1, 2],
        "[1; 2; 3]",
        "[1]",
    );
    test(
        "[170141183460469231731687303715884105726, 5; 7, 170141183460469231731687303715884105725]",
        "170141183460469231731687303715884105727",
        &[0, 1],
        "[1, 0; 170141183460469231731687303715884105720, 1]",
        "[170141183460469231731687303715884105726, 5; 0, 33]",
    );
    test("[3, 4]", "7", &[0], "[1]", "[3, 4]");
    test("[1, 2; 2, 4]", "11", &[0, 1], "[1; 2]", "[1, 2]");
}

// The next output of the SplitMix64 generator with state `seed`.
const fn splitmix64(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *seed;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

// A pseudorandom element less than `m`, made of several outputs of the SplitMix64 generator.
fn pseudorandom_natural(m: &Natural, seed: &mut u64) -> Natural {
    (0..3).fold(Natural::ZERO, |x, _| {
        (x << 64u32) + Natural::from(splitmix64(seed))
    }) % m
}

// A matrix with pseudorandom elements less than `m`.
fn pseudorandom_matrix(height: u64, width: u64, m: &Natural, seed: &mut u64) -> NaturalMatrix {
    let elements = (0..height * width)
        .map(|_| pseudorandom_natural(m, seed))
        .collect();
    NaturalMatrix::from_owned_elements(height, width, elements)
}

// A pseudorandom `height` by `width` matrix modulo `m` with rank at most `rank`: a product of
// `height` by `rank` and `rank` by `width` matrices.
fn pseudorandom_matrix_with_rank(
    height: u64,
    width: u64,
    rank: u64,
    m: &Natural,
    seed: &mut u64,
) -> NaturalMatrix {
    let b = pseudorandom_matrix(height, rank, m, seed);
    let c = pseudorandom_matrix(rank, width, m, seed);
    b.mod_mul(c, m)
}

#[test]
fn test_mod_lu_decomposition_large() {
    // - recursive
    // - recursive, rank-deficient first half
    // - recursive, full-rank first half but rank-deficient matrix
    let mut seed = 0;
    for (h, w, r, m) in [
        (20, 20, 20, "170141183460469231731687303715884105727"),
        (20, 20, 6, "170141183460469231731687303715884105727"),
        (20, 20, 15, "36893488147419103363"),
        (25, 18, 18, "36893488147419103363"),
        (
            12,
            30,
            12,
            "1606938044258990275541962092341162602522202993782792835301611",
        ),
    ] {
        let m = Natural::from_str(m).unwrap();
        let a = pseudorandom_matrix_with_rank(h, w, r, &m, &mut seed);
        verify_lu(&a, &m);
    }
}

#[test]
#[should_panic]
fn mod_lu_decomposition_fail_1() {
    NaturalMatrix::from_str("[1]")
        .unwrap()
        .mod_lu_decomposition(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_lu_decomposition_fail_2() {
    NaturalMatrix::from_str("[5]")
        .unwrap()
        .mod_lu_decomposition(Natural::from(5u32));
}

#[test]
#[should_panic]
fn mod_lu_decomposition_fail_3() {
    NaturalMatrix::from_str("[2, 1; 1, 1]")
        .unwrap()
        .mod_lu_decomposition(Natural::from_str("1267650600228229401496703205376").unwrap());
}

#[test]
fn mod_lu_decomposition_properties() {
    natural_matrix_natural_pair_gen_var_5().test_properties(|(a, m)| {
        verify_lu(&a, &m);
    });
}
