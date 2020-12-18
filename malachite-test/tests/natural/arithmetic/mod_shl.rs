use malachite_base::num::arithmetic::traits::{ModIsReduced, ModNeg, ModShl, ModShlAssign, ModShr};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    small_signeds, small_unsigneds, triples_of_unsigned_signed_and_unsigned_var_1,
    triples_of_unsigned_unsigned_and_unsigned_var_1,
};
use malachite_test::inputs::natural::{
    pairs_of_naturals_var_1, pairs_of_positive_natural_and_signed,
    pairs_of_positive_natural_and_unsigned, triples_of_natural_small_signed_and_natural_var_1,
    triples_of_natural_small_unsigned_and_natural_var_1,
};

macro_rules! properties_unsigned {
    ($t:ident, $mod_shl_u_properties:ident) => {
        #[test]
        fn $mod_shl_u_properties() {
            test_properties(
                triples_of_natural_small_unsigned_and_natural_var_1::<$t>,
                |&(ref n, u, ref m)| {
                    assert!(n.mod_is_reduced(m));
                    let mut mut_n = n.clone();
                    mut_n.mod_shl_assign(u, m);
                    assert!(mut_n.is_valid());
                    let shifted = mut_n;
                    assert!(shifted.mod_is_reduced(m));

                    let mut mut_n = n.clone();
                    mut_n.mod_shl_assign(u, m.clone());
                    let shifted_alt = mut_n;
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);

                    let shifted_alt = n.mod_shl(u, m);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);
                    let shifted_alt = n.mod_shl(u, m.clone());
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);
                    let shifted_alt = n.clone().mod_shl(u, m);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);
                    let shifted_alt = n.clone().mod_shl(u, m.clone());
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);

                    assert_eq!((n << u) % m, shifted);
                    assert_eq!(n.mod_neg(m).mod_shl(u, m), n.mod_shl(u, m).mod_neg(m));
                },
            );

            test_properties(pairs_of_naturals_var_1, |&(ref n, ref m)| {
                assert_eq!(n.mod_shl($t::ZERO, m), *n);
            });

            test_properties(
                pairs_of_positive_natural_and_unsigned::<$t>,
                |&(ref m, u)| {
                    assert_eq!(Natural::ZERO.mod_shl(u, m), 0);
                },
            );

            test_properties_no_special(small_unsigneds::<$t>, |&u| {
                assert_eq!(Natural::ZERO.mod_shl(u, Natural::ONE), 0);
            });

            test_properties(
                triples_of_unsigned_unsigned_and_unsigned_var_1::<Limb, $t>,
                |&(n, u, m)| {
                    assert_eq!(
                        Natural::from(n).mod_shl(u, Natural::from(m)),
                        n.mod_shl(u, m)
                    );
                },
            );
        }
    };
}
properties_unsigned!(u8, mod_shl_u8_properties);
properties_unsigned!(u16, mod_shl_u16_properties);
properties_unsigned!(u32, mod_shl_u32_properties);
properties_unsigned!(u64, mod_shl_u64_properties);
properties_unsigned!(usize, mod_shl_usize_properties);

macro_rules! properties_signed {
    ($t:ident, $mod_shl_i_properties:ident) => {
        #[test]
        fn $mod_shl_i_properties() {
            test_properties(
                triples_of_natural_small_signed_and_natural_var_1::<$t>,
                |&(ref n, i, ref m)| {
                    assert!(n.mod_is_reduced(m));
                    let mut mut_n = n.clone();
                    mut_n.mod_shl_assign(i, m);
                    assert!(mut_n.is_valid());
                    let shifted = mut_n;
                    assert!(shifted.mod_is_reduced(m));

                    let mut mut_n = n.clone();
                    mut_n.mod_shl_assign(i, m.clone());
                    let shifted_alt = mut_n;
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);

                    let shifted_alt = n.mod_shl(i, m);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);
                    let shifted_alt = n.mod_shl(i, m.clone());
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);
                    let shifted_alt = n.clone().mod_shl(i, m);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);
                    let shifted_alt = n.clone().mod_shl(i, m.clone());
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);

                    assert_eq!((n << i) % m, shifted);

                    if i != $t::MIN {
                        assert_eq!(n.mod_shr(-i, m), shifted);
                    }
                },
            );

            test_properties(pairs_of_naturals_var_1, |&(ref n, ref m)| {
                assert_eq!(n.mod_shl($t::ZERO, m), *n);
            });

            test_properties(pairs_of_positive_natural_and_signed::<$t>, |&(ref m, i)| {
                assert_eq!(Natural::ZERO.mod_shl(i, m), 0);
            });

            test_properties_no_special(small_signeds::<$t>, |&i| {
                assert_eq!(Natural::ZERO.mod_shl(i, Natural::ONE), 0);
            });

            test_properties(
                triples_of_unsigned_signed_and_unsigned_var_1::<Limb, $t>,
                |&(n, i, m)| {
                    assert_eq!(
                        Natural::from(n).mod_shl(i, Natural::from(m)),
                        n.mod_shl(i, m)
                    );
                },
            );
        }
    };
}
properties_signed!(i8, mod_shl_i8_properties);
properties_signed!(i16, mod_shl_i16_properties);
properties_signed!(i32, mod_shl_i32_properties);
properties_signed!(i64, mod_shl_i64_properties);
properties_signed!(isize, mod_shl_isize_properties);
