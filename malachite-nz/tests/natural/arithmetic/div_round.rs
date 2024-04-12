// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingDivNegMod, DivRound, DivRoundAssign, DivisibleBy,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_unsigned_rounding_mode_triple_gen_var_1,
    unsigned_vec_unsigned_rounding_mode_triple_gen_var_1,
};
use malachite_nz::natural::arithmetic::div_round::limbs_limb_div_round_limbs;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_natural_rounding_mode_triple_gen_var_1, natural_pair_gen_var_5, natural_pair_gen_var_7,
    natural_rounding_mode_pair_gen, natural_rounding_mode_pair_gen_var_2,
};
use num::{BigUint, Integer};
use rug::ops::DivRounding;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_div_round() {
    let test = |s, t, rm, quotient, o| {
        let u = Natural::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        let mut x = u.clone();
        assert_eq!(x.div_round_assign(v.clone(), rm), o);
        assert_eq!(x.to_string(), quotient);
        assert!(x.is_valid());

        let mut x = u.clone();
        assert_eq!(x.div_round_assign(&v, rm), o);
        assert_eq!(x.to_string(), quotient);
        assert!(x.is_valid());

        let (q, o_alt) = u.clone().div_round(v.clone(), rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert_eq!(o_alt, o);

        let (q, o_alt) = u.clone().div_round(&v, rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert_eq!(o_alt, o);

        let (q, o_alt) = (&u).div_round(v.clone(), rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert_eq!(o_alt, o);

        let (q, o_alt) = (&u).div_round(&v, rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert_eq!(o_alt, o);

        match rm {
            RoundingMode::Down => {
                assert_eq!(
                    rug::Integer::from_str(s)
                        .unwrap()
                        .div_trunc(rug::Integer::from_str(t).unwrap())
                        .to_string(),
                    quotient
                );
            }
            RoundingMode::Floor => {
                assert_eq!(
                    BigUint::from_str(s)
                        .unwrap()
                        .div_floor(&BigUint::from_str(t).unwrap())
                        .to_string(),
                    quotient
                );
                assert_eq!(
                    rug::Integer::from_str(s)
                        .unwrap()
                        .div_floor(rug::Integer::from_str(t).unwrap())
                        .to_string(),
                    quotient
                );
            }
            RoundingMode::Ceiling => {
                assert_eq!(
                    rug::Integer::from_str(s)
                        .unwrap()
                        .div_ceil(rug::Integer::from_str(t).unwrap())
                        .to_string(),
                    quotient
                );
            }
            _ => {}
        }
    };
    test("0", "1", RoundingMode::Down, "0", Ordering::Equal);
    test("0", "1", RoundingMode::Floor, "0", Ordering::Equal);
    test("0", "1", RoundingMode::Up, "0", Ordering::Equal);
    test("0", "1", RoundingMode::Ceiling, "0", Ordering::Equal);
    test("0", "1", RoundingMode::Nearest, "0", Ordering::Equal);
    test("0", "1", RoundingMode::Exact, "0", Ordering::Equal);

    test("0", "123", RoundingMode::Down, "0", Ordering::Equal);
    test("0", "123", RoundingMode::Floor, "0", Ordering::Equal);
    test("0", "123", RoundingMode::Up, "0", Ordering::Equal);
    test("0", "123", RoundingMode::Ceiling, "0", Ordering::Equal);
    test("0", "123", RoundingMode::Nearest, "0", Ordering::Equal);
    test("0", "123", RoundingMode::Exact, "0", Ordering::Equal);

    test("1", "1", RoundingMode::Down, "1", Ordering::Equal);
    test("1", "1", RoundingMode::Floor, "1", Ordering::Equal);
    test("1", "1", RoundingMode::Up, "1", Ordering::Equal);
    test("1", "1", RoundingMode::Ceiling, "1", Ordering::Equal);
    test("1", "1", RoundingMode::Nearest, "1", Ordering::Equal);
    test("1", "1", RoundingMode::Exact, "1", Ordering::Equal);

    test("123", "1", RoundingMode::Down, "123", Ordering::Equal);
    test("123", "1", RoundingMode::Floor, "123", Ordering::Equal);
    test("123", "1", RoundingMode::Up, "123", Ordering::Equal);
    test("123", "1", RoundingMode::Ceiling, "123", Ordering::Equal);
    test("123", "1", RoundingMode::Nearest, "123", Ordering::Equal);
    test("123", "1", RoundingMode::Exact, "123", Ordering::Equal);

    test("123", "2", RoundingMode::Down, "61", Ordering::Less);
    test("123", "2", RoundingMode::Floor, "61", Ordering::Less);
    test("123", "2", RoundingMode::Up, "62", Ordering::Greater);
    test("123", "2", RoundingMode::Ceiling, "62", Ordering::Greater);
    test("123", "2", RoundingMode::Nearest, "62", Ordering::Greater);

    test("125", "2", RoundingMode::Down, "62", Ordering::Less);
    test("125", "2", RoundingMode::Floor, "62", Ordering::Less);
    test("125", "2", RoundingMode::Up, "63", Ordering::Greater);
    test("125", "2", RoundingMode::Ceiling, "63", Ordering::Greater);
    test("125", "2", RoundingMode::Nearest, "62", Ordering::Less);

    test("123", "123", RoundingMode::Down, "1", Ordering::Equal);
    test("123", "123", RoundingMode::Floor, "1", Ordering::Equal);
    test("123", "123", RoundingMode::Up, "1", Ordering::Equal);
    test("123", "123", RoundingMode::Ceiling, "1", Ordering::Equal);
    test("123", "123", RoundingMode::Nearest, "1", Ordering::Equal);
    test("123", "123", RoundingMode::Exact, "1", Ordering::Equal);

    test("123", "456", RoundingMode::Down, "0", Ordering::Less);
    test("123", "456", RoundingMode::Floor, "0", Ordering::Less);
    test("123", "456", RoundingMode::Up, "1", Ordering::Greater);
    test("123", "456", RoundingMode::Ceiling, "1", Ordering::Greater);
    test("123", "456", RoundingMode::Nearest, "0", Ordering::Less);

    test(
        "1000000000000",
        "1",
        RoundingMode::Down,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        "1",
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        "1",
        RoundingMode::Up,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        "1",
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        "1",
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        "1",
        RoundingMode::Exact,
        "1000000000000",
        Ordering::Equal,
    );

    test(
        "1000000000000",
        "3",
        RoundingMode::Down,
        "333333333333",
        Ordering::Less,
    );
    test(
        "1000000000000",
        "3",
        RoundingMode::Floor,
        "333333333333",
        Ordering::Less,
    );
    test(
        "1000000000000",
        "3",
        RoundingMode::Up,
        "333333333334",
        Ordering::Greater,
    );
    test(
        "1000000000000",
        "3",
        RoundingMode::Ceiling,
        "333333333334",
        Ordering::Greater,
    );
    test(
        "1000000000000",
        "3",
        RoundingMode::Nearest,
        "333333333333",
        Ordering::Less,
    );

    test(
        "999999999999",
        "2",
        RoundingMode::Down,
        "499999999999",
        Ordering::Less,
    );
    test(
        "999999999999",
        "2",
        RoundingMode::Floor,
        "499999999999",
        Ordering::Less,
    );
    test(
        "999999999999",
        "2",
        RoundingMode::Up,
        "500000000000",
        Ordering::Greater,
    );
    test(
        "999999999999",
        "2",
        RoundingMode::Ceiling,
        "500000000000",
        Ordering::Greater,
    );
    test(
        "999999999999",
        "2",
        RoundingMode::Nearest,
        "500000000000",
        Ordering::Greater,
    );

    test(
        "1000000000001",
        "2",
        RoundingMode::Down,
        "500000000000",
        Ordering::Less,
    );
    test(
        "1000000000001",
        "2",
        RoundingMode::Floor,
        "500000000000",
        Ordering::Less,
    );
    test(
        "1000000000001",
        "2",
        RoundingMode::Up,
        "500000000001",
        Ordering::Greater,
    );
    test(
        "1000000000001",
        "2",
        RoundingMode::Ceiling,
        "500000000001",
        Ordering::Greater,
    );
    test(
        "1000000000001",
        "2",
        RoundingMode::Nearest,
        "500000000000",
        Ordering::Less,
    );

    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Down,
        "232830643708079",
        Ordering::Less,
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Floor,
        "232830643708079",
        Ordering::Less,
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Up,
        "232830643708080",
        Ordering::Greater,
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Ceiling,
        "232830643708080",
        Ordering::Greater,
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Nearest,
        "232830643708080",
        Ordering::Greater,
    );

    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Down,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Up,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Exact,
        "1000000000000",
        Ordering::Equal,
    );

    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Down,
        "999999999999",
        Ordering::Less,
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Floor,
        "999999999999",
        Ordering::Less,
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Up,
        "1000000000000",
        Ordering::Greater,
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Greater,
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Nearest,
        "999999999999",
        Ordering::Less,
    );

    test(
        "2999999999999999999999999",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "1",
        Ordering::Less,
    );
    test(
        "3000000000000000000000000",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "2",
        Ordering::Greater,
    );
    test(
        "3000000000000000000000001",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "2",
        Ordering::Greater,
    );
}

#[test]
#[should_panic]
fn div_round_assign_fail_1() {
    let mut n = Natural::from(10u32);
    n.div_round_assign(Natural::ZERO, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn div_round_assign_fail_2() {
    let mut n = Natural::from(10u32);
    n.div_round_assign(Natural::from(3u32), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn div_round_assign_ref_fail_1() {
    let mut n = Natural::from(10u32);
    n.div_round_assign(&Natural::ZERO, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn div_round_assign_ref_fail_2() {
    let mut n = Natural::from(10u32);
    n.div_round_assign(&Natural::from(3u32), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn div_round_fail_1() {
    Natural::from(10u32).div_round(Natural::ZERO, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn div_round_fail_2() {
    Natural::from(10u32).div_round(Natural::from(3u32), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn div_round_val_ref_fail_1() {
    Natural::from(10u32).div_round(&Natural::ZERO, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn div_round_val_ref_fail_2() {
    Natural::from(10u32).div_round(&Natural::from(3u32), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn div_round_ref_val_fail_1() {
    (&Natural::from(10u32)).div_round(Natural::ZERO, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn div_round_ref_val_fail_2() {
    (&Natural::from(10u32)).div_round(Natural::from(3u32), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn div_round_ref_ref_fail_1() {
    (&Natural::from(10u32)).div_round(&Natural::ZERO, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn div_round_ref_ref_fail_2() {
    (&Natural::from(10u32)).div_round(&Natural::from(3u32), RoundingMode::Exact);
}

#[test]
fn limbs_limb_div_round_limbs_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_rounding_mode_triple_gen_var_1().test_properties_with_config(
        &config,
        |(ys, x, rm)| {
            let result = limbs_limb_div_round_limbs(x, &ys, rm);
            let a = Natural::from(x);
            let b = Natural::from_owned_limbs_asc(ys);
            if rm != RoundingMode::Exact || (&a).divisible_by(&b) {
                let (result, o) = result.unwrap();
                assert_eq!((Natural::from(result), o), a.div_round(b, rm));
            } else {
                assert!(result.is_none());
            }
        },
    );
}

#[test]
fn div_round_properties() {
    natural_natural_rounding_mode_triple_gen_var_1().test_properties(|(x, y, rm)| {
        let mut mut_x = x.clone();
        let o = mut_x.div_round_assign(&y, rm);
        assert!(mut_x.is_valid());
        let q = mut_x;

        let mut mut_x = x.clone();
        assert_eq!(mut_x.div_round_assign(y.clone(), rm), o);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, q);

        let (q_alt, o_alt) = (&x).div_round(&y, rm);
        assert!(q_alt.is_valid());
        assert_eq!(q_alt, q);
        assert_eq!(o_alt, o);

        let (q_alt, o_alt) = (&x).div_round(y.clone(), rm);
        assert!(q_alt.is_valid());
        assert_eq!(q_alt, q);
        assert_eq!(o_alt, o);

        let (q_alt, o_alt) = x.clone().div_round(&y, rm);
        assert!(q_alt.is_valid());
        assert_eq!(q_alt, q);
        assert_eq!(o_alt, o);

        let (q_alt, o_alt) = x.clone().div_round(y.clone(), rm);
        assert!(q_alt.is_valid());
        assert_eq!(q_alt, q);
        assert_eq!(o_alt, o);
        assert!(q <= x);

        assert_eq!((q * &y).cmp(&x), o);
        match rm {
            RoundingMode::Floor | RoundingMode::Down => assert_ne!(o, Ordering::Greater),
            RoundingMode::Ceiling | RoundingMode::Up => assert_ne!(o, Ordering::Less),
            RoundingMode::Exact => assert_eq!(o, Ordering::Equal),
            _ => {}
        }
    });

    natural_pair_gen_var_5().test_properties(|(x, y)| {
        let left_multiplied = &x * &y;
        let xo = (x.clone(), Ordering::Equal);
        assert_eq!((&left_multiplied).div_round(&y, RoundingMode::Down), xo);
        assert_eq!((&left_multiplied).div_round(&y, RoundingMode::Up), xo);
        assert_eq!((&left_multiplied).div_round(&y, RoundingMode::Floor), xo);
        assert_eq!((&left_multiplied).div_round(&y, RoundingMode::Ceiling), xo);
        assert_eq!((&left_multiplied).div_round(&y, RoundingMode::Nearest), xo);
        assert_eq!((&left_multiplied).div_round(&y, RoundingMode::Exact), xo);

        assert_eq!(
            Natural::exact_from(&rug::Integer::from(&x).div_trunc(rug::Integer::from(&y))),
            (&x).div_round(&y, RoundingMode::Down).0
        );
        assert_eq!(
            Natural::from(&BigUint::from(&x).div_floor(&BigUint::from(&y))),
            (&x).div_round(&y, RoundingMode::Floor).0
        );
        assert_eq!(
            Natural::exact_from(&rug::Integer::from(&x).div_floor(rug::Integer::from(&y))),
            (&x).div_round(&y, RoundingMode::Floor).0
        );
        assert_eq!(
            Natural::exact_from(&rug::Integer::from(&x).div_ceil(rug::Integer::from(&y))),
            (&x).div_round(&y, RoundingMode::Ceiling).0
        );
        assert_eq!(
            (&x).ceiling_div_neg_mod(&y).0,
            x.div_round(y, RoundingMode::Ceiling).0
        );
    });

    natural_pair_gen_var_7().test_properties(|(x, y)| {
        let down = (&x).div_round(&y, RoundingMode::Down);
        assert_eq!(down.1, Ordering::Less);
        let up = (&down.0 + Natural::ONE, Ordering::Greater);
        assert_eq!((&x).div_round(&y, RoundingMode::Up), up);
        assert_eq!((&x).div_round(&y, RoundingMode::Floor), down);
        assert_eq!((&x).div_round(&y, RoundingMode::Ceiling), up);
        let nearest = x.div_round(y, RoundingMode::Nearest);
        assert!(nearest == down || nearest == up);
    });

    natural_rounding_mode_pair_gen().test_properties(|(x, rm)| {
        assert_eq!((&x).div_round(Natural::ONE, rm), (x, Ordering::Equal));
    });

    natural_rounding_mode_pair_gen_var_2().test_properties(|(x, rm)| {
        assert_eq!(
            Natural::ZERO.div_round(&x, rm),
            (Natural::ZERO, Ordering::Equal)
        );
        assert_eq!((&x).div_round(&x, rm), (Natural::ONE, Ordering::Equal));
    });

    unsigned_unsigned_rounding_mode_triple_gen_var_1::<Limb>().test_properties(|(x, y, rm)| {
        let (q, o) = x.div_round(y, rm);
        assert_eq!(
            Natural::from(x).div_round(Natural::from(y), rm),
            (Natural::from(q), o)
        );
    });
}
