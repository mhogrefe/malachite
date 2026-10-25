// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{Parity, Permanent};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::permanent::*;
use malachite_nz::test_util::generators::{
    integer_matrix_gen_var_2, integer_matrix_gen_var_4, integer_matrix_integer_pair_gen,
};
use malachite_nz::test_util::integer_matrix::arithmetic::permanent::integer_matrix_permanent_naive;
use std::panic::catch_unwind;

// Checks the permanent of `a` against every algorithm that applies to it.
fn verify_permanent(a: &IntegerMatrix) -> Integer {
    let p = a.clone().permanent();
    assert_eq!(a.permanent(), p);
    let n = a.height();
    if n <= 12 {
        assert_eq!(permanent_dense(a), p);
    }
    assert_eq!(permanent_structured(a.clone()), p);
    if n <= 6 {
        assert_eq!(permanent_cofactor(a), p);
    }
    if (1..=10).contains(&n) {
        assert_eq!(permanent_glynn(a), p);
    }
    if n <= 8 {
        assert_eq!(integer_matrix_permanent_naive(a), p);
    }
    p
}

#[test]
fn test_permanent() {
    let test = |s, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let p = verify_permanent(&a);
        assert_eq!(p.to_string(), out);
        assert_eq!(verify_permanent(&a.transpose()), p);
    };
    // - dimension 0
    test("[]_{0,0}", "1");
    // - dimension 1
    test("[-7]", "-7");
    // - dimension 2
    test("[1, 2; 3, 4]", "10");
    test("[1, -2; 3, 4]", "-2");
    // - dimension 3: cofactor expansion
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "450");
    test(
        "[4611686018427387903, 1, 1; 1, 4611686018427387903, 1; 1, 1, 4611686018427387903]",
        "98079714615416886871131265939943825879886681036858327038",
    );
    // - dimension 4, small elements: Glynn's formula on words
    test("[1, 2, 3, 4; 5, 6, 7, 8; 2, 6, 4, 8; 3, 1, 1, 2]", "3472");
    // - dimension 4, large elements: cofactor expansion
    test(
        "[1180591620717411303424, 1, 2, 3; 4, -1180591620717411303424, 5, 6; \
        7, 8, 1180591620717411303424, 9; 10, 11, 12, 1180591620717411303424]",
        "-1942668892225729070919461906823518906642465378508285664137557843049162719961649504879",
    );
    // - dimension 5, small elements: Glynn's formula on words
    test(
        "[1, -2, 3, -4, 5; -6, 7, -8, 9, -10; 11, -12, 13, -14, 15; -16, 17, -18, 19, -20; \
        21, -22, 23, -24, 26]",
        "14573776",
    );
    // - dimension 5, large elements: Glynn's formula on Integers
    test(
        "[1180591620717411303424, 1, 2, 3, 4; 5, 1180591620717411303424, 6, 7, 8; \
        9, 10, -1180591620717411303424, 11, 12; 13, 14, 15, 1180591620717411303424, 16; \
        17, 18, 19, 20, 1180591620717411303424]",
        "-22934986159900715116108208953020869407968990265934173285641368732785424892363907925370784\
        05610193717223512",
    );
    test(
        "[3, 1, 4, 1, 5, 9; 2, 6, 5, 3, 5, 8; 9, 7, 9, 3, 2, 3; 8, 4, 6, 2, 6, 4; \
        3, 3, 8, 3, 2, 7; 9, 5, 0, 2, 8, 8]",
        "6896252",
    );
    // - all ones: n!
    test(
        "[1, 1, 1, 1, 1, 1; 1, 1, 1, 1, 1, 1; 1, 1, 1, 1, 1, 1; 1, 1, 1, 1, 1, 1; \
        1, 1, 1, 1, 1, 1; 1, 1, 1, 1, 1, 1]",
        "720",
    );
    // - zero diagonal, ones elsewhere: the number of derangements
    test(
        "[0, 1, 1, 1, 1, 1, 1; 1, 0, 1, 1, 1, 1, 1; 1, 1, 0, 1, 1, 1, 1; 1, 1, 1, 0, 1, 1, 1; \
        1, 1, 1, 1, 0, 1, 1; 1, 1, 1, 1, 1, 0, 1; 1, 1, 1, 1, 1, 1, 0]",
        "1854",
    );
    test(
        "[-5, -2, 1, 4, -4, -1, 2, 5; 2, 5, -3, 0, 3, -5, -2, 1; -2, 1, 4, -4, -1, 2, 5, -3; \
        5, -3, 0, 3, -5, -2, 1, 4; 1, 4, -4, -1, 2, 5, -3, 0; -3, 0, 3, -5, -2, 1, 4, -4; \
        4, -4, -1, 2, 5, -3, 0, 3; 0, 3, -5, -2, 1, 4, -4, -1]",
        "-824014",
    );
    // - dimension 5, elements of about 10 bits: Glynn's formula on words, 4 factors at a time
    test(
        "[256, 763, 733, -610, -813; 622, -697, 20, -216, -914; -757, 85, 695, -957, 755; \
        -989, 754, 698, 847, -333; -870, 39, -730, 477, -113]",
        "187198997500297",
    );
    // - dimension 5, elements of about 20 bits: Glynn's formula on words, 2 factors at a time
    test(
        "[34447, -850169, -796122, -392898, 370576; 381757, -83738, 382812, -513636, 292072; \
        938314, 954214, -953459, -685337, 359236; -819364, 657456, -62293, -30703, -609601; \
        301393, 518348, -900213, 669262, -400216]",
        "63077072637586351217806696857",
    );
    // - dimension 5, elements of about 40 bits: Glynn's formula on words, 1 factor at a time
    test(
        "[-459575895295, 1060839173422, 509236780095, -796490387226, 658424768235; \
        -484943371756, 408420741772, 415898577219, 21180798887, -148149359686; \
        -1000003736895, 499502565336, -977544392695, 593668900844, -55591394781; \
        -994189484284, -812534894534, 513499589272, 759435661654, -949560026141; \
        264028283045, -363188861065, 800992970368, 549029265006, 218629922895]",
        "-335736459914737536183791875013439569857547235000609077229804",
    );
    // - dimension 5, row sums too large for words: Glynn's formula on Integers
    test(
        "[-4127195245393839, -6598075111005988, 7002805659164900, 2952604509871474, \
        -278445019329094; 2877495999384027, -8812737368551806, 1745045772546411, \
        8201269052012459, 4089514418362404; -748995049688436, -427400059065481, \
        8906446698609980, -8793537283772096, 2118612122585206; -3662117653761108, \
        -1427394313368860, -3282809170008475, 7672648229646398, 5238482936564153; \
        -3719875613002424, 6044527154490950, -5300531925884171, 8272331737468815, \
        852508383684422]",
        "16939416244715746987840913845854021325251070777333299152360181156237488182767926",
    );
    // - a column with a single nonzero element
    test("[5, 1, 1; 0, 1, 1; 0, 1, 1]", "10");
    // - a column with two nonzero elements and no row with two: contraction of the column
    test("[1, 1, 1, 1; 1, 1, 1, 1; 0, 1, 1, 1; 0, 1, 1, 1]", "12");
    // - independent blocks that are not square
    test(
        "[1, 1, 0, 0, 0; 1, 1, 0, 0, 0; 1, 1, 0, 0, 0; 0, 0, 1, 1, 1; 0, 0, 1, 1, 1]",
        "0",
    );
    // - independent square blocks, one with permanent 0
    test(
        "[1, -1, 0, 0, 0; 1, 1, 0, 0, 0; 0, 0, 1, 2, 3; 0, 0, 4, 5, 6; 0, 0, 7, 8, 9]",
        "0",
    );
    // - a zero row
    test("[1, 2, 3; 0, 0, 0; 7, 8, 9]", "0");
    // - rows that meet only one column: the blocks are not square
    test(
        "[1, 0, 0, 0, 0; 2, 0, 0, 0, 0; 0, 1, 1, 1, 1; 0, 1, 1, 1, 1; 0, 1, 1, 1, 1]",
        "0",
    );
}

