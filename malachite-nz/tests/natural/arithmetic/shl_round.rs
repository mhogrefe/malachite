// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ArithmeticCheckedShl, ShlRound, ShlRoundAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{NegativeOne, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    signed_rounding_mode_pair_gen, unsigned_signed_rounding_mode_triple_gen_var_2,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_rounding_mode_pair_gen, natural_signed_rounding_mode_triple_gen_var_1,
};
use std::cmp::Ordering::*;
use std::ops::Shr;
use std::panic::catch_unwind;
use std::str::FromStr;

macro_rules! test_shl_round_signed_helper {
    ($t:ident) => {
        let test = |s, j: $t, rm: RoundingMode, out, o| {
            let u = Natural::from_str(s).unwrap();

            let mut n = u.clone();
            assert_eq!(n.shl_round_assign(j, rm), o);
            assert_eq!(n.to_string(), out);
            assert!(n.is_valid());

            let (n, o_alt) = u.clone().shl_round(j, rm);
            assert_eq!(n.to_string(), out);
            assert!(n.is_valid());
            assert_eq!(o_alt, o);

            let (n, o_alt) = (&u).shl_round(j, rm);
            assert_eq!(n.to_string(), out);
            assert!(n.is_valid());
            assert_eq!(o_alt, o);
        };
        test("0", 0, Down, "0", Equal);
        test("0", 0, Up, "0", Equal);
        test("0", 0, Floor, "0", Equal);
        test("0", 0, Ceiling, "0", Equal);
        test("0", 0, Nearest, "0", Equal);
        test("0", 0, Exact, "0", Equal);

        test("0", -10, Down, "0", Equal);
        test("0", -10, Up, "0", Equal);
        test("0", -10, Floor, "0", Equal);
        test("0", -10, Ceiling, "0", Equal);
        test("0", -10, Nearest, "0", Equal);
        test("0", -10, Exact, "0", Equal);

        test("123", 0, Down, "123", Equal);
        test("123", 0, Up, "123", Equal);
        test("123", 0, Floor, "123", Equal);
        test("123", 0, Ceiling, "123", Equal);
        test("123", 0, Nearest, "123", Equal);
        test("123", 0, Exact, "123", Equal);

        test("245", -1, Down, "122", Less);
        test("245", -1, Up, "123", Greater);
        test("245", -1, Floor, "122", Less);
        test("245", -1, Ceiling, "123", Greater);
        test("245", -1, Nearest, "122", Less);

        test("246", -1, Down, "123", Equal);
        test("246", -1, Up, "123", Equal);
        test("246", -1, Floor, "123", Equal);
        test("246", -1, Ceiling, "123", Equal);
        test("246", -1, Nearest, "123", Equal);
        test("246", -1, Exact, "123", Equal);

        test("247", -1, Down, "123", Less);
        test("247", -1, Up, "124", Greater);
        test("247", -1, Floor, "123", Less);
        test("247", -1, Ceiling, "124", Greater);
        test("247", -1, Nearest, "124", Greater);

        test("491", -2, Down, "122", Less);
        test("491", -2, Up, "123", Greater);
        test("491", -2, Floor, "122", Less);
        test("491", -2, Ceiling, "123", Greater);
        test("491", -2, Nearest, "123", Greater);

        test("492", -2, Down, "123", Equal);
        test("492", -2, Up, "123", Equal);
        test("492", -2, Floor, "123", Equal);
        test("492", -2, Ceiling, "123", Equal);
        test("492", -2, Nearest, "123", Equal);
        test("492", -2, Exact, "123", Equal);

        test("493", -2, Down, "123", Less);
        test("493", -2, Up, "124", Greater);
        test("493", -2, Floor, "123", Less);
        test("493", -2, Ceiling, "124", Greater);
        test("493", -2, Nearest, "123", Less);

        test("4127195135", -25, Down, "122", Less);
        test("4127195135", -25, Up, "123", Greater);
        test("4127195135", -25, Floor, "122", Less);
        test("4127195135", -25, Ceiling, "123", Greater);
        test("4127195135", -25, Nearest, "123", Greater);

        test("4127195136", -25, Down, "123", Equal);
        test("4127195136", -25, Up, "123", Equal);
        test("4127195136", -25, Floor, "123", Equal);
        test("4127195136", -25, Ceiling, "123", Equal);
        test("4127195136", -25, Nearest, "123", Equal);
        test("4127195136", -25, Exact, "123", Equal);

        test("4127195137", -25, Down, "123", Less);
        test("4127195137", -25, Up, "124", Greater);
        test("4127195137", -25, Floor, "123", Less);
        test("4127195137", -25, Ceiling, "124", Greater);
        test("4127195137", -25, Nearest, "123", Less);

        test("8254390271", -26, Down, "122", Less);
        test("8254390271", -26, Up, "123", Greater);
        test("8254390271", -26, Floor, "122", Less);
        test("8254390271", -26, Ceiling, "123", Greater);
        test("8254390271", -26, Nearest, "123", Greater);

        test("8254390272", -26, Down, "123", Equal);
        test("8254390272", -26, Up, "123", Equal);
        test("8254390272", -26, Floor, "123", Equal);
        test("8254390272", -26, Ceiling, "123", Equal);
        test("8254390272", -26, Nearest, "123", Equal);
        test("8254390272", -26, Exact, "123", Equal);

        test("8254390273", -26, Down, "123", Less);
        test("8254390273", -26, Up, "124", Greater);
        test("8254390273", -26, Floor, "123", Less);
        test("8254390273", -26, Ceiling, "124", Greater);
        test("8254390273", -26, Nearest, "123", Less);

        test("155921023828072216384094494261247", -100, Down, "122", Less);
        test(
            "155921023828072216384094494261247",
            -100,
            Up,
            "123",
            Greater,
        );
        test(
            "155921023828072216384094494261247",
            -100,
            Floor,
            "122",
            Less,
        );
        test(
            "155921023828072216384094494261247",
            -100,
            Ceiling,
            "123",
            Greater,
        );
        test(
            "155921023828072216384094494261247",
            -100,
            Nearest,
            "123",
            Greater,
        );

        test(
            "155921023828072216384094494261248",
            -100,
            Down,
            "123",
            Equal,
        );
        test("155921023828072216384094494261248", -100, Up, "123", Equal);
        test(
            "155921023828072216384094494261248",
            -100,
            Floor,
            "123",
            Equal,
        );
        test(
            "155921023828072216384094494261248",
            -100,
            Ceiling,
            "123",
            Equal,
        );
        test(
            "155921023828072216384094494261248",
            -100,
            Nearest,
            "123",
            Equal,
        );
        test(
            "155921023828072216384094494261248",
            -100,
            Exact,
            "123",
            Equal,
        );

        test("155921023828072216384094494261249", -100, Down, "123", Less);
        test(
            "155921023828072216384094494261249",
            -100,
            Up,
            "124",
            Greater,
        );
        test(
            "155921023828072216384094494261249",
            -100,
            Floor,
            "123",
            Less,
        );
        test(
            "155921023828072216384094494261249",
            -100,
            Ceiling,
            "124",
            Greater,
        );
        test(
            "155921023828072216384094494261249",
            -100,
            Nearest,
            "123",
            Less,
        );

        test("4294967295", -1, Down, "2147483647", Less);
        test("4294967295", -1, Up, "2147483648", Greater);
        test("4294967295", -1, Floor, "2147483647", Less);
        test("4294967295", -1, Ceiling, "2147483648", Greater);
        test("4294967295", -1, Nearest, "2147483648", Greater);

        test("4294967296", -1, Down, "2147483648", Equal);
        test("4294967296", -1, Up, "2147483648", Equal);
        test("4294967296", -1, Floor, "2147483648", Equal);
        test("4294967296", -1, Ceiling, "2147483648", Equal);
        test("4294967296", -1, Nearest, "2147483648", Equal);
        test("4294967296", -1, Exact, "2147483648", Equal);

        test("4294967297", -1, Down, "2147483648", Less);
        test("4294967297", -1, Up, "2147483649", Greater);
        test("4294967297", -1, Floor, "2147483648", Less);
        test("4294967297", -1, Ceiling, "2147483649", Greater);
        test("4294967297", -1, Nearest, "2147483648", Less);

        test("1000000000000", 0, Down, "1000000000000", Equal);
        test("1000000000000", 0, Up, "1000000000000", Equal);
        test("1000000000000", 0, Floor, "1000000000000", Equal);
        test("1000000000000", 0, Ceiling, "1000000000000", Equal);
        test("1000000000000", 0, Nearest, "1000000000000", Equal);
        test("1000000000000", 0, Exact, "1000000000000", Equal);

        test("7999999999999", -3, Down, "999999999999", Less);
        test("7999999999999", -3, Up, "1000000000000", Greater);
        test("7999999999999", -3, Floor, "999999999999", Less);
        test("7999999999999", -3, Ceiling, "1000000000000", Greater);
        test("7999999999999", -3, Nearest, "1000000000000", Greater);

        test("8000000000000", -3, Down, "1000000000000", Equal);
        test("8000000000000", -3, Up, "1000000000000", Equal);
        test("8000000000000", -3, Floor, "1000000000000", Equal);
        test("8000000000000", -3, Ceiling, "1000000000000", Equal);
        test("8000000000000", -3, Nearest, "1000000000000", Equal);
        test("8000000000000", -3, Exact, "1000000000000", Equal);

        test("8000000000001", -3, Down, "1000000000000", Less);
        test("8000000000001", -3, Up, "1000000000001", Greater);
        test("8000000000001", -3, Floor, "1000000000000", Less);
        test("8000000000001", -3, Ceiling, "1000000000001", Greater);
        test("8000000000001", -3, Nearest, "1000000000000", Less);

        test("16777216000000000000", -24, Down, "1000000000000", Equal);
        test("16777216000000000000", -24, Up, "1000000000000", Equal);
        test("16777216000000000000", -24, Floor, "1000000000000", Equal);
        test("16777216000000000000", -24, Ceiling, "1000000000000", Equal);
        test("16777216000000000000", -24, Nearest, "1000000000000", Equal);
        test("16777216000000000000", -24, Exact, "1000000000000", Equal);

        test("33554432000000000000", -25, Down, "1000000000000", Equal);
        test("33554432000000000000", -25, Up, "1000000000000", Equal);
        test("33554432000000000000", -25, Floor, "1000000000000", Equal);
        test("33554432000000000000", -25, Ceiling, "1000000000000", Equal);
        test("33554432000000000000", -25, Nearest, "1000000000000", Equal);
        test("33554432000000000000", -25, Exact, "1000000000000", Equal);

        test("2147483648000000000000", -31, Down, "1000000000000", Equal);
        test("2147483648000000000000", -31, Up, "1000000000000", Equal);
        test("2147483648000000000000", -31, Floor, "1000000000000", Equal);
        test(
            "2147483648000000000000",
            -31,
            Ceiling,
            "1000000000000",
            Equal,
        );
        test(
            "2147483648000000000000",
            -31,
            Nearest,
            "1000000000000",
            Equal,
        );
        test("2147483648000000000000", -31, Exact, "1000000000000", Equal);

        test("4294967296000000000000", -32, Down, "1000000000000", Equal);
        test("4294967296000000000000", -32, Up, "1000000000000", Equal);
        test("4294967296000000000000", -32, Floor, "1000000000000", Equal);
        test(
            "4294967296000000000000",
            -32,
            Ceiling,
            "1000000000000",
            Equal,
        );
        test(
            "4294967296000000000000",
            -32,
            Nearest,
            "1000000000000",
            Equal,
        );
        test("4294967296000000000000", -32, Exact, "1000000000000", Equal);

        test("8589934592000000000000", -33, Down, "1000000000000", Equal);
        test("8589934592000000000000", -33, Up, "1000000000000", Equal);
        test("8589934592000000000000", -33, Floor, "1000000000000", Equal);
        test(
            "8589934592000000000000",
            -33,
            Ceiling,
            "1000000000000",
            Equal,
        );
        test(
            "8589934592000000000000",
            -33,
            Nearest,
            "1000000000000",
            Equal,
        );
        test("8589934592000000000000", -33, Exact, "1000000000000", Equal);

        test(
            "1267650600228229401496703205376000000000000",
            -100,
            Down,
            "1000000000000",
            Equal,
        );
        test(
            "1267650600228229401496703205376000000000000",
            -100,
            Up,
            "1000000000000",
            Equal,
        );
        test(
            "1267650600228229401496703205376000000000000",
            -100,
            Floor,
            "1000000000000",
            Equal,
        );
        test(
            "1267650600228229401496703205376000000000000",
            -100,
            Ceiling,
            "1000000000000",
            Equal,
        );
        test(
            "1267650600228229401496703205376000000000000",
            -100,
            Nearest,
            "1000000000000",
            Equal,
        );
        test(
            "1267650600228229401496703205376000000000000",
            -100,
            Exact,
            "1000000000000",
            Equal,
        );

        test("1000000000000", -10, Down, "976562500", Equal);
        test("1000000000000", -10, Up, "976562500", Equal);
        test("1000000000000", -10, Floor, "976562500", Equal);
        test("1000000000000", -10, Ceiling, "976562500", Equal);
        test("1000000000000", -10, Nearest, "976562500", Equal);
        test("1000000000000", -10, Exact, "976562500", Equal);

        test("980657949", -72, Down, "0", Less);
        test("980657949", -72, Up, "1", Greater);
        test("980657949", -72, Floor, "0", Less);
        test("980657949", -72, Ceiling, "1", Greater);
        test("980657949", -72, Nearest, "0", Less);

        test("4294967295", -31, Down, "1", Less);
        test("4294967295", -31, Up, "2", Greater);
        test("4294967295", -31, Floor, "1", Less);
        test("4294967295", -31, Ceiling, "2", Greater);
        test("4294967295", -31, Nearest, "2", Greater);

        test("4294967295", -32, Down, "0", Less);
        test("4294967295", -32, Up, "1", Greater);
        test("4294967295", -32, Floor, "0", Less);
        test("4294967295", -32, Ceiling, "1", Greater);
        test("4294967295", -32, Nearest, "1", Greater);

        test("4294967296", -32, Down, "1", Equal);
        test("4294967296", -32, Up, "1", Equal);
        test("4294967296", -32, Floor, "1", Equal);
        test("4294967296", -32, Ceiling, "1", Equal);
        test("4294967296", -32, Nearest, "1", Equal);
        test("4294967296", -32, Exact, "1", Equal);

        test("4294967296", -33, Down, "0", Less);
        test("4294967296", -33, Up, "1", Greater);
        test("4294967296", -33, Floor, "0", Less);
        test("4294967296", -33, Ceiling, "1", Greater);
        test("4294967296", -33, Nearest, "0", Less);

        test("0", 10, Exact, "0", Equal);
        test("123", 1, Exact, "246", Equal);
        test("123", 2, Exact, "492", Equal);
        test("123", 25, Exact, "4127195136", Equal);
        test("123", 26, Exact, "8254390272", Equal);
        test(
            "123",
            100,
            Exact,
            "155921023828072216384094494261248",
            Equal,
        );
        test("2147483648", 1, Exact, "4294967296", Equal);
        test("1000000000000", 3, Exact, "8000000000000", Equal);
        test("1000000000000", 24, Exact, "16777216000000000000", Equal);
        test("1000000000000", 25, Exact, "33554432000000000000", Equal);
        test("1000000000000", 31, Exact, "2147483648000000000000", Equal);
        test("1000000000000", 32, Exact, "4294967296000000000000", Equal);
        test("1000000000000", 33, Exact, "8589934592000000000000", Equal);
        test(
            "1000000000000",
            100,
            Exact,
            "1267650600228229401496703205376000000000000",
            Equal,
        );
    };
}

