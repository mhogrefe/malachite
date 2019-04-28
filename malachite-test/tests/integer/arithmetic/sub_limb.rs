use std::str::FromStr;

use malachite_base::num::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;
use num::BigInt;
#[cfg(feature = "32_bit_limbs")]
use rug::{self, Assign};

use common::test_properties;
use malachite_test::common::{bigint_to_integer, integer_to_bigint};
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{pairs_of_unsigneds_var_1, unsigneds};
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_unsigned};
use malachite_test::inputs::natural::{
    pairs_of_limb_and_natural_var_1, pairs_of_natural_and_limb_var_1,
};
use malachite_test::integer::arithmetic::sub_limb::num_sub_limb;

#[test]
fn test_sub_assign_limb() {
    let test = |u, v: Limb, out| {
        let mut n = Integer::from_str(u).unwrap();
        n -= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        #[cfg(feature = "32_bit_limbs")]
        {
            let mut n = rug::Integer::from_str(u).unwrap();
            n -= v;
            assert_eq!(n.to_string(), out);
        }

        let n = Integer::from_str(u).unwrap() - v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_sub_limb(BigInt::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        #[cfg(feature = "32_bit_limbs")]
        {
            let n = rug::Integer::from_str(u).unwrap() - v;
            assert_eq!(n.to_string(), out);
        }

        let n = &Integer::from_str(u).unwrap() - v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v - Integer::from_str(u).unwrap();
        assert!(n.is_valid());
        assert_eq!((-n).to_string(), out);

        #[cfg(feature = "32_bit_limbs")]
        {
            let n = v - rug::Integer::from_str(u).unwrap();
            assert_eq!((-n).to_string(), out);
        }

        let n = v - &Integer::from_str(u).unwrap();
        assert!(n.is_valid());
        assert_eq!((-n).to_string(), out);

        #[cfg(feature = "32_bit_limbs")]
        {
            let mut n = rug::Integer::from(0);
            n.assign(v - &rug::Integer::from_str(u).unwrap());
            assert_eq!((-n).to_string(), out);
        }
    };
    test("0", 0, "0");
    test("123", 123, "0");
    test("123", 0, "123");
    test("456", 123, "333");
    test("123", 456, "-333");
    test("-456", 123, "-579");
    test("1000000000000", 123, "999999999877");
    test("-1000000000000", 123, "-1000000000123");
    test("4294967296", 1, "4294967295");
    test("-4294967295", 1, "-4294967296");
    test("2147483648", 1, "2147483647");
    test("-2147483647", 1, "-2147483648");
    test("18446744073709551616", 1, "18446744073709551615");
    test("-18446744073709551615", 1, "-18446744073709551616");
}

#[test]
fn sub_limb_properties() {
    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref n, u): &(Integer, Limb)| {
            let mut mut_n = n.clone();
            mut_n -= u;
            assert!(mut_n.is_valid());
            let difference = mut_n;

            #[cfg(feature = "32_bit_limbs")]
            {
                let mut rug_n = integer_to_rug_integer(n);
                rug_n -= u;
                assert_eq!(rug_integer_to_integer(&rug_n), difference);
            }

            let difference_alt = n - u;
            assert!(difference_alt.is_valid());
            assert_eq!(difference_alt, difference);
            let difference_alt = n.clone() - u;
            assert!(difference_alt.is_valid());
            assert_eq!(difference_alt, difference);

            let difference_alt = u - n;
            assert!(difference_alt.is_valid());
            assert_eq!(difference_alt, -&difference);
            let difference_alt = u - n.clone();
            assert_eq!(difference_alt, -&difference);
            assert!(difference_alt.is_valid());

            assert_eq!(n - Integer::from(u), difference);
            assert_eq!(Integer::from(u) - n, -&difference);

            assert_eq!(
                bigint_to_integer(&num_sub_limb(integer_to_bigint(n), u)),
                difference
            );
            #[cfg(feature = "32_bit_limbs")]
            {
                assert_eq!(
                    rug_integer_to_integer(&(integer_to_rug_integer(n) - u)),
                    difference
                );
            }

            assert_eq!(&difference + u, *n);
            assert_eq!(n - difference, u);
        },
    );

    #[allow(unknown_lints, identity_op)]
    test_properties(integers, |n| {
        assert_eq!(n + 0 as Limb, *n);
        assert_eq!(0 as Limb - n, -n);
    });

    test_properties(unsigneds, |&u: &Limb| {
        assert_eq!(Integer::ZERO - u, -Integer::from(u));
        assert_eq!(u - Integer::ZERO, u);
    });

    test_properties(pairs_of_unsigneds_var_1::<Limb>, |&(x, y)| {
        let difference = x - y;
        assert_eq!(difference, Integer::from(x) - y);
        assert_eq!(difference, x - Integer::from(y));
    });

    test_properties(pairs_of_natural_and_limb_var_1, |&(ref n, u)| {
        assert_eq!(n - u, Integer::from(n) - u);
    });

    test_properties(pairs_of_limb_and_natural_var_1, |&(u, ref n)| {
        assert_eq!(u - n, u - Integer::from(n));
    });
}
