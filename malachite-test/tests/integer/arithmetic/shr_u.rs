use common::test_properties;
use malachite_base::misc::{CheckedFrom, Max};
use malachite_base::num::{
    PartialOrdAbs, PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned, ShrRound, ShrRoundAssign,
    Zero,
};
use malachite_base::round::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::{
    pairs_of_negative_signed_not_min_and_small_unsigned,
    pairs_of_positive_signed_and_small_unsigned, pairs_of_signed_and_small_unsigned,
    pairs_of_unsigned_and_rounding_mode, triples_of_signed_small_unsigned_and_rounding_mode_var_1,
    unsigneds,
};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_rounding_mode, pairs_of_integer_and_small_unsigned,
    pairs_of_integer_and_small_unsigned_var_2,
    triples_of_integer_small_unsigned_and_rounding_mode_var_1,
    triples_of_integer_small_unsigned_and_small_unsigned,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_small_unsigned, triples_of_natural_small_unsigned_and_rounding_mode_var_1,
};
use rug;
use std::i32;
use std::str::FromStr;

macro_rules! tests_and_properties {
    (
        $t:ident,
        $test_shr_u:ident,
        $shr_u_properties:ident,
        $test_shr_round_u:ident,
        $shr_round_assign_u_fail_1:ident,
        $shr_round_assign_u_fail_2:ident,
        $shr_round_assign_u_fail_3:ident,
        $shr_round_assign_u_fail_4:ident,
        $shr_round_u_fail_1:ident,
        $shr_round_u_fail_2:ident,
        $shr_round_u_fail_3:ident,
        $shr_round_u_fail_4:ident,
        $shr_round_u_ref_fail_1:ident,
        $shr_round_u_ref_fail_2:ident,
        $shr_round_u_ref_fail_3:ident,
        $shr_round_u_ref_fail_4:ident,
        $shr_round_u_properties:ident,
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
                let mut n = Integer::from_str($u).unwrap();
                n >>= $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = Integer::from_str($u).unwrap() >> $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = &Integer::from_str($u).unwrap() >> $v;
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

            test("-123", 0, "-123");
            test("-245", 1, "-123");
            test("-246", 1, "-123");
            test("-247", 1, "-124");
            test("-491", 2, "-123");
            test("-492", 2, "-123");
            test("-493", 2, "-124");
            test("-4127195135", 25, "-123");
            test("-4127195136", 25, "-123");
            test("-4127195137", 25, "-124");
            test("-8254390271", 26, "-123");
            test("-8254390272", 26, "-123");
            test("-8254390273", 26, "-124");
            test("-155921023828072216384094494261247", 100, "-123");
            test("-155921023828072216384094494261248", 100, "-123");
            test("-155921023828072216384094494261249", 100, "-124");
            test("-4294967295", 1, "-2147483648");
            test("-4294967296", 1, "-2147483648");
            test("-4294967297", 1, "-2147483649");
            test("-1000000000000", 0, "-1000000000000");
            test("-7999999999999", 3, "-1000000000000");
            test("-8000000000000", 3, "-1000000000000");
            test("-8000000000001", 3, "-1000000000001");
            test("-16777216000000000000", 24, "-1000000000000");
            test("-33554432000000000000", 25, "-1000000000000");
            test("-2147483648000000000000", 31, "-1000000000000");
            test("-4294967296000000000000", 32, "-1000000000000");
            test("-8589934592000000000000", 33, "-1000000000000");
            test(
                "-1267650600228229401496703205376000000000000",
                100,
                "-1000000000000",
            );
            test("-1000000000000", 10, "-976562500");
            test("-980657949", 72, "-1");
            test("-4294967295", 31, "-2");
            test("-4294967295", 32, "-1");
            test("-4294967296", 32, "-1");
            test("-4294967296", 33, "-1");
        }

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

                    if $u
                        < (<$t as PrimitiveUnsigned>::SignedOfEqualWidth::MAX.to_unsigned_bitwise())
                    {
                        let u = $u.to_signed_bitwise();
                        assert_eq!($n >> u, $shifted);
                        assert_eq!($n << -u, $shifted);
                    }

                    $shl_library_comparison_properties
                },
            );

            test_properties(pairs_of_signed_and_small_unsigned::<i32, $t>, |&(i, j)| {
                if let Some(sum) = j.checked_add($t::checked_from(i32::WIDTH).unwrap()) {
                    let shifted = Integer::from(i) >> sum;
                    if i >= 0 {
                        assert_eq!(shifted, 0);
                    } else {
                        assert_eq!(shifted, -1);
                    }
                }

                if j < $t::checked_from(i32::WIDTH).unwrap() {
                    assert_eq!(i >> j, Integer::from(i) >> j);
                }
            });

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

        #[test]
        fn $test_shr_round_u() {
            let test = |u, v: $t, rm: RoundingMode, out| {
                let mut n = Integer::from_str(u).unwrap();
                n.shr_round_assign(v, rm);
                assert_eq!(n.to_string(), out);
                assert!(n.is_valid());

                let n = Integer::from_str(u).unwrap().shr_round(v, rm);
                assert_eq!(n.to_string(), out);
                assert!(n.is_valid());

                let n = &Integer::from_str(u).unwrap().shr_round(v, rm);
                assert_eq!(n.to_string(), out);
                assert!(n.is_valid());
            };
            test("0", 0, RoundingMode::Down, "0");
            test("0", 0, RoundingMode::Up, "0");
            test("0", 0, RoundingMode::Floor, "0");
            test("0", 0, RoundingMode::Ceiling, "0");
            test("0", 0, RoundingMode::Nearest, "0");
            test("0", 0, RoundingMode::Exact, "0");

            test("0", 10, RoundingMode::Down, "0");
            test("0", 10, RoundingMode::Up, "0");
            test("0", 10, RoundingMode::Floor, "0");
            test("0", 10, RoundingMode::Ceiling, "0");
            test("0", 10, RoundingMode::Nearest, "0");
            test("0", 10, RoundingMode::Exact, "0");

            test("123", 0, RoundingMode::Down, "123");
            test("123", 0, RoundingMode::Up, "123");
            test("123", 0, RoundingMode::Floor, "123");
            test("123", 0, RoundingMode::Ceiling, "123");
            test("123", 0, RoundingMode::Nearest, "123");
            test("123", 0, RoundingMode::Exact, "123");

            test("245", 1, RoundingMode::Down, "122");
            test("245", 1, RoundingMode::Up, "123");
            test("245", 1, RoundingMode::Floor, "122");
            test("245", 1, RoundingMode::Ceiling, "123");
            test("245", 1, RoundingMode::Nearest, "122");

            test("246", 1, RoundingMode::Down, "123");
            test("246", 1, RoundingMode::Up, "123");
            test("246", 1, RoundingMode::Floor, "123");
            test("246", 1, RoundingMode::Ceiling, "123");
            test("246", 1, RoundingMode::Nearest, "123");
            test("246", 1, RoundingMode::Exact, "123");

            test("247", 1, RoundingMode::Down, "123");
            test("247", 1, RoundingMode::Up, "124");
            test("247", 1, RoundingMode::Floor, "123");
            test("247", 1, RoundingMode::Ceiling, "124");
            test("247", 1, RoundingMode::Nearest, "124");

            test("491", 2, RoundingMode::Down, "122");
            test("491", 2, RoundingMode::Up, "123");
            test("491", 2, RoundingMode::Floor, "122");
            test("491", 2, RoundingMode::Ceiling, "123");
            test("491", 2, RoundingMode::Nearest, "123");

            test("492", 2, RoundingMode::Down, "123");
            test("492", 2, RoundingMode::Up, "123");
            test("492", 2, RoundingMode::Floor, "123");
            test("492", 2, RoundingMode::Ceiling, "123");
            test("492", 2, RoundingMode::Nearest, "123");
            test("492", 2, RoundingMode::Exact, "123");

            test("493", 2, RoundingMode::Down, "123");
            test("493", 2, RoundingMode::Up, "124");
            test("493", 2, RoundingMode::Floor, "123");
            test("493", 2, RoundingMode::Ceiling, "124");
            test("493", 2, RoundingMode::Nearest, "123");

            test("4127195135", 25, RoundingMode::Down, "122");
            test("4127195135", 25, RoundingMode::Up, "123");
            test("4127195135", 25, RoundingMode::Floor, "122");
            test("4127195135", 25, RoundingMode::Ceiling, "123");
            test("4127195135", 25, RoundingMode::Nearest, "123");

            test("4127195136", 25, RoundingMode::Down, "123");
            test("4127195136", 25, RoundingMode::Up, "123");
            test("4127195136", 25, RoundingMode::Floor, "123");
            test("4127195136", 25, RoundingMode::Ceiling, "123");
            test("4127195136", 25, RoundingMode::Nearest, "123");
            test("4127195136", 25, RoundingMode::Exact, "123");

            test("4127195137", 25, RoundingMode::Down, "123");
            test("4127195137", 25, RoundingMode::Up, "124");
            test("4127195137", 25, RoundingMode::Floor, "123");
            test("4127195137", 25, RoundingMode::Ceiling, "124");
            test("4127195137", 25, RoundingMode::Nearest, "123");

            test("8254390271", 26, RoundingMode::Down, "122");
            test("8254390271", 26, RoundingMode::Up, "123");
            test("8254390271", 26, RoundingMode::Floor, "122");
            test("8254390271", 26, RoundingMode::Ceiling, "123");
            test("8254390271", 26, RoundingMode::Nearest, "123");

            test("8254390272", 26, RoundingMode::Down, "123");
            test("8254390272", 26, RoundingMode::Up, "123");
            test("8254390272", 26, RoundingMode::Floor, "123");
            test("8254390272", 26, RoundingMode::Ceiling, "123");
            test("8254390272", 26, RoundingMode::Nearest, "123");
            test("8254390272", 26, RoundingMode::Exact, "123");

            test("8254390273", 26, RoundingMode::Down, "123");
            test("8254390273", 26, RoundingMode::Up, "124");
            test("8254390273", 26, RoundingMode::Floor, "123");
            test("8254390273", 26, RoundingMode::Ceiling, "124");
            test("8254390273", 26, RoundingMode::Nearest, "123");

            test(
                "155921023828072216384094494261247",
                100,
                RoundingMode::Down,
                "122",
            );
            test(
                "155921023828072216384094494261247",
                100,
                RoundingMode::Up,
                "123",
            );
            test(
                "155921023828072216384094494261247",
                100,
                RoundingMode::Floor,
                "122",
            );
            test(
                "155921023828072216384094494261247",
                100,
                RoundingMode::Ceiling,
                "123",
            );
            test(
                "155921023828072216384094494261247",
                100,
                RoundingMode::Nearest,
                "123",
            );

            test(
                "155921023828072216384094494261248",
                100,
                RoundingMode::Down,
                "123",
            );
            test(
                "155921023828072216384094494261248",
                100,
                RoundingMode::Up,
                "123",
            );
            test(
                "155921023828072216384094494261248",
                100,
                RoundingMode::Floor,
                "123",
            );
            test(
                "155921023828072216384094494261248",
                100,
                RoundingMode::Ceiling,
                "123",
            );
            test(
                "155921023828072216384094494261248",
                100,
                RoundingMode::Nearest,
                "123",
            );
            test(
                "155921023828072216384094494261248",
                100,
                RoundingMode::Exact,
                "123",
            );

            test(
                "155921023828072216384094494261249",
                100,
                RoundingMode::Down,
                "123",
            );
            test(
                "155921023828072216384094494261249",
                100,
                RoundingMode::Up,
                "124",
            );
            test(
                "155921023828072216384094494261249",
                100,
                RoundingMode::Floor,
                "123",
            );
            test(
                "155921023828072216384094494261249",
                100,
                RoundingMode::Ceiling,
                "124",
            );
            test(
                "155921023828072216384094494261249",
                100,
                RoundingMode::Nearest,
                "123",
            );

            test("4294967295", 1, RoundingMode::Down, "2147483647");
            test("4294967295", 1, RoundingMode::Up, "2147483648");
            test("4294967295", 1, RoundingMode::Floor, "2147483647");
            test("4294967295", 1, RoundingMode::Ceiling, "2147483648");
            test("4294967295", 1, RoundingMode::Nearest, "2147483648");

            test("4294967296", 1, RoundingMode::Down, "2147483648");
            test("4294967296", 1, RoundingMode::Up, "2147483648");
            test("4294967296", 1, RoundingMode::Floor, "2147483648");
            test("4294967296", 1, RoundingMode::Ceiling, "2147483648");
            test("4294967296", 1, RoundingMode::Nearest, "2147483648");
            test("4294967296", 1, RoundingMode::Exact, "2147483648");

            test("4294967297", 1, RoundingMode::Down, "2147483648");
            test("4294967297", 1, RoundingMode::Up, "2147483649");
            test("4294967297", 1, RoundingMode::Floor, "2147483648");
            test("4294967297", 1, RoundingMode::Ceiling, "2147483649");
            test("4294967297", 1, RoundingMode::Nearest, "2147483648");

            test("1000000000000", 0, RoundingMode::Down, "1000000000000");
            test("1000000000000", 0, RoundingMode::Up, "1000000000000");
            test("1000000000000", 0, RoundingMode::Floor, "1000000000000");
            test("1000000000000", 0, RoundingMode::Ceiling, "1000000000000");
            test("1000000000000", 0, RoundingMode::Nearest, "1000000000000");
            test("1000000000000", 0, RoundingMode::Exact, "1000000000000");

            test("7999999999999", 3, RoundingMode::Down, "999999999999");
            test("7999999999999", 3, RoundingMode::Up, "1000000000000");
            test("7999999999999", 3, RoundingMode::Floor, "999999999999");
            test("7999999999999", 3, RoundingMode::Ceiling, "1000000000000");
            test("7999999999999", 3, RoundingMode::Nearest, "1000000000000");

            test("8000000000000", 3, RoundingMode::Down, "1000000000000");
            test("8000000000000", 3, RoundingMode::Up, "1000000000000");
            test("8000000000000", 3, RoundingMode::Floor, "1000000000000");
            test("8000000000000", 3, RoundingMode::Ceiling, "1000000000000");
            test("8000000000000", 3, RoundingMode::Nearest, "1000000000000");
            test("8000000000000", 3, RoundingMode::Exact, "1000000000000");

            test("8000000000001", 3, RoundingMode::Down, "1000000000000");
            test("8000000000001", 3, RoundingMode::Up, "1000000000001");
            test("8000000000001", 3, RoundingMode::Floor, "1000000000000");
            test("8000000000001", 3, RoundingMode::Ceiling, "1000000000001");
            test("8000000000001", 3, RoundingMode::Nearest, "1000000000000");

            test(
                "16777216000000000000",
                24,
                RoundingMode::Down,
                "1000000000000",
            );
            test(
                "16777216000000000000",
                24,
                RoundingMode::Up,
                "1000000000000",
            );
            test(
                "16777216000000000000",
                24,
                RoundingMode::Floor,
                "1000000000000",
            );
            test(
                "16777216000000000000",
                24,
                RoundingMode::Ceiling,
                "1000000000000",
            );
            test(
                "16777216000000000000",
                24,
                RoundingMode::Nearest,
                "1000000000000",
            );
            test(
                "16777216000000000000",
                24,
                RoundingMode::Exact,
                "1000000000000",
            );

            test(
                "33554432000000000000",
                25,
                RoundingMode::Down,
                "1000000000000",
            );
            test(
                "33554432000000000000",
                25,
                RoundingMode::Up,
                "1000000000000",
            );
            test(
                "33554432000000000000",
                25,
                RoundingMode::Floor,
                "1000000000000",
            );
            test(
                "33554432000000000000",
                25,
                RoundingMode::Ceiling,
                "1000000000000",
            );
            test(
                "33554432000000000000",
                25,
                RoundingMode::Nearest,
                "1000000000000",
            );
            test(
                "33554432000000000000",
                25,
                RoundingMode::Exact,
                "1000000000000",
            );

            test(
                "2147483648000000000000",
                31,
                RoundingMode::Down,
                "1000000000000",
            );
            test(
                "2147483648000000000000",
                31,
                RoundingMode::Up,
                "1000000000000",
            );
            test(
                "2147483648000000000000",
                31,
                RoundingMode::Floor,
                "1000000000000",
            );
            test(
                "2147483648000000000000",
                31,
                RoundingMode::Ceiling,
                "1000000000000",
            );
            test(
                "2147483648000000000000",
                31,
                RoundingMode::Nearest,
                "1000000000000",
            );
            test(
                "2147483648000000000000",
                31,
                RoundingMode::Exact,
                "1000000000000",
            );

            test(
                "4294967296000000000000",
                32,
                RoundingMode::Down,
                "1000000000000",
            );
            test(
                "4294967296000000000000",
                32,
                RoundingMode::Up,
                "1000000000000",
            );
            test(
                "4294967296000000000000",
                32,
                RoundingMode::Floor,
                "1000000000000",
            );
            test(
                "4294967296000000000000",
                32,
                RoundingMode::Ceiling,
                "1000000000000",
            );
            test(
                "4294967296000000000000",
                32,
                RoundingMode::Nearest,
                "1000000000000",
            );
            test(
                "4294967296000000000000",
                32,
                RoundingMode::Exact,
                "1000000000000",
            );

            test(
                "8589934592000000000000",
                33,
                RoundingMode::Down,
                "1000000000000",
            );
            test(
                "8589934592000000000000",
                33,
                RoundingMode::Up,
                "1000000000000",
            );
            test(
                "8589934592000000000000",
                33,
                RoundingMode::Floor,
                "1000000000000",
            );
            test(
                "8589934592000000000000",
                33,
                RoundingMode::Ceiling,
                "1000000000000",
            );
            test(
                "8589934592000000000000",
                33,
                RoundingMode::Nearest,
                "1000000000000",
            );
            test(
                "8589934592000000000000",
                33,
                RoundingMode::Exact,
                "1000000000000",
            );

            test(
                "1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Down,
                "1000000000000",
            );
            test(
                "1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Up,
                "1000000000000",
            );
            test(
                "1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Floor,
                "1000000000000",
            );
            test(
                "1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Ceiling,
                "1000000000000",
            );
            test(
                "1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Nearest,
                "1000000000000",
            );
            test(
                "1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Exact,
                "1000000000000",
            );

            test("1000000000000", 10, RoundingMode::Down, "976562500");
            test("1000000000000", 10, RoundingMode::Up, "976562500");
            test("1000000000000", 10, RoundingMode::Floor, "976562500");
            test("1000000000000", 10, RoundingMode::Ceiling, "976562500");
            test("1000000000000", 10, RoundingMode::Nearest, "976562500");
            test("1000000000000", 10, RoundingMode::Exact, "976562500");

            test("980657949", 72, RoundingMode::Down, "0");
            test("980657949", 72, RoundingMode::Up, "1");
            test("980657949", 72, RoundingMode::Floor, "0");
            test("980657949", 72, RoundingMode::Ceiling, "1");
            test("980657949", 72, RoundingMode::Nearest, "0");

            test("4294967295", 31, RoundingMode::Down, "1");
            test("4294967295", 31, RoundingMode::Up, "2");
            test("4294967295", 31, RoundingMode::Floor, "1");
            test("4294967295", 31, RoundingMode::Ceiling, "2");
            test("4294967295", 31, RoundingMode::Nearest, "2");

            test("4294967295", 32, RoundingMode::Down, "0");
            test("4294967295", 32, RoundingMode::Up, "1");
            test("4294967295", 32, RoundingMode::Floor, "0");
            test("4294967295", 32, RoundingMode::Ceiling, "1");
            test("4294967295", 32, RoundingMode::Nearest, "1");

            test("4294967296", 32, RoundingMode::Down, "1");
            test("4294967296", 32, RoundingMode::Up, "1");
            test("4294967296", 32, RoundingMode::Floor, "1");
            test("4294967296", 32, RoundingMode::Ceiling, "1");
            test("4294967296", 32, RoundingMode::Nearest, "1");
            test("4294967296", 32, RoundingMode::Exact, "1");

            test("4294967296", 33, RoundingMode::Down, "0");
            test("4294967296", 33, RoundingMode::Up, "1");
            test("4294967296", 33, RoundingMode::Floor, "0");
            test("4294967296", 33, RoundingMode::Ceiling, "1");
            test("4294967296", 33, RoundingMode::Nearest, "0");

            test("-123", 0, RoundingMode::Down, "-123");
            test("-123", 0, RoundingMode::Up, "-123");
            test("-123", 0, RoundingMode::Floor, "-123");
            test("-123", 0, RoundingMode::Ceiling, "-123");
            test("-123", 0, RoundingMode::Nearest, "-123");
            test("-123", 0, RoundingMode::Exact, "-123");

            test("-245", 1, RoundingMode::Down, "-122");
            test("-245", 1, RoundingMode::Up, "-123");
            test("-245", 1, RoundingMode::Floor, "-123");
            test("-245", 1, RoundingMode::Ceiling, "-122");
            test("-245", 1, RoundingMode::Nearest, "-122");

            test("-246", 1, RoundingMode::Down, "-123");
            test("-246", 1, RoundingMode::Up, "-123");
            test("-246", 1, RoundingMode::Floor, "-123");
            test("-246", 1, RoundingMode::Ceiling, "-123");
            test("-246", 1, RoundingMode::Nearest, "-123");
            test("-246", 1, RoundingMode::Exact, "-123");

            test("-247", 1, RoundingMode::Down, "-123");
            test("-247", 1, RoundingMode::Up, "-124");
            test("-247", 1, RoundingMode::Floor, "-124");
            test("-247", 1, RoundingMode::Ceiling, "-123");
            test("-247", 1, RoundingMode::Nearest, "-124");

            test("-491", 2, RoundingMode::Down, "-122");
            test("-491", 2, RoundingMode::Up, "-123");
            test("-491", 2, RoundingMode::Floor, "-123");
            test("-491", 2, RoundingMode::Ceiling, "-122");
            test("-491", 2, RoundingMode::Nearest, "-123");

            test("-492", 2, RoundingMode::Down, "-123");
            test("-492", 2, RoundingMode::Up, "-123");
            test("-492", 2, RoundingMode::Floor, "-123");
            test("-492", 2, RoundingMode::Ceiling, "-123");
            test("-492", 2, RoundingMode::Nearest, "-123");
            test("-492", 2, RoundingMode::Exact, "-123");

            test("-493", 2, RoundingMode::Down, "-123");
            test("-493", 2, RoundingMode::Up, "-124");
            test("-493", 2, RoundingMode::Floor, "-124");
            test("-493", 2, RoundingMode::Ceiling, "-123");
            test("-493", 2, RoundingMode::Nearest, "-123");

            test("-4127195135", 25, RoundingMode::Down, "-122");
            test("-4127195135", 25, RoundingMode::Up, "-123");
            test("-4127195135", 25, RoundingMode::Floor, "-123");
            test("-4127195135", 25, RoundingMode::Ceiling, "-122");
            test("-4127195135", 25, RoundingMode::Nearest, "-123");

            test("-4127195136", 25, RoundingMode::Down, "-123");
            test("-4127195136", 25, RoundingMode::Up, "-123");
            test("-4127195136", 25, RoundingMode::Floor, "-123");
            test("-4127195136", 25, RoundingMode::Ceiling, "-123");
            test("-4127195136", 25, RoundingMode::Nearest, "-123");
            test("-4127195136", 25, RoundingMode::Exact, "-123");

            test("-4127195137", 25, RoundingMode::Down, "-123");
            test("-4127195137", 25, RoundingMode::Up, "-124");
            test("-4127195137", 25, RoundingMode::Floor, "-124");
            test("-4127195137", 25, RoundingMode::Ceiling, "-123");
            test("-4127195137", 25, RoundingMode::Nearest, "-123");

            test("-8254390271", 26, RoundingMode::Down, "-122");
            test("-8254390271", 26, RoundingMode::Up, "-123");
            test("-8254390271", 26, RoundingMode::Floor, "-123");
            test("-8254390271", 26, RoundingMode::Ceiling, "-122");
            test("-8254390271", 26, RoundingMode::Nearest, "-123");

            test("-8254390272", 26, RoundingMode::Down, "-123");
            test("-8254390272", 26, RoundingMode::Up, "-123");
            test("-8254390272", 26, RoundingMode::Floor, "-123");
            test("-8254390272", 26, RoundingMode::Ceiling, "-123");
            test("-8254390272", 26, RoundingMode::Nearest, "-123");
            test("-8254390272", 26, RoundingMode::Exact, "-123");

            test("-8254390273", 26, RoundingMode::Down, "-123");
            test("-8254390273", 26, RoundingMode::Up, "-124");
            test("-8254390273", 26, RoundingMode::Floor, "-124");
            test("-8254390273", 26, RoundingMode::Ceiling, "-123");
            test("-8254390273", 26, RoundingMode::Nearest, "-123");

            test(
                "-155921023828072216384094494261247",
                100,
                RoundingMode::Down,
                "-122",
            );
            test(
                "-155921023828072216384094494261247",
                100,
                RoundingMode::Up,
                "-123",
            );
            test(
                "-155921023828072216384094494261247",
                100,
                RoundingMode::Floor,
                "-123",
            );
            test(
                "-155921023828072216384094494261247",
                100,
                RoundingMode::Ceiling,
                "-122",
            );
            test(
                "-155921023828072216384094494261247",
                100,
                RoundingMode::Nearest,
                "-123",
            );

            test(
                "-155921023828072216384094494261248",
                100,
                RoundingMode::Down,
                "-123",
            );
            test(
                "-155921023828072216384094494261248",
                100,
                RoundingMode::Up,
                "-123",
            );
            test(
                "-155921023828072216384094494261248",
                100,
                RoundingMode::Floor,
                "-123",
            );
            test(
                "-155921023828072216384094494261248",
                100,
                RoundingMode::Ceiling,
                "-123",
            );
            test(
                "-155921023828072216384094494261248",
                100,
                RoundingMode::Nearest,
                "-123",
            );
            test(
                "-155921023828072216384094494261248",
                100,
                RoundingMode::Exact,
                "-123",
            );

            test(
                "-155921023828072216384094494261249",
                100,
                RoundingMode::Down,
                "-123",
            );
            test(
                "-155921023828072216384094494261249",
                100,
                RoundingMode::Up,
                "-124",
            );
            test(
                "-155921023828072216384094494261249",
                100,
                RoundingMode::Floor,
                "-124",
            );
            test(
                "-155921023828072216384094494261249",
                100,
                RoundingMode::Ceiling,
                "-123",
            );
            test(
                "-155921023828072216384094494261249",
                100,
                RoundingMode::Nearest,
                "-123",
            );

            test("-4294967295", 1, RoundingMode::Down, "-2147483647");
            test("-4294967295", 1, RoundingMode::Up, "-2147483648");
            test("-4294967295", 1, RoundingMode::Floor, "-2147483648");
            test("-4294967295", 1, RoundingMode::Ceiling, "-2147483647");
            test("-4294967295", 1, RoundingMode::Nearest, "-2147483648");

            test("-4294967296", 1, RoundingMode::Down, "-2147483648");
            test("-4294967296", 1, RoundingMode::Up, "-2147483648");
            test("-4294967296", 1, RoundingMode::Floor, "-2147483648");
            test("-4294967296", 1, RoundingMode::Ceiling, "-2147483648");
            test("-4294967296", 1, RoundingMode::Nearest, "-2147483648");
            test("-4294967296", 1, RoundingMode::Exact, "-2147483648");

            test("-4294967297", 1, RoundingMode::Down, "-2147483648");
            test("-4294967297", 1, RoundingMode::Up, "-2147483649");
            test("-4294967297", 1, RoundingMode::Floor, "-2147483649");
            test("-4294967297", 1, RoundingMode::Ceiling, "-2147483648");
            test("-4294967297", 1, RoundingMode::Nearest, "-2147483648");

            test("-1000000000000", 0, RoundingMode::Down, "-1000000000000");
            test("-1000000000000", 0, RoundingMode::Up, "-1000000000000");
            test("-1000000000000", 0, RoundingMode::Floor, "-1000000000000");
            test("-1000000000000", 0, RoundingMode::Ceiling, "-1000000000000");
            test("-1000000000000", 0, RoundingMode::Nearest, "-1000000000000");
            test("-1000000000000", 0, RoundingMode::Exact, "-1000000000000");

            test("-7999999999999", 3, RoundingMode::Down, "-999999999999");
            test("-7999999999999", 3, RoundingMode::Up, "-1000000000000");
            test("-7999999999999", 3, RoundingMode::Floor, "-1000000000000");
            test("-7999999999999", 3, RoundingMode::Ceiling, "-999999999999");
            test("-7999999999999", 3, RoundingMode::Nearest, "-1000000000000");

            test("-8000000000000", 3, RoundingMode::Down, "-1000000000000");
            test("-8000000000000", 3, RoundingMode::Up, "-1000000000000");
            test("-8000000000000", 3, RoundingMode::Floor, "-1000000000000");
            test("-8000000000000", 3, RoundingMode::Ceiling, "-1000000000000");
            test("-8000000000000", 3, RoundingMode::Nearest, "-1000000000000");
            test("-8000000000000", 3, RoundingMode::Exact, "-1000000000000");

            test("-8000000000001", 3, RoundingMode::Down, "-1000000000000");
            test("-8000000000001", 3, RoundingMode::Up, "-1000000000001");
            test("-8000000000001", 3, RoundingMode::Floor, "-1000000000001");
            test("-8000000000001", 3, RoundingMode::Ceiling, "-1000000000000");
            test("-8000000000001", 3, RoundingMode::Nearest, "-1000000000000");

            test(
                "-16777216000000000000",
                24,
                RoundingMode::Down,
                "-1000000000000",
            );
            test(
                "-16777216000000000000",
                24,
                RoundingMode::Up,
                "-1000000000000",
            );
            test(
                "-16777216000000000000",
                24,
                RoundingMode::Floor,
                "-1000000000000",
            );
            test(
                "-16777216000000000000",
                24,
                RoundingMode::Ceiling,
                "-1000000000000",
            );
            test(
                "-16777216000000000000",
                24,
                RoundingMode::Nearest,
                "-1000000000000",
            );
            test(
                "-16777216000000000000",
                24,
                RoundingMode::Exact,
                "-1000000000000",
            );

            test(
                "-33554432000000000000",
                25,
                RoundingMode::Down,
                "-1000000000000",
            );
            test(
                "-33554432000000000000",
                25,
                RoundingMode::Up,
                "-1000000000000",
            );
            test(
                "-33554432000000000000",
                25,
                RoundingMode::Floor,
                "-1000000000000",
            );
            test(
                "-33554432000000000000",
                25,
                RoundingMode::Ceiling,
                "-1000000000000",
            );
            test(
                "-33554432000000000000",
                25,
                RoundingMode::Nearest,
                "-1000000000000",
            );
            test(
                "-33554432000000000000",
                25,
                RoundingMode::Exact,
                "-1000000000000",
            );

            test(
                "-2147483648000000000000",
                31,
                RoundingMode::Down,
                "-1000000000000",
            );
            test(
                "-2147483648000000000000",
                31,
                RoundingMode::Up,
                "-1000000000000",
            );
            test(
                "-2147483648000000000000",
                31,
                RoundingMode::Floor,
                "-1000000000000",
            );
            test(
                "-2147483648000000000000",
                31,
                RoundingMode::Ceiling,
                "-1000000000000",
            );
            test(
                "-2147483648000000000000",
                31,
                RoundingMode::Nearest,
                "-1000000000000",
            );
            test(
                "-2147483648000000000000",
                31,
                RoundingMode::Exact,
                "-1000000000000",
            );

            test(
                "-4294967296000000000000",
                32,
                RoundingMode::Down,
                "-1000000000000",
            );
            test(
                "-4294967296000000000000",
                32,
                RoundingMode::Up,
                "-1000000000000",
            );
            test(
                "-4294967296000000000000",
                32,
                RoundingMode::Floor,
                "-1000000000000",
            );
            test(
                "-4294967296000000000000",
                32,
                RoundingMode::Ceiling,
                "-1000000000000",
            );
            test(
                "-4294967296000000000000",
                32,
                RoundingMode::Nearest,
                "-1000000000000",
            );
            test(
                "-4294967296000000000000",
                32,
                RoundingMode::Exact,
                "-1000000000000",
            );

            test(
                "-8589934592000000000000",
                33,
                RoundingMode::Down,
                "-1000000000000",
            );
            test(
                "-8589934592000000000000",
                33,
                RoundingMode::Up,
                "-1000000000000",
            );
            test(
                "-8589934592000000000000",
                33,
                RoundingMode::Floor,
                "-1000000000000",
            );
            test(
                "-8589934592000000000000",
                33,
                RoundingMode::Ceiling,
                "-1000000000000",
            );
            test(
                "-8589934592000000000000",
                33,
                RoundingMode::Nearest,
                "-1000000000000",
            );
            test(
                "-8589934592000000000000",
                33,
                RoundingMode::Exact,
                "-1000000000000",
            );

            test(
                "-1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Down,
                "-1000000000000",
            );
            test(
                "-1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Up,
                "-1000000000000",
            );
            test(
                "-1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Floor,
                "-1000000000000",
            );
            test(
                "-1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Ceiling,
                "-1000000000000",
            );
            test(
                "-1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Nearest,
                "-1000000000000",
            );
            test(
                "-1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Exact,
                "-1000000000000",
            );

            test("-1000000000000", 10, RoundingMode::Down, "-976562500");
            test("-1000000000000", 10, RoundingMode::Up, "-976562500");
            test("-1000000000000", 10, RoundingMode::Floor, "-976562500");
            test("-1000000000000", 10, RoundingMode::Ceiling, "-976562500");
            test("-1000000000000", 10, RoundingMode::Nearest, "-976562500");
            test("-1000000000000", 10, RoundingMode::Exact, "-976562500");

            test("-980657949", 72, RoundingMode::Down, "0");
            test("-980657949", 72, RoundingMode::Up, "-1");
            test("-980657949", 72, RoundingMode::Floor, "-1");
            test("-980657949", 72, RoundingMode::Ceiling, "0");
            test("-980657949", 72, RoundingMode::Nearest, "0");

            test("-4294967295", 31, RoundingMode::Down, "-1");
            test("-4294967295", 31, RoundingMode::Up, "-2");
            test("-4294967295", 31, RoundingMode::Floor, "-2");
            test("-4294967295", 31, RoundingMode::Ceiling, "-1");
            test("-4294967295", 31, RoundingMode::Nearest, "-2");

            test("-4294967295", 32, RoundingMode::Down, "0");
            test("-4294967295", 32, RoundingMode::Up, "-1");
            test("-4294967295", 32, RoundingMode::Floor, "-1");
            test("-4294967295", 32, RoundingMode::Ceiling, "0");
            test("-4294967295", 32, RoundingMode::Nearest, "-1");

            test("-4294967296", 32, RoundingMode::Down, "-1");
            test("-4294967296", 32, RoundingMode::Up, "-1");
            test("-4294967296", 32, RoundingMode::Floor, "-1");
            test("-4294967296", 32, RoundingMode::Ceiling, "-1");
            test("-4294967296", 32, RoundingMode::Nearest, "-1");
            test("-4294967296", 32, RoundingMode::Exact, "-1");

            test("-4294967296", 33, RoundingMode::Down, "0");
            test("-4294967296", 33, RoundingMode::Up, "-1");
            test("-4294967296", 33, RoundingMode::Floor, "-1");
            test("-4294967296", 33, RoundingMode::Ceiling, "0");
            test("-4294967296", 33, RoundingMode::Nearest, "0");
        }

        #[test]
        #[should_panic(expected = "Right shift is not exact: 123 >>= 1")]
        fn $shr_round_assign_u_fail_1() {
            Integer::from(123u32).shr_round_assign(1 as $t, RoundingMode::Exact);
        }

        #[test]
        #[should_panic(expected = "Right shift is not exact: 123 >>= 100")]
        fn $shr_round_assign_u_fail_2() {
            Integer::from(123u32).shr_round_assign(100 as $t, RoundingMode::Exact);
        }

        #[test]
        #[should_panic(expected = "Right shift is not exact.")]
        fn $shr_round_assign_u_fail_3() {
            Integer::from_str("1000000000001")
                .unwrap()
                .shr_round_assign(1 as $t, RoundingMode::Exact);
        }

        #[test]
        #[should_panic(expected = "Right shift is not exact.")]
        fn $shr_round_assign_u_fail_4() {
            Integer::from_str("1000000000001")
                .unwrap()
                .shr_round_assign(100 as $t, RoundingMode::Exact);
        }

        #[test]
        #[should_panic(expected = "Right shift is not exact: 123 >>= 1")]
        fn $shr_round_u_fail_1() {
            Integer::from(123u32).shr_round(1 as $t, RoundingMode::Exact);
        }

        #[test]
        #[should_panic(expected = "Right shift is not exact: 123 >>= 100")]
        fn $shr_round_u_fail_2() {
            Integer::from(123u32).shr_round(100 as $t, RoundingMode::Exact);
        }

        #[test]
        #[should_panic(expected = "Right shift is not exact.")]
        fn $shr_round_u_fail_3() {
            Integer::from_str("1000000000001")
                .unwrap()
                .shr_round(1 as $t, RoundingMode::Exact);
        }

        #[test]
        #[should_panic(expected = "Right shift is not exact.")]
        fn $shr_round_u_fail_4() {
            Integer::from_str("1000000000001")
                .unwrap()
                .shr_round(100 as $t, RoundingMode::Exact);
        }

        #[test]
        #[should_panic(expected = "Right shift is not exact: 123 >> 1")]
        fn $shr_round_u_ref_fail_1() {
            (&Integer::from(123u32)).shr_round(1 as $t, RoundingMode::Exact);
        }

        #[test]
        #[should_panic(expected = "Right shift is not exact: 123 >> 100")]
        fn $shr_round_u_ref_fail_2() {
            (&Integer::from(123u32)).shr_round(100 as $t, RoundingMode::Exact);
        }

        #[test]
        #[should_panic(expected = "Right shift is not exact: 1000000000001 >> 1")]
        fn $shr_round_u_ref_fail_3() {
            (&Integer::from_str("1000000000001").unwrap()).shr_round(1 as $t, RoundingMode::Exact);
        }

        #[test]
        #[should_panic(expected = "Right shift is not exact: 1000000000001 >> 100")]
        fn $shr_round_u_ref_fail_4() {
            (&Integer::from_str("1000000000001").unwrap())
                .shr_round(100 as $t, RoundingMode::Exact);
        }

        #[test]
        fn $shr_round_u_properties() {
            // TODO n.shr_round(u, rm) == n.div_round(1 << u)
            test_properties(
                triples_of_integer_small_unsigned_and_rounding_mode_var_1::<$t>,
                |&(ref n, u, rm)| {
                    let mut mut_n = n.clone();
                    mut_n.shr_round_assign(u, rm);
                    assert!(mut_n.is_valid());
                    let shifted = mut_n;

                    let shifted_alt = n.shr_round(u, rm);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);
                    let shifted_alt = n.clone().shr_round(u, rm);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);

                    assert!(n.shr_round(u, rm).le_abs(n));
                    assert_eq!(-(-n).shr_round(u, -rm), shifted);
                },
            );

            test_properties(pairs_of_integer_and_small_unsigned::<$t>, |&(ref n, u)| {
                let left_shifted = n << u;
                assert_eq!((&left_shifted).shr_round(u, RoundingMode::Down), *n);
                assert_eq!((&left_shifted).shr_round(u, RoundingMode::Up), *n);
                assert_eq!((&left_shifted).shr_round(u, RoundingMode::Floor), *n);
                assert_eq!((&left_shifted).shr_round(u, RoundingMode::Ceiling), *n);
                assert_eq!((&left_shifted).shr_round(u, RoundingMode::Nearest), *n);
                assert_eq!((&left_shifted).shr_round(u, RoundingMode::Exact), *n);
            });

            // TODO test using Rationals
            test_properties(
                pairs_of_integer_and_small_unsigned_var_2::<$t>,
                |&(ref n, u)| {
                    let floor = n.shr_round(u, RoundingMode::Floor);
                    let ceiling = &floor + 1;
                    assert_eq!(n.shr_round(u, RoundingMode::Ceiling), ceiling);
                    if *n >= 0 {
                        assert_eq!(n.shr_round(u, RoundingMode::Up), ceiling);
                        assert_eq!(n.shr_round(u, RoundingMode::Down), floor);
                    } else {
                        assert_eq!(n.shr_round(u, RoundingMode::Up), floor);
                        assert_eq!(n.shr_round(u, RoundingMode::Down), ceiling);
                    }
                    let nearest = n.shr_round(u, RoundingMode::Nearest);
                    assert!(nearest == floor || nearest == ceiling);
                },
            );

            test_properties(
                pairs_of_positive_signed_and_small_unsigned::<i32, $t>,
                |&(i, u)| {
                    if let Some(sum) = u.checked_add($t::checked_from(u32::WIDTH - 1).unwrap()) {
                        assert_eq!(Integer::from(i).shr_round(sum, RoundingMode::Down), 0);
                        assert_eq!(Integer::from(i).shr_round(sum, RoundingMode::Floor), 0);
                        assert_eq!(Integer::from(i).shr_round(sum, RoundingMode::Up), 1);
                        assert_eq!(Integer::from(i).shr_round(sum, RoundingMode::Ceiling), 1);
                        if let Some(sum) = sum.checked_add(1) {
                            assert_eq!(Integer::from(i).shr_round(sum, RoundingMode::Nearest), 0);
                        }
                    }
                },
            );

            test_properties(
                pairs_of_negative_signed_not_min_and_small_unsigned::<i32, $t>,
                |&(i, u)| {
                    if let Some(sum) = u.checked_add($t::checked_from(u32::WIDTH - 1).unwrap()) {
                        assert_eq!(Integer::from(i).shr_round(sum, RoundingMode::Down), 0);
                        assert_eq!(Integer::from(i).shr_round(sum, RoundingMode::Floor), -1);
                        assert_eq!(Integer::from(i).shr_round(sum, RoundingMode::Up), -1);
                        assert_eq!(Integer::from(i).shr_round(sum, RoundingMode::Ceiling), 0);
                        if let Some(sum) = sum.checked_add(1) {
                            assert_eq!(Integer::from(i).shr_round(sum, RoundingMode::Nearest), 0);
                        }
                    }
                },
            );

            test_properties(pairs_of_integer_and_rounding_mode, |&(ref n, rm)| {
                assert_eq!(n.shr_round($t::ZERO, rm), *n);
            });

            test_properties(pairs_of_unsigned_and_rounding_mode::<$t>, |&(u, rm)| {
                assert_eq!(Integer::ZERO.shr_round(u, rm), 0);
            });

            test_properties(
                triples_of_natural_small_unsigned_and_rounding_mode_var_1::<$t>,
                |&(ref n, u, rm)| {
                    assert_eq!(n.shr_round(u, rm), Integer::from(n).shr_round(u, rm));
                },
            );

            test_properties(
                triples_of_signed_small_unsigned_and_rounding_mode_var_1::<i32, $t>,
                |&(n, u, rm)| {
                    if u < $t::checked_from(i32::WIDTH).unwrap() {
                        assert_eq!(n.shr_round(u, rm), Integer::from(n).shr_round(u, rm));
                    }
                },
            );
        }
    };
}
tests_and_properties!(
    u8,
    test_shr_u8,
    shr_u8_properties,
    test_shr_round_u8,
    shr_round_assign_u8_fail_1,
    shr_round_assign_u8_fail_2,
    shr_round_assign_u8_fail_3,
    shr_round_assign_u8_fail_4,
    shr_round_u8_fail_1,
    shr_round_u8_fail_2,
    shr_round_u8_fail_3,
    shr_round_u8_fail_4,
    shr_round_u8_ref_fail_1,
    shr_round_u8_ref_fail_2,
    shr_round_u8_ref_fail_3,
    shr_round_u8_ref_fail_4,
    shr_round_u8_properties,
    u,
    v,
    out,
    {},
    n,
    shifted,
    {}
);
tests_and_properties!(
    u16,
    test_shr_u16,
    shr_u16_properties,
    test_shr_round_u16,
    shr_round_assign_u16_fail_1,
    shr_round_assign_u16_fail_2,
    shr_round_assign_u16_fail_3,
    shr_round_assign_u16_fail_4,
    shr_round_u16_fail_1,
    shr_round_u16_fail_2,
    shr_round_u16_fail_3,
    shr_round_u16_fail_4,
    shr_round_u16_ref_fail_1,
    shr_round_u16_ref_fail_2,
    shr_round_u16_ref_fail_3,
    shr_round_u16_ref_fail_4,
    shr_round_u16_properties,
    u,
    v,
    out,
    {},
    n,
    shifted,
    {}
);
tests_and_properties!(
    u32,
    test_shr_u32,
    shr_u32_properties,
    test_shr_round_u32,
    shr_round_assign_u32_fail_1,
    shr_round_assign_u32_fail_2,
    shr_round_assign_u32_fail_3,
    shr_round_assign_u32_fail_4,
    shr_round_u32_fail_1,
    shr_round_u32_fail_2,
    shr_round_u32_fail_3,
    shr_round_u32_fail_4,
    shr_round_u32_ref_fail_1,
    shr_round_u32_ref_fail_2,
    shr_round_u32_ref_fail_3,
    shr_round_u32_ref_fail_4,
    shr_round_u32_properties,
    u,
    v,
    out,
    {
        let mut n = rug::Integer::from_str(u).unwrap();
        n >>= v;
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() >> v;
        assert_eq!(n.to_string(), out);
    },
    n,
    shifted,
    {
        let mut rug_n = integer_to_rug_integer(n);
        rug_n >>= u;
        assert_eq!(rug_integer_to_integer(&rug_n), shifted);

        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(n) >> u)),
            shifted
        );

        assert_eq!(
            bigint_to_integer(&(&integer_to_bigint(n) >> u as usize)),
            shifted
        );
        assert_eq!(
            bigint_to_integer(&(integer_to_bigint(n) >> u as usize)),
            shifted
        );
    }
);
tests_and_properties!(
    u64,
    test_shr_u64,
    shr_u64_properties,
    test_shr_round_u64,
    shr_round_assign_u64_fail_1,
    shr_round_assign_u64_fail_2,
    shr_round_assign_u64_fail_3,
    shr_round_assign_u64_fail_4,
    shr_round_u64_fail_1,
    shr_round_u64_fail_2,
    shr_round_u64_fail_3,
    shr_round_u64_fail_4,
    shr_round_u64_ref_fail_1,
    shr_round_u64_ref_fail_2,
    shr_round_u64_ref_fail_3,
    shr_round_u64_ref_fail_4,
    shr_round_u64_properties,
    u,
    v,
    out,
    {},
    n,
    shifted,
    {}
);
