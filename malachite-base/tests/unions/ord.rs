// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::Union3;
use malachite_base::test_util::common::test_cmp_helper;

#[test]
fn test_cmp() {
    test_cmp_helper::<Union3<char, u32, bool>>(&[
        "A(a)", "A(d)", "B(5)", "B(8)", "C(false)", "C(true)",
    ]);
}
