use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{DivisibleBy, UnsignedAbs};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use num::BigInt;
use rug;

use common::test_properties;
use malachite_test::common::{integer_to_bigint, integer_to_rug_integer};
use malachite_test::inputs::base::{nonzero_signeds, pairs_of_signeds};
use malachite_test::inputs::integer::{
    integers, nonzero_integers, pairs_of_integer_and_nonzero_signed_limb_var_1,
    pairs_of_integer_and_nonzero_signed_limb_var_2, pairs_of_integer_and_signed,
    pairs_of_signed_and_integer,
};
use malachite_test::integer::arithmetic::divisible_by_signed_limb::{
    num_divisible_by_signed_limb, rug_divisible_by_signed_limb,
};

#[test]
fn test_divisible_by_signed_limb() {
    let test = |i, j: SignedLimb, divisible| {
        let n = Integer::from_str(i).unwrap();
        assert_eq!(n.divisible_by(j), divisible);

        assert_eq!(
            num_divisible_by_signed_limb(BigInt::from_str(i).unwrap(), j),
            divisible
        );
        assert_eq!(
            rug_divisible_by_signed_limb(rug::Integer::from_str(i).unwrap(), j),
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
fn test_signed_limb_divisible_by_integer() {
    let test = |i: SignedLimb, j, divisible| {
        let n = Integer::from_str(j).unwrap();
        assert_eq!(i.divisible_by(&n), divisible);
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

fn divisible_by_signed_limb_properties_helper(n: &Integer, i: SignedLimb) {
    let divisible = n.divisible_by(i);

    //TODO assert_eq!(n.divisible_by(Integer::from(u)), remainder);

    assert_eq!(
        num_divisible_by_signed_limb(integer_to_bigint(n), i),
        divisible
    );
    assert_eq!(
        rug_divisible_by_signed_limb(integer_to_rug_integer(n), i),
        divisible
    );

    assert_eq!((-n).divisible_by(i), divisible);
    assert_eq!(n.divisible_by(i.unsigned_abs()), divisible);
}

#[test]
fn divisible_by_signed_limb_properties() {
    test_properties(
        pairs_of_integer_and_signed,
        |&(ref n, i): &(Integer, SignedLimb)| {
            divisible_by_signed_limb_properties_helper(n, i);
        },
    );

    test_properties(
        pairs_of_integer_and_nonzero_signed_limb_var_1,
        |&(ref n, i): &(Integer, SignedLimb)| {
            assert!(n.divisible_by(i));

            //TODO assert!(n.divisible_by(Integer::from(u));

            assert!(num_divisible_by_signed_limb(integer_to_bigint(n), i));
            assert!(rug_divisible_by_signed_limb(integer_to_rug_integer(n), i));
        },
    );

    test_properties(
        pairs_of_integer_and_nonzero_signed_limb_var_2,
        |&(ref n, i): &(Integer, SignedLimb)| {
            assert!(!n.divisible_by(i));

            //TODO assert!(n.divisible_by(Integer::from(u));

            assert!(!num_divisible_by_signed_limb(integer_to_bigint(n), i));
            assert!(!rug_divisible_by_signed_limb(integer_to_rug_integer(n), i));
        },
    );

    test_properties(
        pairs_of_signed_and_integer,
        |&(i, ref n): &(SignedLimb, Integer)| {
            let divisible = i.divisible_by(n);
            assert_eq!(i.divisible_by(&-n), divisible);
        },
    );

    test_properties(integers, |n| {
        assert!(n.divisible_by(1 as SignedLimb));
    });

    test_properties(nonzero_integers, |n| {
        assert!(!n.divisible_by(0 as SignedLimb));
    });

    test_properties(nonzero_signeds, |&i: &SignedLimb| {
        assert!(Integer::ZERO.divisible_by(i));
        if i > 1 {
            assert!(!Integer::ONE.divisible_by(i));
        }
        assert!(Integer::from(i).divisible_by(i));
        assert!((-Integer::from(i)).divisible_by(i));
        assert!(i.divisible_by(&Integer::from(i)));
        assert!(i.divisible_by(&-Integer::from(i)));
    });

    test_properties(pairs_of_signeds::<SignedLimb>, |&(x, y)| {
        let divisible = x.divisible_by(y);
        assert_eq!(divisible, Integer::from(x).divisible_by(y));
        assert_eq!(divisible, x.divisible_by(&Integer::from(y)));
    });
}
