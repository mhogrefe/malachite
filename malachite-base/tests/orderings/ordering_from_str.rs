// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::orderings::ordering_from_str;
use std::cmp::Ordering::*;

#[test]
fn test_from_str() {
    let test = |s, out| {
        assert_eq!(ordering_from_str(s), out);
    };
    test("Equal", Some(Equal));
    test("Less", Some(Less));
    test("Greater", Some(Greater));

    test("", None);
    test("abc", None);
    test("Lesser", None);
}
