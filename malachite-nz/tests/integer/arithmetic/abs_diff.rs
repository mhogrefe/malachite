// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, AbsDiff, AbsDiffAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::signed_pair_gen;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::generators::{integer_gen, integer_pair_gen, integer_triple_gen};
use std::str::FromStr;

#[test]
fn test_abs_diff_integer() {
    let test = |s, t, out| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        let mut n = u.clone();
        n.abs_diff_assign(v.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n.abs_diff_assign(&v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().abs_diff(v.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().abs_diff(&v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).abs_diff(v.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).abs_diff(&v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", "0");
    test("0", "123", "123");
    test("123", "0", "123");
    test("456", "123", "333");
    test("1000000000000", "123", "999999999877");
    test("123", "1000000000000", "999999999877");
    test("12345678987654321", "314159265358979", "12031519722295342");
    test("4294967296", "1", "4294967295");
    test("4294967295", "4294967295", "0");
    test("4294967296", "4294967295", "1");
    test("4294967296", "4294967296", "0");
    test("4294967295", "4294967296", "1");
    test("18446744073709551616", "1", "18446744073709551615");
    test("18446744073709551615", "18446744073709551615", "0");
    test("18446744073709551616", "18446744073709551615", "1");
    test("18446744073709551615", "18446744073709551616", "1");
    test("70734740290631708", "282942734368", "70734457347897340");
    test("282942734368", "70734740290631708", "70734457347897340");

    test("0", "-123", "123");
    test("456", "-123", "579");
    test("1000000000000", "-123", "1000000000123");
    test("123", "-1000000000000", "1000000000123");
    test("12345678987654321", "-314159265358979", "12659838253013300");
    test("4294967296", "-1", "4294967297");
    test("4294967295", "-4294967295", "8589934590");
    test("4294967296", "-4294967295", "8589934591");
    test("4294967296", "-4294967296", "8589934592");
    test("4294967295", "-4294967296", "8589934591");
    test("18446744073709551616", "-1", "18446744073709551617");
    test(
        "18446744073709551615",
        "-18446744073709551615",
        "36893488147419103230",
    );
    test(
        "18446744073709551616",
        "-18446744073709551615",
        "36893488147419103231",
    );
    test(
        "18446744073709551615",
        "-18446744073709551616",
        "36893488147419103231",
    );
    test("70734740290631708", "-282942734368", "70735023233366076");
    test("282942734368", "-70734740290631708", "70735023233366076");

    test("-123", "0", "123");
    test("-456", "123", "579");
    test("-1000000000000", "123", "1000000000123");
    test("-123", "1000000000000", "1000000000123");
    test("-12345678987654321", "314159265358979", "12659838253013300");
    test("-4294967296", "1", "4294967297");
    test("-4294967295", "4294967295", "8589934590");
    test("-4294967296", "4294967295", "8589934591");
    test("-4294967296", "4294967296", "8589934592");
    test("-4294967295", "4294967296", "8589934591");
    test("-18446744073709551616", "1", "18446744073709551617");
    test(
        "-18446744073709551615",
        "18446744073709551615",
        "36893488147419103230",
    );
    test(
        "-18446744073709551616",
        "18446744073709551615",
        "36893488147419103231",
    );
    test(
        "-18446744073709551615",
        "18446744073709551616",
        "36893488147419103231",
    );
    test("-70734740290631708", "282942734368", "70735023233366076");
    test("-282942734368", "70734740290631708", "70735023233366076");

    test("-456", "-123", "333");
    test("-1000000000000", "-123", "999999999877");
    test("-123", "-1000000000000", "999999999877");
    test(
        "-12345678987654321",
        "-314159265358979",
        "12031519722295342",
    );
    test("-4294967296", "-1", "4294967295");
    test("-4294967295", "-4294967295", "0");
    test("-4294967296", "-4294967295", "1");
    test("-4294967296", "-4294967296", "0");
    test("-4294967295", "-4294967296", "1");
    test("-18446744073709551616", "-1", "18446744073709551615");
    test("-18446744073709551615", "-18446744073709551615", "0");
    test("-18446744073709551616", "-18446744073709551615", "1");
    test("-18446744073709551615", "-18446744073709551616", "1");
    test("-70734740290631708", "-282942734368", "70734457347897340");
    test("-282942734368", "-70734740290631708", "70734457347897340");
}

#[test]
fn abs_diff_properties() {
    integer_pair_gen().test_properties(|(x, y)| {
        let mut mut_x = x.clone();
        mut_x.abs_diff_assign(&y);
        assert!(mut_x.is_valid());
        let diff = mut_x;

        let mut mut_x = x.clone();
        mut_x.abs_diff_assign(y.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, diff);

        let diff_alt = x.clone().abs_diff(y.clone());
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        let diff_alt = x.clone().abs_diff(&y);
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        let diff_alt = (&x).abs_diff(y.clone());
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        let diff_alt = (&x).abs_diff(&y);
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        assert_eq!((&x - &y).abs(), diff);
        assert_eq!((&y).abs_diff(&x), diff);
        assert_eq!((-&x).abs_diff(-&y), diff);
        assert_eq!(diff == 0, x == y);
    });

    integer_gen().test_properties(|x| {
        assert_eq!((&x).abs_diff(Integer::ZERO), (&x).abs());
        assert_eq!((&x).abs_diff(&x), 0);
        assert_eq!(Integer::ZERO.abs_diff(&x), x.abs());
    });

    integer_triple_gen().test_properties(|(x, y, z)| {
        assert!((&x).abs_diff(&z) <= x.abs_diff(&y) + y.abs_diff(z));
    });

    signed_pair_gen::<SignedLimb>().test_properties(|(x, y)| {
        assert_eq!(x.abs_diff(y), Integer::from(x).abs_diff(Integer::from(y)));
    });
}
