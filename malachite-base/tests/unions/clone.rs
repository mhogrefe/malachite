// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::unions::Union2;

#[allow(clippy::redundant_clone)]
#[test]
fn test_clone() {
    let test = |u: Union2<Vec<char>, u32>| {
        let cloned = u.clone();
        assert_eq!(cloned, u);
    };
    test(Union2::A(vec![]));
    test(Union2::A(vec!['a', 'b', 'c']));
    test(Union2::B(5));
}

#[test]
fn test_clone_from() {
    let test = |mut u: Union2<Vec<char>, u32>, v: Union2<Vec<char>, u32>| {
        u.clone_from(&v);
        assert_eq!(u, v);
    };
    test(Union2::A(vec!['a', 'b', 'c']), Union2::A(vec![]));
    test(Union2::A(vec![]), Union2::A(vec!['a', 'b', 'c']));
    test(Union2::B(5), Union2::B(6));
    test(Union2::A(vec!['a', 'b', 'c']), Union2::B(6));
    test(Union2::B(6), Union2::A(vec!['a', 'b', 'c']));
}
