// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::Union3;

#[test]
fn test_to_string() {
    let test = |u: Union3<char, u32, bool>, out| {
        assert_eq!(u.to_string(), out);
    };
    test(Union3::A('a'), "A(a)");
    test(Union3::B(5), "B(5)");
    test(Union3::C(false), "C(false)");
}
