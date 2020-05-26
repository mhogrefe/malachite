use malachite_base::num::arithmetic::traits::{DivisibleBy, Parity};
use malachite_base::num::basic::traits::{One, Two};
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::signeds;
use malachite_test::inputs::integer::integers;

#[test]
fn even_properties() {
    test_properties(integers, |x| {
        let even = x.even();
        assert_eq!(!x.odd(), even);
        assert_eq!(x.divisible_by(Integer::TWO), even);
        assert_eq!((x + Integer::ONE).odd(), even);
        assert_eq!((x - Integer::ONE).odd(), even);
        assert_eq!((-x).even(), even);
    });

    test_properties(signeds::<SignedLimb>, |&i| {
        assert_eq!(i.even(), Integer::from(i).even());
    });
}

#[test]
fn odd_properties() {
    test_properties(integers, |x| {
        let odd = x.odd();
        assert_eq!(!x.even(), odd);
        assert_eq!(!x.divisible_by(Integer::TWO), odd);
        assert_eq!((x + Integer::ONE).even(), odd);
        assert_eq!((x - Integer::ONE).even(), odd);
        assert_eq!((-x).odd(), odd);
    });

    test_properties(signeds::<SignedLimb>, |&i| {
        assert_eq!(i.odd(), Integer::from(i).odd());
    });
}
