// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    ModHowellForm, ModIsReduced, ModStrongEchelonForm, ModStrongEchelonFormAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_9;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_natural_pair_gen_var_4;
use malachite_nz::test_util::natural_matrix::arithmetic::mod_strong_echelon_form::*;

// Whether enumerating the row span of `a` modulo `m`, as the naive algorithm does, is cheap.
fn a_is_small(a: &NaturalMatrix, m: &Natural) -> bool {
    u64::convertible_from(m)
        && u64::exact_from(m)
            .checked_pow(u32::exact_from(a.width() + 1))
            .and_then(|x| x.checked_mul(a.height()))
            .is_some_and(|x| x <= 1 << 14)
}

#[test]
fn test_mod_strong_echelon_form() {
    let test = |s, m, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let b = (&a).mod_strong_echelon_form(&m);
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);
        assert_eq!(a.clone().mod_strong_echelon_form(m.clone()), b);
        assert_eq!(a.clone().mod_strong_echelon_form(&m), b);
        assert_eq!((&a).mod_strong_echelon_form(m.clone()), b);
        let mut c = a.clone();
        c.mod_strong_echelon_form_assign(m.clone());
        assert!(c.is_valid());
        assert_eq!(c, b);
        let mut c = a.clone();
        c.mod_strong_echelon_form_assign(&m);
        assert_eq!(c, b);
        if a_is_small(&a, &m) {
            assert_eq!(natural_matrix_mod_strong_echelon_form_naive(&a, &m), b);
        }
    };
    test("[]_{0,0}", "3", "[]_{0,0}");
    test("[]_{0,3}", "4", "[0, 0, 0; 0, 0, 0; 0, 0, 0]");
    test("[]_{2,0}", "4", "[]_{2,0}");
    test("[0]", "1", "[0]");
    test("[0, 0; 0, 0]", "1", "[0, 0; 0, 0]");
    test("[3]", "7", "[1]");
    test("[3]", "6", "[3]");
    test("[4]", "6", "[2]");
    test("[0]", "6", "[0]");
    test("[2, 1]", "4", "[2, 1; 0, 2]");
    test("[2; 3]", "6", "[1; 0]");
    test("[0, 2; 0, 0; 0, 0]", "4", "[0, 0; 0, 2; 0, 0]");
    test("[4, 2; 2, 0]", "8", "[2, 0; 0, 2]");
    test("[1, 2; 3, 4]", "5", "[1, 0; 0, 1]");
    test("[1, 2; 2, 4]", "5", "[1, 2; 0, 0]");
    test("[2, 4; 4, 2]", "6", "[2, 4; 0, 0]");
    test("[3, 6; 6, 3]", "9", "[3, 6; 0, 0]");
    test("[2, 0, 1; 0, 2, 1]", "4", "[2, 0, 1; 0, 2, 1; 0, 0, 2]");
    test(
        "[6, 4, 2; 3, 0, 3; 0, 2, 4]",
        "12",
        "[3, 0, 3; 0, 2, 4; 0, 0, 0]",
    );
    test("[0, 0; 0, 3; 0, 0]", "9", "[0, 0; 0, 3; 0, 0]");
    test(
        "[1, 1, 1; 1, 1, 1; 1, 1, 1]",
        "2",
        "[1, 1, 1; 0, 0, 0; 0, 0, 0]",
    );
    test("[5, 3; 2, 7; 1, 4; 0, 0]", "8", "[1, 0; 0, 1; 0, 0; 0, 0]");
    test("[4, 6, 2]", "8", "[4, 2, 6; 0, 4, 4; 0, 0, 0]");
    test("[2, 3; 4, 6]", "12", "[2, 3; 0, 6]");
    test("[255, 128; 64, 32]", "256", "[1, 0; 0, 32]");
    test("[8]", "20", "[4]");
    test("[12, 1]", "28", "[4, 5; 0, 7]");
    test(
        "[590295810358705651712, 1; 0, 295147905179352825856]",
        "1180591620717411303424",
        "[590295810358705651712, 1; 0, 2]",
    );
    test(
        "[18446744073709551616, 27670116110564327424; 9223372036854775808, 0]",
        "110680464442257309696",
        "[9223372036854775808, 0; 0, 27670116110564327424]",
    );
    test(
        "[3541774862152233910271, 6; 3, 1180591620717411303424]",
        "10625324586456701730816",
        "[1, 0; 0, 2]",
    );
    test(
        "[9223372036854775808]",
        "18446744073709551616",
        "[9223372036854775808]",
    );
    test(
        "[9223372036854775808, 1]",
        "18446744073709551616",
        "[9223372036854775808, 1; 0, 2]",
    );
    test(
        "[100000000000000000005, 2; 10000000000000000000, 25]",
        "1000000000000000000000",
        "[5, 2; 0, 25]",
    );
    test("[]_{0,0}", "55340232221128654848", "[]_{0,0}");
    test("[]_{2,0}", "55340232221128654848", "[]_{2,0}");
    test("[]_{0,2}", "55340232221128654848", "[0, 0; 0, 0]");
    test("[0, 0; 0, 0]", "73786976294838206464", "[0, 0; 0, 0]");
    test("[0, 1; 1, 0]", "92233720368547758080", "[1, 0; 0, 1]");
    test(
        "[0, 36893488147419103232; 0, 0; 0, 0]",
        "73786976294838206464",
        "[0, 0; 0, 36893488147419103232; 0, 0]",
    );
    test(
        "[147573952589676412928]",
        "368934881474191032320",
        "[73786976294838206464]",
    );
    test(
        "[147573952589676412928, 1; 73786976294838206464, 3]",
        "368934881474191032320",
        "[73786976294838206464, 3; 0, 5]",
    );
}

