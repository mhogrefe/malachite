use common::test_properties;
use malachite_base::num::{DivRem, NegativeOne, One, PartialOrdAbs, Zero};
use malachite_nz::integer::Integer;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::nonzero_signeds;
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_nonzero_i32_var_1, pairs_of_integer_and_nonzero_signed,
    pairs_of_signed_and_nonzero_integer,
};
use num::BigInt;
use rug;
use std::str::FromStr;

#[test]
fn test_div_i32() {
    let test = |u, v: i32, quotient| {
        let mut n = Integer::from_str(u).unwrap();
        n /= v;
        assert_eq!(n.to_string(), quotient);
        assert!(n.is_valid());

        let q = Integer::from_str(u).unwrap() / v;
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = &Integer::from_str(u).unwrap() / v;
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = Integer::from_str(u).unwrap().div_rem(v).0;
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = BigInt::from_str(u).unwrap() / v;
        assert_eq!(q.to_string(), quotient);

        let q = rug::Integer::from_str(u).unwrap() / v;
        assert_eq!(q.to_string(), quotient);
    };
    test("0", 1, "0");
    test("0", 123, "0");
    test("1", 1, "1");
    test("123", 1, "123");
    test("123", 123, "1");
    test("123", 456, "0");
    test("456", 123, "3");
    test("2147483647", 1, "2147483647");
    test("2147483647", 2_147_483_647, "1");
    test("1000000000000", 1, "1000000000000");
    test("1000000000000", 3, "333333333333");
    test("1000000000000", 123, "8130081300");
    test("1000000000000", 2_147_483_647, "465");
    test("1000000000000000000000000", 1, "1000000000000000000000000");
    test("1000000000000000000000000", 3, "333333333333333333333333");
    test("1000000000000000000000000", 123, "8130081300813008130081");
    test(
        "1000000000000000000000000",
        2_147_483_647,
        "465661287524579",
    );

    test("-1", 1, "-1");
    test("-123", 1, "-123");
    test("-123", 123, "-1");
    test("-123", 456, "0");
    test("-456", 123, "-3");
    test("-2147483647", 1, "-2147483647");
    test("-2147483647", 2_147_483_647, "-1");
    test("-1000000000000", 1, "-1000000000000");
    test("-1000000000000", 3, "-333333333333");
    test("-1000000000000", 123, "-8130081300");
    test("-1000000000000", 2_147_483_647, "-465");
    test(
        "-1000000000000000000000000",
        1,
        "-1000000000000000000000000",
    );
    test("-1000000000000000000000000", 3, "-333333333333333333333333");
    test("-1000000000000000000000000", 123, "-8130081300813008130081");
    test(
        "-1000000000000000000000000",
        2_147_483_647,
        "-465661287524579",
    );

    test("0", -1, "0");
    test("0", -123, "0");
    test("1", -1, "-1");
    test("123", -1, "-123");
    test("123", -123, "-1");
    test("123", -456, "0");
    test("456", -123, "-3");
    test("2147483647", -1, "-2147483647");
    test("2147483647", -2_147_483_647, "-1");
    test("2147483648", -2_147_483_648, "-1");
    test("1000000000000", -1, "-1000000000000");
    test("1000000000000", -3, "-333333333333");
    test("1000000000000", -123, "-8130081300");
    test("1000000000000", -2_147_483_647, "-465");
    test("1000000000000", -2_147_483_648, "-465");
    test(
        "1000000000000000000000000",
        -1,
        "-1000000000000000000000000",
    );
    test("1000000000000000000000000", -3, "-333333333333333333333333");
    test("1000000000000000000000000", -123, "-8130081300813008130081");
    test(
        "1000000000000000000000000",
        -2_147_483_647,
        "-465661287524579",
    );
    test(
        "1000000000000000000000000",
        -2_147_483_648,
        "-465661287307739",
    );

    test("-1", -1, "1");
    test("-123", -1, "123");
    test("-123", -123, "1");
    test("-123", -456, "0");
    test("-456", -123, "3");
    test("-2147483647", -1, "2147483647");
    test("-2147483647", -2_147_483_647, "1");
    test("-1000000000000", -1, "1000000000000");
    test("-1000000000000", -3, "333333333333");
    test("-1000000000000", -123, "8130081300");
    test("-1000000000000", -2_147_483_647, "465");
    test("-1000000000000", -2_147_483_648, "465");
    test(
        "-1000000000000000000000000",
        -1,
        "1000000000000000000000000",
    );
    test("-1000000000000000000000000", -3, "333333333333333333333333");
    test("-1000000000000000000000000", -123, "8130081300813008130081");
    test(
        "-1000000000000000000000000",
        -2_147_483_647,
        "465661287524579",
    );
    test(
        "-1000000000000000000000000",
        -2_147_483_648,
        "465661287307739",
    );
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_assign_i32_fail() {
    let mut n = Integer::from(10i32);
    n /= 0i32;
}

#[test]
#[allow(unused_must_use)]
#[should_panic(expected = "division by zero")]
fn div_i32_fail() {
    Integer::from(10i32) / 0i32;
}

#[test]
#[allow(unused_must_use)]
#[should_panic(expected = "division by zero")]
fn div_i32_ref_fail() {
    &Integer::from(10i32) / 0i32;
}

#[test]
fn test_i32_div_integer() {
    let test = |u: i32, v, quotient| {
        let q = u / Integer::from_str(v).unwrap();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = u / &Integer::from_str(v).unwrap();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = u.div_rem(Integer::from_str(v).unwrap()).0;
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
    };
    test(0, "1", "0");
    test(0, "123", "0");
    test(1, "1", "1");
    test(123, "1", "123");
    test(123, "123", "1");
    test(123, "456", "0");
    test(456, "123", "3");
    test(2_147_483_647, "1", "2147483647");
    test(2_147_483_647, "2147483647", "1");
    test(0, "1000000000000", "0");
    test(123, "1000000000000", "0");

    test(1, "-1", "-1");
    test(123, "-1", "-123");
    test(123, "-123", "-1");
    test(123, "-456", "0");
    test(456, "-123", "-3");
    test(2_147_483_647, "-1", "-2147483647");
    test(2_147_483_647, "-2147483647", "-1");
    test(0, "-1000000000000", "0");
    test(123, "-1000000000000", "0");

    test(-1, "1", "-1");
    test(-123, "1", "-123");
    test(-123, "123", "-1");
    test(-123, "456", "0");
    test(-456, "123", "-3");
    test(-2_147_483_647, "1", "-2147483647");
    test(-2_147_483_647, "2147483647", "-1");
    test(-2_147_483_648, "2147483648", "-1");
    test(-123, "1000000000000", "0");

    test(-1, "-1", "1");
    test(-123, "-1", "123");
    test(-123, "-123", "1");
    test(-123, "-456", "0");
    test(-456, "-123", "3");
    test(-2_147_483_647, "-1", "2147483647");
    test(-2_147_483_648, "-1", "2147483648");
    test(-2_147_483_647, "-2147483647", "1");
    test(-2_147_483_648, "-2147483648", "1");
    test(-123, "-1000000000000", "0");
}

#[test]
#[allow(unused_must_use)]
#[should_panic(expected = "division by zero")]
fn i32_div_integer_fail() {
    10 / Integer::ZERO;
}

#[test]
#[allow(unused_must_use)]
#[should_panic(expected = "division by zero")]
fn i32_div_integer_ref_fail() {
    10 / &Integer::ZERO;
}

fn div_i32_properties_helper(n: &Integer, i: i32) {
    let mut mut_n = n.clone();
    mut_n /= i;
    assert!(mut_n.is_valid());
    let quotient = mut_n;

    let quotient_alt = n / i;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = n.clone() / i;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    assert_eq!(n.div_rem(i).0, quotient);

    //TODO assert_eq!(n / Integer::from(i), quotient);

    assert_eq!(bigint_to_integer(&(integer_to_bigint(n) / i)), quotient);
    assert_eq!(
        rug_integer_to_integer(&(integer_to_rug_integer(n) / i)),
        quotient
    );

    assert_eq!(-n / i, -&quotient);

    assert!((n - quotient * i).lt_abs(&i));
}

#[test]
fn div_i32_properties() {
    test_properties(
        pairs_of_integer_and_nonzero_signed,
        |&(ref n, i): &(Integer, i32)| {
            div_i32_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_integer_and_nonzero_i32_var_1,
        |&(ref n, i): &(Integer, i32)| {
            div_i32_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_signed_and_nonzero_integer,
        |&(i, ref n): &(i32, Integer)| {
            let quotient = i / n;
            assert!(quotient.is_valid());

            let quotient_alt = i / n.clone();
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);

            assert_eq!(i.div_rem(n).0, quotient);

            assert_eq!(i / -n, -&quotient);

            assert!((i - quotient * n).lt_abs(n));
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n / 1i32, *n);
        assert_eq!(n / -1i32, -n);
    });

    test_properties(nonzero_signeds, |&i: &i32| {
        assert_eq!(Integer::ZERO / i, 0);
        if i > 1 {
            assert_eq!(1 / i, 0);
        }
        assert_eq!(i / Integer::ONE, i);
        assert_eq!(i / Integer::NEGATIVE_ONE, -Integer::from(i));
        assert_eq!(i / Integer::from(i), 1);
        assert_eq!(Integer::from(i) / i, 1);
        assert_eq!(i / -Integer::from(i), -1);
        assert_eq!(-Integer::from(i) / i, -1);
    });
}
