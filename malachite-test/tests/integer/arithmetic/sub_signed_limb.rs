use common::test_properties;
use malachite_base::num::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{SignedDoubleLimb, SignedLimb};
use malachite_test::common::{bigint_to_integer, integer_to_bigint};
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{pairs_of_signeds, signeds};
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_signed};
use malachite_test::integer::arithmetic::sub_signed_limb::num_sub_signed_limb;
use num::BigInt;
#[cfg(feature = "32_bit_limbs")]
use rug::{self, Assign};
use std::str::FromStr;

#[test]
fn test_sub_assign_signed_limb() {
    let test = |i, j: SignedLimb, out| {
        let mut n = Integer::from_str(i).unwrap();
        n -= j;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        #[cfg(feature = "32_bit_limbs")]
        {
            let mut n = rug::Integer::from_str(i).unwrap();
            n -= j;
            assert_eq!(n.to_string(), out);
        }

        let n = Integer::from_str(i).unwrap() - j;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_sub_signed_limb(BigInt::from_str(i).unwrap(), j);
        assert_eq!(n.to_string(), out);

        #[cfg(feature = "32_bit_limbs")]
        {
            let n = rug::Integer::from_str(i).unwrap() - j;
            assert_eq!(n.to_string(), out);
        }

        let n = &Integer::from_str(i).unwrap() - j;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = j - Integer::from_str(i).unwrap();
        assert!(n.is_valid());
        assert_eq!((-n).to_string(), out);

        #[cfg(feature = "32_bit_limbs")]
        {
            let n = j - rug::Integer::from_str(i).unwrap();
            assert_eq!((-n).to_string(), out);
        }

        let n = j - &Integer::from_str(i).unwrap();
        assert!(n.is_valid());
        assert_eq!((-n).to_string(), out);

        #[cfg(feature = "32_bit_limbs")]
        {
            let mut n = rug::Integer::from(0);
            n.assign(j - &rug::Integer::from_str(i).unwrap());
            assert_eq!((-n).to_string(), out);
        }
    };
    test("0", 0, "0");
    test("0", -123, "123");
    test("123", 0, "123");
    test("123", -456, "579");
    test("-123", -456, "333");
    test("-500", -456, "-44");
    test("123", 123, "0");
    test("456", 123, "333");
    test("123", 456, "-333");
    test("-456", 123, "-579");
    test("1000000000000", -123, "1000000000123");
    test("-1000000000000", -123, "-999999999877");
    test("1000000000000", 123, "999999999877");
    test("-1000000000000", 123, "-1000000000123");
    test("4294967295", -1, "4294967296");
    test("-4294967296", -1, "-4294967295");
    test("2147483647", -1, "2147483648");
    test("-2147483648", -1, "-2147483647");
    test("18446744073709551615", -1, "18446744073709551616");
    test("-18446744073709551616", -1, "-18446744073709551615");
    test("4294967296", 1, "4294967295");
    test("-4294967295", 1, "-4294967296");
    test("2147483648", 1, "2147483647");
    test("-2147483647", 1, "-2147483648");
    test("18446744073709551616", 1, "18446744073709551615");
    test("-18446744073709551615", 1, "-18446744073709551616");
}

#[test]
fn sub_signed_limb_properties() {
    test_properties(
        pairs_of_integer_and_signed,
        |&(ref n, i): &(Integer, SignedLimb)| {
            let mut mut_n = n.clone();
            mut_n -= i;
            assert!(mut_n.is_valid());
            let difference = mut_n;

            #[cfg(feature = "32_bit_limbs")]
            {
                let mut rug_n = integer_to_rug_integer(n);
                rug_n -= i;
                assert_eq!(rug_integer_to_integer(&rug_n), difference);
            }

            let difference_alt = n - i;
            assert!(difference_alt.is_valid());
            assert_eq!(difference_alt, difference);
            let difference_alt = n.clone() - i;
            assert!(difference_alt.is_valid());
            assert_eq!(difference_alt, difference);

            let difference_alt = i - n;
            assert!(difference_alt.is_valid());
            assert_eq!(difference_alt, -&difference);
            let difference_alt = i - n.clone();
            assert!(difference_alt.is_valid());
            assert_eq!(difference_alt, -&difference);

            assert_eq!(n - Integer::from(i), difference);
            assert_eq!(Integer::from(i) - n, -&difference);

            assert_eq!(
                bigint_to_integer(&num_sub_signed_limb(integer_to_bigint(n), i)),
                difference
            );
            #[cfg(feature = "32_bit_limbs")]
            {
                assert_eq!(
                    rug_integer_to_integer(&(integer_to_rug_integer(n) - i)),
                    difference
                );
            }

            assert_eq!(&difference + i, *n);
            assert_eq!(n - difference, i);
        },
    );

    #[allow(unknown_lints, identity_op)]
    test_properties(integers, |n| {
        assert_eq!(n + 0 as SignedLimb, *n);
        assert_eq!(0 as SignedLimb - n, -n);
    });

    test_properties(signeds, |&i: &SignedLimb| {
        assert_eq!(Integer::ZERO - i, -Integer::from(i));
        assert_eq!(i - Integer::ZERO, i);
    });

    test_properties(pairs_of_signeds::<SignedLimb>, |&(x, y)| {
        assert_eq!(
            Integer::from(SignedDoubleLimb::from(x) - SignedDoubleLimb::from(y)),
            Integer::from(x) - Integer::from(y)
        );
    });
}
