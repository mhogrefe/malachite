// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::common::test_eq_helper;
use malachite_base::test_util::generators::unsigned_pair_gen_var_27;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_gen, natural_pair_gen, natural_triple_gen};
use num::BigUint;
use rug;

#[test]
fn test_eq() {
    let strings = vec!["0", "1", "2", "123", "1000000000000"];
    test_eq_helper::<Natural>(&strings);
    test_eq_helper::<BigUint>(&strings);
    test_eq_helper::<rug::Integer>(&strings);
}

#[allow(clippy::cmp_owned, clippy::eq_op)]
#[test]
fn eq_properties() {
    natural_pair_gen().test_properties(|(x, y)| {
        let eq = x == y;
        assert_eq!(BigUint::from(&x) == BigUint::from(&y), eq);
        assert_eq!(rug::Integer::from(&x) == rug::Integer::from(&y), eq);
        assert_eq!(y == x, eq);
    });

    natural_gen().test_properties(|x| {
        assert_eq!(x, x);
    });

    natural_triple_gen().test_properties(|(x, y, z)| {
        if x == y && y == z {
            assert_eq!(x, z);
        }
    });

    unsigned_pair_gen_var_27::<Limb>().test_properties(|(x, y)| {
        assert_eq!(Natural::from(x) == Natural::from(y), x == y);
    });
}
