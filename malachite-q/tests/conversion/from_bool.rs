// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_q::Rational;

#[test]
fn test_from_bool() {
    let test = |b, s| {
        let n = Rational::from(b);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), s);
    };
    test(false, "0");
    test(true, "1");
}
