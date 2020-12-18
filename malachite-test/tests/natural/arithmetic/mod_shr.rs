use malachite_base::num::arithmetic::traits::{Mod, ModIsReduced, ModShl, ModShr, ModShrAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{small_signeds, triples_of_unsigned_signed_and_unsigned_var_1};
use malachite_test::inputs::natural::{
    pairs_of_naturals_var_1, pairs_of_positive_natural_and_signed,
    triples_of_natural_small_signed_and_natural_var_1,
};

macro_rules! properties_signed {
    ($t:ident, $mod_shr_i_properties:ident) => {
        #[test]
        fn $mod_shr_i_properties() {
            test_properties(
                triples_of_natural_small_signed_and_natural_var_1::<$t>,
                |&(ref n, i, ref m)| {
                    assert!(n.mod_is_reduced(m));
                    let mut mut_n = n.clone();
                    mut_n.mod_shr_assign(i, m);
                    assert!(mut_n.is_valid());
                    let shifted = mut_n;
                    assert!(shifted.mod_is_reduced(m));

                    let mut mut_n = n.clone();
                    mut_n.mod_shr_assign(i, m.clone());
                    let shifted_alt = mut_n;
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);

                    let shifted_alt = n.mod_shr(i, m);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);
                    let shifted_alt = n.mod_shr(i, m.clone());
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);
                    let shifted_alt = n.clone().mod_shr(i, m);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);
                    let shifted_alt = n.clone().mod_shr(i, m.clone());
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);

                    assert_eq!((n >> i).mod_op(m), shifted);

                    if i != $t::MIN {
                        assert_eq!(n.mod_shl(-i, m), shifted);
                    }
                },
            );

            test_properties(pairs_of_naturals_var_1, |&(ref n, ref m)| {
                assert_eq!(n.mod_shr($t::ZERO, m), *n);
            });

            test_properties(pairs_of_positive_natural_and_signed::<$t>, |&(ref m, i)| {
                assert_eq!(Natural::ZERO.mod_shr(i, m), 0);
            });

            test_properties_no_special(small_signeds::<$t>, |&i| {
                assert_eq!(Natural::ZERO.mod_shr(i, Natural::ONE), 0);
            });

            test_properties(
                triples_of_unsigned_signed_and_unsigned_var_1::<Limb, $t>,
                |&(n, i, m)| {
                    assert_eq!(
                        Natural::from(n).mod_shr(i, Natural::from(m)),
                        n.mod_shr(i, m)
                    );
                },
            );
        }
    };
}
properties_signed!(i8, mod_shr_i8_properties);
properties_signed!(i16, mod_shr_i16_properties);
properties_signed!(i32, mod_shr_i32_properties);
properties_signed!(i64, mod_shr_i64_properties);
properties_signed!(isize, mod_shr_isize_properties);
