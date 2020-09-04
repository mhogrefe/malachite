use malachite_base::num::arithmetic::traits::ShrRound;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_signed_and_small_unsigned, signeds, unsigneds};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_small_signed, pairs_of_integer_and_small_unsigned,
    triples_of_integer_small_unsigned_and_small_unsigned,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_small_signed, pairs_of_natural_and_small_unsigned,
};

macro_rules! properties_unsigned {
    (
        $t:ident,
        $shr_u_properties:ident,
        $n:ident,
        $u:ident,
        $shifted:ident,
        $shl_library_comparison_properties:expr
    ) => {
        #[test]
        fn $shr_u_properties() {
            test_properties(
                pairs_of_integer_and_small_unsigned::<$t>,
                |&(ref $n, $u)| {
                    let mut mut_n = $n.clone();
                    mut_n >>= $u;
                    assert!(mut_n.is_valid());
                    let $shifted = mut_n;

                    let shifted_alt = $n >> $u;
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, $shifted);
                    let shifted_alt = $n.clone() >> $u;
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, $shifted);

                    assert!($shifted.le_abs($n));
                    assert_eq!($n.shr_round($u, RoundingMode::Floor), $shifted);

                    if $u < $t::wrapping_from(<$t as PrimitiveUnsigned>::SignedOfEqualWidth::MAX) {
                        let u = <$t as PrimitiveUnsigned>::SignedOfEqualWidth::wrapping_from($u);
                        assert_eq!($n >> u, $shifted);
                        assert_eq!($n << -u, $shifted);
                    }

                    $shl_library_comparison_properties
                },
            );

            test_properties(
                pairs_of_signed_and_small_unsigned::<SignedLimb, $t>,
                |&(i, j)| {
                    if let Some(sum) = j.checked_add($t::exact_from(SignedLimb::WIDTH)) {
                        let shifted = Integer::from(i) >> sum;
                        if i >= 0 {
                            assert_eq!(shifted, 0);
                        } else {
                            assert_eq!(shifted, -1);
                        }
                    }

                    if j < $t::exact_from(SignedLimb::WIDTH) {
                        assert_eq!(i >> j, Integer::from(i) >> j);
                    }
                },
            );

            test_properties(
                triples_of_integer_small_unsigned_and_small_unsigned::<$t, $t>,
                |&(ref n, u, v)| {
                    if let Some(sum) = u.checked_add(v) {
                        assert_eq!(n >> u >> v, n >> sum);
                    }
                },
            );

            test_properties(integers, |n| {
                assert_eq!(n >> $t::ZERO, *n);
            });

            test_properties(unsigneds::<$t>, |&u| {
                assert_eq!(Integer::ZERO >> u, 0);
            });

            test_properties(pairs_of_natural_and_small_unsigned::<$t>, |&(ref n, u)| {
                assert_eq!(n >> u, Integer::from(n) >> u);
            });
        }
    };
}
properties_unsigned!(u8, shr_u8_properties, n, u, shifted, {});
properties_unsigned!(u16, shr_u16_properties, n, u, shifted, {});
properties_unsigned!(u32, shr_u32_properties, n, u, shifted, {
    let mut rug_n = integer_to_rug_integer(n);
    rug_n >>= u;
    assert_eq!(rug_integer_to_integer(&rug_n), shifted);

    assert_eq!(
        rug_integer_to_integer(&(integer_to_rug_integer(n) >> u)),
        shifted
    );

    assert_eq!(
        bigint_to_integer(&(&integer_to_bigint(n) >> usize::exact_from(u))),
        shifted
    );
    assert_eq!(
        bigint_to_integer(&(integer_to_bigint(n) >> usize::exact_from(u))),
        shifted
    );
});
properties_unsigned!(u64, shr_u64_properties, n, u, shifted, {});
properties_unsigned!(usize, shr_usize_properties, n, u, shifted, {});

macro_rules! properties_signed {
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
properties_signed!(i8, shr_i8_properties, n, i, shifted, {});
properties_signed!(i16, shr_i16_properties, n, i, shifted, {});
properties_signed!(i32, shr_i32_properties, n, i, shifted, {
    let mut rug_n = integer_to_rug_integer(n);
    rug_n >>= i;
    assert_eq!(rug_integer_to_integer(&rug_n), shifted);

    assert_eq!(
        rug_integer_to_integer(&(integer_to_rug_integer(n) >> i)),
        shifted
    );
});
properties_signed!(i64, shr_i64_properties, n, i, shifted, {});
properties_signed!(isize, shr_isize_properties, n, i, shifted, {});
