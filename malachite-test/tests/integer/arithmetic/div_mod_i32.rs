use common::test_properties;
use malachite_base::num::{
    CeilingDivAssignMod, CeilingDivMod, CeilingMod, DivAssignMod, DivAssignRem, DivMod, DivRem,
    DivRound, Mod, NegativeOne, One, PartialOrdAbs, UnsignedAbs, Zero,
};
use malachite_base::round::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::nonzero_signeds;
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_nonzero_i32_var_1, pairs_of_integer_and_nonzero_signed,
    pairs_of_signed_and_nonzero_integer,
};
use malachite_test::integer::arithmetic::div_mod_i32::{
    num_div_mod_i32, num_div_rem_i32, rug_ceiling_div_mod_i32, rug_div_mod_i32, rug_div_rem_i32,
};
use num::BigInt;
use rug;
use std::str::FromStr;

#[test]
fn test_div_mod_i32() {
    let test = |u, v: i32, quotient, remainder| {
        let mut n = Integer::from_str(u).unwrap();
        let r = n.div_assign_mod(v);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = Integer::from_str(u).unwrap().div_mod(v);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = (&Integer::from_str(u).unwrap()).div_mod(v);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = num_div_mod_i32(BigInt::from_str(u).unwrap(), v);
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r.to_string(), remainder);

        let (q, r) = rug_div_mod_i32(rug::Integer::from_str(u).unwrap(), v);
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r.to_string(), remainder);

        let (q, r) = (
            Integer::from_str(u)
                .unwrap()
                .div_round(v, RoundingMode::Floor),
            Integer::from_str(u).unwrap().mod_op(v),
        );
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r.to_string(), remainder);
    };
    test("0", 1, "0", "0");
    test("0", 123, "0", "0");
    test("1", 1, "1", "0");
    test("123", 1, "123", "0");
    test("123", 123, "1", "0");
    test("123", 456, "0", "123");
    test("456", 123, "3", "87");
    test("2147483647", 1, "2147483647", "0");
    test("2147483647", 2_147_483_647, "1", "0");
    test("1000000000000", 1, "1000000000000", "0");
    test("1000000000000", 3, "333333333333", "1");
    test("1000000000000", 123, "8130081300", "100");
    test("1000000000000", 2_147_483_647, "465", "1420104145");
    test(
        "1000000000000000000000000",
        1,
        "1000000000000000000000000",
        "0",
    );
    test(
        "1000000000000000000000000",
        3,
        "333333333333333333333333",
        "1",
    );
    test(
        "1000000000000000000000000",
        123,
        "8130081300813008130081",
        "37",
    );
    test(
        "1000000000000000000000000",
        2_147_483_647,
        "465661287524579",
        "1486940387",
    );

    test("-1", 1, "-1", "0");
    test("-123", 1, "-123", "0");
    test("-123", 123, "-1", "0");
    test("-123", 456, "-1", "333");
    test("-456", 123, "-4", "36");
    test("-2147483647", 1, "-2147483647", "0");
    test("-2147483647", 2_147_483_647, "-1", "0");
    test("-1000000000000", 1, "-1000000000000", "0");
    test("-1000000000000", 3, "-333333333334", "2");
    test("-1000000000000", 123, "-8130081301", "23");
    test("-1000000000000", 2_147_483_647, "-466", "727379502");
    test(
        "-1000000000000000000000000",
        1,
        "-1000000000000000000000000",
        "0",
    );
    test(
        "-1000000000000000000000000",
        3,
        "-333333333333333333333334",
        "2",
    );
    test(
        "-1000000000000000000000000",
        123,
        "-8130081300813008130082",
        "86",
    );
    test(
        "-1000000000000000000000000",
        2_147_483_647,
        "-465661287524580",
        "660543260",
    );

    test("0", -1, "0", "0");
    test("0", -123, "0", "0");
    test("1", -1, "-1", "0");
    test("123", -1, "-123", "0");
    test("123", -123, "-1", "0");
    test("123", -456, "-1", "-333");
    test("456", -123, "-4", "-36");
    test("2147483647", -1, "-2147483647", "0");
    test("2147483647", -2_147_483_647, "-1", "0");
    test("2147483648", -2_147_483_648, "-1", "0");
    test("1000000000000", -1, "-1000000000000", "0");
    test("1000000000000", -3, "-333333333334", "-2");
    test("1000000000000", -123, "-8130081301", "-23");
    test("1000000000000", -2_147_483_647, "-466", "-727379502");
    test("1000000000000", -2_147_483_648, "-466", "-727379968");
    test(
        "1000000000000000000000000",
        -1,
        "-1000000000000000000000000",
        "0",
    );
    test(
        "1000000000000000000000000",
        -3,
        "-333333333333333333333334",
        "-2",
    );
    test(
        "1000000000000000000000000",
        -123,
        "-8130081300813008130082",
        "-86",
    );
    test(
        "1000000000000000000000000",
        -2_147_483_647,
        "-465661287524580",
        "-660543260",
    );
    test(
        "1000000000000000000000000",
        -2_147_483_648,
        "-465661287307740",
        "-1593835520",
    );

    test("-1", -1, "1", "0");
    test("-123", -1, "123", "0");
    test("-123", -123, "1", "0");
    test("-123", -456, "0", "-123");
    test("-456", -123, "3", "-87");
    test("-2147483647", -1, "2147483647", "0");
    test("-2147483647", -2_147_483_647, "1", "0");
    test("-2147483648", -2_147_483_648, "1", "0");
    test("-1000000000000", -1, "1000000000000", "0");
    test("-1000000000000", -3, "333333333333", "-1");
    test("-1000000000000", -123, "8130081300", "-100");
    test("-1000000000000", -2_147_483_647, "465", "-1420104145");
    test("-1000000000000", -2_147_483_648, "465", "-1420103680");
    test(
        "-1000000000000000000000000",
        -1,
        "1000000000000000000000000",
        "0",
    );
    test(
        "-1000000000000000000000000",
        -3,
        "333333333333333333333333",
        "-1",
    );
    test(
        "-1000000000000000000000000",
        -123,
        "8130081300813008130081",
        "-37",
    );
    test(
        "-1000000000000000000000000",
        -2_147_483_647,
        "465661287524579",
        "-1486940387",
    );
    test(
        "-1000000000000000000000000",
        -2_147_483_648,
        "465661287307739",
        "-553648128",
    );
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_assign_mod_i32_fail() {
    Integer::from(10i32).div_assign_mod(0i32);
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_mod_i32_fail() {
    Integer::from(10i32).div_mod(0i32);
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_mod_i32_ref_fail() {
    (&Integer::from(10i32)).div_mod(0i32);
}

#[test]
fn test_div_rem_i32() {
    let test = |u, v: i32, quotient, remainder| {
        let mut n = Integer::from_str(u).unwrap();
        let r = n.div_assign_rem(v);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = Integer::from_str(u).unwrap().div_rem(v);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = (&Integer::from_str(u).unwrap()).div_rem(v);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = num_div_rem_i32(BigInt::from_str(u).unwrap(), v);
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r.to_string(), remainder);

        let (q, r) = rug_div_rem_i32(rug::Integer::from_str(u).unwrap(), v);
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r.to_string(), remainder);

        let (q, r) = (
            Integer::from_str(u).unwrap() / v,
            Integer::from_str(u).unwrap() % v,
        );
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r.to_string(), remainder);
    };
    test("0", 1, "0", "0");
    test("0", 123, "0", "0");
    test("1", 1, "1", "0");
    test("123", 1, "123", "0");
    test("123", 123, "1", "0");
    test("123", 456, "0", "123");
    test("456", 123, "3", "87");
    test("2147483647", 1, "2147483647", "0");
    test("2147483647", 2_147_483_647, "1", "0");
    test("1000000000000", 1, "1000000000000", "0");
    test("1000000000000", 3, "333333333333", "1");
    test("1000000000000", 123, "8130081300", "100");
    test("1000000000000", 2_147_483_647, "465", "1420104145");
    test(
        "1000000000000000000000000",
        1,
        "1000000000000000000000000",
        "0",
    );
    test(
        "1000000000000000000000000",
        3,
        "333333333333333333333333",
        "1",
    );
    test(
        "1000000000000000000000000",
        123,
        "8130081300813008130081",
        "37",
    );
    test(
        "1000000000000000000000000",
        2_147_483_647,
        "465661287524579",
        "1486940387",
    );

    test("-1", 1, "-1", "0");
    test("-123", 1, "-123", "0");
    test("-123", 123, "-1", "0");
    test("-123", 456, "0", "-123");
    test("-456", 123, "-3", "-87");
    test("-2147483647", 1, "-2147483647", "0");
    test("-2147483647", 2_147_483_647, "-1", "0");
    test("-1000000000000", 1, "-1000000000000", "0");
    test("-1000000000000", 3, "-333333333333", "-1");
    test("-1000000000000", 123, "-8130081300", "-100");
    test("-1000000000000", 2_147_483_647, "-465", "-1420104145");
    test(
        "-1000000000000000000000000",
        1,
        "-1000000000000000000000000",
        "0",
    );
    test(
        "-1000000000000000000000000",
        3,
        "-333333333333333333333333",
        "-1",
    );
    test(
        "-1000000000000000000000000",
        123,
        "-8130081300813008130081",
        "-37",
    );
    test(
        "-1000000000000000000000000",
        2_147_483_647,
        "-465661287524579",
        "-1486940387",
    );

    test("0", -1, "0", "0");
    test("0", -123, "0", "0");
    test("1", -1, "-1", "0");
    test("123", -1, "-123", "0");
    test("123", -123, "-1", "0");
    test("123", -456, "0", "123");
    test("456", -123, "-3", "87");
    test("2147483647", -1, "-2147483647", "0");
    test("2147483647", -2_147_483_647, "-1", "0");
    test("2147483648", -2_147_483_648, "-1", "0");
    test("1000000000000", -1, "-1000000000000", "0");
    test("1000000000000", -3, "-333333333333", "1");
    test("1000000000000", -123, "-8130081300", "100");
    test("1000000000000", -2_147_483_647, "-465", "1420104145");
    test("1000000000000", -2_147_483_648, "-465", "1420103680");
    test(
        "1000000000000000000000000",
        -1,
        "-1000000000000000000000000",
        "0",
    );
    test(
        "1000000000000000000000000",
        -3,
        "-333333333333333333333333",
        "1",
    );
    test(
        "1000000000000000000000000",
        -123,
        "-8130081300813008130081",
        "37",
    );
    test(
        "1000000000000000000000000",
        -2_147_483_647,
        "-465661287524579",
        "1486940387",
    );
    test(
        "1000000000000000000000000",
        -2_147_483_648,
        "-465661287307739",
        "553648128",
    );

    test("-1", -1, "1", "0");
    test("-123", -1, "123", "0");
    test("-123", -123, "1", "0");
    test("-123", -456, "0", "-123");
    test("-456", -123, "3", "-87");
    test("-2147483647", -1, "2147483647", "0");
    test("-2147483647", -2_147_483_647, "1", "0");
    test("-1000000000000", -1, "1000000000000", "0");
    test("-1000000000000", -3, "333333333333", "-1");
    test("-1000000000000", -123, "8130081300", "-100");
    test("-1000000000000", -2_147_483_647, "465", "-1420104145");
    test("-1000000000000", -2_147_483_648, "465", "-1420103680");
    test(
        "-1000000000000000000000000",
        -1,
        "1000000000000000000000000",
        "0",
    );
    test(
        "-1000000000000000000000000",
        -3,
        "333333333333333333333333",
        "-1",
    );
    test(
        "-1000000000000000000000000",
        -123,
        "8130081300813008130081",
        "-37",
    );
    test(
        "-1000000000000000000000000",
        -2_147_483_647,
        "465661287524579",
        "-1486940387",
    );
    test(
        "-1000000000000000000000000",
        -2_147_483_648,
        "465661287307739",
        "-553648128",
    );
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_assign_rem_i32_fail() {
    Integer::from(10i32).div_assign_rem(0i32);
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_rem_i32_fail() {
    Integer::from(10i32).div_rem(0i32);
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_rem_i32_ref_fail() {
    (&Integer::from(10i32)).div_rem(0i32);
}

#[test]
fn test_ceiling_div_mod_i32() {
    let test = |u, v: i32, quotient, remainder| {
        let mut n = Integer::from_str(u).unwrap();
        let r = n.ceiling_div_assign_mod(v);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = Integer::from_str(u).unwrap().ceiling_div_mod(v);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = (&Integer::from_str(u).unwrap()).ceiling_div_mod(v);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = rug_ceiling_div_mod_i32(rug::Integer::from_str(u).unwrap(), v);
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r.to_string(), remainder);
    };
    test("0", 1, "0", "0");
    test("0", 123, "0", "0");
    test("1", 1, "1", "0");
    test("123", 1, "123", "0");
    test("123", 123, "1", "0");
    test("123", 456, "1", "-333");
    test("456", 123, "4", "-36");
    test("2147483647", 1, "2147483647", "0");
    test("2147483647", 2_147_483_647, "1", "0");
    test("1000000000000", 1, "1000000000000", "0");
    test("1000000000000", 3, "333333333334", "-2");
    test("1000000000000", 123, "8130081301", "-23");
    test("1000000000000", 2_147_483_647, "466", "-727379502");
    test(
        "1000000000000000000000000",
        1,
        "1000000000000000000000000",
        "0",
    );
    test(
        "1000000000000000000000000",
        3,
        "333333333333333333333334",
        "-2",
    );
    test(
        "1000000000000000000000000",
        123,
        "8130081300813008130082",
        "-86",
    );
    test(
        "1000000000000000000000000",
        2_147_483_647,
        "465661287524580",
        "-660543260",
    );

    test("-1", 1, "-1", "0");
    test("-123", 1, "-123", "0");
    test("-123", 123, "-1", "0");
    test("-123", 456, "0", "-123");
    test("-456", 123, "-3", "-87");
    test("-2147483647", 1, "-2147483647", "0");
    test("-2147483647", 2_147_483_647, "-1", "0");
    test("-1000000000000", 1, "-1000000000000", "0");
    test("-1000000000000", 3, "-333333333333", "-1");
    test("-1000000000000", 123, "-8130081300", "-100");
    test("-1000000000000", 2_147_483_647, "-465", "-1420104145");
    test(
        "-1000000000000000000000000",
        1,
        "-1000000000000000000000000",
        "0",
    );
    test(
        "-1000000000000000000000000",
        3,
        "-333333333333333333333333",
        "-1",
    );
    test(
        "-1000000000000000000000000",
        123,
        "-8130081300813008130081",
        "-37",
    );
    test(
        "-1000000000000000000000000",
        2_147_483_647,
        "-465661287524579",
        "-1486940387",
    );

    test("0", -1, "0", "0");
    test("0", -123, "0", "0");
    test("1", -1, "-1", "0");
    test("123", -1, "-123", "0");
    test("123", -123, "-1", "0");
    test("123", -456, "0", "123");
    test("456", -123, "-3", "87");
    test("2147483647", -1, "-2147483647", "0");
    test("2147483647", -2_147_483_647, "-1", "0");
    test("2147483648", -2_147_483_648, "-1", "0");
    test("1000000000000", -1, "-1000000000000", "0");
    test("1000000000000", -3, "-333333333333", "1");
    test("1000000000000", -123, "-8130081300", "100");
    test("1000000000000", -2_147_483_647, "-465", "1420104145");
    test("1000000000000", -2_147_483_648, "-465", "1420103680");
    test(
        "1000000000000000000000000",
        -1,
        "-1000000000000000000000000",
        "0",
    );
    test(
        "1000000000000000000000000",
        -3,
        "-333333333333333333333333",
        "1",
    );
    test(
        "1000000000000000000000000",
        -123,
        "-8130081300813008130081",
        "37",
    );
    test(
        "1000000000000000000000000",
        -2_147_483_647,
        "-465661287524579",
        "1486940387",
    );
    test(
        "1000000000000000000000000",
        -2_147_483_648,
        "-465661287307739",
        "553648128",
    );

    test("-1", -1, "1", "0");
    test("-123", -1, "123", "0");
    test("-123", -123, "1", "0");
    test("-123", -456, "1", "333");
    test("-456", -123, "4", "36");
    test("-2147483647", -1, "2147483647", "0");
    test("-2147483647", -2_147_483_647, "1", "0");
    test("-1000000000000", -1, "1000000000000", "0");
    test("-1000000000000", -3, "333333333334", "2");
    test("-1000000000000", -123, "8130081301", "23");
    test("-1000000000000", -2_147_483_647, "466", "727379502");
    test("-1000000000000", -2_147_483_648, "466", "727379968");
    test(
        "-1000000000000000000000000",
        -1,
        "1000000000000000000000000",
        "0",
    );
    test(
        "-1000000000000000000000000",
        -3,
        "333333333333333333333334",
        "2",
    );
    test(
        "-1000000000000000000000000",
        -123,
        "8130081300813008130082",
        "86",
    );
    test(
        "-1000000000000000000000000",
        -2_147_483_647,
        "465661287524580",
        "660543260",
    );
    test(
        "-1000000000000000000000000",
        -2_147_483_648,
        "465661287307740",
        "1593835520",
    );
}

#[test]
#[should_panic(expected = "division by zero")]
fn ceiling_div_assign_mod_i32_fail() {
    Integer::from(10i32).ceiling_div_assign_mod(0i32);
}

#[test]
#[should_panic(expected = "division by zero")]
fn ceiling_div_mod_i32_fail() {
    Integer::from(10i32).ceiling_div_mod(0i32);
}

#[test]
#[should_panic(expected = "division by zero")]
fn ceiling_div_mod_i32_ref_fail() {
    (&Integer::from(10i32)).ceiling_div_mod(0i32);
}

#[test]
fn test_i32_div_mod_integer() {
    let test = |i: i32, j, quotient, remainder| {
        let (q, r) = i.div_mod(Integer::from_str(j).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = i.div_mod(&Integer::from_str(j).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
    };
    test(0, "1", "0", "0");
    test(0, "123", "0", "0");
    test(1, "1", "1", "0");
    test(123, "1", "123", "0");
    test(123, "123", "1", "0");
    test(123, "456", "0", "123");
    test(456, "123", "3", "87");
    test(2_147_483_647, "1", "2147483647", "0");
    test(2_147_483_647, "2147483647", "1", "0");
    test(0, "1000000000000", "0", "0");
    test(123, "1000000000000", "0", "123");

    test(1, "-1", "-1", "0");
    test(123, "-1", "-123", "0");
    test(123, "-123", "-1", "0");
    test(123, "-456", "-1", "-333");
    test(456, "-123", "-4", "-36");
    test(2_147_483_647, "-1", "-2147483647", "0");
    test(2_147_483_647, "-2147483647", "-1", "0");
    test(0, "-1000000000000", "0", "0");
    test(123, "-1000000000000", "-1", "-999999999877");

    test(-1, "1", "-1", "0");
    test(-123, "1", "-123", "0");
    test(-123, "123", "-1", "0");
    test(-123, "456", "-1", "333");
    test(-456, "123", "-4", "36");
    test(-2_147_483_647, "1", "-2147483647", "0");
    test(-2_147_483_647, "2147483647", "-1", "0");
    test(-2_147_483_648, "2147483648", "-1", "0");
    test(-123, "1000000000000", "-1", "999999999877");

    test(-1, "-1", "1", "0");
    test(-123, "-1", "123", "0");
    test(-123, "-123", "1", "0");
    test(-123, "-456", "0", "-123");
    test(-456, "-123", "3", "-87");
    test(-2_147_483_647, "-1", "2147483647", "0");
    test(-2_147_483_648, "-1", "2147483648", "0");
    test(-2_147_483_647, "-2147483647", "1", "0");
    test(-2_147_483_648, "-2147483648", "1", "0");
    test(-123, "-1000000000000", "0", "-123");
}

#[test]
#[should_panic(expected = "division by zero")]
fn i32_div_mod_integer_fail() {
    10i32.div_mod(Integer::ZERO);
}

#[test]
#[should_panic(expected = "division by zero")]
fn i32_div_mod_integer_ref_fail() {
    10i32.div_mod(&Integer::ZERO);
}

#[test]
fn test_i32_div_rem_integer() {
    let test = |i: i32, j, quotient, remainder| {
        let (q, r) = i.div_rem(Integer::from_str(j).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = i.div_rem(&Integer::from_str(j).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
    };
    test(0, "1", "0", "0");
    test(0, "123", "0", "0");
    test(1, "1", "1", "0");
    test(123, "1", "123", "0");
    test(123, "123", "1", "0");
    test(123, "456", "0", "123");
    test(456, "123", "3", "87");
    test(2_147_483_647, "1", "2147483647", "0");
    test(2_147_483_647, "2147483647", "1", "0");
    test(0, "1000000000000", "0", "0");
    test(123, "1000000000000", "0", "123");

    test(1, "-1", "-1", "0");
    test(123, "-1", "-123", "0");
    test(123, "-123", "-1", "0");
    test(123, "-456", "0", "123");
    test(456, "-123", "-3", "87");
    test(2_147_483_647, "-1", "-2147483647", "0");
    test(2_147_483_647, "-2147483647", "-1", "0");
    test(0, "-1000000000000", "0", "0");
    test(123, "-1000000000000", "0", "123");

    test(-1, "1", "-1", "0");
    test(-123, "1", "-123", "0");
    test(-123, "123", "-1", "0");
    test(-123, "456", "0", "-123");
    test(-456, "123", "-3", "-87");
    test(-2_147_483_647, "1", "-2147483647", "0");
    test(-2_147_483_647, "2147483647", "-1", "0");
    test(-2_147_483_648, "2147483648", "-1", "0");
    test(-123, "1000000000000", "0", "-123");

    test(-1, "-1", "1", "0");
    test(-123, "-1", "123", "0");
    test(-123, "-123", "1", "0");
    test(-123, "-456", "0", "-123");
    test(-456, "-123", "3", "-87");
    test(-2_147_483_647, "-1", "2147483647", "0");
    test(-2_147_483_648, "-1", "2147483648", "0");
    test(-2_147_483_647, "-2147483647", "1", "0");
    test(-2_147_483_648, "-2147483648", "1", "0");
    test(-123, "-1000000000000", "0", "-123");
}

#[test]
#[should_panic(expected = "division by zero")]
fn i32_div_rem_integer_fail() {
    10i32.div_rem(Integer::ZERO);
}

#[test]
#[should_panic(expected = "division by zero")]
fn i32_div_rem_integer_ref_fail() {
    10i32.div_rem(&Integer::ZERO);
}

#[test]
fn test_i32_ceiling_div_mod_integer() {
    let test = |i: i32, j, quotient, remainder| {
        let (q, r) = i.ceiling_div_mod(Integer::from_str(j).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = i.ceiling_div_mod(&Integer::from_str(j).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
    };
    test(0, "1", "0", "0");
    test(0, "123", "0", "0");
    test(1, "1", "1", "0");
    test(123, "1", "123", "0");
    test(123, "123", "1", "0");
    test(123, "456", "1", "-333");
    test(456, "123", "4", "-36");
    test(2_147_483_647, "1", "2147483647", "0");
    test(2_147_483_647, "2147483647", "1", "0");
    test(0, "1000000000000", "0", "0");
    test(123, "1000000000000", "1", "-999999999877");

    test(1, "-1", "-1", "0");
    test(123, "-1", "-123", "0");
    test(123, "-123", "-1", "0");
    test(123, "-456", "0", "123");
    test(456, "-123", "-3", "87");
    test(2_147_483_647, "-1", "-2147483647", "0");
    test(2_147_483_647, "-2147483647", "-1", "0");
    test(0, "-1000000000000", "0", "0");
    test(123, "-1000000000000", "0", "123");

    test(-1, "1", "-1", "0");
    test(-123, "1", "-123", "0");
    test(-123, "123", "-1", "0");
    test(-123, "456", "0", "-123");
    test(-456, "123", "-3", "-87");
    test(-2_147_483_647, "1", "-2147483647", "0");
    test(-2_147_483_648, "1", "-2147483648", "0");
    test(-2_147_483_647, "2147483647", "-1", "0");
    test(-2_147_483_648, "2147483648", "-1", "0");
    test(-123, "1000000000000", "0", "-123");

    test(-1, "-1", "1", "0");
    test(-123, "-1", "123", "0");
    test(-123, "-123", "1", "0");
    test(-123, "-456", "1", "333");
    test(-456, "-123", "4", "36");
    test(-2_147_483_647, "-1", "2147483647", "0");
    test(-2_147_483_648, "-1", "2147483648", "0");
    test(-2_147_483_647, "-2147483647", "1", "0");
    test(-2_147_483_648, "-2147483648", "1", "0");
    test(-123, "-1000000000000", "1", "999999999877");
}

#[test]
#[should_panic(expected = "division by zero")]
fn i32_ceiling_div_mod_integer_fail() {
    10i32.ceiling_div_mod(Integer::ZERO);
}

#[test]
#[should_panic(expected = "division by zero")]
fn i32_ceiling_div_mod_integer_ref_fail() {
    10i32.ceiling_div_mod(&Integer::ZERO);
}

fn div_mod_i32_properties_helper(n: &Integer, i: i32) {
    let mut mut_n = n.clone();
    let remainder = mut_n.div_assign_mod(i);
    assert!(mut_n.is_valid());
    let quotient = mut_n;

    let (quotient_alt, remainder_alt) = n.div_mod(i);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = n.clone().div_mod(i);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = (n.div_round(i, RoundingMode::Floor), n.mod_op(i));
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    //TODO assert_eq!(n.div_mod(Integer::from(u)), (quotient.clone(), remainder));

    let (num_quotient, num_remainder) = num_div_mod_i32(integer_to_bigint(n), i);
    assert_eq!(bigint_to_integer(&num_quotient), quotient);
    assert_eq!(bigint_to_integer(&num_remainder), remainder);

    let (rug_quotient, rug_remainder) = rug_div_mod_i32(integer_to_rug_integer(n), i);
    assert_eq!(rug_integer_to_integer(&rug_quotient), quotient);
    assert_eq!(rug_integer_to_integer(&rug_remainder), remainder);

    assert!(remainder < i.unsigned_abs());
    assert!(remainder == 0 || (remainder > 0) == (i > 0));
    assert_eq!(quotient * i + remainder, *n);

    let (neg_quotient, neg_remainder) = (-n).div_mod(i);
    assert_eq!(n.ceiling_div_mod(i), (-neg_quotient, -neg_remainder));
}

#[test]
fn div_mod_i32_properties() {
    test_properties(
        pairs_of_integer_and_nonzero_signed,
        |&(ref n, i): &(Integer, i32)| {
            div_mod_i32_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_integer_and_nonzero_i32_var_1,
        |&(ref n, i): &(Integer, i32)| {
            div_mod_i32_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_signed_and_nonzero_integer,
        |&(i, ref n): &(i32, Integer)| {
            let (quotient, remainder) = i.div_mod(n);
            assert!(quotient.is_valid());
            assert!(remainder.is_valid());

            let (quotient_alt, remainder_alt) = i.div_mod(n.clone());
            assert!(quotient_alt.is_valid());
            assert!(remainder_alt.is_valid());
            assert_eq!(quotient_alt, quotient);
            assert_eq!(remainder_alt, remainder);

            let (quotient_alt, remainder_alt) = (i.div_round(n, RoundingMode::Floor), i.mod_op(n));
            assert_eq!(quotient_alt, quotient);
            assert_eq!(remainder_alt, remainder);

            //TODO assert_eq!((quotient.clone(), remainder), Natural::from(u).div_mod(n));

            if i > 0 && i < *n {
                assert_eq!(remainder, i.unsigned_abs());
            }
            assert!(remainder.lt_abs(n));
            assert!(remainder == 0 || (remainder > 0) == (*n > 0));
            assert_eq!(&quotient * n + &remainder, i);

            let (neg_quotient, neg_remainder) = i.div_mod(-n);
            assert_eq!(i.ceiling_div_mod(n), (-neg_quotient, neg_remainder));
        },
    );

    test_properties(integers, |n| {
        let (q, r) = n.div_mod(1i32);
        assert_eq!(q, *n);
        assert_eq!(r, 0);

        let (q, r) = n.div_mod(-1);
        assert_eq!(q, -n);
        assert_eq!(r, 0u32);
    });

    test_properties(nonzero_signeds, |&i: &i32| {
        assert_eq!(i.div_mod(Integer::ONE), (Integer::from(i), Integer::ZERO));
        assert_eq!(
            i.div_mod(Integer::NEGATIVE_ONE),
            (-Integer::from(i), Integer::ZERO)
        );
        assert_eq!(i.div_mod(Integer::from(i)), (Integer::ONE, Integer::ZERO));
        assert_eq!(Integer::from(i).div_mod(i), (Integer::ONE, Integer::ZERO));
        assert_eq!(
            i.div_mod(-Integer::from(i)),
            (Integer::NEGATIVE_ONE, Integer::ZERO)
        );
        assert_eq!(
            (-Integer::from(i)).div_mod(i),
            (Integer::NEGATIVE_ONE, Integer::ZERO)
        );
        assert_eq!(Integer::ZERO.div_mod(i), (Integer::ZERO, Integer::ZERO));
        if i > 1 {
            assert_eq!(Integer::ONE.div_mod(i), (Integer::ZERO, Integer::ONE));
            assert_eq!(
                Integer::NEGATIVE_ONE.div_mod(i),
                (Integer::NEGATIVE_ONE, Integer::from(i.unsigned_abs() - 1))
            );
        }
    });

    //TODO test_properties(pairs_of_signed_and_nonzero_signed, |&(i, j): &(i32, i32)| {
    //TODO     assert_eq!(i.div_mod(j), Integer::from(i).div_mod(Integer::from(j)));
    //TODO });
}

fn div_rem_i32_properties_helper(n: &Integer, i: i32) {
    let mut mut_n = n.clone();
    let remainder = mut_n.div_assign_rem(i);
    assert!(remainder.is_valid());
    assert!(mut_n.is_valid());
    let quotient = mut_n;

    let (quotient_alt, remainder_alt) = n.div_rem(i);
    assert!(quotient_alt.is_valid());
    assert!(remainder_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = n.clone().div_rem(i);
    assert!(quotient_alt.is_valid());
    assert!(remainder_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = (n / i, n % i);
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    //TODO assert_eq!(n.div_rem(Integer::from(i)), (quotient.clone(), remainder));

    let (num_quotient, num_remainder) = num_div_rem_i32(integer_to_bigint(n), i);
    assert_eq!(bigint_to_integer(&num_quotient), quotient);
    assert_eq!(bigint_to_integer(&num_remainder), remainder);

    let (rug_quotient, rug_remainder) = rug_div_rem_i32(integer_to_rug_integer(n), i);
    assert_eq!(rug_integer_to_integer(&rug_quotient), quotient);
    assert_eq!(rug_integer_to_integer(&rug_remainder), remainder);

    assert!(remainder.lt_abs(&i));
    assert!(remainder == 0 || (remainder > 0) == (*n > 0));
    assert_eq!(&quotient * i + &remainder, *n);

    assert_eq!((-n).div_rem(i), (-quotient, -remainder));
}

#[test]
fn div_rem_i32_properties() {
    test_properties(
        pairs_of_integer_and_nonzero_signed,
        |&(ref n, i): &(Integer, i32)| {
            div_rem_i32_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_integer_and_nonzero_i32_var_1,
        |&(ref n, i): &(Integer, i32)| {
            div_rem_i32_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_signed_and_nonzero_integer,
        |&(i, ref n): &(i32, Integer)| {
            let (quotient, remainder) = i.div_rem(n);
            assert!(quotient.is_valid());
            assert!(remainder.is_valid());

            let (quotient_alt, remainder_alt) = i.div_rem(n.clone());
            assert!(quotient_alt.is_valid());
            assert!(remainder_alt.is_valid());
            assert_eq!(quotient_alt, quotient);
            assert_eq!(remainder_alt, remainder);

            let (quotient_alt, remainder_alt) = (i / n, i % n);
            assert_eq!(quotient_alt, quotient);
            assert_eq!(remainder_alt, remainder);

            //TODO assert_eq!((quotient.clone(), remainder), Natural::from(u).div_rem(n));

            if i > 0 && i.lt_abs(n) {
                assert_eq!(remainder, i);
            }
            assert!(remainder.lt_abs(n));
            assert!(remainder == 0 || (remainder > 0) == (i > 0));
            assert_eq!(&quotient * n + &remainder, i);

            assert_eq!(i.div_rem(-n), (-quotient, remainder));
        },
    );

    test_properties(integers, |n| {
        let (q, r) = n.div_rem(1i32);
        assert_eq!(q, *n);
        assert_eq!(r, 0);

        let (q, r) = n.div_rem(-1);
        assert_eq!(q, -n);
        assert_eq!(r, 0);
    });

    test_properties(nonzero_signeds, |&i: &i32| {
        assert_eq!(i.div_rem(Integer::ONE), (Integer::from(i), Integer::ZERO));
        assert_eq!(
            i.div_rem(Integer::NEGATIVE_ONE),
            (-Integer::from(i), Integer::ZERO)
        );
        assert_eq!(i.div_rem(Integer::from(i)), (Integer::ONE, Integer::ZERO));
        assert_eq!(Integer::from(i).div_rem(i), (Integer::ONE, Integer::ZERO));
        assert_eq!(
            i.div_rem(-Integer::from(i)),
            (Integer::NEGATIVE_ONE, Integer::ZERO)
        );
        assert_eq!(
            (-Integer::from(i)).div_rem(i),
            (Integer::NEGATIVE_ONE, Integer::ZERO)
        );
        assert_eq!(Integer::ZERO.div_rem(i), (Integer::ZERO, Integer::ZERO));
        if i > 1 {
            assert_eq!(Integer::ONE.div_rem(i), (Integer::ZERO, Integer::ONE));
            assert_eq!(
                Integer::NEGATIVE_ONE.div_rem(i),
                (Integer::ZERO, Integer::NEGATIVE_ONE)
            );
        }
    });
}

fn ceiling_div_mod_i32_properties_helper(n: &Integer, i: i32) {
    let mut mut_n = n.clone();
    let remainder = mut_n.ceiling_div_assign_mod(i);
    assert!(mut_n.is_valid());
    assert!(remainder.is_valid());
    let quotient = mut_n;

    let (quotient_alt, remainder_alt) = n.ceiling_div_mod(i);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = n.clone().ceiling_div_mod(i);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = (n.div_round(i, RoundingMode::Ceiling), n.ceiling_mod(i));
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    //TODO assert_eq!(n.ceiling_div_mod(Integer::from(u)), (quotient.clone(), remainder));

    let (rug_quotient, rug_remainder) = rug_ceiling_div_mod_i32(integer_to_rug_integer(n), i);
    assert_eq!(rug_integer_to_integer(&rug_quotient), quotient);
    assert_eq!(rug_integer_to_integer(&rug_remainder), remainder);

    assert!(remainder.lt_abs(&i));
    assert!(remainder == 0 || (remainder >= 0) != (i > 0));
    assert_eq!(quotient * i + remainder, *n);

    let (neg_quotient, neg_remainder) = (-n).ceiling_div_mod(i);
    assert_eq!(n.div_mod(i), (-neg_quotient, -neg_remainder));
}

#[test]
fn ceiling_div_mod_i32_properties() {
    test_properties(
        pairs_of_integer_and_nonzero_signed,
        |&(ref n, i): &(Integer, i32)| {
            ceiling_div_mod_i32_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_integer_and_nonzero_i32_var_1,
        |&(ref n, i): &(Integer, i32)| {
            ceiling_div_mod_i32_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_signed_and_nonzero_integer,
        |&(i, ref n): &(i32, Integer)| {
            let (quotient, remainder) = i.ceiling_div_mod(n);
            assert!(quotient.is_valid());
            assert!(remainder.is_valid());

            let (quotient_alt, remainder_alt) = i.ceiling_div_mod(n.clone());
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);
            assert!(remainder_alt.is_valid());
            assert_eq!(remainder_alt, remainder);

            let (quotient_alt, remainder_alt) =
                (i.div_round(n, RoundingMode::Ceiling), i.ceiling_mod(n));
            assert_eq!(quotient_alt, quotient);
            assert_eq!(remainder_alt, remainder);

            //TODO assert_eq!((quotient, remainder), Natural::from(u).ceiling_div_mod(n));

            assert!(remainder.lt_abs(n));
            assert!(remainder == 0 || (remainder >= 0) != (*n > 0));
            assert_eq!(&quotient * n + &remainder, i);

            let (neg_quotient, neg_remainder) = i.ceiling_div_mod(-n);
            assert_eq!(i.div_mod(n), (-neg_quotient, neg_remainder));
        },
    );

    test_properties(integers, |n| {
        let (q, r) = n.ceiling_div_mod(1i32);
        assert_eq!(q, *n);
        assert_eq!(r, 0);

        let (q, r) = n.ceiling_div_mod(-1);
        assert_eq!(q, -n);
        assert_eq!(r, 0);
    });

    test_properties(nonzero_signeds, |&i: &i32| {
        assert_eq!(
            i.ceiling_div_mod(Integer::ONE),
            (Integer::from(i), Integer::ZERO)
        );
        assert_eq!(
            i.ceiling_div_mod(Integer::NEGATIVE_ONE),
            (-Integer::from(i), Integer::ZERO)
        );
        assert_eq!(
            i.ceiling_div_mod(Integer::from(i)),
            (Integer::ONE, Integer::ZERO)
        );
        assert_eq!(
            Integer::from(i).ceiling_div_mod(i),
            (Integer::ONE, Integer::ZERO)
        );
        assert_eq!(
            i.ceiling_div_mod(-Integer::from(i)),
            (Integer::NEGATIVE_ONE, Integer::ZERO)
        );
        assert_eq!(
            Integer::ZERO.ceiling_div_mod(i),
            (Integer::ZERO, Integer::ZERO)
        );
    });
}
