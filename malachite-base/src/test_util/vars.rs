// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::latex::ToLatex;
use crate::strings::typst::ToTypst;
use crate::vars::{VarScheme, char_is_reserved};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Checks that a scheme keeps the [`VarScheme`](crate::vars::VarScheme) contract, and gives back
/// the names it wrote so that a caller can go on to check them against something else.
///
/// Every variable the scheme can name is checked, or the first `limit` of them if it can name any
/// number.
pub fn var_scheme_properties<S: VarScheme>(scheme: &S, limit: usize) -> Vec<String> {
    let n = scheme.capacity().map_or(limit, |c| c.min(limit));
    let mut names = Vec::with_capacity(n);
    for index in 0..n {
        let name = scheme.var(index).to_string();
        assert!(!name.is_empty(), "the name of variable {index} is empty");
        if let Some(c) = name.chars().find(|&c| char_is_reserved(c)) {
            panic!("the name {name:?} of variable {index} holds the reserved character {c:?}");
        }
        assert_eq!(
            scheme.parse_var(&name),
            Some(index),
            "the name {name:?} of variable {index} does not read back as it"
        );
        // A name is written the same way every time, and so are the two markup names; nothing here
        // depends on how often it has been asked for.
        assert_eq!(scheme.var(index).to_string(), name);
        assert!(!scheme.var(index).to_latex_string().is_empty());
        assert!(!scheme.var(index).to_typst_string().is_empty());
        names.push(name);
    }
    let mut distinct = names.clone();
    distinct.sort_unstable();
    distinct.dedup();
    assert_eq!(
        distinct.len(),
        names.len(),
        "two variables have the same name"
    );
    // Nothing that could not be a name is one.
    for bad in ["", " ", "+", "^", "0"] {
        assert_eq!(scheme.parse_var(bad), None, "{bad:?} reads as a variable");
    }
    names
}
