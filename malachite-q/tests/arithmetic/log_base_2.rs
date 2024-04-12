// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    Abs, CeilingLogBase2, CheckedLogBase2, FloorLogBase2, IsPowerOf2, PowerOf2,
};
use malachite_base::num::basic::traits::{NegativeOne, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::test_util::generators::natural_gen_var_2;
use malachite_q::test_util::generators::{rational_gen_var_1, rational_gen_var_2};
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_floor_log_base_2_abs() {
    let test = |n, out| {
        assert_eq!(Rational::from_str(n).unwrap().floor_log_base_2_abs(), out);
    };
    test("1", 0);
    test("100", 6);
    test("1000000000000", 39);
    test("4294967295", 31);
    test("4294967296", 32);
    test("4294967297", 32);
    test("22/7", 1);
    test("936851431250/1397", 29);
    test("1/1000000000000", -40);
    test("1/4294967295", -32);
    test("1/4294967296", -32);
    test("1/4294967297", -33);
    test("1/2", -1);
    test("1/3", -2);
    test("1/4", -2);
    test("1/5", -3);
    test("1/6", -3);
    test("1/7", -3);
    test("1/8", -3);
    test("1/9", -4);

    test("-1", 0);
    test("-100", 6);
    test("-1000000000000", 39);
    test("-4294967295", 31);
    test("-4294967296", 32);
    test("-4294967297", 32);
    test("-22/7", 1);
    test("-936851431250/1397", 29);
    test("-1/1000000000000", -40);
    test("-1/4294967295", -32);
    test("-1/4294967296", -32);
    test("-1/4294967297", -33);
    test("-1/2", -1);
    test("-1/3", -2);
    test("-1/4", -2);
    test("-1/5", -3);
    test("-1/6", -3);
    test("-1/7", -3);
    test("-1/8", -3);
    test("-1/9", -4);
}

#[test]
#[should_panic]
fn floor_log_base_2_abs_fail() {
    Rational::ZERO.floor_log_base_2_abs();
}

#[test]
fn test_ceiling_log_base_2_abs() {
    let test = |n, out| {
        assert_eq!(Rational::from_str(n).unwrap().ceiling_log_base_2_abs(), out);
    };
    test("1", 0);
    test("100", 7);
    test("1000000000000", 40);
    test("4294967295", 32);
    test("4294967296", 32);
    test("4294967297", 33);
    test("22/7", 2);
    test("936851431250/1397", 30);
    test("1/1000000000000", -39);
    test("1/4294967295", -31);
    test("1/4294967296", -32);
    test("1/4294967297", -32);
    test("1/2", -1);
    test("1/3", -1);
    test("1/4", -2);
    test("1/5", -2);
    test("1/6", -2);
    test("1/7", -2);
    test("1/8", -3);
    test("1/9", -3);

    test("-1", 0);
    test("-100", 7);
    test("-1000000000000", 40);
    test("-4294967295", 32);
    test("-4294967296", 32);
    test("-4294967297", 33);
    test("-22/7", 2);
    test("-936851431250/1397", 30);
    test("-1/1000000000000", -39);
    test("-1/4294967295", -31);
    test("-1/4294967296", -32);
    test("-1/4294967297", -32);
    test("-1/2", -1);
    test("-1/3", -1);
    test("-1/4", -2);
    test("-1/5", -2);
    test("-1/6", -2);
    test("-1/7", -2);
    test("-1/8", -3);
    test("-1/9", -3);
}

#[test]
#[should_panic]
fn ceiling_log_base_2_abs_fail() {
    Rational::ZERO.ceiling_log_base_2_abs();
}

#[test]
fn test_floor_log_base_2() {
    let test = |n, out| {
        assert_eq!(Rational::from_str(n).unwrap().floor_log_base_2(), out);
    };
    test("1", 0);
    test("100", 6);
    test("1000000000000", 39);
    test("4294967295", 31);
    test("4294967296", 32);
    test("4294967297", 32);
    test("22/7", 1);
    test("936851431250/1397", 29);
    test("1/1000000000000", -40);
    test("1/4294967295", -32);
    test("1/4294967296", -32);
    test("1/4294967297", -33);
    test("1/2", -1);
    test("1/3", -2);
    test("1/4", -2);
    test("1/5", -3);
    test("1/6", -3);
    test("1/7", -3);
    test("1/8", -3);
    test("1/9", -4);
}

#[test]
#[should_panic]
fn floor_log_base_2_fail_1() {
    Rational::ZERO.floor_log_base_2();
}

#[test]
#[should_panic]
fn floor_log_base_2_fail_2() {
    Rational::NEGATIVE_ONE.floor_log_base_2();
}

#[test]
fn test_ceiling_log_base_2() {
    let test = |n, out| {
        assert_eq!(Rational::from_str(n).unwrap().ceiling_log_base_2(), out);
    };
    test("1", 0);
    test("100", 7);
    test("1000000000000", 40);
    test("4294967295", 32);
    test("4294967296", 32);
    test("4294967297", 33);
    test("22/7", 2);
    test("936851431250/1397", 30);
    test("1/1000000000000", -39);
    test("1/4294967295", -31);
    test("1/4294967296", -32);
    test("1/4294967297", -32);
    test("1/2", -1);
    test("1/3", -1);
    test("1/4", -2);
    test("1/5", -2);
    test("1/6", -2);
    test("1/7", -2);
    test("1/8", -3);
    test("1/9", -3);
}

#[test]
#[should_panic]
fn ceiling_log_base_2_fail() {
    Rational::ZERO.ceiling_log_base_2();
}

#[test]
#[should_panic]
fn ceiling_log_base_2_fail_2() {
    Rational::NEGATIVE_ONE.ceiling_log_base_2();
}

#[test]
fn test_checked_log_base_2() {
    let test = |n, out| {
        assert_eq!(Rational::from_str(n).unwrap().checked_log_base_2(), out);
    };
    test("1", Some(0));
    test("100", None);
    test("1000000000000", None);
    test("4294967295", None);
    test("4294967296", Some(32));
    test("4294967297", None);
    test("22/7", None);
    test("936851431250/1397", None);
    test("1/1000000000000", None);
    test("1/4294967295", None);
    test("1/4294967296", Some(-32));
    test("1/4294967297", None);
    test("1/2", Some(-1));
    test("1/3", None);
    test("1/4", Some(-2));
    test("1/5", None);
    test("1/6", None);
    test("1/7", None);
    test("1/8", Some(-3));
    test("1/9", None);
}

#[test]
#[should_panic]
fn checked_log_base_2_fail() {
    Rational::ZERO.checked_log_base_2();
}

#[test]
fn floor_log_base_2_abs_properties() {
    rational_gen_var_1().test_properties(|x| {
        let floor_log_base_2 = x.floor_log_base_2_abs();
        assert_eq!((&x).abs().floor_log_base_2(), floor_log_base_2);
        assert!(Rational::power_of_2(floor_log_base_2).le_abs(&x));
        assert!(x.lt_abs(&Rational::power_of_2(floor_log_base_2 + 1)));
    });
}

#[test]
fn ceiling_log_base_2_abs_properties() {
    rational_gen_var_1().test_properties(|x| {
        let ceiling_log_base_2 = x.ceiling_log_base_2_abs();
        assert_eq!((&x).abs().ceiling_log_base_2(), ceiling_log_base_2);
        assert!(Rational::power_of_2(ceiling_log_base_2 - 1).lt_abs(&x));
        assert!(x.le_abs(&Rational::power_of_2(ceiling_log_base_2)));
    });
}

#[test]
fn floor_log_base_2_properties() {
    rational_gen_var_2().test_properties(|x| {
        let floor_log_base_2 = x.floor_log_base_2();
        assert!(Rational::power_of_2(floor_log_base_2) <= x);
        assert!(x < Rational::power_of_2(floor_log_base_2 + 1));
    });

    natural_gen_var_2().test_properties(|n| {
        assert_eq!(
            i64::exact_from((&n).floor_log_base_2()),
            Rational::from(n).floor_log_base_2()
        );
    });
}

#[test]
fn ceiling_log_base_2_properties() {
    rational_gen_var_2().test_properties(|x| {
        let ceiling_log_base_2 = x.ceiling_log_base_2();
        assert!(Rational::power_of_2(ceiling_log_base_2 - 1) < x);
        assert!(x <= Rational::power_of_2(ceiling_log_base_2));
    });

    natural_gen_var_2().test_properties(|n| {
        assert_eq!(
            i64::exact_from((&n).ceiling_log_base_2()),
            Rational::from(n).ceiling_log_base_2()
        );
    });
}

#[test]
fn checked_log_base_2_properties() {
    rational_gen_var_2().test_properties(|x| {
        let checked_log_base_2 = x.checked_log_base_2();
        assert_eq!(checked_log_base_2.is_some(), x.is_power_of_2());
        if let Some(log_base_2) = checked_log_base_2 {
            assert_eq!(x.floor_log_base_2(), log_base_2);
            assert_eq!(x.ceiling_log_base_2(), log_base_2);
            assert_eq!(Rational::power_of_2(log_base_2), x);
        }
    });

    natural_gen_var_2().test_properties(|n| {
        assert_eq!(
            (&n).checked_log_base_2().map(i64::exact_from),
            Rational::from(n).checked_log_base_2()
        );
    });
}
