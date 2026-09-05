// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::{
    gaussian_rational_gen, gaussian_rational_gen_var_1, gaussian_rational_gen_var_2,
};

#[test]
fn test_to_string() {
    let test = |s| {
        assert_eq!(GaussianRational::from_str(s).unwrap().to_string(), s);
    };
    test("0");
    test("1");
    test("-1");
    test("2/3");
    test("-2/3");
    test("i");
    test("-i");
    test("2i");
    test("-2i");
    test("i/2");
    test("-i/2");
    test("5i/6");
    test("-5i/6");
    test("1+i");
    test("1-i");
    test("2/3+5i/6");
    test("2/3-5i/6");
    test("-2/3+i/2");
    test("1-i/2");
    test("123456789012345678901234567891/7+987654321098765432109876543210i/11");
}

#[test]
fn to_string_properties() {
    gaussian_rational_gen().test_properties(|x| {
        let s = x.to_string();
        assert_eq!(GaussianRational::from_str(&s).unwrap(), x);
    });

    gaussian_rational_gen_var_1().test_properties(|x| {
        assert_eq!(x.to_string(), x.real.to_string());
    });

    gaussian_rational_gen_var_2().test_properties(|x| {
        let s = x.to_string();
        if x.imaginary == 0u32 {
            assert_eq!(s, "0");
        } else {
            assert!(s.contains('i'));
        }
        assert_eq!(GaussianRational::from_str(&s).unwrap(), x);
    });
}
