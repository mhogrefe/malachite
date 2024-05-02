// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingLogBase, CeilingLogBase2, CeilingLogBasePowerOf2, CheckedLogBase, CheckedLogBase2,
    CheckedLogBasePowerOf2, DivisibleBy, FloorLogBase, FloorLogBase2, FloorLogBasePowerOf2,
    IsPowerOf2, PowerOf2, Reciprocal,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::signed_gen_var_12;
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_8;
use malachite_q::test_util::generators::{rational_gen_var_2, rational_signed_pair_gen_var_5};
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_floor_log_base_power_of_2() {
    let test = |n, pow, out| {
        assert_eq!(
            Rational::from_str(n)
                .unwrap()
                .floor_log_base_power_of_2(pow),
            out
        );
    };
    test("1", 1, 0);
    test("1", 2, 0);
    test("1", 5, 0);
    test("100", 1, 6);
    test("100", 2, 3);
    test("100", 5, 1);
    test("1000000000000", 1, 39);
    test("1000000000000", 2, 19);
    test("1000000000000", 5, 7);
    test("1000000000000", 5, 7);
    test("1", -1, 0);
    test("1", -2, 0);
    test("1", -5, 0);
    test("100", -1, -7);
    test("100", -2, -4);
    test("100", -5, -2);
    test("1000000000000", -1, -40);
    test("1000000000000", -2, -20);
    test("1000000000000", -5, -8);
    test("1000000000000", -5, -8);
    test("1/2", 1, -1);
    test("1/3", 1, -2);
    test("1/4", 1, -2);
    test("1/5", 1, -3);
    test("1/6", 1, -3);
    test("1/7", 1, -3);
    test("1/8", 1, -3);
    test("1/9", 1, -4);
    test("1/2", 2, -1);
    test("1/3", 2, -1);
    test("1/4", 2, -1);
    test("1/5", 2, -2);
    test("1/6", 2, -2);
    test("1/7", 2, -2);
    test("1/8", 2, -2);
    test("1/9", 2, -2);
    test("1/2", -1, 1);
    test("1/3", -1, 1);
    test("1/4", -1, 2);
    test("1/5", -1, 2);
    test("1/6", -1, 2);
    test("1/7", -1, 2);
    test("1/8", -1, 3);
    test("1/9", -1, 3);
    test("1/2", -2, 0);
    test("1/3", -2, 0);
    test("1/4", -2, 1);
    test("1/5", -2, 1);
    test("1/6", -2, 1);
    test("1/7", -2, 1);
    test("1/8", -2, 1);
    test("1/9", -2, 1);
}

#[test]
#[should_panic]
fn floor_log_base_power_of_2_fail_1() {
    Rational::ZERO.floor_log_base_power_of_2(1);
}

#[test]
#[should_panic]
fn floor_log_base_power_of_2_fail_2() {
    Rational::ONE.floor_log_base_power_of_2(0);
}

#[test]
#[should_panic]
fn floor_log_base_power_of_2_fail_3() {
    Rational::NEGATIVE_ONE.floor_log_base_power_of_2(1);
}

#[test]
fn test_ceiling_log_base_power_of_2() {
    let test = |n, pow, out| {
        assert_eq!(
            Rational::from_str(n)
                .unwrap()
                .ceiling_log_base_power_of_2(pow),
            out
        );
    };
    test("1", 1, 0);
    test("1", 2, 0);
    test("1", 5, 0);
    test("100", 1, 7);
    test("100", 2, 4);
    test("100", 5, 2);
    test("1000000000000", 1, 40);
    test("1000000000000", 2, 20);
    test("1000000000000", 5, 8);
    test("1000000000000", 5, 8);
    test("1", -1, 0);
    test("1", -2, 0);
    test("1", -5, 0);
    test("100", -1, -6);
    test("100", -2, -3);
    test("100", -5, -1);
    test("1000000000000", -1, -39);
    test("1000000000000", -2, -19);
    test("1000000000000", -5, -7);
    test("1000000000000", -5, -7);
    test("1/2", 1, -1);
    test("1/3", 1, -1);
    test("1/4", 1, -2);
    test("1/5", 1, -2);
    test("1/6", 1, -2);
    test("1/7", 1, -2);
    test("1/8", 1, -3);
    test("1/9", 1, -3);
    test("1/2", 2, 0);
    test("1/3", 2, 0);
    test("1/4", 2, -1);
    test("1/5", 2, -1);
    test("1/6", 2, -1);
    test("1/7", 2, -1);
    test("1/8", 2, -1);
    test("1/9", 2, -1);
    test("1/2", -1, 1);
    test("1/3", -1, 2);
    test("1/4", -1, 2);
    test("1/5", -1, 3);
    test("1/6", -1, 3);
    test("1/7", -1, 3);
    test("1/8", -1, 3);
    test("1/9", -1, 4);
    test("1/2", -2, 1);
    test("1/3", -2, 1);
    test("1/4", -2, 1);
    test("1/5", -2, 2);
    test("1/6", -2, 2);
    test("1/7", -2, 2);
    test("1/8", -2, 2);
    test("1/9", -2, 2);
}

