// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::gaussian_integer_gen;
use std::str::FromStr;

#[test]
fn test_serde() {
    let test = |s, out| {
        let g = GaussianInteger::from_str(s).unwrap();
        assert_eq!(serde_json::to_string(&g).unwrap(), out);
        assert_eq!(serde_json::from_str::<GaussianInteger>(out).unwrap(), g);
    };
    test("0", "{\"real\":\"0x0\",\"imaginary\":\"0x0\"}");
    test("1", "{\"real\":\"0x1\",\"imaginary\":\"0x0\"}");
    test("-i", "{\"real\":\"0x0\",\"imaginary\":\"-0x1\"}");
    test("2-3i", "{\"real\":\"0x2\",\"imaginary\":\"-0x3\"}");
}

#[test]
fn serde_properties() {
    gaussian_integer_gen().test_properties(|x| {
        assert_eq!(
            serde_json::from_str::<GaussianInteger>(&serde_json::to_string(&x).unwrap()).unwrap(),
            x
        );
    });
}
