// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingRoot, CeilingRootAssign, CeilingSqrt, CheckedRoot, CheckedSqrt, FloorRoot,
    FloorRootAssign, FloorSqrt, Parity, Pow,
};
use malachite_base::num::basic::traits::{NegativeOne, One};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{signed_gen, signed_unsigned_pair_gen_var_18};
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::generators::{
    integer_gen, integer_gen_var_4, integer_unsigned_pair_gen_var_3, natural_gen,
    natural_unsigned_pair_gen_var_7,
};
use num::BigInt;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_floor_root() {
    let test = |s, exp, out| {
        let n = Integer::from_str(s).unwrap();
        assert_eq!(n.clone().floor_root(exp).to_string(), out);
        assert_eq!((&n).floor_root(exp).to_string(), out);

        let mut n = n;
        n.floor_root_assign(exp);
        assert_eq!(n.to_string(), out);
    };
    test("0", 1, "0");
    test("1", 1, "1");
    test("2", 1, "2");
    test("100", 1, "100");

    test("0", 2, "0");
    test("0", 2, "0");
    test("1", 2, "1");
    test("2", 2, "1");
    test("3", 2, "1");
    test("4", 2, "2");
    test("5", 2, "2");
    test("0", 3, "0");
    test("1", 3, "1");
    test("2", 3, "1");
    test("7", 3, "1");
    test("8", 3, "2");
    test("9", 3, "2");
    test("10", 2, "3");
    test("100", 2, "10");
    test("100", 3, "4");
    test("1000000000", 2, "31622");
    test("1000000000", 3, "1000");
    test("1000000000", 4, "177");
    test("1000000000", 5, "63");
    test("1000000000", 6, "31");
    test("1000000000", 7, "19");
    test("1000000000", 8, "13");
    test("1000000000", 9, "10");
    test("1000000000", 10, "7");

    test("-1", 1, "-1");
    test("-2", 1, "-2");
    test("-100", 1, "-100");

    test("-1", 3, "-1");
    test("-2", 3, "-2");
    test("-7", 3, "-2");
    test("-8", 3, "-2");
    test("-9", 3, "-3");
    test("-100", 3, "-5");
    test("-1000000000", 3, "-1000");
    test("-1000000000", 5, "-64");
    test("-1000000000", 7, "-20");
    test("-1000000000", 9, "-10");
}

#[test]
fn floor_root_fail() {
    assert_panic!(Integer::ONE.floor_root(0));
    assert_panic!(Integer::NEGATIVE_ONE.floor_root(0));
    assert_panic!(Integer::NEGATIVE_ONE.floor_root(2));
    assert_panic!(Integer::NEGATIVE_ONE.floor_root(4));
    assert_panic!(Integer::NEGATIVE_ONE.floor_root(100));
}

#[test]
fn floor_root_ref_fail() {
    assert_panic!((&Integer::ONE).floor_root(0));
    assert_panic!((&Integer::NEGATIVE_ONE).floor_root(0));
    assert_panic!((&Integer::NEGATIVE_ONE).floor_root(2));
    assert_panic!((&Integer::NEGATIVE_ONE).floor_root(4));
    assert_panic!((&Integer::NEGATIVE_ONE).floor_root(100));
}

#[test]
fn floor_root_assign_fail() {
    assert_panic!({
        let mut x = Integer::ONE;
        x.floor_root_assign(0);
    });
    assert_panic!({
        let mut x = Integer::NEGATIVE_ONE;
        x.floor_root_assign(0);
    });
    assert_panic!({
        let mut x = Integer::NEGATIVE_ONE;
        x.floor_root_assign(2);
    });
    assert_panic!({
        let mut x = Integer::NEGATIVE_ONE;
        x.floor_root_assign(4);
    });
    assert_panic!({
        let mut x = Integer::NEGATIVE_ONE;
        x.floor_root_assign(100);
    });
}

