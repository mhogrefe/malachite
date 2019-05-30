use std::str::FromStr;

use malachite_base::comparison::Max;
use malachite_base::num::arithmetic::traits::{SaturatingSub, SaturatingSubAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use common::test_properties;
use malachite_test::inputs::base::{pairs_of_unsigneds, unsigneds};
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_unsigned};

#[test]
fn test_saturating_sub_limb() {
    let test = |u, v: Limb, out| {
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
fn test_limb_saturating_sub_natural() {
    let test = |u: Limb, v, out| {
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
    #[cfg(feature = "32_bit_limbs")]
    test(Limb::MAX, "4294967295", "0");
    #[cfg(feature = "64_bit_limbs")]
    test(Limb::MAX, "18446744073709551615", "0");
}

#[test]
fn saturating_sub_limb_properties() {
    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref n, u): &(Natural, Limb)| {
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
            assert!(difference == 0 as Limb || reverse_difference == 0);

            let reverse_difference_alt = SaturatingSub::saturating_sub(u, n.clone());
            assert_eq!(reverse_difference_alt, reverse_difference);
            if reverse_difference != 0 {
                assert_eq!(
                    Natural::from(u).saturating_sub(Limb::checked_from(n).unwrap()),
                    reverse_difference,
                );
            }

            assert!(difference <= *n);
            if *n >= u {
                assert_eq!(difference, n - u);
            }
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n.saturating_sub(0 as Limb), *n);
        assert_eq!(SaturatingSub::saturating_sub(0 as Limb, n), 0);
    });

    test_properties(pairs_of_unsigneds::<Limb>, |&(x, y)| {
        let difference = x.saturating_sub(y);
        assert_eq!(difference, Natural::from(x).saturating_sub(y));
        assert_eq!(
            difference,
            SaturatingSub::saturating_sub(x, Natural::from(y))
        );
    });

    test_properties(unsigneds, |&u: &Limb| {
        assert_eq!(SaturatingSub::saturating_sub(u, &Natural::ZERO), u);
        assert_eq!(Natural::ZERO.saturating_sub(u), 0 as Limb);
    });
}
