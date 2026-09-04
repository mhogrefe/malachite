// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CheckedSqrt, Conjugate, MulI, Square};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::test_util::generators::gaussian_integer_gen;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::gaussian_rational::arithmetic::sqrt::*;
use malachite_q::test_util::generators::{
    gaussian_rational_gen, gaussian_rational_gen_var_4, rational_gen,
};
use std::str::FromStr;

#[test]
fn test_checked_sqrt() {
    let test = |s, out: Option<&str>| {
        let x = GaussianRational::from_str(s).unwrap();
        let out = out.map(ToString::to_string);
        let root = x.clone().checked_sqrt();
        if let Some(root) = &root {
            assert!(root.real.is_valid());
            assert!(root.imaginary.is_valid());
        }
        assert_eq!(root.map(|r| r.to_string()), out);
        assert_eq!((&x).checked_sqrt().map(|r| r.to_string()), out);
        assert_eq!(
            gaussian_rational_checked_sqrt_naive(&x).map(|r| r.to_string()),
            out
        );
    };
    test("0", Some("0"));
    test("1", Some("1"));
    test("-1", Some("i"));
    test("1/4", Some("1/2"));
    test("-1/4", Some("i/2"));
    test("1/2", None);
    test("1/8", None);
    test("9/16", Some("3/4"));
    test("2i", Some("1+i"));
    test("i", None);
    test("i/2", Some("1/2+i/2"));
    test("-i/2", Some("1/2-i/2"));
    test("-2i/9", Some("1/3-i/3"));
    test("3/4+i", Some("1+i/2"));
    test("3+4i", Some("2+i"));
    test("-3/25+4i/25", Some("1/5+2i/5"));
    test("5/9+4i/3", Some("1+2i/3"));
    test("-7/4+6i", Some("3/2+2i"));
    test("21/4+5i", Some("5/2+i"));
    test("1/2+i/3", None);
    test("2+i", None);
}

#[test]
fn test_checked_sqrts() {
    let test = |s, out| {
        let roots = GaussianRational::from_str(s).unwrap().checked_sqrts();
        assert_eq!(
            roots.iter().map(ToString::to_string).collect::<Vec<_>>(),
            out
        );
    };
    test("0", vec!["0"]);
    test("1/4", vec!["1/2", "-1/2"]);
    test("-1/4", vec!["i/2", "-i/2"]);
    test("1/2", vec![]);
    test("3/4+i", vec!["1+i/2", "-1-i/2"]);
}

fn principal(x: GaussianRational) -> GaussianRational {
    if (&x.real, &x.imaginary) >= (&Rational::ZERO, &Rational::ZERO) {
        x
    } else {
        -x
    }
}

#[test]
fn checked_sqrt_properties() {
    gaussian_rational_gen().test_properties(|x| {
        let root = (&x).checked_sqrt();
        if let Some(root) = &root {
            assert!(root.real.is_valid());
            assert!(root.imaginary.is_valid());
            assert_eq!(root.square(), x);
            assert_eq!(principal(root.clone()), *root);
        }
        assert_eq!(x.clone().checked_sqrt(), root);
        assert_eq!(gaussian_rational_checked_sqrt_naive(&x), root);
        assert_eq!(
            (&x).conjugate().checked_sqrt(),
            root.as_ref().map(|r| principal(r.conjugate()))
        );
        assert_eq!(
            (-&x).checked_sqrt(),
            root.as_ref().map(|r| principal(r.mul_i()))
        );

        let roots = x.checked_sqrts();
        assert_eq!(roots.first(), root.as_ref());
        match roots.as_slice() {
            [] => assert!(root.is_none()),
            [r] => assert_eq!(*r, GaussianRational::ZERO),
            [r, s] => {
                assert_eq!(*s, -r);
                assert_ne!(*r, GaussianRational::ZERO);
            }
            _ => panic!("more than two square roots"),
        }
        for r in &roots {
            assert_eq!(r.square(), x);
        }
    });

    gaussian_rational_gen_var_4().test_properties(|x| {
        let root = (&x).checked_sqrt().unwrap();
        assert_eq!((&root).square(), x);
        assert_eq!(principal(root.clone()), root);
    });

    gaussian_rational_gen().test_properties(|x| {
        assert_eq!((&x).square().checked_sqrt(), Some(principal(x)));
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_eq!(
            GaussianRational::from(x.clone()).checked_sqrt(),
            x.checked_sqrt().map(GaussianRational::from)
        );
    });

    rational_gen().test_properties(|q| {
        let root = GaussianRational::from(q.clone()).checked_sqrt();
        if q >= 0 {
            assert_eq!(root, q.checked_sqrt().map(GaussianRational::from));
        } else {
            assert_eq!(
                root,
                (-q).checked_sqrt().map(|r| GaussianRational {
                    real: Rational::ZERO,
                    imaginary: r,
                })
            );
        }
    });
}
