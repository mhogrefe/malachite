// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingLogBase, CeilingLogBase2, CheckedLogBase, CheckedLogBase2, FloorLogBase, FloorLogBase2,
    Pow,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_nz::test_util::generators::natural_pair_gen_var_3;
use malachite_q::test_util::generators::{
    rational_gen_var_2, rational_gen_var_8, rational_pair_gen_var_7,
};
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_approx_log() {
    let test = |s, out| {
        assert_eq!(
            NiceFloat(Rational::from_str(s).unwrap().approx_log()),
            NiceFloat(out)
        );
    };
    test("1", 0.0);
    test("2", std::f64::consts::LN_2);
    test("3", 1.0986122886681096);
    test("10", 2.3025850929940455);
    test("100", 4.605170185988091);
    test("1000", 6.907755278982137);
    test(
        "1000000000000000000000000000000000000000000000000000000000000",
        138.15510557964274,
    );
    test("1/2", -std::f64::consts::LN_2);
    test("1/3", -1.0986122886681098);
    test("22/7", 1.1451323043030026);
    test("936851431250/1397", 20.323708189458696);
}

#[test]
#[should_panic]
fn approx_log_fail() {
    Rational::ZERO.approx_log();
}

#[test]
fn test_floor_log_base() {
    let test = |n, base, out| {
        assert_eq!(
            Rational::from_str(n)
                .unwrap()
                .floor_log_base(&Rational::from_str(base).unwrap()),
            out
        );
    };
    test("1", "2", 0);
    test("1", "5", 0);
    test("2", "2", 1);
    test("2", "3", 0);
    test("3", "2", 1);
    test("3", "3", 1);
    test("3", "4", 0);
    test("100", "2", 6);
    test("100", "3", 4);
    test("100", "4", 3);
    test("100", "5", 2);
    test("100", "10", 2);
    test("100", "11", 1);
    test(
        "999999999999999999999999999999999999999999999999999999999999",
        "10",
        59,
    );
    test(
        "1000000000000000000000000000000000000000000000000000000000000",
        "10",
        60,
    );
    test(
        "1000000000000000000000000000000000000000000000000000000000001",
        "10",
        60,
    );
    test("1/2", "2", -1);
    test("1/3", "2", -2);
    test("1/2", "3", -1);
    test("1/3", "3", -1);
    test("22/7", "3", 1);
    test("1/64", "4", -3);
    test("22/7", "10", 0);
    test("936851431250/1397", "10", 8);
    test("1/2", "1/2", 1);
    test("1/3", "1/2", 1);
    test("1/2", "1/3", 0);
    test("1/3", "1/3", 1);
    test("22/7", "1/3", -2);
    test("1/64", "1/4", 3);
    test("22/7", "1/10", -1);
    test("936851431250/1397", "1/10", -9);
    test("936851431250/1397", "22/7", 17);
    test("100", "101/100", 462);
    test("100", "100/101", -463);
    test("5153632/16807", "22/7", 5);
}

#[test]
#[should_panic]
fn floor_log_base_fail_1() {
    Rational::ZERO.floor_log_base(&Rational::TWO);
}

#[test]
#[should_panic]
fn floor_log_base_fail_2() {
    Rational::ONE.floor_log_base(&Rational::ZERO);
}

#[test]
#[should_panic]
fn floor_log_base_fail_3() {
    Rational::ONE.floor_log_base(&Rational::ONE);
}

#[test]
#[should_panic]
fn floor_log_base_fail_4() {
    Rational::NEGATIVE_ONE.floor_log_base(&Rational::TWO);
}