#[test]
fn test_ceiling_root() {
    let test = |s, exp, out| {
        let n = Integer::from_str(s).unwrap();
        assert_eq!(n.clone().ceiling_root(exp).to_string(), out);
        assert_eq!((&n).ceiling_root(exp).to_string(), out);

        let mut n = n;
        n.ceiling_root_assign(exp);
        assert_eq!(n.to_string(), out);
    };
    test("0", 1, "0");
    test("1", 1, "1");
    test("2", 1, "2");
    test("100", 1, "100");

    test("0", 2, "0");
    test("1", 2, "1");
    test("2", 2, "2");
    test("3", 2, "2");
    test("4", 2, "2");
    test("5", 2, "3");
    test("0", 3, "0");
    test("1", 3, "1");
    test("2", 3, "2");
    test("7", 3, "2");
    test("8", 3, "2");
    test("9", 3, "3");
    test("10", 2, "4");
    test("100", 2, "10");
    test("100", 3, "5");
    test("1000000000", 2, "31623");
    test("1000000000", 3, "1000");
    test("1000000000", 4, "178");
    test("1000000000", 5, "64");
    test("1000000000", 6, "32");
    test("1000000000", 7, "20");
    test("1000000000", 8, "14");
    test("1000000000", 9, "10");
    test("1000000000", 10, "8");

    test("-1", 1, "-1");
    test("-2", 1, "-2");
    test("-100", 1, "-100");

    test("-1", 3, "-1");
    test("-2", 3, "-1");
    test("-7", 3, "-1");
    test("-8", 3, "-2");
    test("-9", 3, "-2");
    test("-100", 3, "-4");
    test("-1000000000", 3, "-1000");
    test("-1000000000", 5, "-63");
    test("-1000000000", 7, "-19");
    test("-1000000000", 9, "-10");
}

#[test]
fn ceiling_root_fail() {
    assert_panic!(Integer::ONE.ceiling_root(0));
    assert_panic!(Integer::NEGATIVE_ONE.ceiling_root(0));
    assert_panic!(Integer::NEGATIVE_ONE.ceiling_root(2));
    assert_panic!(Integer::NEGATIVE_ONE.ceiling_root(4));
    assert_panic!(Integer::NEGATIVE_ONE.ceiling_root(100));
}

#[test]
fn ceiling_root_ref_fail() {
    assert_panic!((&Integer::ONE).ceiling_root(0));
    assert_panic!((&Integer::NEGATIVE_ONE).ceiling_root(0));
    assert_panic!((&Integer::NEGATIVE_ONE).ceiling_root(2));
    assert_panic!((&Integer::NEGATIVE_ONE).ceiling_root(4));
    assert_panic!((&Integer::NEGATIVE_ONE).ceiling_root(100));
}

#[test]
fn ceiling_root_assign_fail() {
    assert_panic!({
        let mut x = Integer::ONE;
        x.ceiling_root_assign(0);
    });
    assert_panic!({
        let mut x = Integer::NEGATIVE_ONE;
        x.ceiling_root_assign(0);
    });
    assert_panic!({
        let mut x = Integer::NEGATIVE_ONE;
        x.ceiling_root_assign(2);
    });
    assert_panic!({
        let mut x = Integer::NEGATIVE_ONE;
        x.ceiling_root_assign(4);
    });
    assert_panic!({
        let mut x = Integer::NEGATIVE_ONE;
        x.ceiling_root_assign(100);
    });
}

