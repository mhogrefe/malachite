// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::fmt::Debug;
use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModLuDecomposition, ModMul};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_10;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_lu_decomposition::*;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_rank::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;

// `a` with its rows permuted: row `i` of the result is row `p[i]` of `a`.
fn permute_rows<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, p: &[u64]) -> UnsignedMatrix<T> {
    let mut elements = Vec::new();
    for &i in p {
        elements.extend(a.row_iter(i).copied());
    }
    UnsignedMatrix::from_owned_elements(a.height(), a.width(), elements)
}

// Checks that `(p, l, u)` is an LU decomposition of `a` modulo `m` as `mod_lu_decomposition`
// describes, and that it matches the packed decompositions computed by every algorithm.
fn verify_lu<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    m: T,
    p: &[u64],
    l: &UnsignedMatrix<T>,
    u: &UnsignedMatrix<T>,
) {
    assert!(l.is_valid());
    assert!(u.is_valid());
    let (h, w) = (a.height(), a.width());
    let r = l.width();
    assert_eq!(l.height(), h);
    assert_eq!(u.height(), r);
    assert_eq!(u.width(), w);
    assert!(l.mod_is_reduced(&m));
    assert!(u.mod_is_reduced(&m));
    let mut sorted = p.to_vec();
    sorted.sort_unstable();
    assert_eq!(sorted, (0..h).collect::<Vec<_>>());
    for i in 0..h {
        for j in 0..r {
            let x = *l.get(i, j).unwrap();
            if i == j {
                assert_eq!(x, T::ONE);
            } else if j > i {
                assert_eq!(x, T::ZERO);
            }
        }
    }
    let mut pivots = Vec::new();
    for i in 0..r {
        let pivot = (0..w).find(|&j| *u.get(i, j).unwrap() != T::ZERO).unwrap();
        if let Some(&q) = pivots.last() {
            assert!(pivot > q);
        }
        pivots.push(pivot);
    }
    assert_eq!(permute_rows(a, p), l.mod_mul(u, m));
    assert_eq!(r, unsigned_matrix_mod_rank_naive(a, m));
    let packed = unsigned_matrix_mod_lu_packed(a, m, false);
    assert_eq!(packed.1, usize::exact_from(r));
    assert_eq!(unsigned_matrix_mod_lu_packed_classical(a, m, false), packed);
    assert_eq!(unsigned_matrix_mod_lu_packed_delayed(a, m, false), packed);
    assert_eq!(unsigned_matrix_mod_lu_packed_recursive(a, m, false), packed);
    let checked = unsigned_matrix_mod_lu_packed(a, m, true);
    // With the rank check, the decomposition succeeds only if no column is skipped before the rank
    // reaches its maximum, which means that every pivot is on the diagonal.
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
}

fn test<T: PrimitiveUnsigned>(a: &str, m: T, p: &[u64], l: &str, u: &str)
where
    <UnsignedMatrix<T> as FromStr>::Err: Debug,
{
    let a = UnsignedMatrix::<T>::from_str(a).unwrap();
    let (p_out, l_out, u_out) = (&a).mod_lu_decomposition(m);
    assert_eq!(p_out, p);
    assert_eq!(l_out.to_string(), l);
    assert_eq!(u_out.to_string(), u);
    assert_eq!(
        a.clone().mod_lu_decomposition(m),
        (p_out.clone(), l_out.clone(), u_out.clone())
    );
    verify_lu(&a, m, &p_out, &l_out, &u_out);
}

#[test]
fn test_mod_lu_decomposition() {
    test::<u64>("[]_{0,0}", 5, &[], "[]_{0,0}", "[]_{0,0}");
    test::<u64>("[]_{2,0}", 5, &[0, 1], "[]_{2,0}", "[]_{0,0}");
    test::<u64>("[]_{0,3}", 5, &[], "[]_{0,0}", "[]_{0,3}");
    test::<u64>("[3]", 7, &[0], "[1]", "[3]");
    test::<u64>("[0]", 7, &[0], "[]_{1,0}", "[]_{0,1}");
    test::<u64>("[0, 1; 1, 0]", 5, &[1, 0], "[1, 0; 0, 1]", "[1, 0; 0, 1]");
    test::<u64>("[2, 1; 4, 3]", 5, &[0, 1], "[1, 0; 2, 1]", "[2, 1; 0, 1]");
    test::<u64>(
        "[1, 2, 3; 2, 4, 6; 1, 0, 1]",
        7,
        &[0, 2, 1],
        "[1, 0; 1, 1; 2, 0]",
        "[1, 2, 3; 0, 5, 5]",
    );
    test::<u64>(
        "[0, 0, 1; 0, 2, 3]",
        11,
        &[1, 0],
        "[1, 0; 0, 1]",
        "[0, 2, 3; 0, 0, 1]",
    );
    test::<u64>("[1; 2; 3]", 5, &[0, 1, 2], "[1; 2; 3]", "[1]");
    test::<u8>("[200, 100; 50, 25]", 251, &[0, 1], "[1; 63]", "[200, 100]");
    test::<u64>("[0, 0]", 1, &[0], "[]_{1,0}", "[]_{0,2}");
    test::<u64>(
        "[18446744073709551556, 5; 7, 18446744073709551000]",
        18446744073709551557,
        &[0, 1],
        "[1, 0; 18446744073709551550, 1]",
        "[18446744073709551556, 5; 0, 18446744073709551035]",
    );
}

