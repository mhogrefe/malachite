use common::test_properties;
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{SaturatingSub, SaturatingSubAssign, Zero};
use malachite_nz::natural::Natural;
use malachite_test::inputs::base::{pairs_of_unsigneds, unsigneds};
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_unsigned};
use std::str::FromStr;
use std::u32;

#[test]
fn test_saturating_sub_u32() {
    let test = |u, v: u32, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.saturating_sub_assign(v);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap().saturating_sub(v);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = (&Natural::from_str(u).unwrap()).saturating_sub(v);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
    };
    test("0", 0, "0");
    test("123", 123, "0");
    test("123", 0, "123");
    test("456", 123, "333");
    test("123", 456, "0");
    test("1000000000000", 123, "999999999877");
    test("4294967296", 1, "4294967295");
    test("18446744073709551616", 1, "18446744073709551615");
}

#[test]
fn test_u32_saturating_sub_natural() {
    let test = |u: u32, v, out| {
        let n = SaturatingSub::saturating_sub(u, Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);

        let n = SaturatingSub::saturating_sub(u, &Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
    };
    test(0, "0", "0");
    test(123, "123", "0");
    test(123, "0", "123");
    test(456, "123", "333");
    test(123, "456", "0");
    test(123, "1000000000000", "0");
    test(u32::MAX, "4294967295", "0");
}

#[test]
fn saturating_sub_u32_properties() {
    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref n, u): &(Natural, u32)| {
            let mut mut_n = n.clone();
            mut_n.saturating_sub_assign(u);
            assert!(mut_n.is_valid());
            let difference = mut_n;

            let difference_alt = n.saturating_sub(u);
            assert!(difference_alt.is_valid());
            assert_eq!(difference_alt, difference);

            let difference_alt = n.clone().saturating_sub(u);
            assert!(difference_alt.is_valid());
            assert_eq!(difference_alt, difference);

            let reverse_difference = SaturatingSub::saturating_sub(u, n);
            assert!(difference == 0 || reverse_difference == 0);

            let reverse_difference_alt = SaturatingSub::saturating_sub(u, n.clone());
            assert_eq!(reverse_difference_alt, reverse_difference);
            if reverse_difference != 0 {
                assert_eq!(
                    Natural::from(u).saturating_sub(u32::checked_from(n).unwrap()),
                    reverse_difference,
                );
            }

            //TODO assert_eq!(n.saturating_sub(Natural::from(u)), difference);
            //TODO assert_eq!(
            //TODO     SaturatingSub::saturating_sub(u, n).map(Natural::from),
            //TODO     Natural::from(u).saturating_sub(n)
            //TODO );

            assert!(difference <= *n);
            if *n >= u {
                assert_eq!(difference, n - u);
            }
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n.saturating_sub(0), *n);
        assert_eq!(0.saturating_sub(n), 0);
    });

    test_properties(pairs_of_unsigneds::<u32>, |&(x, y)| {
        let difference = x.saturating_sub(y);
        assert_eq!(difference, Natural::from(x).saturating_sub(y));
        assert_eq!(
            difference,
            SaturatingSub::saturating_sub(x, Natural::from(y))
        );
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(SaturatingSub::saturating_sub(u, &Natural::ZERO), u);
        assert_eq!(Natural::ZERO.saturating_sub(u), 0);
    });
}