#[allow(clippy::redundant_closure_for_method_calls)]
#[test]
fn test_checked_root() {
    let test = |s, exp, out: Option<&str>| {
        let n = Integer::from_str(s).unwrap();
        let out = out.map(|s| s.to_string());

        assert_eq!(n.clone().checked_root(exp).map(|x| x.to_string()), out);
        assert_eq!((&n).checked_root(exp).map(|x| x.to_string()), out);
    };
    test("0", 1, Some("0"));
    test("1", 1, Some("1"));
    test("2", 1, Some("2"));
    test("100", 1, Some("100"));

    test("0", 2, Some("0"));
    test("1", 2, Some("1"));
    test("2", 2, None);
    test("3", 2, None);
    test("4", 2, Some("2"));
    test("5", 2, None);
    test("0", 3, Some("0"));
    test("1", 3, Some("1"));
    test("2", 3, None);
    test("7", 3, None);
    test("8", 3, Some("2"));
    test("9", 3, None);
    test("10", 2, None);
    test("100", 2, Some("10"));
    test("100", 3, None);
    test("1000000000", 2, None);
    test("1000000000", 3, Some("1000"));
    test("1000000000", 4, None);
    test("1000000000", 5, None);
    test("1000000000", 6, None);
    test("1000000000", 7, None);
    test("1000000000", 8, None);
    test("1000000000", 9, Some("10"));
    test("1000000000", 10, None);

    test("-1", 1, Some("-1"));
    test("-2", 1, Some("-2"));
    test("-100", 1, Some("-100"));

    test("-1", 3, Some("-1"));
    test("-2", 3, None);
    test("-7", 3, None);
    test("-8", 3, Some("-2"));
    test("-9", 3, None);
    test("-100", 3, None);
    test("-1000000000", 3, Some("-1000"));
    test("-1000000000", 5, None);
    test("-1000000000", 7, None);
    test("-1000000000", 9, Some("-10"));
}

#[test]
fn checked_root_fail() {
    assert_panic!(Integer::ONE.checked_root(0));
    assert_panic!(Integer::NEGATIVE_ONE.checked_root(0));
    assert_panic!(Integer::NEGATIVE_ONE.checked_root(2));
    assert_panic!(Integer::NEGATIVE_ONE.checked_root(4));
    assert_panic!(Integer::NEGATIVE_ONE.checked_root(100));
}

#[test]
fn checked_root_ref_fail() {
    assert_panic!((&Integer::ONE).checked_root(0));
    assert_panic!((&Integer::NEGATIVE_ONE).checked_root(0));
    assert_panic!((&Integer::NEGATIVE_ONE).checked_root(2));
    assert_panic!((&Integer::NEGATIVE_ONE).checked_root(4));
    assert_panic!((&Integer::NEGATIVE_ONE).checked_root(100));
}

#[test]
fn floor_cbrt_properties() {
    integer_gen().test_properties(|n| {
        let cbrt = n.clone().floor_root(3);
        assert!(cbrt.is_valid());
        let cbrt_alt = (&n).floor_root(3);
        assert!(cbrt_alt.is_valid());
        assert_eq!(cbrt_alt, cbrt);
        let mut n_alt = n.clone();
        n_alt.floor_root_assign(3);
        assert!(cbrt_alt.is_valid());
        assert_eq!(n_alt, cbrt);
        if n >= 0 {
            assert_eq!(Integer::from(&BigInt::from(&n).nth_root(3)), cbrt);
            assert_eq!(Integer::from(&rug::Integer::from(&n).root(3)), cbrt);
        }

        let cube = (&cbrt).pow(3);
        let ceiling_cbrt = (&n).ceiling_root(3);
        if cube == n {
            assert_eq!(ceiling_cbrt, cbrt);
        } else {
            assert_eq!(ceiling_cbrt, &cbrt + Integer::ONE);
        }
        assert!(cube <= n);
        assert!((&cbrt + Integer::ONE).pow(3) > n);
        assert_eq!(-(-n).ceiling_root(3), cbrt);
    });

    natural_gen().test_properties(|n| {
        assert_eq!((&n).floor_root(3), Integer::from(n).floor_root(3));
    });

    signed_gen::<SignedLimb>().test_properties(|i| {
        assert_eq!(i.floor_root(3), Integer::from(i).floor_root(3));
    });
}

