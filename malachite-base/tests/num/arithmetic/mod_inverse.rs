// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::mod_inverse::mod_inverse_binary;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::test_util::generators::{unsigned_gen_var_6, unsigned_pair_gen_var_38};
use malachite_base::test_util::num::arithmetic::mod_inverse::mod_inverse_euclidean;
use std::panic::catch_unwind;

fn mod_inverse_helper<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() {
    let test = |n: U, m, out| {
        assert_eq!(n.mod_inverse(m), out);
        assert_eq!(mod_inverse_euclidean::<U, S>(n, m), out);
        assert_eq!(mod_inverse_binary::<U, S>(n, m), out);
    };

    test(U::ONE, U::exact_from(5), Some(U::ONE));
    test(U::exact_from(7), U::exact_from(10), Some(U::exact_from(3)));
    test(U::exact_from(6), U::exact_from(10), None);
    test(
        U::exact_from(100),
        U::exact_from(101),
        Some(U::exact_from(100)),
    );
    test(U::ONE, U::MAX, Some(U::ONE));
    test(U::MAX - U::ONE, U::MAX, Some(U::MAX - U::ONE));
}

#[test]
fn test_mod_inverse() {
    apply_fn_to_unsigned_signed_pairs!(mod_inverse_helper);
}

fn mod_inverse_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.mod_inverse(T::ZERO));
    assert_panic!(T::ZERO.mod_inverse(T::from(10u8)));
    assert_panic!(T::from(123u8).mod_inverse(T::from(123u8)));
}

#[test]
fn mod_inverse_fail() {
    apply_fn_to_unsigneds!(mod_inverse_fail_helper);
}

fn mod_inverse_properties_helper<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() {
    unsigned_pair_gen_var_38::<U>().test_properties(|(n, m)| {
        assert!(n.mod_is_reduced(&m));
        let inverse = n.mod_inverse(m);
        assert_eq!(mod_inverse_euclidean::<U, S>(n, m), inverse);
        assert_eq!(mod_inverse_binary::<U, S>(n, m), inverse);
        assert_eq!(inverse.is_some(), n.coprime_with(m));
        if let Some(inverse) = inverse {
            assert!(inverse.mod_is_reduced(&m));
            assert_eq!(inverse.mod_inverse(m), Some(n));
            assert_eq!(n.mod_mul(inverse, m), U::ONE);
            assert_eq!((m - n).mod_inverse(m), Some(m - inverse));
        }
    });

    unsigned_gen_var_6::<U>().test_properties(|m| {
        assert_eq!(U::ONE.mod_inverse(m), Some(U::ONE));
        assert_eq!((m - U::ONE).mod_inverse(m), Some(m - U::ONE));
    });
}

#[test]
fn mod_inverse_properties() {
    apply_fn_to_unsigned_signed_pairs!(mod_inverse_properties_helper);
}
