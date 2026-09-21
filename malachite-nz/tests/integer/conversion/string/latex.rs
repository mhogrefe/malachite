// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::strings::latex::ToLatex;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_gen;
use std::str::FromStr;

#[test]
fn test_integer_to_latex() {
    let test = |s: &str| {
        let x = Integer::from_str(s).unwrap();
        assert_eq!(x.to_latex_string(), s);
    };
    test("0");
    test("1");
    test("123");
    test("1000000000000000000000000000000");
    test("-1");
    test("-123");
}

#[test]
fn integer_to_latex_properties() {
    integer_gen().test_properties(|x| {
        // The fragment is the number as `Display` writes it, which is already what math mode wants;
        // nothing is added and nothing is escaped.
        assert_eq!(x.to_latex_string(), x.to_string());
    });
}

#[test]
fn test_integer_to_latex_embedding() {
    // A fragment carries no delimiters of its own, so it can be substituted into a group.
    let x = Integer::from(10).pow(20);
    assert_eq!(
        format!("x^{{{}}}", x.to_latex_string()),
        "x^{100000000000000000000}"
    );
}
