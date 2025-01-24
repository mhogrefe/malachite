// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModIsReduced, ModPowerOf2, ModPowerOf2IsReduced, PowerOf2,
};
use malachite_base::test_util::generators::unsigned_pair_gen_var_2;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_4;
use std::str::FromStr;

#[test]
fn test_mod_power_of_2_is_reduced() {
    let test = |u, pow, out| {
        assert_eq!(
            Natural::from_str(u).unwrap().mod_power_of_2_is_reduced(pow),
            out
        );
    };

    test("0", 5, true);
    test("100", 5, false);
    test("100", 8, true);
    test("1000000000000", 39, false);
    test("1000000000000", 40, true);
}

#[test]
fn mod_power_of_2_is_reduced_properties() {
    natural_unsigned_pair_gen_var_4().test_properties(|(n, pow)| {
        let is_reduced = n.mod_power_of_2_is_reduced(pow);
        assert_eq!(is_reduced, (&n).mod_power_of_2(pow) == n);
        assert_eq!(is_reduced, n.mod_is_reduced(&Natural::power_of_2(pow)));
    });

    unsigned_pair_gen_var_2::<Limb, u64>().test_properties(|(n, pow)| {
        assert_eq!(
            n.mod_power_of_2_is_reduced(pow),
            Natural::from(n).mod_power_of_2_is_reduced(pow)
        );
    });
}
