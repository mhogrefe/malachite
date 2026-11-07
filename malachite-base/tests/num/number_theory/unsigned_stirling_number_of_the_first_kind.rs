// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::number_theory::traits::CheckedUnsignedStirlingNumberOfTheFirstKind;
use malachite_base::test_util::generators::{
    unsigned_gen_var_5, unsigned_pair_gen_var_28, unsigned_pair_gen_var_55,
};
use std::panic::catch_unwind;

fn unsigned_stirling_number_of_the_first_kind_helper<T: PrimitiveUnsigned>() {
    let test = |n: u64, k: u64, out: Option<u8>| {
        let out = out.map(T::from);
        assert_eq!(
            T::checked_unsigned_stirling_number_of_the_first_kind(n, k),
            out
        );
        if let Some(out) = out {
            assert_eq!(T::unsigned_stirling_number_of_the_first_kind(n, k), out);
        }
    };
    // (expected values computed independently with python)
    // - k > n
    test(0, 1, Some(0));
    test(3, 5, Some(0));
    test(u64::MAX - 1, u64::MAX, Some(0));
    // - k == 0 < n
    test(5, 0, Some(0));
    test(u64::MAX, 0, Some(0));
    // - k == n
    test(0, 0, Some(1));
    test(5, 5, Some(1));
    test(u64::MAX, u64::MAX, Some(1));
    // - k == 1: [n, 1] = (n - 1)!
    test(5, 1, Some(24));
    test(6, 1, Some(120));
    test(u64::MAX, 1, None);
    // - k == n - 1, with n odd and even
    test(5, 4, Some(10));
    test(6, 5, Some(15));
    // - k == n - 2
    test(6, 4, Some(85));
    test(7, 5, Some(175));
    // - k <= n - k: the band is indexed by column
    test(6, 3, Some(225));
    // In the rows below, every type narrower than `u64` returns `None` because n does not fit in
    // it, except where noted.
    // - (1000, 500): n > `u8::MAX` for `u8`, and min(k - 1, n - k) >= `T::WIDTH` for wider types,
    //   so that [n, k] >= 2^`T::WIDTH` without any steps of the recurrence
    // - (u64::MAX, u64::MAX >> 1): min(k - 1, n - k) >= `T::WIDTH`
    test(1000, 500, None);
    test(u64::MAX, u64::MAX >> 1, None);
    // - (10000, 3): n > `u8::MAX` for `u8`, and the column band overflows for wider types
    // - (u64::MAX, 3) and (u64::MAX, 7): the column band overflows after a few rows, although n is
    //   huge
    test(10000, 3, None);
    test(u64::MAX, 3, None);
    test(u64::MAX, 7, None);
    // - k > n - k: the band is indexed by diagonal, and overflows after a few rows; a column band
    //   would have about n entries
    test(u64::MAX, u64::MAX - 3, None);
    test(1000000000000000000, 999999999999999997, None);
    // - k == n - 2, with C(n, 3) overflowing
    test(u64::MAX, u64::MAX - 2, None);
}

