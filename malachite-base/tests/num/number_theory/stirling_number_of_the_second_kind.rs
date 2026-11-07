// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::number_theory::traits::CheckedStirlingNumberOfTheSecondKind;
use malachite_base::test_util::generators::{
    unsigned_gen_var_5, unsigned_pair_gen_var_28, unsigned_pair_gen_var_54,
};
use std::panic::catch_unwind;

fn stirling_number_of_the_second_kind_helper<T: PrimitiveUnsigned>() {
    let test = |n: u64, k: u64, out: Option<u8>| {
        let out = out.map(T::from);
        assert_eq!(T::checked_stirling_number_of_the_second_kind(n, k), out);
        if let Some(out) = out {
            assert_eq!(T::stirling_number_of_the_second_kind(n, k), out);
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
    // - k == n || k == 1
    test(0, 0, Some(1));
    test(5, 5, Some(1));
    test(u64::MAX, u64::MAX, Some(1));
    test(5, 1, Some(1));
    test(u64::MAX, 1, Some(1));
    // - k == 2, fitting
    test(5, 2, Some(15));
    test(8, 2, Some(127));
    // - k == n - 1, with n odd and even
    test(5, 4, Some(10));
    test(6, 5, Some(15));
    // - k == n - 2
    test(6, 4, Some(65));
    test(7, 5, Some(140));
    // - k <= n - k: the band is indexed by column
    test(6, 3, Some(90));
    // In the rows below, every type narrower than `u64` returns `None` because n does not fit in
    // it, except where noted.
    // - (1000, 500): n > `u8::MAX` for `u8`, and min(k - 1, n - k) >= `T::WIDTH` for wider types,
    //   so that S(n, k) >= 2^`T::WIDTH` without any steps of the recurrence
    // - (u64::MAX, u64::MAX >> 1): min(k - 1, n - k) >= `T::WIDTH`
    test(1000, 500, None);
    test(u64::MAX, u64::MAX >> 1, None);
    // - (10000, 3): n > `u8::MAX` for `u8`, and the column band's multiplication overflows for
    //   wider types
    // - (u64::MAX, 3) and (u64::MAX, 7): the column band's multiplication overflows after a few
    //   rows, although n is huge
    test(10000, 3, None);
    test(u64::MAX, 3, None);
    test(u64::MAX, 7, None);
    // - k > n - k: the band is indexed by diagonal, and its addition overflows after a few rows; a
    //   column band would have about n entries
    test(u64::MAX, u64::MAX - 3, None);
    test(1000000000000000000, 999999999999999997, None);
    // - k == n - 2, with C(n, 3) overflowing
    test(u64::MAX, u64::MAX - 2, None);
}

#[test]
fn test_stirling_number_of_the_second_kind() {
    apply_fn_to_unsigneds!(stirling_number_of_the_second_kind_helper);
    // For each type: the largest n for which S(n, k) fits, followed by the smallest n for which it
    // does not, with k fixed at 3 and 5 (band indexed by column), with n - k fixed at 3 and 5 (band
    // indexed by diagonal), with k = n - 2 and k = n - 1 (closed forms), and with k = n / 2 and k =
    // (n + 1) / 2 (the middle of the row, indexed by column and by diagonal). The rows that do not
    // fit overflow in the closed forms' binomial coefficients or in the bands' multiplications and
    // additions; `u128`'s (46, 23) is the only one whose column band overflows in an addition. u8
    assert_eq!(
        u8::checked_stirling_number_of_the_second_kind(6, 3),
        Some(90)
    );
    assert_eq!(u8::checked_stirling_number_of_the_second_kind(7, 3), None);
    assert_eq!(u8::checked_stirling_number_of_the_second_kind(7, 4), None);
    assert_eq!(
        u8::checked_stirling_number_of_the_second_kind(7, 5),
        Some(140)
    );
    assert_eq!(u8::checked_stirling_number_of_the_second_kind(8, 6), None);
    assert_eq!(u8::checked_stirling_number_of_the_second_kind(8, 4), None);
    assert_eq!(
        u8::checked_stirling_number_of_the_second_kind(5, 3),
        Some(25)
    );
    assert_eq!(
        u8::checked_stirling_number_of_the_second_kind(23, 22),
        Some(253)
    );
    assert_eq!(u8::checked_stirling_number_of_the_second_kind(24, 23), None);
    // - k == n - 2, with C(n, 3) = 165 fitting but C(n, 4) = 330 overflowing
    assert_eq!(u8::checked_stirling_number_of_the_second_kind(11, 9), None);
    // u16
    assert_eq!(
        u16::checked_stirling_number_of_the_second_kind(11, 3),
        Some(28501)
    );
    assert_eq!(u16::checked_stirling_number_of_the_second_kind(12, 3), None);
    assert_eq!(
        u16::checked_stirling_number_of_the_second_kind(10, 5),
        Some(42525)
    );
    assert_eq!(u16::checked_stirling_number_of_the_second_kind(11, 5), None);
    assert_eq!(
        u16::checked_stirling_number_of_the_second_kind(13, 10),
        Some(39325)
    );
    assert_eq!(
        u16::checked_stirling_number_of_the_second_kind(14, 11),
        None
    );
    assert_eq!(u16::checked_stirling_number_of_the_second_kind(11, 6), None);
    assert_eq!(
        u16::checked_stirling_number_of_the_second_kind(28, 26),
        Some(64701)
    );
    assert_eq!(
        u16::checked_stirling_number_of_the_second_kind(29, 27),
        None
    );
    assert_eq!(u16::checked_stirling_number_of_the_second_kind(12, 6), None);
    assert_eq!(
        u16::checked_stirling_number_of_the_second_kind(9, 5),
        Some(6951)
    );
    assert_eq!(
        u16::checked_stirling_number_of_the_second_kind(362, 361),
        Some(65341)
    );
    assert_eq!(
        u16::checked_stirling_number_of_the_second_kind(363, 362),
        None
    );
    // u32
    assert_eq!(
        u32::checked_stirling_number_of_the_second_kind(21, 3),
        Some(1742343625)
    );
    assert_eq!(u32::checked_stirling_number_of_the_second_kind(22, 3), None);
    assert_eq!(
        u32::checked_stirling_number_of_the_second_kind(16, 5),
        Some(1096190550)
    );
    assert_eq!(u32::checked_stirling_number_of_the_second_kind(17, 5), None);
    assert_eq!(
        u32::checked_stirling_number_of_the_second_kind(78, 75),
        Some(4065311250)
    );
    assert_eq!(
        u32::checked_stirling_number_of_the_second_kind(79, 76),
        None
    );
    assert_eq!(
        u32::checked_stirling_number_of_the_second_kind(24, 19),
        Some(3880739170)
    );
    assert_eq!(
        u32::checked_stirling_number_of_the_second_kind(25, 20),
        None
    );
    assert_eq!(
        u32::checked_stirling_number_of_the_second_kind(431, 429),
        Some(4266852590)
    );
    assert_eq!(
        u32::checked_stirling_number_of_the_second_kind(432, 430),
        None
    );
    assert_eq!(
        u32::checked_stirling_number_of_the_second_kind(16, 8),
        Some(2141764053)
    );
    assert_eq!(u32::checked_stirling_number_of_the_second_kind(18, 9), None);
    assert_eq!(
        u32::checked_stirling_number_of_the_second_kind(15, 8),
        Some(216627840)
    );
    assert_eq!(u32::checked_stirling_number_of_the_second_kind(17, 9), None);
    assert_eq!(
        u32::checked_stirling_number_of_the_second_kind(92682, 92681),
        Some(4294930221)
    );
    assert_eq!(
        u32::checked_stirling_number_of_the_second_kind(92683, 92682),
        None
    );
    // u64
    assert_eq!(
        u64::checked_stirling_number_of_the_second_kind(42, 3),
        Some(18236495989562137650)
    );
    assert_eq!(u64::checked_stirling_number_of_the_second_kind(43, 3), None);
    assert_eq!(
        u64::checked_stirling_number_of_the_second_kind(30, 5),
        Some(7713000216608565075)
    );
    assert_eq!(u64::checked_stirling_number_of_the_second_kind(31, 5), None);
    assert_eq!(
        u64::checked_stirling_number_of_the_second_kind(3100, 3097),
        Some(18424141917706155175)
    );
    assert_eq!(
        u64::checked_stirling_number_of_the_second_kind(3101, 3098),
        None
    );
    assert_eq!(
        u64::checked_stirling_number_of_the_second_kind(195, 190),
        Some(17568627203284370200)
    );
    assert_eq!(
        u64::checked_stirling_number_of_the_second_kind(196, 191),
        None
    );
    assert_eq!(
        u64::checked_stirling_number_of_the_second_kind(110219, 110217),
        Some(18446649271452144547)
    );
    assert_eq!(
        u64::checked_stirling_number_of_the_second_kind(110220, 110218),
        None
    );
    assert_eq!(
        u64::checked_stirling_number_of_the_second_kind(26, 13),
        Some(1850568574253550060)
    );
    assert_eq!(
        u64::checked_stirling_number_of_the_second_kind(28, 14),
        None
    );
    assert_eq!(
        u64::checked_stirling_number_of_the_second_kind(27, 14),
        Some(8541149231801585700)
    );
    assert_eq!(
        u64::checked_stirling_number_of_the_second_kind(29, 15),
        None
    );
    assert_eq!(
        u64::checked_stirling_number_of_the_second_kind(6074001000, 6074000999),
        Some(18446744070963499500)
    );
    assert_eq!(
        u64::checked_stirling_number_of_the_second_kind(6074001001, 6074001000),
        None
    );
    // u128
    assert_eq!(
        u128::checked_stirling_number_of_the_second_kind(82, 3),
        Some(221713244121516467122485586051225534050)
    );
    assert_eq!(
        u128::checked_stirling_number_of_the_second_kind(83, 3),
        None
    );
    assert_eq!(
        u128::checked_stirling_number_of_the_second_kind(58, 5),
        Some(289117117798621336574981561105995800645)
    );
    assert_eq!(
        u128::checked_stirling_number_of_the_second_kind(59, 5),
        None
    );
    assert_eq!(
        u128::checked_stirling_number_of_the_second_kind(5037096, 5037093),
        Some(340282160941152998910387213975806689410)
    );
    assert_eq!(
        u128::checked_stirling_number_of_the_second_kind(5037097, 5037094),
        None
    );
    assert_eq!(
        u128::checked_stirling_number_of_the_second_kind(16281, 16276),
        Some(340122001864972086298627328820509621406)
    );
    assert_eq!(
        u128::checked_stirling_number_of_the_second_kind(16282, 16277),
        None
    );
    assert_eq!(
        u128::checked_stirling_number_of_the_second_kind(7223245206, 7223245204),
        Some(340282366762020606090040287351554243165)
    );
    assert_eq!(
        u128::checked_stirling_number_of_the_second_kind(7223245207, 7223245205),
        None
    );
    assert_eq!(
        u128::checked_stirling_number_of_the_second_kind(44, 22),
        Some(2601501750631019038096563064832102352)
    );
    assert_eq!(
        u128::checked_stirling_number_of_the_second_kind(46, 23),
        None
    );
    assert_eq!(
        u128::checked_stirling_number_of_the_second_kind(45, 23),
        Some(12316028862363042241608552018954965700)
    );
    assert_eq!(
        u128::checked_stirling_number_of_the_second_kind(47, 24),
        None
    );
    assert_eq!(
        u128::checked_stirling_number_of_the_second_kind(u64::MAX, u64::MAX - 1),
        Some(170141183460469231704017187605319778305)
    );
}

fn stirling_number_of_the_second_kind_width_helper<T: PrimitiveUnsigned>() {
    // S(n, 2) = 2^(n - 1) - 1 fits up to n = T::WIDTH + 1
    assert_eq!(
        T::checked_stirling_number_of_the_second_kind(T::WIDTH + 1, 2),
        Some(T::MAX)
    );
    assert_eq!(
        T::checked_stirling_number_of_the_second_kind(T::WIDTH + 2, 2),
        None
    );
    // min(k - 1, n - k) == T::WIDTH: the bound alone rules S(n, k) out
    assert_eq!(
        T::checked_stirling_number_of_the_second_kind(2 * T::WIDTH + 1, T::WIDTH + 1),
        None
    );
    // min(k - 1, n - k) == T::WIDTH - 1: the bound does not apply, and the recurrence overflows
    assert_eq!(
        T::checked_stirling_number_of_the_second_kind(2 * T::WIDTH, T::WIDTH),
        None
    );
}

#[test]
fn test_stirling_number_of_the_second_kind_width() {
    apply_fn_to_unsigneds!(stirling_number_of_the_second_kind_width_helper);
}

fn stirling_number_of_the_second_kind_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::stirling_number_of_the_second_kind(T::WIDTH + 2, 2));
    assert_panic!(T::stirling_number_of_the_second_kind(u64::MAX, 3));
}

