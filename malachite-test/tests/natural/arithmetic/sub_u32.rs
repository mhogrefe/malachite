use common::test_properties;
use malachite_base::misc::CheckedFrom;
use malachite_base::num::Zero;
use malachite_nz::natural::Natural;
use malachite_test::common::{biguint_to_natural, natural_to_biguint, natural_to_rug_integer,
                             rug_integer_to_natural};
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_unsigned};
use malachite_test::natural::arithmetic::sub_u32::{num_sub_u32, rug_sub_u32};
use num::BigUint;
use rug;
use std::str::FromStr;
use std::u32;

#[test]
fn test_sub_assign_u32() {
    let test = |u, v: u32, out| {
        let mut n = Natural::from_str(u).unwrap();
        n -= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 0, "0");
    test("123", 123, "0");
    test("123", 0, "123");
    test("456", 123, "333");
    test("1000000000000", 123, "999999999877");
    test("4294967296", 1, "4294967295");
    test("18446744073709551616", 1, "18446744073709551615");
}

#[test]
#[should_panic(expected = "Cannot subtract a u32 from a smaller Natural. self: 123, other: 456")]
fn sub_assign_u32_fail() {
    let mut x = Natural::from_str("123").unwrap();
    x -= 456;
}

#[test]
fn test_sub_u32() {
    let test = |u, v: u32, out| {
        let on = Natural::from_str(u).unwrap() - v;
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = &Natural::from_str(u).unwrap() - v;
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let ux = BigUint::from_str(u).unwrap();
        let on = num_sub_u32(ux, v).map(|x| biguint_to_natural(&x));
        assert_eq!(format!("{:?}", on), out);

        let on = rug_sub_u32(rug::Integer::from_str(u).unwrap(), v);
        assert_eq!(format!("{:?}", on), out);
    };
    test("0", 0, "Some(0)");
    test("123", 123, "Some(0)");
    test("123", 0, "Some(123)");
    test("456", 123, "Some(333)");
    test("123", 456, "None");
    test("1000000000000", 123, "Some(999999999877)");
    test("4294967296", 1, "Some(4294967295)");
    test("18446744073709551616", 1, "Some(18446744073709551615)");
}

#[test]
fn test_u32_sub_natural() {
    let test = |u: u32, v, out| {
        let on = u - &Natural::from_str(v).unwrap();
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test(0, "0", "Some(0)");
    test(123, "123", "Some(0)");
    test(123, "0", "Some(123)");
    test(456, "123", "Some(333)");
    test(123, "456", "None");
    test(123, "1000000000000", "None");
    test(u32::MAX, "4294967295", "Some(0)");
}

#[test]
fn sub_u32_properties() {
    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref n, u): &(Natural, u32)| {
            let difference = if *n >= u {
                let mut mut_n = n.clone();
                mut_n -= u;
                assert!(mut_n.is_valid());
                let difference = mut_n;

                let mut rug_n = natural_to_rug_integer(n);
                rug_n -= u;
                assert_eq!(rug_integer_to_natural(&rug_n), difference);
                Some(difference)
            } else {
                None
            };

            let difference_alt = n - u;
            assert!(difference_alt.as_ref().map_or(true, |n| n.is_valid()));
            assert_eq!(difference_alt, difference);

            let difference_alt = n.clone() - u;
            assert!(difference_alt.as_ref().map_or(true, |n| n.is_valid()));
            assert_eq!(difference_alt, difference);

            let reverse_difference = u - n;
            assert_eq!(
                reverse_difference.is_some(),
                *n == u || difference.is_none()
            );
            if reverse_difference.is_some() {
                assert_eq!(
                    Natural::from(u) - u32::checked_from(n).unwrap(),
                    reverse_difference
                );
            }
            assert!(reverse_difference.map_or(true, |n| n.is_valid()));

            assert_eq!(n - &Natural::from(u), difference);
            assert_eq!(u - n, Natural::from(u) - n);

            assert_eq!(
                num_sub_u32(natural_to_biguint(n), u).map(|x| biguint_to_natural(&x)),
                difference
            );
            assert_eq!(
                rug_sub_u32(natural_to_rug_integer(n), u).map(|x| rug_integer_to_natural(&x)),
                difference
            );

            if let Some(difference) = difference {
                assert!(difference <= *n);
                assert_eq!(difference + u, *n);
            }
        },
    );

    #[allow(unknown_lints, identity_op)]
    test_properties(naturals, |n| {
        assert_eq!((n - 0).as_ref(), Some(n));
        if *n != 0 {
            assert!((0 - n).is_none());
        }
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(u - &Natural::ZERO, Some(Natural::from(u)));
        if u != 0 {
            assert!((Natural::ZERO - u).is_none());
        }
    });
}
