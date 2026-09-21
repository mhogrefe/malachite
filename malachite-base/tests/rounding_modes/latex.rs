// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::rounding_modes::ROUNDING_MODES;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::strings::latex::ToLatex;

#[test]
fn test_to_latex() {
    // These six assertions cover the whole domain.
    assert_eq!(Down.to_latex_string(), r"\text{DOWN}");
    assert_eq!(Up.to_latex_string(), r"\text{UP}");
    assert_eq!(Floor.to_latex_string(), r"\text{FLOOR}");
    assert_eq!(Ceiling.to_latex_string(), r"\text{CEILING}");
    assert_eq!(Nearest.to_latex_string(), r"\text{NEAREST}");
    assert_eq!(Exact.to_latex_string(), r"\text{EXACT}");
}

#[test]
fn to_latex_distinguishes_rounding_modes() {
    // `ROUNDING_MODES` is the entire domain, so this establishes injectivity outright rather than
    // sampling for it.
    let fragments = ROUNDING_MODES
        .iter()
        .map(ToLatex::to_latex_string)
        .collect_vec();
    assert_eq!(fragments.iter().unique().count(), fragments.len());
}
