// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::IsInteger;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_q::test_util::generators::rational_gen;
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_is_integer() {
    let test = |n, out| {
        assert_eq!(Rational::from_str(n).unwrap().is_integer(), out);
    };
    test("0", true);
    test("1", true);
    test("100", true);
    test("-1", true);
    test("-100", true);

    test("22/7", false);
    test("-22/7", false);
}

#[test]
fn is_integer_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 256);
    config.insert("mean_stripe_n", 128);
    rational_gen().test_properties_with_config(&config, |x| {
        assert_eq!(x.is_integer(), (-&x).is_integer());
    });
}
