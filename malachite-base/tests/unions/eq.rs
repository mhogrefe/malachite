// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::Union3;
use malachite_base::test_util::common::test_eq_helper;

#[test]
fn test_eq() {
    test_eq_helper::<Union3<char, u32, bool>>(&[
        "B(8)", "A(d)", "C(true)", "B(5)", "C(false)", "A(a)",
    ]);
}