#[test]
fn test_ceiling_log_base() {
    let test = |n, base, out| {
        assert_eq!(
            Rational::from_str(n)
                .unwrap()
                .ceiling_log_base(&Rational::from_str(base).unwrap()),
            out
        );
    };
    test("1", "2", 0);
    test("1", "5", 0);
    test("2", "2", 1);
    test("2", "3", 1);
    test("3", "2", 2);
    test("3", "3", 1);
    test("3", "4", 1);
    test("100", "2", 7);
    test("100", "3", 5);
    test("100", "4", 4);
    test("100", "5", 3);
    test("100", "10", 2);
    test("100", "11", 2);
    test(
        "999999999999999999999999999999999999999999999999999999999999",
        "10",
        60,
    );
    test(
        "1000000000000000000000000000000000000000000000000000000000000",
        "10",
        60,
    );
    test(
        "1000000000000000000000000000000000000000000000000000000000001",
        "10",
        61,
    );
    test("1/2", "2", -1);
    test("1/3", "2", -1);
    test("1/2", "3", 0);
    test("1/3", "3", -1);
    test("22/7", "3", 2);
    test("1/64", "4", -3);
    test("22/7", "10", 1);
    test("936851431250/1397", "10", 9);
    test("1/2", "1/2", 1);
    test("1/3", "1/2", 2);
    test("1/2", "1/3", 1);
    test("1/3", "1/3", 1);
    test("22/7", "1/3", -1);
    test("1/64", "1/4", 3);
    test("22/7", "1/10", 0);
    test("936851431250/1397", "1/10", -8);
    test("936851431250/1397", "22/7", 18);
    test("100", "101/100", 463);
    test("100", "100/101", -462);
    test("5153632/16807", "22/7", 5);
}

#[test]
#[should_panic]
fn ceiling_log_base_fail_1() {
    Rational::ZERO.ceiling_log_base(&Rational::TWO);
}

#[test]
#[should_panic]
fn ceiling_log_base_fail_2() {
    Rational::ONE.ceiling_log_base(&Rational::ZERO);
}

#[test]
#[should_panic]
fn ceiling_log_base_fail_3() {
    Rational::ONE.ceiling_log_base(&Rational::ONE);
}

#[test]
#[should_panic]
fn ceiling_log_base_fail_4() {
    Rational::NEGATIVE_ONE.ceiling_log_base(&Rational::TWO);
}

#[test]
fn test_checked_log_base() {
    let test = |n, base, out| {
        assert_eq!(
            Rational::from_str(n)
                .unwrap()
                .checked_log_base(&Rational::from_str(base).unwrap()),
            out
        );
    };
    test("1", "2", Some(0));
    test("1", "5", Some(0));
    test("2", "2", Some(1));
    test("2", "3", None);
    test("3", "2", None);
    test("3", "3", Some(1));
    test("3", "4", None);
    test("100", "2", None);
    test("100", "3", None);
    test("100", "4", None);
    test("100", "5", None);
    test("100", "10", Some(2));
    test("100", "11", None);
    test(
        "999999999999999999999999999999999999999999999999999999999999",
        "10",
        None,
    );
    test(
        "1000000000000000000000000000000000000000000000000000000000000",
        "10",
        Some(60),
    );
    test(
        "1000000000000000000000000000000000000000000000000000000000001",
        "10",
        None,
    );
    test("1/2", "2", Some(-1));
    test("1/3", "2", None);
    test("1/2", "3", None);
    test("1/3", "3", Some(-1));
    test("22/7", "3", None);
    test("1/64", "4", Some(-3));
    test("22/7", "10", None);
    test("936851431250/1397", "10", None);
    test("1/2", "1/2", Some(1));
    test("1/3", "1/2", None);
    test("1/2", "1/3", None);
    test("1/3", "1/3", Some(1));
    test("22/7", "1/3", None);
    test("1/64", "1/4", Some(3));
    test("22/7", "1/10", None);
    test("936851431250/1397", "1/10", None);
    test("936851431250/1397", "22/7", None);
    test("100", "101/100", None);
    test("100", "100/101", None);
    test("5153632/16807", "22/7", Some(5));
}

#[test]
#[should_panic]
fn checked_log_base_fail_1() {
    Rational::ZERO.checked_log_base(&Rational::TWO);
}

