use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::num::logic::traits::BitConvertible;
use malachite_base_test_util::num::logic::bit_convertible::{
    from_bits_asc_alt, from_bits_desc_alt,
};
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::integer::logic::from_bits::{
    from_bits_asc_naive, from_bits_desc_naive,
};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::vecs_of_bool;

#[test]
fn from_bits_asc_properties() {
    test_properties(vecs_of_bool, |bits| {
        let x = Integer::from_bits_asc(bits.iter().cloned());
        assert!(x.is_valid());
        assert_eq!(from_bits_asc_naive(bits.iter().cloned()), x);
        assert_eq!(from_bits_asc_alt::<Integer, _>(bits.iter().cloned()), x);
        assert_eq!(Integer::from_bits_desc(bits.iter().cloned().rev()), x);
        assert_eq!(bits.iter().all(|b| !b), x == 0);
        if SignedLimb::convertible_from(&x) {
            assert_eq!(SignedLimb::from_bits_asc(bits.iter().cloned()), x);
        }
    });
}

#[test]
fn from_bits_desc_properties() {
    test_properties(vecs_of_bool, |bits| {
        let x = Integer::from_bits_desc(bits.iter().cloned());
        assert!(x.is_valid());
        assert_eq!(from_bits_desc_naive(bits.iter().cloned()), x);
        assert_eq!(from_bits_desc_alt::<Integer, _>(bits.iter().cloned()), x);
        assert_eq!(Integer::from_bits_asc(bits.iter().cloned().rev()), x);
        assert_eq!(bits.iter().all(|b| !b), x == 0);
        if SignedLimb::convertible_from(&x) {
            assert_eq!(SignedLimb::from_bits_desc(bits.iter().cloned()), x);
        }
    });
}
