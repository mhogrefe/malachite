// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::gaussian_rational_gen;
use std::str::FromStr;

#[test]
fn test_serde() {
    let test = |s, out| {
        let g = GaussianRational::from_str(s).unwrap();
        assert_eq!(serde_json::to_string(&g).unwrap(), out);
        assert_eq!(serde_json::from_str::<GaussianRational>(out).unwrap(), g);
    };
    test(
        "0",
        "{\"real\":{\"s\":true,\"n\":\"0x0\",\"d\":\"0x1\"},\"imaginary\":{\"s\":true,\
        \"n\":\"0x0\",\"d\":\"0x1\"}}",
    );
    test(
        "-i",
        "{\"real\":{\"s\":true,\"n\":\"0x0\",\"d\":\"0x1\"},\"imaginary\":{\"s\":false,\
        \"n\":\"0x1\",\"d\":\"0x1\"}}",
    );
    test(
        "2/3-5i/6",
        "{\"real\":{\"s\":true,\"n\":\"0x2\",\"d\":\"0x3\"},\"imaginary\":{\"s\":false,\
        \"n\":\"0x5\",\"d\":\"0x6\"}}",
    );
}

#[test]
fn serde_properties() {
    gaussian_rational_gen().test_properties(|x| {
        assert_eq!(
            serde_json::from_str::<GaussianRational>(&serde_json::to_string(&x).unwrap()).unwrap(),
            x
        );
    });
}
