use common::test_properties;
use malachite_base::num::{DivisibleBy, One, UnsignedAbs, Zero};
use malachite_nz::integer::Integer;
use malachite_test::common::{integer_to_bigint, integer_to_rug_integer};
use malachite_test::inputs::base::{nonzero_signeds, pairs_of_signeds};
use malachite_test::inputs::integer::{
    integers, nonzero_integers, pairs_of_integer_and_nonzero_i32_var_1,
    pairs_of_integer_and_nonzero_i32_var_2, pairs_of_integer_and_signed,
    pairs_of_signed_and_integer,
};
use malachite_test::integer::arithmetic::divisible_by_i32::{
    num_divisible_by_i32, rug_divisible_by_i32,
};
use num::BigInt;
use rug;
use std::str::FromStr;

#[test]
fn test_divisible_by_i32() {
    let test = |i, j: i32, divisible| {
        let n = Integer::from_str(i).unwrap();
        assert_eq!(n.divisible_by(j), divisible);
        assert_eq!(n == 0 || j != 0 && n % j == 0, divisible);

        assert_eq!(
            num_divisible_by_i32(BigInt::from_str(i).unwrap(), j),
            divisible
        );
        assert_eq!(
            rug_divisible_by_i32(rug::Integer::from_str(i).unwrap(), j),
            divisible
        );
    };
    test("0", 0, true);
    test("1", 0, false);
    test("1000000000000", 0, false);
    test("0", 1, true);
    test("0", 123, true);
    test("1", 1, true);
    test("123", 1, true);
    test("123", 123, true);
    test("123", 456, false);
    test("456", 123, false);
    test("369", 123, true);
    test("2147483647", 1, true);
    test("2147483647", 2_147_483_647, true);
    test("1000000000000", 1, true);
    test("1000000000000", 3, false);
    test("1000000000002", 3, true);
    test("1000000000000", 123, false);
    test("1000000000000", 2_147_483_647, false);
    test("1000000000000000000000000", 1, true);
    test("1000000000000000000000000", 3, false);
    test("1000000000000000000000002", 3, true);
    test("1000000000000000000000000", 123, false);
    test("1000000000000000000000000", 2_147_483_647, false);

    test("-1", 0, false);
    test("-1000000000000", 0, false);
    test("-1", 1, true);
    test("-123", 1, true);
    test("-123", 123, true);
    test("-123", 456, false);
    test("-456", 123, false);
    test("-369", 123, true);
    test("-2147483647", 1, true);
    test("-2147483647", 2_147_483_647, true);
    test("-1000000000000", 1, true);
    test("-1000000000000", 3, false);
    test("-1000000000002", 3, true);
    test("-1000000000000", 123, false);
    test("-1000000000000", 2_147_483_647, false);
    test("-1000000000000000000000000", 1, true);
    test("-1000000000000000000000000", 3, false);
    test("-1000000000000000000000002", 3, true);
    test("-1000000000000000000000000", 123, false);
    test("-1000000000000000000000000", 2_147_483_647, false);

    test("0", -1, true);
    test("0", -123, true);
    test("1", -1, true);
    test("123", -1, true);
    test("123", -123, true);
    test("123", -456, false);
    test("456", -123, false);
    test("369", -123, true);
    test("2147483647", -1, true);
    test("2147483647", -2_147_483_647, true);
    test("2147483648", -2_147_483_648, true);
    test("1000000000000", -1, true);
    test("1000000000000", -3, false);
    test("1000000000002", -3, true);
    test("1000000000000", -123, false);
    test("1000000000000", -2_147_483_647, false);
    test("1000000000000", -2_147_483_648, false);
    test("1000000000000000000000000", -1, true);
    test("1000000000000000000000000", -3, false);
    test("1000000000000000000000002", -3, true);
    test("1000000000000000000000000", -123, false);
    test("1000000000000000000000000", -2_147_483_647, false);
    test("1000000000000000000000000", -2_147_483_648, false);

    test("-1", -1, true);
    test("-123", -1, true);
    test("-123", -123, true);
    test("-123", -456, false);
    test("-456", -123, false);
    test("-369", -123, true);
    test("-2147483647", -1, true);
    test("-2147483648", -1, true);
    test("-2147483647", -2_147_483_647, true);
    test("-2147483648", -2_147_483_648, true);
    test("-1000000000000", -1, true);
    test("-1000000000000", -3, false);
    test("-1000000000002", -3, true);
    test("-1000000000000", -123, false);
    test("-1000000000000", -2_147_483_647, false);
    test("-1000000000000", -2_147_483_648, false);
    test("-1000000000000000000000000", -1, true);
    test("-1000000000000000000000000", -3, false);
    test("-1000000000000000000000002", -3, true);
    test("-1000000000000000000000000", -123, false);
    test("-1000000000000000000000000", -2_147_483_647, false);
    test("-1000000000000000000000000", -2_147_483_648, false);
}

