// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::test_util::generators::string_gen;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::generators::integer_matrix_gen;

#[test]
fn test_serde() {
    let test = |s, out| {
        let m = IntegerMatrix::from_str(s).unwrap();
        assert_eq!(serde_json::to_string(&m).unwrap(), out);
        assert_eq!(serde_json::from_str::<IntegerMatrix>(out).unwrap(), m);
    };
    test("[]_{0,0}", "{\"height\":0,\"width\":0,\"elements\":[]}");
    test("[]_{0,4}", "{\"height\":0,\"width\":4,\"elements\":[]}");
    test("[]_{4,0}", "{\"height\":4,\"width\":0,\"elements\":[]}");
    test("[-1]", "{\"height\":1,\"width\":1,\"elements\":[\"-0x1\"]}");
    test(
        "[1, 2; 3, 4]",
        "{\"height\":2,\"width\":2,\"elements\":[\"0x1\",\"0x2\",\"0x3\",\"0x4\"]}",
    );
}

#[test]
fn test_serde_fail() {
    let test = |s: &str| {
        assert!(
            serde_json::from_str::<IntegerMatrix>(s).is_err(),
            "{s:?} should not deserialize"
        );
    };
    // - the number of elements is not the height times the width
    test("{\"height\":2,\"width\":2,\"elements\":[\"0x1\"]}");
    test("{\"height\":0,\"width\":2,\"elements\":[\"0x1\"]}");
    test("{\"height\":18446744073709551615,\"width\":2,\"elements\":[]}");
    // - malformed
    test("{\"height\":1,\"width\":1,\"elements\":[\"1\"]}");
    test("{\"height\":-1,\"width\":0,\"elements\":[]}");
    test("{\"height\":1,\"elements\":[\"0x1\"]}");
    test("[]");
    test("{}");
    test("");
}

#[test]
fn serde_properties() {
    integer_matrix_gen().test_properties(|m| {
        let s = serde_json::to_string(&m).unwrap();
        assert_eq!(serde_json::from_str::<IntegerMatrix>(&s).unwrap(), m);
    });

    string_gen().test_properties(|s| {
        if let Ok(m) = serde_json::from_str::<IntegerMatrix>(&s) {
            assert!(m.is_valid());
            let t = serde_json::to_string(&m).unwrap();
            assert_eq!(serde_json::from_str::<IntegerMatrix>(&t).unwrap(), m);
        }
    });
}
