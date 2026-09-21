// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::chars::latex::{assert_braces_balanced, assert_specials_escaped};
use crate::strings::typst::assert_typst_compiles;
use itertools::Itertools;
use malachite_base::strings::latex::ToLatex;
use malachite_base::strings::typst::ToTypst;
use malachite_base::test_util::vars::var_scheme_properties;
use malachite_base::vars::VarScheme;
use malachite_base::vars::greek::{GreekCapsVars, GreekVars};

#[test]
fn test_greek_vars() {
    assert_eq!(GreekVars.capacity(), Some(24));
    assert_eq!(
        (0..24).map(|i| GreekVars.var(i).to_string()).join(""),
        "αβγδεζηθικλμνξοπρστυφχψω"
    );
    assert_eq!(
        (0..24)
            .map(|i| GreekVars.var(i).to_latex_string())
            .collect_vec(),
        [
            r"\alpha",
            r"\beta",
            r"\gamma",
            r"\delta",
            r"\varepsilon",
            r"\zeta",
            r"\eta",
            r"\theta",
            r"\iota",
            r"\kappa",
            r"\lambda",
            r"\mu",
            r"\nu",
            r"\xi",
            "o",
            r"\pi",
            r"\rho",
            r"\sigma",
            r"\tau",
            r"\upsilon",
            r"\varphi",
            r"\chi",
            r"\psi",
            r"\omega"
        ]
    );
    assert_eq!(
        (0..24).map(|i| GreekVars.var(i).to_typst_string()).join(""),
        "αβγδεζηθικλμνξοπρστυφχψω"
    );
    assert_eq!(GreekVars.parse_var("α"), Some(0));
    assert_eq!(GreekVars.parse_var("σ"), Some(17));
    assert_eq!(GreekVars.parse_var("ω"), Some(23));
    // The final sigma is a second form of a letter already named, so it is not a name of its own.
    assert_eq!(GreekVars.parse_var("ς"), None);
    assert_eq!(GreekVars.parse_var("Α"), None);
    assert_eq!(GreekVars.parse_var("a"), None);
    assert_eq!(GreekVars.parse_var("αβ"), None);
}

#[test]
fn test_greek_caps_vars() {
    assert_eq!(GreekCapsVars.capacity(), Some(24));
    assert_eq!(
        (0..24).map(|i| GreekCapsVars.var(i).to_string()).join(""),
        "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ"
    );
    assert_eq!(
        (0..24)
            .map(|i| GreekCapsVars.var(i).to_latex_string())
            .collect_vec(),
        [
            "A",
            "B",
            r"\Gamma",
            r"\Delta",
            "E",
            "Z",
            "H",
            r"\Theta",
            "I",
            "K",
            r"\Lambda",
            "M",
            "N",
            r"\Xi",
            "O",
            r"\Pi",
            "P",
            r"\Sigma",
            "T",
            r"\Upsilon",
            r"\Phi",
            "X",
            r"\Psi",
            r"\Omega"
        ]
    );
    assert_eq!(GreekCapsVars.parse_var("Α"), Some(0));
    assert_eq!(GreekCapsVars.parse_var("Ω"), Some(23));
    assert_eq!(GreekCapsVars.parse_var("α"), None);
    // The code point Unicode leaves unassigned among the capitals is not a name.
    assert_eq!(GreekCapsVars.parse_var("\u{3a2}"), None);
    assert_eq!(GreekCapsVars.parse_var("A"), None);
}

#[test]
fn greek_vars_properties() {
    for names in
        [var_scheme_properties(&GreekVars, 100), var_scheme_properties(&GreekCapsVars, 100)]
    {
        assert_eq!(names.len(), 24);
    }
    let mut frags = Vec::new();
    for i in 0..24 {
        for s in [GreekVars.var(i).to_latex_string(), GreekCapsVars.var(i).to_latex_string()] {
            assert_specials_escaped(&s);
            assert_braces_balanced(&s);
        }
        frags.push(GreekVars.var(i).to_typst_string());
        frags.push(GreekCapsVars.var(i).to_typst_string());
    }
    assert_typst_compiles(&frags);
}

#[test]
#[should_panic]
fn greek_vars_fail() {
    GreekVars.var(24).to_string();
}

#[test]
#[should_panic]
fn greek_caps_vars_fail() {
    GreekCapsVars.var(24).to_string();
}
