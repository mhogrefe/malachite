// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::{
    gaussian_integer_gen, gaussian_integer_gen_var_1, gaussian_integer_gen_var_2,
};

#[test]
fn test_to_string() {
    let test = |s| {
        assert_eq!(GaussianInteger::from_str(s).unwrap().to_string(), s);
    };
    test("0");
    test("1");
    test("-1");
    test("2");
    test("-2");
    test("i");
    test("-i");
    test("2i");
    test("-2i");
    test("1+i");
    test("1-i");
    test("2+3i");
    test("2-3i");
    test("-2+3i");
    test("-2-3i");
    test("-1+i");
    test("-1-i");
    test("123456789012345678901234567890+987654321098765432109876543210i");
}

#[test]
fn to_string_properties() {
    gaussian_integer_gen().test_properties(|x| {
        let s = x.to_string();
        assert_eq!(GaussianInteger::from_str(&s).unwrap(), x);
    });

    gaussian_integer_gen_var_1().test_properties(|x| {
        assert_eq!(x.to_string(), x.real.to_string());
    });

    gaussian_integer_gen_var_2().test_properties(|x| {
        let s = x.to_string();
        if x.imaginary == 0u32 {
            assert_eq!(s, "0");
        } else {
            assert!(s.ends_with('i'));
        }
        assert_eq!(GaussianInteger::from_str(&s).unwrap(), x);
    });
}
