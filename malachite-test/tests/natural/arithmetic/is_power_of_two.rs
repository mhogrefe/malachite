use malachite_base::num::arithmetic::traits::IsPowerOfTwo;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::is_power_of_two::limbs_is_power_of_two;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::natural_to_rug_integer;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{unsigneds, vecs_of_unsigned_var_1};
use malachite_test::inputs::natural::naturals;

#[test]
fn limbs_is_power_of_two_properties() {
    test_properties(vecs_of_unsigned_var_1, |ref limbs| {
        assert_eq!(
            limbs_is_power_of_two(limbs),
            Natural::from_limbs_asc(limbs).is_power_of_two()
        );
    });
}

#[test]
fn is_power_of_two_properties() {
    test_properties(naturals, |x| {
        let is_power_of_two = x.is_power_of_two();
        assert_eq!(natural_to_rug_integer(x).is_power_of_two(), is_power_of_two);
        if *x != 0 {
            let trailing_zeros = x.trailing_zeros().unwrap();
            assert_eq!(trailing_zeros == x.significant_bits() - 1, is_power_of_two);
            if trailing_zeros <= u64::from(Limb::MAX) {
                assert_eq!(x >> trailing_zeros == 1, is_power_of_two);
            }
        }
    });

    test_properties(unsigneds::<Limb>, |&u| {
        assert_eq!(u.is_power_of_two(), Natural::from(u).is_power_of_two());
    });
}
