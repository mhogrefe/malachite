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
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_gen_var_1;
use malachite_nz::test_util::natural_matrix::arithmetic::square::{
    natural_matrix_square_naive, natural_matrix_square_with,
};
use malachite_nz::test_util::natural_matrix::pseudorandom::pseudorandom_natural_matrix;

#[test]
fn test_square() {
    let test = |s, out| {
        let a = NaturalMatrix::from_str(s).unwrap();

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

        assert_eq!(natural_matrix_square_naive(&a).to_string(), out);
        assert_eq!(
            natural_matrix_square_with(&a, matrix_square_bodrato).to_string(),
            out
        );
    };
    test("[]_{0,0}", "[]_{0,0}");
    test("[5]", "[25]");
    test("[1, 2; 3, 4]", "[7, 10; 15, 22]");
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 9]",
        "[30, 36, 42; 66, 81, 96; 102, 126, 150]",
    );
    test(
        "[18446744073709551615, 18446744073709551616, 170141183460469231731687303715884105728; 1, 0\
        , 1180591620717411303424; 3, 36893488147419103239, 5]",
        "[850705917302346158639989774505710977025, 627710173538668076536706007435188950166909443383\
        3281912832, 3138550867693340404376530928385771796633901812728719212544; 3560221606225943461\
        887, 43556142965880123331594537840361919741952, 170141183460469237634645407302940622848; 92\
        233720368547758099, 239807672958224171043, 44066566516261531026771153007435862507545]",
    );
    test(
        "[1361129467683753853853498429727072845824, 1, 2; 3, 27222589353675077077069968594541456916\
        49, 4; 5, 6, 680564733841876926926749214863536422912]",
        "[1852673427797059126777135760139006525652319754650249024631321344126610074238989, 40833884\
        03051261561560495289181218537485, 4083388403051261561560495289181218537476; 122501652091537\
        84684681485867543655612439, 741069371118823650710854304055602610261472353647173111394069937\
        0225348588339228, 13611294676837538538534984297270728458250; 102084710076281539039012382229\
        53046343698, 20416942015256307807802476445906092687371, 46316835694926478169428394003475163\
        1413079938662562256157830336031652518559778]",
    );
    test(
        "[4611686018427387904, 1; 1, 3]",
        "[21267647932558653966460912964485513217, 4611686018427387907; 4611686018427387907, 10]",
    );
    test(
        "[1, 2, 3, 4; 5, 6, 7, 8; 9, 10, 11, 12; 13, 14, 15, 16]",
        "[90, 100, 110, 120; 202, 228, 254, 280; 314, 356, 398, 440; 426, 484, 542, 600]",
    );
}

#[test]
fn test_square_kernels() {
    // Dimensions up to 9, which cover each explicit formula, both halves of Bodrato's algorithm
    // with and without an odd last row and column, and squaring through multiplication; and
    // elements that fit in small `fmpz`s, in double words, and in neither.
    let check = |a: &NaturalMatrix| {
        let expected = natural_matrix_square_naive(a);
        assert_eq!(a.square(), expected);
        let mut b = a.clone();
        b.square_assign();
        assert_eq!(b, expected);
        assert_eq!(
            natural_matrix_square_with(a, matrix_square_bodrato),
            expected
        );
    };
    for bits in [10, 30, 62, 63, 100, 127, 128, 200, 1100] {
        for n in 0..10 {
            for seed in 0..2 {
                check(&pseudorandom_natural_matrix(n, n, bits, seed));
            }
        }
    }
}

#[test]
#[should_panic]
fn square_fail() {
    NaturalMatrix::from_str("[1, 2]").unwrap().square();
}

#[test]
#[should_panic]
fn square_ref_fail() {
    (&NaturalMatrix::from_str("[]_{0,2}").unwrap()).square();
}

#[test]
#[should_panic]
fn square_assign_fail() {
    let mut a = NaturalMatrix::from_str("[1; 2]").unwrap();
    a.square_assign();
}

#[test]
fn square_properties() {
    natural_matrix_gen_var_1().test_properties(|a| {
        let b = (&a).square();
        assert!(b.is_valid());
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        assert_eq!(a.clone().square(), b);
        let mut c = a.clone();
        c.square_assign();
        assert_eq!(c, b);
        assert_eq!(natural_matrix_square_naive(&a), b);
        assert_eq!(&a * &a, b);
        assert_eq!(natural_matrix_square_with(&a, matrix_square_bodrato), b);
        // The square of the transpose is the transpose of the square.
        assert_eq!(a.transpose().square(), b.transpose());
        // Squaring commutes with the conversion to an `IntegerMatrix`.
        assert_eq!(
            IntegerMatrix::from(a.clone()).square(),
            IntegerMatrix::from(b.clone())
        );
    });
}
