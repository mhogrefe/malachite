// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Parity, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::num::number_theory::traits::{
    CheckedStirlingNumberOfTheFirstKind, CheckedUnsignedStirlingNumberOfTheFirstKind,
};
use malachite_base::test_util::generators::{unsigned_pair_gen_var_28, unsigned_pair_gen_var_56};
use std::panic::catch_unwind;

fn stirling_number_of_the_first_kind_helper<T: PrimitiveSigned>() {
    let test = |n: u64, k: u64, out: Option<i8>| {
        let out = out.map(T::from);
        assert_eq!(T::checked_stirling_number_of_the_first_kind(n, k), out);
        if let Some(out) = out {
            assert_eq!(T::stirling_number_of_the_first_kind(n, k), out);
        }
    };
    // (expected values computed independently with python)
    // - n + k even: s(n, k) = [n, k]
    test(0, 0, Some(1));
    test(5, 5, Some(1));
    test(5, 1, Some(24));
    test(5, 3, Some(35));
    test(6, 4, Some(85));
    // - n + k odd: s(n, k) = -[n, k]
    test(5, 2, Some(-50));
    test(5, 4, Some(-10));
    test(6, 1, Some(-120));
    test(6, 5, Some(-15));
    // - zero values, which have no sign
    test(3, 5, Some(0));
    test(5, 0, Some(0));
    test(u64::MAX, 0, Some(0));
    // - the unsigned value does not fit in the unsigned type of the same width
    test(u64::MAX, 1, None);
    test(u64::MAX, 3, None);
}

#[test]
fn test_stirling_number_of_the_first_kind() {
    apply_fn_to_signeds!(stirling_number_of_the_first_kind_helper);
    // For each type: the largest n for which s(n, k) fits, followed by the smallest n for which it
    // does not, with k = 1, k = 3, n - k = 2, n - k = 3, and k = n / 2. The negative values may
    // reach -2^(W - 1), one further than the positive ones. i8
    assert_eq!(
        i8::checked_stirling_number_of_the_first_kind(6, 1),
        Some(-120)
    );
    assert_eq!(i8::checked_stirling_number_of_the_first_kind(7, 1), None);
    assert_eq!(
        i8::checked_stirling_number_of_the_first_kind(5, 3),
        Some(35)
    );
    assert_eq!(i8::checked_stirling_number_of_the_first_kind(6, 3), None);
    assert_eq!(
        i8::checked_stirling_number_of_the_first_kind(6, 4),
        Some(85)
    );
    assert_eq!(i8::checked_stirling_number_of_the_first_kind(7, 5), None);
    assert_eq!(
        i8::checked_stirling_number_of_the_first_kind(5, 2),
        Some(-50)
    );
    // i16
    assert_eq!(
        i16::checked_stirling_number_of_the_first_kind(8, 1),
        Some(-5040)
    );
    assert_eq!(i16::checked_stirling_number_of_the_first_kind(9, 1), None);
    assert_eq!(
        i16::checked_stirling_number_of_the_first_kind(8, 3),
        Some(-13132)
    );
    assert_eq!(i16::checked_stirling_number_of_the_first_kind(9, 3), None);
    assert_eq!(
        i16::checked_stirling_number_of_the_first_kind(23, 21),
        Some(30107)
    );
    assert_eq!(i16::checked_stirling_number_of_the_first_kind(24, 22), None);
    assert_eq!(
        i16::checked_stirling_number_of_the_first_kind(12, 9),
        Some(-32670)
    );
    assert_eq!(i16::checked_stirling_number_of_the_first_kind(13, 10), None);
    assert_eq!(
        i16::checked_stirling_number_of_the_first_kind(8, 4),
        Some(6769)
    );
    assert_eq!(i16::checked_stirling_number_of_the_first_kind(9, 4), None);
    // i32
    assert_eq!(
        i32::checked_stirling_number_of_the_first_kind(13, 1),
        Some(479001600)
    );
    assert_eq!(i32::checked_stirling_number_of_the_first_kind(14, 1), None);
    assert_eq!(
        i32::checked_stirling_number_of_the_first_kind(13, 3),
        Some(1931559552)
    );
    assert_eq!(i32::checked_stirling_number_of_the_first_kind(14, 3), None);
    assert_eq!(
        i32::checked_stirling_number_of_the_first_kind(362, 360),
        Some(2126849550)
    );
    assert_eq!(
        i32::checked_stirling_number_of_the_first_kind(363, 361),
        None
    );
    assert_eq!(
        i32::checked_stirling_number_of_the_first_kind(69, 66),
        Some(-2028119346)
    );
    assert_eq!(i32::checked_stirling_number_of_the_first_kind(70, 67), None);
    assert_eq!(
        i32::checked_stirling_number_of_the_first_kind(14, 7),
        Some(-790943153)
    );
    assert_eq!(i32::checked_stirling_number_of_the_first_kind(15, 7), None);
    // i64
    assert_eq!(
        i64::checked_stirling_number_of_the_first_kind(21, 1),
        Some(2432902008176640000)
    );
    assert_eq!(i64::checked_stirling_number_of_the_first_kind(22, 1), None);
    assert_eq!(
        i64::checked_stirling_number_of_the_first_kind(20, 3),
        Some(-668609730341153280)
    );
    assert_eq!(i64::checked_stirling_number_of_the_first_kind(21, 3), None);
    assert_eq!(
        i64::checked_stirling_number_of_the_first_kind(24, 12),
        Some(4070384057007569521)
    );
    assert_eq!(i64::checked_stirling_number_of_the_first_kind(25, 12), None);
    // i128
    assert_eq!(
        i128::checked_stirling_number_of_the_first_kind(34, 1),
        Some(-8683317618811886495518194401280000000)
    );
    assert_eq!(i128::checked_stirling_number_of_the_first_kind(35, 1), None);
    assert_eq!(
        i128::checked_stirling_number_of_the_first_kind(34, 3),
        Some(-65572877917416254708035086636810240000)
    );
    assert_eq!(i128::checked_stirling_number_of_the_first_kind(35, 3), None);
    assert_eq!(
        i128::checked_stirling_number_of_the_first_kind(41, 20),
        Some(-59004689496900490381793190946351418980)
    );
    assert_eq!(
        i128::checked_stirling_number_of_the_first_kind(42, 21),
        None
    );
}