#[test]
#[should_panic]
fn ceiling_log_base_power_of_2_fail_1() {
    Rational::ZERO.ceiling_log_base_power_of_2(1);
}

#[test]
#[should_panic]
fn ceiling_log_base_power_of_2_fail_2() {
    Rational::ONE.ceiling_log_base_power_of_2(0);
}

#[test]
#[should_panic]
fn ceiling_log_base_power_of_2_fail_3() {
    Rational::NEGATIVE_ONE.ceiling_log_base_power_of_2(1);
}

#[test]
fn test_checked_log_base_power_of_2() {
    let test = |n, pow, out| {
        assert_eq!(
            Rational::from_str(n)
                .unwrap()
                .checked_log_base_power_of_2(pow),
            out
        );
    };
    test("1", 1, Some(0));
    test("1", 2, Some(0));
    test("1", 5, Some(0));
    test("100", 1, None);
    test("100", 2, None);
    test("100", 5, None);
    test("1000000000000", 1, None);
    test("1000000000000", 2, None);
    test("1000000000000", 5, None);
    test("1000000000000", 5, None);
    test("1", -1, Some(0));
    test("1", -2, Some(0));
    test("1", -5, Some(0));
    test("100", -1, None);
    test("100", -2, None);
    test("100", -5, None);
    test("1000000000000", -1, None);
    test("1000000000000", -2, None);
    test("1000000000000", -5, None);
    test("1000000000000", -5, None);
    test("1/2", 1, Some(-1));
    test("1/3", 1, None);
    test("1/4", 1, Some(-2));
    test("1/5", 1, None);
    test("1/6", 1, None);
    test("1/7", 1, None);
    test("1/8", 1, Some(-3));
    test("1/9", 1, None);
    test("1/2", 2, None);
    test("1/3", 2, None);
    test("1/4", 2, Some(-1));
    test("1/5", 2, None);
    test("1/6", 2, None);
    test("1/7", 2, None);
    test("1/8", 2, None);
    test("1/9", 2, None);
    test("1/2", -1, Some(1));
    test("1/3", -1, None);
    test("1/4", -1, Some(2));
    test("1/5", -1, None);
    test("1/6", -1, None);
    test("1/7", -1, None);
    test("1/8", -1, Some(3));
    test("1/9", -1, None);
    test("1/2", -2, None);
    test("1/3", -2, None);
    test("1/4", -2, Some(1));
    test("1/5", -2, None);
    test("1/6", -2, None);
    test("1/7", -2, None);
    test("1/8", -2, None);
    test("1/9", -2, None);
}

#[test]
#[should_panic]
fn checked_log_base_power_of_2_fail_1() {
    Rational::ZERO.checked_log_base_power_of_2(1);
}

#[test]
#[should_panic]
fn checked_log_base_power_of_2_fail_2() {
    Rational::ONE.checked_log_base_power_of_2(0);
}

#[test]
#[should_panic]
fn checked_log_base_power_of_2_fail_3() {
    Rational::NEGATIVE_ONE.checked_log_base_power_of_2(1);
}

