// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_pair_gen_var_16;
use std::panic::catch_unwind;

#[test]
fn test_mod_abs_squared() {
    fn test<T: PrimitiveUnsigned>(x: T, m: T, out: T) {
        assert_eq!(x.mod_abs_squared(m), out);
        let mut mut_x = x;
        mut_x.mod_abs_squared_assign(m);
        assert_eq!(mut_x, out);
    }
    test::<u8>(0, 1, 0);
    test::<u8>(0, 5, 0);
    test::<u8>(2, 10, 4);
    test::<u8>(254, 255, 1);
    test::<u32>(100, 497, 60);
    test::<u64>(18446744073709551614, 18446744073709551615, 1);
}

fn mod_abs_squared_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.mod_abs_squared(T::ZERO));
    assert_panic!(T::exact_from(7u8).mod_abs_squared(T::exact_from(7u8)));
    assert_panic!({
        let mut x = T::exact_from(7u8);
        x.mod_abs_squared_assign(T::exact_from(7u8));
    });
}

#[test]
fn mod_abs_squared_fail() {
    apply_fn_to_unsigneds!(mod_abs_squared_fail_helper);
}

fn mod_abs_squared_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_16::<T>().test_properties(|(x, m)| {
        let y = x.mod_abs_squared(m);
        assert!(y.mod_is_reduced(&m));
        let mut mut_x = x;
        mut_x.mod_abs_squared_assign(m);
        assert_eq!(mut_x, y);
        // For an unsigned number it is the square.
        assert_eq!(x.mod_square(m), y);
    });
}

#[test]
fn mod_abs_squared_properties() {
    apply_fn_to_unsigneds!(mod_abs_squared_properties_helper);
}
