use common::test_properties;
use malachite_base::num::{
    Abs, CeilingDivAssignMod, CeilingDivAssignNegMod, CeilingDivMod, CeilingDivNegMod,
    DivAssignMod, DivAssignRem, DivMod, DivRem, NegAssign, NegativeOne, One, PartialOrdAbs, Zero,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::positive_unsigneds;
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_positive_u32_var_1, pairs_of_integer_and_positive_unsigned,
    pairs_of_unsigned_and_nonzero_integer,
};
use malachite_test::integer::arithmetic::div_mod_u32::{
    num_div_mod_u32, num_div_rem_u32, rug_ceiling_div_mod_u32, rug_ceiling_div_neg_mod_u32,
    rug_div_mod_u32, rug_div_rem_u32,
};
use num::BigInt;
use rug;
use std::str::FromStr;

#[test]
fn test_div_mod_u32() {
    let test = |u, v: u32, quotient, remainder| {
        let mut n = Integer::from_str(u).unwrap();
        assert_eq!(n.div_assign_mod(v), remainder);
        assert_eq!(n.to_string(), quotient);
        assert!(n.is_valid());

        let (q, r) = Integer::from_str(u).unwrap().div_mod(v);
        assert_eq!(q.to_string(), quotient);
        assert!(q.is_valid());
        assert_eq!(r, remainder);

        let (q, r) = (&Integer::from_str(u).unwrap()).div_mod(v);
        assert_eq!(q.to_string(), quotient);
        assert!(q.is_valid());
        assert_eq!(r, remainder);

        let (q, r) = num_div_mod_u32(BigInt::from_str(u).unwrap(), v);
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r, remainder);

        let (q, r) = rug_div_mod_u32(rug::Integer::from_str(u).unwrap(), v);
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r, remainder);

        //TODO let (q, r) = (Integer::from_str(u).unwrap() / v, Integer::from_str(u).unwrap().mod_op(v));
        //assert_eq!(q.to_string(), quotient);
        //assert_eq!(r, remainder);
    };
    test("0", 1, "0", 0);
    test("0", 123, "0", 0);
    test("1", 1, "1", 0);
    test("123", 1, "123", 0);
    test("123", 123, "1", 0);
    test("123", 456, "0", 123);
    test("456", 123, "3", 87);
    test("4294967295", 1, "4294967295", 0);
    test("4294967295", 4_294_967_295, "1", 0);
    test("1000000000000", 1, "1000000000000", 0);
    test("1000000000000", 3, "333333333333", 1);
    test("1000000000000", 123, "8130081300", 100);
    test("1000000000000", 4_294_967_295, "232", 3_567_587_560);
    test(
        "1000000000000000000000000",
        1,
        "1000000000000000000000000",
        0,
    );
    test(
        "1000000000000000000000000",
        3,
        "333333333333333333333333",
        1,
    );
    test(
        "1000000000000000000000000",
        123,
        "8130081300813008130081",
        37,
    );
    test(
        "1000000000000000000000000",
        4_294_967_295,
        "232830643708079",
        3_167_723_695,
    );

    test("-1", 1, "-1", 0);
    test("-123", 1, "-123", 0);
    test("-123", 123, "-1", 0);
    test("-123", 456, "-1", 333);
    test("-456", 123, "-4", 36);
    test("-4294967295", 1, "-4294967295", 0);
    test("-4294967295", 4_294_967_295, "-1", 0);
    test("-1000000000000", 1, "-1000000000000", 0);
    test("-1000000000000", 3, "-333333333334", 2);
    test("-1000000000000", 123, "-8130081301", 23);
    test("-1000000000000", 4_294_967_295, "-233", 727_379_735);
    test(
        "-1000000000000000000000000",
        1,
        "-1000000000000000000000000",
        0,
    );
    test(
        "-1000000000000000000000000",
        3,
        "-333333333333333333333334",
        2,
    );
    test(
        "-1000000000000000000000000",
        123,
        "-8130081300813008130082",
        86,
    );
    test(
        "-1000000000000000000000000",
        4_294_967_295,
        "-232830643708080",
        1_127_243_600,
    );
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_assign_mod_u32_fail() {
    Integer::from(10u32).div_assign_mod(0);
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_mod_u32_fail() {
    Integer::from(10u32).div_mod(0);
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_mod_u32_ref_fail() {
    (&Integer::from(10u32)).div_mod(0);
}

#[test]
fn test_div_rem_u32() {
    let test = |u, v: u32, quotient, remainder| {
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

        let (q, r) = num_div_rem_u32(BigInt::from_str(u).unwrap(), v);
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r.to_string(), remainder);

        let (q, r) = rug_div_rem_u32(rug::Integer::from_str(u).unwrap(), v);
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r.to_string(), remainder);

        //TODO let (q, r) = (Integer::from_str(u).unwrap() / v, Integer::from_str(u).unwrap() % v);
        //assert_eq!(q.to_string(), quotient);
        //assert_eq!(r, remainder);
    };
    test("0", 1, "0", "0");
    test("0", 123, "0", "0");
    test("1", 1, "1", "0");
    test("123", 1, "123", "0");
    test("123", 123, "1", "0");
    test("123", 456, "0", "123");
    test("456", 123, "3", "87");
    test("4294967295", 1, "4294967295", "0");
    test("4294967295", 4_294_967_295, "1", "0");
    test("1000000000000", 1, "1000000000000", "0");
    test("1000000000000", 3, "333333333333", "1");
    test("1000000000000", 123, "8130081300", "100");
    test("1000000000000", 4_294_967_295, "232", "3567587560");
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
        4_294_967_295,
        "232830643708079",
        "3167723695",
    );

    test("-1", 1, "-1", "0");
    test("-123", 1, "-123", "0");
    test("-123", 123, "-1", "0");
    test("-123", 456, "0", "-123");
    test("-456", 123, "-3", "-87");
    test("-4294967295", 1, "-4294967295", "0");
    test("-4294967295", 4_294_967_295, "-1", "0");
    test("-1000000000000", 1, "-1000000000000", "0");
    test("-1000000000000", 3, "-333333333333", "-1");
    test("-1000000000000", 123, "-8130081300", "-100");
    test("-1000000000000", 4_294_967_295, "-232", "-3567587560");
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
        4_294_967_295,
        "-232830643708079",
        "-3167723695",
    );
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_assign_rem_u32_fail() {
    Integer::from(10u32).div_assign_rem(0);
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_rem_u32_fail() {
    Integer::from(10u32).div_rem(0);
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_rem_u32_ref_fail() {
    (&Integer::from(10u32)).div_rem(0);
}

#[test]
fn test_ceiling_div_neg_mod_u32() {
    let test = |u, v: u32, quotient, remainder| {
        let mut n = Integer::from_str(u).unwrap();
        assert_eq!(n.ceiling_div_assign_neg_mod(v), remainder);
        assert_eq!(n.to_string(), quotient);
        assert!(n.is_valid());

        let (q, r) = Integer::from_str(u).unwrap().ceiling_div_neg_mod(v);
        assert_eq!(q.to_string(), quotient);
        assert!(q.is_valid());
        assert_eq!(r, remainder);

        let (q, r) = (&Integer::from_str(u).unwrap()).ceiling_div_neg_mod(v);
        assert_eq!(q.to_string(), quotient);
        assert!(q.is_valid());
        assert_eq!(r, remainder);

        let (q, r) = rug_ceiling_div_neg_mod_u32(rug::Integer::from_str(u).unwrap(), v);
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r, remainder);
    };
    test("0", 1, "0", 0);
    test("0", 123, "0", 0);
    test("1", 1, "1", 0);
    test("123", 1, "123", 0);
    test("123", 123, "1", 0);
    test("123", 456, "1", 333);
    test("456", 123, "4", 36);
    test("4294967295", 1, "4294967295", 0);
    test("4294967295", 4_294_967_295, "1", 0);
    test("1000000000000", 1, "1000000000000", 0);
    test("1000000000000", 3, "333333333334", 2);
    test("1000000000000", 123, "8130081301", 23);
    test("1000000000000", 4_294_967_295, "233", 727_379_735);
    test(
        "1000000000000000000000000",
        1,
        "1000000000000000000000000",
        0,
    );
    test(
        "1000000000000000000000000",
        3,
        "333333333333333333333334",
        2,
    );
    test(
        "1000000000000000000000000",
        123,
        "8130081300813008130082",
        86,
    );
    test(
        "1000000000000000000000000",
        4_294_967_295,
        "232830643708080",
        1_127_243_600,
    );

    test("-1", 1, "-1", 0);
    test("-123", 1, "-123", 0);
    test("-123", 123, "-1", 0);
    test("-123", 456, "0", 123);
    test("-456", 123, "-3", 87);
    test("-4294967295", 1, "-4294967295", 0);
    test("-4294967295", 4_294_967_295, "-1", 0);
    test("-1000000000000", 1, "-1000000000000", 0);
    test("-1000000000000", 3, "-333333333333", 1);
    test("-1000000000000", 123, "-8130081300", 100);
    test("-1000000000000", 4_294_967_295, "-232", 3_567_587_560);
    test(
        "-1000000000000000000000000",
        1,
        "-1000000000000000000000000",
        0,
    );
    test(
        "-1000000000000000000000000",
        3,
        "-333333333333333333333333",
        1,
    );
    test(
        "-1000000000000000000000000",
        123,
        "-8130081300813008130081",
        37,
    );
    test(
        "-1000000000000000000000000",
        4_294_967_295,
        "-232830643708079",
        3_167_723_695,
    );
}

#[test]
#[should_panic(expected = "division by zero")]
fn ceiling_div_assign_neg_mod_u32_fail() {
    Integer::from(10u32).ceiling_div_assign_neg_mod(0);
}

#[test]
#[should_panic(expected = "division by zero")]
fn ceiling_div_neg_mod_u32_fail() {
    Integer::from(10u32).ceiling_div_neg_mod(0);
}

#[test]
#[should_panic(expected = "division by zero")]
fn ceiling_div_neg_mod_u32_ref_fail() {
    (&Integer::from(10u32)).ceiling_div_neg_mod(0);
}

#[test]
fn test_ceiling_div_mod_u32() {
    let test = |u, v: u32, quotient, remainder| {
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

        let (q, r) = rug_ceiling_div_mod_u32(rug::Integer::from_str(u).unwrap(), v);
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
    test("4294967295", 1, "4294967295", "0");
    test("4294967295", 4_294_967_295, "1", "0");
    test("1000000000000", 1, "1000000000000", "0");
    test("1000000000000", 3, "333333333334", "-2");
    test("1000000000000", 123, "8130081301", "-23");
    test("1000000000000", 4_294_967_295, "233", "-727379735");
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
        4_294_967_295,
        "232830643708080",
        "-1127243600",
    );

    test("-1", 1, "-1", "0");
    test("-123", 1, "-123", "0");
    test("-123", 123, "-1", "0");
    test("-123", 456, "0", "-123");
    test("-456", 123, "-3", "-87");
    test("-4294967295", 1, "-4294967295", "0");
    test("-4294967295", 4_294_967_295, "-1", "0");
    test("-1000000000000", 1, "-1000000000000", "0");
    test("-1000000000000", 3, "-333333333333", "-1");
    test("-1000000000000", 123, "-8130081300", "-100");
    test("-1000000000000", 4_294_967_295, "-232", "-3567587560");
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
        4_294_967_295,
        "-232830643708079",
        "-3167723695",
    );
}