#[test]
fn stirling_number_of_the_second_kind_fail() {
    apply_fn_to_unsigneds!(stirling_number_of_the_second_kind_fail_helper);
}

fn stirling_number_of_the_second_kind_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_54::<T>().test_properties(|(n, k)| {
        let s = T::stirling_number_of_the_second_kind(n, k);
        assert_eq!(T::checked_stirling_number_of_the_second_kind(n, k), Some(s));
        assert_eq!(s == T::ZERO, k > n || k == 0 && n != 0);
        if n != 0 && k != 0 {
            // S(n, k) = kS(n - 1, k) + S(n - 1, k - 1), and neither term exceeds S(n, k)
            let a = T::stirling_number_of_the_second_kind(n - 1, k);
            let b = T::stirling_number_of_the_second_kind(n - 1, k - 1);
            if a == T::ZERO {
                assert_eq!(s, b);
            } else {
                assert_eq!(a * T::exact_from(k) + b, s);
            }
        }
        if k != 0 && k <= n {
            // S(n, k) >= C(n - 1, k - 1)
            if let (Ok(top), Ok(bottom)) = (T::try_from(n - 1), T::try_from(k - 1))
                && let Some(c) = T::checked_binomial_coefficient(top, bottom)
            {
                assert!(s >= c);
            }
            // S(n, k) <= S(n + 1, k)
            if let Some(next) = T::checked_stirling_number_of_the_second_kind(n + 1, k) {
                assert!(next >= s);
            }
        }
    });

    unsigned_gen_var_5::<u8>().test_properties(|n| {
        let n = u64::from(n);
        // the row of S(n, k) sums to the Bell number B(n), so all of it fits whenever B(n) does
        if let Some(bell) = T::checked_bell_number(n) {
            let mut sum = T::ZERO;
            for k in 0..=n {
                sum += T::stirling_number_of_the_second_kind(n, k);
            }
            assert_eq!(sum, bell);
        }
    });
}

#[test]
fn stirling_number_of_the_second_kind_properties() {
    apply_fn_to_unsigneds!(stirling_number_of_the_second_kind_properties_helper);

    unsigned_pair_gen_var_28::<u8, u8>().test_properties(|(n, k)| {
        let (n, k) = (u64::from(n), u64::from(k));
        // whenever a value fits in a narrower type, every wider type agrees with it
        let values = [
            u8::checked_stirling_number_of_the_second_kind(n, k).map(u128::from),
            u16::checked_stirling_number_of_the_second_kind(n, k).map(u128::from),
            u32::checked_stirling_number_of_the_second_kind(n, k).map(u128::from),
            u64::checked_stirling_number_of_the_second_kind(n, k).map(u128::from),
            u128::checked_stirling_number_of_the_second_kind(n, k),
        ];
        for w in values.windows(2) {
            if w[0].is_some() {
                assert_eq!(w[1], w[0]);
            }
        }
        // once S(n, k) overflows a type, so does S(n + 1, k), as long as k != 0
        if k != 0 && values[4].is_none() {
            assert_eq!(
                u128::checked_stirling_number_of_the_second_kind(n + 1, k),
                None
            );
        }
    });
}
