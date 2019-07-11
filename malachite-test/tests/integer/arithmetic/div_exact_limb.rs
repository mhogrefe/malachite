use common::test_properties;
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign, DivRound};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::round::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedDoubleLimb, SignedLimb};
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{
    pairs_of_limb_and_nonzero_signed_limb_var_1, pairs_of_limb_and_positive_limb_var_1,
    pairs_of_signed_limb_and_positive_limb_var_1, positive_unsigneds,
};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_positive_limb_var_1, pairs_of_limb_and_nonzero_integer_var_2,
};
use malachite_test::integer::arithmetic::div_exact_limb::rug_div_exact_limb;
use rug;
use std::str::FromStr;

#[test]
fn test_div_exact_limb() {
    let test = |u, v: Limb, quotient| {
        let mut n = Integer::from_str(u).unwrap();
        n.div_exact_assign(v);
        assert_eq!(n.to_string(), quotient);
        assert!(n.is_valid());

        let q = Integer::from_str(u).unwrap().div_exact(v);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = (&Integer::from_str(u).unwrap()).div_exact(v);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = Integer::from_str(u)
            .unwrap()
            .div_round(v, RoundingMode::Exact);
        assert_eq!(q.to_string(), quotient);

        let q = rug_div_exact_limb(rug::Integer::from_str(u).unwrap(), v);
        assert_eq!(q.to_string(), quotient);
    };
    test("0", 1, "0");
    test("0", 123, "0");
    test("1", 1, "1");
    test("123", 1, "123");
    test("123", 123, "1");
    test("912", 456, "2");
    test("369", 123, "3");
    test("4294967295", 1, "4294967295");
    test("4294967295", 4_294_967_295, "1");
    test("1000000000000", 1, "1000000000000");
    test("333333333333", 3, "111111111111");
    test("999999999900", 123, "8130081300");
    test("996432412440", 4_294_967_295, "232");
    test("1000000000000000000000000", 1, "1000000000000000000000000");
    test("333333333333333333333333", 3, "111111111111111111111111");
    test("999999999999999999999963", 123, "8130081300813008130081");
    test("999999999999996832276305", 4_294_967_295, "232830643708079");

    test("-1", 1, "-1");
    test("-123", 1, "-123");
    test("-123", 123, "-1");
    test("-912", 456, "-2");
    test("-369", 123, "-3");
    test("-4294967295", 1, "-4294967295");
    test("-4294967295", 4_294_967_295, "-1");
    test("-1000000000000", 1, "-1000000000000");
    test("-333333333333", 3, "-111111111111");
    test("-999999999900", 123, "-8130081300");
    test("-996432412440", 4_294_967_295, "-232");
    test(
        "-1000000000000000000000000",
        1,
        "-1000000000000000000000000",
    );
    test("-333333333333333333333333", 3, "-111111111111111111111111");
    test("-999999999999999999999963", 123, "-8130081300813008130081");
    test(
        "-999999999999996832276305",
        4_294_967_295,
        "-232830643708079",
    );
}

#[test]
#[should_panic]
fn div_exact_assign_limb_fail() {
    let mut n = Integer::from(10);
    n.div_exact_assign(0 as Limb);
}

#[test]
#[should_panic]
fn div_exact_limb_fail() {
    Integer::from(10).div_exact(0 as Limb);
}

#[test]
#[should_panic]
fn div_exact_limb_ref_fail() {
    (&Integer::from(10)).div_exact(0 as Limb);
}

#[test]
fn test_limb_div_exact_integer() {
    let test = |u: Limb, v, quotient| {
        let q = u.div_exact(Integer::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = u.div_exact(&Integer::from_str(v).unwrap());
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
    test(4_294_967_295, "1", "4294967295");
    test(4_294_967_295, "4294967295", "1");

    test(0, "-1", "0");
    test(0, "-123", "0");
    test(1, "-1", "-1");
    test(123, "-1", "-123");
    test(123, "-123", "-1");
    test(912, "-456", "-2");
    test(369, "-123", "-3");
    test(4_294_967_295, "-1", "-4294967295");
    test(4_294_967_295, "-4294967295", "-1");
}

#[test]
#[should_panic]
fn limb_div_exact_integer_fail() {
    (10 as Limb).div_exact(Integer::ZERO);
}

#[test]
#[should_panic]
fn limb_div_exact_integer_ref_fail() {
    (10 as Limb).div_exact(&Integer::ZERO);
}

fn div_exact_limb_properties_helper(n: &Integer, u: Limb) {
    let mut mut_n = n.clone();
    mut_n.div_exact_assign(u);
    assert!(mut_n.is_valid());
    let quotient = mut_n;

    let quotient_alt = n.div_exact(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = n.clone().div_exact(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = n.div_round(u, RoundingMode::Exact);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    //TODO assert_eq!(n.div_exact(Integer::from(u)), quotient);

    assert_eq!(
        rug_integer_to_integer(&rug_div_exact_limb(integer_to_rug_integer(n), u)),
        quotient
    );

    assert_eq!((-n).div_exact(u), -&quotient);

    assert_eq!(quotient * u, *n);
}

#[test]
fn div_exact_limb_properties() {
    test_properties(
        pairs_of_integer_and_positive_limb_var_1,
        |&(ref n, u): &(Integer, Limb)| {
            div_exact_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_limb_and_nonzero_integer_var_2,
        |&(u, ref n): &(Limb, Integer)| {
            let quotient = u.div_exact(n);
            assert!(quotient.is_valid());

            let quotient_alt = u.div_exact(n.clone());
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);

            assert_eq!(u.div_round(n, RoundingMode::Exact), quotient);

            assert_eq!(u.div_exact(-n), -&quotient);

            assert_eq!(quotient * n, u);
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n.div_exact(1 as Limb), *n);
    });

    test_properties(positive_unsigneds, |&u: &Limb| {
        assert_eq!(Integer::ZERO.div_exact(u), 0 as Limb);
        assert_eq!(u.div_exact(Integer::ONE), u);
        assert_eq!(u.div_exact(Integer::NEGATIVE_ONE), -Natural::from(u));
        assert_eq!(u.div_exact(Integer::from(u)), 1 as Limb);
        assert_eq!(Integer::from(u).div_exact(u), 1 as Limb);
        assert_eq!(u.div_exact(-Natural::from(u)), -1 as SignedLimb);
        assert_eq!((-Natural::from(u)).div_exact(u), -1 as SignedLimb);
    });

    test_properties(pairs_of_limb_and_nonzero_signed_limb_var_1, |&(x, y)| {
        let quotient =
            Integer::from(SignedDoubleLimb::from(x).div_exact(SignedDoubleLimb::from(y)));
        assert_eq!(quotient, DivExact::div_exact(x, Integer::from(y)));
    });

    test_properties(pairs_of_signed_limb_and_positive_limb_var_1, |&(x, y)| {
        let quotient =
            Integer::from(SignedDoubleLimb::from(x).div_exact(SignedDoubleLimb::from(y)));
        assert_eq!(quotient, Integer::from(x).div_exact(y));
    });

    test_properties(pairs_of_limb_and_positive_limb_var_1, |&(x, y)| {
        let quotient = x.div_exact(y);
        assert_eq!(quotient, Integer::from(x).div_exact(y));
        assert_eq!(quotient, DivExact::div_exact(x, Integer::from(y)));
    });
}
