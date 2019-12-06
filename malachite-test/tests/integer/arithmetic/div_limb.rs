use std::str::FromStr;

use malachite_base::num::arithmetic::traits::DivRem;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use num::BigInt;
#[cfg(feature = "32_bit_limbs")]
use rug;

use common::test_properties;
use malachite_test::common::{bigint_to_integer, integer_to_bigint};
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{pairs_of_unsigned_and_positive_unsigned, positive_unsigneds};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_positive_limb_var_1, pairs_of_integer_and_positive_unsigned,
    pairs_of_unsigned_and_nonzero_integer,
};
use malachite_test::inputs::natural::pairs_of_natural_and_positive_unsigned;

#[test]
fn test_div_limb() {
    let test = |u, v: Limb, quotient| {
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

        #[cfg(feature = "32_bit_limbs")]
        {
            let q = rug::Integer::from_str(u).unwrap() / v;
            assert_eq!(q.to_string(), quotient);
        }
    };
    test("0", 1, "0");
    test("0", 123, "0");
    test("1", 1, "1");
    test("123", 1, "123");
    test("123", 123, "1");
    test("123", 456, "0");
    test("456", 123, "3");
    test("4294967295", 1, "4294967295");
    test("4294967295", 4_294_967_295, "1");
    test("1000000000000", 1, "1000000000000");
    test("1000000000000", 3, "333333333333");
    test("1000000000000", 123, "8130081300");
    test("1000000000000", 4_294_967_295, "232");
    test("1000000000000000000000000", 1, "1000000000000000000000000");
    test("1000000000000000000000000", 3, "333333333333333333333333");
    test("1000000000000000000000000", 123, "8130081300813008130081");
    test(
        "1000000000000000000000000",
        4_294_967_295,
        "232830643708079",
    );

    test("-1", 1, "-1");
    test("-123", 1, "-123");
    test("-123", 123, "-1");
    test("-123", 456, "0");
    test("-456", 123, "-3");
    test("-4294967295", 1, "-4294967295");
    test("-4294967295", 4_294_967_295, "-1");
    test("-1000000000000", 1, "-1000000000000");
    test("-1000000000000", 3, "-333333333333");
    test("-1000000000000", 123, "-8130081300");
    test("-1000000000000", 4_294_967_295, "-232");
    test(
        "-1000000000000000000000000",
        1,
        "-1000000000000000000000000",
    );
    test("-1000000000000000000000000", 3, "-333333333333333333333333");
    test("-1000000000000000000000000", 123, "-8130081300813008130081");
    test(
        "-1000000000000000000000000",
        4_294_967_295,
        "-232830643708079",
    );
}

#[test]
#[should_panic]
fn div_assign_limb_fail() {
    let mut n = Integer::from(10);
    n /= 0 as Limb;
}

#[test]
#[allow(unused_must_use)]
#[should_panic]
fn div_limb_fail() {
    Integer::from(10) / 0 as Limb;
}

#[test]
#[allow(unused_must_use)]
#[should_panic]
fn div_limb_ref_fail() {
    &Integer::from(10) / 0 as Limb;
}

#[test]
fn test_limb_div_integer() {
    let test = |u: Limb, v, quotient| {
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
    test(4_294_967_295, "1", "4294967295");
    test(4_294_967_295, "4294967295", "1");
    test(0, "1000000000000", "0");
    test(123, "1000000000000", "0");

    test(0, "-1", "0");
    test(0, "-123", "0");
    test(1, "-1", "-1");
    test(123, "-1", "-123");
    test(123, "-123", "-1");
    test(123, "-456", "0");
    test(456, "-123", "-3");
    test(4_294_967_295, "-1", "-4294967295");
    test(4_294_967_295, "-4294967295", "-1");
    test(0, "-1000000000000", "0");
    test(123, "-1000000000000", "0");
}

#[test]
#[allow(unused_must_use)]
#[should_panic]
fn limb_div_integer_fail() {
    10 as Limb / Integer::ZERO;
}

#[test]
#[allow(unused_must_use)]
#[should_panic]
fn limb_div_integer_ref_fail() {
    10 as Limb / &Integer::ZERO;
}

fn div_limb_properties_helper(n: &Integer, u: Limb) {
    let mut mut_n = n.clone();
    mut_n /= u;
    assert!(mut_n.is_valid());
    let quotient = mut_n;

    let quotient_alt = n / u;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = n.clone() / u;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    assert_eq!(n.div_rem(u).0, quotient);

    //TODO assert_eq!(n / Natural::from(u), quotient);

    assert_eq!(bigint_to_integer(&(integer_to_bigint(n) / u)), quotient);
    #[cfg(feature = "32_bit_limbs")]
    assert_eq!(
        rug_integer_to_integer(&(integer_to_rug_integer(n) / u)),
        quotient
    );
    assert_eq!(-n / u, -&quotient);
    assert!((n - quotient * Natural::from(u)).lt_abs(&u));
}

#[test]
fn div_limb_properties() {
    test_properties(
        pairs_of_integer_and_positive_unsigned,
        |&(ref n, u): &(Integer, Limb)| {
            div_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_integer_and_positive_limb_var_1,
        |&(ref n, u): &(Integer, Limb)| {
            div_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_unsigned_and_nonzero_integer,
        |&(u, ref n): &(Limb, Integer)| {
            let quotient = u / n;
            assert!(quotient.is_valid());

            let quotient_alt = u / n.clone();
            assert!(quotient_alt.is_valid());
            assert_eq!(quotient_alt, quotient);

            assert_eq!(u.div_rem(n).0, quotient);

            assert_eq!(u / -n, -&quotient);

            assert!((u - quotient * n).lt_abs(n));
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n / 1 as Limb, *n);
    });

    test_properties(positive_unsigneds, |&u: &Limb| {
        assert_eq!(Integer::ZERO / u, 0 as Limb);
        if u > 1 {
            assert_eq!(1 / u, 0);
        }
        assert_eq!(u / Integer::ONE, u);
        assert_eq!(u / Integer::NEGATIVE_ONE, -Natural::from(u));
        assert_eq!(u / Integer::from(u), 1 as Limb);
        assert_eq!(Integer::from(u) / u, 1 as Limb);
        assert_eq!(u / -Natural::from(u), -1 as SignedLimb);
        assert_eq!(-Natural::from(u) / u, -1 as SignedLimb);
    });

    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<Limb>,
        |&(x, y)| {
            let quotient = x / y;
            assert_eq!(quotient, Integer::from(x) / y);
            assert_eq!(quotient, x / Integer::from(y));
        },
    );

    test_properties(
        pairs_of_natural_and_positive_unsigned::<Limb>,
        |&(ref n, u)| {
            assert_eq!(n / u, Integer::from(n) / u);
        },
    );
}
