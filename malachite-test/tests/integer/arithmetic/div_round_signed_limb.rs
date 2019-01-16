use common::test_properties;
use malachite_base::num::{
    CeilingDivMod, DivRound, DivRoundAssign, NegativeOne, One, PartialOrdAbs, Zero,
};
use malachite_base::round::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_test::common::{bigint_to_integer, integer_to_bigint};
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{
    pairs_of_nonzero_signed_and_rounding_mode, pairs_of_signed_and_rounding_mode,
    triples_of_signed_limb_nonzero_signed_limb_and_rounding_mode_var_1,
};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_nonzero_signed, pairs_of_integer_and_nonzero_signed_limb_var_2,
    pairs_of_integer_and_rounding_mode, pairs_of_nonzero_integer_and_rounding_mode,
    pairs_of_signed_limb_and_nonzero_integer_var_1,
    triples_of_integer_nonzero_signed_and_rounding_mode_var_1,
    triples_of_signed_nonzero_integer_and_rounding_mode_var_1,
};
use malachite_test::integer::arithmetic::div_round_signed_limb::num_div_round_signed_limb_floor;
use num::BigInt;
#[cfg(feature = "32_bit_limbs")]
use rug::{self, ops::DivRounding};
use std::str::FromStr;

#[test]
fn test_div_round_signed_limb() {
    let test = |i, j: SignedLimb, rm: RoundingMode, quotient| {
        let mut n = Integer::from_str(i).unwrap();
        n.div_round_assign(j, rm);
        assert_eq!(n.to_string(), quotient);
        assert!(n.is_valid());

        let q = Integer::from_str(i).unwrap().div_round(j, rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = (&Integer::from_str(i).unwrap()).div_round(j, rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        match rm {
            #[cfg(feature = "32_bit_limbs")]
            RoundingMode::Down => {
                assert_eq!(
                    rug::Integer::from_str(i).unwrap().div_trunc(j).to_string(),
                    quotient
                );
            }
            RoundingMode::Floor => {
                assert_eq!(
                    num_div_round_signed_limb_floor(BigInt::from_str(i).unwrap(), j).to_string(),
                    quotient
                );
                #[cfg(feature = "32_bit_limbs")]
                assert_eq!(
                    rug::Integer::from_str(i).unwrap().div_floor(j).to_string(),
                    quotient
                );
            }
            #[cfg(feature = "32_bit_limbs")]
            RoundingMode::Ceiling => {
                assert_eq!(
                    rug::Integer::from_str(i).unwrap().div_ceil(j).to_string(),
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
        2_147_483_647,
        RoundingMode::Down,
        "465661287524579",
    );
    test(
        "1000000000000000000000000",
        2_147_483_647,
        RoundingMode::Floor,
        "465661287524579",
    );
    test(
        "1000000000000000000000000",
        2_147_483_647,
        RoundingMode::Up,
        "465661287524580",
    );
    test(
        "1000000000000000000000000",
        2_147_483_647,
        RoundingMode::Ceiling,
        "465661287524580",
    );
    test(
        "1000000000000000000000000",
        2_147_483_647,
        RoundingMode::Nearest,
        "465661287524580",
    );

    test("-1", 1, RoundingMode::Down, "-1");
    test("-1", 1, RoundingMode::Floor, "-1");
    test("-1", 1, RoundingMode::Up, "-1");
    test("-1", 1, RoundingMode::Ceiling, "-1");
    test("-1", 1, RoundingMode::Nearest, "-1");
    test("-1", 1, RoundingMode::Exact, "-1");

    test("-123", 1, RoundingMode::Down, "-123");
    test("-123", 1, RoundingMode::Floor, "-123");
    test("-123", 1, RoundingMode::Up, "-123");
    test("-123", 1, RoundingMode::Ceiling, "-123");
    test("-123", 1, RoundingMode::Nearest, "-123");
    test("-123", 1, RoundingMode::Exact, "-123");

    test("-123", 2, RoundingMode::Down, "-61");
    test("-123", 2, RoundingMode::Floor, "-62");
    test("-123", 2, RoundingMode::Up, "-62");
    test("-123", 2, RoundingMode::Ceiling, "-61");
    test("-123", 2, RoundingMode::Nearest, "-62");

    test("-125", 2, RoundingMode::Down, "-62");
    test("-125", 2, RoundingMode::Floor, "-63");
    test("-125", 2, RoundingMode::Up, "-63");
    test("-125", 2, RoundingMode::Ceiling, "-62");
    test("-125", 2, RoundingMode::Nearest, "-62");

    test("-123", 123, RoundingMode::Down, "-1");
    test("-123", 123, RoundingMode::Floor, "-1");
    test("-123", 123, RoundingMode::Up, "-1");
    test("-123", 123, RoundingMode::Ceiling, "-1");
    test("-123", 123, RoundingMode::Nearest, "-1");
    test("-123", 123, RoundingMode::Exact, "-1");

    test("-123", 456, RoundingMode::Down, "0");
    test("-123", 456, RoundingMode::Floor, "-1");
    test("-123", 456, RoundingMode::Up, "-1");
    test("-123", 456, RoundingMode::Ceiling, "0");
    test("-123", 456, RoundingMode::Nearest, "0");

    test("-1000000000000", 1, RoundingMode::Down, "-1000000000000");
    test("-1000000000000", 1, RoundingMode::Floor, "-1000000000000");
    test("-1000000000000", 1, RoundingMode::Up, "-1000000000000");
    test("-1000000000000", 1, RoundingMode::Ceiling, "-1000000000000");
    test("-1000000000000", 1, RoundingMode::Nearest, "-1000000000000");
    test("-1000000000000", 1, RoundingMode::Exact, "-1000000000000");

    test("-1000000000000", 3, RoundingMode::Down, "-333333333333");
    test("-1000000000000", 3, RoundingMode::Floor, "-333333333334");
    test("-1000000000000", 3, RoundingMode::Up, "-333333333334");
    test("-1000000000000", 3, RoundingMode::Ceiling, "-333333333333");
    test("-1000000000000", 3, RoundingMode::Nearest, "-333333333333");

    test("-999999999999", 2, RoundingMode::Down, "-499999999999");
    test("-999999999999", 2, RoundingMode::Floor, "-500000000000");
    test("-999999999999", 2, RoundingMode::Up, "-500000000000");
    test("-999999999999", 2, RoundingMode::Ceiling, "-499999999999");
    test("-999999999999", 2, RoundingMode::Nearest, "-500000000000");

    test("-1000000000001", 2, RoundingMode::Down, "-500000000000");
    test("-1000000000001", 2, RoundingMode::Floor, "-500000000001");
    test("-1000000000001", 2, RoundingMode::Up, "-500000000001");
    test("-1000000000001", 2, RoundingMode::Ceiling, "-500000000000");
    test("-1000000000001", 2, RoundingMode::Nearest, "-500000000000");

    test(
        "-1000000000000000000000000",
        2_147_483_647,
        RoundingMode::Down,
        "-465661287524579",
    );
    test(
        "-1000000000000000000000000",
        2_147_483_647,
        RoundingMode::Floor,
        "-465661287524580",
    );
    test(
        "-1000000000000000000000000",
        2_147_483_647,
        RoundingMode::Up,
        "-465661287524580",
    );
    test(
        "-1000000000000000000000000",
        2_147_483_647,
        RoundingMode::Ceiling,
        "-465661287524579",
    );
    test(
        "-1000000000000000000000000",
        2_147_483_647,
        RoundingMode::Nearest,
        "-465661287524580",
    );

    test("0", -1, RoundingMode::Down, "0");
    test("0", -1, RoundingMode::Floor, "0");
    test("0", -1, RoundingMode::Up, "0");
    test("0", -1, RoundingMode::Ceiling, "0");
    test("0", -1, RoundingMode::Nearest, "0");
    test("0", -1, RoundingMode::Exact, "0");

    test("0", -123, RoundingMode::Down, "0");
    test("0", -123, RoundingMode::Floor, "0");
    test("0", -123, RoundingMode::Up, "0");
    test("0", -123, RoundingMode::Ceiling, "0");
    test("0", -123, RoundingMode::Nearest, "0");

    test("1", -1, RoundingMode::Down, "-1");
    test("1", -1, RoundingMode::Floor, "-1");
    test("1", -1, RoundingMode::Up, "-1");
    test("1", -1, RoundingMode::Ceiling, "-1");
    test("1", -1, RoundingMode::Nearest, "-1");
    test("1", -1, RoundingMode::Exact, "-1");

    test("123", -1, RoundingMode::Down, "-123");
    test("123", -1, RoundingMode::Floor, "-123");
    test("123", -1, RoundingMode::Up, "-123");
    test("123", -1, RoundingMode::Ceiling, "-123");
    test("123", -1, RoundingMode::Nearest, "-123");
    test("123", -1, RoundingMode::Exact, "-123");

    test("123", -2, RoundingMode::Down, "-61");
    test("123", -2, RoundingMode::Floor, "-62");
    test("123", -2, RoundingMode::Up, "-62");
    test("123", -2, RoundingMode::Ceiling, "-61");
    test("123", -2, RoundingMode::Nearest, "-62");

    test("125", -2, RoundingMode::Down, "-62");
    test("125", -2, RoundingMode::Floor, "-63");
    test("125", -2, RoundingMode::Up, "-63");
    test("125", -2, RoundingMode::Ceiling, "-62");
    test("125", -2, RoundingMode::Nearest, "-62");

    test("123", -123, RoundingMode::Down, "-1");
    test("123", -123, RoundingMode::Floor, "-1");
    test("123", -123, RoundingMode::Up, "-1");
    test("123", -123, RoundingMode::Ceiling, "-1");
    test("123", -123, RoundingMode::Nearest, "-1");
    test("123", -123, RoundingMode::Exact, "-1");

    test("123", -456, RoundingMode::Down, "0");
    test("123", -456, RoundingMode::Floor, "-1");
    test("123", -456, RoundingMode::Up, "-1");
    test("123", -456, RoundingMode::Ceiling, "0");
    test("123", -456, RoundingMode::Nearest, "0");

    test("1000000000000", -1, RoundingMode::Down, "-1000000000000");
    test("1000000000000", -1, RoundingMode::Floor, "-1000000000000");
    test("1000000000000", -1, RoundingMode::Up, "-1000000000000");
    test("1000000000000", -1, RoundingMode::Ceiling, "-1000000000000");
    test("1000000000000", -1, RoundingMode::Nearest, "-1000000000000");
    test("1000000000000", -1, RoundingMode::Exact, "-1000000000000");

    test("1000000000000", -3, RoundingMode::Down, "-333333333333");
    test("1000000000000", -3, RoundingMode::Floor, "-333333333334");
    test("1000000000000", -3, RoundingMode::Up, "-333333333334");
    test("1000000000000", -3, RoundingMode::Ceiling, "-333333333333");
    test("1000000000000", -3, RoundingMode::Nearest, "-333333333333");

    test("999999999999", -2, RoundingMode::Down, "-499999999999");
    test("999999999999", -2, RoundingMode::Floor, "-500000000000");
    test("999999999999", -2, RoundingMode::Up, "-500000000000");
    test("999999999999", -2, RoundingMode::Ceiling, "-499999999999");
    test("999999999999", -2, RoundingMode::Nearest, "-500000000000");

    test("1000000000001", -2, RoundingMode::Down, "-500000000000");
    test("1000000000001", -2, RoundingMode::Floor, "-500000000001");
    test("1000000000001", -2, RoundingMode::Up, "-500000000001");
    test("1000000000001", -2, RoundingMode::Ceiling, "-500000000000");
    test("1000000000001", -2, RoundingMode::Nearest, "-500000000000");

    test(
        "1000000000000000000000000",
        -2_147_483_647,
        RoundingMode::Down,
        "-465661287524579",
    );
    test(
        "1000000000000000000000000",
        -2_147_483_647,
        RoundingMode::Floor,
        "-465661287524580",
    );
    test(
        "1000000000000000000000000",
        -2_147_483_647,
        RoundingMode::Up,
        "-465661287524580",
    );
    test(
        "1000000000000000000000000",
        -2_147_483_647,
        RoundingMode::Ceiling,
        "-465661287524579",
    );
    test(
        "1000000000000000000000000",
        -2_147_483_647,
        RoundingMode::Nearest,
        "-465661287524580",
    );

    test(
        "1000000000000000000000000",
        -2_147_483_648,
        RoundingMode::Down,
        "-465661287307739",
    );
    test(
        "1000000000000000000000000",
        -2_147_483_648,
        RoundingMode::Floor,
        "-465661287307740",
    );
    test(
        "1000000000000000000000000",
        -2_147_483_648,
        RoundingMode::Up,
        "-465661287307740",
    );
    test(
        "1000000000000000000000000",
        -2_147_483_648,
        RoundingMode::Ceiling,
        "-465661287307739",
    );
    test(
        "1000000000000000000000000",
        -2_147_483_648,
        RoundingMode::Nearest,
        "-465661287307739",
    );

    test("-1", -1, RoundingMode::Down, "1");
    test("-1", -1, RoundingMode::Floor, "1");
    test("-1", -1, RoundingMode::Up, "1");
    test("-1", -1, RoundingMode::Ceiling, "1");
    test("-1", -1, RoundingMode::Nearest, "1");
    test("-1", -1, RoundingMode::Exact, "1");

    test("-123", -1, RoundingMode::Down, "123");
    test("-123", -1, RoundingMode::Floor, "123");
    test("-123", -1, RoundingMode::Up, "123");
    test("-123", -1, RoundingMode::Ceiling, "123");
    test("-123", -1, RoundingMode::Nearest, "123");
    test("-123", -1, RoundingMode::Exact, "123");

    test("-123", -2, RoundingMode::Down, "61");
    test("-123", -2, RoundingMode::Floor, "61");
    test("-123", -2, RoundingMode::Up, "62");
    test("-123", -2, RoundingMode::Ceiling, "62");
    test("-123", -2, RoundingMode::Nearest, "62");

    test("-125", -2, RoundingMode::Down, "62");
    test("-125", -2, RoundingMode::Floor, "62");
    test("-125", -2, RoundingMode::Up, "63");
    test("-125", -2, RoundingMode::Ceiling, "63");
    test("-125", -2, RoundingMode::Nearest, "62");

    test("-123", -123, RoundingMode::Down, "1");
    test("-123", -123, RoundingMode::Floor, "1");
    test("-123", -123, RoundingMode::Up, "1");
    test("-123", -123, RoundingMode::Ceiling, "1");
    test("-123", -123, RoundingMode::Nearest, "1");
    test("-123", -123, RoundingMode::Exact, "1");

    test("-123", -456, RoundingMode::Down, "0");
    test("-123", -456, RoundingMode::Floor, "0");
    test("-123", -456, RoundingMode::Up, "1");
    test("-123", -456, RoundingMode::Ceiling, "1");
    test("-123", -456, RoundingMode::Nearest, "0");

    test("-1000000000000", -1, RoundingMode::Down, "1000000000000");
    test("-1000000000000", -1, RoundingMode::Floor, "1000000000000");
    test("-1000000000000", -1, RoundingMode::Up, "1000000000000");
    test("-1000000000000", -1, RoundingMode::Ceiling, "1000000000000");
    test("-1000000000000", -1, RoundingMode::Nearest, "1000000000000");
    test("-1000000000000", -1, RoundingMode::Exact, "1000000000000");

    test("-1000000000000", -3, RoundingMode::Down, "333333333333");
    test("-1000000000000", -3, RoundingMode::Floor, "333333333333");
    test("-1000000000000", -3, RoundingMode::Up, "333333333334");
    test("-1000000000000", -3, RoundingMode::Ceiling, "333333333334");
    test("-1000000000000", -3, RoundingMode::Nearest, "333333333333");

    test("-999999999999", -2, RoundingMode::Down, "499999999999");
    test("-999999999999", -2, RoundingMode::Floor, "499999999999");
    test("-999999999999", -2, RoundingMode::Up, "500000000000");
    test("-999999999999", -2, RoundingMode::Ceiling, "500000000000");
    test("-999999999999", -2, RoundingMode::Nearest, "500000000000");

    test("-1000000000001", -2, RoundingMode::Down, "500000000000");
    test("-1000000000001", -2, RoundingMode::Floor, "500000000000");
    test("-1000000000001", -2, RoundingMode::Up, "500000000001");
    test("-1000000000001", -2, RoundingMode::Ceiling, "500000000001");
    test("-1000000000001", -2, RoundingMode::Nearest, "500000000000");

    test(
        "-1000000000000000000000000",
        -2_147_483_647,
        RoundingMode::Down,
        "465661287524579",
    );
    test(
        "-1000000000000000000000000",
        -2_147_483_647,
        RoundingMode::Floor,
        "465661287524579",
    );
    test(
        "-1000000000000000000000000",
        -2_147_483_647,
        RoundingMode::Up,
        "465661287524580",
    );
    test(
        "-1000000000000000000000000",
        -2_147_483_647,
        RoundingMode::Ceiling,
        "465661287524580",
    );
    test(
        "-1000000000000000000000000",
        -2_147_483_647,
        RoundingMode::Nearest,
        "465661287524580",
    );

    test(
        "-1000000000000000000000000",
        -2_147_483_648,
        RoundingMode::Down,
        "465661287307739",
    );
    test(
        "-1000000000000000000000000",
        -2_147_483_648,
        RoundingMode::Floor,
        "465661287307739",
    );
    test(
        "-1000000000000000000000000",
        -2_147_483_648,
        RoundingMode::Up,
        "465661287307740",
    );
    test(
        "-1000000000000000000000000",
        -2_147_483_648,
        RoundingMode::Ceiling,
        "465661287307740",
    );
    test(
        "-1000000000000000000000000",
        -2_147_483_648,
        RoundingMode::Nearest,
        "465661287307739",
    );
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_round_assign_signed_limb_fail_1() {
    let mut n = Integer::from(10);
    n.div_round_assign(0 as SignedLimb, RoundingMode::Floor);
}

#[test]
#[should_panic(expected = "Division is not exact")]
fn div_round_assign_signed_limb_fail_2() {
    let mut n = Integer::from(10);
    n.div_round_assign(3 as SignedLimb, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_round_signed_limb_fail_1() {
    Integer::from(10).div_round(0 as SignedLimb, RoundingMode::Floor);
}

#[test]
#[should_panic(expected = "Division is not exact")]
fn div_round_signed_limb_fail_2() {
    Integer::from(10).div_round(3 as SignedLimb, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_round_signed_limb_ref_fail_1() {
    (&Integer::from(10)).div_round(0 as SignedLimb, RoundingMode::Floor);
}

#[test]
#[should_panic(expected = "Division is not exact: 10 / 3")]
fn div_round_signed_limb_ref_fail_2() {
    (&Integer::from(10)).div_round(3 as SignedLimb, RoundingMode::Exact);
}

#[test]
fn test_signed_limb_div_round_integer() {
    let test = |i: SignedLimb, j, rm, quotient| {
        let q = i.div_round(Integer::from_str(j).unwrap(), rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = i.div_round(&Integer::from_str(j).unwrap(), rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
    };
    test(0, "1", RoundingMode::Down, "0");
    test(0, "1", RoundingMode::Floor, "0");
    test(0, "1", RoundingMode::Up, "0");
    test(0, "1", RoundingMode::Ceiling, "0");
    test(0, "1", RoundingMode::Nearest, "0");
    test(0, "1", RoundingMode::Exact, "0");

    test(0, "123", RoundingMode::Down, "0");
    test(0, "123", RoundingMode::Floor, "0");
    test(0, "123", RoundingMode::Up, "0");
    test(0, "123", RoundingMode::Ceiling, "0");
    test(0, "123", RoundingMode::Nearest, "0");

    test(1, "1", RoundingMode::Down, "1");
    test(1, "1", RoundingMode::Floor, "1");
    test(1, "1", RoundingMode::Up, "1");
    test(1, "1", RoundingMode::Ceiling, "1");
    test(1, "1", RoundingMode::Nearest, "1");
    test(1, "1", RoundingMode::Exact, "1");

    test(123, "1", RoundingMode::Down, "123");
    test(123, "1", RoundingMode::Floor, "123");
    test(123, "1", RoundingMode::Up, "123");
    test(123, "1", RoundingMode::Ceiling, "123");
    test(123, "1", RoundingMode::Nearest, "123");
    test(123, "1", RoundingMode::Exact, "123");

    test(123, "2", RoundingMode::Down, "61");
    test(123, "2", RoundingMode::Floor, "61");
    test(123, "2", RoundingMode::Up, "62");
    test(123, "2", RoundingMode::Ceiling, "62");
    test(123, "2", RoundingMode::Nearest, "62");

    test(125, "2", RoundingMode::Down, "62");
    test(125, "2", RoundingMode::Floor, "62");
    test(125, "2", RoundingMode::Up, "63");
    test(125, "2", RoundingMode::Ceiling, "63");
    test(125, "2", RoundingMode::Nearest, "62");

    test(123, "123", RoundingMode::Down, "1");
    test(123, "123", RoundingMode::Floor, "1");
    test(123, "123", RoundingMode::Up, "1");
    test(123, "123", RoundingMode::Ceiling, "1");
    test(123, "123", RoundingMode::Nearest, "1");
    test(123, "123", RoundingMode::Exact, "1");

    test(123, "456", RoundingMode::Down, "0");
    test(123, "456", RoundingMode::Floor, "0");
    test(123, "456", RoundingMode::Up, "1");
    test(123, "456", RoundingMode::Ceiling, "1");
    test(123, "456", RoundingMode::Nearest, "0");

    test(123, "1000000000000", RoundingMode::Down, "0");
    test(123, "1000000000000", RoundingMode::Floor, "0");
    test(123, "1000000000000", RoundingMode::Up, "1");
    test(123, "1000000000000", RoundingMode::Ceiling, "1");
    test(123, "1000000000000", RoundingMode::Nearest, "0");

    test(300_000_000, "599999999", RoundingMode::Down, "0");
    test(300_000_000, "599999999", RoundingMode::Floor, "0");
    test(300_000_000, "599999999", RoundingMode::Up, "1");
    test(300_000_000, "599999999", RoundingMode::Ceiling, "1");
    test(300_000_000, "599999999", RoundingMode::Nearest, "1");

    test(300_000_000, "600000000", RoundingMode::Down, "0");
    test(300_000_000, "600000000", RoundingMode::Floor, "0");
    test(300_000_000, "600000000", RoundingMode::Up, "1");
    test(300_000_000, "600000000", RoundingMode::Ceiling, "1");
    test(300_000_000, "600000000", RoundingMode::Nearest, "0");

    test(300_000_000, "600000001", RoundingMode::Down, "0");
    test(300_000_000, "600000001", RoundingMode::Floor, "0");
    test(300_000_000, "600000001", RoundingMode::Up, "1");
    test(300_000_000, "600000001", RoundingMode::Ceiling, "1");
    test(300_000_000, "600000001", RoundingMode::Nearest, "0");

    test(0, "-1", RoundingMode::Down, "0");
    test(0, "-1", RoundingMode::Floor, "0");
    test(0, "-1", RoundingMode::Up, "0");
    test(0, "-1", RoundingMode::Ceiling, "0");
    test(0, "-1", RoundingMode::Nearest, "0");
    test(0, "-1", RoundingMode::Exact, "0");

    test(0, "-123", RoundingMode::Down, "0");
    test(0, "-123", RoundingMode::Floor, "0");
    test(0, "-123", RoundingMode::Up, "0");
    test(0, "-123", RoundingMode::Ceiling, "0");
    test(0, "-123", RoundingMode::Nearest, "0");

    test(1, "-1", RoundingMode::Down, "-1");
    test(1, "-1", RoundingMode::Floor, "-1");
    test(1, "-1", RoundingMode::Up, "-1");
    test(1, "-1", RoundingMode::Ceiling, "-1");
    test(1, "-1", RoundingMode::Nearest, "-1");
    test(1, "-1", RoundingMode::Exact, "-1");

    test(123, "-1", RoundingMode::Down, "-123");
    test(123, "-1", RoundingMode::Floor, "-123");
    test(123, "-1", RoundingMode::Up, "-123");
    test(123, "-1", RoundingMode::Ceiling, "-123");
    test(123, "-1", RoundingMode::Nearest, "-123");
    test(123, "-1", RoundingMode::Exact, "-123");

    test(123, "-2", RoundingMode::Down, "-61");
    test(123, "-2", RoundingMode::Floor, "-62");
    test(123, "-2", RoundingMode::Up, "-62");
    test(123, "-2", RoundingMode::Ceiling, "-61");
    test(123, "-2", RoundingMode::Nearest, "-62");

    test(125, "-2", RoundingMode::Down, "-62");
    test(125, "-2", RoundingMode::Floor, "-63");
    test(125, "-2", RoundingMode::Up, "-63");
    test(125, "-2", RoundingMode::Ceiling, "-62");
    test(125, "-2", RoundingMode::Nearest, "-62");

    test(123, "-123", RoundingMode::Down, "-1");
    test(123, "-123", RoundingMode::Floor, "-1");
    test(123, "-123", RoundingMode::Up, "-1");
    test(123, "-123", RoundingMode::Ceiling, "-1");
    test(123, "-123", RoundingMode::Nearest, "-1");
    test(123, "-123", RoundingMode::Exact, "-1");

    test(123, "-456", RoundingMode::Down, "0");
    test(123, "-456", RoundingMode::Floor, "-1");
    test(123, "-456", RoundingMode::Up, "-1");
    test(123, "-456", RoundingMode::Ceiling, "0");
    test(123, "-456", RoundingMode::Nearest, "0");

    test(123, "-1000000000000", RoundingMode::Down, "0");
    test(123, "-1000000000000", RoundingMode::Floor, "-1");
    test(123, "-1000000000000", RoundingMode::Up, "-1");
    test(123, "-1000000000000", RoundingMode::Ceiling, "0");
    test(123, "-1000000000000", RoundingMode::Nearest, "0");

    test(300_000_000, "-599999999", RoundingMode::Down, "0");
    test(300_000_000, "-599999999", RoundingMode::Floor, "-1");
    test(300_000_000, "-599999999", RoundingMode::Up, "-1");
    test(300_000_000, "-599999999", RoundingMode::Ceiling, "0");
    test(300_000_000, "-599999999", RoundingMode::Nearest, "-1");

    test(300_000_000, "-600000000", RoundingMode::Down, "0");
    test(300_000_000, "-600000000", RoundingMode::Floor, "-1");
    test(300_000_000, "-600000000", RoundingMode::Up, "-1");
    test(300_000_000, "-600000000", RoundingMode::Ceiling, "0");
    test(300_000_000, "-600000000", RoundingMode::Nearest, "0");

    test(300_000_000, "-600000001", RoundingMode::Down, "0");
    test(300_000_000, "-600000001", RoundingMode::Floor, "-1");
    test(300_000_000, "-600000001", RoundingMode::Up, "-1");
    test(300_000_000, "-600000001", RoundingMode::Ceiling, "0");
    test(300_000_000, "-600000001", RoundingMode::Nearest, "0");
}

#[test]
#[should_panic(expected = "division by zero")]
fn signed_limb_div_round_integer_fail_1() {
    (10 as SignedLimb).div_round(Integer::ZERO, RoundingMode::Floor);
}

#[test]
#[should_panic(expected = "Division is not exact")]
fn signed_limb_div_round_integer_fail_2() {
    (10 as SignedLimb).div_round(Integer::from(3), RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "division by zero")]
fn signed_limb_div_round_integer_ref_fail_1() {
    (10 as SignedLimb).div_round(&Integer::ZERO, RoundingMode::Floor);
}

#[test]
#[should_panic(expected = "Division is not exact: 10 / 3")]
fn signed_limb_div_round_integer_ref_fail_2() {
    (10 as SignedLimb).div_round(&Integer::from(3), RoundingMode::Exact);
}

#[test]
fn div_round_signed_limb_properties() {
    test_properties(
        triples_of_integer_nonzero_signed_and_rounding_mode_var_1,
        |&(ref n, i, rm): &(Integer, SignedLimb, RoundingMode)| {
            let mut mut_n = n.clone();
            mut_n.div_round_assign(i, rm);
            assert!(mut_n.is_valid());
            let quotient = mut_n;

            let quotient_alt = n.div_round(i, rm);
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);

            let quotient_alt = n.clone().div_round(i, rm);
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);

            assert!(quotient.le_abs(n));

            assert_eq!(-(-n).div_round(i, -rm), quotient);

            //TODO assert_eq!(n.div_round(Integer::from(u), rm), quotient);
        },
    );

    test_properties(
        pairs_of_integer_and_nonzero_signed::<SignedLimb>,
        |&(ref n, i)| {
            let left_multiplied = n * i;
            assert_eq!((&left_multiplied).div_round(i, RoundingMode::Down), *n);
            assert_eq!((&left_multiplied).div_round(i, RoundingMode::Up), *n);
            assert_eq!((&left_multiplied).div_round(i, RoundingMode::Floor), *n);
            assert_eq!((&left_multiplied).div_round(i, RoundingMode::Ceiling), *n);
            assert_eq!((&left_multiplied).div_round(i, RoundingMode::Nearest), *n);
            assert_eq!((&left_multiplied).div_round(i, RoundingMode::Exact), *n);

            #[cfg(feature = "32_bit_limbs")]
            assert_eq!(
                rug_integer_to_integer(&integer_to_rug_integer(n).div_trunc(i)),
                n.div_round(i, RoundingMode::Down)
            );
            assert_eq!(
                bigint_to_integer(&num_div_round_signed_limb_floor(integer_to_bigint(n), i)),
                n.div_round(i, RoundingMode::Floor)
            );
            #[cfg(feature = "32_bit_limbs")]
            {
                assert_eq!(
                    rug_integer_to_integer(&integer_to_rug_integer(n).div_floor(i)),
                    n.div_round(i, RoundingMode::Floor)
                );
                assert_eq!(
                    rug_integer_to_integer(&integer_to_rug_integer(n).div_ceil(i)),
                    n.div_round(i, RoundingMode::Ceiling)
                );
            }
            assert_eq!(
                n.ceiling_div_mod(i).0,
                n.div_round(i, RoundingMode::Ceiling),
            );
        },
    );

    // TODO test using Rationals
    test_properties(
        pairs_of_integer_and_nonzero_signed_limb_var_2,
        |&(ref n, i)| {
            let down = n.div_round(i, RoundingMode::Down);
            let up = if (*n >= 0 as Limb) == (i >= 0) {
                &down + 1 as Limb
            } else {
                &down - 1 as Limb
            };
            let floor = n.div_round(i, RoundingMode::Floor);
            let ceiling = &floor + 1 as Limb;
            assert_eq!(n.div_round(i, RoundingMode::Up), up);
            assert_eq!(n.div_round(i, RoundingMode::Ceiling), ceiling);
            let nearest = n.div_round(i, RoundingMode::Nearest);
            assert!(nearest == down || nearest == up);
        },
    );

    test_properties(pairs_of_integer_and_rounding_mode, |&(ref n, rm)| {
        assert_eq!(n.div_round(1 as SignedLimb, rm), *n);
    });

    test_properties(
        pairs_of_nonzero_integer_and_rounding_mode,
        |&(ref n, rm)| {
            assert_eq!((0 as SignedLimb).div_round(n, rm), 0 as Limb);
        },
    );

    test_properties(
        pairs_of_nonzero_signed_and_rounding_mode::<SignedLimb>,
        |&(i, rm)| {
            assert_eq!(Integer::ZERO.div_round(i, rm), 0 as Limb);
            assert_eq!(i.div_round(Integer::from(i), rm), 1 as Limb);
            assert_eq!(Integer::from(i).div_round(i, rm), 1 as Limb);
            assert_eq!(i.div_round(-Integer::from(i), rm), -1 as SignedLimb);
            assert_eq!((-Integer::from(i)).div_round(i, rm), -1 as SignedLimb);
        },
    );

    test_properties(
        triples_of_signed_nonzero_integer_and_rounding_mode_var_1,
        |&(i, ref n, rm): &(SignedLimb, Integer, RoundingMode)| {
            let quotient = i.div_round(n, rm);
            assert!(quotient.is_valid());

            let quotient_alt = i.div_round(n.clone(), rm);
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);

            assert!(quotient.le_abs(&i));

            //TODO assert_eq!(Integer::from(u).div_round(n, rm), quotient);
        },
    );

    // TODO test using Rationals
    test_properties(
        pairs_of_signed_limb_and_nonzero_integer_var_1,
        |&(i, ref n)| {
            let down = i.div_round(n, RoundingMode::Down);
            let up = if (*n >= 0 as Limb) == (i >= 0) {
                &down + 1 as Limb
            } else {
                &down - 1 as Limb
            };
            let floor = i.div_round(n, RoundingMode::Floor);
            let ceiling = &floor + 1 as Limb;
            assert_eq!(i.div_round(n, RoundingMode::Up), up);
            assert_eq!(i.div_round(n, RoundingMode::Ceiling), ceiling);
            let nearest = i.div_round(n, RoundingMode::Nearest);
            assert!(nearest == down || nearest == up);
        },
    );

    test_properties(
        pairs_of_signed_and_rounding_mode::<SignedLimb>,
        |&(i, rm)| {
            assert_eq!(i.div_round(Integer::ONE, rm), i);
            assert_eq!(i.div_round(Integer::NEGATIVE_ONE, rm), -Integer::from(i));
        },
    );

    test_properties(
        triples_of_signed_limb_nonzero_signed_limb_and_rounding_mode_var_1,
        |&(x, y, rm)| {
            let quotient = x.div_round(y, rm);
            assert_eq!(quotient, Integer::from(x).div_round(y, rm));
            assert_eq!(quotient, x.div_round(Integer::from(y), rm));
        },
    );
}
