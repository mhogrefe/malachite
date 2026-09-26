// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{MakeMonic, MakeMonicAssign, Polynomial, PrimitivePart};
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::rational_polynomial_gen;

#[test]
fn test_make_monic() {
    let test = |s, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = p.clone().make_monic();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!((&p).make_monic(), q);
        let mut r = p;
        r.make_monic_assign();
        assert_eq!(r, q);
    };
    test("0", "0");
    test("1", "1");
    test("-1", "1");
    test("3/7", "1");
    test("-x", "x");
    test("6*x+4", "x+2/3");
    test("-2/3*x-4/3", "x+2");
    test("1/2*x^2+1/3", "x^2+2/3");
    test("-5/2*x^3+10/3", "x^3-4/3");
    test("x-1/2", "x-1/2");
    test("-1/2*x^2+1/3", "x^2-2/3");
    test("4/9*x^2-2/3*x+8/15", "x^2-3/2*x+6/5");
}

#[test]
fn make_monic_properties() {
    rational_polynomial_gen().test_properties(|p| {
        let q = (&p).make_monic();
        assert!(q.is_valid());
        assert_eq!(p.clone().make_monic(), q);
        let mut r = p.clone();
        r.make_monic_assign();
        assert_eq!(r, q);

        assert_eq!(q == RationalPolynomial::ZERO, p == RationalPolynomial::ZERO);
        if p != RationalPolynomial::ZERO {
            assert!(q.is_monic());
            // q is p divided by its leading coefficient.
            let leading = p.leading_coefficient();
            for i in 0..p.len() {
                assert_eq!(p.coefficient(i) / &leading, q.coefficient(i));
            }
        }
        // Making it monic again changes nothing, and the primitive part has the same monic form.
        assert_eq!((&q).make_monic(), q);
        assert_eq!(
            RationalPolynomial::from((&p).primitive_part()).make_monic(),
            q
        );
    });
}
