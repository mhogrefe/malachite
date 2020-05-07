use malachite_base::num::arithmetic::traits::ShrRound;
use malachite_base::num::basic::traits::Zero;
use malachite_base::round::RoundingMode;
use malachite_nz::integer::Integer;

use malachite_test::common::test_properties;
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::signeds;
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_small_signed};
use malachite_test::inputs::natural::pairs_of_natural_and_small_signed;

macro_rules! tests_and_properties {
    (
        $t:ident,
        $shr_i_properties:ident,
        $n:ident,
        $i:ident,
        $shifted:ident,
        $shr_library_comparison_properties:expr
    ) => {
        #[test]
        fn $shr_i_properties() {
            test_properties(pairs_of_integer_and_small_signed::<$t>, |&(ref $n, $i)| {
                let mut mut_n = $n.clone();
                mut_n >>= $i;
                assert!(mut_n.is_valid());
                let $shifted = mut_n;

                let shifted_alt = $n >> $i;
                assert_eq!(shifted_alt, $shifted);
                assert!(shifted_alt.is_valid());
                let shifted_alt = $n.clone() >> $i;
                assert_eq!(shifted_alt, $shifted);
                assert!(shifted_alt.is_valid());

                assert_eq!($n.shr_round($i, RoundingMode::Floor), $shifted);

                $shr_library_comparison_properties
            });

            test_properties(integers, |n| {
                assert_eq!(n >> $t::ZERO, *n);
            });

            test_properties(signeds::<$t>, |&i| {
                assert_eq!(Integer::ZERO >> i, 0);
            });

            test_properties(pairs_of_natural_and_small_signed::<$t>, |&(ref n, i)| {
                assert_eq!(n >> i, Integer::from(n) >> i);
            });
        }
    };
}
tests_and_properties!(i8, shr_i8_properties, n, i, shifted, {});
tests_and_properties!(i16, shr_i16_properties, n, i, shifted, {});
tests_and_properties!(i32, shr_i32_properties, n, i, shifted, {
    let mut rug_n = integer_to_rug_integer(n);
    rug_n >>= i;
    assert_eq!(rug_integer_to_integer(&rug_n), shifted);

    assert_eq!(
        rug_integer_to_integer(&(integer_to_rug_integer(n) >> i)),
        shifted
    );
});
tests_and_properties!(i64, shr_i64_properties, n, i, shifted, {});
tests_and_properties!(isize, shr_isize_properties, n, i, shifted, {});
