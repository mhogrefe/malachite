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
use malachite_base::strings::latex::ToLatex;
use malachite_base::strings::typst::ToTypst;
use malachite_base::test_util::vars::var_scheme_properties;
use malachite_base::vars::VarScheme;
use malachite_base::vars::indexed::{IndexedCapsVars, IndexedVars};

#[test]
fn test_indexed_vars() {
    assert_eq!(IndexedVars.capacity(), None);
    assert_eq!(IndexedVars.var(0).to_string(), "x₀");
    assert_eq!(IndexedVars.var(9).to_string(), "x₉");
    assert_eq!(IndexedVars.var(10).to_string(), "x₁₀");
    assert_eq!(IndexedVars.var(123).to_string(), "x₁₂₃");
    assert_eq!(IndexedVars.var(0).to_latex_string(), "x_0");
    assert_eq!(IndexedVars.var(9).to_latex_string(), "x_9");
    assert_eq!(IndexedVars.var(10).to_latex_string(), "x_{10}");
    assert_eq!(IndexedVars.var(123).to_latex_string(), "x_{123}");
    assert_eq!(IndexedVars.var(0).to_typst_string(), "x_0");
    assert_eq!(IndexedVars.var(10).to_typst_string(), "x_(10)");
    assert_eq!(IndexedVars.var(123).to_typst_string(), "x_(123)");
    assert_eq!(IndexedVars.parse_var("x₀"), Some(0));
    assert_eq!(IndexedVars.parse_var("x₁₀"), Some(10));
    // The subscript is the only spelling of the index: no ASCII digits, no leading zero, and never
    // an empty one.
    assert_eq!(IndexedVars.parse_var("x10"), None);
    assert_eq!(IndexedVars.parse_var("x₀₁"), None);
    assert_eq!(IndexedVars.parse_var("x"), None);
    assert_eq!(IndexedVars.parse_var("X₀"), None);
    assert_eq!(IndexedVars.parse_var("y₀"), None);
    assert_eq!(IndexedVars.parse_var("₀"), None);
}

#[test]
fn test_indexed_caps_vars() {
    assert_eq!(IndexedCapsVars.capacity(), None);
    assert_eq!(IndexedCapsVars.var(0).to_string(), "X₀");
    assert_eq!(IndexedCapsVars.var(10).to_string(), "X₁₀");
    assert_eq!(IndexedCapsVars.var(10).to_latex_string(), "X_{10}");
    assert_eq!(IndexedCapsVars.var(10).to_typst_string(), "X_(10)");
    assert_eq!(IndexedCapsVars.parse_var("X₀"), Some(0));
    assert_eq!(IndexedCapsVars.parse_var("x₀"), None);
}

#[test]
fn indexed_vars_properties() {
    // The scheme can name any number of variables, so the check is of the first few hundred.
    assert_eq!(var_scheme_properties(&IndexedVars, 300).len(), 300);
    assert_eq!(var_scheme_properties(&IndexedCapsVars, 300).len(), 300);
    let mut frags = Vec::new();
    for i in (0..300).chain([1000, 12345]) {
        for s in [IndexedVars.var(i).to_typst_string(), IndexedCapsVars.var(i).to_typst_string()] {
            assert_strings_closed(&s);
            assert_delimiters_balanced(&s);
            frags.push(s);
        }
        // A subscript is braced exactly when it is more than one digit long.
        let latex = IndexedVars.var(i).to_latex_string();
        assert_eq!(latex.contains('{'), i >= 10, "{latex:?}");
    }
    assert_typst_compiles(&frags);
}
