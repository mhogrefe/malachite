use malachite_base::num::arithmetic::traits::{CeilingLogTwo, FloorLogTwo, PowerOfTwo};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::log_two::{limbs_ceiling_log_two, limbs_floor_log_two};
use malachite_nz::natural::logic::significant_bits::limbs_significant_bits;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{positive_unsigneds, vecs_of_unsigned_var_1};
use malachite_test::inputs::natural::positive_naturals;

#[test]
fn limbs_floor_log_two_properties() {
    test_properties(vecs_of_unsigned_var_1, |limbs| {
        let floor_log_two = limbs_floor_log_two(limbs);
        assert_eq!(limbs.len() == 1, floor_log_two < Limb::WIDTH);
        assert_eq!(floor_log_two, limbs_significant_bits(limbs) - 1);
        assert_eq!(
            floor_log_two,
            Natural::from_limbs_asc(limbs).floor_log_two()
        );
    });
}

#[test]
fn limbs_ceiling_log_two_properties() {
    test_properties(vecs_of_unsigned_var_1, |limbs| {
        let ceiling_log_two = limbs_ceiling_log_two(limbs);
        assert_eq!(
            limbs.len() == 1 || limbs == &[0, 1],
            ceiling_log_two <= Limb::WIDTH
        );
        assert_eq!(
            ceiling_log_two,
            Natural::from_limbs_asc(limbs).ceiling_log_two()
        );
    });
}

#[test]
fn floor_log_two_properties() {
    test_properties(positive_naturals, |x| {
        let floor_log_two = x.floor_log_two();
        assert_eq!(*x <= Limb::MAX, floor_log_two < Limb::WIDTH);
        assert_eq!(floor_log_two, x.significant_bits() - 1);
        assert_eq!(floor_log_two, limbs_floor_log_two(&x.to_limbs_asc()));
        assert!(Natural::power_of_two(floor_log_two) <= *x);
        assert!(*x < Natural::power_of_two(floor_log_two + 1));
    });

    test_properties(positive_unsigneds::<Limb>, |&u| {
        assert_eq!(u.floor_log_two(), Natural::from(u).floor_log_two());
    });
}

#[test]
fn ceiling_log_two_properties() {
    test_properties(positive_naturals, |x| {
        let ceiling_log_two = x.ceiling_log_two();
        assert_eq!(*x <= Limb::MAX, ceiling_log_two <= Limb::WIDTH);
        assert_eq!(ceiling_log_two, limbs_ceiling_log_two(&x.to_limbs_asc()));
        if ceiling_log_two != 0 {
            assert!(Natural::power_of_two(ceiling_log_two - 1) < *x);
        }
        assert!(*x <= Natural::power_of_two(ceiling_log_two));
    });

    test_properties(positive_unsigneds::<Limb>, |&u| {
        assert_eq!(u.ceiling_log_two(), Natural::from(u).ceiling_log_two());
    });
}
