use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, SaturatingFrom,
};
use malachite_nz::natural::Natural;
use malachite_nz_test_util::common::{biguint_to_natural, rug_integer_to_natural};
use num::BigUint;
use rug;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{signeds, unsigneds};

macro_rules! unsigned_properties {
    ($t: ident) => {
        test_properties(unsigneds::<$t>, |&u| {
            let n = Natural::from(u);
            assert!(n.is_valid());
            assert_eq!($t::exact_from(&n), u);
            assert_eq!(Natural::from(u128::exact_from(u)), n);
        });
    };
}

macro_rules! signed_properties {
    ($t: ident) => {
        test_properties(signeds::<$t>, |&i| {
            let on = Natural::checked_from(i);
            assert!(on.as_ref().map_or(true, |n| n.is_valid()));
            assert_eq!(on.is_some(), i >= 0);
            assert_eq!(Natural::convertible_from(i), i >= 0);
            let n = Natural::saturating_from(i);
            assert!(n.is_valid());
            if let Some(x) = on.as_ref() {
                assert_eq!(*x, n);
                assert_eq!($t::exact_from(x), i);
                assert_eq!(Natural::exact_from(i128::exact_from(i)), n);
            } else {
                assert_eq!(n, 0);
            }
        });
    };
}

#[test]
fn from_primitive_integer_properties() {
    test_properties(unsigneds::<u32>, |&u| {
        let n = Natural::from(u);
        assert_eq!(biguint_to_natural(&BigUint::from(u)), n);
        assert_eq!(rug_integer_to_natural(&rug::Integer::from(u)), n);
    });

    test_properties(unsigneds::<u64>, |&u| {
        let n = Natural::from(u);
        assert_eq!(biguint_to_natural(&BigUint::from(u)), n);
        assert_eq!(rug_integer_to_natural(&rug::Integer::from(u)), n);
    });

    unsigned_properties!(u8);
    unsigned_properties!(u16);
    unsigned_properties!(u32);
    unsigned_properties!(u64);
    unsigned_properties!(usize);

    signed_properties!(i8);
    signed_properties!(i16);
    signed_properties!(i32);
    signed_properties!(i64);
    signed_properties!(isize);
}
