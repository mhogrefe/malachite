// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CheckedRoot, CheckedSqrt, Pow, Reciprocal};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::test_util::generators::integer_unsigned_pair_gen_var_3;
use malachite_q::test_util::generators::{
    rational_gen, rational_gen_var_1, rational_gen_var_3, rational_signed_pair_gen_var_4,
    rational_unsigned_pair_gen_var_4,
};
use malachite_q::Rational;
use std::panic::catch_unwind;
use std::str::FromStr;

fn test_helper<T: Copy>(s: &str, exp: T, out: Option<&str>)
where
    Rational: CheckedRoot<T, Output = Rational>,
    for<'a> &'a Rational: CheckedRoot<T, Output = Rational>,
{
    let n = Rational::from_str(s).unwrap();
    let out = out.map(ToString::to_string);

    assert_eq!(n.clone().checked_root(exp).map(|x| x.to_string()), out);
    assert_eq!((&n).checked_root(exp).map(|x| x.to_string()), out);
}

#[test]
fn test_checked_root() {
    let test = |s, exp, out: Option<&str>| {
        test_helper::<u64>(s, exp, out);
        test_helper::<i64>(s, i64::exact_from(exp), out);
    };
    test("0", 1, Some("0"));
    test("1", 1, Some("1"));
    test("2", 1, Some("2"));
    test("22/7", 1, Some("22/7"));

    test("0", 2, Some("0"));
    test("1", 2, Some("1"));
    test("2", 2, None);
    test("3", 2, None);
    test("4", 2, Some("2"));
    test("5", 2, None);
    test("22/7", 2, None);
    test("4/9", 2, Some("2/3"));

    test("-1", 1, Some("-1"));
    test("-2", 1, Some("-2"));
    test("-100", 1, Some("-100"));

    test("-1", 3, Some("-1"));
    test("-2", 3, None);
    test("-7", 3, None);
    test("-8", 3, Some("-2"));
    test("-9", 3, None);
    test("-27/8", 3, Some("-3/2"));
    test("27/8", 3, Some("3/2"));

    let test_i = |s, exp: i64, out: Option<&str>| {
        test_helper::<i64>(s, exp, out);
    };
    test_i("1", -1, Some("1"));
    test_i("2", -1, Some("1/2"));
    test_i("22/7", -1, Some("7/22"));

    test_i("1", -2, Some("1"));
    test_i("2", -2, None);
    test_i("3", -2, None);
    test_i("4", -2, Some("1/2"));
    test_i("5", -2, None);
    test_i("22/7", -2, None);
    test_i("4/9", -2, Some("3/2"));

    test_i("-1", -1, Some("-1"));
    test_i("-2", -1, Some("-1/2"));
    test_i("-100", -1, Some("-1/100"));

    test_i("-1", -3, Some("-1"));
    test_i("-2", -3, None);
    test_i("-7", -3, None);
    test_i("-8", -3, Some("-1/2"));
    test_i("-9", -3, None);
    test_i("-27/8", -3, Some("-2/3"));
    test_i("27/8", -3, Some("2/3"));
}

#[test]
fn checked_root_fail() {
    assert_panic!(Rational::ONE.checked_root(0u64));
    assert_panic!(Rational::NEGATIVE_ONE.checked_root(0u64));
    assert_panic!(Rational::NEGATIVE_ONE.checked_root(2u64));
    assert_panic!(Rational::NEGATIVE_ONE.checked_root(4u64));
    assert_panic!(Rational::NEGATIVE_ONE.checked_root(100u64));

    assert_panic!(Rational::ZERO.checked_root(-2i64));
    assert_panic!(Rational::ONE.checked_root(0i64));
    assert_panic!(Rational::NEGATIVE_ONE.checked_root(0i64));
    assert_panic!(Rational::NEGATIVE_ONE.checked_root(2i64));
    assert_panic!(Rational::NEGATIVE_ONE.checked_root(4i64));
    assert_panic!(Rational::NEGATIVE_ONE.checked_root(100i64));
}

#[test]
fn checked_root_ref_fail() {
    assert_panic!((&Rational::ONE).checked_root(0u64));
    assert_panic!((&Rational::NEGATIVE_ONE).checked_root(0u64));
    assert_panic!((&Rational::NEGATIVE_ONE).checked_root(2u64));
    assert_panic!((&Rational::NEGATIVE_ONE).checked_root(4u64));
    assert_panic!((&Rational::NEGATIVE_ONE).checked_root(100u64));

    assert_panic!((&Rational::ZERO).checked_root(-2i64));
    assert_panic!((&Rational::ONE).checked_root(0i64));
    assert_panic!((&Rational::NEGATIVE_ONE).checked_root(0i64));
    assert_panic!((&Rational::NEGATIVE_ONE).checked_root(2i64));
    assert_panic!((&Rational::NEGATIVE_ONE).checked_root(4i64));
    assert_panic!((&Rational::NEGATIVE_ONE).checked_root(100i64));
}

#[test]
fn checked_root_properties() {
    rational_unsigned_pair_gen_var_4::<u64>().test_properties(|(n, exp)| {
        let root = n.clone().checked_root(exp);
        assert!(root.as_ref().map_or(true, Rational::is_valid));
        let root_alt = (&n).checked_root(exp);
        assert!(root_alt.as_ref().map_or(true, Rational::is_valid));
        assert_eq!(root_alt, root);
        assert_eq!((&n).checked_root(u64::exact_from(exp)), root);
        if n != 0 {
            assert_eq!(
                (&n).reciprocal().checked_root(exp),
                root.as_ref().map(Reciprocal::reciprocal)
            );
        }
        if let Some(root) = root {
            assert_eq!((&root).pow(exp), n);
        }
    });

    rational_signed_pair_gen_var_4::<i64>().test_properties(|(n, exp)| {
        let root = n.clone().checked_root(exp);
        assert!(root.as_ref().map_or(true, Rational::is_valid));
        let root_alt = (&n).checked_root(exp);
        assert!(root_alt.as_ref().map_or(true, Rational::is_valid));
        assert_eq!(root_alt, root);
        assert_eq!((&n).checked_root(exp), root);
        if n != 0u32 {
            assert_eq!(
                (&n).checked_root(-exp),
                root.as_ref().map(Reciprocal::reciprocal)
            );
            assert_eq!(
                (&n).reciprocal().checked_root(exp),
                root.as_ref().map(Reciprocal::reciprocal)
            );
        }
        if let Some(root) = root {
            assert_eq!((&root).pow(exp), n);
        }
    });

    rational_gen().test_properties(|n| {
        assert_eq!((&n).checked_root(1u64).unwrap(), n);
        assert_eq!((&n).checked_root(1i64).unwrap(), n);
    });

    rational_gen_var_3().test_properties(|n| {
        assert_eq!((&n).checked_root(2u64), (&n).checked_sqrt());
        assert_eq!((&n).checked_root(2i64), n.checked_sqrt());
    });

    rational_gen_var_1().test_properties(|n| {
        assert_eq!((&n).checked_root(-1i64), Some(n.reciprocal()));
    });

    integer_unsigned_pair_gen_var_3().test_properties(|(n, exp)| {
        assert_eq!(
            (&n).checked_root(exp).map(Rational::from),
            Rational::from(n).checked_root(exp)
        );
    });
}
