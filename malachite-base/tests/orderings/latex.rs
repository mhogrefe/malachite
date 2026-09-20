// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::*;
use malachite_base::strings::latex::ToLatex;

#[test]
fn test_to_latex() {
    // An `Ordering` has three values, so these three assertions cover the whole domain.
    assert_eq!(Less.to_latex().to_string(), "<");
    assert_eq!(Equal.to_latex().to_string(), "=");
    assert_eq!(Greater.to_latex().to_string(), ">");
}
