use common::test_properties;
use malachite_base::num::arithmetic::traits::{
    CeilingDivMod, CeilingMod, CeilingModAssign, DivMod, DivRem, Mod, ModAssign, UnsignedAbs,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_test::common::{bigint_to_integer, integer_to_bigint};
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{nonzero_signeds, pairs_of_signed_and_nonzero_signed};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_nonzero_signed, pairs_of_integer_and_nonzero_signed_limb_var_1,
    pairs_of_signed_and_nonzero_integer, triples_of_integer_integer_and_nonzero_signed,
};
use malachite_test::integer::arithmetic::mod_signed_limb::num_mod_signed_limb;
use num::BigInt;
#[cfg(feature = "32_bit_limbs")]
use rug::{self, ops::RemRounding};
use std::str::FromStr;

#[test]
fn test_mod_signed_limb() {
    let test = |i, j: SignedLimb, remainder| {
        let mut n = Integer::from_str(i).unwrap();
        n.mod_assign(j);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), remainder);

        let remainder_alt = Integer::from_str(i).unwrap().mod_op(j);
        assert!(remainder_alt.is_valid());
        assert_eq!(remainder_alt.to_string(), remainder);

        let remainder_alt = (&Integer::from_str(i).unwrap()).mod_op(j);
        assert!(remainder_alt.is_valid());
        assert_eq!(remainder_alt.to_string(), remainder);

        assert_eq!(
            num_mod_signed_limb(BigInt::from_str(i).unwrap(), j).to_string(),
            remainder
        );
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(
            rug::Integer::from_str(i).unwrap().rem_floor(j).to_string(),
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
#[should_panic]
fn mod_assign_signed_limb_fail() {
    Integer::from(10).mod_assign(0 as SignedLimb);
}

#[test]
#[should_panic]
fn mod_signed_limb_fail() {
    Integer::from(10).mod_op(0 as SignedLimb);
}

#[test]
#[should_panic]
fn mod_signed_limb_ref_fail() {
    (&Integer::from(10)).mod_op(0 as SignedLimb);
}

#[test]
fn test_rem_signed_limb() {
    let test = |i, j: SignedLimb, remainder| {
        let mut n = Integer::from_str(i).unwrap();
        n %= j;
        assert!(n.is_valid());
        assert_eq!(n.to_string(), remainder);

        let r = Integer::from_str(i).unwrap() % j;
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
        let r = &Integer::from_str(i).unwrap() % j;
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        assert_eq!((BigInt::from_str(i).unwrap() % j).to_string(), remainder);
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(
            (rug::Integer::from_str(i).unwrap() % j).to_string(),
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
#[should_panic]
fn rem_assign_signed_limb_fail() {
    let mut n = Integer::from(10);
    n %= 0 as SignedLimb;
}

#[test]
#[allow(unused_must_use)]
#[should_panic]
fn rem_signed_limb_fail() {
    Integer::from(10) % 0 as SignedLimb;
}

#[test]
#[allow(unused_must_use)]
#[should_panic]
fn rem_signed_limb_ref_fail() {
    &Integer::from(10) % 0 as SignedLimb;
}

#[test]
fn test_ceiling_mod_signed_limb() {
    let test = |i, j: SignedLimb, remainder| {
        let mut n = Integer::from_str(i).unwrap();
        n.ceiling_mod_assign(j);
        assert_eq!(n.to_string(), remainder);

        let r = Integer::from_str(i).unwrap().ceiling_mod(j);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
        let r = (&Integer::from_str(i).unwrap()).ceiling_mod(j);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(
            rug::Integer::from_str(i).unwrap().rem_ceil(j).to_string(),
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
#[should_panic]
fn ceiling_mod_assign_signed_limb_fail() {
    Integer::from(10).ceiling_mod_assign(0 as SignedLimb);
}

#[test]
#[should_panic]
fn ceiling_mod_signed_limb_fail() {
    Integer::from(10).ceiling_mod(0 as SignedLimb);
}

#[test]
#[should_panic]
fn ceiling_mod_signed_limb_ref_fail() {
    (&Integer::from(10)).ceiling_mod(0 as SignedLimb);
}

#[test]
fn test_signed_limb_mod_integer() {
    let test = |i: SignedLimb, v, remainder| {
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
#[should_panic]
fn signed_limb_mod_integer_fail() {
    (10 as SignedLimb).mod_op(Integer::ZERO);
}

#[test]
#[should_panic]
fn signed_limb_mod_integer_ref_fail() {
    (10 as SignedLimb).mod_op(&Integer::ZERO);
}

#[test]
fn test_signed_limb_rem_integer() {
    let test = |i: SignedLimb, v, remainder| {
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
#[should_panic]
fn signed_limb_rem_integer_fail() {
    10 as SignedLimb % Integer::ZERO;
}

#[test]
#[allow(unused_must_use)]
#[should_panic]
fn signed_limb_rem_integer_ref_fail() {
    10 as SignedLimb % &Integer::ZERO;
}

#[test]
fn test_signed_limb_ceiling_mod_integer() {
    let test = |i: SignedLimb, v, remainder| {
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
#[should_panic]
fn signed_limb_ceiling_mod_integer_fail() {
    (10 as SignedLimb).ceiling_mod(Integer::ZERO);
}

#[test]
#[should_panic]
fn signed_limb_ceiling_mod_integer_ref_fail() {
    (10 as SignedLimb).ceiling_mod(&Integer::ZERO);
}

fn mod_signed_limb_properties_helper(n: &Integer, i: SignedLimb) {
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
        bigint_to_integer(&num_mod_signed_limb(integer_to_bigint(n), i)),
        remainder
    );
    #[cfg(feature = "32_bit_limbs")]
    assert_eq!(
        rug_integer_to_integer(&integer_to_rug_integer(n).rem_floor(i)),
        remainder
    );

    assert!(remainder < i.unsigned_abs());
    assert!(remainder == 0 as Limb || (remainder > 0 as Limb) == (i > 0));

    assert_eq!((-n).mod_op(i), -n.ceiling_mod(i));
}

#[test]
fn mod_signed_limb_properties() {
    test_properties(
        pairs_of_integer_and_nonzero_signed,
        |&(ref n, i): &(Integer, SignedLimb)| {
            mod_signed_limb_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_integer_and_nonzero_signed_limb_var_1,
        |&(ref n, i): &(Integer, SignedLimb)| {
            mod_signed_limb_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_signed_and_nonzero_integer,
        |&(i, ref n): &(SignedLimb, Integer)| {
            let remainder = i.mod_op(n);
            assert_eq!(i.mod_op(n.clone()), remainder);
            assert_eq!(i.div_mod(n).1, remainder);

            if i > 0 && i < *n {
                assert_eq!(remainder, i.unsigned_abs());
            }
            assert!(remainder.lt_abs(n));
            assert!(remainder == 0 as Limb || (remainder > 0 as Limb) == (*n > 0 as Limb));
            assert_eq!(i.mod_op(-n), i.ceiling_mod(n));
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n.mod_op(1 as SignedLimb), 0 as Limb);
        assert_eq!(n.mod_op(-1 as SignedLimb), 0 as Limb);
    });

    test_properties(nonzero_signeds, |&i: &SignedLimb| {
        assert_eq!(i.mod_op(Integer::ONE), 0 as Limb);
        assert_eq!(i.mod_op(Integer::NEGATIVE_ONE), 0 as Limb);
        assert_eq!(i.mod_op(Integer::from(i)), 0 as Limb);
        assert_eq!(Integer::from(i).mod_op(i), 0 as Limb);
        assert_eq!(i.mod_op(-Integer::from(i)), 0 as Limb);
        assert_eq!((-Integer::from(i)).mod_op(i), 0 as Limb);
        assert_eq!(Integer::ZERO.mod_op(i), 0 as Limb);
    });

    test_properties(
        triples_of_integer_integer_and_nonzero_signed::<SignedLimb>,
        |&(ref x, ref y, i)| {
            assert_eq!(
                (x + y).mod_op(i),
                Integer::from(x.mod_op(i) + Integer::from(y.mod_op(i))).mod_op(i),
            );
        },
    );

    test_properties(
        pairs_of_signed_and_nonzero_signed::<SignedLimb>,
        |&(x, y)| {
            let remainder = x.mod_op(y);
            assert_eq!(remainder, Integer::from(x).mod_op(y));
            assert_eq!(remainder, x.mod_op(Integer::from(y)));
        },
    );
}

fn rem_signed_limb_properties_helper(n: &Integer, i: SignedLimb) {
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
    #[cfg(feature = "32_bit_limbs")]
    assert_eq!(
        rug_integer_to_integer(&(integer_to_rug_integer(n) % i)),
        remainder
    );

    assert!(remainder.lt_abs(&i));
    assert!(remainder == 0 as Limb || (remainder > 0 as Limb) == (*n > 0 as Limb));
    assert_eq!(-n % i, -(n % i));
}

#[test]
fn rem_signed_limb_properties() {
    test_properties(
        pairs_of_integer_and_nonzero_signed,
        |&(ref n, i): &(Integer, SignedLimb)| {
            rem_signed_limb_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_integer_and_nonzero_signed_limb_var_1,
        |&(ref n, i): &(Integer, SignedLimb)| {
            rem_signed_limb_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_signed_and_nonzero_integer,
        |&(i, ref n): &(SignedLimb, Integer)| {
            let remainder = i % n;
            assert_eq!(i % n.clone(), remainder);

            assert_eq!(i.div_rem(n).1, remainder);

            if i > 0 && i.lt_abs(n) {
                assert_eq!(remainder, i);
            }
            assert!(remainder.lt_abs(n));
            assert!(remainder == 0 as Limb || (remainder > 0 as Limb) == (i > 0));
            assert_eq!(i % -n, remainder);
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n % 1 as SignedLimb, 0 as Limb);
        assert_eq!(n % -1 as SignedLimb, 0 as Limb);
    });

    test_properties(nonzero_signeds, |&i: &SignedLimb| {
        assert_eq!(i % Integer::ONE, 0 as Limb);
        assert_eq!(i % Integer::NEGATIVE_ONE, 0 as Limb);
        assert_eq!(i % Integer::from(i), 0 as Limb);
        assert_eq!(Integer::from(i) % i, 0 as Limb);
        assert_eq!(i % -Integer::from(i), 0 as Limb);
        assert_eq!(-Integer::from(i) % i, 0 as Limb);
        assert_eq!(Integer::ZERO % i, 0 as Limb);
        if i > 1 {
            assert_eq!(Integer::ONE % i, 1 as Limb);
            assert_eq!(Integer::NEGATIVE_ONE % i, -1 as SignedLimb);
        }
    });

    test_properties(
        triples_of_integer_integer_and_nonzero_signed::<SignedLimb>,
        |&(ref x, ref y, i)| {
            assert_eq!(x * y % i, Integer::from(x % i) * Integer::from(y % i) % i);
        },
    );

    test_properties(
        pairs_of_signed_and_nonzero_signed::<SignedLimb>,
        |&(x, y)| {
            let remainder = x % y;
            assert_eq!(remainder, Integer::from(x) % y);
            assert_eq!(remainder, x % Integer::from(y));
        },
    );
}

fn ceiling_mod_signed_limb_properties_helper(n: &Integer, i: SignedLimb) {
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

    #[cfg(feature = "32_bit_limbs")]
    assert_eq!(
        rug_integer_to_integer(&integer_to_rug_integer(n).rem_ceil(i)),
        remainder
    );

    assert!(remainder.lt_abs(&i));
    assert!(remainder == 0 as Limb || (remainder >= 0 as Limb) != (i > 0));
    assert_eq!((-n).ceiling_mod(i), -n.mod_op(i));
}

#[test]
fn ceiling_mod_signed_limb_properties() {
    test_properties(
        pairs_of_integer_and_nonzero_signed,
        |&(ref n, i): &(Integer, SignedLimb)| {
            ceiling_mod_signed_limb_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_integer_and_nonzero_signed_limb_var_1,
        |&(ref n, i): &(Integer, SignedLimb)| {
            ceiling_mod_signed_limb_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_signed_and_nonzero_integer,
        |&(i, ref n): &(SignedLimb, Integer)| {
            let remainder = i.ceiling_mod(n);
            assert!(remainder.is_valid());

            let remainder_alt = i.ceiling_mod(n.clone());
            assert!(remainder_alt.is_valid());
            assert_eq!(remainder_alt, remainder);

            assert_eq!(i.ceiling_div_mod(n).1, remainder);

            assert!(remainder.lt_abs(n));
            assert!(remainder == 0 as Limb || (remainder >= 0 as Limb) != (*n > 0 as Limb));
            assert_eq!(i.ceiling_mod(-n), i.mod_op(n));
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n.ceiling_mod(1 as SignedLimb), 0 as Limb);
        assert_eq!(n.ceiling_mod(-1 as SignedLimb), 0 as Limb);
    });

    test_properties(nonzero_signeds, |&i: &SignedLimb| {
        assert_eq!(i.ceiling_mod(Integer::ONE), 0 as Limb);
        assert_eq!(i.ceiling_mod(Integer::NEGATIVE_ONE), 0 as Limb);
        assert_eq!(i.ceiling_mod(Integer::from(i)), 0 as Limb);
        assert_eq!(Integer::from(i).ceiling_mod(i), 0 as Limb);
        assert_eq!(i.ceiling_mod(-Integer::from(i)), 0 as Limb);
        assert_eq!((-Integer::from(i)).ceiling_mod(i), 0 as Limb);
        assert_eq!(Integer::ZERO.ceiling_mod(i), 0 as Limb);
    });

    test_properties(
        triples_of_integer_integer_and_nonzero_signed::<SignedLimb>,
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

    test_properties(
        pairs_of_signed_and_nonzero_signed::<SignedLimb>,
        |&(x, y)| {
            let remainder = x.ceiling_mod(y);
            assert_eq!(remainder, Integer::from(x).ceiling_mod(y));
            assert_eq!(remainder, x.ceiling_mod(Integer::from(y)));
        },
    );
}
