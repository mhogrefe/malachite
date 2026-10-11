// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModAdd, ModIsReduced, ModSum};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use std::panic::catch_unwind;

#[test]
fn test_mod_sum() {
    let test = |ss: &[&str], m: u8, out| {
        let ps: Vec<UnsignedPolynomial<u8>> = ss
            .iter()
            .map(|s| UnsignedPolynomial::from_str(s).unwrap())
            .collect();
        let sum = UnsignedPolynomial::mod_sum(ps.iter(), m);
        assert_eq!(sum.to_string(), out);
        assert_eq!(UnsignedPolynomial::mod_sum(ps.into_iter(), m), sum);
    };
    test(&[], 7, "0");
    test(&["0"], 1, "0");
    test(
        &["x^2+2*x+3", "7*x+5", "x^3+7*x^2+6*x"],
        9,
        "x^3+8*x^2+6*x+8",
    );
    test(&["x+254", "254*x+1"], 255, "0");
    test(&["x+6", "1"], 7, "x");
}

fn mod_sum_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(UnsignedPolynomial::<T>::mod_sum(
        core::iter::empty::<UnsignedPolynomial<T>>(),
        T::ZERO
    ));
    let p = UnsignedPolynomial::<T>::from_str("7*x+1").unwrap();
    assert_panic!(UnsignedPolynomial::mod_sum(
        [&p].into_iter(),
        T::exact_from(7u8)
    ));
}

#[test]
fn mod_sum_fail() {
    apply_fn_to_unsigneds!(mod_sum_fail_helper);
}

fn mod_sum_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>().test_properties(
        |(p, q, m)| {
            let sum = UnsignedPolynomial::mod_sum([&p, &q].into_iter(), m);
            assert!(sum.mod_is_reduced(&m));
            assert_eq!(
                UnsignedPolynomial::mod_sum([p.clone(), q.clone()].into_iter(), m),
                sum
            );
            // It is `mod_add`, in either order, and summing more terms keeps adding.
            assert_eq!((&p).mod_add(&q, m), sum);
            assert_eq!(UnsignedPolynomial::mod_sum([&q, &p].into_iter(), m), sum);
            assert_eq!(
                UnsignedPolynomial::mod_sum([&p, &q, &p].into_iter(), m),
                (&sum).mod_add(&p, m)
            );
            // The sum of one polynomial is that polynomial, and the sum of none is zero.
            assert_eq!(UnsignedPolynomial::mod_sum([&p].into_iter(), m), p);
            assert_eq!(
                UnsignedPolynomial::mod_sum(core::iter::empty::<UnsignedPolynomial<T>>(), m),
                UnsignedPolynomial::ZERO
            );
        },
    );
}

#[test]
fn mod_sum_properties() {
    apply_fn_to_unsigneds!(mod_sum_properties_helper);
}
