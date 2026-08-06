// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::crt::crt_unsigned;
use malachite_base::num::arithmetic::traits::Crt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_16, unsigned_quadruple_gen_var_13,
};
use malachite_base::test_util::num::arithmetic::crt::crt_symmetric;
use std::panic::catch_unwind;

fn crt_helper<
    U: Crt<U, U, U, Output = U> + PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() {
    let test = |r1: U, m1: U, r2: U, m2: U, out: Option<U>| {
        assert_eq!(r1.crt(m1, r2, m2), out);
        assert_eq!(crt_unsigned(r1, m1, r2, m2), out);
        assert_eq!(crt_symmetric::<U, S>(r1, m1, r2, m2), out);
        // The solution is symmetric in the two congruences.
        assert_eq!(r2.crt(m2, r1, m1), out);
    };

    test(U::ZERO, U::ONE, U::ZERO, U::ONE, Some(U::ZERO));
    // - crt_unsigned: c == 0 and m2 == 1: the second congruence is vacuous
    test(
        U::exact_from(5),
        U::exact_from(10),
        U::ZERO,
        U::ONE,
        Some(U::exact_from(5)),
    );
    // - m1 == 1: the first congruence is vacuous
    test(
        U::ZERO,
        U::ONE,
        U::exact_from(5),
        U::exact_from(10),
        Some(U::exact_from(5)),
    );
    test(
        U::TWO,
        U::exact_from(3),
        U::exact_from(3),
        U::exact_from(5),
        Some(U::exact_from(8)),
    );
    test(
        U::exact_from(3),
        U::exact_from(4),
        U::TWO,
        U::exact_from(3),
        Some(U::exact_from(11)),
    );
    // - a solution smaller than both moduli
    test(
        U::TWO,
        U::exact_from(6),
        U::TWO,
        U::exact_from(7),
        Some(U::TWO),
    );
    test(
        U::ZERO,
        U::exact_from(16),
        U::exact_from(14),
        U::exact_from(15),
        Some(U::exact_from(224)),
    );
    // - crt_unsigned: c == 0 and m2 > 1: not coprime, even though the congruences are compatible
    test(U::TWO, U::exact_from(4), U::ZERO, U::TWO, None);
    // - crt_unsigned: c == 0 and m2 > 1: incompatible congruences
    test(U::ZERO, U::exact_from(4), U::ONE, U::TWO, None);
    // - c != 0 and the moduli are not coprime
    test(
        U::ONE,
        U::exact_from(4),
        U::exact_from(3),
        U::exact_from(6),
        None,
    );
    test(
        U::ONE,
        U::exact_from(6),
        U::exact_from(3),
        U::exact_from(4),
        None,
    );
    // - the largest representable modulus
    test(
        U::MAX - U::ONE,
        U::MAX,
        U::ZERO,
        U::ONE,
        Some(U::MAX - U::ONE),
    );

    // A width-relative pair of coprime moduli whose product nearly fills the type: 2^(W/2) - 1 and
    // 2^(W/2).
    let half = U::power_of_2(U::WIDTH >> 1);
    let m1 = half - U::ONE;
    // The solution is 0 mod 2^h and -1 mod 2^h - 1, and since 2^h is 1 mod 2^h - 1, it is (2^h - 2)
    // * 2^h.
    test(m1 - U::ONE, m1, U::ZERO, half, Some((half - U::TWO) * half));
}

#[test]
fn test_crt() {
    apply_fn_to_unsigned_signed_pairs!(crt_helper);
}

fn crt_fail_helper<T: Crt<T, T, T, Output = T> + PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.crt(T::ZERO, T::ZERO, T::ONE));
    assert_panic!(T::ZERO.crt(T::ONE, T::ZERO, T::ZERO));
    assert_panic!(T::from(123u8).crt(T::from(123u8), T::ZERO, T::ONE));
    assert_panic!(T::ZERO.crt(T::ONE, T::from(123u8), T::from(123u8)));
    // The moduli product 2^W overflows at every width.
    let half = T::power_of_2(T::WIDTH >> 1);
    assert_panic!(T::ZERO.crt(half, T::ZERO, half));
}

#[test]
fn crt_fail() {
    apply_fn_to_unsigneds!(crt_fail_helper);
}

fn crt_properties_helper<
    U: Crt<U, U, U, Output = U> + PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() {
    unsigned_quadruple_gen_var_13::<U>().test_properties(|(r1, m1, r2, m2)| {
        assert!(r1.mod_is_reduced(&m1));
        assert!(r2.mod_is_reduced(&m2));
        let m = m1.checked_mul(m2).unwrap();
        let result = r1.crt(m1, r2, m2);
        assert_eq!(crt_unsigned(r1, m1, r2, m2), result);
        assert_eq!(crt_symmetric::<U, S>(r1, m1, r2, m2), result);
        assert_eq!(r2.crt(m2, r1, m1), result);
        assert_eq!(result.is_some(), m1.coprime_with(m2));
        if let Some(x) = result {
            assert!(x < m);
            assert_eq!(x % m1, r1);
            assert_eq!(x % m2, r2);
        }
    });

    unsigned_pair_gen_var_16::<U>().test_properties(|(x, m)| {
        assert_eq!(x.crt(m, U::ZERO, U::ONE), Some(x));
        assert_eq!(U::ZERO.crt(U::ONE, x, m), Some(x));
    });
}

#[test]
fn crt_properties() {
    apply_fn_to_unsigned_signed_pairs!(crt_properties_helper);
}
