use malachite_base::num::arithmetic::traits::{
    ModNeg, ModPowerOfTwo, ModPowerOfTwoAdd, ModPowerOfTwoIsReduced, ModPowerOfTwoNeg,
    ModPowerOfTwoNegAssign, PowerOfTwo,
};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::pairs_of_unsigned_and_small_u64_var_2;
use malachite_test::inputs::natural::pairs_of_natural_and_u64_var_1;

#[test]
fn mod_power_of_two_neg_properties() {
    test_properties(pairs_of_natural_and_u64_var_1, |&(ref n, pow)| {
        assert!(n.mod_power_of_two_is_reduced(pow));
        let neg = n.mod_power_of_two_neg(pow);
        assert!(neg.is_valid());
        assert!(neg.mod_power_of_two_is_reduced(pow));

        let neg_alt = n.clone().mod_power_of_two_neg(pow);
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        let mut n_alt = n.clone();
        n_alt.mod_power_of_two_neg_assign(pow);
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        assert_eq!(neg, (-n).mod_power_of_two(pow));
        assert_eq!(neg, n.mod_neg(Natural::power_of_two(pow)));
        assert_eq!((&neg).mod_power_of_two_neg(pow), *n);
        assert_eq!(n.mod_power_of_two_add(&neg, pow), 0);
        assert_eq!(
            *n == neg,
            *n == Natural::ZERO || *n == Natural::power_of_two(pow - 1)
        );
    });

    test_properties_no_special(
        pairs_of_unsigned_and_small_u64_var_2::<Limb>,
        |&(n, pow)| {
            assert_eq!(
                n.mod_power_of_two_neg(pow),
                Natural::from(n).mod_power_of_two_neg(pow)
            );
        },
    );
}
