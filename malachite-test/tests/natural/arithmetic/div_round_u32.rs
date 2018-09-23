use common::test_properties;
use malachite_base::num::{CeilingDivNegMod, DivRound, DivRoundAssign, One, Zero};
use malachite_base::round::RoundingMode;
use malachite_nz::natural::arithmetic::div_round_u32::limbs_limb_div_round_limbs;
use malachite_nz::natural::Natural;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::{
    pairs_of_positive_unsigned_and_rounding_mode, pairs_of_unsigned_and_rounding_mode,
    triples_of_unsigned_unsigned_vec_and_rounding_mode_var_1,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_positive_u32_var_1, pairs_of_natural_and_positive_unsigned,
    pairs_of_natural_and_rounding_mode, pairs_of_u32_and_positive_natural_var_1,
    triples_of_natural_positive_unsigned_and_rounding_mode_var_1,
    triples_of_unsigned_positive_natural_and_rounding_mode_var_1,
};
use malachite_test::natural::arithmetic::div_round_u32::num_div_round_u32_floor;
use num::BigUint;
use rug::{self, ops::DivRounding};
use std::str::FromStr;

#[test]
fn test_limbs_limb_div_round_limbs() {
    let test = |limb: u32, limbs: &[u32], rm, quotient| {
        assert_eq!(limbs_limb_div_round_limbs(limb, limbs, rm), quotient);
    };
    test(789, &[123, 456], RoundingMode::Down, Some(0));
    test(789, &[123, 456], RoundingMode::Floor, Some(0));
    test(789, &[123, 456], RoundingMode::Up, Some(1));
    test(789, &[123, 456], RoundingMode::Ceiling, Some(1));
    test(0, &[123, 456], RoundingMode::Exact, Some(0));
    test(789, &[123, 456], RoundingMode::Exact, None);
    test(789, &[123, 456], RoundingMode::Nearest, Some(0));
    test(0xffff_ffff, &[123, 1], RoundingMode::Nearest, Some(1));
    test(
        0xffff_ffff,
        &[0xffff_ffff, 1],
        RoundingMode::Nearest,
        Some(0),
    );
    test(
        0xffff_ffff,
        &[0xffff_fffe, 1],
        RoundingMode::Nearest,
        Some(0),
    );
    test(
        0xffff_ffff,
        &[0xffff_fffd, 1],
        RoundingMode::Nearest,
        Some(1),
    );
}

