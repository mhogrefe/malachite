use common::test_properties;
use malachite_base::comparison::Max;
use malachite_base::num::traits::Parity;
use malachite_nz::natural::logic::trailing_zeros::limbs_trailing_zeros;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_test::inputs::base::{positive_unsigneds, vecs_of_unsigned_var_3};
use malachite_test::inputs::natural::naturals;
use malachite_test::natural::logic::trailing_zeros::natural_trailing_zeros_alt;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_trailing_zeros() {
    let test = |limbs, out| {
        assert_eq!(limbs_trailing_zeros(limbs), out);
    };
    test(&[4], 2);
    test(&[0, 4], 34);
    test(&[1, 2, 3], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_trailing_zeros_fail_1() {
    limbs_trailing_zeros(&[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_trailing_zeros_fail_2() {
    limbs_trailing_zeros(&[0, 0, 0]);
}

#[test]
fn test_trailing_zeros() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().trailing_zeros(), out);
        assert_eq!(
            natural_trailing_zeros_alt(&Natural::from_str(n).unwrap()),
            out
        );
    };
    test("0", None);
    test("123", Some(0));
    test("1000000000000", Some(12));
    test("4294967295", Some(0));
    test("4294967296", Some(32));
    test("18446744073709551615", Some(0));
    test("18446744073709551616", Some(64));
}

#[test]
fn limbs_trailing_zeros_properties() {
    test_properties(vecs_of_unsigned_var_3, |limbs| {
        assert_eq!(
            Some(limbs_trailing_zeros(limbs)),
            Natural::from_limbs_asc(limbs).trailing_zeros()
        );
    });
}

#[test]
fn trailing_zeros_properties() {
    test_properties(naturals, |x| {
        let trailing_zeros = x.trailing_zeros();
        assert_eq!(natural_trailing_zeros_alt(x), trailing_zeros);
        assert_eq!(trailing_zeros.is_none(), *x == 0 as Limb);
        if *x != 0 as Limb {
            let trailing_zeros = trailing_zeros.unwrap();
            if trailing_zeros <= u64::from(Limb::MAX) {
                assert!((x >> trailing_zeros).odd());
                assert_eq!(x >> trailing_zeros << trailing_zeros, *x);
            }
        }
    });

    test_properties(positive_unsigneds::<Limb>, |&u| {
        assert_eq!(
            Natural::from(u).trailing_zeros(),
            Some(u64::from(u.trailing_zeros()))
        );
    });
}
