// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::nevers::Never;
use malachite_base::strings::latex::ToLatex;

// A `Never` cannot be instantiated, so the implementation can only be reached through a type that
// holds one without having one, which is what it is for.
#[test]
fn test_never_to_latex() {
    assert_eq!(None::<Never>.to_latex().to_string(), r"\bot");
    assert_eq!(None::<Option<Never>>.to_latex().to_string(), r"\bot");
    assert_eq!(
        Some(None::<Never>).to_latex().to_string(),
        r"\left[\bot\right]"
    );
}
