use common::test_properties;
use malachite_base::num::{
    CeilingDivMod, CeilingMod, CeilingModAssign, DivMod, DivRem, Mod, ModAssign, NegativeOne, One,
    PartialOrdAbs, UnsignedAbs, Zero,
};
use malachite_nz::integer::Integer;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::nonzero_signeds;
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_nonzero_i32_var_1, pairs_of_integer_and_nonzero_signed,
    pairs_of_signed_and_nonzero_integer, triples_of_integer_integer_and_nonzero_signed,
};
use malachite_test::integer::arithmetic::mod_i32::num_mod_i32;
use num::BigInt;
use rug;
use rug::ops::RemRounding;
use std::str::FromStr;

#[test]
fn test_mod_i32() {
    let test = |u, v: i32, remainder| {
        let mut n = Integer::from_str(u).unwrap();
        n.mod_assign(v);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), remainder);

        let remainder_alt = Integer::from_str(u).unwrap().mod_op(v);
        assert!(remainder_alt.is_valid());
        assert_eq!(remainder_alt.to_string(), remainder);

        let remainder_alt = (&Integer::from_str(u).unwrap()).mod_op(v);
        assert!(remainder_alt.is_valid());
        assert_eq!(remainder_alt.to_string(), remainder);

        assert_eq!(
            num_mod_i32(BigInt::from_str(u).unwrap(), v).to_string(),
            remainder
        );
        assert_eq!(
            rug::Integer::from_str(u).unwrap().rem_floor(v).to_string(),
            remainder
        );
    };
    test("0", 1, "0");
    test("0", 123, "0");
    test("1", 1, "0");
    test("123", 1, "0");
    test("123", 123, "0");
    test("123", 456, "123");
    test("456", 123, "87");
    test("2147483647", 1, "0");
    test("2147483647", 2_147_483_647, "0");
    test("1000000000000", 1, "0");
    test("1000000000000", 3, "1");
    test("1000000000000", 123, "100");
    test("1000000000000", 2_147_483_647, "1420104145");
    test("1000000000000000000000000", 1, "0");
    test("1000000000000000000000000", 3, "1");
    test("1000000000000000000000000", 123, "37");
    test("1000000000000000000000000", 2_147_483_647, "1486940387");

    test("-1", 1, "0");
    test("-123", 1, "0");
    test("-123", 123, "0");
    test("-123", 456, "333");
    test("-456", 123, "36");
    test("-2147483647", 1, "0");
    test("-2147483647", 2_147_483_647, "0");
    test("-1000000000000", 1, "0");
    test("-1000000000000", 3, "2");
    test("-1000000000000", 123, "23");
    test("-1000000000000", 2_147_483_647, "727379502");
    test("-1000000000000000000000000", 1, "0");
    test("-1000000000000000000000000", 3, "2");
    test("-1000000000000000000000000", 123, "86");
    test("-1000000000000000000000000", 2_147_483_647, "660543260");

    test("0", -1, "0");
    test("0", -123, "0");
    test("1", -1, "0");
    test("123", -1, "0");
    test("123", -123, "0");
    test("123", -456, "-333");
    test("456", -123, "-36");
    test("2147483647", -1, "0");
    test("2147483647", -2_147_483_647, "0");
    test("2147483648", -2_147_483_648, "0");
    test("1000000000000", -1, "0");
    test("1000000000000", -3, "-2");
    test("1000000000000", -123, "-23");
    test("1000000000000", -2_147_483_647, "-727379502");
    test("1000000000000", -2_147_483_648, "-727379968");
    test("1000000000000000000000000", -1, "0");
    test("1000000000000000000000000", -3, "-2");
    test("1000000000000000000000000", -123, "-86");
    test("1000000000000000000000000", -2_147_483_647, "-660543260");
    test("1000000000000000000000000", -2_147_483_648, "-1593835520");

    test("-1", -1, "0");
    test("-123", -1, "0");
    test("-123", -123, "0");
    test("-123", -456, "-123");
    test("-456", -123, "-87");
    test("-2147483647", -1, "0");
    test("-2147483647", -2_147_483_647, "0");
    test("-2147483648", -2_147_483_648, "0");
    test("-1000000000000", -1, "0");
    test("-1000000000000", -3, "-1");
    test("-1000000000000", -123, "-100");
    test("-1000000000000", -2_147_483_647, "-1420104145");
    test("-1000000000000", -2_147_483_648, "-1420103680");
    test("-1000000000000000000000000", -1, "0");
    test("-1000000000000000000000000", -3, "-1");
    test("-1000000000000000000000000", -123, "-37");
    test("-1000000000000000000000000", -2_147_483_647, "-1486940387");
    test("-1000000000000000000000000", -2_147_483_648, "-553648128");
}

