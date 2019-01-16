use common::test_properties;
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{
    CeilingDivAssignMod, CeilingDivMod, CeilingMod, DivAssignMod, DivAssignRem, DivMod, DivRem,
    DivRound, Mod, NegativeOne, One, PartialOrdAbs, Zero,
};
use malachite_base::round::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::{pairs_of_unsigned_and_positive_unsigned, positive_unsigneds};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_positive_limb_var_1, pairs_of_integer_and_positive_unsigned,
    pairs_of_unsigned_and_nonzero_integer,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_positive_unsigned, pairs_of_unsigned_and_positive_natural,
};
#[cfg(feature = "32_bit_limbs")]
use malachite_test::integer::arithmetic::div_mod_limb::{num_div_mod_u32, rug_div_mod_u32};
use malachite_test::integer::arithmetic::div_mod_limb::{
    num_div_rem_limb, rug_ceiling_div_mod_limb, rug_div_rem_limb,
};
use num::BigInt;
use rug;
use std::str::FromStr;

#[test]
fn test_div_mod_limb() {
    let test = |u, v: Limb, quotient, remainder| {
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

        #[cfg(feature = "32_bit_limbs")]
        {
            let (q, r) = num_div_mod_u32(BigInt::from_str(u).unwrap(), v);
            assert_eq!(q.to_string(), quotient);
            assert_eq!(r, remainder);

            let (q, r) = rug_div_mod_u32(rug::Integer::from_str(u).unwrap(), v);
            assert_eq!(q.to_string(), quotient);
            assert_eq!(r, remainder);
        }

        let (q, r) = (
            Integer::from_str(u)
                .unwrap()
                .div_round(v, RoundingMode::Floor),
            Integer::from_str(u).unwrap().mod_op(v),
        );
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r, remainder);
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
fn div_assign_mod_limb_fail() {
    Integer::from(10).div_assign_mod(0 as Limb);
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_mod_limb_fail() {
    Integer::from(10).div_mod(0 as Limb);
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_mod_limb_ref_fail() {
    (&Integer::from(10)).div_mod(0 as Limb);
}

#[test]
fn test_div_rem_limb() {
    let test = |u, v: Limb, quotient, remainder| {
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

        let (q, r) = num_div_rem_limb(BigInt::from_str(u).unwrap(), v);
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r.to_string(), remainder);

        let (q, r) = rug_div_rem_limb(rug::Integer::from_str(u).unwrap(), v);
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
fn div_assign_rem_limb_fail() {
    Integer::from(10).div_assign_rem(0 as Limb);
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_rem_limb_fail() {
    Integer::from(10).div_rem(0 as Limb);
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_rem_limb_ref_fail() {
    (&Integer::from(10)).div_rem(0 as Limb);
}

#[test]
fn test_ceiling_div_mod_limb() {
    let test = |u, v: Limb, quotient, remainder| {
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

        let (q, r) = rug_ceiling_div_mod_limb(rug::Integer::from_str(u).unwrap(), v);
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
fn ceiling_div_assign_mod_limb_fail() {
    Integer::from(10).ceiling_div_assign_mod(0 as Limb);
}

#[test]
#[should_panic(expected = "division by zero")]
fn ceiling_div_mod_limb_fail() {
    Integer::from(10).ceiling_div_mod(0 as Limb);
}

#[test]
#[should_panic(expected = "division by zero")]
fn ceiling_div_mod_limb_ref_fail() {
    (&Integer::from(10)).ceiling_div_mod(0 as Limb);
}

#[test]
fn test_limb_div_mod_integer() {
    let test = |u: Limb, v, quotient, remainder| {
        let (q, r) = u.div_mod(Integer::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = u.div_mod(&Integer::from_str(v).unwrap());
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
    test(4_294_967_295, "1", "4294967295", "0");
    test(4_294_967_295, "4294967295", "1", "0");
    test(0, "1000000000000", "0", "0");
    test(123, "1000000000000", "0", "123");

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
fn limb_div_mod_integer_fail() {
    (10 as Limb).div_mod(Integer::ZERO);
}

#[test]
#[should_panic(expected = "division by zero")]
fn limb_div_mod_integer_ref_fail() {
    (10 as Limb).div_mod(&Integer::ZERO);
}

#[test]
fn test_limb_div_rem_integer() {
    let test = |u: Limb, v, quotient, remainder| {
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
fn limb_div_rem_integer_fail() {
    (10 as Limb).div_rem(Integer::ZERO);
}

#[test]
#[should_panic(expected = "division by zero")]
fn limb_div_rem_integer_ref_fail() {
    (10 as Limb).div_rem(&Integer::ZERO);
}

#[test]
fn test_limb_ceiling_div_mod_integer() {
    let test = |u: Limb, v, quotient, remainder| {
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
    test(123, "-456", "0", "123");
    test(456, "-123", "-3", "87");
    test(4_294_967_295, "-1", "-4294967295", "0");
    test(4_294_967_295, "-4294967295", "-1", "0");
    test(0, "-1000000000000", "0", "0");
    test(123, "-1000000000000", "0", "123");
}

#[test]
#[should_panic(expected = "division by zero")]
fn limb_ceiling_div_mod_integer_fail() {
    (10 as Limb).ceiling_div_mod(Integer::ZERO);
}

#[test]
#[should_panic(expected = "division by zero")]
fn limb_ceiling_div_mod_integer_ref_fail() {
    (10 as Limb).ceiling_div_mod(&Integer::ZERO);
}

fn div_mod_limb_properties_helper(n: &Integer, u: Limb) {
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

    let (quotient_alt, remainder_alt) = (n.div_round(u, RoundingMode::Floor), n.mod_op(u));
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    //TODO assert_eq!(n.div_mod(Integer::from(u)), (quotient.clone(), remainder));

    #[cfg(feature = "32_bit_limbs")]
    {
        let (num_quotient, num_remainder) = num_div_mod_u32(integer_to_bigint(n), u);
        assert_eq!(bigint_to_integer(&num_quotient), quotient);
        assert_eq!(num_remainder, remainder);

        let (rug_quotient, rug_remainder) = rug_div_mod_u32(integer_to_rug_integer(n), u);
        assert_eq!(rug_integer_to_integer(&rug_quotient), quotient);
        assert_eq!(rug_remainder, remainder);
    }

    assert!(remainder < u);
    assert_eq!(quotient * u + remainder, *n);

    let (neg_quotient, neg_remainder) = (-n).div_mod(u);
    assert_eq!(
        n.ceiling_div_mod(u),
        (-neg_quotient, -Natural::from(neg_remainder))
    );
}

#[test]
fn div_mod_limb_properties() {
    test_properties(
        pairs_of_integer_and_positive_unsigned,
        |&(ref n, u): &(Integer, Limb)| {
            div_mod_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_integer_and_positive_limb_var_1,
        |&(ref n, u): &(Integer, Limb)| {
            div_mod_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_unsigned_and_nonzero_integer,
        |&(u, ref n): &(Limb, Integer)| {
            let (quotient, remainder) = u.div_mod(n);
            assert!(quotient.is_valid());
            assert!(remainder.is_valid());

            let (quotient_alt, remainder_alt) = u.div_mod(n.clone());
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);
            assert!(remainder_alt.is_valid());
            assert_eq!(remainder_alt, remainder);

            let (quotient_alt, remainder_alt) = (u.div_round(n, RoundingMode::Floor), u.mod_op(n));
            assert_eq!(quotient_alt, quotient);
            assert_eq!(remainder_alt, remainder);

            //TODO assert_eq!((quotient.clone(), remainder), Natural::from(u).div_mod(n));

            if u != 0 && u < *n {
                assert_eq!(remainder, u);
            }
            assert!(remainder.lt_abs(n));
            assert!(remainder == 0 as Limb || (remainder > 0 as Limb) == (*n > 0 as Limb));
            assert_eq!(&quotient * n + &remainder, u);

            let (neg_quotient, neg_remainder) = u.div_mod(-n);
            assert_eq!(u.ceiling_div_mod(n), (-neg_quotient, neg_remainder));
        },
    );

    test_properties(integers, |n| {
        let (q, r) = n.div_mod(1 as Limb);
        assert_eq!(q, *n);
        assert_eq!(r, 0);
    });

    test_properties(positive_unsigneds, |&u: &Limb| {
        assert_eq!(u.div_mod(Integer::ONE), (Integer::from(u), Integer::ZERO));
        assert_eq!(
            u.div_mod(Integer::NEGATIVE_ONE),
            (-Natural::from(u), Integer::ZERO)
        );
        assert_eq!(u.div_mod(Integer::from(u)), (Integer::ONE, Integer::ZERO));
        assert_eq!(Integer::from(u).div_mod(u), (Integer::ONE, 0));
        assert_eq!(
            u.div_mod(-Natural::from(u)),
            (Integer::NEGATIVE_ONE, Integer::ZERO)
        );
        assert_eq!((-Natural::from(u)).div_mod(u), (Integer::NEGATIVE_ONE, 0));
        assert_eq!(Integer::ZERO.div_mod(u), (Integer::ZERO, 0));
        if u > 1 {
            assert_eq!(Integer::ONE.div_mod(u), (Integer::ZERO, 1));
            assert_eq!(
                Integer::NEGATIVE_ONE.div_mod(u),
                (Integer::NEGATIVE_ONE, u - 1)
            );
        }
    });

    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<Limb>,
        |&(x, y)| {
            let (quotient, remainder) = x.div_mod(y);
            let quotient = Integer::from(quotient);
            assert_eq!((quotient.clone(), remainder), Integer::from(x).div_mod(y));
            assert_eq!(
                (quotient, Integer::from(remainder)),
                x.div_mod(Integer::from(y))
            );
        },
    );

    test_properties(
        pairs_of_natural_and_positive_unsigned::<Limb>,
        |&(ref n, u)| {
            let (quotient, remainder) = n.div_mod(u);
            assert_eq!(
                (Integer::from(quotient), remainder),
                Integer::from(n).div_mod(u)
            );
        },
    );

    test_properties(
        pairs_of_unsigned_and_positive_natural::<Limb>,
        |&(u, ref n)| {
            let (quotient, remainder) = u.div_mod(n);
            assert_eq!(
                (Integer::from(quotient), Integer::from(remainder)),
                u.div_mod(Integer::from(n))
            );
        },
    );
}

fn div_rem_limb_properties_helper(n: &Integer, u: Limb) {
    let mut mut_n = n.clone();
    let remainder = mut_n.div_assign_rem(u);
    assert!(mut_n.is_valid());
    assert!(remainder.is_valid());
    let quotient = mut_n;

    let (quotient_alt, remainder_alt) = n.div_rem(u);
    assert!(quotient_alt.is_valid());
    assert!(remainder_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = n.clone().div_rem(u);
    assert!(quotient_alt.is_valid());
    assert!(remainder_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = (n / u, n % u);
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    //TODO assert_eq!(n.div_rem(Integer::from(u)), (quotient.clone(), remainder));

    let (num_quotient, num_remainder) = num_div_rem_limb(integer_to_bigint(n), u);
    assert_eq!(bigint_to_integer(&num_quotient), quotient);
    assert_eq!(bigint_to_integer(&num_remainder), remainder);

    let (rug_quotient, rug_remainder) = rug_div_rem_limb(integer_to_rug_integer(n), u);
    assert_eq!(rug_integer_to_integer(&rug_quotient), quotient);
    assert_eq!(rug_integer_to_integer(&rug_remainder), remainder);

    assert!(remainder.lt_abs(&u));
    assert!(remainder == 0 as Limb || (remainder > 0 as Limb) == (*n > 0 as Limb));
    assert_eq!(&quotient * u + &remainder, *n);

    assert_eq!((-n).div_rem(u), (-quotient, -remainder));
}

#[test]
fn div_rem_limb_properties() {
    test_properties(
        pairs_of_integer_and_positive_unsigned,
        |&(ref n, u): &(Integer, Limb)| {
            div_rem_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_integer_and_positive_limb_var_1,
        |&(ref n, u): &(Integer, Limb)| {
            div_rem_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_unsigned_and_nonzero_integer,
        |&(u, ref n): &(Limb, Integer)| {
            let (quotient, remainder) = u.div_rem(n);
            assert!(quotient.is_valid());

            let (quotient_alt, remainder_alt) = u.div_rem(n.clone());
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);
            assert_eq!(remainder_alt, remainder);

            let (quotient_alt, remainder_alt) = (u / n, u % n);
            assert_eq!(quotient_alt, quotient);
            assert_eq!(remainder_alt, remainder);

            //TODO assert_eq!((quotient.clone(), remainder), Natural::from(u).div_rem(n));

            if u > 0 && u.lt_abs(n) {
                assert_eq!(remainder, u);
            }
            assert!(remainder.lt_abs(n));
            assert_eq!(&quotient * n + remainder, u);

            assert_eq!(u.div_rem(-n), (-quotient, remainder));
        },
    );

    test_properties(integers, |n| {
        let (q, r) = n.div_rem(1 as Limb);
        assert_eq!(q, *n);
        assert_eq!(r, 0 as Limb);
    });

    test_properties(positive_unsigneds, |&u: &Limb| {
        assert_eq!(u.div_rem(Integer::ONE), (Integer::from(u), 0));
        assert_eq!(u.div_rem(Integer::NEGATIVE_ONE), (-Natural::from(u), 0));
        assert_eq!(u.div_rem(Integer::from(u)), (Integer::ONE, 0));
        assert_eq!(Integer::from(u).div_rem(u), (Integer::ONE, Integer::ZERO));
        assert_eq!(u.div_rem(-Natural::from(u)), (Integer::NEGATIVE_ONE, 0));
        assert_eq!(
            (-Natural::from(u)).div_rem(u),
            (Integer::NEGATIVE_ONE, Integer::ZERO)
        );
        assert_eq!(Integer::ZERO.div_rem(u), (Integer::ZERO, Integer::ZERO));
        if u > 1 {
            assert_eq!(Integer::ONE.div_rem(u), (Integer::ZERO, Integer::ONE));
            assert_eq!(
                Integer::NEGATIVE_ONE.div_rem(u),
                (Integer::ZERO, Integer::NEGATIVE_ONE)
            );
        }
    });

    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<Limb>,
        |&(x, y)| {
            let (quotient, remainder) = x.div_rem(y);
            let quotient = Integer::from(quotient);
            assert_eq!(
                (quotient.clone(), Integer::from(remainder)),
                Integer::from(x).div_rem(y)
            );
            assert_eq!((quotient, remainder), x.div_rem(Integer::from(y)));
        },
    );

    test_properties(
        pairs_of_natural_and_positive_unsigned::<Limb>,
        |&(ref n, u)| {
            let (quotient, remainder) = n.div_rem(u);
            assert_eq!(
                (Integer::from(quotient), Integer::from(remainder)),
                Integer::from(n).div_rem(u)
            );
        },
    );

    test_properties(
        pairs_of_unsigned_and_positive_natural::<Limb>,
        |&(u, ref n)| {
            let (quotient, remainder) = u.div_rem(n);
            assert_eq!(
                (Integer::from(quotient), remainder),
                u.div_rem(Integer::from(n))
            );
        },
    );
}

fn ceiling_div_mod_limb_properties_helper(n: &Integer, u: Limb) {
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

    let (quotient_alt, remainder_alt) = (n.div_round(u, RoundingMode::Ceiling), n.ceiling_mod(u));
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    //TODO assert_eq!(n.ceiling_div_mod(Integer::from(u)), (quotient.clone(), remainder));

    let (rug_quotient, rug_remainder) = rug_ceiling_div_mod_limb(integer_to_rug_integer(n), u);
    assert_eq!(rug_integer_to_integer(&rug_quotient), quotient);
    assert_eq!(rug_integer_to_integer(&rug_remainder), remainder);

    assert!(remainder <= 0 as Limb);
    assert!(-&remainder < u);
    assert_eq!(quotient * u + remainder, *n);

    let (neg_quotient, neg_remainder) = (-n).ceiling_div_mod(u);
    assert_eq!(
        n.div_mod(u),
        (-neg_quotient, Limb::checked_from(-neg_remainder).unwrap())
    );
}

#[test]
fn ceiling_div_mod_limb_properties() {
    test_properties(
        pairs_of_integer_and_positive_unsigned,
        |&(ref n, u): &(Integer, Limb)| {
            ceiling_div_mod_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_integer_and_positive_limb_var_1,
        |&(ref n, u): &(Integer, Limb)| {
            ceiling_div_mod_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_unsigned_and_nonzero_integer,
        |&(u, ref n): &(Limb, Integer)| {
            let (quotient, remainder) = u.ceiling_div_mod(n);
            assert!(quotient.is_valid());
            assert!(remainder.is_valid());

            let (quotient_alt, remainder_alt) = u.ceiling_div_mod(n.clone());
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);
            assert!(remainder_alt.is_valid());
            assert_eq!(remainder_alt, remainder);

            let (quotient_alt, remainder_alt) =
                (u.div_round(n, RoundingMode::Ceiling), u.ceiling_mod(n));
            assert_eq!(quotient_alt, quotient);
            assert_eq!(remainder_alt, remainder);

            //TODO assert_eq!((quotient, remainder), Natural::from(u).ceiling_div_mod(n));

            assert!(remainder.lt_abs(n));
            assert!(remainder == 0 as Limb || (remainder > 0 as Limb) != (*n > 0 as Limb));
            assert_eq!(&quotient * n + &remainder, u);

            let (neg_quotient, neg_remainder) = u.ceiling_div_mod(-n);
            assert_eq!(u.div_mod(n), (-neg_quotient, neg_remainder));
        },
    );

    test_properties(integers, |n| {
        let (q, r) = n.ceiling_div_mod(1 as Limb);
        assert_eq!(q, *n);
        assert_eq!(r, 0 as Limb);
    });

    test_properties(positive_unsigneds, |&u: &Limb| {
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
            Integer::from(u).ceiling_div_mod(u),
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
