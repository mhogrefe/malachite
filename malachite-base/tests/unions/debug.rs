// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::ToDebugString;
use malachite_base::unions::Union2;

#[test]
fn test_to_debug() {
    let test = |u: Union2<Vec<char>, u32>, out| {
        assert_eq!(u.to_debug_string(), out);
    };
    test(Union2::A(vec![]), "A([])");
    test(Union2::A(vec!['a', 'b', 'c']), "A(['a', 'b', 'c'])");
    test(Union2::B(5), "B(5)");
}
