// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::typst::assert_typst_compiles;
use malachite_base::strings::latex::ToLatex;
use malachite_base::strings::typst::ToTypst;
use malachite_base::test_util::vars::var_scheme_properties;
use malachite_base::vars::VarScheme;
use malachite_base::vars::list::ListVars;

#[test]
fn test_list_vars() {
    let vars = ListVars::new(["t", "price", "α", "x₀"]);
    assert_eq!(vars.capacity(), Some(4));
    assert_eq!(vars.names(), &["t", "price", "α", "x₀"]);
    assert_eq!(vars.var(0).to_string(), "t");
    assert_eq!(vars.var(1).to_string(), "price");
    // A name of one ASCII letter is a variable in the ordinary sense, and is set in math italics; a
    // longer one is set upright.
    assert_eq!(vars.var(0).to_latex_string(), "t");
    assert_eq!(vars.var(1).to_latex_string(), r"\text{price}");
    assert_eq!(vars.var(2).to_latex_string(), r"\alpha");
    assert_eq!(vars.var(3).to_latex_string(), r"\text{x}_0");
    assert_eq!(vars.var(0).to_typst_string(), "t");
    assert_eq!(vars.var(1).to_typst_string(), "\"price\"");
    assert_eq!(vars.var(2).to_typst_string(), "\"α\"");
    assert_eq!(vars.parse_var("t"), Some(0));
    assert_eq!(vars.parse_var("price"), Some(1));
    assert_eq!(vars.parse_var("cost"), None);
    assert_eq!(vars.parse_var("pric"), None);
    assert_eq!(ListVars::new(Vec::<String>::new()).capacity(), Some(0));
}

#[test]
fn list_vars_properties() {
    for names in [
        vec!["t"],
        vec!["x", "y", "z"],
        vec!["price", "quantity"],
        vec!["α", "β"],
        vec!["a", "aa", "aaa"],
        vec!["%", "\\", "{", "&"],
    ] {
        let vars = ListVars::new(names.clone());
        assert_eq!(var_scheme_properties(&vars, 100), names);
        let frags: Vec<String> = (0..names.len())
            .map(|i| vars.var(i).to_typst_string())
            .collect();
        assert_typst_compiles(&frags);
    }
}

#[test]
#[should_panic]
fn list_vars_new_fail_empty_name() {
    ListVars::new(["x", ""]);
}

#[test]
#[should_panic]
fn list_vars_new_fail_reserved() {
    ListVars::new(["x", "y^2"]);
}

#[test]
#[should_panic]
fn list_vars_new_fail_digit() {
    ListVars::new(["x0"]);
}

#[test]
#[should_panic]
fn list_vars_new_fail_duplicate() {
    ListVars::new(["x", "y", "x"]);
}

#[test]
#[should_panic]
fn list_vars_fail() {
    ListVars::new(["x"]).var(1).to_string();
}
