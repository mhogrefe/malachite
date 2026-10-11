// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_quadruple_gen_var_3, unsigned_quintuple_gen_var_1,
};
use std::panic::catch_unwind;

fn mod_power_of_2_sub_mul_shl_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, y: T, z: T, bits, pow, out| {
        assert_eq!(x.mod_power_of_2_sub_mul_shl(y, z, bits, pow), out);

        let mut x = x;
        x.mod_power_of_2_sub_mul_shl_assign(y, z, bits, pow);
        assert_eq!(x, out);
    };
    test(
        T::exact_from(0u8),
        T::exact_from(0u8),
        T::exact_from(0u8),
        0,
        0,
        T::exact_from(0u8),
    );
    test(
        T::exact_from(3u8),
        T::exact_from(2u8),
        T::exact_from(5u8),
        1,
        5,
        T::exact_from(15u8),
    );
    test(
        T::exact_from(10u8),
        T::exact_from(14u8),
        T::exact_from(3u8),
        2,
        6,
        T::exact_from(34u8),
    );
    test(
        T::exact_from(7u8),
        T::exact_from(1u8),
        T::exact_from(1u8),
        8,
        8,
        T::exact_from(7u8),
    );
    test(
        T::exact_from(7u8),
        T::exact_from(5u8),
        T::exact_from(3u8),
        9,
        8,
        T::exact_from(7u8),
    );
    test(
        T::exact_from(200u8),
        T::exact_from(100u8),
        T::exact_from(3u8),
        3,
        8,
        T::exact_from(104u8),
    );
    test(
        T::exact_from(1u8),
        T::exact_from(1u8),
        T::exact_from(1u8),
        0,
        1,
        T::exact_from(0u8),
    );
    test(T::MAX, T::MAX, T::MAX, 1, T::WIDTH, T::MAX - T::TWO);
    test(T::MAX, T::MAX, T::MAX, T::WIDTH, T::WIDTH, T::MAX);
}

#[test]
fn test_mod_power_of_2_sub_mul_shl() {
    apply_fn_to_unsigneds!(mod_power_of_2_sub_mul_shl_helper);
}

fn mod_power_of_2_sub_mul_shl_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ONE.mod_power_of_2_sub_mul_shl(T::ZERO, T::ZERO, 0, 0));
    assert_panic!(T::ZERO.mod_power_of_2_sub_mul_shl(T::ONE, T::ZERO, 0, 0));
    assert_panic!(T::ZERO.mod_power_of_2_sub_mul_shl(T::ZERO, T::ONE, 0, 0));
    assert_panic!(T::ZERO.mod_power_of_2_sub_mul_shl(T::ZERO, T::ZERO, 0, T::WIDTH + 1));
    assert_panic!({
        let mut x = T::ONE;
        x.mod_power_of_2_sub_mul_shl_assign(T::ZERO, T::ZERO, 0, 0);
    });
    assert_panic!({
        let mut x = T::ZERO;
        x.mod_power_of_2_sub_mul_shl_assign(T::ZERO, T::ZERO, 0, T::WIDTH + 1);
    });
}

#[test]
fn mod_power_of_2_sub_mul_shl_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_sub_mul_shl_fail_helper);
}

fn mod_power_of_2_sub_mul_shl_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_quintuple_gen_var_1::<T>().test_properties(|(x, y, z, bits, pow)| {
        let w = x.mod_power_of_2_sub_mul_shl(y, z, bits, pow);
        assert!(w.mod_power_of_2_is_reduced(pow));

        let mut x_alt = x;
        x_alt.mod_power_of_2_sub_mul_shl_assign(y, z, bits, pow);
        assert_eq!(x_alt, w);

        // It is the unfused combination, and the factors commute.
        assert_eq!(
            w,
            x.mod_power_of_2_sub(
                y.mod_power_of_2_mul(z, pow).mod_power_of_2_shl(bits, pow),
                pow
            )
        );
        assert_eq!(x.mod_power_of_2_sub_mul_shl(z, y, bits, pow), w);
        // Negating a factor gives the opposite operation, which undoes this one.
        assert_eq!(
            x.mod_power_of_2_add_mul_shl(y.mod_power_of_2_neg(pow), z, bits, pow),
            w
        );
        assert_eq!(w.mod_power_of_2_add_mul_shl(y, z, bits, pow), x);
        // Negating everything negates the result.
        assert_eq!(
            x.mod_power_of_2_neg(pow).mod_power_of_2_sub_mul_shl(
                y.mod_power_of_2_neg(pow),
                z,
                bits,
                pow
            ),
            w.mod_power_of_2_neg(pow)
        );
        // Shifting by at least `pow` changes nothing.
        if bits >= pow {
            assert_eq!(w, x);
        }
    });

    unsigned_quadruple_gen_var_3::<T>().test_properties(|(x, y, z, pow)| {
        // With no shift, this is `mod_power_of_2_sub_mul`.
        assert_eq!(
            x.mod_power_of_2_sub_mul_shl(y, z, 0, pow),
            x.mod_power_of_2_sub_mul(y, z, pow)
        );
        // Shifts compose.
        if pow != 0 {
            assert_eq!(
                x.mod_power_of_2_sub_mul_shl(y.mod_power_of_2_shl(1u64, pow), z, 0, pow),
                x.mod_power_of_2_sub_mul_shl(y, z, 1, pow)
            );
        }
    });
}

#[test]
fn mod_power_of_2_sub_mul_shl_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_sub_mul_shl_properties_helper);
}