#[test]
#[should_panic(expected = "division by zero")]
fn ceiling_div_assign_mod_u32_fail() {
    Integer::from(10u32).ceiling_div_assign_mod(0);
}

#[test]
#[should_panic(expected = "division by zero")]
fn ceiling_div_mod_u32_fail() {
    Integer::from(10u32).ceiling_div_mod(0);
}

#[test]
#[should_panic(expected = "division by zero")]
fn ceiling_div_mod_u32_ref_fail() {
    (&Integer::from(10u32)).ceiling_div_mod(0);
}

#[test]
fn test_u32_div_mod_integer() {
    let test = |u: u32, v, quotient, remainder| {
        let (q, r) = u.div_mod(Integer::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r, remainder);

        let (q, r) = u.div_mod(&Integer::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r, remainder);

        let (q, r) = u.div_rem(Integer::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r, remainder);

        let (q, r) = u.div_rem(&Integer::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r, remainder);
    };
    test(0, "1", "0", 0);
    test(0, "123", "0", 0);
    test(1, "1", "1", 0);
    test(123, "1", "123", 0);
    test(123, "123", "1", 0);
    test(123, "456", "0", 123);
    test(456, "123", "3", 87);
    test(4_294_967_295, "1", "4294967295", 0);
    test(4_294_967_295, "4294967295", "1", 0);
    test(0, "1000000000000", "0", 0);
    test(123, "1000000000000", "0", 123);

    test(1, "-1", "-1", 0);
    test(123, "-1", "-123", 0);
    test(123, "-123", "-1", 0);
    test(123, "-456", "0", 123);
    test(456, "-123", "-3", 87);
    test(4_294_967_295, "-1", "-4294967295", 0);
    test(4_294_967_295, "-4294967295", "-1", 0);
    test(0, "-1000000000000", "0", 0);
    test(123, "-1000000000000", "0", 123);
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_div_mod_integer_fail() {
    10.div_mod(Integer::ZERO);
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_div_mod_integer_ref_fail() {
    10.div_mod(&Integer::ZERO);
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_div_rem_integer_fail() {
    10.div_rem(Integer::ZERO);
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_div_rem_integer_ref_fail() {
    10.div_rem(&Integer::ZERO);
}

#[test]
fn test_u32_ceiling_div_neg_mod_integer() {
    let test = |u: u32, v, quotient, remainder| {
        let (q, r) = u.ceiling_div_neg_mod(Integer::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = u.ceiling_div_neg_mod(&Integer::from_str(v).unwrap());
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
    test(123, "456", "1", "333");
    test(456, "123", "4", "36");
    test(4_294_967_295, "1", "4294967295", "0");
    test(4_294_967_295, "4294967295", "1", "0");
    test(0, "1000000000000", "0", "0");
    test(123, "1000000000000", "1", "999999999877");

    test(1, "-1", "-1", "0");
    test(123, "-1", "-123", "0");
    test(123, "-123", "-1", "0");
    test(123, "-456", "-1", "333");
    test(456, "-123", "-4", "36");
    test(4_294_967_295, "-1", "-4294967295", "0");
    test(4_294_967_295, "-4294967295", "-1", "0");
    test(0, "-1000000000000", "0", "0");
    test(123, "-1000000000000", "-1", "999999999877");
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_ceiling_div_neg_mod_integer_fail() {
    10.ceiling_div_neg_mod(Integer::ZERO);
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_ceiling_div_neg_mod_integer_ref_fail() {
    10.ceiling_div_neg_mod(&Integer::ZERO);
}

#[test]
fn test_u32_ceiling_div_mod_integer() {
    let test = |u: u32, v, quotient, remainder| {
        let (q, r) = u.ceiling_div_mod(Integer::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = u.ceiling_div_mod(&Integer::from_str(v).unwrap());
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
    test(4_294_967_295, "1", "4294967295", "0");
    test(4_294_967_295, "4294967295", "1", "0");
    test(0, "1000000000000", "0", "0");
    test(123, "1000000000000", "1", "-999999999877");

    test(1, "-1", "-1", "0");
    test(123, "-1", "-123", "0");
    test(123, "-123", "-1", "0");
    test(123, "-456", "-1", "-333");
    test(456, "-123", "-4", "-36");
    test(4_294_967_295, "-1", "-4294967295", "0");
    test(4_294_967_295, "-4294967295", "-1", "0");
    test(0, "-1000000000000", "0", "0");
    test(123, "-1000000000000", "-1", "-999999999877");
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_ceiling_div_mod_integer_fail() {
    10.ceiling_div_mod(Integer::ZERO);
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_ceiling_div_mod_integer_ref_fail() {
    10.ceiling_div_mod(&Integer::ZERO);
}

fn div_mod_u32_properties_helper(n: &Integer, u: u32) {
    let mut mut_n = n.clone();
    let remainder = mut_n.div_assign_mod(u);
    assert!(mut_n.is_valid());
    let quotient = mut_n;

    let (quotient_alt, remainder_alt) = n.div_mod(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = n.clone().div_mod(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    //TODO let (quotient_alt, remainder_alt) = (n.div_round(u, RoundingMode::Floor), n.mod_op(u));
    // assert_eq!(quotient_alt, quotient);
    // assert_eq!(remainder_alt, remainder);

    //TODO assert_eq!(n.div_mod(Integer::from(u)), (quotient.clone(), remainder));

    let (num_quotient, num_remainder) = num_div_mod_u32(integer_to_bigint(n), u);
    assert_eq!(bigint_to_integer(&num_quotient), quotient);
    assert_eq!(num_remainder, remainder);

    let (rug_quotient, rug_remainder) = rug_div_mod_u32(integer_to_rug_integer(n), u);
    assert_eq!(rug_integer_to_integer(&rug_quotient), quotient);
    assert_eq!(rug_remainder, remainder);

    assert!(remainder < u);
    assert_eq!(quotient * u + remainder, *n);

    let (quotient_neg, remainder_neg) = (-n).div_mod(u);
    let (mut quotient_neg_alt, remainder_neg_alt) = n.ceiling_div_neg_mod(u);
    quotient_neg_alt.neg_assign();
    assert_eq!(quotient_neg_alt, quotient_neg);
    assert_eq!(remainder_neg_alt, remainder_neg);
}

#[test]
fn div_mod_u32_properties() {
    test_properties(
        pairs_of_integer_and_positive_unsigned,
        |&(ref n, u): &(Integer, u32)| {
            div_mod_u32_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_integer_and_positive_u32_var_1,
        |&(ref n, u): &(Integer, u32)| {
            div_mod_u32_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_unsigned_and_nonzero_integer,
        |&(u, ref n): &(u32, Integer)| {
            let (quotient, remainder) = u.div_mod(n);
            assert!(quotient.is_valid());

            let (quotient_alt, remainder_alt) = u.div_mod(n.clone());
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);
            assert_eq!(remainder_alt, remainder);

            //TODO assert_eq!((quotient.clone(), remainder), Natural::from(u).div_mod(n));

            if u != 0 && u < *n {
                assert_eq!(remainder, u);
            }
            assert!(remainder.lt_abs(n));
            assert_eq!(&quotient * n + remainder, u);

            let (mut quotient_neg, remainder_neg) = u.div_mod(-n);
            quotient_neg.neg_assign();
            assert_eq!(quotient_neg, quotient);
            assert_eq!(remainder_neg, remainder);
        },
    );

    test_properties(integers, |n| {
        let (q, r) = n.div_mod(1);
        assert_eq!(q, *n);
        assert_eq!(r, 0);
    });

    test_properties(positive_unsigneds, |&u: &u32| {
        assert_eq!(u.div_mod(Integer::ONE), (Integer::from(u), 0));
        assert_eq!(u.div_mod(Integer::NEGATIVE_ONE), (-Natural::from(u), 0));
        assert_eq!(u.div_mod(Integer::from(u)), (Integer::ONE, 0));
        assert_eq!(u.div_mod(-Natural::from(u)), (Integer::NEGATIVE_ONE, 0));
        assert_eq!(Integer::ZERO.div_mod(u), (Integer::ZERO, 0));
        if u > 1 {
            assert_eq!(Integer::ONE.div_mod(u), (Integer::ZERO, 1));
            assert_eq!(
                Integer::NEGATIVE_ONE.div_mod(u),
                (Integer::NEGATIVE_ONE, u - 1)
            );
        }
    });
}

fn div_rem_u32_properties_helper(n: &Integer, u: u32) {
    let mut mut_n = n.clone();
    let remainder = mut_n.div_assign_rem(u);
    assert!(mut_n.is_valid());
    let quotient = mut_n;

    let (quotient_alt, remainder_alt) = n.div_rem(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = n.clone().div_rem(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = (n / u, n % u);
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    //TODO assert_eq!(n.div_rem(Integer::from(u)), (quotient.clone(), remainder));

    let (num_quotient, num_remainder) = num_div_rem_u32(integer_to_bigint(n), u);
    assert_eq!(bigint_to_integer(&num_quotient), quotient);
    assert_eq!(bigint_to_integer(&num_remainder), remainder);

    let (rug_quotient, rug_remainder) = rug_div_rem_u32(integer_to_rug_integer(n), u);
    assert_eq!(rug_integer_to_integer(&rug_quotient), quotient);
    assert_eq!(rug_integer_to_integer(&rug_remainder), remainder);

    assert!(remainder.lt_abs(&u));
    assert!(remainder == 0 || (remainder > 0) == (*n > 0));
    assert_eq!(quotient * u + remainder, *n);

    let (quotient_neg, remainder_neg) = (-n).div_rem(u);
    let (mut quotient_neg_alt, mut remainder_neg_alt) = n.div_rem(u);
    quotient_neg_alt.neg_assign();
    remainder_neg_alt.neg_assign();
    assert_eq!(quotient_neg_alt, quotient_neg);
    assert_eq!(remainder_neg_alt, remainder_neg);
}

#[test]
fn div_rem_u32_properties() {
    test_properties(
        pairs_of_integer_and_positive_unsigned,
        |&(ref n, u): &(Integer, u32)| {
            div_rem_u32_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_integer_and_positive_u32_var_1,
        |&(ref n, u): &(Integer, u32)| {
            div_rem_u32_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_unsigned_and_nonzero_integer,
        |&(u, ref n): &(u32, Integer)| {
            let (quotient, remainder) = u.div_rem(n);
            assert!(quotient.is_valid());

            let (quotient_alt, remainder_alt) = u.div_rem(n.clone());
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);
            assert_eq!(remainder_alt, remainder);

            //TODO assert_eq!((quotient.clone(), remainder), Natural::from(u).div_rem(n));

            if u > 0 && u.lt_abs(n) {
                assert_eq!(remainder, u);
            }
            assert!(remainder.lt_abs(n));
            assert_eq!(&quotient * n + remainder, u);

            let (mut quotient_neg, remainder_neg) = u.div_rem(-n);
            quotient_neg.neg_assign();
            assert_eq!(quotient_neg, quotient);
            assert_eq!(remainder_neg, remainder);
        },
    );

    test_properties(integers, |n| {
        let (q, r) = n.div_rem(1);
        assert_eq!(q, *n);
        assert_eq!(r, 0);
    });

    test_properties(positive_unsigneds, |&u: &u32| {
        assert_eq!(u.div_rem(Integer::ONE), (Integer::from(u), 0));
        assert_eq!(u.div_rem(Integer::NEGATIVE_ONE), (-Natural::from(u), 0));
        assert_eq!(u.div_rem(Integer::from(u)), (Integer::ONE, 0));
        assert_eq!(u.div_rem(-Natural::from(u)), (Integer::NEGATIVE_ONE, 0));
        assert_eq!(Integer::ZERO.div_rem(u), (Integer::ZERO, Integer::ZERO));
        if u > 1 {
            assert_eq!(Integer::ONE.div_rem(u), (Integer::ZERO, Integer::ONE));
            assert_eq!(
                Integer::NEGATIVE_ONE.div_rem(u),
                (Integer::ZERO, Integer::NEGATIVE_ONE)
            );
        }
    });
}

fn ceiling_div_neg_mod_u32_properties_helper(n: &Integer, u: u32) {
    let mut mut_n = n.clone();
    let remainder = mut_n.ceiling_div_assign_neg_mod(u);
    assert!(mut_n.is_valid());
    let quotient = mut_n;

    let (quotient_alt, remainder_alt) = n.ceiling_div_neg_mod(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = n.clone().ceiling_div_mod(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(-remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = n.clone().ceiling_div_mod(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(-remainder_alt, remainder);

    //TODO let (quotient_alt, remainder_alt) = (n.div_round(u, RoundingMode::Ceiling), n.neg_mod(u));
    //assert_eq!(quotient_alt, quotient);
    //assert_eq!(remainder_alt, remainder);

    //TODO assert_eq!(n.ceiling_div_neg_mod(Integer::from(u)), (quotient.clone(), remainder));

    let (rug_quotient, rug_remainder) = rug_ceiling_div_neg_mod_u32(integer_to_rug_integer(n), u);
    assert_eq!(rug_integer_to_integer(&rug_quotient), quotient);
    assert_eq!(rug_remainder, remainder);

    assert!(remainder < u);
    assert_eq!(quotient * u - remainder, *n);

    let (quotient_neg, remainder_neg) = (-n).ceiling_div_neg_mod(u);
    let (mut quotient_neg_alt, remainder_neg_alt) = n.div_mod(u);
    quotient_neg_alt.neg_assign();
    assert_eq!(quotient_neg_alt, quotient_neg);
    assert_eq!(remainder_neg_alt, remainder_neg);
}

#[test]
fn ceiling_div_neg_mod_u32_properties() {
    test_properties(
        pairs_of_integer_and_positive_unsigned,
        |&(ref n, u): &(Integer, u32)| {
            ceiling_div_neg_mod_u32_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_integer_and_positive_u32_var_1,
        |&(ref n, u): &(Integer, u32)| {
            ceiling_div_neg_mod_u32_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_unsigned_and_nonzero_integer,
        |&(u, ref n): &(u32, Integer)| {
            let (quotient, remainder) = u.ceiling_div_neg_mod(n);
            assert!(quotient.is_valid());
            assert!(remainder.is_valid());

            let (quotient_alt, remainder_alt) = u.ceiling_div_neg_mod(n.clone());
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);
            assert!(remainder_alt.is_valid());
            assert_eq!(remainder_alt, remainder);

            //TODO assert_eq!((quotient, remainder), Natural::from(u).ceiling_div_neg_mod(n));

            if u != 0 && u.lt_abs(n) {
                assert_eq!(remainder, n.abs() - u);
            }
            assert!(remainder.lt_abs(n));
            assert_eq!(&quotient * n - Integer::from(&remainder), u);

            let (mut quotient_neg, remainder_neg) = u.ceiling_div_neg_mod(-n);
            quotient_neg.neg_assign();
            assert_eq!(quotient_neg, quotient);
            assert_eq!(remainder_neg, remainder);
        },
    );

    test_properties(integers, |n| {
        let (q, r) = n.ceiling_div_neg_mod(1);
        assert_eq!(q, *n);
        assert_eq!(r, 0);
    });

    test_properties(positive_unsigneds, |&u: &u32| {
        assert_eq!(
            u.ceiling_div_neg_mod(Integer::ONE),
            (Integer::from(u), Natural::ZERO)
        );
        assert_eq!(
            u.ceiling_div_neg_mod(Integer::NEGATIVE_ONE),
            (-Natural::from(u), Natural::ZERO)
        );
        assert_eq!(
            u.ceiling_div_neg_mod(Integer::from(u)),
            (Integer::ONE, Natural::ZERO)
        );
        assert_eq!(
            u.ceiling_div_neg_mod(-Natural::from(u)),
            (Integer::NEGATIVE_ONE, Natural::ZERO)
        );
        assert_eq!(Integer::ZERO.ceiling_div_neg_mod(u), (Integer::ZERO, 0));
        assert_eq!(Integer::ONE.ceiling_div_neg_mod(u), (Integer::ONE, u - 1));
    });
}

fn ceiling_div_mod_u32_properties_helper(n: &Integer, u: u32) {
    let mut mut_n = n.clone();
    let remainder = mut_n.ceiling_div_assign_mod(u);
    assert!(mut_n.is_valid());
    let quotient = mut_n;

    let (quotient_alt, remainder_alt) = n.ceiling_div_mod(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = n.clone().ceiling_div_mod(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = n.clone().ceiling_div_neg_mod(u);
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, -&remainder);

    //TODO let (quotient_alt, remainder_alt) = (n.div_round(u, RoundingMode::Ceiling), n.neg_mod(u));
    //assert_eq!(quotient_alt, quotient);
    //assert_eq!(remainder_alt, remainder);

    //TODO assert_eq!(n.ceiling_div_mod(Integer::from(u)), (quotient.clone(), remainder));

    let (rug_quotient, rug_remainder) = rug_ceiling_div_mod_u32(integer_to_rug_integer(n), u);
    assert_eq!(rug_integer_to_integer(&rug_quotient), quotient);
    assert_eq!(rug_integer_to_integer(&rug_remainder), remainder);

    assert!(remainder <= 0);
    assert!(-&remainder < u);
    assert_eq!(quotient * u + remainder, *n);

    let (quotient_neg, remainder_neg) = (-n).ceiling_div_mod(u);
    let (mut quotient_neg_alt, remainder_neg_alt) = n.div_mod(u);
    quotient_neg_alt.neg_assign();
    let remainder_neg_alt = -Natural::from(remainder_neg_alt);
    assert_eq!(quotient_neg_alt, quotient_neg);
    assert_eq!(remainder_neg_alt, remainder_neg);
}

#[test]
fn ceiling_div_mod_u32_properties() {
    test_properties(
        pairs_of_integer_and_positive_unsigned,
        |&(ref n, u): &(Integer, u32)| {
            ceiling_div_mod_u32_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_integer_and_positive_u32_var_1,
        |&(ref n, u): &(Integer, u32)| {
            ceiling_div_mod_u32_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_unsigned_and_nonzero_integer,
        |&(u, ref n): &(u32, Integer)| {
            let (quotient, remainder) = u.ceiling_div_mod(n);
            assert!(quotient.is_valid());
            assert!(remainder.is_valid());

            let (quotient_alt, remainder_alt) = u.ceiling_div_mod(n.clone());
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);
            assert!(remainder_alt.is_valid());
            assert_eq!(remainder_alt, remainder);

            //TODO assert_eq!((quotient, remainder), Natural::from(u).ceiling_div_mod(n));

            if u != 0 && u.lt_abs(n) {
                assert_eq!(remainder, u - n.abs());
            }
            assert!(remainder <= 0);
            assert!((-&remainder).lt_abs(n));
            assert_eq!(&quotient * n + &remainder, u);

            let (mut quotient_neg, remainder_neg) = u.ceiling_div_mod(-n);
            quotient_neg.neg_assign();
            assert_eq!(quotient_neg, quotient);
            assert_eq!(remainder_neg, remainder);
        },
    );

    test_properties(integers, |n| {
        let (q, r) = n.ceiling_div_mod(1);
        assert_eq!(q, *n);
        assert_eq!(r, 0);
    });

    test_properties(positive_unsigneds, |&u: &u32| {
        assert_eq!(
            u.ceiling_div_mod(Integer::ONE),
            (Integer::from(u), Integer::ZERO)
        );
        assert_eq!(
            u.ceiling_div_mod(Integer::NEGATIVE_ONE),
            (-Natural::from(u), Integer::ZERO)
        );
        assert_eq!(
            u.ceiling_div_mod(Integer::from(u)),
            (Integer::ONE, Integer::ZERO)
        );
        assert_eq!(
            u.ceiling_div_mod(-Natural::from(u)),
            (Integer::NEGATIVE_ONE, Integer::ZERO)
        );
        assert_eq!(
            Integer::ZERO.ceiling_div_mod(u),
            (Integer::ZERO, Integer::ZERO)
        );
        assert_eq!(
            Integer::ONE.ceiling_div_mod(u),
            (Integer::ONE, -Natural::from(u - 1))
        );
    });
}
