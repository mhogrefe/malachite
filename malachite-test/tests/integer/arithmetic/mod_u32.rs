use common::test_properties;
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{
    CeilingDivMod, CeilingMod, CeilingModAssign, DivMod, DivRem, Mod, ModAssign, NegativeOne, One,
    PartialOrdAbs, Zero,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::{pairs_of_unsigned_and_positive_unsigned, positive_unsigneds};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_positive_u32_var_1, pairs_of_integer_and_positive_unsigned,
    pairs_of_unsigned_and_nonzero_integer, triples_of_integer_integer_and_positive_unsigned,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_positive_unsigned, pairs_of_unsigned_and_positive_natural,
};
use malachite_test::integer::arithmetic::mod_u32::num_mod_u32;
use num::BigInt;
use rug;
use rug::ops::RemRounding;
use std::str::FromStr;

#[test]
fn test_mod_u32() {
    let test = |u, v: u32, remainder| {
        let mut n = Integer::from_str(u).unwrap();
        n.mod_assign(v);
        assert!(n.is_valid());
        assert_eq!(n, remainder);

        assert_eq!(Integer::from_str(u).unwrap().mod_op(v), remainder);
        assert_eq!((&Integer::from_str(u).unwrap()).mod_op(v), remainder);

        assert_eq!(num_mod_u32(BigInt::from_str(u).unwrap(), v), remainder);
        assert_eq!(rug::Integer::from_str(u).unwrap().mod_u(v), remainder);
    };
    test("0", 1, 0);
    test("0", 123, 0);
    test("1", 1, 0);
    test("123", 1, 0);
    test("123", 123, 0);
    test("123", 456, 123);
    test("456", 123, 87);
    test("4294967295", 1, 0);
    test("4294967295", 4_294_967_295, 0);
    test("1000000000000", 1, 0);
    test("1000000000000", 3, 1);
    test("1000000000000", 123, 100);
    test("1000000000000", 4_294_967_295, 3_567_587_560);
    test("1000000000000000000000000", 1, 0);
    test("1000000000000000000000000", 3, 1);
    test("1000000000000000000000000", 123, 37);
    test("1000000000000000000000000", 4_294_967_295, 3_167_723_695);

    test("-1", 1, 0);
    test("-123", 1, 0);
    test("-123", 123, 0);
    test("-123", 456, 333);
    test("-456", 123, 36);
    test("-4294967295", 1, 0);
    test("-4294967295", 4_294_967_295, 0);
    test("-1000000000000", 1, 0);
    test("-1000000000000", 3, 2);
    test("-1000000000000", 123, 23);
    test("-1000000000000", 4_294_967_295, 727_379_735);
    test("-1000000000000000000000000", 1, 0);
    test("-1000000000000000000000000", 3, 2);
    test("-1000000000000000000000000", 123, 86);
    test("-1000000000000000000000000", 4_294_967_295, 1_127_243_600);
}

#[test]
#[should_panic(expected = "division by zero")]
fn mod_assign_u32_fail() {
    Integer::from(10u32).mod_assign(0u32);
}

#[test]
#[should_panic(expected = "division by zero")]
fn mod_u32_fail() {
    Integer::from(10u32).mod_op(0u32);
}

#[test]
#[should_panic(expected = "division by zero")]
fn mod_u32_ref_fail() {
    (&Integer::from(10u32)).mod_op(0u32);
}

