use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, SaturatingFrom,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

use malachite_test::common::test_properties;
use malachite_test::inputs::integer::integers;

#[test]
fn checked_from_integer_properties() {
    test_properties(integers, |x| {
        let natural_x = Natural::checked_from(x.clone());
        assert!(natural_x.as_ref().map_or(true, |n| n.is_valid()));

        let natural_x_alt = Natural::checked_from(x);
        assert!(natural_x_alt.as_ref().map_or(true, |n| n.is_valid()));
        assert_eq!(natural_x, natural_x_alt);

        assert_eq!(natural_x.is_some(), *x >= 0);
        assert_eq!(natural_x.is_some(), Natural::convertible_from(x));
        if let Some(n) = natural_x {
            assert_eq!(n.to_string(), x.to_string());
            assert_eq!(Natural::exact_from(x), n);
            assert_eq!(Integer::from(&n), *x);
            assert_eq!(Integer::from(n), *x);
        }
    });
}

#[test]
fn saturating_from_integer_properties() {
    test_properties(integers, |x| {
        let natural_x = Natural::saturating_from(x.clone());
        assert!(natural_x.is_valid());

        let natural_x_alt = Natural::saturating_from(x);
        assert!(natural_x_alt.is_valid());
        assert_eq!(natural_x, natural_x_alt);

        assert_eq!(natural_x == 0, *x <= 0);
        assert!(natural_x >= *x);
        assert_eq!(natural_x == *x, Natural::convertible_from(x));
    });
}

#[test]
fn convertible_from_integer_properties() {
    test_properties(integers, |x| {
        let convertible = Natural::convertible_from(x.clone());
        assert_eq!(Natural::convertible_from(x), convertible);
        assert_eq!(convertible, *x >= 0);
    });
}
