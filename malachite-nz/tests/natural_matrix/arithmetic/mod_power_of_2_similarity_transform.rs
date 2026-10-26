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
    ModPowerOf2, ModPowerOf2Determinant, ModPowerOf2Neg, ModPowerOf2SimilarityTransform,
    ModPowerOf2SimilarityTransformAssign, ModPowerOf2Square, ModPowerOf2Trace,
    ModSimilarityTransform, PowerOf2, SimilarityTransform,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_matrix::arithmetic::similarity_transform::*;
use malachite_base::unsigned_matrix::arithmetic::permanent_over_ring::ModPowerOf2Ring;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::*;
use std::panic::catch_unwind;

// Checks every way of calling the transform, FLINT's loops, the general-modulus transform, and the
// transform over the integers reduced modulo 2^pow, and returns the result.
fn verify(a: &NaturalMatrix, r: u64, d: &Natural, pow: u64) -> NaturalMatrix {
    let b = a.mod_power_of_2_similarity_transform(r, d, pow);
    assert!(b.is_valid());
    assert_eq!(a.clone().mod_power_of_2_similarity_transform(r, d, pow), b);
    assert_eq!(
        a.clone()
            .mod_power_of_2_similarity_transform(r, d.clone(), pow),
        b
    );
    let mut c = a.clone();
    c.mod_power_of_2_similarity_transform_assign(r, d, pow);
    assert_eq!(c, b);
    let mut c = a.clone();
    c.mod_power_of_2_similarity_transform_assign(r, d.clone(), pow);
    assert_eq!(c, b);
    let n = usize::exact_from(a.height());
    assert_eq!(
        matrix_similarity_transform_flint_loops(
            &ModPowerOf2Ring::new(pow),
            a.elements_ref(),
            n,
            usize::exact_from(r),
            d
        ),
        b.elements_ref()
    );
    assert_eq!(
        a.mod_similarity_transform(r, d, &Natural::power_of_2(pow)),
        b
    );
    // The transform commutes with reduction modulo 2^pow.
    assert_eq!(
        IntegerMatrix::from(a)
            .similarity_transform(r, Integer::from(d))
            .mod_power_of_2(pow),
        b
    );
    b
}

#[test]
fn test_mod_power_of_2_similarity_transform() {
    let test = |s, r, d, pow, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let d = Natural::from_str(d).unwrap();
        assert_eq!(verify(&a, r, &d, pow).to_string(), out);
    };
    // - modulo 1
    test("[0]", 0, "0", 0, "[0]");
    // - d = 0
    test("[1, 2; 3, 4]", 0, "0", 3, "[1, 2; 3, 4]");
    // - n = 2 and r = 1: nothing to mix
    test("[5, 6; 7, 8]", 1, "4", 100, "[5, 6; 7, 8]");
    // - r = 0
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        0,
        "2",
        4,
        "[11, 14, 9; 4, 13, 14; 7, 6, 8]",
    );
    // - r > 0: column r - 1 is left out
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        2,
        "5",
        100,
        "[16, 2, 3; 34, 5, 6; 1267650600228229401496703205353, \
        1267650600228229401496703205374, 1267650600228229401496703205371]",
    );
    // - a large d
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        0,
        "123456789012345678901234567890",
        100,
        "[1177276521320656335079826163963, 366160831762325405658218733374, \
        1263441064953517770451218235081; 4, 493827156049382715604938271565, \
        493827156049382715604938271566; 7, 864197523086419752308641975238, \
        864197523086419752308641975240]",
    );
}

#[test]
fn mod_power_of_2_similarity_transform_fail() {
    // - not square
    let a = NaturalMatrix::from_str("[1, 2]").unwrap();
    assert_panic!(a.mod_power_of_2_similarity_transform(0, &Natural::ZERO, 3));
    // - r too large
    let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    assert_panic!(a.mod_power_of_2_similarity_transform(2, &Natural::ZERO, 3));
    // - the matrix is not reduced
    let a = NaturalMatrix::from_str("[1, 2; 3, 9]").unwrap();
    assert_panic!(a.mod_power_of_2_similarity_transform(0, &Natural::ZERO, 3));
    // - d is not reduced
    let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    assert_panic!(a.mod_power_of_2_similarity_transform(0, &Natural::from(8u32), 3));
}

#[test]
#[should_panic]
fn mod_power_of_2_similarity_transform_assign_fail() {
    let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    a.mod_power_of_2_similarity_transform_assign(2, Natural::ZERO, 3);
}

#[test]
fn mod_power_of_2_similarity_transform_properties() {
    natural_matrix_unsigned_natural_unsigned_quadruple_gen_var_1().test_properties(
        |(a, r, d, pow)| {
            let b = verify(&a, r, &d, pow);
            // A similar matrix has the same invariants.
            assert_eq!(
                (&b).mod_power_of_2_determinant(pow),
                (&a).mod_power_of_2_determinant(pow)
            );
            assert_eq!(
                b.to_mod_power_of_2_trace(pow),
                a.to_mod_power_of_2_trace(pow)
            );
            // The transform with -d undoes it.
            assert_eq!(
                b.mod_power_of_2_similarity_transform(r, (&d).mod_power_of_2_neg(pow), pow),
                a
            );
            // A transform with d = 0 does nothing.
            assert_eq!(
                (&a).mod_power_of_2_similarity_transform(r, &Natural::ZERO, pow),
                a
            );
            // Conjugation respects products.
            assert_eq!(
                (&a).mod_power_of_2_square(pow)
                    .mod_power_of_2_similarity_transform(r, &d, pow),
                (&a).mod_power_of_2_similarity_transform(r, &d, pow)
                    .mod_power_of_2_square(pow)
            );
        },
    );

    unsigned_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_2::<u64>().test_properties(
        |(a, r, d, pow)| {
            // The transform agrees with the transform of the UnsignedMatrix.
            assert_eq!(
                NaturalMatrix::from(&a).mod_power_of_2_similarity_transform(
                    r,
                    Natural::from(d),
                    pow
                ),
                NaturalMatrix::from(a.mod_power_of_2_similarity_transform(r, d, pow))
            );
        },
    );
}