#[test]
fn test_unsigned_stirling_number_of_the_first_kind() {
    apply_fn_to_unsigneds!(unsigned_stirling_number_of_the_first_kind_helper);
    // For each type: the largest n for which [n, k] fits, followed by the smallest n for which it
    // does not, with k = 1 (a factorial), with k fixed at 3 and 5, with n - k fixed at 3 and 5,
    // with k = n - 2 and k = n - 1 (closed forms), and with k = n / 2 and k = (n + 1) / 2 (the
    // middle of the row) u8
    assert_eq!(
        u8::checked_unsigned_stirling_number_of_the_first_kind(6, 1),
        Some(120)
    );
    assert_eq!(
        u8::checked_unsigned_stirling_number_of_the_first_kind(7, 1),
        None
    );
    assert_eq!(
        u8::checked_unsigned_stirling_number_of_the_first_kind(6, 3),
        Some(225)
    );
    assert_eq!(
        u8::checked_unsigned_stirling_number_of_the_first_kind(7, 3),
        None
    );
    assert_eq!(
        u8::checked_unsigned_stirling_number_of_the_first_kind(7, 4),
        None
    );
    assert_eq!(
        u8::checked_unsigned_stirling_number_of_the_first_kind(7, 5),
        Some(175)
    );
    assert_eq!(
        u8::checked_unsigned_stirling_number_of_the_first_kind(8, 6),
        None
    );
    assert_eq!(
        u8::checked_unsigned_stirling_number_of_the_first_kind(8, 4),
        None
    );
    assert_eq!(
        u8::checked_unsigned_stirling_number_of_the_first_kind(5, 3),
        Some(35)
    );
    assert_eq!(
        u8::checked_unsigned_stirling_number_of_the_first_kind(23, 22),
        Some(253)
    );
    assert_eq!(
        u8::checked_unsigned_stirling_number_of_the_first_kind(24, 23),
        None
    );
    // - k == n - 2, with 3C(n, 4) overflowing although C(n, 4) fits
    assert_eq!(
        u8::checked_unsigned_stirling_number_of_the_first_kind(9, 7),
        None
    );
    // - k == n - 2, with C(n, 4) overflowing although C(n, 3) fits
    assert_eq!(
        u8::checked_unsigned_stirling_number_of_the_first_kind(11, 9),
        None
    );
    // u16
    assert_eq!(
        u16::checked_unsigned_stirling_number_of_the_first_kind(9, 1),
        Some(40320)
    );
    assert_eq!(
        u16::checked_unsigned_stirling_number_of_the_first_kind(10, 1),
        None
    );
    assert_eq!(
        u16::checked_unsigned_stirling_number_of_the_first_kind(8, 3),
        Some(13132)
    );
    assert_eq!(
        u16::checked_unsigned_stirling_number_of_the_first_kind(9, 3),
        None
    );
    assert_eq!(
        u16::checked_unsigned_stirling_number_of_the_first_kind(9, 5),
        Some(22449)
    );
    assert_eq!(
        u16::checked_unsigned_stirling_number_of_the_first_kind(10, 5),
        None
    );
    assert_eq!(
        u16::checked_unsigned_stirling_number_of_the_first_kind(13, 10),
        Some(55770)
    );
    assert_eq!(
        u16::checked_unsigned_stirling_number_of_the_first_kind(14, 11),
        None
    );
    assert_eq!(
        u16::checked_unsigned_stirling_number_of_the_first_kind(27, 25),
        Some(58500)
    );
    assert_eq!(
        u16::checked_unsigned_stirling_number_of_the_first_kind(28, 26),
        None
    );
    assert_eq!(
        u16::checked_unsigned_stirling_number_of_the_first_kind(8, 4),
        Some(6769)
    );
    assert_eq!(
        u16::checked_unsigned_stirling_number_of_the_first_kind(11, 6),
        None
    );
    assert_eq!(
        u16::checked_unsigned_stirling_number_of_the_first_kind(362, 361),
        Some(65341)
    );
    assert_eq!(
        u16::checked_unsigned_stirling_number_of_the_first_kind(363, 362),
        None
    );
    // u32
    assert_eq!(
        u32::checked_unsigned_stirling_number_of_the_first_kind(13, 1),
        Some(479001600)
    );
    assert_eq!(
        u32::checked_unsigned_stirling_number_of_the_first_kind(14, 1),
        None
    );
    assert_eq!(
        u32::checked_unsigned_stirling_number_of_the_first_kind(13, 3),
        Some(1931559552)
    );
    assert_eq!(
        u32::checked_unsigned_stirling_number_of_the_first_kind(14, 3),
        None
    );
    assert_eq!(
        u32::checked_unsigned_stirling_number_of_the_first_kind(13, 5),
        Some(657206836)
    );
    assert_eq!(
        u32::checked_unsigned_stirling_number_of_the_first_kind(14, 5),
        None
    );
    assert_eq!(
        u32::checked_unsigned_stirling_number_of_the_first_kind(78, 75),
        Some(4283554275)
    );
    assert_eq!(
        u32::checked_unsigned_stirling_number_of_the_first_kind(79, 76),
        None
    );
    assert_eq!(
        u32::checked_unsigned_stirling_number_of_the_first_kind(22, 17),
        Some(2792167686)
    );
    assert_eq!(
        u32::checked_unsigned_stirling_number_of_the_first_kind(23, 18),
        None
    );
    assert_eq!(
        u32::checked_unsigned_stirling_number_of_the_first_kind(431, 429),
        Some(4280103685)
    );
    assert_eq!(
        u32::checked_unsigned_stirling_number_of_the_first_kind(432, 430),
        None
    );
    assert_eq!(
        u32::checked_unsigned_stirling_number_of_the_first_kind(14, 7),
        Some(790943153)
    );
    assert_eq!(
        u32::checked_unsigned_stirling_number_of_the_first_kind(16, 8),
        None
    );
    assert_eq!(
        u32::checked_unsigned_stirling_number_of_the_first_kind(15, 8),
        Some(2681453775)
    );
    assert_eq!(
        u32::checked_unsigned_stirling_number_of_the_first_kind(17, 9),
        None
    );
    assert_eq!(
        u32::checked_unsigned_stirling_number_of_the_first_kind(92682, 92681),
        Some(4294930221)
    );
    assert_eq!(
        u32::checked_unsigned_stirling_number_of_the_first_kind(92683, 92682),
        None
    );
    // u64
    assert_eq!(
        u64::checked_unsigned_stirling_number_of_the_first_kind(21, 1),
        Some(2432902008176640000)
    );
    assert_eq!(
        u64::checked_unsigned_stirling_number_of_the_first_kind(22, 1),
        None
    );
    assert_eq!(
        u64::checked_unsigned_stirling_number_of_the_first_kind(21, 3),
        Some(13803759753640704000)
    );
    assert_eq!(
        u64::checked_unsigned_stirling_number_of_the_first_kind(22, 3),
        None
    );
    assert_eq!(
        u64::checked_unsigned_stirling_number_of_the_first_kind(21, 5),
        Some(8037811822645051776)
    );
    assert_eq!(
        u64::checked_unsigned_stirling_number_of_the_first_kind(22, 5),
        None
    );
    assert_eq!(
        u64::checked_unsigned_stirling_number_of_the_first_kind(3099, 3096),
        Some(18412251536450402226)
    );
    assert_eq!(
        u64::checked_unsigned_stirling_number_of_the_first_kind(3100, 3097),
        None
    );
    assert_eq!(
        u64::checked_unsigned_stirling_number_of_the_first_kind(194, 189),
        Some(17876289853418090472)
    );
    assert_eq!(
        u64::checked_unsigned_stirling_number_of_the_first_kind(195, 190),
        None
    );
    assert_eq!(
        u64::checked_unsigned_stirling_number_of_the_first_kind(110218, 110216),
        Some(18446202967828369862)
    );
    assert_eq!(
        u64::checked_unsigned_stirling_number_of_the_first_kind(110219, 110217),
        None
    );
    assert_eq!(
        u64::checked_unsigned_stirling_number_of_the_first_kind(24, 12),
        Some(4070384057007569521)
    );
    assert_eq!(
        u64::checked_unsigned_stirling_number_of_the_first_kind(26, 13),
        None
    );
    assert_eq!(
        u64::checked_unsigned_stirling_number_of_the_first_kind(25, 13),
        Some(13990945200239106865)
    );
    assert_eq!(
        u64::checked_unsigned_stirling_number_of_the_first_kind(27, 14),
        None
    );
    assert_eq!(
        u64::checked_unsigned_stirling_number_of_the_first_kind(6074001000, 6074000999),
        Some(18446744070963499500)
    );
    assert_eq!(
        u64::checked_unsigned_stirling_number_of_the_first_kind(6074001001, 6074001000),
        None
    );
    // u128
    assert_eq!(
        u128::checked_unsigned_stirling_number_of_the_first_kind(35, 1),
        Some(295232799039604140847618609643520000000)
    );
    assert_eq!(
        u128::checked_unsigned_stirling_number_of_the_first_kind(36, 1),
        None
    );
    assert_eq!(
        u128::checked_unsigned_stirling_number_of_the_first_kind(34, 3),
        Some(65572877917416254708035086636810240000)
    );
    assert_eq!(
        u128::checked_unsigned_stirling_number_of_the_first_kind(35, 3),
        None
    );
    assert_eq!(
        u128::checked_unsigned_stirling_number_of_the_first_kind(34, 5),
        Some(57211756647500554229633570676449280000)
    );
    assert_eq!(
        u128::checked_unsigned_stirling_number_of_the_first_kind(35, 5),
        None
    );
    assert_eq!(
        u128::checked_unsigned_stirling_number_of_the_first_kind(5037095, 5037092),
        Some(340282025830673271535850100376391057175)
    );
    assert_eq!(
        u128::checked_unsigned_stirling_number_of_the_first_kind(5037096, 5037093),
        None
    );
    assert_eq!(
        u128::checked_unsigned_stirling_number_of_the_first_kind(16280, 16275),
        Some(340191657602127293476957376965372841500)
    );
    assert_eq!(
        u128::checked_unsigned_stirling_number_of_the_first_kind(16281, 16276),
        None
    );
    assert_eq!(
        u128::checked_unsigned_stirling_number_of_the_first_kind(7223245206, 7223245204),
        Some(340282366824833069119424014810516533985)
    );
    assert_eq!(
        u128::checked_unsigned_stirling_number_of_the_first_kind(7223245207, 7223245205),
        None
    );
    assert_eq!(
        u128::checked_unsigned_stirling_number_of_the_first_kind(42, 21),
        Some(212990426703080522783477516781640221309)
    );
    assert_eq!(
        u128::checked_unsigned_stirling_number_of_the_first_kind(44, 22),
        None
    );
    assert_eq!(
        u128::checked_unsigned_stirling_number_of_the_first_kind(41, 21),
        Some(3755749687955610546382544532568019569)
    );
    assert_eq!(
        u128::checked_unsigned_stirling_number_of_the_first_kind(43, 22),
        None
    );
    // u128 holds [n, n - 1] = C(n, 2) for every u64 n
    assert_eq!(
        u128::checked_unsigned_stirling_number_of_the_first_kind(u64::MAX, u64::MAX - 1),
        Some(170141183460469231704017187605319778305)
    );
}

