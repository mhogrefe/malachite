use malachite_base::num::arithmetic::traits::{DivisibleBy, Parity};
use malachite_base::num::basic::traits::{One, Two};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::natural::naturals;

#[test]
fn even_properties() {
    test_properties(naturals, |x| {
        let even = x.even();
        assert_eq!(x.divisible_by(Natural::TWO), even);
        assert_eq!(!x.odd(), even);
        assert_eq!((x + Natural::ONE).odd(), even);
    });

    test_properties(unsigneds::<Limb>, |&u| {
        assert_eq!(u.even(), Natural::from(u).even());
    });
}

#[test]
fn odd_properties() {
    test_properties(naturals, |x| {
        let odd = x.odd();
        assert_eq!(!x.divisible_by(Natural::TWO), odd);
        assert_eq!(!x.even(), odd);
        assert_eq!((x + Natural::ONE).even(), odd);
    });

    test_properties(unsigneds::<Limb>, |&u| {
        assert_eq!(u.odd(), Natural::from(u).odd());
    });
}
