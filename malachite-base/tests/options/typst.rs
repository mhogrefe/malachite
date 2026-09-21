// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::typst::{
    assert_delimiters_balanced, assert_strings_closed, assert_typst_compiles,
};
use itertools::Itertools;
use malachite_base::strings::typst::ToTypst;
use malachite_base::test_util::generators::option_unsigned_gen;

#[test]
fn test_option_to_typst() {
    let mut frags = Vec::new();
    let mut test = |s: String, out: &str| {
        assert_eq!(s, out);
        frags.push(s);
    };
    test(None::<u8>.to_typst_string(), "bot");
    test(None::<char>.to_typst_string(), "bot");
    test(None::<&str>.to_typst_string(), "bot");
    test(Some(0u8).to_typst_string(), "[0]");
    test(Some(5u8).to_typst_string(), "[5]");
    test(Some(-5i32).to_typst_string(), "[-5]");
    test(Some('α').to_typst_string(), r#"["α"]"#);
    test(Some("hi").to_typst_string(), r#"["hi"]"#);
    test(Some(true).to_typst_string(), r#"["T"]"#);
    test(Some(()).to_typst_string(), "[()]");
    // nesting
    test(Some(None::<u8>).to_typst_string(), "[bot]");
    test(Some(Some(5u8)).to_typst_string(), "[[5]]");
    test(Some(Some(None::<u8>)).to_typst_string(), "[[bot]]");
    assert_typst_compiles(&frags);
}

#[test]
fn test_option_to_typst_is_injective_over_nesting() {
    // The brackets exist so that distinct `Option`s never share a fragment. Without them every one
    // of these would collapse onto the same string.
    let fragments = [
        None::<Option<Option<u8>>>.to_typst_string(),
        Some(None::<Option<u8>>).to_typst_string(),
        Some(Some(None::<u8>)).to_typst_string(),
        Some(Some(Some(0u8))).to_typst_string(),
    ];
    assert_eq!(fragments.iter().unique().count(), fragments.len());
}

#[test]
fn option_to_typst_properties() {
    let mut frags = Vec::new();
    option_unsigned_gen::<u8>().test_properties(|o| {
        let s = o.to_typst_string();
        assert!(!s.is_empty());
        assert_strings_closed(&s);
        assert_delimiters_balanced(&s);
        // Every fragment is either `bot` or a bracketed copy of the inner value's fragment.
        match o {
            None => assert_eq!(s, "bot"),
            Some(x) => assert_eq!(s, format!("[{}]", x.to_typst())),
        }
        frags.push(s);
    });
    assert_typst_compiles(&frags);
}
