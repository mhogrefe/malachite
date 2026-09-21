// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::strings::typst::ToTypst;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_gen;
use std::str::FromStr;

#[test]
fn test_natural_to_typst() {
    let test = |s: &str| {
        let x = Natural::from_str(s).unwrap();
        assert_eq!(x.to_typst_string(), s);
    };
    test("0");
    test("1");
    test("123");
    test("1000000000000000000000000000000");
}

#[test]
fn natural_to_typst_properties() {
    natural_gen().test_properties(|x| {
        // The fragment is the number as `Display` writes it, which is already what math mode wants;
        // nothing is added and nothing is escaped.
        assert_eq!(x.to_typst_string(), x.to_string());
    });
}

#[test]
fn test_natural_to_typst_embedding() {
    // A fragment carries no delimiters of its own, so it can be substituted into a group.
    let x = Natural::from(10u32).pow(20);
    assert_eq!(
        format!("x^({})", x.to_typst_string()),
        "x^(100000000000000000000)"
    );
}
