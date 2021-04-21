use malachite_base::num::arithmetic::traits::{FloorLogBase2, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::log_base_2::limbs_floor_log_base_2;
use malachite_nz::natural::logic::significant_bits::limbs_significant_bits;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::{natural_to_biguint, natural_to_rug_integer};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{unsigneds, vecs_of_unsigned_var_1};
use malachite_test::inputs::natural::naturals;

#[test]
fn limbs_significant_bits_properties() {
    test_properties(vecs_of_unsigned_var_1, |limbs| {
        let significant_bits = limbs_significant_bits(limbs);
        assert_eq!(limbs.len() == 1, significant_bits <= Limb::WIDTH);
        assert_eq!(significant_bits, limbs_floor_log_base_2(limbs) + 1);
        assert_eq!(
            significant_bits,
            Natural::from_limbs_asc(limbs).significant_bits()
        );
    });
}

#[test]
fn significant_bits_properties() {
    test_properties(naturals, |x| {
        let significant_bits = x.significant_bits();
        assert_eq!(
            u64::wrapping_from(natural_to_biguint(x).bits()),
            significant_bits
        );
        assert_eq!(
            u64::from(natural_to_rug_integer(x).significant_bits()),
            significant_bits
        );
        assert_eq!(*x <= Limb::MAX, significant_bits <= Limb::WIDTH);
        if *x != 0 {
            assert_eq!(significant_bits, x.floor_log_base_2() + 1);
            assert_eq!(significant_bits, limbs_significant_bits(&x.to_limbs_asc()));
            assert!(Natural::power_of_2(significant_bits - 1) <= *x);
            assert!(*x < Natural::power_of_2(significant_bits));
        }
    });

    test_properties(unsigneds::<Limb>, |&u| {
        assert_eq!(Natural::from(u).significant_bits(), u.significant_bits());
    });
}