#[test]
fn test_rem_u32() {
    let test = |u, v: u32, remainder| {
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
    test("4294967295", 1, "0");
    test("4294967295", 4_294_967_295, "0");
    test("1000000000000", 1, "0");
    test("1000000000000", 3, "1");
    test("1000000000000", 123, "100");
    test("1000000000000", 4_294_967_295, "3567587560");
    test("1000000000000000000000000", 1, "0");
    test("1000000000000000000000000", 3, "1");
    test("1000000000000000000000000", 123, "37");
    test("1000000000000000000000000", 4_294_967_295, "3167723695");

    test("-1", 1, "0");
    test("-123", 1, "0");
    test("-123", 123, "0");
    test("-123", 456, "-123");
    test("-456", 123, "-87");
    test("-4294967295", 1, "0");
    test("-4294967295", 4_294_967_295, "0");
    test("-1000000000000", 1, "0");
    test("-1000000000000", 3, "-1");
    test("-1000000000000", 123, "-100");
    test("-1000000000000", 4_294_967_295, "-3567587560");
    test("-1000000000000000000000000", 1, "0");
    test("-1000000000000000000000000", 3, "-1");
    test("-1000000000000000000000000", 123, "-37");
    test("-1000000000000000000000000", 4_294_967_295, "-3167723695");
}

#[test]
#[should_panic(expected = "division by zero")]
fn rem_assign_u32_fail() {
    let mut n = Integer::from(10u32);
    n %= 0u32;
}

#[test]
#[allow(unused_must_use)]
#[should_panic(expected = "division by zero")]
fn rem_u32_fail() {
    Integer::from(10u32) % 0u32;
}

#[test]
#[allow(unused_must_use)]
#[should_panic(expected = "division by zero")]
fn rem_u32_ref_fail() {
    &Integer::from(10u32) % 0u32;
}

#[test]
fn test_ceiling_mod_u32() {
    let test = |u, v: u32, remainder| {
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
    test("4294967295", 1, "0");
    test("4294967295", 4_294_967_295, "0");
    test("1000000000000", 1, "0");
    test("1000000000000", 3, "-2");
    test("1000000000000", 123, "-23");
    test("1000000000000", 4_294_967_295, "-727379735");
    test("1000000000000000000000000", 1, "0");
    test("1000000000000000000000000", 3, "-2");
    test("1000000000000000000000000", 123, "-86");
    test("1000000000000000000000000", 4_294_967_295, "-1127243600");

    test("-1", 1, "0");
    test("-123", 1, "0");
    test("-123", 123, "0");
    test("-123", 456, "-123");
    test("-456", 123, "-87");
    test("-4294967295", 1, "0");
    test("-4294967295", 4_294_967_295, "0");
    test("-1000000000000", 1, "0");
    test("-1000000000000", 3, "-1");
    test("-1000000000000", 123, "-100");
    test("-1000000000000", 4_294_967_295, "-3567587560");
    test("-1000000000000000000000000", 1, "0");
    test("-1000000000000000000000000", 3, "-1");
    test("-1000000000000000000000000", 123, "-37");
    test("-1000000000000000000000000", 4_294_967_295, "-3167723695");
}

#[test]
#[should_panic(expected = "division by zero")]
fn ceiling_mod_assign_u32_fail() {
    Integer::from(10u32).ceiling_mod_assign(0u32);
}

#[test]
#[should_panic(expected = "division by zero")]
fn ceiling_mod_u32_fail() {
    Integer::from(10u32).ceiling_mod(0u32);
}

#[test]
#[should_panic(expected = "division by zero")]
fn ceiling_mod_u32_ref_fail() {
    (&Integer::from(10u32)).ceiling_mod(0u32);
}

#[test]
fn test_u32_mod_integer() {
    let test = |u: u32, v, remainder| {
        let remainder_alt = u.mod_op(Integer::from_str(v).unwrap());
        assert!(remainder_alt.is_valid());
        assert_eq!(remainder_alt.to_string(), remainder);

        let remainder_alt = u.mod_op(&Integer::from_str(v).unwrap());
        assert!(remainder_alt.is_valid());
        assert_eq!(remainder_alt.to_string(), remainder);
    };
    test(0, "1", "0");
    test(0, "123", "0");
    test(1, "1", "0");
    test(123, "1", "0");
    test(123, "123", "0");
    test(123, "456", "123");
    test(456, "123", "87");
    test(4_294_967_295, "1", "0");
    test(4_294_967_295, "4294967295", "0");
    test(0, "1000000000000", "0");
    test(123, "1000000000000", "123");

    test(0, "-1", "0");
    test(0, "-123", "0");
    test(1, "-1", "0");
    test(123, "-1", "0");
    test(123, "-123", "0");
    test(123, "-456", "-333");
    test(456, "-123", "-36");
    test(4_294_967_295, "-1", "0");
    test(4_294_967_295, "-4294967295", "0");
    test(0, "-1000000000000", "0");
    test(123, "-1000000000000", "-999999999877");
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_mod_integer_fail() {
    10u32.mod_op(Integer::ZERO);
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_mod_integer_ref_fail() {
    10u32.mod_op(&Integer::ZERO);
}

#[test]
fn test_u32_rem_integer() {
    let test = |u: u32, v, remainder| {
        assert_eq!(u % Integer::from_str(v).unwrap(), remainder);
        assert_eq!(u % &Integer::from_str(v).unwrap(), remainder);
    };
    test(0, "1", 0);
    test(0, "123", 0);
    test(1, "1", 0);
    test(123, "1", 0);
    test(123, "123", 0);
    test(123, "456", 123);
    test(456, "123", 87);
    test(4_294_967_295, "1", 0);
    test(4_294_967_295, "4294967295", 0);
    test(0, "1000000000000", 0);
    test(123, "1000000000000", 123);

    test(0, "-1", 0);
    test(0, "-123", 0);
    test(1, "-1", 0);
    test(123, "-1", 0);
    test(123, "-123", 0);
    test(123, "-456", 123);
    test(456, "-123", 87);
    test(4_294_967_295, "-1", 0);
    test(4_294_967_295, "-4294967295", 0);
    test(0, "-1000000000000", 0);
    test(123, "-1000000000000", 123);
}

#[test]
#[allow(unused_must_use)]
#[should_panic(expected = "division by zero")]
fn u32_rem_integer_fail() {
    10u32 % Integer::ZERO;
}

#[test]
#[allow(unused_must_use)]
#[should_panic(expected = "division by zero")]
fn u32_rem_integer_ref_fail() {
    10u32 % &Integer::ZERO;
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_rem_assign_integer_fail() {
    let mut n = 10u32;
    n %= Integer::ZERO;
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_rem_assign_integer_ref_fail() {
    let mut n = 10u32;
    n %= &Integer::ZERO;
}

#[test]
fn test_u32_ceiling_mod_integer() {
    let test = |u: u32, v, remainder| {
        let n = u.ceiling_mod(Integer::from_str(v).unwrap());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), remainder);

        let n = u.ceiling_mod(&Integer::from_str(v).unwrap());
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
    test(4_294_967_295, "1", "0");
    test(4_294_967_295, "4294967295", "0");
    test(0, "1000000000000", "0");
    test(123, "1000000000000", "-999999999877");

    test(0, "-1", "0");
    test(0, "-123", "0");
    test(1, "-1", "0");
    test(123, "-1", "0");
    test(123, "-123", "0");
    test(123, "-456", "123");
    test(456, "-123", "87");
    test(4_294_967_295, "-1", "0");
    test(4_294_967_295, "-4294967295", "0");
    test(0, "-1000000000000", "0");
    test(123, "-1000000000000", "123");
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_ceiling_mod_integer_fail() {
    10u32.ceiling_mod(Integer::ZERO);
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_ceiling_mod_integer_ref_fail() {
    10u32.ceiling_mod(&Integer::ZERO);
}

fn mod_u32_properties_helper(n: &Integer, u: u32) {
    let mut mut_n = n.clone();
    mut_n.mod_assign(u);
    assert!(mut_n.is_valid());
    let remainder = u32::checked_from(mut_n).unwrap();

    assert_eq!(n.mod_op(u), remainder);
    assert_eq!(n.clone().mod_op(u), remainder);

    assert_eq!(n.div_mod(u).1, remainder);

    //TODO assert_eq!(n.mod_op(Integer::from(u)), remainder);

    assert_eq!(num_mod_u32(integer_to_bigint(n), u), remainder);
    assert_eq!(integer_to_rug_integer(n).mod_u(u), remainder);

    assert!(remainder < u);
    assert_eq!((-n).mod_op(u), -n.ceiling_mod(u));
}

#[test]
fn mod_u32_properties() {
    test_properties(
        pairs_of_integer_and_positive_unsigned,
        |&(ref n, u): &(Integer, u32)| {
            mod_u32_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_integer_and_positive_u32_var_1,
        |&(ref n, u): &(Integer, u32)| {
            mod_u32_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_unsigned_and_nonzero_integer,
        |&(u, ref n): &(u32, Integer)| {
            let remainder = u.mod_op(n);
            assert!(remainder.is_valid());

            let remainder_alt = u.mod_op(n.clone());
            assert!(remainder_alt.is_valid());
            assert_eq!(remainder_alt, remainder);

            assert_eq!(u.div_mod(n).1, remainder);

            if u != 0 && u < *n {
                assert_eq!(remainder, u);
            }
            assert!(remainder.lt_abs(n));
            assert!(remainder == 0 || (remainder > 0) == (*n > 0));
            assert_eq!(u.mod_op(-n), u.ceiling_mod(n));
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n.mod_op(1u32), 0);
    });

    test_properties(positive_unsigneds, |&u: &u32| {
        assert_eq!(u.mod_op(Integer::ONE), 0);
        assert_eq!(u.mod_op(Integer::NEGATIVE_ONE), 0);
        assert_eq!(u.mod_op(Integer::from(u)), 0);
        assert_eq!(Integer::from(u).mod_op(u), 0);
        assert_eq!(u.mod_op(-Natural::from(u)), 0);
        assert_eq!((-Natural::from(u)).mod_op(u), 0);
        assert_eq!(Integer::ZERO.mod_op(u), 0);
        if u > 1 {
            assert_eq!(Integer::ONE.mod_op(u), 1);
            assert_eq!(Integer::NEGATIVE_ONE.mod_op(u), u - 1);
        }
    });

    test_properties(
        triples_of_integer_integer_and_positive_unsigned::<u32>,
        |&(ref x, ref y, u)| {
            assert_eq!(
                (x + y).mod_op(u),
                (Integer::from(x.mod_op(u)) + Integer::from(y.mod_op(u))).mod_op(u),
            );
        },
    );

    test_properties(pairs_of_unsigned_and_positive_unsigned::<u32>, |&(x, y)| {
        let remainder = x.mod_op(y);
        assert_eq!(remainder, Integer::from(x).mod_op(y));
        assert_eq!(remainder, x.mod_op(Integer::from(y)));
    });

    test_properties(
        pairs_of_natural_and_positive_unsigned::<u32>,
        |&(ref n, u)| {
            assert_eq!(n.mod_op(u), Integer::from(n).mod_op(u));
        },
    );

    test_properties(
        pairs_of_unsigned_and_positive_natural::<u32>,
        |&(u, ref n)| {
            assert_eq!(u.mod_op(n), u.mod_op(Integer::from(n)));
        },
    );
}

fn rem_u32_properties_helper(n: &Integer, u: u32) {
    let mut mut_n = n.clone();
    mut_n %= u;
    assert!(mut_n.is_valid());
    let remainder = mut_n;

    let remainder_alt = n % u;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = n.clone() % u;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    assert_eq!(n.div_rem(u).1, remainder);

    //TODO assert_eq!(n % Integer::from(u), remainder);

    assert_eq!(bigint_to_integer(&(integer_to_bigint(n) % u)), remainder);
    assert_eq!(
        rug_integer_to_integer(&(integer_to_rug_integer(n) % u)),
        remainder
    );

    assert!(remainder.lt_abs(&u));
    assert!(remainder == 0 || (remainder > 0) == (*n > 0));
    assert_eq!(-n % u, -(n % u));
}

#[test]
fn rem_u32_properties() {
    test_properties(
        pairs_of_integer_and_positive_unsigned,
        |&(ref n, u): &(Integer, u32)| {
            rem_u32_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_integer_and_positive_u32_var_1,
        |&(ref n, u): &(Integer, u32)| {
            rem_u32_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_unsigned_and_nonzero_integer,
        |&(u, ref n): &(u32, Integer)| {
            let remainder = u % n;
            assert_eq!(u % n.clone(), remainder);

            assert_eq!(u.div_rem(n).1, remainder);

            if u > 0 && u.lt_abs(n) {
                assert_eq!(remainder, u);
            }
            assert!(remainder.lt_abs(n));

            assert_eq!(u % -n, remainder);
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n % 1u32, 0);
    });

    test_properties(positive_unsigneds, |&u: &u32| {
        assert_eq!(u % Integer::ONE, 0);
        assert_eq!(u % Integer::NEGATIVE_ONE, 0);
        assert_eq!(u % Integer::from(u), 0);
        assert_eq!(Integer::from(u) % u, 0);
        assert_eq!(u % -Natural::from(u), 0);
        assert_eq!(-Natural::from(u) % u, 0);
        assert_eq!(Integer::ZERO % u, 0);
        if u > 1 {
            assert_eq!(Integer::ONE % u, 1);
            assert_eq!(Integer::NEGATIVE_ONE % u, -1);
        }
    });

    test_properties(
        triples_of_integer_integer_and_positive_unsigned::<u32>,
        |&(ref x, ref y, u)| {
            assert_eq!(x * y % u, Integer::from(x % u) * Integer::from(y % u) % u);
        },
    );
}

fn ceiling_mod_u32_properties_helper(n: &Integer, u: u32) {
    let mut mut_n = n.clone();
    mut_n.ceiling_mod_assign(u);
    assert!(mut_n.is_valid());
    let remainder = mut_n;

    let remainder_alt = n.ceiling_mod(u);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = n.clone().ceiling_mod(u);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    assert_eq!(n.ceiling_div_mod(u).1, remainder);

    //TODO assert_eq!(n.ceiling_mod(Integer::from(u)), remainder);

    assert_eq!(
        rug_integer_to_integer(&integer_to_rug_integer(n).rem_ceil(u)),
        remainder
    );

    assert!(remainder <= 0);
    assert!(-&remainder < u);
    assert_eq!((-n).ceiling_mod(u), -Natural::from(n.mod_op(u)));
}

#[test]
fn ceiling_mod_u32_properties() {
    test_properties(
        pairs_of_integer_and_positive_unsigned,
        |&(ref n, u): &(Integer, u32)| {
            ceiling_mod_u32_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_integer_and_positive_u32_var_1,
        |&(ref n, u): &(Integer, u32)| {
            ceiling_mod_u32_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_unsigned_and_nonzero_integer,
        |&(u, ref n): &(u32, Integer)| {
            let remainder = u.ceiling_mod(n);
            assert!(remainder.is_valid());

            let remainder_alt = u.ceiling_mod(n.clone());
            assert!(remainder_alt.is_valid());
            assert_eq!(remainder_alt, remainder);

            assert_eq!(u.ceiling_div_mod(n).1, remainder);

            if u != 0 && u < *n {
                assert_eq!(remainder, u - n);
            }
            assert!(remainder.lt_abs(n));
            assert!(remainder == 0 || (remainder > 0) != (*n > 0));
            assert_eq!(u.ceiling_mod(-n), u.mod_op(n));
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n.ceiling_mod(1u32), 0);
    });

    test_properties(positive_unsigneds, |&u: &u32| {
        assert_eq!(u.ceiling_mod(Integer::ONE), 0);
        assert_eq!(u.ceiling_mod(Integer::NEGATIVE_ONE), 0);
        assert_eq!(u.ceiling_mod(Integer::from(u)), 0);
        assert_eq!(Integer::from(u).ceiling_mod(u), 0);
        assert_eq!(u.ceiling_mod(-Natural::from(u)), 0);
        assert_eq!((-Natural::from(u)).ceiling_mod(u), 0);
        assert_eq!(Integer::ZERO.ceiling_mod(u), 0);
        assert_eq!(-Integer::ONE.ceiling_mod(u), u - 1);
    });

    test_properties(
        triples_of_integer_integer_and_positive_unsigned::<u32>,
        |&(ref x, ref y, u)| {
            assert_eq!(
                (x + y).ceiling_mod(u),
                (Integer::from(x.mod_op(u)) + Integer::from(y.mod_op(u))).ceiling_mod(u)
            );
            assert_eq!(
                (x * y).ceiling_mod(u),
                (Integer::from(x % u) * Integer::from(y % u)).ceiling_mod(u)
            );
        },
    );

    test_properties(pairs_of_unsigned_and_positive_unsigned::<u32>, |&(x, y)| {
        let remainder = x % y;
        assert_eq!(remainder, Integer::from(x) % y);
        assert_eq!(remainder, x % Integer::from(y));
    });

    test_properties(
        pairs_of_natural_and_positive_unsigned::<u32>,
        |&(ref n, u)| {
            assert_eq!(n % u, Integer::from(n) % u);
        },
    );

    test_properties(
        pairs_of_unsigned_and_positive_natural::<u32>,
        |&(u, ref n)| {
            assert_eq!(u % n, u % Integer::from(n));
        },
    );
}
