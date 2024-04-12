// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingRoot, CeilingSqrt, CeilingSqrtAssign, CheckedRoot, CheckedSqrt, FloorRoot, FloorSqrt,
    FloorSqrtAssign, Square,
};
use malachite_base::num::basic::traits::{NegativeOne, One};
use malachite_base::test_util::generators::signed_gen_var_2;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::generators::{integer_gen_var_4, natural_gen};
use num::BigInt;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_floor_sqrt() {
    let test = |s, out| {
        let n = Integer::from_str(s).unwrap();
        assert_eq!(n.clone().floor_sqrt().to_string(), out);
        assert_eq!((&n).floor_sqrt().to_string(), out);

        let mut n = n;
        n.floor_sqrt_assign();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("2", "1");
    test("3", "1");
    test("4", "2");
    test("5", "2");
    test("10", "3");
    test("100", "10");
    test("1000000000", "31622");
    test("152415765279683", "12345677");
    test("152415765279684", "12345678");
    test("152415765279685", "12345678");
    test(
        "10000000000000000000000000000000000000000",
        "100000000000000000000",
    );
    test(
        "100000000000000000000000000000000000000000",
        "316227766016837933199",
    );
}

#[test]
pub fn floor_sqrt_fail() {
    assert_panic!(Integer::NEGATIVE_ONE.floor_sqrt());
}

#[test]
fn test_ceiling_sqrt() {
    let test = |s, out| {
        let n = Integer::from_str(s).unwrap();
        assert_eq!(n.clone().ceiling_sqrt().to_string(), out);
        assert_eq!((&n).ceiling_sqrt().to_string(), out);

        let mut n = n;
        n.ceiling_sqrt_assign();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("2", "2");
    test("3", "2");
    test("4", "2");
    test("5", "3");
    test("10", "4");
    test("100", "10");
    test("1000000000", "31623");
    test("152415765279683", "12345678");
    test("152415765279684", "12345678");
    test("152415765279685", "12345679");
    test(
        "10000000000000000000000000000000000000000",
        "100000000000000000000",
    );
    test(
        "100000000000000000000000000000000000000000",
        "316227766016837933200",
    );
}

#[test]
pub fn ceiling_sqrt_fail() {
    assert_panic!(Integer::NEGATIVE_ONE.ceiling_sqrt());
}

#[allow(clippy::redundant_closure_for_method_calls)]
#[test]
fn test_checked_sqrt() {
    let test = |s, out: Option<&str>| {
        let n = Integer::from_str(s).unwrap();
        let out = out.map(|s| s.to_string());

        assert_eq!(n.clone().checked_sqrt().map(|x| x.to_string()), out);
        assert_eq!((&n).checked_sqrt().map(|x| x.to_string()), out);
    };
    test("0", Some("0"));
    test("1", Some("1"));
    test("2", None);
    test("3", None);
    test("4", Some("2"));
    test("5", None);
    test("10", None);
    test("100", Some("10"));
    test("1000000000", None);
    test("152415765279683", None);
    test("152415765279684", Some("12345678"));
    test("152415765279685", None);
    test(
        "10000000000000000000000000000000000000000",
        Some("100000000000000000000"),
    );
    test("100000000000000000000000000000000000000000", None);
}

#[test]
pub fn checked_sqrt_fail() {
    assert_panic!(Integer::NEGATIVE_ONE.checked_sqrt());
}

#[test]
fn floor_sqrt_properties() {
    integer_gen_var_4().test_properties(|n| {
        let sqrt = n.clone().floor_sqrt();
        assert_eq!((&n).floor_sqrt(), sqrt);
        let mut n_alt = n.clone();
        n_alt.floor_sqrt_assign();
        assert_eq!(n_alt, sqrt);
        assert_eq!((&n).floor_root(2), sqrt);
        assert_eq!(Integer::from(&BigInt::from(&n).sqrt()), sqrt);
        assert_eq!(Integer::from(&rug::Integer::from(&n).sqrt()), sqrt);

        let square = (&sqrt).square();
        let ceiling_sqrt = (&n).ceiling_sqrt();
        if square == n {
            assert_eq!(ceiling_sqrt, sqrt);
        } else {
            assert_eq!(ceiling_sqrt, &sqrt + Integer::ONE);
        }
        assert!(square <= n);
        assert!((sqrt + Integer::ONE).square() > n);
    });

    natural_gen().test_properties(|n| {
        assert_eq!((&n).floor_sqrt(), Integer::from(n).floor_sqrt());
    });

    signed_gen_var_2::<SignedLimb>().test_properties(|i| {
        assert_eq!(i.floor_sqrt(), Integer::from(i).floor_sqrt());
    });
}

#[test]
fn ceiling_sqrt_properties() {
    integer_gen_var_4().test_properties(|n| {
        let sqrt = n.clone().ceiling_sqrt();
        assert_eq!((&n).ceiling_sqrt(), sqrt);
        let mut n_alt = n.clone();
        n_alt.ceiling_sqrt_assign();
        assert_eq!(n_alt, sqrt);
        assert_eq!((&n).ceiling_root(2), sqrt);
        let square = (&sqrt).square();
        let floor_sqrt = (&n).floor_sqrt();
        if square == n {
            assert_eq!(floor_sqrt, sqrt);
        } else {
            assert_eq!(floor_sqrt, &sqrt - Integer::ONE);
        }
        assert!(square >= n);
        if n != 0 {
            assert!((sqrt - Integer::ONE).square() < n);
        }
    });

    natural_gen().test_properties(|n| {
        assert_eq!((&n).ceiling_sqrt(), Integer::from(n).ceiling_sqrt());
    });

    signed_gen_var_2::<SignedLimb>().test_properties(|i| {
        assert_eq!(i.ceiling_sqrt(), Integer::from(i).ceiling_sqrt());
    });
}

#[test]
fn checked_sqrt_properties() {
    integer_gen_var_4().test_properties(|n| {
        let sqrt = n.clone().checked_sqrt();
        assert_eq!((&n).checked_sqrt(), sqrt);
        assert_eq!((&n).checked_root(2), sqrt);
        if let Some(sqrt) = sqrt {
            assert_eq!((&sqrt).square(), n);
            assert_eq!((&n).floor_sqrt(), sqrt);
            assert_eq!(n.ceiling_sqrt(), sqrt);
        }
    });

    natural_gen().test_properties(|n| {
        assert_eq!(
            (&n).checked_sqrt().map(Integer::from),
            Integer::from(n).checked_sqrt()
        );
    });

    signed_gen_var_2::<SignedLimb>().test_properties(|i| {
        assert_eq!(
            i.checked_sqrt().map(Integer::from),
            Integer::from(i).checked_sqrt()
        );
    });
}
