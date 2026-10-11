// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_16, unsigned_vector_unsigned_pair_gen_var_5,
};
use std::panic::catch_unwind;

fn mod_product_helper<T: PrimitiveUnsigned>() {
    let test = |xs: &[u8], m: u8, out: u8| {
        let xs: Vec<T> = xs.iter().map(|&x| T::from(x)).collect();
        let m = T::from(m);
        let out = T::from(out);
        assert_eq!(T::mod_product(xs.iter().copied(), m), out);
        assert_eq!(T::mod_product(xs.iter(), m), out);
    };
    test(&[], 1, 0);
    test(&[], 7, 1);
    test(&[0], 1, 0);
    test(&[6], 7, 6);
    test(&[2, 3, 5, 7], 11, 1);
    test(&[200, 100, 50], 255, 145);
    test(&[254, 254, 254], 255, 254);
    test(&[3, 5, 0], 7, 0);
    let m = T::MAX;
    let x = m - T::ONE;
    assert_eq!(T::mod_product([x, x, x].into_iter(), m), x);
    // Once the partial product is zero, the remaining factors are not examined, so these unreduced
    // factors do not panic.
    let ten = T::exact_from(10u8);
    assert_eq!(T::mod_product([T::ZERO, ten].into_iter(), ten), T::ZERO);
    assert_eq!(
        T::mod_product([T::TWO, T::exact_from(5u8), ten].into_iter(), ten),
        T::ZERO
    );
}

#[test]
fn test_mod_product() {
    apply_fn_to_unsigneds!(mod_product_helper);
}

fn mod_product_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::mod_product(core::iter::empty::<T>(), T::ZERO));
    assert_panic!(T::mod_product([T::ONE].into_iter(), T::ONE));
    assert_panic!(T::mod_product(
        [T::ONE, T::exact_from(7u8)].into_iter(),
        T::exact_from(7u8)
    ));
}

#[test]
fn mod_product_fail() {
    apply_fn_to_unsigneds!(mod_product_fail_helper);
}

fn mod_product_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_pair_gen_var_5::<T>().test_properties(|(v, m)| {
        let xs = v.elements;
        let product = T::mod_product(xs.iter().copied(), m);
        assert!(product.mod_is_reduced(&m));
        assert_eq!(T::mod_product(xs.iter(), m), product);
        // It is the modular product taken one element at a time, in any order.
        let one = if m == T::ONE { T::ZERO } else { T::ONE };
        assert_eq!(xs.iter().fold(one, |p, &x| p.mod_mul(x, m)), product);
        assert_eq!(T::mod_product(xs.iter().rev(), m), product);
    });

    unsigned_pair_gen_var_16::<T>().test_properties(|(x, m)| {
        assert_eq!(T::mod_product([x].into_iter(), m), x);
        assert_eq!(T::mod_product([x, x].into_iter(), m), x.mod_square(m));
        assert_eq!(T::mod_product([x, T::ZERO].into_iter(), m), T::ZERO);
        if m != T::ONE {
            assert_eq!(T::mod_product([x, T::ONE].into_iter(), m), x);
        }
        if x != T::ZERO
            && let Some(inverse) = x.mod_inverse(m)
        {
            assert_eq!(
                T::mod_product([x, inverse].into_iter(), m),
                if m == T::ONE { T::ZERO } else { T::ONE }
            );
        }
    });
}

#[test]
fn mod_product_properties() {
    apply_fn_to_unsigneds!(mod_product_properties_helper);
}
