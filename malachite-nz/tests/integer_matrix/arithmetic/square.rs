// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::square::bodrato::matrix_square_bodrato;
use malachite_nz::test_util::generators::integer_matrix_gen_var_1;
use malachite_nz::test_util::integer_matrix::arithmetic::square::{
    integer_matrix_square_naive, integer_matrix_square_with,
};
use malachite_nz::test_util::integer_matrix::pseudorandom::pseudorandom_integer_matrix;

#[test]
fn test_square() {
    let test = |s, out| {
        let a = IntegerMatrix::from_str(s).unwrap();

        let b = a.clone().square();
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);

        let b = (&a).square();
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);

        let mut b = a.clone();
        b.square_assign();
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);

        assert_eq!(integer_matrix_square_naive(&a).to_string(), out);
        assert_eq!(
            integer_matrix_square_with(&a, matrix_square_bodrato).to_string(),
            out
        );
    };
    test("[]_{0,0}", "[]_{0,0}");
    test("[5]", "[25]");
    test("[-3]", "[9]");
    test("[1, 2; 3, 4]", "[7, 10; 15, 22]");
    test("[1, -2; 3, 4]", "[-5, -10; 15, 10]");
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, -9]",
        "[30, 36, -12; 66, 81, -12; -24, -18, 150]",
    );
    test(
        "[18446744073709551615, 18446744073709551616, -18446744073709551616; 1, -1, 118059162071741\
        1303424; 3, 36893488147419103239, -5]",
        "[340282366920938463389587631136930004993, -340282366920938463629395304095154176000, 214377\
        89116019123198303280732643654631424; 3560221606225943461886, 435561429658801233315945378403\
        61919741953, -7101996468378177372160; 92233720368547758069, -166020696663385964586, 4355614\
        2965880123331520750864067081535513]",
    );
    test(
        "[1361129467683753853853498429727072845824, -1, 2; 3, 2722258935367507707706996859454145691\
        649, -4; 5, 6, -680564733841876926926749214863536422912]",
        "[1852673427797059126777135760139006525652319754650249024631321344126610074238983, -4083388\
        403051261561560495289181218537461, 1361129467683753853853498429727072845828; 12250165209153\
        784684681485867543655612399, 74106937111882365071085430405560261026147235364717311139406993\
        70225348588339174, -8166776806102523123120990578362437074942; 34028236692093846346337460743\
        17682114578, 12250165209153784684681485867543655612417, 46316835694926478169428394003475163\
        1413079938662562256157830336031652518559730]",
    );
    test(
        "[4611686018427387904, 1; -1, 3]",
        "[21267647932558653966460912964485513215, 4611686018427387907; -4611686018427387907, 8]",
    );
    test(
        "[1, 2, 3, 4; 5, 6, 7, 8; 9, 10, 11, 12; 13, 14, 15, -16]",
        "[90, 100, 110, -8; 202, 228, 254, 24; 314, 356, 398, 56; 10, 36, 62, 600]",
    );
    test(
        "[1, 0, 2, 0, 3; 0, 4, 0, 5, 0; 6, 0, 7, 0, 8; 0, 9, 0, 10, 0; 11, 0, 12, 0, -13]",
        "[46, 0, 52, 0, -20; 0, 61, 0, 70, 0; 136, 0, 157, 0, -30; 0, 126, 0, 145, 0; -60, 0, -50, \
        0, 298]",
    );
}

#[test]
fn test_square_kernels() {
    // Dimensions up to 9, which cover each explicit formula, both halves of Bodrato's algorithm
    // with and without an odd last row and column, and squaring through multiplication; and
    // elements that fit in small `fmpz`s, in double words, and in neither.
    let check = |a: &IntegerMatrix| {
        let expected = integer_matrix_square_naive(a);
        assert_eq!(a.square(), expected);
        let mut b = a.clone();
        b.square_assign();
        assert_eq!(b, expected);
        assert_eq!(
            integer_matrix_square_with(a, matrix_square_bodrato),
            expected
        );
    };
    for signed in [false, true] {
        for bits in [10, 30, 62, 63, 100, 127, 128, 200, 1100] {
            for n in 0..10 {
                for seed in 0..2 {
                    check(&pseudorandom_integer_matrix(n, n, bits, signed, seed));
                }
            }
        }
    }
}

#[test]
#[should_panic]
fn square_fail() {
    IntegerMatrix::from_str("[1, 2]").unwrap().square();
}

#[test]
#[should_panic]
fn square_ref_fail() {
    (&IntegerMatrix::from_str("[]_{0,2}").unwrap()).square();
}

#[test]
#[should_panic]
fn square_assign_fail() {
    let mut a = IntegerMatrix::from_str("[1; 2]").unwrap();
    a.square_assign();
}

#[test]
fn square_properties() {
    integer_matrix_gen_var_1().test_properties(|a| {
        let b = (&a).square();
        assert!(b.is_valid());
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        assert_eq!(a.clone().square(), b);
        let mut c = a.clone();
        c.square_assign();
        assert_eq!(c, b);
        assert_eq!(integer_matrix_square_naive(&a), b);
        assert_eq!(&a * &a, b);
        assert_eq!(integer_matrix_square_with(&a, matrix_square_bodrato), b);
        // The square of the transpose is the transpose of the square.
        assert_eq!(a.transpose().square(), b.transpose());
        // Negating a matrix leaves its square unchanged.
        assert_eq!((-&a).square(), b);
    });
}
