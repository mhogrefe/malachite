use std::str::FromStr;

use malachite_base::num::conversion::traits::ExactFrom;
use num::BigUint;
use rug;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::shl::{
    limbs_shl, limbs_shl_to_out, limbs_shl_with_complement_to_out, limbs_slice_shl_in_place,
    limbs_vec_shl_in_place,
};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_shl_and_limbs_vec_shl_in_place() {
    let test = |limbs: &[Limb], bits: u64, out: &[Limb]| {
        assert_eq!(limbs_shl(limbs, bits), out);

        let mut limbs = limbs.to_vec();
        limbs_vec_shl_in_place(&mut limbs, bits);
        assert_eq!(limbs, out);
    };
    test(&[], 0, &[]);
    test(&[], 5, &[]);
    test(&[], 100, &[0, 0, 0]);
    test(&[6, 7], 2, &[24, 28]);
    test(&[100, 101, 102], 10, &[102_400, 103_424, 104_448]);
    test(&[123, 456], 1, &[246, 912]);
    test(&[123, 456], 31, &[0x8000_0000, 61, 228]);
    test(&[123, 456], 32, &[0, 123, 456]);
    test(&[123, 456], 100, &[0, 0, 0, 1_968, 7_296]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_shl_to_out() {
    let test =
        |out_before: &[Limb], limbs_in: &[Limb], bits: u64, carry: Limb, out_after: &[Limb]| {
            let mut out = out_before.to_vec();
            assert_eq!(limbs_shl_to_out(&mut out, limbs_in, bits), carry);
            assert_eq!(out, out_after);
        };
    test(&[10, 10, 10, 10], &[], 5, 0, &[10, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[6, 7], 2, 0, &[24, 28, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        0,
        &[102_400, 103_424, 104_448, 10],
    );
    test(&[10, 10, 10, 10], &[123, 456], 1, 0, &[246, 912, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        31,
        228,
        &[0x8000_0000, 61, 10, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shl_to_out_fail_1() {
    limbs_shl_to_out(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shl_to_out_fail_2() {
    limbs_shl_to_out(&mut [10, 10, 10], &[123, 456], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shl_to_out_fail_3() {
    limbs_shl_to_out(&mut [10, 10, 10], &[123, 456], 100);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_shl_in_place() {
    let test = |limbs: &[Limb], bits: u64, carry: Limb, out: &[Limb]| {
        let mut limbs = limbs.to_vec();
        assert_eq!(limbs_slice_shl_in_place(&mut limbs, bits), carry);
        assert_eq!(limbs, out);
    };
    test(&[], 5, 0, &[]);
    test(&[6, 7], 2, 0, &[24, 28]);
    test(&[100, 101, 102], 10, 0, &[102_400, 103_424, 104_448]);
    test(&[123, 456], 1, 0, &[246, 912]);
    test(&[123, 456], 31, 228, &[0x8000_0000, 61]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_shl_in_place_fail_1() {
    limbs_slice_shl_in_place(&mut [123, 456], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_shl_in_place_fail_2() {
    limbs_slice_shl_in_place(&mut [123, 456], 100);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_shl_with_complement_to_out() {
    let test =
        |out_before: &[Limb], limbs_in: &[Limb], bits: u64, carry: Limb, out_after: &[Limb]| {
            let mut out = out_before.to_vec();
            assert_eq!(
                limbs_shl_with_complement_to_out(&mut out, limbs_in, bits),
                carry
            );
            assert_eq!(out, out_after);
        };
    test(
        &[10, 10, 10, 10],
        &[6, 7],
        2,
        0,
        &[4_294_967_271, 4_294_967_267, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        0,
        &[4_294_864_895, 4_294_863_871, 4_294_862_847, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        1,
        0,
        &[4_294_967_049, 4_294_966_383, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        31,
        228,
        &[0x7fff_ffff, 4_294_967_234, 10, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shl_with_complement_to_out_fail_1() {
    limbs_shl_with_complement_to_out(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shl_with_complement_to_out_fail_2() {
    limbs_shl_with_complement_to_out(&mut [10, 10, 10], &[123, 456], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shl_with_complement_to_out_fail_3() {
    limbs_shl_with_complement_to_out(&mut [10, 10, 10], &[123, 456], 100);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shl_with_complement_to_out_fail_4() {
    limbs_shl_with_complement_to_out(&mut [10, 10, 10], &[], 100);
}

macro_rules! tests_and_properties_unsigned {
    (
        $t: ident,
        $test_shl_u: ident,
        $u: ident,
        $v: ident,
        $out: ident,
        $library_comparison_tests: expr
    ) => {
        #[test]
        fn $test_shl_u() {
            let test = |$u, $v: $t, $out| {
                let mut n = Natural::from_str($u).unwrap();
                n <<= $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = Natural::from_str($u).unwrap() << $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = &Natural::from_str($u).unwrap() << $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                $library_comparison_tests
            };
            test("0", 0, "0");
            test("0", 10, "0");
            test("123", 0, "123");
            test("123", 1, "246");
            test("123", 2, "492");
            test("123", 25, "4127195136");
            test("123", 26, "8254390272");
            test("123", 100, "155921023828072216384094494261248");
            test("2147483648", 1, "4294967296");
            test("1000000000000", 0, "1000000000000");
            test("1000000000000", 3, "8000000000000");
            test("1000000000000", 24, "16777216000000000000");
            test("1000000000000", 25, "33554432000000000000");
            test("1000000000000", 31, "2147483648000000000000");
            test("1000000000000", 32, "4294967296000000000000");
            test("1000000000000", 33, "8589934592000000000000");
            test(
                "1000000000000",
                100,
                "1267650600228229401496703205376000000000000",
            );
        }
    };
}
tests_and_properties_unsigned!(u8, test_shl_u8, u, v, out, {});
tests_and_properties_unsigned!(u16, test_shl_u16, u, v, out, {});
tests_and_properties_unsigned!(u32, test_shl_limb, u, v, out, {
    let mut n = rug::Integer::from_str(u).unwrap();
    n <<= v;
    assert_eq!(n.to_string(), out);

    let n = rug::Integer::from_str(u).unwrap() << v;
    assert_eq!(n.to_string(), out);

    let n = BigUint::from_str(u).unwrap() << usize::exact_from(v);
    assert_eq!(n.to_string(), out);

    let n = &BigUint::from_str(u).unwrap() << usize::exact_from(v);
    assert_eq!(n.to_string(), out);
});
tests_and_properties_unsigned!(u64, test_shl_u64, u, v, out, {});
tests_and_properties_unsigned!(u128, test_shl_u128, u, v, out, {});
tests_and_properties_unsigned!(usize, test_shl_usize, u, v, out, {});

macro_rules! tests_and_properties_signed {
    (
        $t:ident,
        $test_shl_i:ident,
        $i:ident,
        $j:ident,
        $out:ident,
        $shl_library_comparison_tests:expr
    ) => {
        #[test]
        fn $test_shl_i() {
            let test = |$i, $j: $t, $out| {
                let mut n = Natural::from_str($i).unwrap();
                n <<= $j;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = Natural::from_str($i).unwrap() << $j;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = &Natural::from_str($i).unwrap() << $j;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                $shl_library_comparison_tests
            };
            test("0", 0, "0");
            test("0", 10, "0");
            test("123", 0, "123");
            test("123", 1, "246");
            test("123", 2, "492");
            test("123", 25, "4127195136");
            test("123", 26, "8254390272");
            test("123", 100, "155921023828072216384094494261248");
            test("2147483648", 1, "4294967296");
            test("1000000000000", 0, "1000000000000");
            test("1000000000000", 3, "8000000000000");
            test("1000000000000", 24, "16777216000000000000");
            test("1000000000000", 25, "33554432000000000000");
            test("1000000000000", 31, "2147483648000000000000");
            test("1000000000000", 32, "4294967296000000000000");
            test("1000000000000", 33, "8589934592000000000000");
            test(
                "1000000000000",
                100,
                "1267650600228229401496703205376000000000000",
            );

            test("0", -10, "0");
            test("123", 0, "123");
            test("245", -1, "122");
            test("246", -1, "123");
            test("247", -1, "123");
            test("491", -2, "122");
            test("492", -2, "123");
            test("493", -2, "123");
            test("4127195135", -25, "122");
            test("4127195136", -25, "123");
            test("4127195137", -25, "123");
            test("8254390271", -26, "122");
            test("8254390272", -26, "123");
            test("8254390273", -26, "123");
            test("155921023828072216384094494261247", -100, "122");
            test("155921023828072216384094494261248", -100, "123");
            test("155921023828072216384094494261249", -100, "123");
            test("4294967295", -1, "2147483647");
            test("4294967296", -1, "2147483648");
            test("4294967297", -1, "2147483648");
            test("7999999999999", -3, "999999999999");
            test("8000000000000", -3, "1000000000000");
            test("8000000000001", -3, "1000000000000");
            test("16777216000000000000", -24, "1000000000000");
            test("33554432000000000000", -25, "1000000000000");
            test("2147483648000000000000", -31, "1000000000000");
            test("4294967296000000000000", -32, "1000000000000");
            test("8589934592000000000000", -33, "1000000000000");
            test(
                "1267650600228229401496703205376000000000000",
                -100,
                "1000000000000",
            );
            test("1000000000000", -10, "976562500");
            test("980657949", -72, "0");
            test("4294967295", -31, "1");
            test("4294967295", -32, "0");
            test("4294967296", -32, "1");
            test("4294967296", -33, "0");
        }
    };
}
tests_and_properties_signed!(i8, test_shl_i8, i, j, out, {});
tests_and_properties_signed!(i16, test_shl_i16, i, j, out, {});
tests_and_properties_signed!(i32, test_shl_signed_limb, i, j, out, {
    let mut n = rug::Integer::from_str(i).unwrap();
    n <<= j;
    assert_eq!(n.to_string(), out);

    let n = rug::Integer::from_str(i).unwrap() << j;
    assert_eq!(n.to_string(), out);
});
tests_and_properties_signed!(i64, test_shl_i64, i, j, out, {});
tests_and_properties_signed!(i128, test_shl_i128, i, j, out, {});
tests_and_properties_signed!(isize, test_shl_isize, i, j, out, {});
