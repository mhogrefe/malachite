use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{CeilingDivMod, DivRound, DivRoundAssign};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::round::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use num::BigInt;
#[cfg(feature = "32_bit_limbs")]
use rug::{self, ops::DivRounding};

use common::test_properties;
use malachite_test::common::{bigint_to_integer, integer_to_bigint};
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{
    pairs_of_positive_unsigned_and_rounding_mode, pairs_of_unsigned_and_rounding_mode,
    triples_of_limb_positive_limb_and_rounding_mode_var_1,
};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_positive_limb_var_2, pairs_of_integer_and_positive_unsigned,
    pairs_of_integer_and_rounding_mode, pairs_of_limb_and_nonzero_integer_var_1,
    pairs_of_nonzero_integer_and_rounding_mode,
    triples_of_integer_positive_unsigned_and_rounding_mode_var_1,
    triples_of_unsigned_nonzero_integer_and_rounding_mode_var_1,
};
use malachite_test::inputs::natural::{
    triples_of_natural_positive_unsigned_and_rounding_mode_var_1,
    triples_of_unsigned_positive_natural_and_rounding_mode_var_1,
};
use malachite_test::integer::arithmetic::div_round_limb::num_div_round_limb_floor;

#[test]
fn test_div_round_limb() {
    let test = |u, v: Limb, rm: RoundingMode, quotient| {
        let mut n = Integer::from_str(u).unwrap();
        n.div_round_assign(v, rm);
        assert_eq!(n.to_string(), quotient);
        assert!(n.is_valid());

        let q = Integer::from_str(u).unwrap().div_round(v, rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = (&Integer::from_str(u).unwrap()).div_round(v, rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        match rm {
            #[cfg(feature = "32_bit_limbs")]
            RoundingMode::Down => {
                assert_eq!(
                    rug::Integer::from_str(u).unwrap().div_trunc(v).to_string(),
                    quotient
                );
            }
            RoundingMode::Floor => {
                assert_eq!(
                    num_div_round_limb_floor(BigInt::from_str(u).unwrap(), v).to_string(),
                    quotient
                );
                #[cfg(feature = "32_bit_limbs")]
                assert_eq!(
                    rug::Integer::from_str(u).unwrap().div_floor(v).to_string(),
                    quotient
                );
            }
            #[cfg(feature = "32_bit_limbs")]
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
        4_294_967_295,
        RoundingMode::Down,
        "-232830643708079",
    );
    test(
        "-1000000000000000000000000",
        4_294_967_295,
        RoundingMode::Floor,
        "-232830643708080",
    );
    test(
        "-1000000000000000000000000",
        4_294_967_295,
        RoundingMode::Up,
        "-232830643708080",
    );
    test(
        "-1000000000000000000000000",
        4_294_967_295,
        RoundingMode::Ceiling,
        "-232830643708079",
    );
    test(
        "-1000000000000000000000000",
        4_294_967_295,
        RoundingMode::Nearest,
        "-232830643708080",
    );
}

#[test]
#[should_panic]
fn div_round_assign_limb_fail_1() {
    let mut n = Integer::from(10);
    n.div_round_assign(0 as Limb, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn div_round_assign_limb_fail_2() {
    let mut n = Integer::from(10);
    n.div_round_assign(3 as Limb, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn div_round_limb_fail_1() {
    Integer::from(10).div_round(0 as Limb, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn div_round_limb_fail_2() {
    Integer::from(10).div_round(3 as Limb, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn div_round_limb_ref_fail_1() {
    (&Integer::from(10)).div_round(0 as Limb, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn div_round_limb_ref_fail_2() {
    (&Integer::from(10)).div_round(3 as Limb, RoundingMode::Exact);
}

#[test]
fn test_limb_div_round_integer() {
    let test = |u: Limb, v, rm, quotient| {
        let q = u.div_round(Integer::from_str(v).unwrap(), rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = u.div_round(&Integer::from_str(v).unwrap(), rm);
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

    test(3_000_000_000, "5999999999", RoundingMode::Down, "0");
    test(3_000_000_000, "5999999999", RoundingMode::Floor, "0");
    test(3_000_000_000, "5999999999", RoundingMode::Up, "1");
    test(3_000_000_000, "5999999999", RoundingMode::Ceiling, "1");
    test(3_000_000_000, "5999999999", RoundingMode::Nearest, "1");

    test(3_000_000_000, "6000000000", RoundingMode::Down, "0");
    test(3_000_000_000, "6000000000", RoundingMode::Floor, "0");
    test(3_000_000_000, "6000000000", RoundingMode::Up, "1");
    test(3_000_000_000, "6000000000", RoundingMode::Ceiling, "1");
    test(3_000_000_000, "6000000000", RoundingMode::Nearest, "0");

    test(3_000_000_000, "6000000001", RoundingMode::Down, "0");
    test(3_000_000_000, "6000000001", RoundingMode::Floor, "0");
    test(3_000_000_000, "6000000001", RoundingMode::Up, "1");
    test(3_000_000_000, "6000000001", RoundingMode::Ceiling, "1");
    test(3_000_000_000, "6000000001", RoundingMode::Nearest, "0");

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

    test(3_000_000_000, "-5999999999", RoundingMode::Down, "0");
    test(3_000_000_000, "-5999999999", RoundingMode::Floor, "-1");
    test(3_000_000_000, "-5999999999", RoundingMode::Up, "-1");
    test(3_000_000_000, "-5999999999", RoundingMode::Ceiling, "0");
    test(3_000_000_000, "-5999999999", RoundingMode::Nearest, "-1");

    test(3_000_000_000, "-6000000000", RoundingMode::Down, "0");
    test(3_000_000_000, "-6000000000", RoundingMode::Floor, "-1");
    test(3_000_000_000, "-6000000000", RoundingMode::Up, "-1");
    test(3_000_000_000, "-6000000000", RoundingMode::Ceiling, "0");
    test(3_000_000_000, "-6000000000", RoundingMode::Nearest, "0");

    test(3_000_000_000, "-6000000001", RoundingMode::Down, "0");
    test(3_000_000_000, "-6000000001", RoundingMode::Floor, "-1");
    test(3_000_000_000, "-6000000001", RoundingMode::Up, "-1");
    test(3_000_000_000, "-6000000001", RoundingMode::Ceiling, "0");
    test(3_000_000_000, "-6000000001", RoundingMode::Nearest, "0");
}

#[test]
#[should_panic]
fn limb_div_round_integer_fail_1() {
    (10 as Limb).div_round(Integer::ZERO, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn limb_div_round_integer_fail_2() {
    (10 as Limb).div_round(Integer::from(3), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn limb_div_round_integer_ref_fail_1() {
    (10 as Limb).div_round(&Integer::ZERO, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn limb_div_round_integer_ref_fail_2() {
    (10 as Limb).div_round(&Integer::from(3), RoundingMode::Exact);
}

#[test]
fn div_round_limb_properties() {
    test_properties(
        triples_of_integer_positive_unsigned_and_rounding_mode_var_1,
        |&(ref n, u, rm): &(Integer, Limb, RoundingMode)| {
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

            assert!(quotient.le_abs(n));

            assert_eq!(-(-n).div_round(u, -rm), quotient);

            //TODO assert_eq!(n.div_round(Integer::from(u), rm), quotient);
        },
    );

    test_properties(
        pairs_of_integer_and_positive_unsigned::<Limb>,
        |&(ref n, u)| {
            let left_multiplied = n * Integer::from(u);
            assert_eq!((&left_multiplied).div_round(u, RoundingMode::Down), *n);
            assert_eq!((&left_multiplied).div_round(u, RoundingMode::Up), *n);
            assert_eq!((&left_multiplied).div_round(u, RoundingMode::Floor), *n);
            assert_eq!((&left_multiplied).div_round(u, RoundingMode::Ceiling), *n);
            assert_eq!((&left_multiplied).div_round(u, RoundingMode::Nearest), *n);
            assert_eq!((&left_multiplied).div_round(u, RoundingMode::Exact), *n);

            #[cfg(feature = "32_bit_limbs")]
            assert_eq!(
                rug_integer_to_integer(&integer_to_rug_integer(n).div_trunc(u)),
                n.div_round(u, RoundingMode::Down)
            );
            assert_eq!(
                bigint_to_integer(&num_div_round_limb_floor(integer_to_bigint(n), u)),
                n.div_round(u, RoundingMode::Floor)
            );
            #[cfg(feature = "32_bit_limbs")]
            {
                assert_eq!(
                    rug_integer_to_integer(&integer_to_rug_integer(n).div_floor(u)),
                    n.div_round(u, RoundingMode::Floor)
                );
                assert_eq!(
                    rug_integer_to_integer(&integer_to_rug_integer(n).div_ceil(u)),
                    n.div_round(u, RoundingMode::Ceiling)
                );
            }
            assert_eq!(
                n.ceiling_div_mod(u).0,
                n.div_round(u, RoundingMode::Ceiling)
            );
        },
    );

    // TODO test using Rationals
    test_properties(pairs_of_integer_and_positive_limb_var_2, |&(ref n, u)| {
        let down = n.div_round(u, RoundingMode::Down);
        let up = if *n >= 0 as Limb {
            &down + Integer::ONE
        } else {
            &down - Integer::ONE
        };
        let floor = n.div_round(u, RoundingMode::Floor);
        let ceiling = &floor + Integer::ONE;
        assert_eq!(n.div_round(u, RoundingMode::Up), up);
        assert_eq!(n.div_round(u, RoundingMode::Ceiling), ceiling);
        let nearest = n.div_round(u, RoundingMode::Nearest);
        assert!(nearest == down || nearest == up);
    });

    test_properties(pairs_of_integer_and_rounding_mode, |&(ref n, rm)| {
        assert_eq!(n.div_round(1 as Limb, rm), *n);
    });

    test_properties(
        pairs_of_nonzero_integer_and_rounding_mode,
        |&(ref n, rm)| {
            assert_eq!((0 as Limb).div_round(n, rm), 0 as Limb);
        },
    );

    test_properties(
        pairs_of_positive_unsigned_and_rounding_mode::<Limb>,
        |&(u, rm)| {
            assert_eq!(Integer::ZERO.div_round(u, rm), 0 as Limb);
            assert_eq!(u.div_round(Integer::from(u), rm), 1 as Limb);
            assert_eq!(Integer::from(u).div_round(u, rm), 1 as Limb);
            assert_eq!(u.div_round(-Natural::from(u), rm), -1 as SignedLimb);
            assert_eq!((-Natural::from(u)).div_round(u, rm), -1 as SignedLimb);
        },
    );

    test_properties(
        triples_of_unsigned_nonzero_integer_and_rounding_mode_var_1,
        |&(u, ref n, rm): &(Limb, Integer, RoundingMode)| {
            let quotient = u.div_round(n, rm);
            assert!(quotient.is_valid());

            let quotient_alt = u.div_round(n.clone(), rm);
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);

            assert!(quotient <= u);

            //TODO assert_eq!(Integer::from(u).div_round(n, rm), quotient);
        },
    );

    // TODO test using Rationals
    test_properties(pairs_of_limb_and_nonzero_integer_var_1, |&(u, ref n)| {
        let down = u.div_round(n, RoundingMode::Down);
        let up = if *n >= 0 as Limb {
            &down + Integer::ONE
        } else {
            &down - Integer::ONE
        };
        let floor = u.div_round(n, RoundingMode::Floor);
        let ceiling = &floor + Integer::ONE;
        assert_eq!(u.div_round(n, RoundingMode::Up), up);
        assert_eq!(u.div_round(n, RoundingMode::Ceiling), ceiling);
        let nearest = u.div_round(n, RoundingMode::Nearest);
        assert!(nearest == down || nearest == up);
    });

    test_properties(pairs_of_unsigned_and_rounding_mode::<Limb>, |&(u, rm)| {
        assert_eq!(u.div_round(Integer::ONE, rm), u);
        assert_eq!(u.div_round(Integer::NEGATIVE_ONE, rm), -Natural::from(u));
    });

    test_properties(
        triples_of_limb_positive_limb_and_rounding_mode_var_1,
        |&(x, y, rm)| {
            let quotient = x.div_round(y, rm);
            assert_eq!(Integer::from(x).div_round(y, rm), quotient);
            assert_eq!(x.div_round(Integer::from(y), rm), quotient);
        },
    );

    test_properties(
        triples_of_natural_positive_unsigned_and_rounding_mode_var_1::<Limb>,
        |&(ref n, u, rm)| {
            assert_eq!(n.div_round(u, rm), Integer::from(n).div_round(u, rm));
        },
    );

    test_properties(
        triples_of_unsigned_positive_natural_and_rounding_mode_var_1::<Limb>,
        |&(u, ref n, rm)| {
            assert_eq!(u.div_round(n, rm), u.div_round(Integer::from(n), rm));
        },
    );
}
