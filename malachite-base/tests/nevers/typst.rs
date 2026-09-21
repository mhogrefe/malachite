// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::typst::assert_typst_compiles;
use malachite_base::nevers::Never;
use malachite_base::strings::typst::ToTypst;

// A `Never` cannot be instantiated, so the implementation can only be reached through a type that
// holds one without having one, which is what it is for.
#[test]
fn test_never_to_typst() {
    assert_eq!(None::<Never>.to_typst_string(), "bot");
    assert_eq!(None::<Option<Never>>.to_typst_string(), "bot");
    assert_eq!(Some(None::<Never>).to_typst_string(), "[bot]");
    assert_typst_compiles(&[
        None::<Never>.to_typst_string(),
        Some(None::<Never>).to_typst_string(),
    ]);
}
