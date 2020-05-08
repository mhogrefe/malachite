use malachite_base::num::arithmetic::traits::{Abs, IsPowerOfTwo};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::small_unsigneds;
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_small_unsigned};
use malachite_test::inputs::natural::pairs_of_natural_and_small_unsigned;

macro_rules! properties {
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
properties!(u8, shl_u8_properties, n, u, shifted, {});
properties!(u16, shl_u16_properties, n, u, shifted, {});
properties!(u32, shl_limb_properties, n, u, shifted, {
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
properties!(u64, shl_u64_properties, n, u, shifted, {});
properties!(usize, shl_usize_properties, n, u, shifted, {});
