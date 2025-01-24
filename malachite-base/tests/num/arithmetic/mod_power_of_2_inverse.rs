// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::mod_power_of_2_inverse::mod_power_of_2_inverse_fast;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::test_util::generators::{unsigned_gen_var_3, unsigned_pair_gen_var_39};
use malachite_base::test_util::num::arithmetic::mod_power_of_2_inverse::*;
use std::panic::catch_unwind;

fn mod_power_of_2_inverse_helper<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() {
    let test = |n: U, pow, out| {
        assert_eq!(n.mod_power_of_2_inverse(pow), out);
        assert_eq!(mod_power_of_2_inverse_euclidean::<U, S>(n, pow), out);
        assert_eq!(mod_power_of_2_inverse_fast(n, pow), out);
    };

    test(U::ONE, 5, Some(U::ONE));
    test(U::exact_from(7), 5, Some(U::exact_from(23)));
    test(U::exact_from(31), 5, Some(U::exact_from(31)));
    test(U::ONE, U::WIDTH, Some(U::ONE));
    test(U::MAX, U::WIDTH, Some(U::MAX));
}

#[test]
fn test_mod_power_of_2_inverse() {
    apply_fn_to_unsigned_signed_pairs!(mod_power_of_2_inverse_helper);
}

fn mod_power_of_2_inverse_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.mod_power_of_2_inverse(5));
    assert_panic!(T::from(30u8).mod_power_of_2_inverse(3));
    assert_panic!(T::from(3u8).mod_power_of_2_inverse(200));
    assert_panic!(T::ONE.mod_power_of_2_inverse(0));
    assert_panic!(T::from(200u8).mod_power_of_2_inverse(7));
}

#[test]
fn mod_power_of_2_inverse_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_inverse_fail_helper);
}

fn mod_power_of_2_inverse_properties_helper<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() {
    unsigned_pair_gen_var_39::<U>().test_properties(|(n, pow)| {
        assert!(n.mod_power_of_2_is_reduced(pow));
        let inverse = n.mod_power_of_2_inverse(pow);
        assert_eq!(mod_power_of_2_inverse_euclidean::<U, S>(n, pow), inverse);
        assert_eq!(mod_power_of_2_inverse_fast(n, pow), inverse);
        assert_eq!(inverse.is_some(), n.odd());
        if let Some(inverse) = inverse {
            assert!(inverse.mod_power_of_2_is_reduced(pow));
            assert_eq!(inverse.mod_power_of_2_inverse(pow), Some(n));
            assert_eq!(n.mod_power_of_2_mul(inverse, pow), U::ONE);
            assert_eq!(
                n.mod_power_of_2_neg(pow).mod_power_of_2_inverse(pow),
                Some(inverse.mod_power_of_2_neg(pow))
            );
        }
    });

    unsigned_gen_var_3::<U>().test_properties(|pow| {
        assert_eq!(U::ONE.mod_power_of_2_inverse(pow), Some(U::ONE));
        assert_eq!(
            U::low_mask(pow).mod_power_of_2_inverse(pow),
            Some(U::low_mask(pow))
        );
    });
}

#[test]
fn mod_power_of_2_inverse_properties() {
    apply_fn_to_unsigned_signed_pairs!(mod_power_of_2_inverse_properties_helper);
}
