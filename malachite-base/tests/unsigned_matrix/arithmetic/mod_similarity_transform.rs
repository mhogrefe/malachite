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
    ModDeterminant, ModNeg, ModPowerOf2SimilarityTransform, ModSimilarityTransform,
    ModSimilarityTransformAssign, ModSquare, ModTrace,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_matrix::arithmetic::similarity_transform::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_matrix::arithmetic::permanent_over_ring::ModRing;
use std::panic::catch_unwind;

// Checks every way of calling the transform, and FLINT's loops, and returns the result.
fn verify<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, r: u64, d: T, m: T) -> UnsignedMatrix<T> {
    let b = a.mod_similarity_transform(r, d, m);
    assert_eq!(a.clone().mod_similarity_transform(r, d, m), b);
    let mut c = a.clone();
    c.mod_similarity_transform_assign(r, d, m);
    assert_eq!(c, b);
    let n = usize::exact_from(a.height());
    assert_eq!(
        matrix_similarity_transform_flint_loops(
            &ModRing::new(m),
            a.elements_ref(),
            n,
            usize::exact_from(r),
            &d
        ),
        b.elements_ref()
    );
    if m.is_power_of_2() {
        assert_eq!(
            a.mod_power_of_2_similarity_transform(r, d, m.trailing_zeros()),
            b
        );
    }
    b
}

#[test]
fn test_mod_similarity_transform() {
    fn test<T: PrimitiveUnsigned>(s: &str, r: u64, d: T, m: T, out: &str) {
        let a = UnsignedMatrix::<T>::from_str(s).unwrap();
        assert_eq!(verify(&a, r, d, m).to_string(), out);
    }
    // - modulo 1
    test::<u8>("[0, 0; 0, 0]", 0, 0, 1, "[0, 0; 0, 0]");
    // - d = 0
    test::<u8>("[1, 2; 3, 4]", 0, 0, 5, "[1, 2; 3, 4]");
    // - n = 1
    test::<u8>("[3]", 0, 4, 5, "[3]");
    // - n = 2 and r = 1: nothing to mix
    test::<u8>("[5, 6; 7, 8]", 1, 4, 11, "[5, 6; 7, 8]");
    // - r = 0
    test::<u8>(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        0,
        2,
        11,
        "[1, 0, 6; 4, 2, 3; 7, 0, 2]",
    );
    // - r > 0: column r - 1 is left out
    test::<u8>(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        2,
        5,
        11,
        "[5, 2, 3; 1, 5, 6; 10, 9, 6]",
    );
    // - a power of 2 modulus
    test::<u8>(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        0,
        2,
        16,
        "[11, 14, 9; 4, 13, 14; 7, 6, 8]",
    );
    // - a large prime modulus
    test::<u64>(
        "[18446744073709551556, 18446744073709551555, 3; 4, 18446744073709551552, 6; \
        7, 8, 18446744073709551547]",
        0,
        9223372036854775808,
        18446744073709551557,
        "[9223372036854775453, 13835058055282153975, 4611686018427378408; 4, 113, 124; \
        7, 9223372036854775993, 9223372036854775975]",
    );
}

#[test]
fn mod_similarity_transform_fail() {
    // - not square
    let a = UnsignedMatrix::<u8>::from_str("[1, 2]").unwrap();
    assert_panic!(a.mod_similarity_transform(0, 0, 5));
    // - r too large
    let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    assert_panic!(a.mod_similarity_transform(2, 0, 5));
    // - the 0 by 0 matrix has no row index
    let a = UnsignedMatrix::<u8>::from_str("[]_{0,0}").unwrap();
    assert_panic!(a.mod_similarity_transform(0, 0, 5));
    // - m = 0
    let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    assert_panic!(a.mod_similarity_transform(0, 0, 0));
    // - the matrix is not reduced
    let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 9]").unwrap();
    assert_panic!(a.mod_similarity_transform(0, 0, 5));
    // - d is not reduced
    let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    assert_panic!(a.mod_similarity_transform(0, 5, 5));
}

#[test]
#[should_panic]
fn mod_similarity_transform_assign_fail() {
    let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    a.mod_similarity_transform_assign(2, 0, 5);
}

#[test]
fn mod_similarity_transform_properties() {
    unsigned_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_1::<u8>().test_properties(
        |(a, r, d, m)| {
            verify(&a, r, d, m);
        },
    );

    unsigned_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_1::<u64>().test_properties(
        |(a, r, d, m)| {
            let b = verify(&a, r, d, m);
            // A similar matrix has the same invariants.
            assert_eq!((&b).mod_determinant(m), (&a).mod_determinant(m));
            assert_eq!(b.to_mod_trace(m), a.to_mod_trace(m));
            // The transform with -d undoes it.
            assert_eq!(b.mod_similarity_transform(r, d.mod_neg(m), m), a);
            // A transform with d = 0 does nothing.
            assert_eq!((&a).mod_similarity_transform(r, 0, m), a);
            // Conjugation respects products.
            assert_eq!(
                (&a).mod_square(m).mod_similarity_transform(r, d, m),
                (&a).mod_similarity_transform(r, d, m).mod_square(m)
            );
        },
    );
}