#[test]
#[should_panic(expected = "division by zero")]
fn mod_assign_i32_fail() {
    Integer::from(10i32).mod_assign(0i32);
}

#[test]
#[should_panic(expected = "division by zero")]
fn mod_i32_fail() {
    Integer::from(10i32).mod_op(0i32);
}

#[test]
#[should_panic(expected = "division by zero")]
fn mod_i32_ref_fail() {
    (&Integer::from(10i32)).mod_op(0i32);
}

#[test]
fn test_rem_i32() {
    let test = |u, v: i32, remainder| {
        let mut n = Integer::from_str(u).unwrap();
        n %= v;
        assert!(n.is_valid());
        assert_eq!(n.to_string(), remainder);

        let r = Integer::from_str(u).unwrap() % v;
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
        let r = &Integer::from_str(u).unwrap() % v;
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        assert_eq!((BigInt::from_str(u).unwrap() % v).to_string(), remainder);
        assert_eq!(
            (rug::Integer::from_str(u).unwrap() % v).to_string(),
            remainder
        );
    };
    test("0", 1, "0");
    test("0", 123, "0");
    test("1", 1, "0");
    test("123", 1, "0");
    test("123", 123, "0");
    test("123", 456, "123");
    test("456", 123, "87");
    test("2147483647", 1, "0");
    test("2147483647", 2_147_483_647, "0");
    test("1000000000000", 1, "0");
    test("1000000000000", 3, "1");
    test("1000000000000", 123, "100");
    test("1000000000000", 2_147_483_647, "1420104145");
    test("1000000000000000000000000", 1, "0");
    test("1000000000000000000000000", 3, "1");
    test("1000000000000000000000000", 123, "37");
    test("1000000000000000000000000", 2_147_483_647, "1486940387");

    test("-1", 1, "0");
    test("-123", 1, "0");
    test("-123", 123, "0");
    test("-123", 456, "-123");
    test("-456", 123, "-87");
    test("-2147483647", 1, "0");
    test("-2147483647", 2_147_483_647, "0");
    test("-1000000000000", 1, "0");
    test("-1000000000000", 3, "-1");
    test("-1000000000000", 123, "-100");
    test("-1000000000000", 2_147_483_647, "-1420104145");
    test("-1000000000000000000000000", 1, "0");
    test("-1000000000000000000000000", 3, "-1");
    test("-1000000000000000000000000", 123, "-37");
    test("-1000000000000000000000000", 2_147_483_647, "-1486940387");

    test("0", -1, "0");
    test("0", -123, "0");
    test("1", -1, "0");
    test("123", -1, "0");
    test("123", -123, "0");
    test("123", -456, "123");
    test("456", -123, "87");
    test("2147483647", -1, "0");
    test("2147483647", -2_147_483_647, "0");
    test("2147483648", -2_147_483_648, "0");
    test("1000000000000", -1, "0");
    test("1000000000000", -3, "1");
    test("1000000000000", -123, "100");
    test("1000000000000", -2_147_483_647, "1420104145");
    test("1000000000000", -2_147_483_648, "1420103680");
    test("1000000000000000000000000", -1, "0");
    test("1000000000000000000000000", -3, "1");
    test("1000000000000000000000000", -123, "37");
    test("1000000000000000000000000", -2_147_483_647, "1486940387");
    test("1000000000000000000000000", -2_147_483_648, "553648128");

    test("-1", -1, "0");
    test("-123", -1, "0");
    test("-123", -123, "0");
    test("-123", -456, "-123");
    test("-456", -123, "-87");
    test("-2147483647", -1, "0");
    test("-2147483647", -2_147_483_647, "0");
    test("-1000000000000", -1, "0");
    test("-1000000000000", -3, "-1");
    test("-1000000000000", -123, "-100");
    test("-1000000000000", -2_147_483_647, "-1420104145");
    test("-1000000000000", -2_147_483_648, "-1420103680");
    test("-1000000000000000000000000", -1, "0");
    test("-1000000000000000000000000", -3, "-1");
    test("-1000000000000000000000000", -123, "-37");
    test("-1000000000000000000000000", -2_147_483_647, "-1486940387");
    test("-1000000000000000000000000", -2_147_483_648, "-553648128");
}

