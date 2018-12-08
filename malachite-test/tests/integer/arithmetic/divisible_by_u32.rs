use common::test_properties;
use malachite_base::num::{DivisibleBy, One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{integer_to_bigint, integer_to_rug_integer};
use malachite_test::inputs::base::{pairs_of_unsigneds, positive_unsigneds};
use malachite_test::inputs::integer::{
    integers, nonzero_integers, pairs_of_integer_and_positive_u32_var_1,
    pairs_of_integer_and_positive_u32_var_2, pairs_of_integer_and_unsigned,
    pairs_of_unsigned_and_integer,
};
use malachite_test::inputs::natural::pairs_of_natural_and_unsigned;
use malachite_test::integer::arithmetic::divisible_by_u32::num_divisible_by_u32;
use num::BigInt;
use rug;
use std::str::FromStr;

#[test]
fn test_divisible_by_u32() {
    let test = |u, v: u32, divisible| {
        let n = Integer::from_str(u).unwrap();
        assert_eq!(n.divisible_by(v), divisible);
        assert_eq!(n == 0 || v != 0 && n % v == 0, divisible);

        assert_eq!(
            num_divisible_by_u32(BigInt::from_str(u).unwrap(), v),
            divisible
        );
        assert_eq!(
            rug::Integer::from_str(u).unwrap().is_divisible_u(v),
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
    test("4294967295", 1, true);
    test("4294967295", 4_294_967_295, true);
    test("1000000000000", 1, true);
    test("1000000000000", 3, false);
    test("1000000000002", 3, true);
    test("1000000000000", 123, false);
    test("1000000000000", 4_294_967_295, false);
    test("1000000000000000000000000", 1, true);
    test("1000000000000000000000000", 3, false);
    test("1000000000000000000000002", 3, true);
    test("1000000000000000000000000", 123, false);
    test("1000000000000000000000000", 4_294_967_295, false);

    test("-1", 0, false);
    test("-1000000000000", 0, false);
    test("-1", 1, true);
    test("-123", 1, true);
    test("-123", 123, true);
    test("-123", 456, false);
    test("-456", 123, false);
    test("-369", 123, true);
    test("-4294967295", 1, true);
    test("-4294967295", 4_294_967_295, true);
    test("-1000000000000", 1, true);
    test("-1000000000000", 3, false);
    test("-1000000000002", 3, true);
    test("-1000000000000", 123, false);
    test("-1000000000000", 4_294_967_295, false);
    test("-1000000000000000000000000", 1, true);
    test("-1000000000000000000000000", 3, false);
    test("-1000000000000000000000002", 3, true);
    test("-1000000000000000000000000", 123, false);
    test("-1000000000000000000000000", 4_294_967_295, false);
}

#[test]
fn test_u32_divisible_by_integer() {
    let test = |u: u32, v, divisible| {
        let n = Integer::from_str(v).unwrap();
        assert_eq!(u.divisible_by(&n), divisible);
        assert_eq!(u == 0 || n != 0 && u % n == 0, divisible);
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
    test(4294967295, "1", true);
    test(4294967295, "4294967295", true);
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
    test(4294967295, "-1", true);
    test(4294967295, "-4294967295", true);
    test(0, "-1000000000000", true);
    test(123, "-1000000000000", false);
}

fn divisible_by_u32_properties_helper(n: &Integer, u: u32) {
    let divisible = n.divisible_by(u);
    assert_eq!(*n == 0 || u != 0 && n % u == 0, divisible);

    //TODO assert_eq!(n.divisible_by(Integer::from(u)), remainder);

    assert_eq!(num_divisible_by_u32(integer_to_bigint(n), u), divisible);
    assert_eq!(integer_to_rug_integer(n).is_divisible_u(u), divisible);

    assert_eq!((-n).divisible_by(u), divisible);
}

#[test]
fn divisible_by_u32_properties() {
    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref n, u): &(Integer, u32)| {
            divisible_by_u32_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_integer_and_positive_u32_var_1,
        |&(ref n, u): &(Integer, u32)| {
            assert!(n.divisible_by(u));
            assert!(*n == 0 || u != 0 && n % u == 0);

            //TODO assert!(n.divisible_by(Integer::from(u));

            assert!(num_divisible_by_u32(integer_to_bigint(n), u));
            assert!(integer_to_rug_integer(n).is_divisible_u(u));
        },
    );

    test_properties(
        pairs_of_integer_and_positive_u32_var_2,
        |&(ref n, u): &(Integer, u32)| {
            assert!(!n.divisible_by(u));
            assert!(*n != 0 && (u == 0 || n % u != 0));

            //TODO assert!(n.divisible_by(Integer::from(u));

            assert!(!num_divisible_by_u32(integer_to_bigint(n), u));
            assert!(!integer_to_rug_integer(n).is_divisible_u(u));
        },
    );

    test_properties(
        pairs_of_unsigned_and_integer,
        |&(u, ref n): &(u32, Integer)| {
            let divisible = u.divisible_by(n);
            assert_eq!(u == 0 || *n != 0 && u % n == 0, divisible);
            assert_eq!(u.divisible_by(&-n), divisible);
        },
    );

    test_properties(integers, |n| {
        assert!(n.divisible_by(1));
    });

    test_properties(nonzero_integers, |n| {
        assert!(!n.divisible_by(0));
    });

    test_properties(positive_unsigneds, |&u: &u32| {
        assert!(Integer::ZERO.divisible_by(u));
        if u > 1 {
            assert!(!Integer::ONE.divisible_by(u));
        }
        assert!(Integer::from(u).divisible_by(u));
        assert!((-Natural::from(u)).divisible_by(u));
        assert!(u.divisible_by(&Integer::from(u)));
        assert!(u.divisible_by(&-Natural::from(u)));
    });

    test_properties(pairs_of_unsigneds::<u32>, |&(x, y)| {
        let divisible = x.divisible_by(y);
        assert_eq!(divisible, Integer::from(x).divisible_by(y));
        assert_eq!(divisible, x.divisible_by(&Integer::from(y)));
    });

    test_properties(pairs_of_natural_and_unsigned::<u32>, |&(ref n, u)| {
        assert_eq!(n.divisible_by(u), Integer::from(n).divisible_by(u));
    });
}
