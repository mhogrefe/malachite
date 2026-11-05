// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::linear_algebra::traits::ModGramSchmidt;
use malachite_base::num::modular::traits::ModMul;
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_pair_gen_var_5, unsigned_matrix_unsigned_pair_gen_var_9,
};
use malachite_base::test_util::unsigned_matrix::linear_algebra::gram_schmidt::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Checks the Gram-Schmidt process applied to `a` modulo `m` in every way of calling it, against the
// modified algorithm; checks that the columns of the result are orthogonal, and that
// orthogonalizing them again changes nothing; and returns the result.
fn verify<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, m: T) -> Option<UnsignedMatrix<T>> {
    let b = a.mod_gram_schmidt(m);
    assert_eq!(a.clone().mod_gram_schmidt(m), b);
    assert_eq!(mod_gram_schmidt_modified(a, m), b);
    if let Some(b) = &b {
        assert!(b.is_valid());
        assert_eq!(b.height(), a.height());
        assert_eq!(b.width(), a.width());
        let gram = (&b.transpose()).mod_mul(b, m);
        let width = usize::exact_from(b.width());
        for i in 0..width {
            for j in 0..width {
                if i != j {
                    assert_eq!(gram[(i, j)], T::ZERO);
                }
            }
        }
        assert_eq!(b.mod_gram_schmidt(m).as_ref(), Some(b));
    }
    b
}

fn test_mod_gram_schmidt_helper<T: PrimitiveUnsigned>() {
    let test = |a, m: u8, out: Option<&str>| {
        let a = UnsignedMatrix::<T>::from_str(a).unwrap();
        let b = verify(&a, T::from(m)).map(|b| b.to_string());
        assert_eq!(b.as_deref(), out);
    };
    // - height == 0
    test("[]_{0,0}", 5, Some("[]_{0,0}"));
    test("[]_{0,3}", 5, Some("[]_{0,3}"));
    test("[]_{2,0}", 5, Some("[]_{2,0}"));
    // - ring.is_zero(&den) && column j is zero
    test("[0, 0; 0, 0]", 1, Some("[0, 0; 0, 0]"));
    // - !ring.is_zero(&den) && ring.inverse(&den).is_some()
    test("[1, 1; 1, 0]", 5, Some("[1, 3; 1, 2]"));
    // - ring.is_zero(&den) && column j is nonzero
    test("[1, 1; 1, 0]", 2, None);
    test("[1; 1]", 2, Some("[1; 1]"));
    test("[0, 1; 0, 1]", 5, Some("[0, 1; 0, 1]"));
    // - !ring.is_zero(&den) && ring.inverse(&den).is_none()
    test("[1, 0; 1, 1]", 6, None);
    test("[1, 1; 0, 1]", 6, Some("[1, 0; 0, 1]"));
    test("[1, 0, 1; 0, 1, 1]", 7, Some("[1, 0, 0; 0, 1, 0]"));
    test(
        "[1, 2, 3; 2, 4, 1; 0, 0, 5]",
        7,
        Some("[1, 0, 2; 2, 0, 6; 0, 0, 5]"),
    );
    test("[3, 1; 4, 1]", 5, None);
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", 11, None);
    test("[2, 3; 5, 7; 11, 13]", 17, Some("[2, 1; 5, 2; 11, 2]"));
}

#[test]
fn test_mod_gram_schmidt() {
    apply_fn_to_unsigneds!(test_mod_gram_schmidt_helper);
}

#[test]
#[should_panic]
fn mod_gram_schmidt_fail_1() {
    UnsignedMatrix::<u8>::from_str("[1, 7]")
        .unwrap()
        .mod_gram_schmidt(7);
}

#[test]
#[should_panic]
fn mod_gram_schmidt_fail_2() {
    UnsignedMatrix::<u8>::from_str("[1, 2]")
        .unwrap()
        .mod_gram_schmidt(0);
}

#[test]
fn mod_gram_schmidt_properties() {
    unsigned_matrix_unsigned_pair_gen_var_5().test_properties(|(a, m)| {
        verify(&a, m);
    });

    unsigned_matrix_unsigned_pair_gen_var_9().test_properties(|(a, m)| {
        verify(&a, m);
    });
}
