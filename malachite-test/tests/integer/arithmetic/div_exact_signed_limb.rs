use common::test_properties;
use malachite_base::num::{DivExact, DivExactAssign, DivRound, NegativeOne, One, Zero};
use malachite_base::round::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedDoubleLimb, SignedLimb};
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{
    nonzero_signeds, pairs_of_signed_limb_and_nonzero_signed_limb_var_1,
};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_nonzero_signed_limb_var_1,
    pairs_of_signed_limb_and_nonzero_integer_var_2,
};
use malachite_test::integer::arithmetic::div_exact_signed_limb::rug_div_exact_signed_limb;
use rug;
use std::str::FromStr;

#[test]
fn test_div_exact_signed_limb() {
    let test = |i, j: SignedLimb, quotient| {
        let mut n = Integer::from_str(i).unwrap();
        n.div_exact_assign(j);
        assert_eq!(n.to_string(), quotient);
        assert!(n.is_valid());

        let q = Integer::from_str(i).unwrap().div_exact(j);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = (&Integer::from_str(i).unwrap()).div_exact(j);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = Integer::from_str(i)
            .unwrap()
            .div_round(j, RoundingMode::Exact);
        assert_eq!(q.to_string(), quotient);

        let q = rug_div_exact_signed_limb(rug::Integer::from_str(i).unwrap(), j);
        assert_eq!(q.to_string(), quotient);
    };
    test("0", 1, "0");
    test("0", 123, "0");
    test("1", 1, "1");
    test("123", 1, "123");
    test("123", 123, "1");
    test("912", 456, "2");
    test("369", 123, "3");
    test("2147483647", 1, "2147483647");
    test("2147483647", 2_147_483_647, "1");
    test("333333333333", 3, "111111111111");
    test("999999999900", 123, "8130081300");
    test("998579895855", 2_147_483_647, "465");
    test("1000000000000", 1, "1000000000000");
    test("1000000000000000000000000", 1, "1000000000000000000000000");
    test("333333333333333333333333", 3, "111111111111111111111111");
    test("999999999999999999999963", 123, "8130081300813008130081");
    test("999999999999998513059613", 2_147_483_647, "465661287524579");

    test("-1", 1, "-1");
    test("-123", 1, "-123");
    test("-123", 123, "-1");
    test("-912", 456, "-2");
    test("-369", 123, "-3");
    test("-2147483647", 1, "-2147483647");
    test("-2147483647", 2_147_483_647, "-1");
    test("-333333333333", 3, "-111111111111");
    test("-999999999900", 123, "-8130081300");
    test("-998579895855", 2_147_483_647, "-465");
    test("-1000000000000", 1, "-1000000000000");
    test(
        "-1000000000000000000000000",
        1,
        "-1000000000000000000000000",
    );
    test("-333333333333333333333333", 3, "-111111111111111111111111");
    test("-999999999999999999999963", 123, "-8130081300813008130081");
    test(
        "-999999999999998513059613",
        2_147_483_647,
        "-465661287524579",
    );

    test("0", -1, "0");
    test("0", -123, "0");
    test("1", -1, "-1");
    test("123", -1, "-123");
    test("123", -123, "-1");
    test("912", -456, "-2");
    test("369", -123, "-3");
    test("2147483647", -1, "-2147483647");
    test("2147483647", -2_147_483_647, "-1");
    test("2147483648", -2_147_483_648, "-1");
    test("333333333333", -3, "-111111111111");
    test("999999999900", -123, "-8130081300");
    test("998579895855", -2_147_483_647, "-465");
    test("998579896320", -2_147_483_648, "-465");
    test("1000000000000", -1, "-1000000000000");
    test(
        "1000000000000000000000000",
        -1,
        "-1000000000000000000000000",
    );
    test("333333333333333333333333", -3, "-111111111111111111111111");
    test("999999999999999999999963", -123, "-8130081300813008130081");
    test(
        "999999999999998513059613",
        -2_147_483_647,
        "-465661287524579",
    );
    test(
        "999999999999999446351872",
        -2_147_483_648,
        "-465661287307739",
    );

    test("-1", -1, "1");
    test("-123", -1, "123");
    test("-123", -123, "1");
    test("-912", -456, "2");
    test("-369", -123, "3");
    test("-2147483647", -1, "2147483647");
    test("-2147483647", -2_147_483_647, "1");
    test("-2147483648", -2_147_483_648, "1");
    test("-333333333333", -3, "111111111111");
    test("-999999999900", -123, "8130081300");
    test("-998579895855", -2_147_483_647, "465");
    test("-998579896320", -2_147_483_648, "465");
    test("-1000000000000", -1, "1000000000000");
    test(
        "-1000000000000000000000000",
        -1,
        "1000000000000000000000000",
    );
    test("-333333333333333333333333", -3, "111111111111111111111111");
    test("-999999999999999999999963", -123, "8130081300813008130081");
    test(
        "-999999999999998513059613",
        -2_147_483_647,
        "465661287524579",
    );
    test(
        "-999999999999999446351872",
        -2_147_483_648,
        "465661287307739",
    );
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_exact_assign_signed_limb_fail() {
    let mut n = Integer::from(10);
    n.div_exact_assign(0 as SignedLimb);
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_exact_signed_limb_fail() {
    Integer::from(10).div_exact(0 as SignedLimb);
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_exact_signed_limb_ref_fail() {
    (&Integer::from(10)).div_exact(0 as SignedLimb);
}

#[test]
fn test_signed_limb_div_exact_integer() {
    let test = |i: SignedLimb, j, quotient| {
        let q = i.div_exact(Integer::from_str(j).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = i.div_exact(&Integer::from_str(j).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
    };
    test(0, "1", "0");
    test(0, "123", "0");
    test(1, "1", "1");
    test(123, "1", "123");
    test(123, "123", "1");
    test(912, "456", "2");
    test(369, "123", "3");
    test(2_147_483_647, "1", "2147483647");
    test(2_147_483_647, "2147483647", "1");

    test(0, "-1", "0");
    test(0, "-123", "0");
    test(1, "-1", "-1");
    test(123, "-1", "-123");
    test(123, "-123", "-1");
    test(912, "-456", "-2");
    test(369, "-123", "-3");
    test(2_147_483_647, "-1", "-2147483647");
    test(2_147_483_647, "-2147483647", "-1");

    test(-1, "1", "-1");
    test(-123, "1", "-123");
    test(-123, "123", "-1");
    test(-912, "456", "-2");
    test(-369, "123", "-3");
    test(-2_147_483_647, "1", "-2147483647");
    test(-2_147_483_647, "2147483647", "-1");
    test(-2_147_483_648, "1", "-2147483648");
    test(-2_147_483_648, "2147483648", "-1");

    test(-1, "-1", "1");
    test(-123, "-1", "123");
    test(-123, "-123", "1");
    test(-912, "-456", "2");
    test(-369, "-123", "3");
    test(-2_147_483_647, "-1", "2147483647");
    test(-2_147_483_647, "-2147483647", "1");
    test(-2_147_483_648, "-1", "2147483648");
    test(-2_147_483_648, "-2147483648", "1");
}

#[test]
#[should_panic(expected = "division by zero")]
fn signed_limb_div_exact_integer_fail() {
    (10 as SignedLimb).div_exact(Integer::ZERO);
}

#[test]
#[should_panic(expected = "division by zero")]
fn signed_limb_div_exact_integer_ref_fail() {
    (10 as SignedLimb).div_exact(&Integer::ZERO);
}

fn div_exact_signed_limb_properties_helper(n: &Integer, i: SignedLimb) {
    let mut mut_n = n.clone();
    mut_n.div_exact_assign(i);
    assert!(mut_n.is_valid());
    let quotient = mut_n;

    let quotient_alt = n.div_exact(i);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = n.clone().div_exact(i);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = n.div_round(i, RoundingMode::Exact);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    //TODO assert_eq!(n.div_exact(Integer::from(u)), quotient);

    assert_eq!(
        rug_integer_to_integer(&rug_div_exact_signed_limb(integer_to_rug_integer(n), i)),
        quotient
    );

    assert_eq!((-n).div_exact(i), -&quotient);

    assert_eq!(quotient * i, *n);
}

#[test]
fn div_exact_signed_limb_properties() {
    test_properties(
        pairs_of_integer_and_nonzero_signed_limb_var_1,
        |&(ref n, i): &(Integer, SignedLimb)| {
            div_exact_signed_limb_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_signed_limb_and_nonzero_integer_var_2,
        |&(i, ref n): &(SignedLimb, Integer)| {
            let quotient = i.div_exact(n);
            assert!(quotient.is_valid());

            let quotient_alt = i.div_exact(n.clone());
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);

            assert_eq!(i.div_round(n, RoundingMode::Exact), quotient);

            assert_eq!(i.div_exact(-n), -&quotient);

            assert_eq!(quotient * n, i);
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n.div_exact(1 as SignedLimb), *n);
        assert_eq!(n.div_exact(-1 as SignedLimb), -n);
    });

    test_properties(nonzero_signeds, |&i: &SignedLimb| {
        assert_eq!(Integer::ZERO.div_exact(i), 0 as Limb);
        assert_eq!(i.div_exact(Integer::ONE), i);
        assert_eq!(i.div_exact(Integer::NEGATIVE_ONE), -Integer::from(i));
        assert_eq!(i.div_exact(Integer::from(i)), 1 as Limb);
        assert_eq!(Integer::from(i).div_exact(i), 1 as Limb);
        assert_eq!(i.div_exact(-Integer::from(i)), -1 as SignedLimb);
        assert_eq!((-Integer::from(i)).div_exact(i), -1 as SignedLimb);
    });

    test_properties(
        pairs_of_signed_limb_and_nonzero_signed_limb_var_1,
        |&(x, y)| {
            let quotient =
                Integer::from(SignedDoubleLimb::from(x).div_exact(SignedDoubleLimb::from(y)));
            assert_eq!(quotient, Integer::from(x).div_exact(y));
            assert_eq!(quotient, DivExact::div_exact(x, Integer::from(y)));
        },
    );
}
