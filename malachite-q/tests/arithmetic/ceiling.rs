// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Ceiling, CeilingAssign, Floor};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::integer::Integer;
use malachite_q::test_util::generators::rational_gen;
use malachite_q::Rational;
use num::BigRational;
use std::str::FromStr;

#[test]
fn test_ceiling() {
    let test = |s, out| {
        let x = Rational::from_str(s).unwrap();

        let ceiling = x.clone().ceiling();
        assert!(ceiling.is_valid());
        assert_eq!(ceiling.to_string(), out);

        let ceiling = (&x).ceiling();
        assert!(ceiling.is_valid());
        assert_eq!(ceiling.to_string(), out);

        assert_eq!(BigRational::from_str(s).unwrap().ceil().to_string(), out);
        assert_eq!(rug::Rational::from_str(s).unwrap().ceil().to_string(), out);

        let mut x = x;
        x.ceiling_assign();
        assert!(ceiling.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test("1", "1");
    test("-1", "-1");
    test("123", "123");
    test("22/7", "4");
    test("-22/7", "-3");
    test("936851431250/1397", "670616630");
}

#[test]
fn ceiling_properties() {
    rational_gen().test_properties(|x| {
        let ceiling = x.clone().ceiling();
        assert!(ceiling.is_valid());

        assert_eq!(Rational::from(&BigRational::from(&x).ceil()), ceiling);

        assert_eq!(Rational::from(&rug::Rational::from(&x).ceil()), ceiling);

        let ceiling_alt = (&x).ceiling();
        assert!(ceiling_alt.is_valid());
        assert_eq!(ceiling_alt, ceiling);

        let mut ceiling_alt = x.clone();
        ceiling_alt.ceiling_assign();
        assert!(ceiling_alt.is_valid());
        assert_eq!(ceiling_alt, ceiling);

        assert_eq!(Integer::rounding_from(&x, Ceiling).0, ceiling);
        assert!(ceiling >= x);
        assert!(&ceiling - Integer::ONE < x);
        assert_eq!(ceiling, Rational::from(&ceiling).ceiling());
        assert_eq!(ceiling, -(-x).floor());
    });
}