// The square matrix of dimension `n` whose element in row `i` and column `j` is `f(i, j)`.
fn matrix_from_fn(n: u64, f: impl Fn(u64, u64) -> Integer) -> IntegerMatrix {
    let elements = (0..n * n).map(|k| f(k / n, k % n)).collect();
    IntegerMatrix::from_owned_elements(n, n, elements)
}

#[test]
fn test_permanent_structured() {
    let test = |a: IntegerMatrix, out| {
        assert_eq!(verify_permanent(&a).to_string(), out);
    };
    // - a row with a single nonzero element, repeatedly: the identity
    test(IntegerMatrix::identity(100), "1");
    // - triangular, with elements above the diagonal
    test(
        matrix_from_fn(60, |i, j| match i.cmp(&j) {
            core::cmp::Ordering::Less => Integer::from(i + 3 * j),
            core::cmp::Ordering::Equal => Integer::from(i + 1),
            core::cmp::Ordering::Greater => Integer::ZERO,
        }),
        "8320987112741390144276341183223364380754172606361245952449277696409600000000000000",
    );
    // - independent blocks, more than 64 rows in all
    test(
        matrix_from_fn(66, |i, j| {
            if i / 2 == j / 2 {
                Integer::from(2 * (i % 2) + j % 2 + 1)
            } else {
                Integer::ZERO
            }
        }),
        "1000000000000000000000000000000000",
    );
    // - rows with two nonzero elements: contraction
    test(
        matrix_from_fn(30, |i, j| {
            if i.abs_diff(j) <= 1 {
                Integer::ONE
            } else {
                Integer::ZERO
            }
        }),
        "1346269",
    );
    // - more than 64 rows, connected, with rows of two nonzero elements: contraction
    test(
        matrix_from_fn(70, |i, j| {
            if i.abs_diff(j) <= 1 {
                Integer::ONE
            } else {
                Integer::ZERO
            }
        }),
        "308061521170129",
    );
    // - more than 64 rows, connected, with three nonzero elements in every row and column:
    //   expansion along the sparsest row. The permanent of I + P + P^2, for a cyclic permutation
    //   matrix P, is 2 plus a Lucas number.
    test(
        matrix_from_fn(66, |i, j| {
            if (j + 66 - i) % 66 <= 2 {
                Integer::ONE
            } else {
                Integer::ZERO
            }
        }),
        "62113250390420",
    );
}