fn unsigned_stirling_number_of_the_first_kind_width_helper<T: PrimitiveUnsigned>() {
    // min(k - 1, n - k) == T::WIDTH: the bound alone rules [n, k] out
    assert_eq!(
        T::checked_unsigned_stirling_number_of_the_first_kind(2 * T::WIDTH + 1, T::WIDTH + 1),
        None
    );
    // min(k - 1, n - k) == T::WIDTH - 1: the bound does not apply, and the recurrence overflows
    assert_eq!(
        T::checked_unsigned_stirling_number_of_the_first_kind(2 * T::WIDTH, T::WIDTH),
        None
    );
}

#[test]
fn test_unsigned_stirling_number_of_the_first_kind_width() {
    apply_fn_to_unsigneds!(unsigned_stirling_number_of_the_first_kind_width_helper);
}

fn unsigned_stirling_number_of_the_first_kind_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::unsigned_stirling_number_of_the_first_kind(u64::MAX, 1));
    assert_panic!(T::unsigned_stirling_number_of_the_first_kind(u64::MAX, 3));
}

#[test]
fn unsigned_stirling_number_of_the_first_kind_fail() {
    apply_fn_to_unsigneds!(unsigned_stirling_number_of_the_first_kind_fail_helper);
}

fn unsigned_stirling_number_of_the_first_kind_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_55::<T>().test_properties(|(n, k)| {
        let s = T::unsigned_stirling_number_of_the_first_kind(n, k);
        assert_eq!(
            T::checked_unsigned_stirling_number_of_the_first_kind(n, k),
            Some(s)
        );
        assert_eq!(s == T::ZERO, k > n || k == 0 && n != 0);
        if n != 0 && k != 0 {
            // [n, k] = (n - 1)[n - 1, k] + [n - 1, k - 1], and neither term exceeds [n, k]
            let a = T::unsigned_stirling_number_of_the_first_kind(n - 1, k);
            let b = T::unsigned_stirling_number_of_the_first_kind(n - 1, k - 1);
            if a == T::ZERO {
                assert_eq!(s, b);
            } else {
                assert_eq!(a * T::exact_from(n - 1) + b, s);
            }
        }
        if k != 0 && k <= n {
            // [n, k] >= C(n - 1, k - 1)
            if let (Ok(top), Ok(bottom)) = (T::try_from(n - 1), T::try_from(k - 1))
                && let Some(c) = T::checked_binomial_coefficient(top, bottom)
            {
                assert!(s >= c);
            }
            // [n, k] <= [n + 1, k]
            if let Some(next) = T::checked_unsigned_stirling_number_of_the_first_kind(n + 1, k) {
                assert!(next >= s);
            }
        }
    });

    unsigned_gen_var_5::<u8>().test_properties(|n| {
        let n = u64::from(n);
        // the row of [n, k] sums to n!, so all of it fits whenever n! does
        if let Some(f) = T::checked_factorial(n) {
            let mut sum = T::ZERO;
            for k in 0..=n {
                sum += T::unsigned_stirling_number_of_the_first_kind(n, k);
            }
            assert_eq!(sum, f);
        }
    });
}