#[test]
#[should_panic]
fn checked_log_base_fail_2() {
    Rational::ONE.checked_log_base(&Rational::ZERO);
}

#[test]
#[should_panic]
fn checked_log_base_fail_3() {
    Rational::ONE.checked_log_base(&Rational::ONE);
}

#[test]
#[should_panic]
fn checked_log_base_fail_4() {
    Rational::NEGATIVE_ONE.checked_log_base(&Rational::TWO);
}

#[test]
fn approx_log_properties() {
    rational_gen_var_2().test_properties(|n| {
        let log = n.approx_log();
        assert!(log.is_finite());
        assert!(!log.is_nan());
    });
}

#[test]
fn floor_log_base_properties() {
    rational_pair_gen_var_7().test_properties(|(n, base)| {
        let floor_log = n.floor_log_base(&base);

        let power = (&base).pow(floor_log);
        if base >= 1u32 {
            assert!(power <= n);
            assert!(&power * &base > n);
        } else {
            assert!(power >= n);
            assert!(&power * &base < n);
        }

        let ceiling_log = n.ceiling_log_base(&base);
        if power == n {
            assert_eq!(ceiling_log, floor_log);
        } else {
            assert_eq!(ceiling_log, floor_log + 1);
        }
    });

    rational_gen_var_2().test_properties(|n| {
        assert_eq!(n.floor_log_base(&Rational::TWO), n.floor_log_base_2());
    });

    rational_gen_var_8().test_properties(|base| {
        assert_eq!(Rational::ONE.floor_log_base(&base), 0);
    });

    natural_pair_gen_var_3().test_properties(|(n, base)| {
        assert_eq!(
            n.floor_log_base(&base),
            u64::exact_from(Rational::from(n).floor_log_base(&Rational::from(base)))
        );
    });
}

#[test]
fn ceiling_log_base_properties() {
    rational_pair_gen_var_7().test_properties(|(n, base)| {
        let ceiling_log = n.ceiling_log_base(&base);

        let power = (&base).pow(ceiling_log);
        if base >= 1u32 {
            assert!(power >= n);
            assert!(&power / &base < n);
        } else {
            assert!(power <= n);
            assert!(&power / &base > n);
        }

        let floor_log = n.floor_log_base(&base);
        if power == n {
            assert_eq!(floor_log, ceiling_log);
        } else {
            assert_eq!(floor_log, ceiling_log - 1);
        }
    });

    rational_gen_var_2().test_properties(|n| {
        assert_eq!(n.ceiling_log_base(&Rational::TWO), n.ceiling_log_base_2());
    });

    rational_gen_var_8().test_properties(|base| {
        assert_eq!(Rational::ONE.ceiling_log_base(&base), 0);
    });

    natural_pair_gen_var_3().test_properties(|(n, base)| {
        assert_eq!(
            n.ceiling_log_base(&base),
            u64::exact_from(Rational::from(n).ceiling_log_base(&Rational::from(base)))
        );
    });
}

#[test]
fn checked_log_base_properties() {
    rational_pair_gen_var_7().test_properties(|(n, base)| {
        let checked_log = n.checked_log_base(&base);
        if let Some(log) = checked_log {
            assert_eq!((&base).pow(log), n);
            assert_eq!(n.floor_log_base(&base), log);
            assert_eq!(n.ceiling_log_base(&base), log);
        }
    });

    rational_gen_var_2().test_properties(|n| {
        assert_eq!(n.checked_log_base(&Rational::TWO), n.checked_log_base_2());
    });

    rational_gen_var_8().test_properties(|base| {
        assert_eq!(Rational::ONE.checked_log_base(&base), Some(0));
    });

    natural_pair_gen_var_3().test_properties(|(n, base)| {
        assert_eq!(
            n.checked_log_base(&base),
            Rational::from(n)
                .checked_log_base(&Rational::from(base))
                .map(u64::exact_from)
        );
    });
}