#[test]
#[should_panic(expected = "division by zero")]
fn rem_assign_i32_fail() {
    let mut n = Integer::from(10i32);
    n %= 0i32;
}

#[test]
#[allow(unused_must_use)]
#[should_panic(expected = "division by zero")]
fn rem_i32_fail() {
    Integer::from(10i32) % 0i32;
}

#[test]
#[allow(unused_must_use)]
#[should_panic(expected = "division by zero")]
fn rem_i32_ref_fail() {
    &Integer::from(10i32) % 0i32;
}

#[test]
fn test_ceiling_mod_i32() {
    let test = |u, v: i32, remainder| {
        let mut n = Integer::from_str(u).unwrap();
        n.ceiling_mod_assign(v);
        assert_eq!(n.to_string(), remainder);

        let r = Integer::from_str(u).unwrap().ceiling_mod(v);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
        let r = (&Integer::from_str(u).unwrap()).ceiling_mod(v);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        assert_eq!(
            rug::Integer::from_str(u).unwrap().rem_ceil(v).to_string(),
            remainder
        );
    };
    test("0", 1, "0");
    test("0", 123, "0");
    test("1", 1, "0");
    test("123", 1, "0");
    test("123", 123, "0");
    test("123", 456, "-333");
    test("456", 123, "-36");
    test("2147483647", 1, "0");
    test("2147483647", 2_147_483_647, "0");
    test("1000000000000", 1, "0");
    test("1000000000000", 3, "-2");
    test("1000000000000", 123, "-23");
    test("1000000000000", 2_147_483_647, "-727379502");
    test("1000000000000000000000000", 1, "0");
    test("1000000000000000000000000", 3, "-2");
    test("1000000000000000000000000", 123, "-86");
    test("1000000000000000000000000", 2_147_483_647, "-660543260");

    test("-1", 1, "0");
    test("-123", 1, "0");
    test("-123", 123, "0");
    test("-123", 456, "-123");
    test("-456", 123, "-87");
    test("-2147483647", 1, "0");
    test("-2147483647", 2_147_483_647, "0");
    test("-1000000000000", 1, "0");
    test("-1000000000000", 3, "-1");
    test("-1000000000000", 123, "-100");
    test("-1000000000000", 2_147_483_647, "-1420104145");
    test("-1000000000000000000000000", 1, "0");
    test("-1000000000000000000000000", 3, "-1");
    test("-1000000000000000000000000", 123, "-37");
    test("-1000000000000000000000000", 2_147_483_647, "-1486940387");

    test("0", -1, "0");
    test("0", -123, "0");
    test("1", -1, "0");
    test("123", -1, "0");
    test("123", -123, "0");
    test("123", -456, "123");
    test("456", -123, "87");
    test("2147483647", -1, "0");
    test("2147483647", -2_147_483_647, "0");
    test("2147483648", -2_147_483_648, "0");
    test("1000000000000", -1, "0");
    test("1000000000000", -3, "1");
    test("1000000000000", -123, "100");
    test("1000000000000", -2_147_483_647, "1420104145");
    test("1000000000000", -2_147_483_648, "1420103680");
    test("1000000000000000000000000", -1, "0");
    test("1000000000000000000000000", -3, "1");
    test("1000000000000000000000000", -123, "37");
    test("1000000000000000000000000", -2_147_483_647, "1486940387");
    test("1000000000000000000000000", -2_147_483_648, "553648128");

    test("-1", -1, "0");
    test("-123", -1, "0");
    test("-123", -123, "0");
    test("-123", -456, "333");
    test("-456", -123, "36");
    test("-2147483647", -1, "0");
    test("-2147483647", -2_147_483_647, "0");
    test("-1000000000000", -1, "0");
    test("-1000000000000", -3, "2");
    test("-1000000000000", -123, "23");
    test("-1000000000000", -2_147_483_647, "727379502");
    test("-1000000000000", -2_147_483_648, "727379968");
    test("-1000000000000000000000000", -1, "0");
    test("-1000000000000000000000000", -3, "2");
    test("-1000000000000000000000000", -123, "86");
    test("-1000000000000000000000000", -2_147_483_647, "660543260");
    test("-1000000000000000000000000", -2_147_483_648, "1593835520");
}

