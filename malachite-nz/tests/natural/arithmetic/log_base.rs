// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingLogBase, CeilingLogBase2, CheckedLogBase, CheckedLogBase2, DivExact, FloorLogBase,
    FloorLogBase2, Pow,
};
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::unsigned_pair_gen_var_24;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_gen_var_1, natural_gen_var_2, natural_pair_gen_var_3,
};
use malachite_nz::test_util::natural::arithmetic::log_base::{
    ceiling_log_base_by_squaring, ceiling_log_base_naive, checked_log_base_by_squaring,
    checked_log_base_naive, floor_log_base_by_squaring, floor_log_base_naive,
};
use std::str::FromStr;

#[test]
fn test_approx_log() {
    let test = |s, out| {
        assert_eq!(
            NiceFloat(Natural::from_str(s).unwrap().approx_log()),
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
}

#[test]
#[should_panic]
fn approx_log_fail() {
    Natural::ZERO.approx_log();
}

#[test]
fn test_floor_log_base() {
    let test = |n, base, out| {
        assert_eq!(
            Natural::from_str(n)
                .unwrap()
                .floor_log_base(&Natural::from_str(base).unwrap()),
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
}

#[test]
#[should_panic]
fn floor_log_base_fail_1() {
    Natural::ZERO.floor_log_base(&Natural::TWO);
}

#[test]
#[should_panic]
fn floor_log_base_fail_2() {
    Natural::ONE.floor_log_base(&Natural::ZERO);
}

#[test]
#[should_panic]
fn floor_log_base_fail_3() {
    Natural::ONE.floor_log_base(&Natural::ONE);
}

#[test]
fn test_ceiling_log_base() {
    let test = |n, base, out| {
        assert_eq!(
            Natural::from_str(n)
                .unwrap()
                .ceiling_log_base(&Natural::from_str(base).unwrap()),
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
}

#[test]
#[should_panic]
fn ceiling_log_base_fail_1() {
    Natural::ZERO.ceiling_log_base(&Natural::TWO);
}

#[test]
#[should_panic]
fn ceiling_log_base_fail_2() {
    Natural::ONE.ceiling_log_base(&Natural::ZERO);
}

#[test]
#[should_panic]
fn ceiling_log_base_fail_3() {
    Natural::ONE.ceiling_log_base(&Natural::ONE);
}

#[test]
fn test_checked_log_base() {
    let test = |n, base, out| {
        assert_eq!(
            Natural::from_str(n)
                .unwrap()
                .checked_log_base(&Natural::from_str(base).unwrap()),
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
}

#[test]
#[should_panic]
fn checked_log_base_fail_1() {
    Natural::ZERO.checked_log_base(&Natural::TWO);
}

#[test]
#[should_panic]
fn checked_log_base_fail_2() {
    Natural::ONE.checked_log_base(&Natural::ZERO);
}

#[test]
#[should_panic]
fn checked_log_base_fail_3() {
    Natural::ONE.checked_log_base(&Natural::ONE);
}

#[test]
fn approx_log_properties() {
    natural_gen_var_2().test_properties(|n| {
        let log = n.approx_log();
        assert!(log.is_sign_positive());
        assert!(log.is_finite());
        assert!(!log.is_nan());
    });
}

#[test]
fn floor_log_base_properties() {
    natural_pair_gen_var_3().test_properties(|(n, base)| {
        let floor_log = n.floor_log_base(&base);
        assert_eq!(floor_log_base_naive(&n, &base), floor_log);
        assert_eq!(floor_log_base_by_squaring(&n, &base), floor_log);
        assert_eq!(floor_log == 0, n < base);

        let power = (&base).pow(floor_log);
        assert!(power <= n);
        assert!(&power * &base > n);

        let ceiling_log = n.ceiling_log_base(&base);
        if power == n {
            assert_eq!(ceiling_log, floor_log);
        } else {
            assert_eq!(ceiling_log, floor_log + 1);
        }
    });

    natural_gen_var_2().test_properties(|n| {
        assert_eq!(n.floor_log_base(&Natural::TWO), n.floor_log_base_2());
    });

    natural_gen_var_1().test_properties(|base| {
        assert_eq!(Natural::ONE.floor_log_base(&base), 0);
    });

    unsigned_pair_gen_var_24::<Limb, Limb>().test_properties(|(n, base)| {
        assert_eq!(
            n.floor_log_base(base),
            Natural::from(n).floor_log_base(&Natural::from(base))
        );
    });
}

#[test]
fn ceiling_log_base_properties() {
    natural_pair_gen_var_3().test_properties(|(n, base)| {
        let ceiling_log = n.ceiling_log_base(&base);
        assert_eq!(ceiling_log_base_naive(&n, &base), ceiling_log);
        assert_eq!(ceiling_log_base_by_squaring(&n, &base), ceiling_log);
        assert_eq!(ceiling_log == 0, n == Natural::ONE);

        let power = (&base).pow(ceiling_log);
        assert!(power >= n);
        if power != 1 {
            assert!((&power).div_exact(&base) < n);
        }

        let floor_log = n.floor_log_base(&base);
        if power == n {
            assert_eq!(floor_log, ceiling_log);
        } else {
            assert_eq!(floor_log, ceiling_log - 1);
        }
    });

    natural_gen_var_2().test_properties(|n| {
        assert_eq!(n.ceiling_log_base(&Natural::TWO), n.ceiling_log_base_2());
    });

    natural_gen_var_1().test_properties(|base| {
        assert_eq!(Natural::ONE.ceiling_log_base(&base), 0);
    });

    unsigned_pair_gen_var_24::<Limb, Limb>().test_properties(|(n, base)| {
        assert_eq!(
            n.ceiling_log_base(base),
            Natural::from(n).ceiling_log_base(&Natural::from(base))
        );
    });
}

#[test]
fn checked_log_base_properties() {
    natural_pair_gen_var_3().test_properties(|(n, base)| {
        let checked_log = n.checked_log_base(&base);
        assert_eq!(checked_log_base_naive(&n, &base), checked_log);
        assert_eq!(checked_log_base_by_squaring(&n, &base), checked_log);
        if let Some(log) = checked_log {
            assert_eq!((&base).pow(log), n);
            assert_eq!(log == 0, n == Natural::ONE);
            assert_eq!(n.floor_log_base(&base), log);
            assert_eq!(n.ceiling_log_base(&base), log);
        }
    });

    natural_gen_var_2().test_properties(|n| {
        assert_eq!(n.checked_log_base(&Natural::TWO), n.checked_log_base_2());
    });

    natural_gen_var_1().test_properties(|base| {
        assert_eq!(Natural::ONE.checked_log_base(&base), Some(0));
    });

    unsigned_pair_gen_var_24::<Limb, Limb>().test_properties(|(n, base)| {
        assert_eq!(
            n.checked_log_base(base),
            Natural::from(n).checked_log_base(&Natural::from(base))
        );
    });
}
