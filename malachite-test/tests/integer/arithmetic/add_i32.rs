use common::test_properties;
use malachite_base::num::Zero;
use malachite_nz::integer::Integer;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::{pairs_of_signeds, signeds};
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_signed};
use malachite_test::integer::arithmetic::add_i32::num_add_i32;
use num::BigInt;
use rug::{self, Assign};
use std::str::FromStr;

#[test]
fn test_add_i32() {
    let test = |u, v: i32, out| {
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

        let n = num_add_i32(BigInt::from_str(u).unwrap(), v);
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
    test("123", -123, "0");
    test("456", -123, "333");
    test("123", -456, "-333");
    test("-456", -123, "-579");
    test("1000000000000", 123, "1000000000123");
    test("-1000000000000", 123, "-999999999877");
    test("1000000000000", -123, "999999999877");
    test("-1000000000000", -123, "-1000000000123");
    test("4294967295", 1, "4294967296");
    test("-4294967296", 1, "-4294967295");
    test("2147483647", 1, "2147483648");
    test("-2147483648", 1, "-2147483647");
    test("18446744073709551615", 1, "18446744073709551616");
    test("-18446744073709551616", 1, "-18446744073709551615");
    test("4294967296", -1, "4294967295");
    test("-4294967295", -1, "-4294967296");
    test("2147483648", -1, "2147483647");
    test("-2147483647", -1, "-2147483648");
    test("18446744073709551616", -1, "18446744073709551615");
    test("-18446744073709551615", -1, "-18446744073709551616");
}

#[test]
fn add_i32_properties() {
    test_properties(
        pairs_of_integer_and_signed,
        |&(ref n, i): &(Integer, i32)| {
            let mut mut_n = n.clone();
            mut_n += i;
            let sum = mut_n;
            assert!(sum.is_valid());

            let mut rug_n = integer_to_rug_integer(n);
            rug_n += i;
            assert_eq!(rug_integer_to_integer(&rug_n), sum);

            let result = n + i;
            assert!(result.is_valid());
            assert_eq!(result, sum);
            let result = n.clone() + i;
            assert!(result.is_valid());
            assert_eq!(result, sum);

            let result = i + n;
            assert!(result.is_valid());
            assert_eq!(result, sum);
            let result = i + n.clone();
            assert_eq!(result, sum);
            assert!(result.is_valid());

            assert_eq!(n + Integer::from(i), sum);
            assert_eq!(Integer::from(i) + n, sum);

            assert_eq!(
                bigint_to_integer(&num_add_i32(integer_to_bigint(n), i)),
                sum
            );
            assert_eq!(
                rug_integer_to_integer(&(integer_to_rug_integer(n) + i)),
                sum
            );

            assert_eq!(&sum - i, *n);
            assert_eq!(sum - n, i);
        },
    );

    #[allow(unknown_lints, identity_op)]
    test_properties(integers, |n| {
        assert_eq!(n + 0i32, *n);
        assert_eq!(0i32 + n, *n);
    });

    test_properties(signeds, |&i: &i32| {
        assert_eq!(Integer::ZERO + i, Integer::from(i));
        assert_eq!(i + Integer::ZERO, Integer::from(i));
    });

    test_properties(pairs_of_signeds::<i32>, |&(x, y)| {
        let sum = Integer::from(i64::from(x) + i64::from(y));
        assert_eq!(sum, Integer::from(x) + y);
        assert_eq!(sum, x + Integer::from(y));
    });
}
