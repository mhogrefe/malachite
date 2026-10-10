// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Sqrt, Parity};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::traits::LowMask;
use malachite_base::test_util::generators::unsigned_pair_gen_var_17;
use malachite_base::test_util::num::arithmetic::mod_power_of_2_sqrt::mod_power_of_2_sqrt_naive;
use std::panic::catch_unwind;

fn mod_power_of_2_sqrt_helper<T: PrimitiveUnsigned>() {
    let test = |x: u8, pow, out: Option<u8>| {
        let x = T::from(x);
        let out = out.map(T::from);
        assert_eq!(x.mod_power_of_2_sqrt(pow), out);
        assert_eq!(mod_power_of_2_sqrt_naive(x, pow), out);
    };
    // - x == 0
    test(0, 0, Some(0));
    test(0, 5, Some(0));
    // - n <= 2, so u must be 1 and no Newton steps are taken
    test(1, 1, Some(1));
    test(1, 2, Some(1));
    test(4, 3, Some(2));
    // - u is not 1 mod 8
    test(3, 2, None);
    test(3, 3, None);
    test(5, 3, None);
    test(12, 4, None);
    // - the power of 2 dividing x is odd
    test(2, 2, None);
    test(2, 3, None);
    test(8, 4, None);
    // - Newton steps; the four candidates are distinct roots
    test(1, 3, Some(1));
    test(9, 4, Some(3));
    test(17, 5, Some(7));
    test(25, 5, Some(5));
    test(4, 5, Some(2));
    test(16, 5, Some(4));
    test(129, 8, Some(63));
    // - pow == T::WIDTH
    assert_eq!(T::ONE.mod_power_of_2_sqrt(T::WIDTH), Some(T::ONE));
    assert_eq!(T::from(4u8).mod_power_of_2_sqrt(T::WIDTH), Some(T::TWO));
    assert_eq!(T::MAX.mod_power_of_2_sqrt(T::WIDTH), None);
}

#[test]
fn test_mod_power_of_2_sqrt() {
    apply_fn_to_unsigneds!(mod_power_of_2_sqrt_helper);
    assert_eq!(17u64.mod_power_of_2_sqrt(64), Some(405959429219100393));
    assert_eq!(
        (17u64 << 20).mod_power_of_2_sqrt(64),
        Some(2195515552539648)
    );
    assert_eq!(48u64.mod_power_of_2_sqrt(64), None);
    assert_eq!(
        (12345678901234567890u128 * 12345678901234567890u128).mod_power_of_2_sqrt(128),
        Some(12345678901234567890)
    );
}

fn mod_power_of_2_sqrt_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::from(16u8).mod_power_of_2_sqrt(4));
    assert_panic!(T::ONE.mod_power_of_2_sqrt(0));
    assert_panic!(T::ONE.mod_power_of_2_sqrt(T::WIDTH + 1));
}

#[test]
fn mod_power_of_2_sqrt_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_sqrt_fail_helper);
}

fn mod_power_of_2_sqrt_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_17::<T>().test_properties(|(x, pow)| {
        let root = x.mod_power_of_2_sqrt(pow);
        // A root exists exactly when x is 0 or 4 ^ w times a number that is 1 mod 8.
        let v = x.trailing_zeros();
        let is_square = x == T::ZERO || v.even() && (x >> v).mod_power_of_2(3) == T::ONE;
        assert_eq!(root.is_some(), is_square);
        if let Some(r) = root {
            // The root is reduced, squares to x, and is no greater than its negation.
            assert!(r.mod_power_of_2_is_reduced(pow));
            assert_eq!(r.mod_power_of_2_square(pow), x);
            assert!(r <= r.mod_power_of_2_neg(pow));
        }
        // Multiplying x by 4 and the modulus by 4 doubles the least root.
        if pow + 2 <= T::WIDTH {
            assert_eq!(
                (x << 2u64).mod_power_of_2_sqrt(pow + 2),
                root.map(|r| r << 1u64)
            );
        }
    });

    unsigned_pair_gen_var_17::<T>().test_properties(|(y, pow)| {
        // A square has a root, no greater than either of the roots it was formed from.
        let r = y
            .mod_power_of_2_square(pow)
            .mod_power_of_2_sqrt(pow)
            .unwrap();
        assert!(r <= y);
        assert!(r <= y.mod_power_of_2_neg(pow));
    });
}

#[test]
fn mod_power_of_2_sqrt_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_sqrt_properties_helper);

    // Every u8 input agrees with the search.
    for pow in 0..=u8::WIDTH {
        for x in 0..=u8::low_mask(pow) {
            assert_eq!(
                x.mod_power_of_2_sqrt(pow),
                mod_power_of_2_sqrt_naive(x, pow)
            );
        }
    }
}
