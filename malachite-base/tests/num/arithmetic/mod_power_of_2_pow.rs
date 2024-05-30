// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_17, unsigned_pair_gen_var_23, unsigned_quadruple_gen_var_8,
    unsigned_quadruple_gen_var_9, unsigned_triple_gen_var_16,
};
use std::panic::catch_unwind;

fn mod_power_of_2_pow_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, exp: u64, pow: u64, out| {
        assert_eq!(x.mod_power_of_2_pow(exp, pow), out);

        let mut mut_x = x;
        mut_x.mod_power_of_2_pow_assign(exp, pow);
        assert_eq!(mut_x, out);
    };
    test(T::ZERO, 0, 0, T::ZERO);
    test(T::ZERO, 0, 3, T::ONE);
    test(T::ZERO, 1, 3, T::ZERO);

    test(T::TWO, 2, 3, T::exact_from(4));
    test(T::exact_from(5), 13, 3, T::exact_from(5));
    test(T::exact_from(7), 1000, 6, T::ONE);
    test(T::exact_from(101), 1000000, 8, T::ONE);
}

#[test]
fn test_mod_power_of_2_pow() {
    apply_fn_to_unsigneds!(mod_power_of_2_pow_helper);
}

fn mod_power_of_2_pow_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ONE.mod_power_of_2_pow(10, 0));
    assert_panic!(T::from(200u8).mod_power_of_2_pow(10, 7));
}

#[test]
fn mod_power_of_2_pow_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_pow_fail_helper);
}

fn mod_power_of_2_pow_assign_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!({
        let mut x = T::ONE;
        x.mod_power_of_2_pow_assign(10, 0);
    });
    assert_panic!({
        let mut x = T::from(200u8);
        x.mod_power_of_2_pow_assign(10, 7);
    });
}

#[test]
fn mod_power_of_2_pow_assign_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_pow_assign_fail_helper);
}

fn mod_power_of_2_pow_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_triple_gen_var_16::<T, u64>().test_properties(|(x, exp, pow)| {
        assert!(x.mod_power_of_2_is_reduced(pow));
        let power = x.mod_power_of_2_pow(exp, pow);
        assert!(power.mod_power_of_2_is_reduced(pow));

        let mut x_alt = x;
        x_alt.mod_power_of_2_pow_assign(exp, pow);
        assert_eq!(x_alt, power);
        if exp.even() {
            assert_eq!(
                x.mod_power_of_2_neg(pow).mod_power_of_2_pow(exp, pow),
                power
            );
        } else {
            assert_eq!(
                x.mod_power_of_2_neg(pow).mod_power_of_2_pow(exp, pow),
                power.mod_power_of_2_neg(pow)
            );
        }
    });

    unsigned_pair_gen_var_23::<u64, T>().test_properties(|(exp, pow)| {
        assert_eq!(
            T::ZERO.mod_power_of_2_pow(exp, pow),
            T::from(exp == 0 && pow != 0)
        );
        if pow != 0 {
            assert_eq!(T::ONE.mod_power_of_2_pow(exp, pow), T::ONE);
        }
    });

    unsigned_pair_gen_var_17::<T>().test_properties(|(x, pow)| {
        assert_eq!(x.mod_power_of_2_pow(0, pow), T::from(pow != 0));
        assert_eq!(x.mod_power_of_2_pow(1, pow), x);
        assert_eq!(x.mod_power_of_2_pow(2, pow), x.mod_power_of_2_mul(x, pow));
    });

    unsigned_quadruple_gen_var_8::<T, u64>().test_properties(|(x, y, exp, pow)| {
        assert_eq!(
            x.mod_power_of_2_mul(y, pow).mod_power_of_2_pow(exp, pow),
            x.mod_power_of_2_pow(exp, pow)
                .mod_power_of_2_mul(y.mod_power_of_2_pow(exp, pow), pow)
        );
    });

    unsigned_quadruple_gen_var_9::<T, u64>().test_properties(|(x, e, f, pow)| {
        if let Some(sum) = e.checked_add(f) {
            assert_eq!(
                x.mod_power_of_2_pow(sum, pow),
                x.mod_power_of_2_pow(e, pow)
                    .mod_power_of_2_mul(x.mod_power_of_2_pow(f, pow), pow)
            );
        }
        if let Some(product) = e.checked_mul(f) {
            assert_eq!(
                x.mod_power_of_2_pow(product, pow),
                x.mod_power_of_2_pow(e, pow).mod_power_of_2_pow(f, pow)
            );
        }
    });
}

#[test]
fn mod_power_of_2_pow_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_pow_properties_helper);
}
