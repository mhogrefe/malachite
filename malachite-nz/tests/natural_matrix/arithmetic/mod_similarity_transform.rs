// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    Mod, ModDeterminant, ModNeg, ModSimilarityTransform, ModSimilarityTransformAssign, ModSquare,
    ModTrace, SimilarityTransform,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_matrix::arithmetic::similarity_transform::*;
use malachite_base::unsigned_matrix::arithmetic::permanent_over_ring::ModRing;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::*;
use std::panic::catch_unwind;

// Checks every way of calling the transform, FLINT's loops, and the transform over the integers
// reduced modulo `m`, and returns the result.
fn verify(a: &NaturalMatrix, r: u64, d: &Natural, m: &Natural) -> NaturalMatrix {
    let b = a.mod_similarity_transform(r, d, m);
    assert!(b.is_valid());
    assert_eq!(a.clone().mod_similarity_transform(r, d, m), b);
    assert_eq!(a.clone().mod_similarity_transform(r, d, m.clone()), b);
    assert_eq!(a.clone().mod_similarity_transform(r, d.clone(), m), b);
    assert_eq!(
        a.clone().mod_similarity_transform(r, d.clone(), m.clone()),
        b
    );
    let mut c = a.clone();
    c.mod_similarity_transform_assign(r, d.clone(), m.clone());
    assert_eq!(c, b);
    let mut c = a.clone();
    c.mod_similarity_transform_assign(r, d.clone(), m);
    assert_eq!(c, b);
    let mut c = a.clone();
    c.mod_similarity_transform_assign(r, d, m.clone());
    assert_eq!(c, b);
    let mut c = a.clone();
    c.mod_similarity_transform_assign(r, d, m);
    assert_eq!(c, b);
    let n = usize::exact_from(a.height());
    assert_eq!(
        matrix_similarity_transform_flint_loops(
            &ModRing::new(m.clone()),
            a.elements_ref(),
            n,
            usize::exact_from(r),
            d
        ),
        b.elements_ref()
    );
    // The transform commutes with reduction modulo m.
    assert_eq!(
        IntegerMatrix::from(a)
            .similarity_transform(r, Integer::from(d))
            .mod_op(m),
        b
    );
    b
}

#[test]
fn test_mod_similarity_transform() {
    let test = |s, r, d, m, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let d = Natural::from_str(d).unwrap();
        let m = Natural::from_str(m).unwrap();
        assert_eq!(verify(&a, r, &d, &m).to_string(), out);
    };
    // - modulo 1
    test("[0, 0; 0, 0]", 0, "0", "1", "[0, 0; 0, 0]");
    // - d = 0
    test("[1, 2; 3, 4]", 0, "0", "5", "[1, 2; 3, 4]");
    // - n = 2 and r = 1: nothing to mix
    test("[5, 6; 7, 8]", 1, "4", "11", "[5, 6; 7, 8]");
    // - r = 0
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        0,
        "2",
        "11",
        "[1, 0, 6; 4, 2, 3; 7, 0, 2]",
    );
    // - r > 0: column r - 1 is left out
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        2,
        "5",
        "1000000000000000000000000000057",
        "[16, 2, 3; 34, 5, 6; 1000000000000000000000000000034, \
        1000000000000000000000000000055, 1000000000000000000000000000052]",
    );
    // - a large d
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        0,
        "123456789012345678901234567890",
        "1000000000000000000000000000057",
        "[641975320864197532086419753325, 176801747611645297120561041482, \
        806431380574608260416857337870; 4, 493827156049382715604938271565, \
        493827156049382715604938271566; 7, 864197523086419752308641975238, \
        864197523086419752308641975240]",
    );
}

#[test]
fn mod_similarity_transform_fail() {
    let five = Natural::from(5u32);
    // - not square
    let a = NaturalMatrix::from_str("[1, 2]").unwrap();
    assert_panic!(a.mod_similarity_transform(0, &Natural::ZERO, &five));
    // - r too large
    let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    assert_panic!(a.mod_similarity_transform(2, &Natural::ZERO, &five));
    // - the 0 by 0 matrix has no row index
    let a = NaturalMatrix::from_str("[]_{0,0}").unwrap();
    assert_panic!(a.mod_similarity_transform(0, &Natural::ZERO, &five));
    // - m = 0
    let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    assert_panic!(a.mod_similarity_transform(0, &Natural::ZERO, &Natural::ZERO));
    // - the matrix is not reduced
    let a = NaturalMatrix::from_str("[1, 2; 3, 9]").unwrap();
    assert_panic!(a.mod_similarity_transform(0, &Natural::ZERO, &five));
    // - d is not reduced
    let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    assert_panic!(a.mod_similarity_transform(0, &five, &five));
}

#[test]
#[should_panic]
fn mod_similarity_transform_assign_fail() {
    let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    a.mod_similarity_transform_assign(2, Natural::ZERO, Natural::from(5u32));
}

#[test]
fn mod_similarity_transform_properties() {
    natural_matrix_unsigned_natural_natural_quadruple_gen_var_1().test_properties(
        |(a, r, d, m)| {
            let b = verify(&a, r, &d, &m);
            // A similar matrix has the same invariants.
            assert_eq!((&b).mod_determinant(&m), (&a).mod_determinant(&m));
            assert_eq!(b.to_mod_trace(m.clone()), a.to_mod_trace(m.clone()));
            // The transform with -d undoes it.
            assert_eq!(b.mod_similarity_transform(r, (&d).mod_neg(&m), &m), a);
            // A transform with d = 0 does nothing.
            assert_eq!((&a).mod_similarity_transform(r, &Natural::ZERO, &m), a);
            // Conjugation respects products.
            assert_eq!(
                (&a).mod_square(&m).mod_similarity_transform(r, &d, &m),
                (&a).mod_similarity_transform(r, &d, &m).mod_square(&m)
            );
        },
    );

    unsigned_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_1::<u64>().test_properties(
        |(a, r, d, m)| {
            // The transform agrees with the transform of the UnsignedMatrix, and with the transform
            // over the integers reduced modulo m.
            let b = (&a).mod_similarity_transform(r, d, m);
            assert_eq!(
                NaturalMatrix::from(&a).mod_similarity_transform(
                    r,
                    Natural::from(d),
                    Natural::from(m)
                ),
                NaturalMatrix::from(&b)
            );
            assert_eq!(
                IntegerMatrix::from(&a)
                    .similarity_transform(r, Integer::from(d))
                    .mod_op(m),
                b
            );
        },
    );
}
