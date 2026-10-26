// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_pair_gen_var_11, unsigned_matrix_unsigned_pair_gen_var_12,
};
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_matrix::arithmetic::mod_characteristic_polynomial::*;
use malachite_base::unsigned_matrix::arithmetic::mod_determinant::charpoly_berkowitz;

// Checks Danilevsky's method, which must succeed for a prime modulus, against Berkowitz's
// algorithm, and returns the polynomial.
fn verify_prime(a: &UnsignedMatrix<u64>, m: u64) -> Vec<u64> {
    let n = usize::exact_from(a.height());
    let mut cp = vec![0; n + 1];
    charpoly_berkowitz(&mut cp, a.elements_ref(), n, m);
    assert_eq!(charpoly_danilevsky(a.elements_ref(), n, m).unwrap(), cp);
    assert_eq!(mod_charpoly(a.elements_ref(), n, m), cp);
    cp
}

#[test]
fn test_charpoly_danilevsky() {
    let test = |s, m, out: &[u64]| {
        let a = UnsignedMatrix::<u64>::from_str(s).unwrap();
        assert_eq!(verify_prime(&a, m), out);
    };
    // - dimension 0
    test("[]_{0,0}", 251, &[1]);
    // - dimension 1
    test("[5]", 251, &[246, 1]);
    // - a dense matrix
    test(
        "[3, 1, 4, 1; 5, 9, 2, 6; 5, 3, 5, 8; 9, 7, 9, 3]",
        251,
        &[98, 229, 235, 231, 1],
    );
    // - a diagonal matrix
    test(
        "[2, 0, 0, 0; 0, 3, 0, 0; 0, 0, 5, 0; 0, 0, 0, 7]",
        251,
        &[210, 4, 101, 234, 1],
    );
    // - a zero pivot with a nonzero element to its left
    test(
        "[1, 2, 3, 4; 5, 6, 7, 8; 9, 10, 11, 12; 13, 14, 0, 15]",
        251,
        &[0, 205, 82, 218, 1],
    );
    // - a block triangular matrix
    test(
        "[1, 2, 3, 4; 5, 6, 7, 8; 0, 0, 9, 10; 0, 0, 11, 12]",
        251,
        &[8, 98, 141, 223, 1],
    );
    // - modulo 2
    test("[1, 1, 0; 0, 1, 1; 1, 0, 1]", 2, &[0, 1, 1, 1]);
}

#[test]
fn test_mod_charpoly() {
    // - above dimension 8, with a prime modulus: Danilevsky's method
    let a = UnsignedMatrix::<u64>::from_str(
        "[1, 6, 4, 2, 0, 5, 3, 1, 6; 4, 2, 0, 5, 3, 1, 6, 4, 2; 0, 5, 3, 1, 6, 4, 2, 0, 5; \
        3, 1, 6, 4, 2, 0, 5, 3, 1; 6, 4, 2, 0, 5, 3, 1, 6, 4; 2, 0, 5, 3, 1, 6, 4, 2, 0; \
        5, 3, 1, 6, 4, 2, 0, 5, 3; 1, 6, 4, 2, 0, 5, 3, 1, 6; 4, 2, 0, 5, 3, 1, 6, 4, 2]",
    )
    .unwrap();
    assert_eq!(
        verify_prime(&a, 251),
        &[0, 0, 164, 106, 60, 161, 236, 173, 227, 1]
    );
    // - a pivot that is not invertible: Danilevsky's method fails
    let a = UnsignedMatrix::<u64>::from_str("[1, 2; 2, 3]").unwrap();
    assert_eq!(charpoly_danilevsky(a.elements_ref(), 2, 6), None);
    assert_eq!(mod_charpoly(a.elements_ref(), 2, 6), &[5, 2, 1]);
    // - above dimension 8, with a pivot that is not invertible: Berkowitz's algorithm instead
    let a = UnsignedMatrix::<u64>::from_str(
        "[1, 0, 4, 2, 0, 5, 3, 1, 0; 4, 2, 0, 5, 3, 1, 0, 4, 2; 0, 5, 3, 1, 0, 4, 2, 0, 5; \
        3, 1, 0, 4, 2, 0, 5, 3, 1; 0, 4, 2, 0, 5, 3, 1, 0, 4; 2, 0, 5, 3, 1, 0, 4, 2, 0; \
        5, 3, 1, 0, 4, 2, 0, 5, 3; 1, 0, 4, 2, 0, 5, 3, 1, 0; 4, 2, 0, 5, 3, 1, 0, 2, 2]",
    )
    .unwrap();
    assert_eq!(charpoly_danilevsky(a.elements_ref(), 9, 6), None);
    assert_eq!(
        mod_charpoly(a.elements_ref(), 9, 6),
        &[0, 0, 4, 0, 1, 1, 0, 0, 0, 1]
    );
}

#[test]
fn mod_charpoly_properties() {
    unsigned_matrix_unsigned_pair_gen_var_12().test_properties(|(a, m)| {
        verify_prime(&a, m);
    });

    unsigned_matrix_unsigned_pair_gen_var_11().test_properties(|(a, m)| {
        // For any modulus, Danilevsky's method, when it succeeds, agrees with Berkowitz's
        // algorithm.
        let n = usize::exact_from(a.height());
        let mut cp = vec![0; n + 1];
        charpoly_berkowitz(&mut cp, a.elements_ref(), n, m);
        if m != 1
            && let Some(p) = charpoly_danilevsky(a.elements_ref(), n, m)
        {
            assert_eq!(p, cp);
        }
        assert_eq!(mod_charpoly(a.elements_ref(), n, m), cp);
    });
}