#[test]
fn permanent_fail() {
    let a = IntegerMatrix::from_str("[1, 2]").unwrap();
    assert_panic!(a.clone().permanent());
    assert_panic!((&a).permanent());
    assert_panic!(permanent_cofactor(&a));
    assert_panic!(permanent_glynn(&a));
    assert_panic!(permanent_dense(&a));
    assert_panic!(permanent_structured(a.clone()));
    assert_panic!(permanent_glynn(
        &IntegerMatrix::from_str("[]_{0,0}").unwrap()
    ));
}

#[test]
fn permanent_properties() {
    integer_matrix_gen_var_4().test_properties(|a| {
        let p = verify_permanent(&a);
        // The permanent of the transpose is the same.
        assert_eq!((&a.transpose()).permanent(), p);
        // Reversing the rows or the columns does not change it.
        let mut b = a.clone();
        b.reverse_rows();
        assert_eq!((&b).permanent(), p);
        b.reverse_columns();
        assert_eq!((&b).permanent(), p);
        // The permanent of the negation is (-1)^n times the permanent.
        let negated = (-&a).permanent();
        if a.height().even() {
            assert_eq!(negated, p);
        } else {
            assert_eq!(negated, -p);
        }
    });

    integer_matrix_gen_var_2().test_properties(|a| {
        assert_eq!(permanent_cofactor(&a), integer_matrix_permanent_naive(&a));
    });

    integer_matrix_integer_pair_gen().test_properties(|(a, c)| {
        if a.is_square() && a.height() <= 8 {
            // Scaling every element by c scales the permanent by c^n.
            let mut q = Integer::ONE;
            for _ in 0..a.height() {
                q *= &c;
            }
            assert_eq!((&a * &c).permanent(), (&a).permanent() * q);
        }
    });
}
