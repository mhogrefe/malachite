// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{BalancedCrt, BalancedMod, CoprimeWith, Crt, Mod};
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::integer_matrix::arithmetic::crt::integer_matrix_balanced_crt_naive;

#[test]
fn test_balanced_crt() {
    let test = |a, m1, b, m2: Limb, out: Option<&str>| {
        let a = IntegerMatrix::from_str(a).unwrap();
        let m1 = Natural::from_str(m1).unwrap();
        let b = UnsignedMatrix::from_str(b).unwrap();
        let result = a.clone().balanced_crt(m1.clone(), b.clone(), m2);
        assert!(result.as_ref().is_none_or(IntegerMatrix::is_valid));
        assert_eq!(result.as_ref().map(ToString::to_string).as_deref(), out);
        assert_eq!(a.clone().balanced_crt(m1.clone(), &b, m2), result);
        assert_eq!(a.clone().balanced_crt(&m1, b.clone(), m2), result);
        assert_eq!(a.clone().balanced_crt(&m1, &b, m2), result);
        assert_eq!((&a).balanced_crt(m1.clone(), b.clone(), m2), result);
        assert_eq!((&a).balanced_crt(m1.clone(), &b, m2), result);
        assert_eq!((&a).balanced_crt(&m1, b.clone(), m2), result);
        assert_eq!((&a).balanced_crt(&m1, &b, m2), result);
        assert_eq!(integer_matrix_balanced_crt_naive(&a, &m1, &b, m2), result);
    };
    test("[2, 1; 0, 4]", "5", "[1, 0; 2, 1]", 3, Some("[7, 6; 5, 4]"));
    test("[]_{0,3}", "5", "[]_{0,3}", 3, Some("[]_{0,3}"));
    test("[]_{2,0}", "6", "[]_{2,0}", 3, None);
    test("[2, 1]", "6", "[1, 0]", 3, None);
    test("[0]", "4", "[1]", 2, None);
    // 6 and 4 share a factor, though 4 does not divide 6.
    test("[1]", "6", "[1]", 4, None);
    test("[3, 4]", "9", "[0, 0]", 3, None);
    test("[3, 4]", "5", "[0, 0]", 1, Some("[-2, -1]"));
    test("[0, 0]", "1", "[5, 6]", 7, Some("[-2, -1]"));
    #[cfg(not(feature = "32_bit_limbs"))]
    test(
        "[18446744073709551628, 12345; 0, 1]",
        "18446744073709551629",
        "[18446744073709551556, 0; 1, 98765]",
        18446744073709551557,
        Some(
            "[-1, 99249023685273715099809116540545716559; -23630719925065171014662154946276869863, \
            132332031580364982987351588013913395896]",
        ),
    );
    test(
        "[-2, 1; 0, -5]",
        "5",
        "[1, 0; 2, 1]",
        3,
        Some("[-2, 6; 5, -5]"),
    );
    test("[0]", "5", "[1]", 2, Some("[5]"));
    test("[-1]", "5", "[0]", 2, Some("[4]"));
    #[cfg(not(feature = "32_bit_limbs"))]
    test(
        "[-18446744073709551629, -12345]",
        "18446744073709551629",
        "[18446744073709551556, 7]",
        18446744073709551557,
        Some("[23630719925065171014662154946276869863, 75618303760208550412380178876645030753]"),
    );
}