#[test]
fn test_i32_divisible_by_integer() {
    let test = |i: i32, j, divisible| {
        let n = Integer::from_str(j).unwrap();
        assert_eq!(i.divisible_by(&n), divisible);
        assert_eq!(i == 0 || n != 0 && i % n == 0, divisible);
    };
    test(0, "0", true);
    test(1, "0", false);
    test(0, "1", true);
    test(0, "123", true);
    test(1, "1", true);
    test(123, "1", true);
    test(123, "123", true);
    test(123, "456", false);
    test(456, "123", false);
    test(369, "123", true);
    test(2_147_483_647, "1", true);
    test(2_147_483_647, "2147483647", true);
    test(0, "1000000000000", true);
    test(123, "1000000000000", false);

    test(0, "-1", true);
    test(0, "-123", true);
    test(1, "-1", true);
    test(123, "-1", true);
    test(123, "-123", true);
    test(123, "-456", false);
    test(456, "-123", false);
    test(369, "-123", true);
    test(2_147_483_647, "-1", true);
    test(2_147_483_647, "-2147483647", true);
    test(0, "-1000000000000", true);
    test(123, "-1000000000000", false);

    test(-1, "0", false);
    test(-1, "1", true);
    test(-123, "1", true);
    test(-123, "123", true);
    test(-123, "456", false);
    test(-456, "123", false);
    test(-369, "123", true);
    test(-2_147_483_647, "1", true);
    test(-2_147_483_648, "1", true);
    test(-2_147_483_647, "2147483647", true);
    test(-2_147_483_648, "2147483648", true);
    test(-0, "1000000000000", true);
    test(-123, "1000000000000", false);

    test(-1, "-1", true);
    test(-123, "-1", true);
    test(-123, "-123", true);
    test(-123, "-456", false);
    test(-456, "-123", false);
    test(-369, "-123", true);
    test(-2_147_483_647, "-1", true);
    test(-2_147_483_648, "-1", true);
    test(-2_147_483_647, "-2147483647", true);
    test(-2_147_483_648, "-2147483648", true);
    test(-123, "-1000000000000", false);
}

fn divisible_by_i32_properties_helper(n: &Integer, i: i32) {
    let divisible = n.divisible_by(i);
    assert_eq!(*n == 0 || i != 0 && n % i == 0, divisible);

    //TODO assert_eq!(n.divisible_by(Integer::from(u)), remainder);

    assert_eq!(num_divisible_by_i32(integer_to_bigint(n), i), divisible);
    assert_eq!(
        rug_divisible_by_i32(integer_to_rug_integer(n), i),
        divisible
    );

    assert_eq!((-n).divisible_by(i), divisible);
    assert_eq!(n.divisible_by(i.unsigned_abs()), divisible);
}

#[test]
fn divisible_by_i32_properties() {
    test_properties(
        pairs_of_integer_and_signed,
        |&(ref n, i): &(Integer, i32)| {
            divisible_by_i32_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_integer_and_nonzero_i32_var_1,
        |&(ref n, i): &(Integer, i32)| {
            assert!(n.divisible_by(i));
            assert!(*n == 0 || i != 0 && n % i == 0);

            //TODO assert!(n.divisible_by(Integer::from(u));

            assert!(num_divisible_by_i32(integer_to_bigint(n), i));
            assert!(rug_divisible_by_i32(integer_to_rug_integer(n), i));
        },
    );

    test_properties(
        pairs_of_integer_and_nonzero_i32_var_2,
        |&(ref n, i): &(Integer, i32)| {
            assert!(!n.divisible_by(i));
            assert!(*n != 0 && (i == 0 || n % i != 0));

            //TODO assert!(n.divisible_by(Integer::from(u));

            assert!(!num_divisible_by_i32(integer_to_bigint(n), i));
            assert!(!rug_divisible_by_i32(integer_to_rug_integer(n), i));
        },
    );

    test_properties(
        pairs_of_signed_and_integer,
        |&(i, ref n): &(i32, Integer)| {
            let divisible = i.divisible_by(n);
            assert_eq!(i == 0 || *n != 0 && i % n == 0, divisible);
            assert_eq!(i.divisible_by(&-n), divisible);
        },
    );

    test_properties(integers, |n| {
        assert!(n.divisible_by(1i32));
    });

    test_properties(nonzero_integers, |n| {
        assert!(!n.divisible_by(0i32));
    });

    test_properties(nonzero_signeds, |&i: &i32| {
        assert!(Integer::ZERO.divisible_by(i));
        if i > 1 {
            assert!(!Integer::ONE.divisible_by(i));
        }
        assert!(Integer::from(i).divisible_by(i));
        assert!((-Integer::from(i)).divisible_by(i));
        assert!(i.divisible_by(&Integer::from(i)));
        assert!(i.divisible_by(&-Integer::from(i)));
    });

    test_properties(pairs_of_signeds::<i32>, |&(x, y)| {
        let divisible = x.divisible_by(y);
        assert_eq!(divisible, Integer::from(x).divisible_by(y));
        assert_eq!(divisible, x.divisible_by(&Integer::from(y)));
    });
}