use malachite_base::num::arithmetic::traits::{
    IsPowerOfTwo, ModPowerOfTwo, ModPowerOfTwoIsReduced, ModPowerOfTwoShl, ModPowerOfTwoShr,
    ModPowerOfTwoShrAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    pairs_of_small_signed_and_small_unsigned,
    triples_of_unsigned_small_signed_and_small_unsigned_var_1,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_u64_var_1, triples_of_natural_small_signed_and_u64_var_1,
};

macro_rules! properties_signed {
    ($t:ident, $mod_power_of_two_shr_i_properties:ident) => {
        #[test]
        fn $mod_power_of_two_shr_i_properties() {
            test_properties(
                triples_of_natural_small_signed_and_u64_var_1::<$t>,
                |&(ref n, i, pow)| {
                    assert!(n.mod_power_of_two_is_reduced(pow));
                    let mut mut_n = n.clone();
                    mut_n.mod_power_of_two_shr_assign(i, pow);
                    assert!(mut_n.is_valid());
                    let shifted = mut_n;
                    assert!(shifted.mod_power_of_two_is_reduced(pow));

                    let shifted_alt = n.mod_power_of_two_shr(i, pow);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);
                    let shifted_alt = n.clone().mod_power_of_two_shr(i, pow);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);

                    assert_eq!((n >> i).mod_power_of_two(pow), shifted);

                    if i != $t::MIN {
                        assert_eq!(n.mod_power_of_two_shl(-i, pow), shifted);
                    }
                },
            );

            test_properties(pairs_of_natural_and_u64_var_1, |&(ref n, pow)| {
                assert_eq!(n.mod_power_of_two_shr($t::ZERO, pow), *n);
            });

            test_properties_no_special(
                pairs_of_small_signed_and_small_unsigned::<$t, u64>,
                |&(i, pow)| {
                    assert_eq!(Natural::ZERO.mod_power_of_two_shr(i, pow), 0);
                    if pow != 0 {
                        let shifted = Natural::ONE.mod_power_of_two_shr(i, pow);
                        assert!(shifted == 0 || shifted.is_power_of_two());
                    }
                },
            );

            test_properties_no_special(
                triples_of_unsigned_small_signed_and_small_unsigned_var_1::<Limb, $t>,
                |&(n, i, pow)| {
                    assert_eq!(
                        Natural::from(n).mod_power_of_two_shr(i, pow),
                        n.mod_power_of_two_shr(i, pow)
                    );
                },
            );
        }
    };
}
properties_signed!(i8, mod_power_of_two_shr_i8_properties);
properties_signed!(i16, mod_power_of_two_shr_i16_properties);
properties_signed!(i32, mod_power_of_two_shr_i32_properties);
properties_signed!(i64, mod_power_of_two_shr_i64_properties);
properties_signed!(isize, mod_power_of_two_shr_isize_properties);
