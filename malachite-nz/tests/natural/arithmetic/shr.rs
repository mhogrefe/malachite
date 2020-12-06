use std::str::FromStr;

use malachite_base::num::conversion::traits::ExactFrom;
use num::BigUint;
use rug;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::shr::{
    limbs_shr, limbs_shr_to_out, limbs_slice_shr_in_place, limbs_vec_shr_in_place,
};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

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
    test(&[123, 455], 1, &[2147483709, 227]);
    test(&[123, 456], 31, &[912, 0]);
    test(&[123, 456], 32, &[456]);
    test(&[123, 456], 100, &[]);
    test(&[256, 456], 8, &[3355443201, 1]);
    test(&[u32::MAX, 1], 1, &[u32::MAX, 0]);
    test(&[u32::MAX, u32::MAX], 32, &[u32::MAX]);
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
    test(&[10, 10, 10, 10], &[1], 1, 0x80000000, &[0, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[3], 1, 0x80000000, &[1, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[122, 456], 1, 0, &[61, 228, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        1,
        0x80000000,
        &[61, 228, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 455],
        1,
        0x80000000,
        &[2147483709, 227, 10, 10],
    );
    test(&[10, 10, 10, 10], &[123, 456], 31, 246, &[912, 0, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[256, 456],
        8,
        0,
        &[3355443201, 1, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[u32::MAX, 1],
        1,
        0x80000000,
        &[u32::MAX, 0, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[u32::MAX, u32::MAX],
        31,
        u32::MAX - 1,
        &[u32::MAX, 1, 10, 10],
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
    test(&[1], 1, 0x80000000, &[0]);
    test(&[3], 1, 0x80000000, &[1]);
    test(&[122, 456], 1, 0, &[61, 228]);
    test(&[123, 456], 1, 0x80000000, &[61, 228]);
    test(&[123, 455], 1, 0x80000000, &[2147483709, 227]);
    test(&[123, 456], 31, 246, &[912, 0]);
    test(&[256, 456], 8, 0, &[3355443201, 1]);
    test(&[u32::MAX, 1], 1, 0x80000000, &[u32::MAX, 0]);
    test(&[u32::MAX, u32::MAX], 31, u32::MAX - 1, &[u32::MAX, 1]);
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

macro_rules! tests_unsigned {
    (
        $t:ident,
        $test_shr_u:ident,
        $u:ident,
        $v:ident,
        $out:ident,
        $shl_library_comparison_tests:expr
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
    };
}
tests_unsigned!(u8, test_shr_u8, u, v, out, {});
tests_unsigned!(u16, test_shr_u16, u, v, out, {});
tests_unsigned!(u32, test_shr_u32, u, v, out, {
    let mut n = rug::Integer::from_str(u).unwrap();
    n >>= v;
    assert_eq!(n.to_string(), out);

    let n = rug::Integer::from_str(u).unwrap() >> v;
    assert_eq!(n.to_string(), out);

    let n = BigUint::from_str(u).unwrap() >> usize::exact_from(v);
    assert_eq!(n.to_string(), out);

    let n = &BigUint::from_str(u).unwrap() >> usize::exact_from(v);
    assert_eq!(n.to_string(), out);
});
tests_unsigned!(u64, test_shr_u64, u, v, out, {});
tests_unsigned!(u128, test_shr_u128, u, v, out, {});
tests_unsigned!(usize, test_shr_usize, u, v, out, {});

macro_rules! tests_signed {
    (
        $t:ident,
        $test_shr_i:ident,
        $i:ident,
        $j:ident,
        $out:ident,
        $shr_library_comparison_tests:expr
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
    };
}
tests_signed!(i8, test_shr_i8, i, j, out, {});
tests_signed!(i16, test_shr_i16, i, v, out, {});
tests_signed!(i32, test_shr_i32, i, j, out, {
    let mut n = rug::Integer::from_str(i).unwrap();
    n >>= j;
    assert_eq!(n.to_string(), out);

    let n = rug::Integer::from_str(i).unwrap() >> j;
    assert_eq!(n.to_string(), out);
});
tests_signed!(i64, test_shr_i64, i, j, out, {});
tests_signed!(i128, test_shr_i128, i, j, out, {});
tests_signed!(isize, test_shr_isize, i, j, out, {});
