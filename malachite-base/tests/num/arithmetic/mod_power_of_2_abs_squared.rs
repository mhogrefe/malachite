// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_pair_gen_var_17;
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_abs_squared() {
    fn test<T: PrimitiveUnsigned>(x: T, pow: u64, out: T) {
        assert_eq!(x.mod_power_of_2_abs_squared(pow), out);
        let mut mut_x = x;
        mut_x.mod_power_of_2_abs_squared_assign(pow);
        assert_eq!(mut_x, out);
    }
    test::<u8>(0, 0, 0);
    test::<u8>(0, 5, 0);
    test::<u8>(5, 3, 1);
    test::<u8>(255, 8, 1);
    test::<u32>(100, 8, 16);
    test::<u64>(18446744073709551615, 64, 1);
}

fn mod_power_of_2_abs_squared_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ONE.mod_power_of_2_abs_squared(T::WIDTH + 1));
    assert_panic!(T::exact_from(8u8).mod_power_of_2_abs_squared(3));
    assert_panic!({
        let mut x = T::exact_from(8u8);
        x.mod_power_of_2_abs_squared_assign(3);
    });
}

#[test]
fn mod_power_of_2_abs_squared_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_abs_squared_fail_helper);
}

fn mod_power_of_2_abs_squared_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_17::<T>().test_properties(|(x, pow)| {
        let y = x.mod_power_of_2_abs_squared(pow);
        assert!(y.mod_power_of_2_is_reduced(pow));
        let mut mut_x = x;
        mut_x.mod_power_of_2_abs_squared_assign(pow);
        assert_eq!(mut_x, y);
        // For an unsigned number it is the square.
        assert_eq!(x.mod_power_of_2_square(pow), y);
    });
}

#[test]
fn mod_power_of_2_abs_squared_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_abs_squared_properties_helper);
}
