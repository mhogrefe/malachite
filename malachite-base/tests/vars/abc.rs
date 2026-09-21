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
use malachite_base::vars::abc::{AbcCapsVars, AbcVars};

#[test]
fn test_abc_vars() {
    assert_eq!(AbcVars.capacity(), Some(26));
    assert_eq!(
        (0..26).map(|i| AbcVars.var(i).to_string()).join(""),
        "abcdefghijklmnopqrstuvwxyz"
    );
    assert_eq!(AbcVars.var(0).to_latex_string(), "a");
    assert_eq!(AbcVars.var(25).to_latex_string(), "z");
    assert_eq!(AbcVars.var(0).to_typst_string(), "a");
    assert_eq!(AbcVars.var(25).to_typst_string(), "z");
    assert_eq!(AbcVars.parse_var("a"), Some(0));
    assert_eq!(AbcVars.parse_var("z"), Some(25));
    assert_eq!(AbcVars.parse_var("A"), None);
    assert_eq!(AbcVars.parse_var("ab"), None);
    assert_eq!(AbcVars.parse_var("α"), None);
}

#[test]
fn test_abc_caps_vars() {
    assert_eq!(AbcCapsVars.capacity(), Some(26));
    assert_eq!(
        (0..26).map(|i| AbcCapsVars.var(i).to_string()).join(""),
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
    assert_eq!(AbcCapsVars.var(0).to_latex_string(), "A");
    assert_eq!(AbcCapsVars.var(0).to_typst_string(), "A");
    assert_eq!(AbcCapsVars.parse_var("A"), Some(0));
    assert_eq!(AbcCapsVars.parse_var("Z"), Some(25));
    assert_eq!(AbcCapsVars.parse_var("a"), None);
    assert_eq!(AbcCapsVars.parse_var("AB"), None);
}

#[test]
fn abc_vars_properties() {
    for names in [var_scheme_properties(&AbcVars, 100), var_scheme_properties(&AbcCapsVars, 100)] {
        assert_eq!(names.len(), 26);
    }
    let mut frags = Vec::new();
    for i in 0..26 {
        frags.push(AbcVars.var(i).to_typst_string());
        frags.push(AbcCapsVars.var(i).to_typst_string());
    }
    assert_typst_compiles(&frags);
}

#[test]
#[should_panic]
fn abc_vars_fail() {
    AbcVars.var(26).to_string();
}

#[test]
#[should_panic]
fn abc_caps_vars_fail() {
    AbcCapsVars.var(26).to_string();
}
