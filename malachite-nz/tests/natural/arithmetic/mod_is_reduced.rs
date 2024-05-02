// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModIsReduced;
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::unsigned_pair_gen_var_12;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_pair_gen_var_5;
use std::str::FromStr;

#[test]
fn test_mod_is_reduced() {
    let test = |u, v, out| {
        assert_eq!(
            Natural::from_str(u)
                .unwrap()
                .mod_is_reduced(&Natural::from_str(v).unwrap()),
            out
        );
    };

    test("0", "5", true);
    test("100", "100", false);
    test("100", "101", true);
    test("1000000000000", "1000000000000", false);
    test("1000000000000", "1000000000001", true);
}

#[test]
#[should_panic]
fn mod_is_reduced_fail() {
    Natural::from(123u32).mod_is_reduced(&Natural::ZERO);
}

#[test]
fn mod_is_reduced_properties() {
    natural_pair_gen_var_5().test_properties(|(n, m)| {
        assert_eq!(n.mod_is_reduced(&m), &n % m == n);
    });

    unsigned_pair_gen_var_12::<Limb, Limb>().test_properties(|(n, m)| {
        assert_eq!(
            n.mod_is_reduced(&m),
            Natural::from(n).mod_is_reduced(&Natural::from(m))
        );
    });
}