#[test]
fn test_div_round_u32() {
    let test = |u, v: u32, rm: RoundingMode, quotient| {
        let mut n = Natural::from_str(u).unwrap();
        n.div_round_assign(v, rm);
        assert_eq!(n.to_string(), quotient);
        assert!(n.is_valid());

        let q = Natural::from_str(u).unwrap().div_round(v, rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = (&Natural::from_str(u).unwrap()).div_round(v, rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        match rm {
            RoundingMode::Down => {
                assert_eq!(
                    rug::Integer::from_str(u).unwrap().div_trunc(v).to_string(),
                    quotient
                );
            }
            RoundingMode::Floor => {
                assert_eq!(
                    num_div_round_u32_floor(BigUint::from_str(u).unwrap(), v).to_string(),
                    quotient
                );
                assert_eq!(
                    rug::Integer::from_str(u).unwrap().div_floor(v).to_string(),
                    quotient
                );
            }
            RoundingMode::Ceiling => {
                assert_eq!(
                    rug::Integer::from_str(u).unwrap().div_ceil(v).to_string(),
                    quotient
                );
            }
            _ => {}
        }
    };
    test("0", 1, RoundingMode::Down, "0");
    test("0", 1, RoundingMode::Floor, "0");
    test("0", 1, RoundingMode::Up, "0");
    test("0", 1, RoundingMode::Ceiling, "0");
    test("0", 1, RoundingMode::Nearest, "0");
    test("0", 1, RoundingMode::Exact, "0");

    test("0", 123, RoundingMode::Down, "0");
    test("0", 123, RoundingMode::Floor, "0");
    test("0", 123, RoundingMode::Up, "0");
    test("0", 123, RoundingMode::Ceiling, "0");
    test("0", 123, RoundingMode::Nearest, "0");

    test("1", 1, RoundingMode::Down, "1");
    test("1", 1, RoundingMode::Floor, "1");
    test("1", 1, RoundingMode::Up, "1");
    test("1", 1, RoundingMode::Ceiling, "1");
    test("1", 1, RoundingMode::Nearest, "1");
    test("1", 1, RoundingMode::Exact, "1");

    test("123", 1, RoundingMode::Down, "123");
    test("123", 1, RoundingMode::Floor, "123");
    test("123", 1, RoundingMode::Up, "123");
    test("123", 1, RoundingMode::Ceiling, "123");
    test("123", 1, RoundingMode::Nearest, "123");
    test("123", 1, RoundingMode::Exact, "123");

    test("123", 2, RoundingMode::Down, "61");
    test("123", 2, RoundingMode::Floor, "61");
    test("123", 2, RoundingMode::Up, "62");
    test("123", 2, RoundingMode::Ceiling, "62");
    test("123", 2, RoundingMode::Nearest, "62");

    test("125", 2, RoundingMode::Down, "62");
    test("125", 2, RoundingMode::Floor, "62");
    test("125", 2, RoundingMode::Up, "63");
    test("125", 2, RoundingMode::Ceiling, "63");
    test("125", 2, RoundingMode::Nearest, "62");

    test("123", 123, RoundingMode::Down, "1");
    test("123", 123, RoundingMode::Floor, "1");
    test("123", 123, RoundingMode::Up, "1");
    test("123", 123, RoundingMode::Ceiling, "1");
    test("123", 123, RoundingMode::Nearest, "1");
    test("123", 123, RoundingMode::Exact, "1");

    test("123", 456, RoundingMode::Down, "0");
    test("123", 456, RoundingMode::Floor, "0");
    test("123", 456, RoundingMode::Up, "1");
    test("123", 456, RoundingMode::Ceiling, "1");
    test("123", 456, RoundingMode::Nearest, "0");

    test("1000000000000", 1, RoundingMode::Down, "1000000000000");
    test("1000000000000", 1, RoundingMode::Floor, "1000000000000");
    test("1000000000000", 1, RoundingMode::Up, "1000000000000");
    test("1000000000000", 1, RoundingMode::Ceiling, "1000000000000");
    test("1000000000000", 1, RoundingMode::Nearest, "1000000000000");
    test("1000000000000", 1, RoundingMode::Exact, "1000000000000");

    test("1000000000000", 3, RoundingMode::Down, "333333333333");
    test("1000000000000", 3, RoundingMode::Floor, "333333333333");
    test("1000000000000", 3, RoundingMode::Up, "333333333334");
    test("1000000000000", 3, RoundingMode::Ceiling, "333333333334");
    test("1000000000000", 3, RoundingMode::Nearest, "333333333333");

    test("999999999999", 2, RoundingMode::Down, "499999999999");
    test("999999999999", 2, RoundingMode::Floor, "499999999999");
    test("999999999999", 2, RoundingMode::Up, "500000000000");
    test("999999999999", 2, RoundingMode::Ceiling, "500000000000");
    test("999999999999", 2, RoundingMode::Nearest, "500000000000");

    test("1000000000001", 2, RoundingMode::Down, "500000000000");
    test("1000000000001", 2, RoundingMode::Floor, "500000000000");
    test("1000000000001", 2, RoundingMode::Up, "500000000001");
    test("1000000000001", 2, RoundingMode::Ceiling, "500000000001");
    test("1000000000001", 2, RoundingMode::Nearest, "500000000000");

    test(
        "1000000000000000000000000",
        4_294_967_295,
        RoundingMode::Down,
        "232830643708079",
    );
    test(
        "1000000000000000000000000",
        4_294_967_295,
        RoundingMode::Floor,
        "232830643708079",
    );
    test(
        "1000000000000000000000000",
        4_294_967_295,
        RoundingMode::Up,
        "232830643708080",
    );
    test(
        "1000000000000000000000000",
        4_294_967_295,
        RoundingMode::Ceiling,
        "232830643708080",
    );
    test(
        "1000000000000000000000000",
        4_294_967_295,
        RoundingMode::Nearest,
        "232830643708080",
    );
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_round_assign_u32_fail_1() {
    let mut n = Natural::from(10u32);
    n.div_round_assign(0, RoundingMode::Floor);
}

#[test]
#[should_panic(expected = "Division is not exact")]
fn div_round_assign_u32_fail_2() {
    let mut n = Natural::from(10u32);
    n.div_round_assign(3, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_round_u32_fail_1() {
    Natural::from(10u32).div_round(0, RoundingMode::Floor);
}

#[test]
#[should_panic(expected = "Division is not exact")]
fn div_round_u32_fail_2() {
    Natural::from(10u32).div_round(3, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_round_u32_ref_fail_1() {
    (&Natural::from(10u32)).div_round(0, RoundingMode::Floor);
}

#[test]
#[should_panic(expected = "Division is not exact: 10 / 3")]
fn div_round_u32_ref_fail_2() {
    (&Natural::from(10u32)).div_round(3, RoundingMode::Exact);
}

#[test]
fn test_u32_div_round_natural() {
    let test = |u: u32, v, rm, quotient| {
        let mut mut_u = u;
        mut_u.div_round_assign(Natural::from_str(v).unwrap(), rm);
        assert_eq!(mut_u, quotient);

        let mut mut_u = u;
        mut_u.div_round_assign(&Natural::from_str(v).unwrap(), rm);
        assert_eq!(mut_u, quotient);

        assert_eq!(u.div_round(Natural::from_str(v).unwrap(), rm), quotient);
        assert_eq!(u.div_round(&Natural::from_str(v).unwrap(), rm), quotient);
    };
    test(0, "1", RoundingMode::Down, 0);
    test(0, "1", RoundingMode::Floor, 0);
    test(0, "1", RoundingMode::Up, 0);
    test(0, "1", RoundingMode::Ceiling, 0);
    test(0, "1", RoundingMode::Nearest, 0);
    test(0, "1", RoundingMode::Exact, 0);

    test(0, "123", RoundingMode::Down, 0);
    test(0, "123", RoundingMode::Floor, 0);
    test(0, "123", RoundingMode::Up, 0);
    test(0, "123", RoundingMode::Ceiling, 0);
    test(0, "123", RoundingMode::Nearest, 0);

    test(1, "1", RoundingMode::Down, 1);
    test(1, "1", RoundingMode::Floor, 1);
    test(1, "1", RoundingMode::Up, 1);
    test(1, "1", RoundingMode::Ceiling, 1);
    test(1, "1", RoundingMode::Nearest, 1);
    test(1, "1", RoundingMode::Exact, 1);

    test(123, "1", RoundingMode::Down, 123);
    test(123, "1", RoundingMode::Floor, 123);
    test(123, "1", RoundingMode::Up, 123);
    test(123, "1", RoundingMode::Ceiling, 123);
    test(123, "1", RoundingMode::Nearest, 123);
    test(123, "1", RoundingMode::Exact, 123);

    test(123, "2", RoundingMode::Down, 61);
    test(123, "2", RoundingMode::Floor, 61);
    test(123, "2", RoundingMode::Up, 62);
    test(123, "2", RoundingMode::Ceiling, 62);
    test(123, "2", RoundingMode::Nearest, 62);

    test(125, "2", RoundingMode::Down, 62);
    test(125, "2", RoundingMode::Floor, 62);
    test(125, "2", RoundingMode::Up, 63);
    test(125, "2", RoundingMode::Ceiling, 63);
    test(125, "2", RoundingMode::Nearest, 62);

    test(123, "123", RoundingMode::Down, 1);
    test(123, "123", RoundingMode::Floor, 1);
    test(123, "123", RoundingMode::Up, 1);
    test(123, "123", RoundingMode::Ceiling, 1);
    test(123, "123", RoundingMode::Nearest, 1);
    test(123, "123", RoundingMode::Exact, 1);

    test(123, "456", RoundingMode::Down, 0);
    test(123, "456", RoundingMode::Floor, 0);
    test(123, "456", RoundingMode::Up, 1);
    test(123, "456", RoundingMode::Ceiling, 1);
    test(123, "456", RoundingMode::Nearest, 0);

    test(123, "1000000000000", RoundingMode::Down, 0);
    test(123, "1000000000000", RoundingMode::Floor, 0);
    test(123, "1000000000000", RoundingMode::Up, 1);
    test(123, "1000000000000", RoundingMode::Ceiling, 1);
    test(123, "1000000000000", RoundingMode::Nearest, 0);

    test(3_000_000_000, "5999999999", RoundingMode::Down, 0);
    test(3_000_000_000, "5999999999", RoundingMode::Floor, 0);
    test(3_000_000_000, "5999999999", RoundingMode::Up, 1);
    test(3_000_000_000, "5999999999", RoundingMode::Ceiling, 1);
    test(3_000_000_000, "5999999999", RoundingMode::Nearest, 1);

    test(3_000_000_000, "6000000000", RoundingMode::Down, 0);
    test(3_000_000_000, "6000000000", RoundingMode::Floor, 0);
    test(3_000_000_000, "6000000000", RoundingMode::Up, 1);
    test(3_000_000_000, "6000000000", RoundingMode::Ceiling, 1);
    test(3_000_000_000, "6000000000", RoundingMode::Nearest, 0);

    test(3_000_000_000, "6000000001", RoundingMode::Down, 0);
    test(3_000_000_000, "6000000001", RoundingMode::Floor, 0);
    test(3_000_000_000, "6000000001", RoundingMode::Up, 1);
    test(3_000_000_000, "6000000001", RoundingMode::Ceiling, 1);
    test(3_000_000_000, "6000000001", RoundingMode::Nearest, 0);
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_div_round_assign_natural_fail_1() {
    let mut n = 10;
    n.div_round_assign(Natural::ZERO, RoundingMode::Floor);
}

#[test]
#[should_panic(expected = "Division is not exact")]
fn u32_div_round_assign_natural_fail_2() {
    let mut n = 10;
    n.div_round_assign(Natural::from(3u32), RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_div_round_assign_natural_ref_fail_1() {
    let mut n = 10;
    n.div_round_assign(&Natural::ZERO, RoundingMode::Floor);
}

#[test]
#[should_panic(expected = "Division is not exact")]
fn u32_div_round_assign_natural_ref_fail_2() {
    let mut n = 10;
    n.div_round_assign(&Natural::from(3u32), RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_div_round_natural_fail_1() {
    10.div_round(Natural::ZERO, RoundingMode::Floor);
}

#[test]
#[should_panic(expected = "Division is not exact")]
fn u32_div_round_natural_fail_2() {
    10.div_round(Natural::from(3u32), RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_div_round_natural_ref_fail_1() {
    10.div_round(&Natural::ZERO, RoundingMode::Floor);
}

#[test]
#[should_panic(expected = "Division is not exact: 10 / 3")]
fn u32_div_round_natural_ref_fail_2() {
    10.div_round(&Natural::from(3u32), RoundingMode::Exact);
}

#[test]
fn limbs_limb_div_round_limbs_properties() {
    test_properties(
        triples_of_unsigned_unsigned_vec_and_rounding_mode_var_1,
        |&(limb, ref limbs, rm)| {
            let n = Natural::from_limbs_asc(limbs);
            if let Some(quotient) = limbs_limb_div_round_limbs(limb, limbs, rm) {
                assert_eq!(quotient, limb.div_round(n, rm));
            } else {
                assert_eq!(rm, RoundingMode::Exact);
                //TODO use divisible
                assert_ne!(limb % n, 0);
            }
        },
    );
}

#[test]
fn div_u32_properties() {
    test_properties(
        triples_of_natural_positive_unsigned_and_rounding_mode_var_1,
        |&(ref n, u, rm): &(Natural, u32, RoundingMode)| {
            let mut mut_n = n.clone();
            mut_n.div_round_assign(u, rm);
            assert!(mut_n.is_valid());
            let quotient = mut_n;

            let quotient_alt = n.div_round(u, rm);
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);

            let quotient_alt = n.clone().div_round(u, rm);
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);

            assert!(quotient <= *n);

            //TODO assert_eq!(n.div_round(Natural::from(u), rm), quotient);
        },
    );

    test_properties(
        pairs_of_natural_and_positive_unsigned::<u32>,
        |&(ref n, u)| {
            let left_multiplied = n * u;
            assert_eq!((&left_multiplied).div_round(u, RoundingMode::Down), *n);
            assert_eq!((&left_multiplied).div_round(u, RoundingMode::Up), *n);
            assert_eq!((&left_multiplied).div_round(u, RoundingMode::Floor), *n);
            assert_eq!((&left_multiplied).div_round(u, RoundingMode::Ceiling), *n);
            assert_eq!((&left_multiplied).div_round(u, RoundingMode::Nearest), *n);
            assert_eq!((&left_multiplied).div_round(u, RoundingMode::Exact), *n);

            assert_eq!(
                rug_integer_to_natural(&natural_to_rug_integer(n).div_trunc(u)),
                n.div_round(u, RoundingMode::Down)
            );
            assert_eq!(
                biguint_to_natural(&num_div_round_u32_floor(natural_to_biguint(n), u)),
                n.div_round(u, RoundingMode::Floor)
            );
            assert_eq!(
                rug_integer_to_natural(&natural_to_rug_integer(n).div_floor(u)),
                n.div_round(u, RoundingMode::Floor)
            );
            assert_eq!(
                rug_integer_to_natural(&natural_to_rug_integer(n).div_ceil(u)),
                n.div_round(u, RoundingMode::Ceiling)
            );
            assert_eq!(
                n.ceiling_div_neg_mod(u).0,
                n.div_round(u, RoundingMode::Ceiling)
            );
        },
    );

    // TODO test using Rationals
    test_properties(pairs_of_natural_and_positive_u32_var_1, |&(ref n, u)| {
        let down = n.div_round(u, RoundingMode::Down);
        let up = &down + 1;
        assert_eq!(n.div_round(u, RoundingMode::Up), up);
        assert_eq!(n.div_round(u, RoundingMode::Floor), down);
        assert_eq!(n.div_round(u, RoundingMode::Ceiling), up);
        let nearest = n.div_round(u, RoundingMode::Nearest);
        assert!(nearest == down || nearest == up);
    });

    #[allow(unknown_lints, identity_op)]
    test_properties(pairs_of_natural_and_rounding_mode, |&(ref n, rm)| {
        assert_eq!(n.div_round(1, rm), *n);
    });

    test_properties(
        pairs_of_positive_unsigned_and_rounding_mode::<u32>,
        |&(u, rm)| {
            assert_eq!(Natural::ZERO.div_round(u, rm), 0);
        },
    );

    test_properties(
        triples_of_unsigned_positive_natural_and_rounding_mode_var_1,
        |&(u, ref n, rm): &(u32, Natural, RoundingMode)| {
            let mut mut_u = u;
            mut_u.div_round_assign(n, rm);
            let quotient = mut_u;

            let mut mut_u = u;
            mut_u.div_round_assign(n.clone(), rm);
            assert_eq!(mut_u, quotient);

            let quotient_alt = u.div_round(n, rm);
            assert_eq!(quotient_alt, quotient);

            let quotient_alt = u.div_round(n.clone(), rm);
            assert_eq!(quotient_alt, quotient);

            assert!(quotient <= u);

            //TODO assert_eq!(Natural::from(u).div_round(n, rm), quotient);
        },
    );

    // TODO test using Rationals
    test_properties(pairs_of_u32_and_positive_natural_var_1, |&(u, ref n)| {
        let down = u.div_round(n, RoundingMode::Down);
        let up = &down + 1;
        assert_eq!(u.div_round(n, RoundingMode::Up), up);
        assert_eq!(u.div_round(n, RoundingMode::Floor), down);
        assert_eq!(u.div_round(n, RoundingMode::Ceiling), up);
        let nearest = u.div_round(n, RoundingMode::Nearest);
        assert!(nearest == down || nearest == up);
    });

    test_properties(pairs_of_unsigned_and_rounding_mode::<u32>, |&(u, rm)| {
        assert_eq!(u.div_round(Natural::ONE, rm), u);
    });
}
