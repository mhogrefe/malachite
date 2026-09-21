// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::latex::ToLatex;

#[test]
fn test_to_latex() {
    // A `bool` has two values, so these two assertions cover the whole domain.
    assert_eq!(true.to_latex_string(), r"\text{T}");
    assert_eq!(false.to_latex_string(), r"\text{F}");
}
