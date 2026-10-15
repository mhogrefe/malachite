// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::test_util::generators::string_gen;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::rational_matrix_gen;

#[test]
fn test_serde() {
    let test = |s, out| {
        let m = RationalMatrix::from_str(s).unwrap();
        assert_eq!(serde_json::to_string(&m).unwrap(), out);
        assert_eq!(serde_json::from_str::<RationalMatrix>(out).unwrap(), m);
    };
    test("[]_{0,0}", "{\"height\":0,\"width\":0,\"elements\":[]}");
    test("[]_{0,4}", "{\"height\":0,\"width\":4,\"elements\":[]}");
    test("[]_{4,0}", "{\"height\":4,\"width\":0,\"elements\":[]}");
    test(
        "[-1]",
        "{\"height\":1,\"width\":1,\"elements\":[{\"s\":false,\"n\":\"0x1\",\"d\":\"0x1\"}]}",
    );
    test(
        "[1, 2; 3, 4]",
        "{\"height\":2,\"width\":2,\"elements\":[{\"s\":true,\"n\":\"0x1\",\"d\":\"0x1\"},{\"s\":\
        true,\"n\":\"0x2\",\"d\":\"0x1\"},{\"s\":true,\"n\":\"0x3\",\"d\":\"0x1\"},{\"s\":true,\"\
        n\":\"0x4\",\"d\":\"0x1\"}]}",
    );
    test(
        "[1/2, -22/7]",
        "{\"height\":1,\"width\":2,\"elements\":[{\"s\":true,\"n\":\"0x1\",\"d\":\"0x2\"},{\"s\":\
        false,\"n\":\"0x16\",\"d\":\"0x7\"}]}",
    );
}

#[test]
fn test_serde_fail() {
    let test = |s: &str| {
        assert!(
            serde_json::from_str::<RationalMatrix>(s).is_err(),
            "{s:?} should not deserialize"
        );
    };
    // - the number of elements is not the height times the width
    test("{\"height\":2,\"width\":2,\"elements\":[{\"s\":true,\"n\":\"0x1\",\"d\":\"0x1\"}]}");
    test("{\"height\":0,\"width\":2,\"elements\":[{\"s\":true,\"n\":\"0x1\",\"d\":\"0x1\"}]}");
    test("{\"height\":18446744073709551615,\"width\":2,\"elements\":[]}");
    // - malformed
    test("{\"height\":1,\"width\":1,\"elements\":[\"1\"]}");
    test("{\"height\":1,\"width\":1,\"elements\":[\"0x1\"]}");
    // - an element not in lowest terms, or with a zero denominator
    test("{\"height\":1,\"width\":1,\"elements\":[{\"s\":true,\"n\":\"0x2\",\"d\":\"0x4\"}]}");
    test("{\"height\":1,\"width\":1,\"elements\":[{\"s\":true,\"n\":\"0x1\",\"d\":\"0x0\"}]}");
    test("{\"height\":-1,\"width\":0,\"elements\":[]}");
    test("{\"height\":1,\"elements\":[\"0x1\"]}");
    test("[]");
    test("{}");
    test("");
}

#[test]
fn serde_properties() {
    rational_matrix_gen().test_properties(|m| {
        let s = serde_json::to_string(&m).unwrap();
        assert_eq!(serde_json::from_str::<RationalMatrix>(&s).unwrap(), m);
    });

    string_gen().test_properties(|s| {
        if let Ok(m) = serde_json::from_str::<RationalMatrix>(&s) {
            assert!(m.is_valid());
            let t = serde_json::to_string(&m).unwrap();
            assert_eq!(serde_json::from_str::<RationalMatrix>(&t).unwrap(), m);
        }
    });
}
