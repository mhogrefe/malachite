// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{CoprimeWith, Crt};
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_matrix::arithmetic::crt::natural_matrix_crt_naive;

#[test]
fn test_crt() {
    let test = |a, m1, b, m2: Limb, out: Option<&str>| {
        let a = NaturalMatrix::from_str(a).unwrap();
        let m1 = Natural::from_str(m1).unwrap();
        let b = UnsignedMatrix::from_str(b).unwrap();
        let result = a.clone().crt(m1.clone(), b.clone(), m2);
        assert!(result.as_ref().is_none_or(NaturalMatrix::is_valid));
        assert_eq!(result.as_ref().map(ToString::to_string).as_deref(), out);
        assert_eq!(a.clone().crt(m1.clone(), &b, m2), result);
        assert_eq!(a.clone().crt(&m1, b.clone(), m2), result);
        assert_eq!(a.clone().crt(&m1, &b, m2), result);
        assert_eq!((&a).crt(m1.clone(), b.clone(), m2), result);
        assert_eq!((&a).crt(m1.clone(), &b, m2), result);
        assert_eq!((&a).crt(&m1, b.clone(), m2), result);
        assert_eq!((&a).crt(&m1, &b, m2), result);
        assert_eq!(natural_matrix_crt_naive(&a, &m1, &b, m2), result);
    };
    test("[2, 1; 0, 4]", "5", "[1, 0; 2, 1]", 3, Some("[7, 6; 5, 4]"));
    test("[]_{0,3}", "5", "[]_{0,3}", 3, Some("[]_{0,3}"));
    test("[]_{2,0}", "6", "[]_{2,0}", 3, None);
    test("[2, 1]", "6", "[1, 0]", 3, None);
    test("[0]", "4", "[1]", 2, None);
    // 6 and 4 share a factor, though 4 does not divide 6.
    test("[1]", "6", "[1]", 4, None);
    test("[3, 4]", "9", "[0, 0]", 3, None);
    test("[3, 4]", "5", "[0, 0]", 1, Some("[3, 4]"));
    test("[0, 0]", "1", "[5, 6]", 7, Some("[5, 6]"));
    test(
        "[18446744073709551628, 12345; 0, 1]",
        "18446744073709551629",
        "[18446744073709551556, 0; 1, 98765]",
        18446744073709551557,
        Some(
            "[340282366920938462614824380041128836352, 99249023685273715099809116540545716559; 3166\
            51646995873291600162225094851966490, 132332031580364982987351588013913395896]",
        ),
    );
}

#[test]
#[should_panic]
fn crt_fail_1() {
    // Different shapes.
    let _ = (&NaturalMatrix::from_str("[1, 2]").unwrap()).crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[1]").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn crt_fail_2() {
    // Different shapes.
    let _ = NaturalMatrix::from_str("[1, 2]").unwrap().crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[1]").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn crt_fail_3() {
    // Different shapes with no elements.
    let _ = (&NaturalMatrix::from_str("[]_{0,2}").unwrap()).crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[]_{2,0}").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn crt_fail_4() {
    // Different shapes with no elements.
    let _ = NaturalMatrix::from_str("[]_{0,2}").unwrap().crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[]_{2,0}").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn crt_fail_5() {
    // M1 is 0.
    let _ = (&NaturalMatrix::from_str("[1]").unwrap()).crt(
        Natural::from_str("0").unwrap(),
        UnsignedMatrix::from_str("[1]").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn crt_fail_6() {
    // M1 is 0.
    let _ = NaturalMatrix::from_str("[1]").unwrap().crt(
        Natural::from_str("0").unwrap(),
        UnsignedMatrix::from_str("[1]").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn crt_fail_7() {
    // M1 is 0, with no elements.
    let _ = (&NaturalMatrix::from_str("[]_{0,0}").unwrap()).crt(
        Natural::from_str("0").unwrap(),
        UnsignedMatrix::from_str("[]_{0,0}").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn crt_fail_8() {
    // M1 is 0, with no elements.
    let _ = NaturalMatrix::from_str("[]_{0,0}").unwrap().crt(
        Natural::from_str("0").unwrap(),
        UnsignedMatrix::from_str("[]_{0,0}").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn crt_fail_9() {
    // M2 is 0.
    let _ = (&NaturalMatrix::from_str("[1]").unwrap()).crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[0]").unwrap(),
        0,
    );
}

#[test]
#[should_panic]
fn crt_fail_10() {
    // M2 is 0.
    let _ = NaturalMatrix::from_str("[1]").unwrap().crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[0]").unwrap(),
        0,
    );
}

#[test]
#[should_panic]
fn crt_fail_11() {
    // M2 is 0, with no elements.
    let _ = (&NaturalMatrix::from_str("[]_{0,0}").unwrap()).crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[]_{0,0}").unwrap(),
        0,
    );
}

#[test]
#[should_panic]
fn crt_fail_12() {
    // M2 is 0, with no elements.
    let _ = NaturalMatrix::from_str("[]_{0,0}").unwrap().crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[]_{0,0}").unwrap(),
        0,
    );
}

#[test]
#[should_panic]
fn crt_fail_13() {
    // An element of self is not reduced.
    let _ = (&NaturalMatrix::from_str("[5]").unwrap()).crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[1]").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn crt_fail_14() {
    // An element of self is not reduced.
    let _ = NaturalMatrix::from_str("[5]").unwrap().crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[1]").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn crt_fail_15() {
    // An element of r2 is not reduced.
    let _ = (&NaturalMatrix::from_str("[1]").unwrap()).crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[3]").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn crt_fail_16() {
    // An element of r2 is not reduced.
    let _ = NaturalMatrix::from_str("[1]").unwrap().crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[3]").unwrap(),
        3,
    );
}

#[test]
fn crt_properties() {
    natural_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1().test_properties(
        |(a, m1, b, m2)| {
            let result = (&a).crt(&m1, &b, m2);
            assert!(result.as_ref().is_none_or(NaturalMatrix::is_valid));
            assert_eq!(a.clone().crt(m1.clone(), b.clone(), m2), result);
            assert_eq!(a.clone().crt(m1.clone(), &b, m2), result);
            assert_eq!(a.clone().crt(&m1, b.clone(), m2), result);
            assert_eq!(a.clone().crt(&m1, &b, m2), result);
            assert_eq!((&a).crt(m1.clone(), b.clone(), m2), result);
            assert_eq!((&a).crt(m1.clone(), &b, m2), result);
            assert_eq!((&a).crt(&m1, b.clone(), m2), result);
            assert_eq!(natural_matrix_crt_naive(&a, &m1, &b, m2), result);
            // There is a result exactly when the moduli are coprime.
            assert_eq!(result.is_some(), (&m1).coprime_with(Natural::from(m2)));
            let Some(c) = result else {
                return;
            };
            assert_eq!((c.height(), c.width()), (a.height(), a.width()));
            let m = &m1 * Natural::from(m2);
            assert!(c.elements.iter().all(|x| *x < m));
            // The result is congruent to each matrix modulo its modulus.
            assert_eq!(&c % &m1, a);
            assert_eq!(&c % m2, b);
            // Combination commutes with transposition.
            assert_eq!(
                c.transpose(),
                a.transpose().crt(&m1, &b.transpose(), m2).unwrap()
            );
        },
    );
}