#[test]
#[should_panic]
fn mod_strong_echelon_form_fail_1() {
    // The modulus is zero.
    let _ = NaturalMatrix::from_str("[]_{0,0}")
        .unwrap()
        .mod_strong_echelon_form(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_strong_echelon_form_fail_2() {
    // The matrix is not reduced.
    let _ = NaturalMatrix::from_str("[7]")
        .unwrap()
        .mod_strong_echelon_form(Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_strong_echelon_form_assign_fail() {
    let mut a = NaturalMatrix::from_str("[1, 7]").unwrap();
    a.mod_strong_echelon_form_assign(Natural::from(7u32));
}

#[test]
fn mod_strong_echelon_form_properties() {
    natural_matrix_natural_pair_gen_var_4().test_properties(|(a, m)| {
        let b = (&a).mod_strong_echelon_form(&m);
        assert!(b.is_valid());
        assert_eq!(a.clone().mod_strong_echelon_form(m.clone()), b);
        assert_eq!(a.clone().mod_strong_echelon_form(&m), b);
        assert_eq!((&a).mod_strong_echelon_form(m.clone()), b);
        let mut c = a.clone();
        c.mod_strong_echelon_form_assign(m.clone());
        assert_eq!(c, b);
        let mut c = a.clone();
        c.mod_strong_echelon_form_assign(&m);
        assert_eq!(c, b);
        assert_eq!(
            (b.height(), b.width()),
            (a.height().max(a.width()), a.width())
        );
        assert!(b.mod_is_reduced(&m));
        // The form is canonical: it is its own form, and so is that of any matrix with the same row
        // span.
        assert_eq!((&b).mod_strong_echelon_form(&m), b);
        assert_eq!(
            natural_matrix_mod_row_equivalent(&a, &m).mod_strong_echelon_form(&m),
            b
        );
        // The strong echelon form has the same nonzero rows as the Howell form, and its nonzero
        // rows' pivots are on the diagonal.
        let h = (&a).mod_howell_form(&m);
        let nonzero_rows = |x: &NaturalMatrix| -> Vec<Vec<_>> {
            {
                (0..x.height())
                    .map(|i| x.row_iter(i).cloned().collect::<Vec<_>>())
                    .filter(|r| r.iter().any(|y| *y != 0u32))
                    .collect()
            }
        };
        assert_eq!(nonzero_rows(&b), nonzero_rows(&h));
        for i in 0..b.height() {
            {
                if let Some(j) = b.row_iter(i).position(|x| *x != 0u32) {
                    {
                        assert_eq!(u64::exact_from(j), i);
                    }
                }
            }
        }
        if a_is_small(&a, &m) {
            assert_eq!(natural_matrix_mod_strong_echelon_form_naive(&a, &m), b);
        }
    });

    unsigned_matrix_unsigned_pair_gen_var_9().test_properties(|(a, m)| {
        // The `u64` elements give the same form as their `Natural` counterparts.
        assert_eq!(
            NaturalMatrix::from(a.clone()).mod_strong_echelon_form(Natural::from(m)),
            NaturalMatrix::from(a.mod_strong_echelon_form(m))
        );
    });
}
