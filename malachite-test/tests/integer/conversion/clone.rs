use std::str::FromStr;

use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use num::BigInt;
use rug;

use common::test_properties;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::pairs_of_signeds;
use malachite_test::inputs::integer::{integers, pairs_of_integers};

#[test]
fn test_clone() {
    let test = |u| {
        let x = Integer::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
        assert!(x.is_valid());

        let x = BigInt::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);

        let x = rug::Integer::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
    };
    test("123");
    test("1000000000000");
}

#[test]
fn test_clone_and_clone_from() {
    let test = |u, v| {
        // clone_from
        let mut x = Integer::from_str(u).unwrap();
        x.clone_from(&Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
        assert!(x.is_valid());

        let mut x = BigInt::from_str(u).unwrap();
        x.clone_from(&BigInt::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);

        let mut x = rug::Integer::from_str(u).unwrap();
        x.clone_from(&rug::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
    };
    test("-123", "456");
    test("-123", "1000000000000");
    test("1000000000000", "-123");
    test("1000000000000", "2000000000000");
}

#[test]
fn clone_and_clone_from_properties() {
    test_properties(integers, |x| {
        let mut_x = x.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, *x);

        assert_eq!(bigint_to_integer(&integer_to_bigint(x).clone()), *x);
        assert_eq!(
            rug_integer_to_integer(&integer_to_rug_integer(x).clone()),
            *x
        );
    });

    test_properties(pairs_of_integers, |&(ref x, ref y)| {
        let mut mut_x = x.clone();
        mut_x.clone_from(y);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, *y);

        let mut num_x = integer_to_bigint(x);
        num_x.clone_from(&integer_to_bigint(y));
        assert_eq!(bigint_to_integer(&num_x), *y);

        let mut rug_x = integer_to_rug_integer(x);
        rug_x.clone_from(&integer_to_rug_integer(y));
        assert_eq!(rug_integer_to_integer(&rug_x), *y);
    });

    test_properties(pairs_of_signeds::<SignedLimb>, |&(i, j)| {
        let x = Integer::from(i);
        let y = Integer::from(j);

        let mut mut_i = i;
        let mut mut_x = x.clone();
        mut_i.clone_from(&j);
        mut_x.clone_from(&y);
        assert_eq!(mut_x, mut_i);
    });
}
