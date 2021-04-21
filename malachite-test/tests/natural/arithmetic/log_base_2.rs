use malachite_base::num::arithmetic::traits::{CeilingLogBase2, FloorLogBase2, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::log_base_2::{
    limbs_ceiling_log_base_2, limbs_floor_log_base_2,
};
use malachite_nz::natural::logic::significant_bits::limbs_significant_bits;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{positive_unsigneds, vecs_of_unsigned_var_1};
use malachite_test::inputs::natural::positive_naturals;

#[test]
fn limbs_floor_log_base_2_properties() {
    test_properties(vecs_of_unsigned_var_1, |limbs| {
        let floor_log_base_2 = limbs_floor_log_base_2(limbs);
        assert_eq!(limbs.len() == 1, floor_log_base_2 < Limb::WIDTH);
        assert_eq!(floor_log_base_2, limbs_significant_bits(limbs) - 1);
        assert_eq!(
            floor_log_base_2,
            Natural::from_limbs_asc(limbs).floor_log_base_2()
        );
    });
}

#[test]
fn limbs_ceiling_log_base_2_properties() {
    test_properties(vecs_of_unsigned_var_1, |limbs| {
        let ceiling_log_base_2 = limbs_ceiling_log_base_2(limbs);
        assert_eq!(
            limbs.len() == 1 || limbs == &[0, 1],
            ceiling_log_base_2 <= Limb::WIDTH
        );
        assert_eq!(
            ceiling_log_base_2,
            Natural::from_limbs_asc(limbs).ceiling_log_base_2()
        );
    });
}

#[test]
fn floor_log_base_2_properties() {
    test_properties(positive_naturals, |x| {
        let floor_log_base_2 = x.floor_log_base_2();
        assert_eq!(*x <= Limb::MAX, floor_log_base_2 < Limb::WIDTH);
        assert_eq!(floor_log_base_2, x.significant_bits() - 1);
        assert_eq!(floor_log_base_2, limbs_floor_log_base_2(&x.to_limbs_asc()));
        assert!(Natural::power_of_2(floor_log_base_2) <= *x);
        assert!(*x < Natural::power_of_2(floor_log_base_2 + 1));
    });

    test_properties(positive_unsigneds::<Limb>, |&u| {
        assert_eq!(u.floor_log_base_2(), Natural::from(u).floor_log_base_2());
    });
}

#[test]
fn ceiling_log_base_2_properties() {
    test_properties(positive_naturals, |x| {
        let ceiling_log_base_2 = x.ceiling_log_base_2();
        assert_eq!(*x <= Limb::MAX, ceiling_log_base_2 <= Limb::WIDTH);
        assert_eq!(
            ceiling_log_base_2,
            limbs_ceiling_log_base_2(&x.to_limbs_asc())
        );
        if ceiling_log_base_2 != 0 {
            assert!(Natural::power_of_2(ceiling_log_base_2 - 1) < *x);
        }
        assert!(*x <= Natural::power_of_2(ceiling_log_base_2));
    });

    test_properties(positive_unsigneds::<Limb>, |&u| {
        assert_eq!(
            u.ceiling_log_base_2(),
            Natural::from(u).ceiling_log_base_2()
        );
    });
}