fn stirling_number_of_the_first_kind_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(T::stirling_number_of_the_first_kind(u64::MAX, 1));
}

#[test]
fn stirling_number_of_the_first_kind_fail() {
    apply_fn_to_signeds!(stirling_number_of_the_first_kind_fail_helper);
}

fn stirling_number_of_the_first_kind_properties_helper<
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>() {
    unsigned_pair_gen_var_56::<S>().test_properties(|(n, k)| {
        let s = S::stirling_number_of_the_first_kind(n, k);
        assert_eq!(S::checked_stirling_number_of_the_first_kind(n, k), Some(s));
        // the magnitude is the unsigned Stirling number of the first kind, and the sign is (-1)^(n
        // + k)
        assert_eq!(
            U::checked_unsigned_stirling_number_of_the_first_kind(n, k),
            Some(s.unsigned_abs())
        );
        if s != S::ZERO {
            assert_eq!(s > S::ZERO, n.even() == k.even());
        }
        if n != 0 && k != 0 {
            // s(n, k) = s(n - 1, k - 1) - (n - 1)s(n - 1, k), when the terms fit
            if let (Some(a), Some(b)) = (
                S::checked_stirling_number_of_the_first_kind(n - 1, k - 1),
                S::checked_stirling_number_of_the_first_kind(n - 1, k),
            ) && let Ok(m) = S::try_from(n - 1)
                && let Some(product) = b.checked_mul(m)
                && let Some(difference) = a.checked_sub(product)
            {
                assert_eq!(difference, s);
            }
        }
    });
}

#[test]
fn stirling_number_of_the_first_kind_properties() {
    apply_fn_to_unsigned_signed_pairs!(stirling_number_of_the_first_kind_properties_helper);

    unsigned_pair_gen_var_28::<u8, u8>().test_properties(|(n, k)| {
        let (n, k) = (u64::from(n), u64::from(k));
        // whenever a value fits in a narrower type, every wider type agrees with it
        let values = [
            i8::checked_stirling_number_of_the_first_kind(n, k).map(i128::from),
            i16::checked_stirling_number_of_the_first_kind(n, k).map(i128::from),
            i32::checked_stirling_number_of_the_first_kind(n, k).map(i128::from),
            i64::checked_stirling_number_of_the_first_kind(n, k).map(i128::from),
            i128::checked_stirling_number_of_the_first_kind(n, k),
        ];
        for w in values.windows(2) {
            if w[0].is_some() {
                assert_eq!(w[1], w[0]);
            }
        }
        // s(n, k) fits in a signed type whenever [n, k] fits in its positive range
        if let Some(u) = u64::checked_unsigned_stirling_number_of_the_first_kind(n, k)
            && i64::convertible_from(u)
        {
            assert_eq!(
                i64::checked_stirling_number_of_the_first_kind(n, k).map(i64::unsigned_abs),
                Some(u)
            );
        }
    });
}
