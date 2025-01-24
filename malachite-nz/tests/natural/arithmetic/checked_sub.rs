// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::CheckedSub;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::unsigned_pair_gen_var_27;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_gen, natural_pair_gen};
use malachite_nz::test_util::natural::arithmetic::checked_sub::checked_sub;
use num::BigUint;
use rug;
use std::str::FromStr;

#[test]
fn test_checked_sub_natural() {
    let test = |s, t, out| {
        let u = Natural::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        let on = u.clone().checked_sub(v.clone());
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = u.clone().checked_sub(&v);
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = (&u).checked_sub(v.clone());
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = (&u).checked_sub(&v);
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = checked_sub(BigUint::from_str(s).unwrap(), BigUint::from_str(t).unwrap())
            .map(|x| Natural::from(&x));
        assert_eq!(on.to_debug_string(), out);

        let on = checked_sub(
            rug::Integer::from_str(s).unwrap(),
            rug::Integer::from_str(t).unwrap(),
        );
        assert_eq!(on.to_debug_string(), out);
    };
    test("0", "0", "Some(0)");
    test("0", "123", "None");
    test("123", "0", "Some(123)");
    test("456", "123", "Some(333)");
    test("1000000000000", "123", "Some(999999999877)");
    test("123", "1000000000000", "None");
    test(
        "12345678987654321",
        "314159265358979",
        "Some(12031519722295342)",
    );
    test("4294967296", "1", "Some(4294967295)");
    test("4294967295", "4294967295", "Some(0)");
    test("4294967296", "4294967295", "Some(1)");
    test("4294967296", "4294967296", "Some(0)");
    test("4294967295", "4294967296", "None");
    test("18446744073709551616", "1", "Some(18446744073709551615)");
    test("18446744073709551615", "18446744073709551615", "Some(0)");
    test("18446744073709551616", "18446744073709551615", "Some(1)");
    test("18446744073709551615", "18446744073709551616", "None");
    test(
        "70734740290631708",
        "282942734368",
        "Some(70734457347897340)",
    );
    test("282942734368", "70734740290631708", "None");
}

#[test]
fn checked_sub_properties() {
    natural_pair_gen().test_properties(|(x, y)| {
        let diff = if x >= y {
            let mut mut_x = x.clone();
            mut_x -= &y;
            assert!(mut_x.is_valid());
            let diff = mut_x;

            let mut rug_x = rug::Integer::from(&x);
            rug_x -= rug::Integer::from(&y);
            assert_eq!(Natural::exact_from(&rug_x), diff);
            Some(diff)
        } else {
            None
        };

        let diff_alt = x.clone().checked_sub(y.clone());
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.as_ref().map_or(true, Natural::is_valid));

        let diff_alt = x.clone().checked_sub(&y);
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.as_ref().map_or(true, Natural::is_valid));

        let diff_alt = (&x).checked_sub(y.clone());
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.as_ref().map_or(true, Natural::is_valid));

        let diff_alt = (&x).checked_sub(&y);
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.as_ref().map_or(true, Natural::is_valid));

        let reverse_diff = (&y).checked_sub(&x);
        assert_eq!(reverse_diff.is_some(), x == y || diff.is_none());

        assert_eq!(
            checked_sub(BigUint::from(&x), BigUint::from(&y)).map(|x| Natural::from(&x)),
            diff
        );
        assert_eq!(
            checked_sub(rug::Integer::from(&x), rug::Integer::from(&y))
                .map(|x| Natural::exact_from(&x)),
            diff
        );

        if let Some(diff) = diff {
            assert!(diff <= x);
            assert_eq!(diff + &y, x);
        }
    });

    natural_gen().test_properties(|x| {
        assert_eq!((&x).checked_sub(Natural::ZERO).as_ref(), Some(&x));
        assert_eq!((&x).checked_sub(&x), Some(Natural::ZERO));
        if x != 0 {
            assert!((Natural::ZERO.checked_sub(x)).is_none());
        }
    });

    unsigned_pair_gen_var_27::<Limb>().test_properties(|(x, y)| {
        assert_eq!(
            x.checked_sub(y).map(Natural::from),
            Natural::from(x).checked_sub(Natural::from(y))
        );
    });
}
