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
use malachite_base::num::linear_algebra::traits::ModPowerOf2GramSchmidt;
use malachite_base::num::modular::traits::ModPowerOf2Mul;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen_var_4;
use malachite_nz::test_util::natural_matrix::linear_algebra::gram_schmidt::*;

// Checks the Gram-Schmidt process applied to `a` modulo $2^k$ in every way of calling it, against
// the modified algorithm and, when $2^k$ fits in a `u64`, against `UnsignedMatrix`; checks that the
// columns of the result are orthogonal, and that orthogonalizing them again changes nothing; and
// returns the result.
fn verify(a: &NaturalMatrix, pow: u64) -> Option<NaturalMatrix> {
    let b = a.mod_power_of_2_gram_schmidt(pow);
    assert_eq!(a.clone().mod_power_of_2_gram_schmidt(pow), b);
    assert_eq!(mod_power_of_2_gram_schmidt_modified(a, pow), b);
    if pow <= 64 {
        let a_u64 = UnsignedMatrix::<u64>::from_str(&a.to_string()).unwrap();
        assert_eq!(
            a_u64
                .mod_power_of_2_gram_schmidt(pow)
                .map(|b| b.to_string()),
            b.as_ref().map(ToString::to_string)
        );
    }
    if let Some(b) = &b {
        assert!(b.is_valid());
        let gram = (&b.transpose()).mod_power_of_2_mul(b, pow);
        let width = usize::exact_from(b.width());
        for i in 0..width {
            for j in 0..width {
                if i != j {
                    assert_eq!(gram[(i, j)], 0u32);
                }
            }
        }
        assert_eq!(b.mod_power_of_2_gram_schmidt(pow).as_ref(), Some(b));
    }
    b
}

#[test]
fn test_mod_power_of_2_gram_schmidt() {
    let test = |a, pow: u64, out: Option<&str>| {
        let a = NaturalMatrix::from_str(a).unwrap();
        let b = verify(&a, pow).map(|b| b.to_string());
        assert_eq!(b.as_deref(), out);
    };
    // - height == 0
    test("[]_{0,0}", 3, Some("[]_{0,0}"));
    test("[]_{2,0}", 3, Some("[]_{2,0}"));
    // - ring.is_zero(&den) && column j is zero
    test("[0, 0; 0, 0]", 0, Some("[0, 0; 0, 0]"));
    // - !ring.is_zero(&den) && ring.inverse(&den).is_none()
    test("[1, 1; 1, 0]", 3, None);
    // - !ring.is_zero(&den) && ring.inverse(&den).is_some()
    test("[1, 1; 0, 1]", 3, Some("[1, 0; 0, 1]"));
    // - ring.is_zero(&den) && column j is nonzero
    test("[2, 1; 2, 0]", 3, None);
    test("[1, 0; 2, 1]", 3, Some("[1, 6; 2, 5]"));
    test("[0, 1; 0, 1]", 3, Some("[0, 1; 0, 1]"));
    test("[2; 2]", 3, Some("[2; 2]"));
    test("[3, 5, 7; 1, 2, 4]", 4, None);
    test("[255, 1; 0, 1]", 8, Some("[255, 0; 0, 1]"));
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", 5, None);
    test("[1, 1; 0, 1]", 200, Some("[1, 0; 0, 1]"));
    test("[1, 1; 1, 0]", 200, None);
    test(
        "[1, 0; 2, 1]",
        200,
        Some(
            "[1, 642775217703596110216784836936465041008881197513117134120550; 2, \
            1285550435407192220433569673872930082017762395026234268241101]",
        ),
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_gram_schmidt_fail() {
    NaturalMatrix::from_str("[1, 8]")
        .unwrap()
        .mod_power_of_2_gram_schmidt(3);
}

#[test]
fn mod_power_of_2_gram_schmidt_properties() {
    natural_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        verify(&a, pow);
    });
}
