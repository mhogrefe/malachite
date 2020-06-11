use std::str::FromStr;

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
use num::BigUint;
use rug;

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

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_shr_and_limbs_vec_shr_in_place() {
    let test = |limbs: &[Limb], bits: u64, out: &[Limb]| {
        assert_eq!(limbs_shr(limbs, bits), out);

        let mut limbs = limbs.to_vec();
        limbs_vec_shr_in_place(&mut limbs, bits);
        assert_eq!(limbs, out);
    };
    test(&[], 0, &[]);
    test(&[], 1, &[]);
    test(&[], 100, &[]);
    test(&[0, 0, 0], 0, &[0, 0, 0]);
    test(&[0, 0, 0], 1, &[0, 0, 0]);
    test(&[0, 0, 0], 100, &[]);
    test(&[1], 0, &[1]);
    test(&[1], 1, &[0]);
    test(&[3], 1, &[1]);
    test(&[122, 456], 1, &[61, 228]);
    test(&[123, 456], 0, &[123, 456]);
    test(&[123, 456], 1, &[61, 228]);
    test(&[123, 455], 1, &[2_147_483_709, 227]);
    test(&[123, 456], 31, &[912, 0]);
    test(&[123, 456], 32, &[456]);
    test(&[123, 456], 100, &[]);
    test(&[256, 456], 8, &[3_355_443_201, 1]);
    test(&[4_294_967_295, 1], 1, &[4_294_967_295, 0]);
    test(&[4_294_967_295, 4_294_967_295], 32, &[4_294_967_295]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_shr_to_out() {
    let test =
        |out_before: &[Limb], limbs_in: &[Limb], bits: u64, carry: Limb, out_after: &[Limb]| {
            let mut out = out_before.to_vec();
            assert_eq!(limbs_shr_to_out(&mut out, limbs_in, bits), carry);
            assert_eq!(out, out_after);
        };
    test(&[10, 10, 10, 10], &[0, 0, 0], 1, 0, &[0, 0, 0, 10]);
    test(&[10, 10, 10, 10], &[1], 1, 2_147_483_648, &[0, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[3], 1, 2_147_483_648, &[1, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[122, 456], 1, 0, &[61, 228, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        1,
        2_147_483_648,
        &[61, 228, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 455],
        1,
        2_147_483_648,
        &[2_147_483_709, 227, 10, 10],
    );
    test(&[10, 10, 10, 10], &[123, 456], 31, 246, &[912, 0, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[256, 456],
        8,
        0,
        &[3_355_443_201, 1, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[4_294_967_295, 1],
        1,
        2_147_483_648,
        &[4_294_967_295, 0, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[4_294_967_295, 4_294_967_295],
        31,
        4_294_967_294,
        &[4_294_967_295, 1, 10, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shr_to_out_fail_1() {
    limbs_shr_to_out(&mut [10, 10], &[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shr_to_out_fail_2() {
    limbs_shr_to_out(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shr_to_out_fail_3() {
    limbs_shr_to_out(&mut [10, 10, 10], &[123, 456], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shr_to_out_fail_4() {
    limbs_shr_to_out(&mut [10, 10, 10], &[123, 456], 100);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_shr_in_place() {
    let test = |limbs: &[Limb], bits: u64, carry: Limb, out: &[Limb]| {
        let mut limbs = limbs.to_vec();
        assert_eq!(limbs_slice_shr_in_place(&mut limbs, bits), carry);
        assert_eq!(limbs, out);
    };
    test(&[0, 0, 0], 1, 0, &[0, 0, 0]);
    test(&[1], 1, 2_147_483_648, &[0]);
    test(&[3], 1, 2_147_483_648, &[1]);
    test(&[122, 456], 1, 0, &[61, 228]);
    test(&[123, 456], 1, 2_147_483_648, &[61, 228]);
    test(&[123, 455], 1, 2_147_483_648, &[2_147_483_709, 227]);
    test(&[123, 456], 31, 246, &[912, 0]);
    test(&[256, 456], 8, 0, &[3_355_443_201, 1]);
    test(&[4_294_967_295, 1], 1, 2_147_483_648, &[4_294_967_295, 0]);
    test(
        &[4_294_967_295, 4_294_967_295],
        31,
        4_294_967_294,
        &[4_294_967_295, 1],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_shr_in_place_fail_1() {
    limbs_slice_shr_in_place(&mut [], 1);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_shr_in_place_fail_2() {
    limbs_slice_shr_in_place(&mut [123, 456], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_shr_in_place_fail_3() {
    limbs_slice_shr_in_place(&mut [123, 456], 100);
}

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
        $test_shr_u:ident,
        $shr_u_properties:ident,
        $u:ident,
        $v:ident,
        $out:ident,
        $shl_library_comparison_tests:expr,
        $n:ident,
        $shifted:ident,
        $shl_library_comparison_properties:expr
    ) => {
        #[test]
        fn $test_shr_u() {
            let test = |$u, $v: $t, $out| {
                let mut n = Natural::from_str($u).unwrap();
                n >>= $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = Natural::from_str($u).unwrap() >> $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = &Natural::from_str($u).unwrap() >> $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                $shl_library_comparison_tests
            };
            test("0", 0, "0");
            test("0", 10, "0");
            test("123", 0, "123");
            test("245", 1, "122");
            test("246", 1, "123");
            test("247", 1, "123");
            test("491", 2, "122");
            test("492", 2, "123");
            test("493", 2, "123");
            test("4127195135", 25, "122");
            test("4127195136", 25, "123");
            test("4127195137", 25, "123");
            test("8254390271", 26, "122");
            test("8254390272", 26, "123");
            test("8254390273", 26, "123");
            test("155921023828072216384094494261247", 100, "122");
            test("155921023828072216384094494261248", 100, "123");
            test("155921023828072216384094494261249", 100, "123");
            test("4294967295", 1, "2147483647");
            test("4294967296", 1, "2147483648");
            test("4294967297", 1, "2147483648");
            test("1000000000000", 0, "1000000000000");
            test("7999999999999", 3, "999999999999");
            test("8000000000000", 3, "1000000000000");
            test("8000000000001", 3, "1000000000000");
            test("16777216000000000000", 24, "1000000000000");
            test("33554432000000000000", 25, "1000000000000");
            test("2147483648000000000000", 31, "1000000000000");
            test("4294967296000000000000", 32, "1000000000000");
            test("8589934592000000000000", 33, "1000000000000");
            test(
                "1267650600228229401496703205376000000000000",
                100,
                "1000000000000",
            );
            test("1000000000000", 10, "976562500");
            test("980657949", 72, "0");
            test("4294967295", 31, "1");
            test("4294967295", 32, "0");
            test("4294967296", 32, "1");
            test("4294967296", 33, "0");
        }

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
tests_and_properties_unsigned!(
    u8,
    test_shr_u8,
    shr_u8_properties,
    u,
    v,
    out,
    {},
    n,
    shifted,
    {}
);
tests_and_properties_unsigned!(
    u16,
    test_shr_u16,
    shr_u16_properties,
    u,
    v,
    out,
    {},
    n,
    shifted,
    {}
);
tests_and_properties_unsigned!(
    u32,
    test_shr_u32,
    shr_u32_properties,
    u,
    v,
    out,
    {
        let mut n = rug::Integer::from_str(u).unwrap();
        n >>= v;
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() >> v;
        assert_eq!(n.to_string(), out);

        let n = BigUint::from_str(u).unwrap() >> usize::exact_from(v);
        assert_eq!(n.to_string(), out);

        let n = &BigUint::from_str(u).unwrap() >> usize::exact_from(v);
        assert_eq!(n.to_string(), out);
    },
    n,
    shifted,
    {
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
    }
);
tests_and_properties_unsigned!(
    u64,
    test_shr_u64,
    shr_u64_properties,
    u,
    v,
    out,
    {},
    n,
    shifted,
    {}
);
tests_and_properties_unsigned!(
    usize,
    test_shr_usize,
    shr_usize_properties,
    u,
    v,
    out,
    {},
    n,
    shifted,
    {}
);

macro_rules! tests_and_properties_signed {
    (
        $t:ident,
        $test_shr_i:ident,
        $shr_i_properties:ident,
        $i:ident,
        $j:ident,
        $out:ident,
        $shr_library_comparison_tests:expr,
        $n:ident,
        $shifted:ident,
        $shr_library_comparison_properties:expr
    ) => {
        #[test]
        fn $test_shr_i() {
            let test = |$i, $j: $t, $out| {
                let mut n = Natural::from_str($i).unwrap();
                n >>= $j;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = Natural::from_str($i).unwrap() >> $j;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = &Natural::from_str($i).unwrap() >> $j;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                $shr_library_comparison_tests
            };
            test("0", 0, "0");
            test("0", 10, "0");
            test("123", 0, "123");
            test("245", 1, "122");
            test("246", 1, "123");
            test("247", 1, "123");
            test("491", 2, "122");
            test("492", 2, "123");
            test("493", 2, "123");
            test("4127195135", 25, "122");
            test("4127195136", 25, "123");
            test("4127195137", 25, "123");
            test("8254390271", 26, "122");
            test("8254390272", 26, "123");
            test("8254390273", 26, "123");
            test("155921023828072216384094494261247", 100, "122");
            test("155921023828072216384094494261248", 100, "123");
            test("155921023828072216384094494261249", 100, "123");
            test("4294967295", 1, "2147483647");
            test("4294967296", 1, "2147483648");
            test("4294967297", 1, "2147483648");
            test("1000000000000", 0, "1000000000000");
            test("7999999999999", 3, "999999999999");
            test("8000000000000", 3, "1000000000000");
            test("8000000000001", 3, "1000000000000");
            test("16777216000000000000", 24, "1000000000000");
            test("33554432000000000000", 25, "1000000000000");
            test("2147483648000000000000", 31, "1000000000000");
            test("4294967296000000000000", 32, "1000000000000");
            test("8589934592000000000000", 33, "1000000000000");
            test(
                "1267650600228229401496703205376000000000000",
                100,
                "1000000000000",
            );
            test("1000000000000", 10, "976562500");
            test("980657949", 72, "0");
            test("4294967295", 31, "1");
            test("4294967295", 32, "0");
            test("4294967296", 32, "1");
            test("4294967296", 33, "0");

            test("0", 0, "0");
            test("0", -10, "0");
            test("123", 0, "123");
            test("123", -1, "246");
            test("123", -2, "492");
            test("123", -25, "4127195136");
            test("123", -26, "8254390272");
            test("123", -100, "155921023828072216384094494261248");
            test("2147483648", -1, "4294967296");
            test("1000000000000", 0, "1000000000000");
            test("1000000000000", -3, "8000000000000");
            test("1000000000000", -24, "16777216000000000000");
            test("1000000000000", -25, "33554432000000000000");
            test("1000000000000", -31, "2147483648000000000000");
            test("1000000000000", -32, "4294967296000000000000");
            test("1000000000000", -33, "8589934592000000000000");
            test(
                "1000000000000",
                -100,
                "1267650600228229401496703205376000000000000",
            );
        }

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
tests_and_properties_signed!(
    i8,
    test_shr_i8,
    shr_i8_properties,
    i,
    v,
    out,
    {},
    n,
    shifted,
    {}
);
tests_and_properties_signed!(
    i16,
    test_shr_i16,
    shr_i16_properties,
    i,
    v,
    out,
    {},
    n,
    shifted,
    {}
);
tests_and_properties_signed!(
    i32,
    test_shr_i32,
    shr_i32_properties,
    i,
    j,
    out,
    {
        let mut n = rug::Integer::from_str(i).unwrap();
        n >>= j;
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(i).unwrap() >> j;
        assert_eq!(n.to_string(), out);
    },
    n,
    shifted,
    {
        let mut rug_n = natural_to_rug_integer(n);
        rug_n >>= i;
        assert_eq!(rug_integer_to_natural(&rug_n), shifted);

        assert_eq!(
            rug_integer_to_natural(&(natural_to_rug_integer(n) >> i)),
            shifted
        );
    }
);
tests_and_properties_signed!(
    i64,
    test_shr_i64,
    shr_i64_properties,
    i,
    v,
    out,
    {},
    n,
    shifted,
    {}
);
tests_and_properties_signed!(
    isize,
    test_shr_isize,
    shr_isize_properties,
    i,
    v,
    out,
    {},
    n,
    shifted,
    {}
);
