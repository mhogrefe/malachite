use malachite_base::num::arithmetic::traits::{ArithmeticCheckedShl, ShlRound, ShlRoundAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signed_and_rounding_mode, triples_of_unsigned_small_signed_and_rounding_mode_var_2,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_rounding_mode, triples_of_natural_small_signed_and_rounding_mode_var_1,
};

macro_rules! tests_and_properties_signed {
    (
        $t:ident,
        $shl_round_i_properties:ident
    ) => {
        #[test]
        fn $shl_round_i_properties() {
            test_properties(
                triples_of_natural_small_signed_and_rounding_mode_var_1::<$t>,
                |&(ref n, i, rm)| {
                    let mut mut_n = n.clone();
                    mut_n.shl_round_assign(i, rm);
                    assert!(mut_n.is_valid());
                    let shifted = mut_n;

                    let shifted_alt = n.shl_round(i, rm);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);

                    let shifted_alt = n.clone().shl_round(i, rm);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);
                },
            );

            test_properties(pairs_of_natural_and_rounding_mode, |&(ref n, rm)| {
                assert_eq!(n.shl_round($t::ZERO, rm), *n);
            });

            test_properties(pairs_of_signed_and_rounding_mode::<$t>, |&(i, rm)| {
                assert_eq!(Natural::ZERO.shl_round(i, rm), 0);
            });

            test_properties(
                triples_of_unsigned_small_signed_and_rounding_mode_var_2::<Limb, $t>,
                |&(n, i, rm)| {
                    if n.arithmetic_checked_shl(i).is_some() {
                        assert_eq!(n.shl_round(i, rm), Natural::from(n).shl_round(i, rm));
                    }
                },
            );
        }
    };
}
tests_and_properties_signed!(i8, shl_round_i8_properties);
tests_and_properties_signed!(i16, shl_round_i16_properties);
tests_and_properties_signed!(i32, shl_round_i32_properties);
tests_and_properties_signed!(i64, shl_round_i64_properties);
tests_and_properties_signed!(isize, shl_round_isize_properties);
