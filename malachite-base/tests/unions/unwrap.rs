// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::Union3;

#[test]
fn test_unwrap() {
    let test = |u: Union3<char, char, char>, out| {
        assert_eq!(u.unwrap(), out);
    };
    test(Union3::A('a'), 'a');
    test(Union3::B('b'), 'b');
    test(Union3::C('c'), 'c');
}
