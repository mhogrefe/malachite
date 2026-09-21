// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::typst::assert_typst_compiles;
use itertools::Itertools;
use malachite_base::strings::latex::ToLatex;
use malachite_base::strings::typst::ToTypst;
use malachite_base::test_util::vars::var_scheme_properties;
use malachite_base::vars::VarScheme;
use malachite_base::vars::xyz::{XyzCapsVars, XyzVars};

#[test]
fn test_xyz_vars() {
    assert_eq!(XyzVars.capacity(), Some(26));
    assert_eq!(
        (0..26).map(|i| XyzVars.var(i).to_string()).join(""),
        "xyzwvutsrqponmlkjihgfedcba"
    );
    assert_eq!(XyzVars.var(0).to_latex_string(), "x");
    assert_eq!(XyzVars.var(25).to_latex_string(), "a");
    assert_eq!(XyzVars.var(0).to_typst_string(), "x");
    assert_eq!(XyzVars.parse_var("x"), Some(0));
    assert_eq!(XyzVars.parse_var("y"), Some(1));
    assert_eq!(XyzVars.parse_var("z"), Some(2));
    assert_eq!(XyzVars.parse_var("w"), Some(3));
    assert_eq!(XyzVars.parse_var("a"), Some(25));
    assert_eq!(XyzVars.parse_var("X"), None);
    assert_eq!(XyzVars.parse_var("xy"), None);
}

#[test]
fn test_xyz_caps_vars() {
    assert_eq!(XyzCapsVars.capacity(), Some(26));
    assert_eq!(
        (0..26).map(|i| XyzCapsVars.var(i).to_string()).join(""),
        "XYZWVUTSRQPONMLKJIHGFEDCBA"
    );
    assert_eq!(XyzCapsVars.var(0).to_latex_string(), "X");
    assert_eq!(XyzCapsVars.var(0).to_typst_string(), "X");
    assert_eq!(XyzCapsVars.parse_var("X"), Some(0));
    assert_eq!(XyzCapsVars.parse_var("A"), Some(25));
    assert_eq!(XyzCapsVars.parse_var("x"), None);
}

#[test]
fn xyz_vars_properties() {
    for names in [var_scheme_properties(&XyzVars, 100), var_scheme_properties(&XyzCapsVars, 100)] {
        assert_eq!(names.len(), 26);
    }
    // The two schemes name the same 26 letters, differing only in the order they are reached.
    assert_eq!(
        (0..26)
            .map(|i| XyzVars.var(i).to_string())
            .sorted()
            .join(""),
        "abcdefghijklmnopqrstuvwxyz"
    );
    let mut frags = Vec::new();
    for i in 0..26 {
        frags.push(XyzVars.var(i).to_typst_string());
        frags.push(XyzCapsVars.var(i).to_typst_string());
    }
    assert_typst_compiles(&frags);
}

#[test]
#[should_panic]
fn xyz_vars_fail() {
    XyzVars.var(26).to_string();
}

#[test]
#[should_panic]
fn xyz_caps_vars_fail() {
    XyzCapsVars.var(26).to_string();
}
