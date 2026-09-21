// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::chars::scripts::{fmt_subscript_digits, parse_subscript_digits};
use malachite_base::test_util::generators::{string_gen, unsigned_gen};
use std::fmt::{Display, Formatter, Result};

struct Subscript(u64);

impl Display for Subscript {
    fn fmt(&self, f: &mut Formatter) -> Result {
        fmt_subscript_digits(self.0, f)
    }
}

fn to_string(n: u64) -> String {
    Subscript(n).to_string()
}

#[test]
fn test_fmt_subscript_digits() {
    assert_eq!(to_string(0), "₀");
    assert_eq!(to_string(5), "₅");
    assert_eq!(to_string(9), "₉");
    assert_eq!(to_string(10), "₁₀");
    assert_eq!(to_string(100), "₁₀₀");
    assert_eq!(to_string(1234567890), "₁₂₃₄₅₆₇₈₉₀");
    assert_eq!(to_string(u64::MAX), "₁₈₄₄₆₇₄₄₀₇₃₇₀₉₅₅₁₆₁₅");
}

#[test]
fn test_parse_subscript_digits() {
    assert_eq!(parse_subscript_digits("₀"), Some(0));
    assert_eq!(parse_subscript_digits("₉"), Some(9));
    assert_eq!(parse_subscript_digits("₁₀"), Some(10));
    assert_eq!(
        parse_subscript_digits("₁₈₄₄₆₇₄₄₀₇₃₇₀₉₅₅₁₆₁₅"),
        Some(u64::MAX)
    );
    // The empty string, an ASCII digit, a leading zero, and a number too large for a `u64` are all
    // rejected: each is something `fmt_subscript_digits` never writes.
    assert_eq!(parse_subscript_digits(""), None);
    assert_eq!(parse_subscript_digits("0"), None);
    assert_eq!(parse_subscript_digits("₁0"), None);
    assert_eq!(parse_subscript_digits("₀₁"), None);
    assert_eq!(parse_subscript_digits("₀₀"), None);
    assert_eq!(parse_subscript_digits("x₁"), None);
    assert_eq!(parse_subscript_digits("⁰"), None);
    assert_eq!(parse_subscript_digits("₁₈₄₄₆₇₄₄₀₇₃₇₀₉₅₅₁₆₁₆"), None);
}

#[test]
fn subscript_digits_properties() {
    unsigned_gen::<u64>().test_properties(|n| {
        let s = to_string(n);
        assert!(!s.is_empty());
        assert_eq!(parse_subscript_digits(&s), Some(n));
        assert_eq!(s.chars().count(), n.to_string().len());
    });

    string_gen().test_properties(|s| {
        // Whatever reads back as a number is written the same way it came in.
        if let Some(n) = parse_subscript_digits(&s) {
            assert_eq!(to_string(n), s);
        }
    });
}
