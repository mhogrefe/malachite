use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{CeilingDivNegMod, DivRound, DivRoundAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::round::RoundingMode;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::{BigUint, Integer};
use rug::ops::DivRounding;

use common::test_properties;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_positive_natural, pairs_of_natural_and_positive_natural_var_2,
    pairs_of_natural_and_rounding_mode, pairs_of_positive_natural_and_rounding_mode,
    triples_of_natural_positive_natural_and_rounding_mode_var_1,
    triples_of_natural_positive_unsigned_and_rounding_mode_var_1,
    triples_of_unsigned_positive_natural_and_rounding_mode_var_1,
};

#[test]
fn test_div_round() {
    let test = |u, v, rm, quotient| {
        let mut x = Natural::from_str(u).unwrap();
        x.div_round_assign(Natural::from_str(v).unwrap(), rm);
        assert_eq!(x.to_string(), quotient);
        assert!(x.is_valid());

        let mut x = Natural::from_str(u).unwrap();
        x.div_round_assign(&Natural::from_str(v).unwrap(), rm);
        assert_eq!(x.to_string(), quotient);
        assert!(x.is_valid());

        let q = Natural::from_str(u)
            .unwrap()
            .div_round(Natural::from_str(v).unwrap(), rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = Natural::from_str(u)
            .unwrap()
            .div_round(&Natural::from_str(v).unwrap(), rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = (&Natural::from_str(u).unwrap()).div_round(Natural::from_str(v).unwrap(), rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = (&Natural::from_str(u).unwrap()).div_round(&Natural::from_str(v).unwrap(), rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        match rm {
            RoundingMode::Down => {
                assert_eq!(
                    rug::Integer::from_str(u)
                        .unwrap()
                        .div_trunc(rug::Integer::from_str(v).unwrap())
                        .to_string(),
                    quotient
                );
            }
            RoundingMode::Floor => {
                assert_eq!(
                    BigUint::from_str(u)
                        .unwrap()
                        .div_floor(&BigUint::from_str(v).unwrap())
                        .to_string(),
                    quotient
                );
                assert_eq!(
                    rug::Integer::from_str(u)
                        .unwrap()
                        .div_floor(rug::Integer::from_str(v).unwrap())
                        .to_string(),
                    quotient
                );
            }
            RoundingMode::Ceiling => {
                assert_eq!(
                    rug::Integer::from_str(u)
                        .unwrap()
                        .div_ceil(rug::Integer::from_str(v).unwrap())
                        .to_string(),
                    quotient
                );
            }
            _ => {}
        }
    };
    test("0", "1", RoundingMode::Down, "0");
    test("0", "1", RoundingMode::Floor, "0");
    test("0", "1", RoundingMode::Up, "0");
    test("0", "1", RoundingMode::Ceiling, "0");
    test("0", "1", RoundingMode::Nearest, "0");
    test("0", "1", RoundingMode::Exact, "0");

    test("0", "123", RoundingMode::Down, "0");
    test("0", "123", RoundingMode::Floor, "0");
    test("0", "123", RoundingMode::Up, "0");
    test("0", "123", RoundingMode::Ceiling, "0");
    test("0", "123", RoundingMode::Nearest, "0");

    test("1", "1", RoundingMode::Down, "1");
    test("1", "1", RoundingMode::Floor, "1");
    test("1", "1", RoundingMode::Up, "1");
    test("1", "1", RoundingMode::Ceiling, "1");
    test("1", "1", RoundingMode::Nearest, "1");
    test("1", "1", RoundingMode::Exact, "1");

    test("123", "1", RoundingMode::Down, "123");
    test("123", "1", RoundingMode::Floor, "123");
    test("123", "1", RoundingMode::Up, "123");
    test("123", "1", RoundingMode::Ceiling, "123");
    test("123", "1", RoundingMode::Nearest, "123");
    test("123", "1", RoundingMode::Exact, "123");

    test("123", "2", RoundingMode::Down, "61");
    test("123", "2", RoundingMode::Floor, "61");
    test("123", "2", RoundingMode::Up, "62");
    test("123", "2", RoundingMode::Ceiling, "62");
    test("123", "2", RoundingMode::Nearest, "62");

    test("125", "2", RoundingMode::Down, "62");
    test("125", "2", RoundingMode::Floor, "62");
    test("125", "2", RoundingMode::Up, "63");
    test("125", "2", RoundingMode::Ceiling, "63");
    test("125", "2", RoundingMode::Nearest, "62");

    test("123", "123", RoundingMode::Down, "1");
    test("123", "123", RoundingMode::Floor, "1");
    test("123", "123", RoundingMode::Up, "1");
    test("123", "123", RoundingMode::Ceiling, "1");
    test("123", "123", RoundingMode::Nearest, "1");
    test("123", "123", RoundingMode::Exact, "1");

    test("123", "456", RoundingMode::Down, "0");
    test("123", "456", RoundingMode::Floor, "0");
    test("123", "456", RoundingMode::Up, "1");
    test("123", "456", RoundingMode::Ceiling, "1");
    test("123", "456", RoundingMode::Nearest, "0");

    test("1000000000000", "1", RoundingMode::Down, "1000000000000");
    test("1000000000000", "1", RoundingMode::Floor, "1000000000000");
    test("1000000000000", "1", RoundingMode::Up, "1000000000000");
    test("1000000000000", "1", RoundingMode::Ceiling, "1000000000000");
    test("1000000000000", "1", RoundingMode::Nearest, "1000000000000");
    test("1000000000000", "1", RoundingMode::Exact, "1000000000000");

    test("1000000000000", "3", RoundingMode::Down, "333333333333");
    test("1000000000000", "3", RoundingMode::Floor, "333333333333");
    test("1000000000000", "3", RoundingMode::Up, "333333333334");
    test("1000000000000", "3", RoundingMode::Ceiling, "333333333334");
    test("1000000000000", "3", RoundingMode::Nearest, "333333333333");

    test("999999999999", "2", RoundingMode::Down, "499999999999");
    test("999999999999", "2", RoundingMode::Floor, "499999999999");
    test("999999999999", "2", RoundingMode::Up, "500000000000");
    test("999999999999", "2", RoundingMode::Ceiling, "500000000000");
    test("999999999999", "2", RoundingMode::Nearest, "500000000000");

    test("1000000000001", "2", RoundingMode::Down, "500000000000");
    test("1000000000001", "2", RoundingMode::Floor, "500000000000");
    test("1000000000001", "2", RoundingMode::Up, "500000000001");
    test("1000000000001", "2", RoundingMode::Ceiling, "500000000001");
    test("1000000000001", "2", RoundingMode::Nearest, "500000000000");

    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Down,
        "232830643708079",
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Floor,
        "232830643708079",
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Up,
        "232830643708080",
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Ceiling,
        "232830643708080",
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Nearest,
        "232830643708080",
    );

    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Exact,
        "1000000000000",
    );

    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Down,
        "999999999999",
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Floor,
        "999999999999",
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Nearest,
        "999999999999",
    );

    test(
        "2999999999999999999999999",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "1",
    );
    test(
        "3000000000000000000000000",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "2",
    );
    test(
        "3000000000000000000000001",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "2",
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
fn div_round_properties() {
    test_properties(
        triples_of_natural_positive_natural_and_rounding_mode_var_1,
        |&(ref x, ref y, rm)| {
            let mut mut_x = x.clone();
            mut_x.div_round_assign(y, rm);
            assert!(mut_x.is_valid());
            let quotient = mut_x;

            let mut mut_x = x.clone();
            mut_x.div_round_assign(y.clone(), rm);
            assert!(mut_x.is_valid());
            assert_eq!(mut_x, quotient);

            let quotient_alt = x.div_round(y, rm);
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);

            let quotient_alt = x.div_round(y.clone(), rm);
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);

            let quotient_alt = x.clone().div_round(y, rm);
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);

            let quotient_alt = x.clone().div_round(y.clone(), rm);
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);

            assert!(quotient <= *x);
        },
    );

    test_properties(pairs_of_natural_and_positive_natural, |&(ref x, ref y)| {
        let left_multiplied = x * y;
        assert_eq!((&left_multiplied).div_round(y, RoundingMode::Down), *x);
        assert_eq!((&left_multiplied).div_round(y, RoundingMode::Up), *x);
        assert_eq!((&left_multiplied).div_round(y, RoundingMode::Floor), *x);
        assert_eq!((&left_multiplied).div_round(y, RoundingMode::Ceiling), *x);
        assert_eq!((&left_multiplied).div_round(y, RoundingMode::Nearest), *x);
        assert_eq!((&left_multiplied).div_round(y, RoundingMode::Exact), *x);

        assert_eq!(
            rug_integer_to_natural(&natural_to_rug_integer(x).div_trunc(natural_to_rug_integer(y))),
            x.div_round(y, RoundingMode::Down)
        );
        assert_eq!(
            biguint_to_natural(&natural_to_biguint(x).div_floor(&natural_to_biguint(y))),
            x.div_round(y, RoundingMode::Floor)
        );
        assert_eq!(
            rug_integer_to_natural(&natural_to_rug_integer(x).div_floor(natural_to_rug_integer(y))),
            x.div_round(y, RoundingMode::Floor)
        );
        assert_eq!(
            rug_integer_to_natural(&natural_to_rug_integer(x).div_ceil(natural_to_rug_integer(y))),
            x.div_round(y, RoundingMode::Ceiling)
        );
        assert_eq!(
            x.ceiling_div_neg_mod(y).0,
            x.div_round(y, RoundingMode::Ceiling)
        );
    });

    // TODO test using Rationals
    test_properties(
        pairs_of_natural_and_positive_natural_var_2,
        |&(ref x, ref y)| {
            let down = x.div_round(y, RoundingMode::Down);
            let up = &down + Natural::ONE;
            assert_eq!(x.div_round(y, RoundingMode::Up), up);
            assert_eq!(x.div_round(y, RoundingMode::Floor), down);
            assert_eq!(x.div_round(y, RoundingMode::Ceiling), up);
            let nearest = x.div_round(y, RoundingMode::Nearest);
            assert!(nearest == down || nearest == up);
        },
    );

    test_properties(pairs_of_natural_and_rounding_mode, |&(ref x, rm)| {
        assert_eq!(x.div_round(Natural::ONE, rm), *x);
    });

    test_properties(
        pairs_of_positive_natural_and_rounding_mode,
        |&(ref x, rm)| {
            assert_eq!(Natural::ZERO.div_round(x, rm), 0 as Limb);
            assert_eq!(x.div_round(x, rm), 1 as Limb);
        },
    );

    test_properties(
        triples_of_natural_positive_unsigned_and_rounding_mode_var_1,
        |&(ref n, u, rm): &(Natural, Limb, RoundingMode)| {
            assert_eq!(n.div_round(u, rm), n.div_round(Natural::from(u), rm));
        },
    );

    test_properties(
        triples_of_unsigned_positive_natural_and_rounding_mode_var_1,
        |&(u, ref n, rm): &(Limb, Natural, RoundingMode)| {
            assert_eq!(u.div_round(n, rm), Natural::from(u).div_round(n, rm));
        },
    );
}
