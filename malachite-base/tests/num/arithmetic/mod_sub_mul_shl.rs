// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModShl;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_quadruple_gen_var_4, unsigned_quintuple_gen_var_2,
};
use std::panic::catch_unwind;

fn mod_sub_mul_shl_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, y: T, z: T, bits, m, out| {
        assert_eq!(x.mod_sub_mul_shl(y, z, bits, m), out);

        let mut x = x;
        x.mod_sub_mul_shl_assign(y, z, bits, m);
        assert_eq!(x, out);
    };
    test(
        T::exact_from(0u8),
        T::exact_from(0u8),
        T::exact_from(0u8),
        0,
        T::exact_from(1u8),
        T::exact_from(0u8),
    );
    test(
        T::exact_from(3u8),
        T::exact_from(2u8),
        T::exact_from(5u8),
        1,
        T::exact_from(7u8),
        T::exact_from(4u8),
    );
    test(
        T::exact_from(10u8),
        T::exact_from(14u8),
        T::exact_from(3u8),
        2,
        T::exact_from(15u8),
        T::exact_from(7u8),
    );
    test(
        T::exact_from(200u8),
        T::exact_from(100u8),
        T::exact_from(3u8),
        40,
        T::exact_from(255u8),
        T::exact_from(155u8),
    );
    test(
        T::exact_from(6u8),
        T::exact_from(6u8),
        T::exact_from(6u8),
        100,
        T::exact_from(7u8),
        T::exact_from(4u8),
    );
    test(
        T::exact_from(1u8),
        T::exact_from(1u8),
        T::exact_from(1u8),
        0,
        T::exact_from(2u8),
        T::exact_from(0u8),
    );
    test(
        T::exact_from(5u8),
        T::exact_from(3u8),
        T::exact_from(4u8),
        3,
        T::exact_from(8u8),
        T::exact_from(5u8),
    );
}

#[test]
fn test_mod_sub_mul_shl() {
    apply_fn_to_unsigneds!(mod_sub_mul_shl_helper);
}

fn mod_sub_mul_shl_fail_helper<T: PrimitiveUnsigned>() {
    let seven = T::exact_from(7u8);
    assert_panic!(T::ZERO.mod_sub_mul_shl(T::ZERO, T::ZERO, 0, T::ZERO));
    assert_panic!(seven.mod_sub_mul_shl(T::ONE, T::ONE, 1, seven));
    assert_panic!(T::ONE.mod_sub_mul_shl(seven, T::ONE, 1, seven));
    assert_panic!(T::ONE.mod_sub_mul_shl(T::ONE, seven, 1, seven));
    assert_panic!({
        let mut x = seven;
        x.mod_sub_mul_shl_assign(T::ONE, T::ONE, 1, seven);
    });
}

#[test]
fn mod_sub_mul_shl_fail() {
    apply_fn_to_unsigneds!(mod_sub_mul_shl_fail_helper);
}

fn mod_sub_mul_shl_properties_helper<T: PrimitiveUnsigned + ModShl<u64, T, Output = T>>() {
    unsigned_quintuple_gen_var_2::<T>().test_properties(|(x, y, z, bits, m)| {
        let w = x.mod_sub_mul_shl(y, z, bits, m);
        assert!(w.mod_is_reduced(&m));

        let mut x_alt = x;
        x_alt.mod_sub_mul_shl_assign(y, z, bits, m);
        assert_eq!(x_alt, w);

        // It is the unfused combination, and the factors commute.
        assert_eq!(w, x.mod_sub(y.mod_mul(z, m).mod_shl(bits, m), m));
        assert_eq!(x.mod_sub_mul_shl(z, y, bits, m), w);
        // Negating a factor gives the opposite operation, which undoes this one.
        assert_eq!(x.mod_add_mul_shl(y.mod_neg(m), z, bits, m), w);
        assert_eq!(w.mod_add_mul_shl(y, z, bits, m), x);
        // Negating everything negates the result.
        assert_eq!(
            x.mod_neg(m).mod_sub_mul_shl(y.mod_neg(m), z, bits, m),
            w.mod_neg(m)
        );
        // Shifts compose.
        assert_eq!(
            x.mod_sub_mul_shl(y.mod_shl(1u64, m), z, bits, m),
            x.mod_sub_mul_shl(y, z, bits + 1, m)
        );
    });

    unsigned_quadruple_gen_var_4::<T>().test_properties(|(x, y, z, m)| {
        // With no shift, this is `mod_sub_mul`.
        assert_eq!(x.mod_sub_mul_shl(y, z, 0, m), x.mod_sub_mul(y, z, m));
    });
}

#[test]
fn mod_sub_mul_shl_properties() {
    apply_fn_to_unsigneds!(mod_sub_mul_shl_properties_helper);
}
