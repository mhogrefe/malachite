use std::str::FromStr;

use malachite_base::num::arithmetic::traits::CheckedSub;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::BigUint;
use rug;

use malachite_test::common::test_properties;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::pairs_of_unsigneds;
use malachite_test::inputs::natural::{naturals, pairs_of_naturals};
use malachite_test::natural::arithmetic::checked_sub::checked_sub;

#[test]
fn test_checked_sub_natural() {
    let test = |u, v, out| {
        let on = Natural::from_str(u)
            .unwrap()
            .checked_sub(Natural::from_str(v).unwrap());
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = Natural::from_str(u)
            .unwrap()
            .checked_sub(&Natural::from_str(v).unwrap());
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = (&Natural::from_str(u).unwrap()).checked_sub(Natural::from_str(v).unwrap());
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
fn checked_sub_properties() {
    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        let diff = if *x >= *y {
            let mut mut_x = x.clone();
            mut_x -= y;
            assert!(mut_x.is_valid());
            let diff = mut_x;

            let mut rug_x = natural_to_rug_integer(x);
            rug_x -= natural_to_rug_integer(y);
            assert_eq!(rug_integer_to_natural(&rug_x), diff);
            Some(diff)
        } else {
            None
        };

        let diff_alt = x.clone().checked_sub(y.clone());
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.as_ref().map_or(true, |x| x.is_valid()));

        let diff_alt = x.clone().checked_sub(y);
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.as_ref().map_or(true, |x| x.is_valid()));

        let diff_alt = x.checked_sub(y.clone());
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.as_ref().map_or(true, |x| x.is_valid()));

        let diff_alt = x.checked_sub(y);
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.as_ref().map_or(true, |x| x.is_valid()));

        let reverse_diff = y.checked_sub(x);
        assert_eq!(reverse_diff.is_some(), *x == *y || diff.is_none());

        assert_eq!(
            checked_sub(natural_to_biguint(x), natural_to_biguint(y))
                .map(|x| biguint_to_natural(&x)),
            diff
        );
        assert_eq!(
            checked_sub(natural_to_rug_integer(x), natural_to_rug_integer(y))
                .map(|x| rug_integer_to_natural(&x)),
            diff
        );

        if let Some(diff) = diff {
            assert!(diff <= *x);
            assert_eq!(diff + y, *x);
        }
    });

    test_properties(pairs_of_unsigneds::<Limb>, |&(x, y)| {
        assert_eq!(
            x.checked_sub(y).map(Natural::from),
            Natural::from(x).checked_sub(Natural::from(y))
        );
    });

    #[allow(unknown_lints, identity_op, eq_op)]
    test_properties(naturals, |x| {
        assert_eq!(x.checked_sub(Natural::ZERO).as_ref(), Some(x));
        assert_eq!(x.checked_sub(x), Some(Natural::ZERO));
        if *x != 0 {
            assert!((Natural::ZERO.checked_sub(x)).is_none());
        }
    });
}