#[test]
#[should_panic(expected = "division by zero")]
fn ceiling_mod_assign_i32_fail() {
    Integer::from(10i32).ceiling_mod_assign(0i32);
}

#[test]
#[should_panic(expected = "division by zero")]
fn ceiling_mod_i32_fail() {
    Integer::from(10i32).ceiling_mod(0i32);
}

#[test]
#[should_panic(expected = "division by zero")]
fn ceiling_mod_i32_ref_fail() {
    (&Integer::from(10i32)).ceiling_mod(0i32);
}

#[test]
fn test_i32_mod_integer() {
    let test = |i: i32, v, remainder| {
        let r = i.mod_op(Integer::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = i.mod_op(&Integer::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
    };
    test(0, "1", "0");
    test(0, "123", "0");
    test(1, "1", "0");
    test(123, "1", "0");
    test(123, "123", "0");
    test(123, "456", "123");
    test(456, "123", "87");
    test(2_147_483_647, "1", "0");
    test(2_147_483_647, "2147483647", "0");
    test(0, "1000000000000", "0");
    test(123, "1000000000000", "123");

    test(1, "-1", "0");
    test(123, "-1", "0");
    test(123, "-123", "0");
    test(123, "-456", "-333");
    test(456, "-123", "-36");
    test(2_147_483_647, "-1", "0");
    test(2_147_483_647, "-2147483647", "0");
    test(0, "-1000000000000", "0");
    test(123, "-1000000000000", "-999999999877");

    test(-1, "1", "0");
    test(-123, "1", "0");
    test(-123, "123", "0");
    test(-123, "456", "333");
    test(-456, "123", "36");
    test(-2_147_483_647, "1", "0");
    test(-2_147_483_647, "2147483647", "0");
    test(-2_147_483_648, "2147483648", "0");
    test(-123, "1000000000000", "999999999877");

    test(-1, "-1", "0");
    test(-123, "-1", "0");
    test(-123, "-123", "0");
    test(-123, "-456", "-123");
    test(-456, "-123", "-87");
    test(-2_147_483_647, "-1", "0");
    test(-2_147_483_648, "-1", "0");
    test(-2_147_483_647, "-2147483647", "0");
    test(-2_147_483_648, "-2147483648", "0");
    test(-123, "-1000000000000", "-123");
}

#[test]
#[should_panic(expected = "division by zero")]
fn i32_mod_integer_fail() {
    10i32.mod_op(Integer::ZERO);
}

#[test]
#[should_panic(expected = "division by zero")]
fn i32_mod_integer_ref_fail() {
    10i32.mod_op(&Integer::ZERO);
}

#[test]
fn test_i32_rem_integer() {
    let test = |i: i32, v, remainder| {
        let r = i % Integer::from_str(v).unwrap();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = i % &Integer::from_str(v).unwrap();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
    };
    test(0, "1", "0");
    test(0, "123", "0");
    test(1, "1", "0");
    test(123, "1", "0");
    test(123, "123", "0");
    test(123, "456", "123");
    test(456, "123", "87");
    test(2_147_483_647, "1", "0");
    test(2_147_483_647, "2147483647", "0");
    test(0, "1000000000000", "0");
    test(123, "1000000000000", "123");

    test(1, "-1", "0");
    test(123, "-1", "0");
    test(123, "-123", "0");
    test(123, "-456", "123");
    test(456, "-123", "87");
    test(2_147_483_647, "-1", "0");
    test(2_147_483_647, "-2147483647", "0");
    test(0, "-1000000000000", "0");
    test(123, "-1000000000000", "123");

    test(-1, "1", "0");
    test(-123, "1", "0");
    test(-123, "123", "0");
    test(-123, "456", "-123");
    test(-456, "123", "-87");
    test(-2_147_483_647, "1", "0");
    test(-2_147_483_647, "2147483647", "0");
    test(-2_147_483_648, "2147483648", "0");
    test(-123, "1000000000000", "-123");

    test(-1, "-1", "0");
    test(-123, "-1", "0");
    test(-123, "-123", "0");
    test(-123, "-456", "-123");
    test(-456, "-123", "-87");
    test(-2_147_483_647, "-1", "0");
    test(-2_147_483_648, "-1", "0");
    test(-2_147_483_647, "-2147483647", "0");
    test(-2_147_483_648, "-2147483648", "0");
    test(-123, "-1000000000000", "-123");
}

#[test]
#[allow(unused_must_use)]
#[should_panic(expected = "division by zero")]
fn i32_rem_integer_fail() {
    10i32 % Integer::ZERO;
}

#[test]
#[allow(unused_must_use)]
#[should_panic(expected = "division by zero")]
fn i32_rem_integer_ref_fail() {
    10i32 % &Integer::ZERO;
}

#[test]
fn test_i32_ceiling_mod_integer() {
    let test = |i: i32, v, remainder| {
        let n = i.ceiling_mod(Integer::from_str(v).unwrap());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), remainder);

        let n = i.ceiling_mod(&Integer::from_str(v).unwrap());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), remainder);
    };
    test(0, "1", "0");
    test(0, "123", "0");
    test(1, "1", "0");
    test(123, "1", "0");
    test(123, "123", "0");
    test(123, "456", "-333");
    test(456, "123", "-36");
    test(2_147_483_647, "1", "0");
    test(2_147_483_647, "2147483647", "0");
    test(0, "1000000000000", "0");
    test(123, "1000000000000", "-999999999877");

    test(1, "-1", "0");
    test(123, "-1", "0");
    test(123, "-123", "0");
    test(123, "-456", "123");
    test(456, "-123", "87");
    test(2_147_483_647, "-1", "0");
    test(2_147_483_647, "-2147483647", "0");
    test(0, "-1000000000000", "0");
    test(123, "-1000000000000", "123");

    test(-1, "1", "0");
    test(-123, "1", "0");
    test(-123, "123", "0");
    test(-123, "456", "-123");
    test(-456, "123", "-87");
    test(-2_147_483_647, "1", "0");
    test(-2_147_483_648, "1", "0");
    test(-2_147_483_647, "2147483647", "0");
    test(-2_147_483_648, "2147483648", "0");
    test(-123, "1000000000000", "-123");

    test(-1, "-1", "0");
    test(-123, "-1", "0");
    test(-123, "-123", "0");
    test(-123, "-456", "333");
    test(-456, "-123", "36");
    test(-2_147_483_647, "-1", "0");
    test(-2_147_483_648, "-1", "0");
    test(-2_147_483_647, "-2147483647", "0");
    test(-2_147_483_648, "-2147483648", "0");
    test(-123, "-1000000000000", "999999999877");
}

