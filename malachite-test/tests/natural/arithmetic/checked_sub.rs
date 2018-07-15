use common::test_properties;
use malachite_base::num::{CheckedSub, Zero};
use malachite_nz::natural::Natural;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals};
use malachite_test::natural::arithmetic::checked_sub::checked_sub;
use num::BigUint;
use rug;
use std::str::FromStr;

#[test]
fn test_checked_sub_natural() {
    let test = |u, v, out| {
        let on = Natural::from_str(u)
            .unwrap()
            .checked_sub(&Natural::from_str(v).unwrap());
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = (&Natural::from_str(u).unwrap()).checked_sub(&Natural::from_str(v).unwrap());
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = checked_sub(BigUint::from_str(u).unwrap(), BigUint::from_str(v).unwrap())
            .map(|x| biguint_to_natural(&x));
        assert_eq!(format!("{:?}", on), out);

        let on = checked_sub(
            rug::Integer::from_str(u).unwrap(),
            rug::Integer::from_str(v).unwrap(),
        );
        assert_eq!(format!("{:?}", on), out);
    };
    test("0", "0", "Some(0)");
    test("0", "123", "None");
    test("123", "0", "Some(123)");
    test("456", "123", "Some(333)");
    test("1000000000000", "123", "Some(999999999877)");
    test("123", "1000000000000", "None");
    test(
        "12345678987654321",
        "314159265358979",
        "Some(12031519722295342)",
    );
    test("4294967296", "1", "Some(4294967295)");
    test("4294967295", "4294967295", "Some(0)");
    test("4294967296", "4294967295", "Some(1)");
    test("4294967296", "4294967296", "Some(0)");
    test("4294967295", "4294967296", "None");
    test("18446744073709551616", "1", "Some(18446744073709551615)");
    test("18446744073709551615", "18446744073709551615", "Some(0)");
    test("18446744073709551616", "18446744073709551615", "Some(1)");
    test("18446744073709551615", "18446744073709551616", "None");
    test(
        "70734740290631708",
        "282942734368",
        "Some(70734457347897340)",
    );
    test("282942734368", "70734740290631708", "None");
}

#[test]
fn sub_properties() {
    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        let difference = if *x >= *y {
            let mut mut_x = x.clone();
            mut_x -= y;
            assert!(mut_x.is_valid());
            let difference = mut_x;

            let mut rug_x = natural_to_rug_integer(x);
            rug_x -= natural_to_rug_integer(y);
            assert_eq!(rug_integer_to_natural(&rug_x), difference);
            Some(difference)
        } else {
            None
        };

        let difference_alt = x.checked_sub(y);
        assert_eq!(difference_alt, difference);
        assert!(difference.as_ref().map_or(true, |x| x.is_valid()));

        let difference_alt = x.clone().checked_sub(y);
        assert_eq!(difference_alt, difference);
        assert!(difference.as_ref().map_or(true, |x| x.is_valid()));

        let reverse_difference = y.checked_sub(x);
        assert_eq!(
            reverse_difference.is_some(),
            *x == *y || difference.is_none()
        );
        assert!(reverse_difference.map_or(true, |x| x.is_valid()));

        assert_eq!(
            checked_sub(natural_to_biguint(x), natural_to_biguint(y))
                .map(|x| biguint_to_natural(&x)),
            difference
        );
        assert_eq!(
            checked_sub(natural_to_rug_integer(x), natural_to_rug_integer(y))
                .map(|x| rug_integer_to_natural(&x)),
            difference
        );

        if let Some(difference) = difference {
            assert!(difference <= *x);
            assert_eq!(difference + y, *x);
        }
    });

    #[allow(unknown_lints, identity_op, eq_op)]
    test_properties(naturals, |x| {
        assert_eq!(x.checked_sub(&Natural::ZERO).as_ref(), Some(x));
        assert_eq!(x.checked_sub(x), Some(Natural::ZERO));
        if *x != 0 {
            assert!((Natural::ZERO.checked_sub(x)).is_none());
        }
    });
}
