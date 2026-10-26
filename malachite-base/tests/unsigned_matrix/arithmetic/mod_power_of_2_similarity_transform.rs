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
    ModPowerOf2Determinant, ModPowerOf2Neg, ModPowerOf2SimilarityTransform,
    ModPowerOf2SimilarityTransformAssign, ModPowerOf2Square, ModPowerOf2Trace,
    ModSimilarityTransform,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_matrix::arithmetic::similarity_transform::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_matrix::arithmetic::permanent_over_ring::ModPowerOf2Ring;
use std::panic::catch_unwind;

// Checks every way of calling the transform, FLINT's loops, and the general-modulus transform, and
// returns the result.
fn verify<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    r: u64,
    d: T,
    pow: u64,
) -> UnsignedMatrix<T> {
    let b = a.mod_power_of_2_similarity_transform(r, d, pow);
    assert_eq!(a.clone().mod_power_of_2_similarity_transform(r, d, pow), b);
    let mut c = a.clone();
    c.mod_power_of_2_similarity_transform_assign(r, d, pow);
    assert_eq!(c, b);
    let n = usize::exact_from(a.height());
    assert_eq!(
        matrix_similarity_transform_flint_loops(
            &ModPowerOf2Ring::new(pow),
            a.elements_ref(),
            n,
            usize::exact_from(r),
            &d
        ),
        b.elements_ref()
    );
    if pow < T::WIDTH {
        assert_eq!(a.mod_similarity_transform(r, d, T::power_of_2(pow)), b);
    }
    b
}

#[test]
fn test_mod_power_of_2_similarity_transform() {
    fn test<T: PrimitiveUnsigned>(s: &str, r: u64, d: T, pow: u64, out: &str) {
        let a = UnsignedMatrix::<T>::from_str(s).unwrap();
        assert_eq!(verify(&a, r, d, pow).to_string(), out);
    }
    // - modulo 1
    test::<u8>("[0]", 0, 0, 0, "[0]");
    // - d = 0
    test::<u8>("[1, 2; 3, 4]", 0, 0, 3, "[1, 2; 3, 4]");
    // - n = 2 and r = 1: nothing to mix
    test::<u8>("[5, 6; 7, 8]", 1, 4, 4, "[5, 6; 7, 8]");
    // - r = 0
    test::<u8>(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        0,
        2,
        4,
        "[11, 14, 9; 4, 13, 14; 7, 6, 8]",
    );
    // - r > 0: column r - 1 is left out
    test::<u8>(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        2,
        5,
        4,
        "[0, 2, 3; 2, 5, 6; 9, 14, 11]",
    );
    // - the full width of the type
    test::<u64>(
        "[18446744073709551556, 18446744073709551555, 3; 4, 18446744073709551552, 6; \
        7, 8, 18446744073709551547]",
        1,
        18446744073709551615,
        64,
        "[18446744073709551556, 18446744073709551555, 64; 11, 18446744073709551560, \
        18446744073709551609; 7, 8, 18446744073709551539]",
    );
}

#[test]
fn mod_power_of_2_similarity_transform_fail() {
    // - not square
    let a = UnsignedMatrix::<u8>::from_str("[1, 2]").unwrap();
    assert_panic!(a.mod_power_of_2_similarity_transform(0, 0, 3));
    // - r too large
    let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    assert_panic!(a.mod_power_of_2_similarity_transform(2, 0, 3));
    // - pow too large
    let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    assert_panic!(a.mod_power_of_2_similarity_transform(0, 0, 9));
    // - the matrix is not reduced
    let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 9]").unwrap();
    assert_panic!(a.mod_power_of_2_similarity_transform(0, 0, 3));
    // - d is not reduced
    let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    assert_panic!(a.mod_power_of_2_similarity_transform(0, 8, 3));
}

#[test]
#[should_panic]
fn mod_power_of_2_similarity_transform_assign_fail() {
    let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    a.mod_power_of_2_similarity_transform_assign(2, 0, 3);
}

#[test]
fn mod_power_of_2_similarity_transform_properties() {
    unsigned_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_2::<u8>().test_properties(
        |(a, r, d, pow)| {
            verify(&a, r, d, pow);
        },
    );

    unsigned_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_2::<u64>().test_properties(
        |(a, r, d, pow)| {
            let b = verify(&a, r, d, pow);
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
                b.mod_power_of_2_similarity_transform(r, d.mod_power_of_2_neg(pow), pow),
                a
            );
            // A transform with d = 0 does nothing.
            assert_eq!((&a).mod_power_of_2_similarity_transform(r, 0, pow), a);
            // Conjugation respects products.
            assert_eq!(
                (&a).mod_power_of_2_square(pow)
                    .mod_power_of_2_similarity_transform(r, d, pow),
                (&a).mod_power_of_2_similarity_transform(r, d, pow)
                    .mod_power_of_2_square(pow)
            );
        },
    );
}
