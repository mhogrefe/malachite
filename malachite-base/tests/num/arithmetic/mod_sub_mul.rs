// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_16, unsigned_quadruple_gen_var_4, unsigned_triple_gen_var_12,
};
use std::panic::catch_unwind;

fn mod_sub_mul_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, y: T, z: T, m, out| {
        assert_eq!(x.mod_sub_mul(y, z, m), out);

        let mut x = x;
        x.mod_sub_mul_assign(y, z, m);
        assert_eq!(x, out);
    };
    test(T::ZERO, T::ZERO, T::ZERO, T::ONE, T::ZERO);
    test(
        T::exact_from(6),
        T::TWO,
        T::exact_from(5),
        T::exact_from(7),
        T::exact_from(3),
    );
    test(
        T::exact_from(10),
        T::exact_from(14),
        T::exact_from(3),
        T::exact_from(15),
        T::exact_from(13),
    );
    test(
        T::exact_from(200),
        T::exact_from(100),
        T::exact_from(3),
        T::exact_from(255),
        T::exact_from(155),
    );
    test(
        T::MAX - T::ONE,
        T::MAX - T::ONE,
        T::MAX - T::ONE,
        T::MAX,
        T::MAX - T::TWO,
    );
    test(T::ZERO, T::ONE, T::ONE, T::MAX, T::MAX - T::ONE);
}

#[test]
fn test_mod_sub_mul() {
    apply_fn_to_unsigneds!(mod_sub_mul_helper);
}

fn mod_sub_mul_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.mod_sub_mul(T::ZERO, T::ZERO, T::ZERO));
    assert_panic!(T::from(7u8).mod_sub_mul(T::ONE, T::ONE, T::from(7u8)));
    assert_panic!(T::ONE.mod_sub_mul(T::from(7u8), T::ONE, T::from(7u8)));
    assert_panic!(T::ONE.mod_sub_mul(T::ONE, T::from(7u8), T::from(7u8)));
}

#[test]
fn mod_sub_mul_fail() {
    apply_fn_to_unsigneds!(mod_sub_mul_fail_helper);
}

fn mod_sub_mul_assign_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!({
        let mut x = T::ZERO;
        x.mod_sub_mul_assign(T::ZERO, T::ZERO, T::ZERO);
    });
    assert_panic!({
        let mut x = T::from(7u8);
        x.mod_sub_mul_assign(T::ONE, T::ONE, T::from(7u8));
    });
    assert_panic!({
        let mut x = T::ONE;
        x.mod_sub_mul_assign(T::from(7u8), T::ONE, T::from(7u8));
    });
    assert_panic!({
        let mut x = T::ONE;
        x.mod_sub_mul_assign(T::ONE, T::from(7u8), T::from(7u8));
    });
}

#[test]
fn mod_sub_mul_assign_fail() {
    apply_fn_to_unsigneds!(mod_sub_mul_assign_fail_helper);
}

fn mod_sub_mul_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_quadruple_gen_var_4::<T>().test_properties(|(x, y, z, m)| {
        let result = x.mod_sub_mul(y, z, m);
        assert!(result.mod_is_reduced(&m));

        let mut x_alt = x;
        x_alt.mod_sub_mul_assign(y, z, m);
        assert_eq!(x_alt, result);

        // It is the unfused combination, and the factors commute.
        assert_eq!(result, x.mod_sub(y.mod_mul(z, m), m));
        assert_eq!(x.mod_sub_mul(z, y, m), result);
        // Negating a factor gives the opposite operation, which undoes this one.
        assert_eq!(x.mod_add_mul(y.mod_neg(m), z, m), result);
        assert_eq!(result.mod_add_mul(y, z, m), x);
        // Negating everything negates the result.
        assert_eq!(
            x.mod_neg(m).mod_sub_mul(y.mod_neg(m), z, m),
            result.mod_neg(m)
        );
    });

    unsigned_triple_gen_var_12::<T>().test_properties(|(x, y, m)| {
        // A zero factor changes nothing, and a factor of 1 leaves the difference.
        assert_eq!(x.mod_sub_mul(y, T::ZERO, m), x);
        assert_eq!(x.mod_sub_mul(T::ZERO, y, m), x);
        if m != T::ONE {
            assert_eq!(x.mod_sub_mul(y, T::ONE, m), x.mod_sub(y, m));
            assert_eq!(x.mod_sub_mul(T::ONE, y, m), x.mod_sub(y, m));
        }
    });

    unsigned_pair_gen_var_16::<T>().test_properties(|(x, m)| {
        // Starting from zero gives the square, negated.
        assert_eq!(T::ZERO.mod_sub_mul(x, x, m), x.mod_square(m).mod_neg(m));
    });
}

#[test]
fn mod_sub_mul_properties() {
    apply_fn_to_unsigneds!(mod_sub_mul_properties_helper);
}
