// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CheckedSqrt, Reciprocal, Square};
use malachite_base::num::basic::traits::NegativeOne;
use malachite_nz::test_util::generators::integer_gen_var_4;
use malachite_q::test_util::generators::rational_gen_var_3;
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_checked_sqrt() {
    let test = |s, out: Option<&str>| {
        let n = Rational::from_str(s).unwrap();
        let out = out.map(ToString::to_string);
        assert_eq!(n.clone().checked_sqrt().map(|x| x.to_string()), out);
        assert_eq!((&n).checked_sqrt().map(|x| x.to_string()), out);
    };
    test("0", Some("0"));
    test("1", Some("1"));
    test("2", None);
    test("3", None);
    test("4", Some("2"));
    test("5", None);
    test("22/7", None);
    test("4/9", Some("2/3"));
}

#[test]
#[should_panic]
fn checked_sqrt_fail() {
    Rational::NEGATIVE_ONE.checked_sqrt();
}

#[test]
#[should_panic]
fn checked_sqrt_ref_fail() {
    (&Rational::NEGATIVE_ONE).checked_sqrt();
}

#[test]
fn checked_sqrt_properties() {
    rational_gen_var_3().test_properties(|n| {
        let sqrt = n.clone().checked_sqrt();
        assert!(sqrt.as_ref().map_or(true, Rational::is_valid));
        let sqrt_alt = (&n).checked_sqrt();
        assert!(sqrt_alt.as_ref().map_or(true, Rational::is_valid));
        assert_eq!(sqrt_alt, sqrt);
        if n != 0 {
            assert_eq!(
                (&n).reciprocal().checked_sqrt(),
                sqrt.as_ref().map(Reciprocal::reciprocal)
            );
        }
        if let Some(sqrt) = sqrt {
            assert_eq!((&sqrt).square(), n);
        }
    });

    integer_gen_var_4().test_properties(|n| {
        assert_eq!(
            (&n).checked_sqrt().map(Rational::from),
            Rational::from(n).checked_sqrt()
        );
    });
}
