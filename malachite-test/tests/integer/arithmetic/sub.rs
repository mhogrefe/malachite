use common::test_properties;
use malachite_base::num::Zero;
use malachite_nz::integer::Integer;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::pairs_of_signeds;
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_signed, pairs_of_integer_and_unsigned, pairs_of_integers,
};
use malachite_test::inputs::natural::pairs_of_naturals_var_1;
use num::BigInt;
use rug;
use std::str::FromStr;

#[test]
fn test_sub() {
    let test = |u, v, out| {
        let mut n = Integer::from_str(u).unwrap();
        n -= Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Integer::from_str(u).unwrap();
        n -= &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() - Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() - Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() - &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() - &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigInt::from_str(u).unwrap() - BigInt::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() - rug::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "-123", "123");
    test("123", "0", "123");
    test("123", "-456", "579");
    test("1000000000000", "-123", "1000000000123");
    test("123", "-1000000000000", "1000000000123");
    test("12345678987654321", "-314159265358979", "12659838253013300");
    test("0", "123", "-123");
    test("123", "123", "0");
    test("123", "456", "-333");
    test("1000000000000", "123", "999999999877");
    test("123", "1000000000000", "-999999999877");
    test("12345678987654321", "314159265358979", "12031519722295342");
}

#[test]
fn sub_properties() {
    test_properties(pairs_of_integers, |&(ref x, ref y)| {
        let difference_val_val = x.clone() - y.clone();
        let difference_val_ref = x.clone() - y;
        let difference_ref_val = x - y.clone();
        let difference = x - y;
        assert!(difference_val_val.is_valid());
        assert!(difference_val_ref.is_valid());
        assert!(difference_ref_val.is_valid());
        assert!(difference.is_valid());
        assert_eq!(difference_val_val, difference);
        assert_eq!(difference_val_ref, difference);
        assert_eq!(difference_ref_val, difference);

        let mut mut_x = x.clone();
        mut_x -= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, difference);
        let mut mut_x = x.clone();
        mut_x -= y;
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, difference);

        let mut mut_x = integer_to_rug_integer(x);
        mut_x -= integer_to_rug_integer(y);
        assert_eq!(rug_integer_to_integer(&mut_x), difference);

        assert_eq!(
            bigint_to_integer(&(integer_to_bigint(x) - integer_to_bigint(y))),
            difference
        );
        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(x) - integer_to_rug_integer(y))),
            difference
        );
        assert_eq!(y - x, -&difference);
        assert_eq!(&difference + y, *x);
        assert_eq!(x - difference, *y);
    });

    test_properties(
        pairs_of_integer_and_signed,
        |&(ref x, y): &(Integer, i32)| {
            let difference = x - Integer::from(y);
            assert_eq!(x - y, difference);
            assert_eq!(y - x, -difference);
        },
    );

    #[allow(unknown_lints, eq_op)]
    test_properties(integers, |x| {
        assert_eq!(x - Integer::ZERO, *x);
        assert_eq!(Integer::ZERO - x, -x);
        assert_eq!(x - -x, x << 1);
        assert_eq!(x - x, 0)
    });

    test_properties(pairs_of_integer_and_unsigned::<u32>, |&(ref x, y)| {
        assert_eq!(x - y, x - Integer::from(y));
        assert_eq!(y - x, Integer::from(y) - x);
    });

    test_properties(pairs_of_integer_and_signed::<i32>, |&(ref x, y)| {
        assert_eq!(x - y, x - Integer::from(y));
        assert_eq!(y - x, Integer::from(y) - x);
    });

    test_properties(pairs_of_naturals_var_1, |&(ref x, ref y)| {
        assert_eq!(x + y, Integer::from(x) + Integer::from(y));
    });

    test_properties(pairs_of_signeds::<i32>, |&(x, y)| {
        assert_eq!(
            Integer::from(i64::from(x) - i64::from(y)),
            Integer::from(x) - Integer::from(y)
        );
    });
}