// The next output of the SplitMix64 generator with state `seed`.
const fn splitmix64(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *seed;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

// A matrix with pseudorandom elements less than `m`, from the SplitMix64 generator.
fn pseudorandom_matrix(height: u64, width: u64, m: u64, seed: &mut u64) -> UnsignedMatrix<u64> {
    let mut elements = Vec::new();
    for _ in 0..height * width {
        let x = splitmix64(seed);
        elements.push(x % m);
    }
    UnsignedMatrix::from_owned_elements(height, width, elements)
}

// A pseudorandom `height` by `width` matrix modulo `m` with rank at most `rank`: a product of
// `height` by `rank` and `rank` by `width` matrices.
fn pseudorandom_matrix_with_rank(
    height: u64,
    width: u64,
    rank: u64,
    m: u64,
    seed: &mut u64,
) -> UnsignedMatrix<u64> {
    let b = pseudorandom_matrix(height, rank, m, seed);
    let c = pseudorandom_matrix(rank, width, m, seed);
    b.mod_mul(c, m)
}

#[test]
fn test_mod_lu_decomposition_large() {
    // - classical with delayed reduction, one word
    // - recursive
    // - classical with delayed reduction, two words
    // - classical with delayed reduction, three words
    // - recursive, rank-deficient first half
    // - recursive, first half of rank less than 20, so that a product is computed by the basic
    //   classical algorithm
    // - recursive, full-rank first half but rank-deficient matrix
    let mut seed = 0;
    for (h, w, r, m) in [
        (40, 40, 40, 2),
        (70, 70, 70, 2),
        (70, 70, 30, 2),
        (70, 70, 10, 2),
        (70, 70, 50, 2),
        (30, 70, 30, 2),
        (100, 80, 50, 1099511627791),
        (40, 30, 30, 1099511627791),
        (20, 20, 20, 9223372036854775837),
        (20, 20, 9, 9223372036854775837),
        (130, 130, 40, 18446744073709551557),
        (120, 140, 140, 4611686018427388039),
        (60, 60, 60, 2305843009213693951),
    ] {
        let a = pseudorandom_matrix_with_rank(h, w, r, m, &mut seed);
        let (p, l, u) = (&a).mod_lu_decomposition(m);
        verify_lu(&a, m, &p, &l, &u);
    }
    // - classical with delayed reduction, two words with the highest bit of m set, which falls back
    //   to the classical algorithm
    let a = pseudorandom_matrix(3, 3, 9223372036854775837, &mut seed);
    assert_eq!(
        unsigned_matrix_mod_lu_packed_delayed(&a, 9223372036854775837, false),
        unsigned_matrix_mod_lu_packed_classical(&a, 9223372036854775837, false)
    );
    // - three words with too many terms for narrow types, which falls back to the classical
    //   algorithm
    // - three words with few enough terms
    for n in [200, 100] {
        let b = pseudorandom_matrix(n, n, 251, &mut seed);
        let a = UnsignedMatrix::<u8>::from_owned_elements(
            n,
            n,
            (0..n)
                .flat_map(|i| {
                    b.row_iter(i)
                        .map(|&x| u8::exact_from(x))
                        .collect::<Vec<_>>()
                })
                .collect(),
        );
        let (p, l, u) = (&a).mod_lu_decomposition(251);
        verify_lu(&a, 251, &p, &l, &u);
    }
}

#[test]
#[should_panic]
fn mod_lu_decomposition_fail_1() {
    UnsignedMatrix::<u64>::from_str("[1]")
        .unwrap()
        .mod_lu_decomposition(0);
}

#[test]
#[should_panic]
fn mod_lu_decomposition_fail_2() {
    UnsignedMatrix::<u64>::from_str("[5]")
        .unwrap()
        .mod_lu_decomposition(5);
}

#[test]
#[should_panic]
fn mod_lu_decomposition_fail_3() {
    UnsignedMatrix::<u64>::from_str("[2, 1; 1, 1]")
        .unwrap()
        .mod_lu_decomposition(4);
}

#[test]
fn mod_lu_decomposition_properties() {
    unsigned_matrix_unsigned_pair_gen_var_10().test_properties(|(a, m)| {
        let (p, l, u) = (&a).mod_lu_decomposition(m);
        assert_eq!(
            a.clone().mod_lu_decomposition(m),
            (p.clone(), l.clone(), u.clone())
        );
        verify_lu(&a, m, &p, &l, &u);
    });
}
