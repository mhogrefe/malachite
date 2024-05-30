// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::mod_pow::simple_binary_mod_pow;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_12, unsigned_pair_gen_var_16, unsigned_quadruple_gen_var_6,
    unsigned_quadruple_gen_var_7, unsigned_triple_gen_var_14, unsigned_triple_gen_var_15,
};
use malachite_base::test_util::num::arithmetic::mod_pow::naive_mod_pow;
use std::panic::catch_unwind;

fn mod_pow_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, exp: u64, m, out| {
        assert_eq!(naive_mod_pow(x, exp, m), out);
        assert_eq!(simple_binary_mod_pow(x, exp, m), out);

        assert_eq!(x.mod_pow(exp, m), out);

        let mut mut_x = x;
        mut_x.mod_pow_assign(exp, m);
        assert_eq!(mut_x, out);

        let data = T::precompute_mod_pow_data(&m);
        assert_eq!(x.mod_pow_precomputed(exp, m, &data), out);

        let mut mut_x = x;
        mut_x.mod_pow_precomputed_assign(exp, m, &data);
        assert_eq!(mut_x, out);
    };
    test(T::ZERO, 0, T::ONE, T::ZERO);
    test(T::ZERO, 0, T::exact_from(10), T::ONE);
    test(T::ZERO, 1, T::exact_from(10), T::ZERO);

    test(T::TWO, 10, T::exact_from(10), T::exact_from(4));
    if T::WIDTH > u8::WIDTH {
        test(T::exact_from(4), 13, T::exact_from(497), T::exact_from(445));
        test(
            T::exact_from(10),
            1000,
            T::exact_from(30),
            T::exact_from(10),
        );
        test(T::TWO, 340, T::exact_from(341), T::ONE);
        test(T::exact_from(5), 216, T::exact_from(217), T::ONE);
    }
    if T::WIDTH > u16::WIDTH {
        test(
            T::TWO,
            1000000,
            T::exact_from(1000000000),
            T::exact_from(747109376),
        );
    }
}

#[test]
fn test_mod_pow() {
    apply_fn_to_unsigneds!(mod_pow_helper);
}

fn mod_pow_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.mod_pow(10, T::ZERO));
    assert_panic!(T::from(123u8).mod_pow(10, T::from(123u8)));
}

#[test]
fn mod_pow_fail() {
    apply_fn_to_unsigneds!(mod_pow_fail_helper);
}

fn mod_pow_assign_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!({
        let mut x = T::ZERO;
        x.mod_pow_assign(10, T::ZERO);
    });
    assert_panic!({
        let mut x = T::from(123u8);
        x.mod_pow_assign(10, T::from(123u8));
    });
}

#[test]
fn mod_pow_assign_fail() {
    apply_fn_to_unsigneds!(mod_pow_assign_fail_helper);
}

fn mod_pow_properties_helper_helper<T: PrimitiveUnsigned, F: Fn(T, u64, T) -> T>(
    x: T,
    exp: u64,
    m: T,
    f: F,
) {
    assert!(x.mod_is_reduced(&m));
    let power = x.mod_pow(exp, m);
    assert!(power.mod_is_reduced(&m));

    let mut x_alt = x;
    x_alt.mod_pow_assign(exp, m);
    assert_eq!(x_alt, power);

    let data = T::precompute_mod_pow_data(&m);

    assert_eq!(x.mod_pow_precomputed(exp, m, &data), power);

    let mut x_alt = x;
    x_alt.mod_pow_precomputed_assign(exp, m, &data);
    assert_eq!(x_alt, power);

    assert_eq!(f(x, exp, m), power);
    if exp.even() {
        assert_eq!(x.mod_neg(m).mod_pow(exp, m), power);
    } else {
        assert_eq!(x.mod_neg(m).mod_pow(exp, m), power.mod_neg(m));
    }
}

fn mod_pow_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_triple_gen_var_15::<T, u64>().test_properties(|(x, exp, m)| {
        mod_pow_properties_helper_helper(x, exp, m, simple_binary_mod_pow);
    });

    unsigned_triple_gen_var_14::<T, u64>()
        .test_properties(|(x, exp, m)| mod_pow_properties_helper_helper(x, exp, m, naive_mod_pow));

    unsigned_pair_gen_var_12::<u64, T>().test_properties(|(exp, m)| {
        assert_eq!(T::ZERO.mod_pow(exp, m), T::from(exp == 0 && m != T::ONE));
        if m != T::ONE {
            assert_eq!(T::ONE.mod_pow(exp, m), T::ONE);
        }
    });

    unsigned_pair_gen_var_16::<T>().test_properties(|(x, m)| {
        assert_eq!(x.mod_pow(0, m), T::from(m != T::ONE));
        assert_eq!(x.mod_pow(1, m), x);
        assert_eq!(x.mod_pow(2, m), x.mod_mul(x, m));
    });

    unsigned_quadruple_gen_var_6::<T, u64>().test_properties(|(x, y, exp, m)| {
        assert_eq!(
            x.mod_mul(y, m).mod_pow(exp, m),
            x.mod_pow(exp, m).mod_mul(y.mod_pow(exp, m), m)
        );
    });

    unsigned_quadruple_gen_var_7::<T, u64>().test_properties(|(x, e, f, m)| {
        if let Some(sum) = e.checked_add(f) {
            assert_eq!(
                x.mod_pow(sum, m),
                x.mod_pow(e, m).mod_mul(x.mod_pow(f, m), m)
            );
        }
        if let Some(product) = e.checked_mul(f) {
            assert_eq!(x.mod_pow(product, m), x.mod_pow(e, m).mod_pow(f, m));
        }
    });
}

#[test]
fn mod_pow_properties() {
    apply_fn_to_unsigneds!(mod_pow_properties_helper);
}