#[test]
fn test_shl_round_signed() {
    apply_to_signeds!(test_shl_round_signed_helper);
}

macro_rules! shl_round_signed_fail_helper {
    ($t:ident) => {
        assert_panic!(Natural::from(123u32).shl_round_assign($t::NEGATIVE_ONE, Exact));
        assert_panic!(Natural::from(123u32).shl_round_assign($t::exact_from(-100), Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shl_round_assign($t::NEGATIVE_ONE, Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shl_round_assign($t::exact_from(-100), Exact));
        assert_panic!(Natural::from(123u32).shl_round($t::NEGATIVE_ONE, Exact));
        assert_panic!(Natural::from(123u32).shl_round($t::exact_from(-100), Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shl_round($t::NEGATIVE_ONE, Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shl_round($t::exact_from(-100), Exact));
        assert_panic!((&Natural::from(123u32)).shl_round($t::NEGATIVE_ONE, Exact));
        assert_panic!((&Natural::from(123u32)).shl_round($t::exact_from(-100), Exact));
        assert_panic!(
            (&Natural::from_str("1000000000001").unwrap()).shl_round($t::NEGATIVE_ONE, Exact)
        );
        assert_panic!(
            (&Natural::from_str("1000000000001").unwrap()).shl_round($t::exact_from(-100), Exact)
        );
    };
}

#[test]
fn shl_round_signed_fail() {
    apply_to_signeds!(shl_round_signed_fail_helper);
}

fn properties_helper<T: PrimitiveSigned>()
where
    Natural: Shr<T, Output = Natural> + ShlRound<T, Output = Natural> + ShlRoundAssign<T>,
    for<'a> &'a Natural: ShlRound<T, Output = Natural>,
    Limb: ArithmeticCheckedShl<T, Output = Limb> + ShlRound<T, Output = Limb>,
{
    natural_signed_rounding_mode_triple_gen_var_1::<T>().test_properties(|(n, i, rm)| {
        let mut mut_n = n.clone();
        let o = mut_n.shl_round_assign(i, rm);
        assert!(mut_n.is_valid());
        let shifted = mut_n;

        let (shifted_alt, o_alt) = (&n).shl_round(i, rm);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        assert_eq!(o_alt, o);

        let (shifted_alt, o_alt) = n.clone().shl_round(i, rm);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        assert_eq!(o_alt, o);

        if i < T::ZERO {
            assert_eq!((shifted >> i).cmp(&n), o);
        }
        match rm {
            Floor | Down => assert_ne!(o, Greater),
            Ceiling | Up => assert_ne!(o, Less),
            Exact => assert_eq!(o, Equal),
            _ => {}
        }
    });

    natural_rounding_mode_pair_gen().test_properties(|(n, rm)| {
        assert_eq!((&n).shl_round(T::ZERO, rm), (n, Equal));
    });

    signed_rounding_mode_pair_gen::<T>().test_properties(|(i, rm)| {
        assert_eq!(Natural::ZERO.shl_round(i, rm), (Natural::ZERO, Equal));
    });

    unsigned_signed_rounding_mode_triple_gen_var_2::<Limb, T>().test_properties(|(n, i, rm)| {
        if n.arithmetic_checked_shl(i).is_some() {
            let (s, o) = n.shl_round(i, rm);
            assert_eq!((Natural::from(s), o), Natural::from(n).shl_round(i, rm));
        }
    });
}

#[test]
fn shl_round_properties() {
    apply_fn_to_signeds!(properties_helper);
}
