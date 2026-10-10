// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_17, unsigned_quadruple_gen_var_3, unsigned_triple_gen_var_11,
};
use std::panic::catch_unwind;

fn mod_power_of_2_sub_mul_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, y: T, z: T, pow, out| {
        assert_eq!(x.mod_power_of_2_sub_mul(y, z, pow), out);

        let mut x = x;
        x.mod_power_of_2_sub_mul_assign(y, z, pow);
        assert_eq!(x, out);
    };
    test(T::ZERO, T::ZERO, T::ZERO, 0, T::ZERO);
    test(T::ONE, T::ZERO, T::ONE, 1, T::ONE);
    test(T::ZERO, T::ONE, T::ONE, 1, T::ONE);
    test(
        T::exact_from(13),
        T::TWO,
        T::exact_from(5),
        5,
        T::exact_from(3),
    );
    test(
        T::exact_from(4),
        T::exact_from(14),
        T::exact_from(3),
        4,
        T::exact_from(10),
    );
    test(
        T::exact_from(10),
        T::exact_from(100),
        T::exact_from(3),
        8,
        T::exact_from(222),
    );
    test(T::ZERO, T::TWO, T::ONE << (T::WIDTH - 1), T::WIDTH, T::ZERO);
    test(T::MAX, T::MAX, T::MAX, T::WIDTH, T::MAX - T::ONE);
}

#[test]
fn test_mod_power_of_2_sub_mul() {
    apply_fn_to_unsigneds!(mod_power_of_2_sub_mul_helper);
}

fn mod_power_of_2_sub_mul_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ONE.mod_power_of_2_sub_mul(T::ZERO, T::ZERO, 0));
    assert_panic!(T::ZERO.mod_power_of_2_sub_mul(T::ONE, T::ZERO, 0));
    assert_panic!(T::ZERO.mod_power_of_2_sub_mul(T::ZERO, T::ONE, 0));
    assert_panic!(T::from(200u8).mod_power_of_2_sub_mul(T::ONE, T::ONE, 7));
    assert_panic!(T::ONE.mod_power_of_2_sub_mul(T::from(200u8), T::ONE, 7));
    assert_panic!(T::ONE.mod_power_of_2_sub_mul(T::ONE, T::from(200u8), 7));
    assert_panic!(T::ZERO.mod_power_of_2_sub_mul(T::ZERO, T::ZERO, T::WIDTH + 1));
}

#[test]
fn mod_power_of_2_sub_mul_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_sub_mul_fail_helper);
}

fn mod_power_of_2_sub_mul_assign_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!({
        let mut x = T::ONE;
        x.mod_power_of_2_sub_mul_assign(T::ZERO, T::ZERO, 0);
    });
    assert_panic!({
        let mut x = T::ZERO;
        x.mod_power_of_2_sub_mul_assign(T::ONE, T::ZERO, 0);
    });
    assert_panic!({
        let mut x = T::ZERO;
        x.mod_power_of_2_sub_mul_assign(T::ZERO, T::ONE, 0);
    });
    assert_panic!({
        let mut x = T::ZERO;
        x.mod_power_of_2_sub_mul_assign(T::ZERO, T::ZERO, T::WIDTH + 1);
    });
}

#[test]
fn mod_power_of_2_sub_mul_assign_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_sub_mul_assign_fail_helper);
}

fn mod_power_of_2_sub_mul_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_quadruple_gen_var_3::<T>().test_properties(|(x, y, z, pow)| {
        let result = x.mod_power_of_2_sub_mul(y, z, pow);
        assert!(result.mod_power_of_2_is_reduced(pow));

        let mut x_alt = x;
        x_alt.mod_power_of_2_sub_mul_assign(y, z, pow);
        assert_eq!(x_alt, result);

        // It is the unfused combination, and the factors commute.
        assert_eq!(
            result,
            x.mod_power_of_2_sub(y.mod_power_of_2_mul(z, pow), pow)
        );
        assert_eq!(x.mod_power_of_2_sub_mul(z, y, pow), result);
        // Negating a factor gives the opposite operation, which undoes this one.
        assert_eq!(
            x.mod_power_of_2_add_mul(y.mod_power_of_2_neg(pow), z, pow),
            result
        );
        assert_eq!(result.mod_power_of_2_add_mul(y, z, pow), x);
        // Negating everything negates the result.
        assert_eq!(
            x.mod_power_of_2_neg(pow)
                .mod_power_of_2_sub_mul(y.mod_power_of_2_neg(pow), z, pow),
            result.mod_power_of_2_neg(pow)
        );
    });

    unsigned_triple_gen_var_11::<T>().test_properties(|(x, y, pow)| {
        // A zero factor changes nothing, and a factor of 1 leaves the difference.
        assert_eq!(x.mod_power_of_2_sub_mul(y, T::ZERO, pow), x);
        assert_eq!(x.mod_power_of_2_sub_mul(T::ZERO, y, pow), x);
        if pow != 0 {
            assert_eq!(
                x.mod_power_of_2_sub_mul(y, T::ONE, pow),
                x.mod_power_of_2_sub(y, pow)
            );
            assert_eq!(
                x.mod_power_of_2_sub_mul(T::ONE, y, pow),
                x.mod_power_of_2_sub(y, pow)
            );
        }
    });

    unsigned_pair_gen_var_17::<T>().test_properties(|(x, pow)| {
        // Starting from zero gives the product, negated.
        assert_eq!(
            T::ZERO.mod_power_of_2_sub_mul(x, x, pow),
            x.mod_power_of_2_square(pow).mod_power_of_2_neg(pow)
        );
    });
}

#[test]
fn mod_power_of_2_sub_mul_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_sub_mul_properties_helper);
}