#[test]
#[should_panic]
fn balanced_crt_fail_1() {
    // Different shapes.
    let _ = (&IntegerMatrix::from_str("[1, 2]").unwrap()).balanced_crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[1]").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn balanced_crt_fail_2() {
    // Different shapes.
    let _ = IntegerMatrix::from_str("[1, 2]").unwrap().balanced_crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[1]").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn balanced_crt_fail_3() {
    // Different shapes with no elements.
    let _ = (&IntegerMatrix::from_str("[]_{0,2}").unwrap()).balanced_crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[]_{2,0}").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn balanced_crt_fail_4() {
    // Different shapes with no elements.
    let _ = IntegerMatrix::from_str("[]_{0,2}").unwrap().balanced_crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[]_{2,0}").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn balanced_crt_fail_5() {
    // M1 is 0.
    let _ = (&IntegerMatrix::from_str("[1]").unwrap()).balanced_crt(
        Natural::from_str("0").unwrap(),
        UnsignedMatrix::from_str("[1]").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn balanced_crt_fail_6() {
    // M1 is 0.
    let _ = IntegerMatrix::from_str("[1]").unwrap().balanced_crt(
        Natural::from_str("0").unwrap(),
        UnsignedMatrix::from_str("[1]").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn balanced_crt_fail_7() {
    // M1 is 0, with no elements.
    let _ = (&IntegerMatrix::from_str("[]_{0,0}").unwrap()).balanced_crt(
        Natural::from_str("0").unwrap(),
        UnsignedMatrix::from_str("[]_{0,0}").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn balanced_crt_fail_8() {
    // M1 is 0, with no elements.
    let _ = IntegerMatrix::from_str("[]_{0,0}").unwrap().balanced_crt(
        Natural::from_str("0").unwrap(),
        UnsignedMatrix::from_str("[]_{0,0}").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn balanced_crt_fail_9() {
    // M2 is 0.
    let _ = (&IntegerMatrix::from_str("[1]").unwrap()).balanced_crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[0]").unwrap(),
        0,
    );
}

#[test]
#[should_panic]
fn balanced_crt_fail_10() {
    // M2 is 0.
    let _ = IntegerMatrix::from_str("[1]").unwrap().balanced_crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[0]").unwrap(),
        0,
    );
}

#[test]
#[should_panic]
fn balanced_crt_fail_11() {
    // M2 is 0, with no elements.
    let _ = (&IntegerMatrix::from_str("[]_{0,0}").unwrap()).balanced_crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[]_{0,0}").unwrap(),
        0,
    );
}

#[test]
#[should_panic]
fn balanced_crt_fail_12() {
    // M2 is 0, with no elements.
    let _ = IntegerMatrix::from_str("[]_{0,0}").unwrap().balanced_crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[]_{0,0}").unwrap(),
        0,
    );
}

#[test]
#[should_panic]
fn balanced_crt_fail_13() {
    // An element of self is not reduced.
    let _ = (&IntegerMatrix::from_str("[5]").unwrap()).balanced_crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[1]").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn balanced_crt_fail_14() {
    // An element of self is not reduced.
    let _ = IntegerMatrix::from_str("[5]").unwrap().balanced_crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[1]").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn balanced_crt_fail_15() {
    // An element of r2 is not reduced.
    let _ = (&IntegerMatrix::from_str("[1]").unwrap()).balanced_crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[3]").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn balanced_crt_fail_16() {
    // An element of r2 is not reduced.
    let _ = IntegerMatrix::from_str("[1]").unwrap().balanced_crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[3]").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn balanced_crt_fail_17() {
    // An element of self is less than -m1.
    let _ = (&IntegerMatrix::from_str("[-6]").unwrap()).balanced_crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[1]").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn balanced_crt_fail_18() {
    // An element of self is less than -m1.
    let _ = IntegerMatrix::from_str("[-6]").unwrap().balanced_crt(
        Natural::from_str("5").unwrap(),
        UnsignedMatrix::from_str("[1]").unwrap(),
        3,
    );
}

#[test]
fn balanced_crt_properties() {
    integer_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1().test_properties(
        |(a, m1, b, m2)| {
            let result = (&a).balanced_crt(&m1, &b, m2);
            assert!(result.as_ref().is_none_or(IntegerMatrix::is_valid));
            assert_eq!(a.clone().balanced_crt(m1.clone(), b.clone(), m2), result);
            assert_eq!(a.clone().balanced_crt(m1.clone(), &b, m2), result);
            assert_eq!(a.clone().balanced_crt(&m1, b.clone(), m2), result);
            assert_eq!(a.clone().balanced_crt(&m1, &b, m2), result);
            assert_eq!((&a).balanced_crt(m1.clone(), b.clone(), m2), result);
            assert_eq!((&a).balanced_crt(m1.clone(), &b, m2), result);
            assert_eq!((&a).balanced_crt(&m1, b.clone(), m2), result);
            assert_eq!(integer_matrix_balanced_crt_naive(&a, &m1, &b, m2), result);
            // There is a result exactly when the moduli are coprime.
            assert_eq!(result.is_some(), (&m1).coprime_with(Natural::from(m2)));
            let Some(c) = result else {
                return;
            };
            assert_eq!((c.height(), c.width()), (a.height(), a.width()));
            let m = &m1 * Natural::from(m2);
            // Each element x satisfies -m / 2 < x <= m / 2.
            let m_integer = Integer::from(&m);
            assert!(c.elements.iter().all(|x| {
                let twice = x << 1u32;
                twice > -&m_integer && twice <= m_integer
            }));
            // The result is congruent to each matrix modulo its modulus.
            assert_eq!((&c).mod_op(&m1), (&a).mod_op(&m1));
            assert_eq!((&c).mod_op(m2), b);
            // Lifting the first matrix into [0, m1) and balancing the canonical combination gives
            // the same result.
            assert_eq!(
                (&a).mod_op(&m1)
                    .crt(&m1, &b, m2)
                    .map(|x| IntegerMatrix::from(x).balanced_mod(Integer::from(&m))),
                Some(c.clone())
            );
            // Combination commutes with transposition.
            assert_eq!(
                c.transpose(),
                a.transpose().balanced_crt(&m1, &b.transpose(), m2).unwrap()
            );
        },
    );
}
