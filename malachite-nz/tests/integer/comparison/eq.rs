// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::common::test_eq_helper;
use malachite_base::test_util::generators::signed_pair_gen;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::generators::{
    integer_gen, integer_pair_gen, integer_triple_gen, natural_pair_gen,
};
use num::BigInt;
use rug;

#[test]
fn test_eq() {
    let strings = &["0", "1", "-1", "2", "-2", "123", "-123", "1000000000000", "-1000000000000"];
    test_eq_helper::<Integer>(strings);
    test_eq_helper::<BigInt>(strings);
    test_eq_helper::<rug::Integer>(strings);
}

#[allow(clippy::cmp_owned, clippy::eq_op)]
#[test]
fn eq_properties() {
    integer_pair_gen().test_properties(|(x, y)| {
        let eq = x == y;
        assert_eq!(BigInt::from(&x) == BigInt::from(&y), eq);
        assert_eq!(rug::Integer::from(&x) == rug::Integer::from(&y), eq);
        assert_eq!(y == x, eq);
    });

    integer_gen().test_properties(|x| {
        assert_eq!(x, x);
    });

    integer_triple_gen().test_properties(|(x, y, z)| {
        if x == y && y == z {
            assert_eq!(x, z);
        }
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(Integer::from(&x) == Integer::from(&y), x == y);
        assert_eq!(Integer::from(&x) == y, x == y);
        assert_eq!(x == Integer::from(&y), x == y);
    });

    signed_pair_gen::<SignedLimb>().test_properties(|(x, y)| {
        assert_eq!(Integer::from(x) == Integer::from(y), x == y);
    });
}
