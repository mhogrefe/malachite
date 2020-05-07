use std::str::FromStr;

use malachite_base::num::conversion::traits::ExactFrom;
use num::BigInt;
use rug;

use malachite_nz::integer::Integer;

macro_rules! tests_and_properties {
    (
        $t:ident,
        $test_shl_u:ident,
        $u:ident,
        $v:ident,
        $out:ident,
        $library_comparison_tests:expr
    ) => {
        #[test]
        fn $test_shl_u() {
            let test = |$u, $v: $t, $out| {
                let mut n = Integer::from_str($u).unwrap();
                n <<= $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = Integer::from_str($u).unwrap() << $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = &Integer::from_str($u).unwrap() << $v;
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

            test("-123", 0, "-123");
            test("-123", 1, "-246");
            test("-123", 2, "-492");
            test("-123", 25, "-4127195136");
            test("-123", 26, "-8254390272");
            test("-123", 100, "-155921023828072216384094494261248");
            test("-2147483648", 1, "-4294967296");
            test("-1000000000000", 0, "-1000000000000");
            test("-1000000000000", 3, "-8000000000000");
            test("-1000000000000", 24, "-16777216000000000000");
            test("-1000000000000", 25, "-33554432000000000000");
            test("-1000000000000", 31, "-2147483648000000000000");
            test("-1000000000000", 32, "-4294967296000000000000");
            test("-1000000000000", 33, "-8589934592000000000000");
            test(
                "-1000000000000",
                100,
                "-1267650600228229401496703205376000000000000",
            );
        }
    };
}
tests_and_properties!(u8, test_shl_u8, u, v, out, {});
tests_and_properties!(u16, test_shl_u16, u, v, out, {});
tests_and_properties!(u32, test_shl_limb, u, v, out, {
    let mut n = rug::Integer::from_str(u).unwrap();
    n <<= v;
    assert_eq!(n.to_string(), out);

    let n = rug::Integer::from_str(u).unwrap() << v;
    assert_eq!(n.to_string(), out);

    let n = BigInt::from_str(u).unwrap() << usize::exact_from(v);
    assert_eq!(n.to_string(), out);

    let n = &BigInt::from_str(u).unwrap() << usize::exact_from(v);
    assert_eq!(n.to_string(), out);
});
tests_and_properties!(u64, test_shl_u64, u, v, out, {});
tests_and_properties!(u128, test_shl_u128, u, v, out, {});
tests_and_properties!(usize, test_shl_usize, u, v, out, {});
