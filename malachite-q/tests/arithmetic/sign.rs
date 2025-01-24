// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Sign;
use malachite_nz::test_util::generators::integer_gen;
use malachite_q::test_util::arithmetic::sign::num_sign;
use malachite_q::test_util::generators::rational_gen;
use malachite_q::Rational;
use num::BigRational;
use rug;
use std::cmp::Ordering::*;
use std::str::FromStr;

#[test]
fn test_sign() {
    let test = |s, out| {
        assert_eq!(Rational::from_str(s).unwrap().sign(), out);
        assert_eq!(num_sign(&BigRational::from_str(s).unwrap()), out);
        assert_eq!(rug::Rational::from_str(s).unwrap().cmp0(), out);
    };
    test("0", Equal);
    test("123", Greater);
    test("-123", Less);
    test("1000000000000", Greater);
    test("-1000000000000", Less);
}

#[test]
fn sign_properties() {
    rational_gen().test_properties(|n| {
        let sign = n.sign();
        assert_eq!(rug::Rational::from(&n).cmp0(), sign);
        assert_eq!(num_sign(&BigRational::from(&n)), sign);
        assert_eq!(n.partial_cmp(&0), Some(sign));
        assert_eq!((-n).sign(), sign.reverse());
    });

    integer_gen().test_properties(|n| {
        assert_eq!(Rational::from(&n).sign(), n.sign());
    });
}
