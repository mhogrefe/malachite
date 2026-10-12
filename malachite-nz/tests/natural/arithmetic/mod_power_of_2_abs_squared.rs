// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2AbsSquared, ModPowerOf2AbsSquaredAssign, ModPowerOf2IsReduced,
    ModPowerOf2Square, Square,
};
use malachite_base::test_util::generators::unsigned_pair_gen_var_17;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_11;

#[test]
fn test_mod_power_of_2_abs_squared() {
    let test = |s, pow, out| {
        let x = Natural::from_str(s).unwrap();
        let y = (&x).mod_power_of_2_abs_squared(pow);
        assert!(y.is_valid());
        assert_eq!(y.to_string(), out);
        assert_eq!(x.clone().mod_power_of_2_abs_squared(pow), y);
        let mut mut_x = x;
        mut_x.mod_power_of_2_abs_squared_assign(pow);
        assert_eq!(mut_x, y);
    };
    test("0", 0, "0");
    test("5", 3, "1");
    test("100", 8, "16");
    test("1267650600228229401496703205375", 100, "1");
}

#[test]
#[should_panic]
fn mod_power_of_2_abs_squared_fail() {
    Natural::from(8u32).mod_power_of_2_abs_squared(3);
}

#[test]
fn mod_power_of_2_abs_squared_properties() {
    natural_unsigned_pair_gen_var_11().test_properties(|(x, pow)| {
        let y = (&x).mod_power_of_2_abs_squared(pow);
        assert!(y.is_valid());
        assert!(y.mod_power_of_2_is_reduced(pow));
        assert_eq!(x.clone().mod_power_of_2_abs_squared(pow), y);
        let mut mut_x = x.clone();
        mut_x.mod_power_of_2_abs_squared_assign(pow);
        assert_eq!(mut_x, y);
        // For a Natural it is the square, reduced.
        assert_eq!((&x).mod_power_of_2_square(pow), y);
        assert_eq!(x.square().mod_power_of_2(pow), y);
    });

    unsigned_pair_gen_var_17::<Limb>().test_properties(|(x, pow)| {
        // It agrees with the implementation for limbs.
        assert_eq!(
            Natural::from(x).mod_power_of_2_abs_squared(pow),
            Natural::from(x.mod_power_of_2_abs_squared(pow))
        );
    });
}
