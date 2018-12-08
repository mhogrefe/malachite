use common::test_properties;
use malachite_base::num::Zero;
use malachite_nz::integer::Integer;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::{pairs_of_unsigneds, unsigneds};
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_unsigned};
use malachite_test::inputs::natural::pairs_of_natural_and_unsigned;
use malachite_test::integer::arithmetic::add_u32::num_add_u32;
use num::BigInt;
use rug::{self, Assign};
use std::str::FromStr;

#[test]
fn test_add_u32() {
    let test = |u, v: u32, out| {
        let mut n = Integer::from_str(u).unwrap();
        n += v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n += v;
        assert_eq!(n.to_string(), out);

        let n = Integer::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_add_u32(BigInt::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);

        let n = &Integer::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v + Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v + rug::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let n = v + &Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from(0);
        n.assign(v + &rug::Integer::from_str(u).unwrap());
        assert_eq!(n.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 123, "123");
    test("123", 0, "123");
    test("123", 456, "579");
    test("-123", 456, "333");
    test("-500", 456, "-44");
    test("1000000000000", 123, "1000000000123");
    test("-1000000000000", 123, "-999999999877");
    test("4294967295", 1, "4294967296");
    test("-4294967296", 1, "-4294967295");
    test("2147483647", 1, "2147483648");
    test("-2147483648", 1, "-2147483647");
    test("18446744073709551615", 1, "18446744073709551616");
    test("-18446744073709551616", 1, "-18446744073709551615");
}

#[test]
fn add_u32_properties() {
    test_properties(pairs_of_integer_and_unsigned::<u32>, |&(ref n, u)| {
        let mut mut_n = n.clone();
        mut_n += u;
        assert!(mut_n.is_valid());
        let result = mut_n;

        let mut rug_n = integer_to_rug_integer(n);
        rug_n += u;
        assert_eq!(rug_integer_to_integer(&rug_n), result);

        let result_alt = n + u;
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);
        let result_alt = n.clone() + u;
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = u + n;
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);
        let result_alt = u + n.clone();
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = n + Integer::from(u);
        assert_eq!(result_alt, result);

        let result_alt = Integer::from(u) + n;
        assert_eq!(result_alt, result);

        assert_eq!(
            bigint_to_integer(&num_add_u32(integer_to_bigint(n), u)),
            result
        );
        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(n) + u)),
            result
        );

        assert_eq!(&result - u, *n);
        assert_eq!(result - n, u);
    });

    #[allow(unknown_lints, identity_op)]
    test_properties(integers, |n| {
        assert_eq!(n + 0, *n);
        assert_eq!(0 + n, *n);
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(Integer::ZERO + u, u);
        assert_eq!(u + Integer::ZERO, u);
    });

    test_properties(pairs_of_unsigneds::<u32>, |&(x, y)| {
        let sum = Integer::from(u64::from(x) + u64::from(y));
        assert_eq!(sum, Integer::from(x) + y);
        assert_eq!(sum, x + Integer::from(y));
    });

    test_properties(pairs_of_natural_and_unsigned::<u32>, |&(ref n, u)| {
        assert_eq!(n + u, Integer::from(n) + u);
    });
}