#[test]
fn ceiling_cbrt_properties() {
    integer_gen().test_properties(|n| {
        let cbrt = n.clone().ceiling_root(3);
        assert!(cbrt.is_valid());
        let cbrt_alt = (&n).ceiling_root(3);
        assert!(cbrt_alt.is_valid());
        assert_eq!(cbrt_alt, cbrt);
        let mut n_alt = n.clone();
        n_alt.ceiling_root_assign(3);
        assert!(cbrt_alt.is_valid());
        assert_eq!(n_alt, cbrt);
        if n < 0 {
            assert_eq!(Integer::from(&BigInt::from(&n).nth_root(3)), cbrt);
            assert_eq!(Integer::from(&rug::Integer::from(&n).root(3)), cbrt);
        }
        let cube = (&cbrt).pow(3);
        let floor_cbrt = (&n).floor_root(3);
        if cube == n {
            assert_eq!(floor_cbrt, cbrt);
        } else {
            assert_eq!(floor_cbrt, &cbrt - Integer::ONE);
        }
        assert!(cube >= n);
        if n != 0 {
            assert!((&cbrt - Integer::ONE).pow(3) < n);
        }
        assert_eq!(-(-n).floor_root(3), cbrt);
    });

    natural_gen().test_properties(|n| {
        assert_eq!((&n).ceiling_root(3), Integer::from(n).ceiling_root(3));
    });

    signed_gen::<SignedLimb>().test_properties(|i| {
        assert_eq!(i.ceiling_root(3), Integer::from(i).ceiling_root(3));
    });
}

#[test]
fn checked_cbrt_properties() {
    integer_gen().test_properties(|n| {
        let cbrt = n.clone().checked_root(3);
        assert!(cbrt.as_ref().map_or(true, Integer::is_valid));
        let cbrt_alt = (&n).checked_root(3);
        assert!(cbrt_alt.as_ref().map_or(true, Integer::is_valid));
        assert_eq!(cbrt_alt, cbrt);
        if let Some(cbrt) = cbrt {
            assert_eq!((&cbrt).pow(3), n);
            assert_eq!((&n).floor_root(3), cbrt);
            assert_eq!(n.ceiling_root(3), cbrt);
        }
    });

    natural_gen().test_properties(|n| {
        assert_eq!(
            (&n).checked_root(3).map(Integer::from),
            Integer::from(n).checked_root(3)
        );
    });

    signed_gen::<SignedLimb>().test_properties(|i| {
        assert_eq!(
            i.checked_root(3).map(Integer::from),
            Integer::from(i).checked_root(3)
        );
    });
}

#[test]
fn floor_root_properties() {
    integer_unsigned_pair_gen_var_3().test_properties(|(n, exp)| {
        let root = n.clone().floor_root(exp);
        assert!(root.is_valid());
        let root_alt = (&n).floor_root(exp);
        assert!(root_alt.is_valid());
        assert_eq!(root_alt, root);
        let mut n_alt = n.clone();
        n_alt.floor_root_assign(exp);
        assert!(root_alt.is_valid());
        assert_eq!(n_alt, root);
        if n >= 0 {
            assert_eq!(
                Integer::from(&BigInt::from(&n).nth_root(u32::exact_from(exp))),
                root
            );
            assert_eq!(
                Integer::from(&rug::Integer::from(&n).root(u32::exact_from(exp))),
                root
            );
        }

        let pow = (&root).pow(exp);
        let ceiling_root = (&n).ceiling_root(exp);
        if pow == n {
            assert_eq!(ceiling_root, root);
        } else {
            assert_eq!(ceiling_root, &root + Integer::ONE);
        }
        assert!(pow <= n);
        assert!((&root + Integer::ONE).pow(exp) > n);
        if exp.odd() {
            assert_eq!(-(-n).ceiling_root(exp), root);
        }
    });

    integer_gen().test_properties(|n| {
        assert_eq!((&n).floor_root(1), n);
    });

    integer_gen_var_4().test_properties(|n| {
        assert_eq!((&n).floor_root(2), (&n).floor_sqrt());
    });

    natural_unsigned_pair_gen_var_7().test_properties(|(n, exp)| {
        assert_eq!((&n).floor_root(exp), Integer::from(n).floor_root(exp));
    });

    signed_unsigned_pair_gen_var_18::<SignedLimb, u64>().test_properties(|(i, exp)| {
        assert_eq!(i.floor_root(exp), Integer::from(i).floor_root(exp));
    });
}

