use common::test_properties;
use malachite_base::num::Zero;
use malachite_nz::integer::Integer;
use malachite_test::common::{bigint_to_integer, integer_to_bigint, integer_to_rug_integer,
                             rug_integer_to_integer};
use malachite_test::integer::arithmetic::sub_u32::num_sub_u32;
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_unsigned};
use num::BigInt;
use rug::{self, Assign};
use std::str::FromStr;

#[test]
fn test_sub_assign_u32() {
    let test = |u, v: u32, out| {
        let mut n = Integer::from_str(u).unwrap();
        n -= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n -= v;
        assert_eq!(n.to_string(), out);

        let n = Integer::from_str(u).unwrap() - v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_sub_u32(BigInt::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() - v;
        assert_eq!(n.to_string(), out);

        let n = &Integer::from_str(u).unwrap() - v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v - Integer::from_str(u).unwrap();
        assert!(n.is_valid());
        assert_eq!((-n).to_string(), out);

        let n = v - rug::Integer::from_str(u).unwrap();
        assert_eq!((-n).to_string(), out);

        let n = v - &Integer::from_str(u).unwrap();
        assert!(n.is_valid());
        assert_eq!((-n).to_string(), out);

        let mut n = rug::Integer::from(0);
        n.assign(v - &rug::Integer::from_str(u).unwrap());
        assert_eq!((-n).to_string(), out);
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
fn sub_u32_properties() {
    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref n, u): &(Integer, u32)| {
            let mut mut_n = n.clone();
            mut_n -= u;
            assert!(mut_n.is_valid());
            let difference = mut_n;

            let mut rug_n = integer_to_rug_integer(n);
            rug_n -= u;
            assert_eq!(rug_integer_to_integer(&rug_n), difference);

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
                bigint_to_integer(&num_sub_u32(integer_to_bigint(n), u)),
                difference
            );
            assert_eq!(
                rug_integer_to_integer(&(integer_to_rug_integer(n) - u)),
                difference
            );

            assert_eq!(&difference + u, *n);
            assert_eq!(n - difference, u);
        },
    );

    #[allow(unknown_lints, identity_op)]
    test_properties(integers, |n| {
        assert_eq!(n + 0u32, *n);
        assert_eq!(0u32 - n, -n);
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(Integer::ZERO - u, -Integer::from(u));
        assert_eq!(u - Integer::ZERO, u);
    });
}
