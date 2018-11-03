use common::test_properties;
use malachite_base::num::Assign;
use malachite_nz::integer::Integer;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::integer::{integers, pairs_of_integers};
use num::BigInt;
use rug;
use rug::Assign as rug_assign;
use std::str::FromStr;

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
fn test_clone_clone_from_assign() {
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

        // assign Integer by value
        let mut x = Integer::from_str(u).unwrap();
        x.assign(Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
        assert!(x.is_valid());

        let mut x = rug::Integer::from_str(u).unwrap();
        x.assign(rug::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);

        // assign Integer by reference
        let mut x = Integer::from_str(u).unwrap();
        x.assign(&Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
        assert!(x.is_valid());

        let mut x = rug::Integer::from_str(u).unwrap();
        x.assign(&rug::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
    };
    test("-123", "456");
    test("-123", "1000000000000");
    test("1000000000000", "-123");
    test("1000000000000", "2000000000000");
}

#[test]
fn clone_clone_from_and_assign_properties() {
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

        let mut mut_x = x.clone();
        mut_x.assign(y.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, *y);
        let mut rug_x = integer_to_rug_integer(x);
        rug_x.assign(integer_to_rug_integer(y));
        assert_eq!(rug_integer_to_integer(&rug_x), *y);

        let mut mut_x = x.clone();
        mut_x.assign(y);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, *y);
        let mut rug_x = integer_to_rug_integer(x);
        rug_x.assign(&integer_to_rug_integer(y));
        assert_eq!(rug_integer_to_integer(&rug_x), *y);
    });
}
