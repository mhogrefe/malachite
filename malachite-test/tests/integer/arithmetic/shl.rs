use malachite_base::num::arithmetic::traits::{Abs, IsPowerOfTwo, ShlRound};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::round::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{signeds, small_unsigneds};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_small_signed, pairs_of_integer_and_small_unsigned,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_small_signed, pairs_of_natural_and_small_unsigned,
};

macro_rules! properties_unsigned {
    (
        $t:ident,
        $shl_u_properties:ident,
        $n:ident,
        $u:ident,
        $shifted:ident,
        $library_comparison_properties:expr
    ) => {
        #[test]
        fn $shl_u_properties() {
            test_properties(
                pairs_of_integer_and_small_unsigned::<$t>,
                |&(ref $n, $u)| {
                    let mut mut_n = $n.clone();
                    mut_n <<= $u;
                    assert!(mut_n.is_valid());
                    let $shifted = mut_n;

                    let shifted_alt = $n << $u;
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, $shifted);
                    let shifted_alt = $n.clone() << $u;
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, $shifted);

                    assert!(($n << $u).abs() >= $n.abs());
                    assert_eq!(-$n << $u, -($n << $u));

                    assert_eq!($n << $u, $n * (Integer::ONE << $u));
                    assert_eq!($n << $u >> $u, *$n);

                    if $u < $t::wrapping_from(<$t as PrimitiveUnsigned>::SignedOfEqualWidth::MAX) {
                        let u = <$t as PrimitiveUnsigned>::SignedOfEqualWidth::wrapping_from($u);
                        assert_eq!($n << u, $shifted);
                        assert_eq!($n >> -u, $shifted);
                    }

                    $library_comparison_properties
                },
            );

            #[allow(unknown_lints, identity_op)]
            test_properties(integers, |n| {
                assert_eq!(n << $t::ZERO, *n);
            });

            test_properties_no_special(small_unsigneds::<$t>, |&u| {
                assert_eq!(Integer::ZERO << u, 0);
                assert!(Natural::exact_from(Integer::ONE << u).is_power_of_two());
            });

            test_properties(pairs_of_natural_and_small_unsigned::<$t>, |&(ref n, u)| {
                assert_eq!(n << u, Integer::from(n) << u);
            });
        }
    };
}
properties_unsigned!(u8, shl_u8_properties, n, u, shifted, {});
properties_unsigned!(u16, shl_u16_properties, n, u, shifted, {});
properties_unsigned!(u32, shl_limb_properties, n, u, shifted, {
    let mut rug_n = integer_to_rug_integer(n);
    rug_n <<= u;
    assert_eq!(rug_integer_to_integer(&rug_n), shifted);

    assert_eq!(
        bigint_to_integer(&(&integer_to_bigint(n) << usize::exact_from(u))),
        shifted
    );
    assert_eq!(
        bigint_to_integer(&(integer_to_bigint(n) << usize::exact_from(u))),
        shifted
    );

    assert_eq!(
        rug_integer_to_integer(&(integer_to_rug_integer(n) << u)),
        shifted
    );
});
properties_unsigned!(u64, shl_u64_properties, n, u, shifted, {});
properties_unsigned!(usize, shl_usize_properties, n, u, shifted, {});

macro_rules! properties_signed {
    (
        $t:ident,
        $shl_i_properties:ident,
        $i:ident,
        $n:ident,
        $shifted:ident,
        $shl_library_comparison_properties:expr
    ) => {
        #[test]
        fn $shl_i_properties() {
            test_properties(
                pairs_of_integer_and_small_signed,
                |&(ref $n, $i): &(Integer, $t)| {
                    let mut mut_n = $n.clone();
                    mut_n <<= $i;
                    assert!(mut_n.is_valid());
                    let $shifted = mut_n;

                    let shifted_alt = $n << $i;
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, $shifted);
                    let shifted_alt = $n.clone() << $i;
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, $shifted);

                    assert_eq!($n.shl_round($i, RoundingMode::Floor), $shifted);

                    $shl_library_comparison_properties
                },
            );

            #[allow(unknown_lints, identity_op)]
            test_properties(integers, |n| {
                assert_eq!(n << $t::ZERO, *n);
            });

            test_properties(signeds::<$t>, |&i| {
                assert_eq!(Integer::ZERO << i, 0);
            });

            test_properties(pairs_of_natural_and_small_signed::<$t>, |&(ref n, i)| {
                assert_eq!(n << i, Integer::from(n) << i);
            });
        }
    };
}
properties_signed!(i8, shl_i8_properties, i, n, shifted, {});
properties_signed!(i16, shl_i16_properties, i, n, shifted, {});
properties_signed!(i32, shl_i32_properties, i, n, shifted, {
    let mut rug_n = integer_to_rug_integer(n);
    rug_n <<= i;
    assert_eq!(rug_integer_to_integer(&rug_n), shifted);

    assert_eq!(
        rug_integer_to_integer(&(integer_to_rug_integer(n) << i)),
        shifted
    );
});
properties_signed!(i64, shl_i64_properties, i, n, shifted, {});
properties_signed!(isize, shl_isize_properties, i, n, shifted, {});
