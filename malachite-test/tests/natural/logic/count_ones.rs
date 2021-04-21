use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::logic::traits::CountOnes;
use malachite_nz::natural::logic::count_ones::limbs_count_ones;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::natural::logic::count_ones::{
    natural_count_ones_alt_1, natural_count_ones_alt_2,
};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{unsigneds, vecs_of_unsigned};
use malachite_test::inputs::natural::naturals;

#[test]
fn limbs_count_ones_properties() {
    test_properties(vecs_of_unsigned, |limbs| {
        assert_eq!(
            limbs_count_ones(limbs),
            Natural::from_limbs_asc(limbs).count_ones()
        );
    });
}

#[test]
fn count_ones_properties() {
    test_properties(naturals, |x| {
        let ones = x.count_ones();
        assert_eq!(natural_count_ones_alt_1(x), ones);
        assert_eq!(natural_count_ones_alt_2(x), ones);
        assert_eq!(ones == 0, *x == 0);
        assert_eq!(ones == 1, x.is_power_of_2());
        assert_eq!((!x).checked_count_zeros(), Some(ones));
    });

    test_properties(unsigneds::<Limb>, |&u| {
        assert_eq!(Natural::from(u).count_ones(), CountOnes::count_ones(u));
    });
}
