// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::string_is_subset;
use malachite_base::test_util::generators::string_gen;
use malachite_q::test_util::generators::{rational_gen, string_gen_var_11};
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_serde() {
    let test = |n, out| {
        assert_eq!(
            serde_json::to_string(&Rational::from_str(n).unwrap()).unwrap(),
            out
        );
        assert_eq!(
            serde_json::from_str::<Rational>(out).unwrap().to_string(),
            n
        );
    };
    test("0", "{\"s\":true,\"n\":\"0x0\",\"d\":\"0x1\"}");
    test("100", "{\"s\":true,\"n\":\"0x64\",\"d\":\"0x1\"}");
    test(
        "1000000000000",
        "{\"s\":true,\"n\":\"0xe8d4a51000\",\"d\":\"0x1\"}",
    );
    test(
        "1000000000000000000000000",
        "{\"s\":true,\"n\":\"0xd3c21bcecceda1000000\",\"d\":\"0x1\"}",
    );
    test(
        "340282366920938463463374607431768211455",
        "{\"s\":true,\"n\":\"0xffffffffffffffffffffffffffffffff\",\"d\":\"0x1\"}",
    );
    test(
        "340282366920938463463374607431768211456",
        "{\"s\":true,\"n\":\"0x100000000000000000000000000000000\",\"d\":\"0x1\"}",
    );
    test("22/7", "{\"s\":true,\"n\":\"0x16\",\"d\":\"0x7\"}");
    test("-100", "{\"s\":false,\"n\":\"0x64\",\"d\":\"0x1\"}");
    test(
        "-1000000000000",
        "{\"s\":false,\"n\":\"0xe8d4a51000\",\"d\":\"0x1\"}",
    );
    test(
        "-1000000000000000000000000",
        "{\"s\":false,\"n\":\"0xd3c21bcecceda1000000\",\"d\":\"0x1\"}",
    );
    test(
        "-340282366920938463463374607431768211455",
        "{\"s\":false,\"n\":\"0xffffffffffffffffffffffffffffffff\",\"d\":\"0x1\"}",
    );
    test(
        "-340282366920938463463374607431768211456",
        "{\"s\":false,\"n\":\"0x100000000000000000000000000000000\",\"d\":\"0x1\"}",
    );
    test("-22/7", "{\"s\":false,\"n\":\"0x16\",\"d\":\"0x7\"}");
}

#[test]
fn serde_properties() {
    rational_gen().test_properties(|x| {
        let s = serde_json::to_string(&x).unwrap();
        assert_eq!(serde_json::from_str::<Rational>(&s).unwrap(), x);
        assert!(string_is_subset(&s, "\",-/0123456789:abcdeflnrstux{}"));
    });

    string_gen().test_properties(|s| {
        let _n: Result<Rational, _> = serde_json::from_str(&s);
    });

    string_gen_var_11().test_properties(|s| {
        let _n: Rational = serde_json::from_str(&s).unwrap();
    });
}
