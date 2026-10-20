// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{Parity, Pow, PowAssign, Square};
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::generators::{
    integer_matrix_gen_var_1, integer_matrix_unsigned_pair_gen_var_5,
};
use malachite_nz::test_util::integer_matrix::arithmetic::pow::integer_matrix_pow_naive;
use malachite_nz::test_util::integer_matrix::pseudorandom::pseudorandom_integer_matrix;

#[test]
fn test_pow() {
    let test = |s, exp: u64, out| {
        let a = IntegerMatrix::from_str(s).unwrap();

        let b = a.clone().pow(exp);
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);

        let b = (&a).pow(exp);
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);

        let mut b = a.clone();
        b.pow_assign(exp);
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);

        assert_eq!(integer_matrix_pow_naive(&a, exp).to_string(), out);
    };
    test("[1, 1; 1, 0]", 10, "[89, 55; 55, 34]");
    test("[2]", 10, "[1024]");
    test("[-1]", 7, "[-1]");
    test("[]_{0,0}", 0, "[]_{0,0}");
    test("[]_{0,0}", 3, "[]_{0,0}");
    test("[1, 2; 3, 4]", 0, "[1, 0; 0, 1]");
    test("[1, 2; 3, 4]", 1, "[1, 2; 3, 4]");
    test("[1, 2; 3, 4]", 2, "[7, 10; 15, 22]");
    test("[1, 2; 3, 4]", 3, "[37, 54; 81, 118]");
    test("[1, -2; 3, 4]", 5, "[-275, 50; -75, -350]");
    test(
        "[0, 1, 0; 0, 0, 1; 0, 0, 0]",
        2,
        "[0, 0, 1; 0, 0, 0; 0, 0, 0]",
    );
    test(
        "[0, 1, 0; 0, 0, 1; 0, 0, 0]",
        3,
        "[0, 0, 0; 0, 0, 0; 0, 0, 0]",
    );
    test(
        "[0, 1, 0; 0, 0, 1; 0, 0, 0]",
        12,
        "[0, 0, 0; 0, 0, 0; 0, 0, 0]",
    );
    test("[0, 0; 0, 0]", 5, "[0, 0; 0, 0]");
    test(
        "[1, 2, 0; 0, 1, 2; 2, 0, 1]",
        6,
        "[225, 252, 252; 252, 225, 252; 252, 252, 225]",
    );
    test(
        "[18446744073709551616, 1; -1, 18446744073709551616]",
        3,
        "[6277101735386680763835789423207666416047015212242905858048, 10208471007628153903901238222\
        95304634367; -1020847100762815390390123822295304634367, 62771017353866807638357894232076664\
        16047015212242905858048]",
    );
}

#[test]
fn test_pow_kernels() {
    // Exponents up to 7 cover each bit pattern of the binary powering loop up to three bits.
    let check = |a: &IntegerMatrix, exp: u64| {
        let expected = integer_matrix_pow_naive(a, exp);
        assert_eq!(a.pow(exp), expected);
        let mut b = a.clone();
        b.pow_assign(exp);
        assert_eq!(b, expected);
    };
    for signed in [false, true] {
        for bits in [10, 62, 100, 200] {
            for n in 0..6 {
                for exp in 0..8 {
                    check(&pseudorandom_integer_matrix(n, n, bits, signed, exp), exp);
                }
            }
        }
    }
}

#[test]
#[should_panic]
fn pow_fail() {
    IntegerMatrix::from_str("[1, 2]").unwrap().pow(2);
}

#[test]
#[should_panic]
fn pow_ref_fail() {
    (&IntegerMatrix::from_str("[]_{0,2}").unwrap()).pow(0);
}

#[test]
#[should_panic]
fn pow_assign_fail() {
    let mut a = IntegerMatrix::from_str("[1; 2]").unwrap();
    a.pow_assign(3);
}

#[test]
fn pow_properties() {
    integer_matrix_unsigned_pair_gen_var_5().test_properties(|(a, exp)| {
        let b = (&a).pow(exp);
        assert!(b.is_valid());
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        assert_eq!(a.clone().pow(exp), b);
        let mut c = a.clone();
        c.pow_assign(exp);
        assert_eq!(c, b);
        assert_eq!(integer_matrix_pow_naive(&a, exp), b);
        // One more factor multiplies the power by the matrix, on either side.
        let c = (&a).pow(exp + 1);
        assert_eq!(&b * &a, c);
        assert_eq!(&a * &b, c);
        // The power of the transpose is the transpose of the power.
        assert_eq!(a.transpose().pow(exp), b.transpose());
        // Negating a matrix negates its odd powers and leaves its even powers unchanged.
        let c = (-&a).pow(exp);
        if exp.even() {
            assert_eq!(c, b);
        } else {
            assert_eq!(c, -&b);
        }
    });

    integer_matrix_gen_var_1().test_properties(|a| {
        assert_eq!((&a).pow(0), IntegerMatrix::identity(a.height()));
        assert_eq!((&a).pow(1), a);
        assert_eq!((&a).pow(2), (&a).square());
        assert_eq!((&a).pow(3), &a * &a * &a);
    });
}
