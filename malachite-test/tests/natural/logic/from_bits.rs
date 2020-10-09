use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::num::logic::traits::BitConvertible;
use malachite_base_test_util::num::logic::bit_convertible::{
    from_bits_asc_alt, from_bits_desc_alt,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::natural::logic::from_bits::{
    from_bits_asc_naive, from_bits_desc_naive,
};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::vecs_of_bool;

#[test]
fn from_bits_asc_properties() {
    test_properties(vecs_of_bool, |bits| {
        let x = Natural::from_bits_asc(bits.iter().cloned());
        assert!(x.is_valid());
        assert_eq!(from_bits_asc_naive(bits.iter().cloned()), x);
        assert_eq!(from_bits_asc_alt::<Natural, _>(bits.iter().cloned()), x);
        let mut trimmed_bits: Vec<bool> =
            bits.iter().cloned().rev().skip_while(|&bit| !bit).collect();
        trimmed_bits.reverse();
        assert_eq!(x.to_bits_asc(), trimmed_bits);
        assert_eq!(Natural::from_bits_desc(bits.iter().cloned().rev()), x);
        if !bits.is_empty() && *bits.last().unwrap() {
            assert_eq!(x.to_bits_asc(), *bits);
        }
        assert_eq!(bits.iter().all(|b| !b), x == 0);
        if Limb::convertible_from(&x) {
            assert_eq!(Limb::from_bits_asc(bits.iter().cloned()), x);
        }
    });
}

#[test]
fn from_bits_desc_properties() {
    test_properties(vecs_of_bool, |bits| {
        let x = Natural::from_bits_desc(bits.iter().cloned());
        assert!(x.is_valid());
        assert_eq!(from_bits_desc_naive(bits.iter().cloned()), x);
        assert_eq!(from_bits_desc_alt::<Natural, _>(bits.iter().cloned()), x);
        assert_eq!(
            x.to_bits_desc(),
            bits.iter()
                .cloned()
                .skip_while(|&b| !b)
                .collect::<Vec<bool>>()
        );
        assert_eq!(Natural::from_bits_asc(bits.iter().cloned().rev()), x);
        if !bits.is_empty() && bits[0] {
            assert_eq!(x.to_bits_desc(), *bits);
        }
        assert_eq!(bits.iter().all(|b| !b), x == 0);
        if Limb::convertible_from(&x) {
            assert_eq!(Limb::from_bits_desc(bits.iter().cloned()), x);
        }
    });
}
