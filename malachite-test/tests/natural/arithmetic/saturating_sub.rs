use common::test_properties;
use malachite_base::num::{SaturatingSub, SaturatingSubAssign, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_test::inputs::base::pairs_of_unsigneds;
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_unsigned, pairs_of_naturals};
use std::str::FromStr;

#[test]
fn test_saturating_sub_natural() {
    let test = |u, v, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.saturating_sub_assign(Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n.saturating_sub_assign(&Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .saturating_sub(Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .saturating_sub(&Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).saturating_sub(Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).saturating_sub(&Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", "0");
    test("0", "123", "0");
    test("123", "0", "123");
    test("456", "123", "333");
    test("1000000000000", "123", "999999999877");
    test("123", "1000000000000", "0");
    test("12345678987654321", "314159265358979", "12031519722295342");
    test("4294967296", "1", "4294967295");
    test("4294967295", "4294967295", "0");
    test("4294967296", "4294967295", "1");
    test("4294967296", "4294967296", "0");
    test("4294967295", "4294967296", "0");
    test("18446744073709551616", "1", "18446744073709551615");
    test("18446744073709551615", "18446744073709551615", "0");
    test("18446744073709551616", "18446744073709551615", "1");
    test("18446744073709551615", "18446744073709551616", "0");
    test("70734740290631708", "282942734368", "70734457347897340");
    test("282942734368", "70734740290631708", "0");
}

#[test]
fn saturating_sub_properties() {
    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        let mut mut_x = x.clone();
        mut_x.saturating_sub_assign(y);
        assert!(mut_x.is_valid());
        let difference = mut_x;

        let mut mut_x = x.clone();
        mut_x.saturating_sub_assign(y.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, difference);

        let difference_alt = x.clone().saturating_sub(y.clone());
        assert_eq!(difference_alt, difference);
        assert!(difference_alt.is_valid());

        let difference_alt = x.clone().saturating_sub(y);
        assert_eq!(difference_alt, difference);
        assert!(difference_alt.is_valid());

        let difference_alt = x.saturating_sub(y.clone());
        assert_eq!(difference_alt, difference);
        assert!(difference_alt.is_valid());

        let difference_alt = x.saturating_sub(y);
        assert_eq!(difference_alt, difference);
        assert!(difference_alt.is_valid());

        let reverse_difference = y.saturating_sub(x);
        if difference > 0 as Limb {
            assert_eq!(reverse_difference, 0 as Limb);
        }
        if reverse_difference > 0 as Limb {
            assert_eq!(difference, 0 as Limb);
        }

        assert!(difference <= *x);
        assert!(difference + y >= *x);
    });

    test_properties(pairs_of_natural_and_unsigned::<Limb>, |&(ref x, y)| {
        let difference = x.saturating_sub(Natural::from(y));
        assert_eq!(x.saturating_sub(y), difference);

        let difference = Natural::from(y).saturating_sub(x);
        assert_eq!(SaturatingSub::saturating_sub(y, x), difference);
    });

    test_properties(pairs_of_unsigneds::<Limb>, |&(x, y)| {
        assert_eq!(
            x.saturating_sub(y),
            Natural::from(x).saturating_sub(Natural::from(y))
        );
    });

    #[allow(unknown_lints, identity_op, eq_op)]
    test_properties(naturals, |x| {
        assert_eq!(x.saturating_sub(Natural::ZERO), *x);
        assert_eq!(x.saturating_sub(x), Natural::ZERO);
        assert_eq!(Natural::ZERO.saturating_sub(x), Natural::ZERO);
    });
}