#[test]
fn ceiling_root_properties() {
    integer_unsigned_pair_gen_var_3().test_properties(|(n, exp)| {
        let root = n.clone().ceiling_root(exp);
        assert!(root.is_valid());
        let root_alt = (&n).ceiling_root(exp);
        assert!(root_alt.is_valid());
        assert_eq!(root_alt, root);
        let mut n_alt = n.clone();
        n_alt.ceiling_root_assign(exp);
        assert!(root_alt.is_valid());
        assert_eq!(n_alt, root);
        if n < 0 {
            assert_eq!(
                Integer::from(&BigInt::from(&n).nth_root(u32::exact_from(exp))),
                root
            );
            assert_eq!(
                Integer::from(&rug::Integer::from(&n).root(u32::exact_from(exp))),
                root
            );
        }
        let pow = (&root).pow(exp);
        let floor_root = (&n).floor_root(exp);
        if pow == n {
            assert_eq!(floor_root, root);
        } else {
            assert_eq!(floor_root, &root - Integer::ONE);
        }
        assert!(pow >= n);
        if n != 0 {
            assert!((&root - Integer::ONE).pow(exp) < n);
        }
        if exp.odd() {
            assert_eq!(-(-n).floor_root(exp), root);
        }
    });

    integer_gen().test_properties(|n| {
        assert_eq!((&n).ceiling_root(1), n);
    });

    integer_gen_var_4().test_properties(|n| {
        assert_eq!((&n).ceiling_root(2), (&n).ceiling_sqrt());
    });

    natural_unsigned_pair_gen_var_7().test_properties(|(n, exp)| {
        assert_eq!((&n).ceiling_root(exp), Integer::from(n).ceiling_root(exp));
    });

    signed_unsigned_pair_gen_var_18::<SignedLimb, u64>().test_properties(|(i, exp)| {
        assert_eq!(i.ceiling_root(exp), Integer::from(i).ceiling_root(exp));
    });
}

#[test]
fn checked_root_properties() {
    integer_unsigned_pair_gen_var_3().test_properties(|(n, exp)| {
        let root = n.clone().checked_root(exp);
        assert!(root.as_ref().map_or(true, Integer::is_valid));
        let root_alt = (&n).checked_root(exp);
        assert!(root_alt.as_ref().map_or(true, Integer::is_valid));
        assert_eq!(root_alt, root);
        if let Some(root) = root {
            assert_eq!((&root).pow(exp), n);
            assert_eq!((&n).floor_root(exp), root);
            assert_eq!(n.ceiling_root(exp), root);
        }
    });

    integer_gen().test_properties(|n| {
        assert_eq!((&n).checked_root(1), Some(n));
    });

    integer_gen_var_4().test_properties(|n| {
        assert_eq!((&n).checked_root(2), (&n).checked_sqrt());
    });

    natural_unsigned_pair_gen_var_7().test_properties(|(n, exp)| {
        assert_eq!(
            (&n).checked_root(exp).map(Integer::from),
            Integer::from(n).checked_root(exp)
        );
    });

    signed_unsigned_pair_gen_var_18::<SignedLimb, u64>().test_properties(|(i, exp)| {
        assert_eq!(
            i.checked_root(exp).map(Integer::from),
            Integer::from(i).checked_root(exp)
        );
    });
}
