use std::str::FromStr;

use malachite_base::num::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedDoubleLimb, SignedLimb};
use num::BigInt;
use rug;

use common::test_properties;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::pairs_of_signeds;
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_signed, pairs_of_integer_and_unsigned, pairs_of_integers,
    triples_of_integers,
};
use malachite_test::inputs::natural::pairs_of_naturals;

#[test]
fn test_add() {
    let test = |u, v, out| {
        let mut n = Integer::from_str(u).unwrap();
        n += Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Integer::from_str(u).unwrap();
        n += &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() + Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() + Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() + &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() + &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigInt::from_str(u).unwrap() + BigInt::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() + rug::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "579");
    test("1000000000000", "123", "1000000000123");
    test("123", "1000000000000", "1000000000123");
    test("12345678987654321", "314159265358979", "12659838253013300");
    test("0", "-123", "-123");
    test("123", "-123", "0");
    test("123", "-456", "-333");
    test("1000000000000", "-123", "999999999877");
    test("123", "-1000000000000", "-999999999877");
    test("12345678987654321", "-314159265358979", "12031519722295342");
}

#[test]
fn add_properties() {
    test_properties(pairs_of_integers, |&(ref x, ref y)| {
        let sum_val_val = x.clone() + y.clone();
        let sum_val_ref = x.clone() + y;
        let sum_ref_val = x + y.clone();
        let sum = x + y;
        assert!(sum_val_val.is_valid());
        assert!(sum_val_ref.is_valid());
        assert!(sum_ref_val.is_valid());
        assert!(sum.is_valid());
        assert_eq!(sum_val_val, sum);
        assert_eq!(sum_val_ref, sum);
        assert_eq!(sum_ref_val, sum);

        let mut mut_x = x.clone();
        mut_x += y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, sum);
        let mut mut_x = x.clone();
        mut_x += y;
        assert_eq!(mut_x, sum);
        assert!(mut_x.is_valid());

        let mut mut_x = integer_to_rug_integer(x);
        mut_x += integer_to_rug_integer(y);
        assert_eq!(rug_integer_to_integer(&mut_x), sum);

        assert_eq!(
            bigint_to_integer(&(integer_to_bigint(x) + integer_to_bigint(y))),
            sum
        );
        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(x) + integer_to_rug_integer(y))),
            sum
        );
        assert_eq!(y + x, sum);
        assert_eq!(&sum - x, *y);
        assert_eq!(sum - y, *x);
    });

    test_properties(
        pairs_of_integer_and_signed,
        |&(ref x, y): &(Integer, SignedLimb)| {
            let sum = x + Integer::from(y);
            assert_eq!(x + y, sum);
            assert_eq!(y + x, sum);
        },
    );

    test_properties(integers, |x| {
        assert_eq!(x + Integer::ZERO, *x);
        assert_eq!(Integer::ZERO + x, *x);
        assert_eq!(x + x, x << 1);
        assert_eq!(x + (-x), 0 as Limb)
    });

    test_properties(triples_of_integers, |&(ref x, ref y, ref z)| {
        assert_eq!((x + y) + z, x + (y + z));
    });

    test_properties(pairs_of_integer_and_unsigned::<Limb>, |&(ref x, y)| {
        let sum = x + Integer::from(y);
        assert_eq!(x + y, sum);
        assert_eq!(y + x, sum);
    });

    test_properties(pairs_of_integer_and_signed::<SignedLimb>, |&(ref x, y)| {
        let sum = x + Integer::from(y);
        assert_eq!(x + y, sum);
        assert_eq!(y + x, sum);
    });

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        assert_eq!(x + y, Integer::from(x) + Integer::from(y));
    });

    test_properties(pairs_of_signeds::<SignedLimb>, |&(x, y)| {
        assert_eq!(
            Integer::from(SignedDoubleLimb::from(x) + SignedDoubleLimb::from(y)),
            Integer::from(x) + Integer::from(y)
        );
    });
}
