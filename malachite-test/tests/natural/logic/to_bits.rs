use malachite_base::num::logic::traits::{BitConvertible, BitIterable};
use malachite_base_test_util::num::logic::bit_convertible::{to_bits_asc_alt, to_bits_desc_alt};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::natural::logic::to_bits::{to_bits_asc_naive, to_bits_desc_naive};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::natural::naturals;

#[test]
fn to_bits_asc_properties() {
    test_properties(naturals, |x| {
        let bits = x.to_bits_asc();
        assert_eq!(to_bits_asc_naive(x), bits);
        assert_eq!(to_bits_asc_alt(x), bits);
        assert_eq!(x.bits().collect::<Vec<bool>>(), bits);
        assert_eq!(Natural::from_bits_asc(&bits), *x);
        if *x != 0 {
            assert_eq!(*bits.last().unwrap(), true);
        }
    });

    test_properties(unsigneds::<Limb>, |&u| {
        assert_eq!(u.to_bits_asc(), Natural::from(u).to_bits_asc());
    });
}

#[test]
fn to_bits_desc_properties() {
    test_properties(naturals, |x| {
        let bits = x.to_bits_desc();
        assert_eq!(to_bits_desc_naive(x), bits);
        assert_eq!(to_bits_desc_alt(x), bits);
        assert_eq!(x.bits().rev().collect::<Vec<bool>>(), bits);
        assert_eq!(Natural::from_bits_desc(&bits), *x);
        if *x != 0 {
            assert_eq!(bits[0], true);
        }
    });

    test_properties(unsigneds::<Limb>, |&u| {
        assert_eq!(u.to_bits_desc(), Natural::from(u).to_bits_desc());
    });
}
