use malachite_base::num::arithmetic::traits::ShrRound;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::rounding_mode::RoundingMode;
use malachite_nz::natural::arithmetic::shr::{
    limbs_shr, limbs_shr_to_out, limbs_slice_shr_in_place, limbs_vec_shr_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_unsigned, pairs_of_unsigned_vec_and_small_unsigned,
    pairs_of_unsigned_vec_and_u64_var_2, signeds,
    triples_of_unsigned_vec_unsigned_vec_and_u64_var_6, unsigneds,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_small_signed, pairs_of_natural_and_small_unsigned,
    triples_of_natural_small_unsigned_and_small_unsigned,
};

#[test]
fn limbs_shr_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, bits)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_shr(limbs, bits)),
                Natural::from_limbs_asc(limbs) >> bits
            );
        },
    );
}

#[test]
fn limbs_shr_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_u64_var_6,
        |&(ref out, ref in_limbs, bits)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            let carry = limbs_shr_to_out(&mut out, in_limbs, bits);
            let n = Natural::from_limbs_asc(in_limbs);
            let m = &n >> bits;
            assert_eq!(carry == 0, &m << bits == n);
            let len = in_limbs.len();
            let mut limbs = m.into_limbs_asc();
            limbs.resize(len, 0);
            let actual_limbs = out[..len].to_vec();
            assert_eq!(limbs, actual_limbs);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_slice_shr_in_place_properties() {
    test_properties(pairs_of_unsigned_vec_and_u64_var_2, |&(ref limbs, bits)| {
        let mut limbs = limbs.to_vec();
        let old_limbs = limbs.clone();
        let carry = limbs_slice_shr_in_place(&mut limbs, bits);
        let n = Natural::from_limbs_asc(&old_limbs);
        let m = &n >> bits;
        assert_eq!(carry == 0, &m << bits == n);
        let mut expected_limbs = m.into_limbs_asc();
        expected_limbs.resize(limbs.len(), 0);
        assert_eq!(limbs, expected_limbs);
    });
}

#[test]
fn limbs_vec_shr_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, bits)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_vec_shr_in_place(&mut limbs, bits);
            let n = Natural::from_limbs_asc(&old_limbs) >> bits;
            assert_eq!(Natural::from_owned_limbs_asc(limbs), n);
        },
    );
}

macro_rules! tests_and_properties_unsigned {
    (
        $t:ident,
        $shr_u_properties:ident,
        $u:ident,
        $n:ident,
        $shifted:ident,
        $shl_library_comparison_properties:expr
    ) => {
        #[test]
        fn $shr_u_properties() {
            test_properties(
                pairs_of_natural_and_small_unsigned::<$t>,
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

                    assert!($shifted <= *$n);
                    assert_eq!($n.shr_round($u, RoundingMode::Floor), $shifted);

                    $shl_library_comparison_properties

                    if $u < $t::wrapping_from(<$t as PrimitiveUnsigned>::SignedOfEqualWidth::MAX) {
                        let u = <$t as PrimitiveUnsigned>::SignedOfEqualWidth::wrapping_from($u);
                        assert_eq!($n >> u, $shifted);
                        assert_eq!($n << -u, $shifted);
                    }
                },
            );

            test_properties(
                pairs_of_unsigned_and_small_unsigned::<Limb, $t>,
                |&(u, v)| {
                    if let Some(shift) = v.checked_add($t::exact_from(Limb::WIDTH)) {
                        assert_eq!(Natural::from(u) >> shift, 0);
                    }

                    if v < $t::exact_from(Limb::WIDTH) {
                        assert_eq!(u >> v, Natural::from(u) >> v);
                    }
                },
            );

            test_properties(
                triples_of_natural_small_unsigned_and_small_unsigned::<$t>,
                |&(ref n, u, v)| {
                    if let Some(sum) = u.checked_add(v) {
                        assert_eq!(n >> u >> v, n >> sum);
                    }
                },
            );

            #[allow(unknown_lints, identity_op)]
            test_properties(naturals, |n| {
                assert_eq!(n >> $t::ZERO, *n);
            });

            test_properties(unsigneds::<$t>, |&u| {
                assert_eq!(Natural::ZERO >> u, 0);
            });
        }
    };
}
tests_and_properties_unsigned!(u8, shr_u8_properties, u, n, shifted, {});
tests_and_properties_unsigned!(u16, shr_u16_properties, u, n, shifted, {});
tests_and_properties_unsigned!(u32, shr_u32_properties, u, n, shifted, {
    let mut rug_n = natural_to_rug_integer(n);
    rug_n >>= u;
    assert_eq!(rug_integer_to_natural(&rug_n), shifted);

    assert_eq!(
        biguint_to_natural(&(&natural_to_biguint(n) >> usize::exact_from(u))),
        shifted
    );
    assert_eq!(
        biguint_to_natural(&(natural_to_biguint(n) >> usize::exact_from(u))),
        shifted
    );
    assert_eq!(
        rug_integer_to_natural(&(natural_to_rug_integer(n) >> u)),
        shifted
    );
});
tests_and_properties_unsigned!(u64, shr_u64_properties, u, n, shifted, {});
tests_and_properties_unsigned!(usize, shr_usize_properties, u, n, shifted, {});

macro_rules! tests_and_properties_signed {
    (
        $t:ident,
        $shr_i_properties:ident,
        $i:ident,
        $n:ident,
        $shifted:ident,
        $shr_library_comparison_properties:expr
    ) => {
        #[test]
        fn $shr_i_properties() {
            test_properties(pairs_of_natural_and_small_signed::<$t>, |&(ref $n, $i)| {
                let mut mut_n = $n.clone();
                mut_n >>= $i;
                assert!(mut_n.is_valid());
                let $shifted = mut_n;

                let shifted_alt = $n >> $i;
                assert!(shifted_alt.is_valid());
                assert_eq!(shifted_alt, $shifted);

                let shifted_alt = $n.clone() >> $i;
                assert!(shifted_alt.is_valid());
                assert_eq!(shifted_alt, $shifted);

                assert_eq!($n.shr_round($i, RoundingMode::Floor), $shifted);

                $shr_library_comparison_properties
            });

            test_properties(naturals, |n| {
                assert_eq!(n >> $t::ZERO, *n);
            });

            test_properties(signeds::<$t>, |&i| {
                assert_eq!(Natural::ZERO >> i, 0);
            });
        }
    };
}
tests_and_properties_signed!(i8, shr_i8_properties, i, n, shifted, {});
tests_and_properties_signed!(i16, shr_i16_properties, i, n, shifted, {});
tests_and_properties_signed!(i32, shr_i32_properties, i, n, shifted, {
    let mut rug_n = natural_to_rug_integer(n);
    rug_n >>= i;
    assert_eq!(rug_integer_to_natural(&rug_n), shifted);

    assert_eq!(
        rug_integer_to_natural(&(natural_to_rug_integer(n) >> i)),
        shifted
    );
});
tests_and_properties_signed!(i64, shr_i64_properties, i, n, shifted, {});
tests_and_properties_signed!(isize, shr_isize_properties, i, n, shifted, {});
