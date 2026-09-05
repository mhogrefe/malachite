// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CheckedSqrt, Conjugate, MulI, Square};
use malachite_base::num::basic::traits::Zero;
use malachite_base::strings::ToDebugString;
use malachite_nz::gaussian_integer::{ComparableGaussianIntegerRef, GaussianInteger};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::gaussian_integer::arithmetic::sqrt::*;
use malachite_nz::test_util::generators::{
    gaussian_integer_gen, gaussian_integer_gen_var_4, integer_gen, natural_gen,
};
use std::str::FromStr;

#[test]
fn test_checked_sqrt() {
    let test = |s, out: Option<&str>| {
        let x = GaussianInteger::from_str(s).unwrap();
        let out = out.map(|t| GaussianInteger::from_str(t).unwrap().to_debug_string());
        let root = x.clone().checked_sqrt();
        if let Some(root) = &root {
            assert!(root.real.is_valid());
            assert!(root.imaginary.is_valid());
        }
        assert_eq!(root.map(|r| r.to_debug_string()), out);
        assert_eq!((&x).checked_sqrt().map(|r| r.to_debug_string()), out);
        assert_eq!(
            gaussian_integer_checked_sqrt_naive(&x).map(|r| r.to_debug_string()),
            out
        );
    };
    // real inputs: a negative square's root is purely imaginary
    test("0", Some("0"));
    test("1", Some("1"));
    test("-1", Some("i"));
    test("4", Some("2"));
    test("-4", Some("2i"));
    test("9", Some("3"));
    test("-9", Some("3i"));
    test("2", None);
    test("-2", None);
    // purely imaginary inputs: 2i = (1+i)^2
    test("i", None);
    test("-i", None);
    test("2i", Some("1+i"));
    test("-2i", Some("1-i"));
    test("8i", Some("2+2i"));
    test("-8i", Some("2-2i"));
    test("4i", None);
    test("-4i", None);
    test("18i", Some("3+3i"));
    test("32i", Some("4+4i"));
    // general inputs, and the principal root among the two
    test("3+4i", Some("2+i"));
    test("-3+4i", Some("1+2i"));
    test("3-4i", Some("2-i"));
    test("-3-4i", Some("1-2i"));
    test("5+12i", Some("3+2i"));
    test("-5-12i", Some("2-3i"));
    test("-7+24i", Some("3+4i"));
    test("7-24i", Some("4-3i"));
    test("21+20i", Some("5+2i"));
    test("-21+20i", Some("2+5i"));
    test("24+10i", Some("5+i"));
    test("2+i", None);
    test("1+2i", None);
    test("1000000000000", Some("1000000"));
    test("-1000000000000", Some("1000000i"));
    test("2000000000000i", Some("1000000+1000000i"));
    test("-999999999999999999+2000000000000000000i", None);
    test(
        "-960219479039780520-243865262225270538i",
        Some("123456789-987654321i"),
    );
}

#[test]
fn test_checked_sqrts() {
    let test = |s, out| {
        let x = GaussianInteger::from_str(s).unwrap();
        let roots = x.checked_sqrts();
        for root in &roots {
            assert!(root.real.is_valid());
            assert!(root.imaginary.is_valid());
        }
        assert_eq!(
            roots.iter().map(ToString::to_string).collect::<Vec<_>>(),
            out
        );
    };
    test("0", vec!["0"]);
    test("1", vec!["-1", "1"]);
    test("-1", vec!["-i", "i"]);
    test("2", vec![]);
    test("2i", vec!["-1-i", "1+i"]);
    test("3+4i", vec!["-2-i", "2+i"]);
    test("-3+4i", vec!["-1-2i", "1+2i"]);
    test("2+i", vec![]);
    test("-1000000000000", vec!["-1000000i", "1000000i"]);
}

fn principal(x: GaussianInteger) -> GaussianInteger {
    if (&x.real, &x.imaginary) >= (&Integer::ZERO, &Integer::ZERO) {
        x
    } else {
        -x
    }
}

#[test]
fn checked_sqrt_properties() {
    gaussian_integer_gen().test_properties(|x| {
        let root = (&x).checked_sqrt();
        if let Some(root) = &root {
            assert!(root.real.is_valid());
            assert!(root.imaginary.is_valid());
            assert_eq!(root.square(), x);
            assert_eq!(principal(root.clone()), *root);
        }
        assert_eq!(x.clone().checked_sqrt(), root);
        assert_eq!(gaussian_integer_checked_sqrt_naive(&x), root);

        // all the roots: the principal one first, then its negative unless it is zero
        let roots = x.checked_sqrts();
        assert!(roots.is_sorted_by(|a, b| {
            ComparableGaussianIntegerRef(a) <= ComparableGaussianIntegerRef(b)
        }));
        assert_eq!(
            roots.contains(root.as_ref().unwrap_or(&GaussianInteger::ZERO)),
            root.is_some()
        );
        match roots.as_slice() {
            [] => assert!(root.is_none()),
            [r] => assert_eq!(*r, GaussianInteger::ZERO),
            [r, s] => {
                assert_eq!(*s, -r);
                assert_ne!(*r, GaussianInteger::ZERO);
            }
            _ => panic!("more than two square roots"),
        }
        for r in &roots {
            assert_eq!(r.square(), x);
        }
        // the roots of the conjugate and of the negative are the conjugate and the rotation of the
        // root, up to the choice of the principal one
        assert_eq!(
            (&x).conjugate().checked_sqrt(),
            root.as_ref().map(|r| principal(r.conjugate()))
        );
        assert_eq!(
            (-&x).checked_sqrt(),
            root.as_ref().map(|r| principal(r.mul_i()))
        );
    });

    gaussian_integer_gen_var_4().test_properties(|x| {
        let root = (&x).checked_sqrt().unwrap();
        assert_eq!((&root).square(), x);
        assert_eq!(principal(root.clone()), root);
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_eq!((&x).square().checked_sqrt(), Some(principal(x)));
    });

    integer_gen().test_properties(|n| {
        let root = GaussianInteger::from(n.clone()).checked_sqrt();
        if n >= 0u32 {
            assert_eq!(root, n.checked_sqrt().map(GaussianInteger::from));
        } else {
            assert_eq!(
                root,
                (-n).checked_sqrt().map(|r| GaussianInteger {
                    real: Integer::ZERO,
                    imaginary: r,
                })
            );
        }
    });

    natural_gen().test_properties(|n| {
        assert_eq!(
            GaussianInteger::from(n.clone()).checked_sqrt(),
            n.checked_sqrt().map(GaussianInteger::from)
        );
    });
}
