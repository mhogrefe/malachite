// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::mod_div::mod_div_unsigned;
use malachite_base::num::arithmetic::traits::ModDiv;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::test_util::generators::{
    unsigned_gen_var_6, unsigned_pair_gen_var_16, unsigned_triple_gen_var_12,
};
use malachite_base::test_util::num::arithmetic::mod_div::mod_div_euclidean;
use std::panic::catch_unwind;

fn mod_div_helper<
    U: ModDiv<U, U, Output = U> + PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() {
    let test = |b: U, c: U, m: U, out: Option<U>| {
        assert_eq!(b.mod_div(c, m), out);
        assert_eq!(mod_div_unsigned::<U, S>(b, c, m), out);
        assert_eq!(mod_div_euclidean::<U, S>(b, c, m), out);
    };

    test(U::ZERO, U::ZERO, U::ONE, Some(U::ZERO));
    test(U::ZERO, U::ZERO, U::exact_from(10), Some(U::ZERO));
    test(U::ONE, U::ZERO, U::exact_from(10), None);
    test(U::ZERO, U::exact_from(7), U::exact_from(10), Some(U::ZERO));
    test(
        U::ONE,
        U::exact_from(3),
        U::exact_from(10),
        Some(U::exact_from(7)),
    );
    test(
        U::exact_from(6),
        U::exact_from(4),
        U::exact_from(10),
        Some(U::exact_from(4)),
    );
    test(
        U::exact_from(5),
        U::exact_from(5),
        U::exact_from(10),
        Some(U::ONE),
    );
    test(U::exact_from(2), U::exact_from(5), U::exact_from(10), None);
    test(
        U::ONE,
        U::exact_from(100),
        U::exact_from(101),
        Some(U::exact_from(100)),
    );
    test(U::ONE, U::MAX - U::ONE, U::MAX, Some(U::MAX - U::ONE));
    test(U::MAX - U::ONE, U::ONE, U::MAX, Some(U::MAX - U::ONE));
}

#[test]
fn test_mod_div() {
    apply_fn_to_unsigned_signed_pairs!(mod_div_helper);
}

fn mod_div_fail_helper<T: ModDiv<T, T, Output = T> + PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.mod_div(T::ZERO, T::ZERO));
    assert_panic!(T::from(123u8).mod_div(T::ONE, T::from(123u8)));
    assert_panic!(T::ONE.mod_div(T::from(123u8), T::from(123u8)));
}

#[test]
fn mod_div_fail() {
    apply_fn_to_unsigneds!(mod_div_fail_helper);
}

fn mod_div_properties_helper<
    U: ModDiv<U, U, Output = U> + PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() {
    unsigned_triple_gen_var_12::<U>().test_properties(|(b, c, m)| {
        assert!(b.mod_is_reduced(&m));
        assert!(c.mod_is_reduced(&m));
        let q = b.mod_div(c, m);
        assert_eq!(mod_div_unsigned::<U, S>(b, c, m), q);
        assert_eq!(mod_div_euclidean::<U, S>(b, c, m), q);
        assert_eq!(q.is_some(), b % c.gcd(m) == U::ZERO);
        if let Some(q) = q {
            assert!(q.mod_is_reduced(&m));
            assert_eq!(q.mod_mul(c, m), b);
            if c != U::ZERO
                && let Some(inverse) = c.mod_inverse(m)
            {
                assert_eq!(q, b.mod_mul(inverse, m));
            }
        }
        // A quotient of b * c and c always exists, though it may not be b.
        let product = b.mod_mul(c, m);
        assert!(product.mod_div(c, m).is_some());
    });

    unsigned_pair_gen_var_16::<U>().test_properties(|(x, m)| {
        assert!(x.mod_div(x, m).is_some());
        if m > U::ONE {
            assert_eq!(x.mod_div(U::ONE, m), Some(x));
            assert_eq!(U::ZERO.mod_div(x, m), Some(U::ZERO));
        }
    });

    unsigned_gen_var_6::<U>().test_properties(|m| {
        assert_eq!(U::ONE.mod_div(U::ONE, m), Some(U::ONE));
        assert_eq!((m - U::ONE).mod_div(m - U::ONE, m), Some(U::ONE));
    });
}

#[test]
fn mod_div_properties() {
    apply_fn_to_unsigned_signed_pairs!(mod_div_properties_helper);
}
