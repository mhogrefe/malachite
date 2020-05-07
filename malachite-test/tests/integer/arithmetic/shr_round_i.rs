use malachite_base::num::arithmetic::traits::{ArithmeticCheckedShr, ShrRound, ShrRoundAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signed_and_rounding_mode, triples_of_signed_small_signed_and_rounding_mode_var_1,
};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_rounding_mode, triples_of_integer_small_signed_and_rounding_mode_var_2,
};
use malachite_test::inputs::natural::triples_of_natural_small_signed_and_rounding_mode_var_2;

macro_rules! tests_and_properties {
    (
        $t:ident,
        $shr_round_i_properties:ident
    ) => {
        #[test]
        fn $shr_round_i_properties() {
            test_properties(
                triples_of_integer_small_signed_and_rounding_mode_var_2::<$t>,
                |&(ref n, i, rm)| {
                    let mut mut_n = n.clone();
                    mut_n.shr_round_assign(i, rm);
                    assert!(mut_n.is_valid());
                    let shifted = mut_n;

                    let shifted_alt = n.shr_round(i, rm);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);
                    let shifted_alt = n.clone().shr_round(i, rm);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);

                    assert_eq!(-(-n).shr_round(i, -rm), shifted);
                },
            );

            test_properties(pairs_of_integer_and_rounding_mode, |&(ref n, rm)| {
                assert_eq!(n.shr_round($t::ZERO, rm), *n);
            });

            test_properties(pairs_of_signed_and_rounding_mode::<$t>, |&(i, rm)| {
                assert_eq!(Integer::ZERO.shr_round(i, rm), 0);
            });

            test_properties(
                triples_of_natural_small_signed_and_rounding_mode_var_2::<$t>,
                |&(ref n, i, rm)| {
                    assert_eq!(n.shr_round(i, rm), Integer::from(n).shr_round(i, rm));
                },
            );

            test_properties(
                triples_of_signed_small_signed_and_rounding_mode_var_1::<SignedLimb, $t>,
                |&(n, i, rm)| {
                    if n.arithmetic_checked_shr(i).is_some() {
                        assert_eq!(n.shr_round(i, rm), Integer::from(n).shr_round(i, rm));
                    }
                },
            );
        }
    };
}
tests_and_properties!(i8, shr_round_i8_properties);
tests_and_properties!(i16, shr_round_i16_properties);
tests_and_properties!(i32, shr_round_i32_properties);
tests_and_properties!(i64, shr_round_i64_properties);
tests_and_properties!(isize, shr_round_isize_properties);
