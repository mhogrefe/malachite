// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::mod_div_list::mod_div_list_unsigned;
use malachite_base::num::arithmetic::traits::{ModDiv, ModDivList};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::test_util::generators::{
    unsigned_gen_var_6, unsigned_pair_gen_var_16, unsigned_triple_gen_var_12,
};
use malachite_base::test_util::num::arithmetic::mod_div_list::mod_div_list_euclidean;
use std::panic::catch_unwind;

fn mod_div_list_helper<
    U: ModDivList<U, U, Output = U> + PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() {
    let test = |b: U, c: U, m: U, out: Option<(U, U, U)>| {
        assert_eq!(b.mod_div_list(c, m), out);
        assert_eq!(mod_div_list_unsigned::<U, S>(b, c, m), out);
        assert_eq!(mod_div_list_euclidean::<U, S>(b, c, m), out);
    };

    let u = U::exact_from(10);
    test(U::ZERO, U::ZERO, U::ONE, Some((U::ZERO, U::ONE, U::ONE)));
    test(U::ZERO, U::ZERO, u, Some((U::ZERO, U::ONE, u)));
    test(U::ONE, U::ZERO, u, None);
    test(U::ZERO, U::exact_from(7), u, Some((U::ZERO, u, U::ONE)));
    test(
        U::ONE,
        U::exact_from(3),
        u,
        Some((U::exact_from(7), u, U::ONE)),
    );
    test(
        U::exact_from(6),
        U::exact_from(4),
        u,
        Some((U::exact_from(4), U::exact_from(5), U::TWO)),
    );
    test(
        U::exact_from(5),
        U::exact_from(5),
        u,
        Some((U::ONE, U::TWO, U::exact_from(5))),
    );
    test(U::TWO, U::exact_from(5), u, None);
    test(
        U::ONE,
        U::MAX - U::ONE,
        U::MAX,
        Some((U::MAX - U::ONE, U::MAX, U::ONE)),
    );
}

#[test]
fn test_mod_div_list() {
    apply_fn_to_unsigned_signed_pairs!(mod_div_list_helper);
}

fn mod_div_list_fail_helper<T: ModDivList<T, T, Output = T> + PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.mod_div_list(T::ZERO, T::ZERO));
    assert_panic!(T::from(123u8).mod_div_list(T::ONE, T::from(123u8)));
    assert_panic!(T::ONE.mod_div_list(T::from(123u8), T::from(123u8)));
}

#[test]
fn mod_div_list_fail() {
    apply_fn_to_unsigneds!(mod_div_list_fail_helper);
}

fn mod_div_list_properties_helper<U, S>()
where
    U: ModDiv<U, U, Output = U> + ModDivList<U, U, Output = U> + PrimitiveUnsigned,
    U: WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
{
    unsigned_triple_gen_var_12::<U>().test_properties(|(b, c, m)| {
        let result = b.mod_div_list(c, m);
        assert_eq!(mod_div_list_unsigned::<U, S>(b, c, m), result);
        assert_eq!(mod_div_list_euclidean::<U, S>(b, c, m), result);
        let q = b.mod_div(c, m);
        assert_eq!(result.is_some(), q.is_some());
        if let Some((start, stride, length)) = result {
            assert_eq!(length, c.gcd(m));
            assert_eq!(stride, m / length);
            assert!(start < stride);
            // any single quotient is start plus some multiple of stride
            assert_eq!(q.unwrap() % stride, start);
            // Spot-check that the first few elements of the progression are quotients. While
            // i < length, start + stride * i < m, so the arithmetic cannot overflow.
            let mut i = U::ZERO;
            while i < length && i < U::exact_from(4u8) {
                assert_eq!((start + stride * i).mod_mul(c, m), b);
                i += U::ONE;
            }
        }
    });

    unsigned_pair_gen_var_16::<U>().test_properties(|(x, m)| {
        if m > U::ONE {
            assert_eq!(
                U::ZERO.mod_div_list(x, m),
                Some((U::ZERO, m / x.gcd(m), x.gcd(m)))
            );
            assert_eq!(x.mod_div_list(U::ONE, m), Some((x, m, U::ONE)));
        }
    });

    unsigned_gen_var_6::<U>().test_properties(|m| {
        assert_eq!(U::ONE.mod_div_list(U::ONE, m), Some((U::ONE, m, U::ONE)));
        assert_eq!(U::ZERO.mod_div_list(U::ZERO, m), Some((U::ZERO, U::ONE, m)));
    });
}

#[test]
fn mod_div_list_properties() {
    apply_fn_to_unsigned_signed_pairs!(mod_div_list_properties_helper);
}
