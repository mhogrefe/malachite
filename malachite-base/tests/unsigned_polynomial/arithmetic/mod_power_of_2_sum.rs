// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Sum,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_sum() {
    let test = |ss: &[&str], pow: u64, out| {
        let ps: Vec<UnsignedPolynomial<u8>> = ss
            .iter()
            .map(|s| UnsignedPolynomial::from_str(s).unwrap())
            .collect();
        let sum = UnsignedPolynomial::mod_power_of_2_sum(ps.iter(), pow);
        assert_eq!(sum.to_string(), out);
        assert_eq!(
            UnsignedPolynomial::mod_power_of_2_sum(ps.into_iter(), pow),
            sum
        );
    };
    test(&[], 3, "0");
    test(&["1"], 1, "1");
    test(&["x^2+2*x+3", "7*x+5", "x^3+7*x^2+6*x"], 3, "x^3+7*x");
    test(&["x+255", "255*x+1"], 8, "0");
    test(&["x+1", "7"], 3, "x");
}

fn mod_power_of_2_sum_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(UnsignedPolynomial::<T>::mod_power_of_2_sum(
        core::iter::empty::<UnsignedPolynomial<T>>(),
        T::WIDTH + 1
    ));
    let p = UnsignedPolynomial::<T>::from_str("8*x+1").unwrap();
    assert_panic!(UnsignedPolynomial::mod_power_of_2_sum([&p].into_iter(), 3));
}

#[test]
fn mod_power_of_2_sum_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_sum_fail_helper);
}

fn mod_power_of_2_sum_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().test_properties(
        |(p, q, pow)| {
            let sum = UnsignedPolynomial::mod_power_of_2_sum([&p, &q].into_iter(), pow);
            assert!(sum.mod_power_of_2_is_reduced(pow));
            assert_eq!(
                UnsignedPolynomial::mod_power_of_2_sum([p.clone(), q.clone()].into_iter(), pow),
                sum
            );
            // It is `mod_power_of_2_add`, in either order, and summing more terms keeps adding.
            assert_eq!((&p).mod_power_of_2_add(&q, pow), sum);
            assert_eq!(
                UnsignedPolynomial::mod_power_of_2_sum([&q, &p].into_iter(), pow),
                sum
            );
            assert_eq!(
                UnsignedPolynomial::mod_power_of_2_sum([&p, &q, &p].into_iter(), pow),
                (&sum).mod_power_of_2_add(&p, pow)
            );
            // The sum of one polynomial is that polynomial, and the sum of none is zero.
            assert_eq!(
                UnsignedPolynomial::mod_power_of_2_sum([&p].into_iter(), pow),
                p
            );
            assert_eq!(
                UnsignedPolynomial::mod_power_of_2_sum(
                    core::iter::empty::<UnsignedPolynomial<T>>(),
                    pow
                ),
                UnsignedPolynomial::ZERO
            );
        },
    );
}

#[test]
fn mod_power_of_2_sum_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_sum_properties_helper);
}
