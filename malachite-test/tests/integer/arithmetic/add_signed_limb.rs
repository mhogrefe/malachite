use std::str::FromStr;

use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{SignedDoubleLimb, SignedLimb};
use num::BigInt;
#[cfg(feature = "32_bit_limbs")]
use rug::{self, Assign};

use common::test_properties;
use malachite_test::common::{bigint_to_integer, integer_to_bigint};
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{pairs_of_signeds, signeds};
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_signed};
use malachite_test::integer::arithmetic::add_signed_limb::num_add_signed_limb;

#[test]
fn test_add_signed_limb() {
    let test = |u, v: SignedLimb, out| {
        let mut n = Integer::from_str(u).unwrap();
        n += v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        #[cfg(feature = "32_bit_limbs")]
        {
            let mut n = rug::Integer::from_str(u).unwrap();
            n += v;
            assert_eq!(n.to_string(), out);
        }

        let n = Integer::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_add_signed_limb(BigInt::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        #[cfg(feature = "32_bit_limbs")]
        {
            let n = rug::Integer::from_str(u).unwrap() + v;
            assert_eq!(n.to_string(), out);
        }

        let n = &Integer::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v + Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        #[cfg(feature = "32_bit_limbs")]
        {
            let n = v + rug::Integer::from_str(u).unwrap();
            assert_eq!(n.to_string(), out);
        }

        let n = v + &Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        #[cfg(feature = "32_bit_limbs")]
        {
            let mut n = rug::Integer::from(0);
            n.assign(v + &rug::Integer::from_str(u).unwrap());
            assert_eq!(n.to_string(), out);
        }
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
fn add_signed_limb_properties() {
    test_properties(
        pairs_of_integer_and_signed,
        |&(ref n, i): &(Integer, SignedLimb)| {
            let mut mut_n = n.clone();
            mut_n += i;
            let sum = mut_n;
            assert!(sum.is_valid());

            #[cfg(feature = "32_bit_limbs")]
            {
                let mut rug_n = integer_to_rug_integer(n);
                rug_n += i;
                assert_eq!(rug_integer_to_integer(&rug_n), sum);
            }

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
                bigint_to_integer(&num_add_signed_limb(integer_to_bigint(n), i)),
                sum
            );
            #[cfg(feature = "32_bit_limbs")]
            {
                assert_eq!(
                    rug_integer_to_integer(&(integer_to_rug_integer(n) + i)),
                    sum
                );
            }

            assert_eq!(&sum - i, *n);
            assert_eq!(sum - n, i);
        },
    );

    #[allow(unknown_lints, identity_op)]
    test_properties(integers, |n| {
        assert_eq!(n + 0 as SignedLimb, *n);
        assert_eq!(0 as SignedLimb + n, *n);
    });

    test_properties(signeds, |&i: &SignedLimb| {
        assert_eq!(Integer::ZERO + i, Integer::from(i));
        assert_eq!(i + Integer::ZERO, Integer::from(i));
    });

    test_properties(pairs_of_signeds::<SignedLimb>, |&(x, y)| {
        let sum = Integer::from(SignedDoubleLimb::from(x) + SignedDoubleLimb::from(y));
        assert_eq!(sum, Integer::from(x) + y);
        assert_eq!(sum, x + Integer::from(y));
    });
}
