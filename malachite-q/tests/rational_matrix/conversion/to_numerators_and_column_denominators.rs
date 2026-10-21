// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Gcd;
use malachite_q::Rational;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::rational_matrix_gen;
use malachite_q::test_util::rational_matrix::conversion::to_numerators_and_column_denominators::*;

#[test]
fn test_to_numerators_and_column_denominators() {
    let test = |s, numerators_out, denominators_out| {
        let a = RationalMatrix::from_str(s).unwrap();
        let (ns, ds) = a.to_numerators_and_column_denominators();
        assert!(ns.is_valid());
        assert_eq!(ns.to_string(), numerators_out);
        assert_eq!(ds.to_string(), denominators_out);
        assert_eq!(
            a.clone().into_numerators_and_column_denominators(),
            (ns.clone(), ds.clone())
        );
        assert_eq!(
            rational_matrix_to_numerators_and_column_denominators_naive(&a),
            (ns, ds)
        );
    };
    test("[]_{0,0}", "[]_{0,0}", "()");
    test("[]_{2,0}", "[]_{2,0}", "()");
    test("[]_{0,3}", "[]_{0,3}", "(1, 1, 1)");
    test("[0]", "[0]", "(1)");
    test("[5, -3; 1, 0]", "[5, -3; 1, 0]", "(1, 1)");
    test("[1/2]", "[1]", "(2)");
    test("[-1/2]", "[-1]", "(2)");
    test("[1/2, -2/3; 5, 1/4]", "[1, -8; 10, 3]", "(2, 12)");
    test("[1/4; 1/6]", "[3; 2]", "(12)");
    test("[1/6, 1/6, 1/6]", "[1, 1, 1]", "(6, 6, 6)");
    test("[1, 1/2; 1/3, 1]", "[3, 1; 1, 2]", "(3, 2)");
    test(
        "[0, 1/1000000000000; 1/3, 0]",
        "[0, 1; 1, 0]",
        "(3, 1000000000000)",
    );
}

#[test]
fn to_numerators_and_column_denominators_properties() {
    rational_matrix_gen().test_properties(|a| {
        let (ns, ds) = a.to_numerators_and_column_denominators();
        assert!(ns.is_valid());
        assert_eq!(
            a.clone().into_numerators_and_column_denominators(),
            (ns.clone(), ds.clone())
        );
        assert_eq!(
            rational_matrix_to_numerators_and_column_denominators_naive(&a),
            (ns.clone(), ds.clone())
        );
        assert_eq!((ns.height(), ns.width()), (a.height(), a.width()));
        assert_eq!(u64::try_from(ds.elements.len()).unwrap(), a.width());
        for (k, d) in ds.elements.iter().enumerate() {
            let k = u64::try_from(k).unwrap();
            assert_ne!(*d, 0u32);
            // The quotients in each column are its elements.
            for (n, x) in ns.column_iter(k).zip(a.column_iter(k)) {
                assert_eq!(&Rational::from(n) / Rational::from(d), *x);
            }
            // Every element's denominator divides its column's, and the column is in lowest terms.
            let mut g = d.clone();
            for (n, x) in ns.column_iter(k).zip(a.column_iter(k)) {
                assert!((d % x.denominator_ref()) == 0u32);
                g = g.gcd(n.unsigned_abs_ref());
            }
            assert_eq!(g, 1u32);
        }
        // Clearing the columns of a matrix clears the rows of its transpose.
        let (ns_t, ds_t) = a.transpose().to_numerators_and_row_denominators();
        assert_eq!((ns_t.transpose(), ds_t), (ns, ds));
    });
}
