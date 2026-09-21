// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::ToDebugString;
use malachite_base::strings::latex::ToLatex;
use malachite_base::strings::typst::ToTypst;
use malachite_base::test_util::vars::var_scheme_properties;
use malachite_base::vars::abc::AbcVars;
use malachite_base::vars::char_is_reserved;
use malachite_base::vars::indexed::IndexedVars;
use malachite_base::vars::xyz::XyzVars;
use malachite_base::vars::{Var, VarScheme};
use std::fmt::{Formatter, Result};

// The least a scheme can say for itself: three methods, with the two markup names left to their
// defaults.
struct PrimedVars;

impl VarScheme for PrimedVars {
    fn capacity(&self) -> Option<usize> {
        Some(3)
    }

    fn fmt_var(&self, index: usize, f: &mut Formatter) -> Result {
        write!(f, "{}", ["p", "p'", "p''"][index])
    }

    fn parse_var(&self, name: &str) -> Option<usize> {
        ["p", "p'", "p''"].iter().position(|&x| x == name)
    }
}

#[test]
fn test_char_is_reserved() {
    for c in "0123456789+-*^/(), \t\n".chars() {
        assert!(char_is_reserved(c), "{c:?}");
    }
    for c in "xyzXYZ_'`.!?αΑ₀⁰".chars() {
        assert!(!char_is_reserved(c), "{c:?}");
    }
}

#[test]
fn test_var() {
    assert_eq!(XyzVars.var(3).index(), 3);
    assert_eq!(XyzVars.var(3).scheme(), &XyzVars);
    assert_eq!(AbcVars.var(3).scheme().capacity(), Some(26));
    assert_eq!(IndexedVars.var(10).to_debug_string(), "x₁₀");
    // A handle is `Copy`, so writing it does not consume it.
    let v = XyzVars.var(0);
    let w = v;
    assert_eq!(v.to_string(), w.to_string());
}

#[test]
fn test_var_dyn() {
    // A scheme can be chosen at run time, which is what makes a `dyn` one worth having.
    let schemes: [&dyn VarScheme; 3] = [&XyzVars, &AbcVars, &IndexedVars];
    assert_eq!(
        schemes
            .iter()
            .map(|&s| Var::new(s, 1).to_string())
            .collect::<Vec<_>>(),
        ["y", "b", "x₁"]
    );
    assert_eq!(
        Var::new(&XyzVars as &dyn VarScheme, 1).to_latex_string(),
        "y"
    );
    assert_eq!(
        Var::new(&XyzVars as &dyn VarScheme, 1).to_typst_string(),
        "y"
    );
}

#[test]
fn test_var_scheme_defaults() {
    assert_eq!(PrimedVars.var(0).to_string(), "p");
    assert_eq!(PrimedVars.var(2).to_string(), "p''");
    // One ASCII letter is written bare, and so set in math italics; anything longer is set upright.
    assert_eq!(PrimedVars.var(0).to_latex_string(), "p");
    assert_eq!(PrimedVars.var(1).to_latex_string(), r"\text{p'}");
    assert_eq!(PrimedVars.var(0).to_typst_string(), "p");
    assert_eq!(PrimedVars.var(1).to_typst_string(), "\"p'\"");
    assert_eq!(var_scheme_properties(&PrimedVars, 100).len(), 3);
}
