use common::test_properties;
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{CheckedSub, Zero};
use malachite_nz::natural::Natural;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_unsigned};
use malachite_test::natural::arithmetic::checked_sub_u32::{
    num_checked_sub_u32, rug_checked_sub_u32,
};
use num::BigUint;
use rug;
use std::str::FromStr;
use std::u32;

#[test]
fn test_checked_sub_u32() {
    let test = |u, v: u32, out| {
        let on = Natural::from_str(u).unwrap().checked_sub(v);
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = (&Natural::from_str(u).unwrap()).checked_sub(v);
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let ux = BigUint::from_str(u).unwrap();
        let on = num_checked_sub_u32(ux, v).map(|x| biguint_to_natural(&x));
        assert_eq!(format!("{:?}", on), out);

        let on = rug_checked_sub_u32(rug::Integer::from_str(u).unwrap(), v);
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
fn test_u32_checked_sub_natural() {
    let test = |u: u32, v, out| {
        let on = CheckedSub::checked_sub(u, &Natural::from_str(v).unwrap());
        assert_eq!(format!("{:?}", on), out);
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

            let difference_alt = n.checked_sub(u);
            assert!(difference_alt.as_ref().map_or(true, |n| n.is_valid()));
            assert_eq!(difference_alt, difference);

            let difference_alt = n.clone().checked_sub(u);
            assert!(difference_alt.as_ref().map_or(true, |n| n.is_valid()));
            assert_eq!(difference_alt, difference);

            let reverse_difference = CheckedSub::checked_sub(u, n);
            assert_eq!(
                reverse_difference.is_some(),
                *n == u || difference.is_none()
            );
            if let Some(reverse_difference) = reverse_difference {
                assert_eq!(
                    u32::checked_from(
                        &Natural::from(u)
                            .checked_sub(u32::checked_from(n).unwrap())
                            .unwrap()
                    ).unwrap(),
                    reverse_difference
                );
            }

            assert_eq!(n.checked_sub(Natural::from(u)), difference);
            assert_eq!(
                CheckedSub::checked_sub(u, n).map(Natural::from),
                Natural::from(u).checked_sub(n)
            );

            assert_eq!(
                num_checked_sub_u32(natural_to_biguint(n), u).map(|x| biguint_to_natural(&x)),
                difference
            );
            assert_eq!(
                rug_checked_sub_u32(natural_to_rug_integer(n), u)
                    .map(|x| rug_integer_to_natural(&x)),
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
        assert_eq!((n.checked_sub(0)).as_ref(), Some(n));
        if *n != 0 {
            assert!((0.checked_sub(n)).is_none());
        }
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(CheckedSub::checked_sub(u, &Natural::ZERO), Some(u));
        if u != 0 {
            assert!((Natural::ZERO.checked_sub(u)).is_none());
        }
    });
}
