// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::typst::assert_typst_compiles;
use itertools::Itertools;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::strings::typst::ToTypst;

#[test]
fn test_to_typst() {
    let test = |rm: RoundingMode, out: &str| assert_eq!(rm.to_typst_string(), out);
    test(Down, r#""DOWN""#);
    test(Up, r#""UP""#);
    test(Floor, r#""FLOOR""#);
    test(Ceiling, r#""CEILING""#);
    test(Nearest, r#""NEAREST""#);
    test(Exact, r#""EXACT""#);
    assert_typst_compiles(
        &exhaustive_rounding_modes()
            .map(|rm| rm.to_typst_string())
            .collect_vec(),
    );
}

#[test]
fn to_typst_distinguishes_rounding_modes() {
    // Distinct modes never share a fragment.
    let fragments = exhaustive_rounding_modes()
        .map(|rm| rm.to_typst_string())
        .collect_vec();
    assert_eq!(fragments.iter().unique().count(), fragments.len());
}