#[test]
fn floor_log_base_power_of_2_properties() {
    rational_signed_pair_gen_var_5().test_properties(|(n, pow)| {
        let floor_log = n.floor_log_base_power_of_2(pow);
        assert_eq!(n.floor_log_base(&Rational::power_of_2(pow)), floor_log);

        let product = floor_log * pow;
        if pow >= 0 {
            assert!(Rational::power_of_2(product) <= n);
            assert!(Rational::power_of_2(product + pow) > n);
        } else {
            assert!(Rational::power_of_2(product) >= n);
            assert!(Rational::power_of_2(product + pow) < n);
        }

        let ceiling_log = n.ceiling_log_base_power_of_2(pow);
        if n.is_power_of_2() && n.checked_log_base_2().unwrap().divisible_by(pow) {
            assert_eq!(ceiling_log, floor_log);
        } else {
            assert_eq!(ceiling_log, floor_log + 1);
        }

        assert_eq!(n.floor_log_base_power_of_2(-pow), -ceiling_log);
        assert_eq!(n.reciprocal().floor_log_base_power_of_2(pow), -ceiling_log);
    });

    rational_gen_var_2().test_properties(|n| {
        assert_eq!(n.floor_log_base_power_of_2(1), n.floor_log_base_2());
    });

    signed_gen_var_12().test_properties(|pow| {
        assert_eq!(Rational::ONE.floor_log_base_power_of_2(pow), 0);
    });

    natural_unsigned_pair_gen_var_8().test_properties(|(n, pow)| {
        assert_eq!(
            i64::exact_from(n.floor_log_base_power_of_2(pow)),
            Rational::from(n).floor_log_base_power_of_2(i64::exact_from(pow))
        );
    });
}

#[test]
fn ceiling_log_base_power_of_2_properties() {
    rational_signed_pair_gen_var_5().test_properties(|(n, pow)| {
        let ceiling_log = n.ceiling_log_base_power_of_2(pow);
        assert_eq!(n.ceiling_log_base(&Rational::power_of_2(pow)), ceiling_log);

        let product = ceiling_log * pow;
        if pow >= 0 {
            assert!(Rational::power_of_2(product) >= n);
            assert!(Rational::power_of_2(product - pow) < n);
        } else {
            assert!(Rational::power_of_2(product) <= n);
            assert!(Rational::power_of_2(product - pow) > n);
        }

        let floor_log = n.floor_log_base_power_of_2(pow);
        if n.is_power_of_2() && n.checked_log_base_2().unwrap().divisible_by(pow) {
            assert_eq!(floor_log, ceiling_log);
        } else {
            assert_eq!(floor_log, ceiling_log - 1);
        }

        assert_eq!(n.ceiling_log_base_power_of_2(-pow), -floor_log);
        assert_eq!(n.reciprocal().ceiling_log_base_power_of_2(pow), -floor_log);
    });

    rational_gen_var_2().test_properties(|n| {
        assert_eq!(n.ceiling_log_base_power_of_2(1), n.ceiling_log_base_2());
    });

    signed_gen_var_12().test_properties(|pow| {
        assert_eq!(Rational::ONE.ceiling_log_base_power_of_2(pow), 0);
    });

    natural_unsigned_pair_gen_var_8().test_properties(|(n, pow)| {
        assert_eq!(
            i64::exact_from(n.ceiling_log_base_power_of_2(pow)),
            Rational::from(n).ceiling_log_base_power_of_2(i64::exact_from(pow))
        );
    });
}

#[test]
fn checked_log_base_power_of_2_properties() {
    rational_signed_pair_gen_var_5().test_properties(|(n, pow)| {
        let checked_log = n.checked_log_base_power_of_2(pow);
        assert_eq!(n.checked_log_base(&Rational::power_of_2(pow)), checked_log);
        assert_eq!(
            checked_log.is_some(),
            n.is_power_of_2() && n.checked_log_base_2().unwrap().divisible_by(pow)
        );
        if let Some(log) = checked_log {
            assert_eq!(Rational::power_of_2(log * pow), n);
            assert_eq!(log == 0, n == Rational::ONE);
            assert_eq!(n.floor_log_base_power_of_2(pow), log);
            assert_eq!(n.ceiling_log_base_power_of_2(pow), log);
            assert_eq!(n.checked_log_base_power_of_2(-pow), Some(-log));
            assert_eq!(n.reciprocal().checked_log_base_power_of_2(pow), Some(-log));
        }
    });

    rational_gen_var_2().test_properties(|n| {
        assert_eq!(n.checked_log_base_power_of_2(1), n.checked_log_base_2());
    });

    signed_gen_var_12().test_properties(|pow| {
        assert_eq!(Rational::ONE.checked_log_base_power_of_2(pow), Some(0));
    });

    natural_unsigned_pair_gen_var_8().test_properties(|(n, pow)| {
        assert_eq!(
            n.checked_log_base_power_of_2(pow).map(i64::exact_from),
            Rational::from(n).checked_log_base_power_of_2(i64::exact_from(pow))
        );
    });
}