#[test]
#[should_panic(expected = "division by zero")]
fn i32_ceiling_mod_integer_fail() {
    10i32.ceiling_mod(Integer::ZERO);
}

#[test]
#[should_panic(expected = "division by zero")]
fn i32_ceiling_mod_integer_ref_fail() {
    10i32.ceiling_mod(&Integer::ZERO);
}

fn mod_i32_properties_helper(n: &Integer, i: i32) {
    let mut mut_n = n.clone();
    mut_n.mod_assign(i);
    assert!(mut_n.is_valid());
    let remainder = mut_n;

    let remainder_alt = n.mod_op(i);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = n.clone().mod_op(i);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    assert_eq!(n.div_mod(i).1, remainder);

    //TODO assert_eq!(n.mod_op(Integer::from(u)), remainder);

    assert_eq!(
        bigint_to_integer(&num_mod_i32(integer_to_bigint(n), i)),
        remainder
    );
    assert_eq!(
        rug_integer_to_integer(&integer_to_rug_integer(n).rem_floor(i)),
        remainder
    );

    assert!(remainder < i.unsigned_abs());
    assert!(remainder == 0 || (remainder > 0) == (i > 0));

    assert_eq!((-n).mod_op(i), -n.ceiling_mod(i));
}

#[test]
fn mod_i32_properties() {
    test_properties(
        pairs_of_integer_and_nonzero_signed,
        |&(ref n, i): &(Integer, i32)| {
            mod_i32_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_integer_and_nonzero_i32_var_1,
        |&(ref n, i): &(Integer, i32)| {
            mod_i32_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_signed_and_nonzero_integer,
        |&(i, ref n): &(i32, Integer)| {
            let remainder = i.mod_op(n);
            assert_eq!(i.mod_op(n.clone()), remainder);
            assert_eq!(i.div_mod(n).1, remainder);

            if i > 0 && i < *n {
                assert_eq!(remainder, i.unsigned_abs());
            }
            assert!(remainder.lt_abs(n));
            assert!(remainder == 0 || (remainder > 0) == (*n > 0));
            assert_eq!(i.mod_op(-n), i.ceiling_mod(n));
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n.mod_op(1i32), 0);
        assert_eq!(n.mod_op(-1i32), 0);
    });

    test_properties(nonzero_signeds, |&i: &i32| {
        assert_eq!(i.mod_op(Integer::ONE), 0);
        assert_eq!(i.mod_op(Integer::NEGATIVE_ONE), 0);
        assert_eq!(i.mod_op(Integer::from(i)), 0);
        assert_eq!(Integer::from(i).mod_op(i), 0);
        assert_eq!(i.mod_op(-Integer::from(i)), 0);
        assert_eq!((-Integer::from(i)).mod_op(i), 0);
        assert_eq!(Integer::ZERO.mod_op(i), 0);
    });

    test_properties(
        triples_of_integer_integer_and_nonzero_signed::<i32>,
        |&(ref x, ref y, i)| {
            assert_eq!(
                (x + y).mod_op(i),
                (Integer::from(x.mod_op(i)) + Integer::from(y.mod_op(i))).mod_op(i),
            );
        },
    );
}

fn rem_i32_properties_helper(n: &Integer, i: i32) {
    let mut mut_n = n.clone();
    mut_n %= i;
    assert!(mut_n.is_valid());
    let remainder = mut_n;

    let remainder_alt = n % i;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = n.clone() % i;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    assert_eq!(n.div_rem(i).1, remainder);

    //TODO assert_eq!(n % Integer::from(u), remainder);

    assert_eq!(bigint_to_integer(&(integer_to_bigint(n) % i)), remainder);
    assert_eq!(
        rug_integer_to_integer(&(integer_to_rug_integer(n) % i)),
        remainder
    );

    assert!(remainder.lt_abs(&i));
    assert!(remainder == 0 || (remainder > 0) == (*n > 0));
    assert_eq!(-n % i, -(n % i));
}

#[test]
fn rem_i32_properties() {
    test_properties(
        pairs_of_integer_and_nonzero_signed,
        |&(ref n, i): &(Integer, i32)| {
            rem_i32_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_integer_and_nonzero_i32_var_1,
        |&(ref n, i): &(Integer, i32)| {
            rem_i32_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_signed_and_nonzero_integer,
        |&(i, ref n): &(i32, Integer)| {
            let remainder = i % n;
            assert_eq!(i % n.clone(), remainder);

            assert_eq!(i.div_rem(n).1, remainder);

            if i > 0 && i.lt_abs(n) {
                assert_eq!(remainder, i);
            }
            assert!(remainder.lt_abs(n));
            assert!(remainder == 0 || (remainder > 0) == (i > 0));
            assert_eq!(i % -n, remainder);
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n % 1i32, 0);
        assert_eq!(n % -1i32, 0);
    });

    test_properties(nonzero_signeds, |&i: &i32| {
        assert_eq!(i % Integer::ONE, 0);
        assert_eq!(i % Integer::NEGATIVE_ONE, 0);
        assert_eq!(i % Integer::from(i), 0);
        assert_eq!(Integer::from(i) % i, 0);
        assert_eq!(i % -Integer::from(i), 0);
        assert_eq!(-Integer::from(i) % i, 0);
        assert_eq!(Integer::ZERO % i, 0);
        if i > 1 {
            assert_eq!(Integer::ONE % i, 1);
            assert_eq!(Integer::NEGATIVE_ONE % i, -1);
        }
    });

    test_properties(
        triples_of_integer_integer_and_nonzero_signed::<i32>,
        |&(ref x, ref y, i)| {
            assert_eq!(x * y % i, Integer::from(x % i) * Integer::from(y % i) % i);
        },
    );
}

fn ceiling_mod_i32_properties_helper(n: &Integer, i: i32) {
    let mut mut_n = n.clone();
    mut_n.ceiling_mod_assign(i);
    assert!(mut_n.is_valid());
    let remainder = mut_n;

    let remainder_alt = n.ceiling_mod(i);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = n.clone().ceiling_mod(i);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    assert_eq!(n.ceiling_div_mod(i).1, remainder);

    //TODO assert_eq!(n.ceiling_mod(Integer::from(u)), remainder);

    assert_eq!(
        rug_integer_to_integer(&integer_to_rug_integer(n).rem_ceil(i)),
        remainder
    );

    assert!(remainder.lt_abs(&i));
    assert!(remainder == 0 || (remainder >= 0) != (i > 0));
    assert_eq!((-n).ceiling_mod(i), -n.mod_op(i));
}

#[test]
fn ceiling_mod_i32_properties() {
    test_properties(
        pairs_of_integer_and_nonzero_signed,
        |&(ref n, i): &(Integer, i32)| {
            ceiling_mod_i32_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_integer_and_nonzero_i32_var_1,
        |&(ref n, i): &(Integer, i32)| {
            ceiling_mod_i32_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_signed_and_nonzero_integer,
        |&(i, ref n): &(i32, Integer)| {
            let remainder = i.ceiling_mod(n);
            assert!(remainder.is_valid());

            let remainder_alt = i.ceiling_mod(n.clone());
            assert!(remainder_alt.is_valid());
            assert_eq!(remainder_alt, remainder);

            assert_eq!(i.ceiling_div_mod(n).1, remainder);

            assert!(remainder.lt_abs(n));
            assert!(remainder == 0 || (remainder >= 0) != (*n > 0));
            assert_eq!(i.ceiling_mod(-n), i.mod_op(n));
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n.ceiling_mod(1i32), 0);
        assert_eq!(n.ceiling_mod(-1i32), 0);
    });

    test_properties(nonzero_signeds, |&i: &i32| {
        assert_eq!(i.ceiling_mod(Integer::ONE), 0);
        assert_eq!(i.ceiling_mod(Integer::NEGATIVE_ONE), 0);
        assert_eq!(i.ceiling_mod(Integer::from(i)), 0);
        assert_eq!(Integer::from(i).ceiling_mod(i), 0);
        assert_eq!(i.ceiling_mod(-Integer::from(i)), 0);
        assert_eq!((-Integer::from(i)).ceiling_mod(i), 0);
        assert_eq!(Integer::ZERO.ceiling_mod(i), 0);
    });

    test_properties(
        triples_of_integer_integer_and_nonzero_signed::<i32>,
        |&(ref x, ref y, i)| {
            assert_eq!(
                (x + y).ceiling_mod(i),
                (Integer::from(x.mod_op(i)) + Integer::from(y.mod_op(i))).ceiling_mod(i)
            );
            assert_eq!(
                (x * y).ceiling_mod(i),
                (Integer::from(x % i) * Integer::from(y % i)).ceiling_mod(i)
            );
        },
    );
}
