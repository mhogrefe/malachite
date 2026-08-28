// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{IsGaussianInteger, IsInteger};
use malachite_q::Rational;
use malachite_q::test_util::generators::rational_gen;
use std::str::FromStr;

#[test]
fn test_is_gaussian_integer() {
    let test = |s, out| {
        assert_eq!(Rational::from_str(s).unwrap().is_gaussian_integer(), out);
    };
    test("0", true);
    test("1", true);
    test("100", true);
    test("-100", true);
    test("22/7", false);
    test("-22/7", false);
}

#[test]
fn is_gaussian_integer_properties() {
    rational_gen().test_properties(|x| {
        assert_eq!(x.is_gaussian_integer(), x.is_integer());
    });
}
