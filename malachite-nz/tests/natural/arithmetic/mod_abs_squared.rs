// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    Mod, ModAbsSquared, ModAbsSquaredAssign, ModIsReduced, ModSquare, Square,
};
use malachite_base::test_util::generators::unsigned_pair_gen_var_16;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_pair_gen_var_8;

#[test]
fn test_mod_abs_squared() {
    let test = |s, m, out| {
        let x = Natural::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let y = (&x).mod_abs_squared(&m);
        assert!(y.is_valid());
        assert_eq!(y.to_string(), out);
        assert_eq!((&x).mod_abs_squared(m.clone()), y);
        assert_eq!(x.clone().mod_abs_squared(&m), y);
        assert_eq!(x.clone().mod_abs_squared(m.clone()), y);
        let mut mut_x = x.clone();
        mut_x.mod_abs_squared_assign(&m);
        assert_eq!(mut_x, y);
        let mut mut_x = x;
        mut_x.mod_abs_squared_assign(m);
        assert_eq!(mut_x, y);
    };
    test("0", "1", "0");
    test("2", "10", "4");
    test("100", "497", "60");
    test(
        "1000000000000000000000000000056",
        "1000000000000000000000000000057",
        "1",
    );
}

#[test]
#[should_panic]
fn mod_abs_squared_fail() {
    Natural::from(7u32).mod_abs_squared(Natural::from(7u32));
}

#[test]
fn mod_abs_squared_properties() {
    natural_pair_gen_var_8().test_properties(|(x, m)| {
        let y = (&x).mod_abs_squared(&m);
        assert!(y.is_valid());
        assert!(y.mod_is_reduced(&m));
        assert_eq!((&x).mod_abs_squared(m.clone()), y);
        assert_eq!(x.clone().mod_abs_squared(&m), y);
        assert_eq!(x.clone().mod_abs_squared(m.clone()), y);
        let mut mut_x = x.clone();
        mut_x.mod_abs_squared_assign(&m);
        assert_eq!(mut_x, y);
        // For a Natural it is the square, reduced.
        assert_eq!((&x).mod_square(&m), y);
        assert_eq!(x.square().mod_op(&m), y);
    });

    unsigned_pair_gen_var_16::<Limb>().test_properties(|(x, m)| {
        // It agrees with the implementation for limbs.
        assert_eq!(
            Natural::from(x).mod_abs_squared(Natural::from(m)),
            Natural::from(x.mod_abs_squared(m))
        );
    });
}
