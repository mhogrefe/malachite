use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::num::logic::traits::BitConvertible;
use malachite_base_test_util::num::logic::bit_convertible::{
    from_bits_asc_alt, from_bits_desc_alt,
};
use malachite_nz::natural::logic::bit_convertible::{
    limbs_asc_from_bits_asc, limbs_asc_from_bits_desc,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::natural::logic::from_bits::{
    from_bits_asc_naive, from_bits_desc_naive,
};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::vecs_of_bool;

#[test]
fn limbs_asc_from_bits_asc_properties() {
    test_properties(vecs_of_bool, |bits| {
        let limbs = limbs_asc_from_bits_asc(bits);
        assert_eq!(
            Natural::from_limbs_asc(&limbs),
            Natural::from_bits_asc(bits)
        );
        let mut limb_count = bits.len() >> Limb::LOG_WIDTH;
        if limb_count << Limb::LOG_WIDTH != bits.len() {
            limb_count += 1;
        }
        assert_eq!(limbs.len(), limb_count);
    });
}

#[test]
fn limbs_asc_from_bits_desc_properties() {
    test_properties(vecs_of_bool, |bits| {
        let limbs = limbs_asc_from_bits_desc(bits);
        assert_eq!(
            Natural::from_limbs_asc(&limbs),
            Natural::from_bits_desc(bits)
        );
        let mut limb_count = bits.len() >> Limb::LOG_WIDTH;
        if limb_count << Limb::LOG_WIDTH != bits.len() {
            limb_count += 1;
        }
        assert_eq!(limbs.len(), limb_count);
    });
}

#[test]
fn from_bits_asc_properties() {
    test_properties(vecs_of_bool, |bits| {
        let x = Natural::from_bits_asc(bits);
        assert!(x.is_valid());
        assert_eq!(from_bits_asc_naive(bits), x);
        assert_eq!(from_bits_asc_alt::<Natural>(bits), x);
        let mut trimmed_bits: Vec<bool> =
            bits.iter().cloned().rev().skip_while(|&bit| !bit).collect();
        trimmed_bits.reverse();
        assert_eq!(x.to_bits_asc(), trimmed_bits);
        assert_eq!(
            Natural::from_bits_desc(&bits.iter().cloned().rev().collect::<Vec<bool>>()),
            x
        );
        if !bits.is_empty() && *bits.last().unwrap() {
            assert_eq!(x.to_bits_asc(), *bits);
        }
        assert_eq!(bits.iter().all(|b| !b), x == 0);

        if Limb::convertible_from(&x) {
            assert_eq!(Limb::from_bits_asc(bits), x);
        }
    });
}

#[test]
fn from_bits_desc_properties() {
    test_properties(vecs_of_bool, |bits| {
        let x = Natural::from_bits_desc(bits);
        assert!(x.is_valid());
        assert_eq!(from_bits_desc_naive(bits), x);
        assert_eq!(from_bits_desc_alt::<Natural>(bits), x);
        assert_eq!(
            x.to_bits_desc(),
            bits.iter()
                .cloned()
                .skip_while(|&b| !b)
                .collect::<Vec<bool>>()
        );
        assert_eq!(
            Natural::from_bits_asc(&bits.iter().cloned().rev().collect::<Vec<bool>>()),
            x
        );
        if !bits.is_empty() && bits[0] {
            assert_eq!(x.to_bits_desc(), *bits);
        }
        assert_eq!(bits.iter().all(|b| !b), x == 0);

        if Limb::convertible_from(&x) {
            assert_eq!(Limb::from_bits_desc(bits), x);
        }
    });
}