#[test]
fn unsigned_stirling_number_of_the_first_kind_properties() {
    apply_fn_to_unsigneds!(unsigned_stirling_number_of_the_first_kind_properties_helper);

    unsigned_pair_gen_var_28::<u8, u8>().test_properties(|(n, k)| {
        let (n, k) = (u64::from(n), u64::from(k));
        // whenever a value fits in a narrower type, every wider type agrees with it
        let values = [
            u8::checked_unsigned_stirling_number_of_the_first_kind(n, k).map(u128::from),
            u16::checked_unsigned_stirling_number_of_the_first_kind(n, k).map(u128::from),
            u32::checked_unsigned_stirling_number_of_the_first_kind(n, k).map(u128::from),
            u64::checked_unsigned_stirling_number_of_the_first_kind(n, k).map(u128::from),
            u128::checked_unsigned_stirling_number_of_the_first_kind(n, k),
        ];
        for w in values.windows(2) {
            if w[0].is_some() {
                assert_eq!(w[1], w[0]);
            }
        }
        // once [n, k] overflows a type, so does [n + 1, k], as long as k != 0
        if k != 0 && values[4].is_none() {
            assert_eq!(
                u128::checked_unsigned_stirling_number_of_the_first_kind(n + 1, k),
                None
            );
        }
    });
}
