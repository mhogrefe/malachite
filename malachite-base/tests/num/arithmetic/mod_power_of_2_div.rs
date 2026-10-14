// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Div, ModPowerOf2Mul};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_triple_gen_var_11;
use std::panic::catch_unwind;

fn mod_power_of_2_div_helper<U: PrimitiveUnsigned>() {
    let test = |x: U, y: U, pow: u64, out: Option<U>| {
        assert_eq!(x.mod_power_of_2_div(y, pow), out);
    };
    test(U::ZERO, U::ZERO, 0, Some(U::ZERO));
    test(U::ZERO, U::ZERO, 4, Some(U::ZERO));
    test(U::ONE, U::ZERO, 4, None);
    test(
        U::exact_from(6),
        U::exact_from(3),
        4,
        Some(U::exact_from(2)),
    );
    test(U::ONE, U::exact_from(3), 4, Some(U::exact_from(11)));
    test(U::exact_from(6), U::exact_from(4), 4, None);
    test(
        U::exact_from(12),
        U::exact_from(4),
        4,
        Some(U::exact_from(3)),
    );
    test(U::ZERO, U::exact_from(4), 4, Some(U::ZERO));
    test(U::exact_from(8), U::exact_from(8), 4, Some(U::ONE));
    test(
        U::exact_from(2),
        U::exact_from(6),
        4,
        Some(U::exact_from(3)),
    );
    test(U::exact_from(5), U::ONE, 3, Some(U::exact_from(5)));
    test(U::MAX, U::MAX, U::WIDTH, Some(U::ONE));
    test(
        U::MAX - U::ONE,
        U::exact_from(2),
        U::WIDTH,
        Some(U::MAX >> 1),
    );
}

#[test]
fn test_mod_power_of_2_div() {
    apply_fn_to_unsigneds!(mod_power_of_2_div_helper);
}

#[test]
fn test_mod_power_of_2_div_u64() {
    assert_eq!(
        u64::MAX.mod_power_of_2_div(3, 64),
        Some(6148914691236517205)
    );
}

fn mod_power_of_2_div_fail_helper<U: PrimitiveUnsigned>() {
    assert_panic!(U::ONE.mod_power_of_2_div(U::ONE, U::WIDTH + 1));
    assert_panic!(U::exact_from(8).mod_power_of_2_div(U::ONE, 3));
    assert_panic!(U::ONE.mod_power_of_2_div(U::exact_from(8), 3));
}

#[test]
fn mod_power_of_2_div_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_div_fail_helper);
}

fn mod_power_of_2_div_properties_helper<U: PrimitiveUnsigned>() {
    unsigned_triple_gen_var_11::<U>().test_properties(|(x, y, pow)| {
        let q = x.mod_power_of_2_div(y, pow);
        // A quotient exists exactly when x is 0, or y is nonzero and has no more trailing zeros
        // than x.
        assert_eq!(
            q.is_some(),
            x == U::ZERO || (y != U::ZERO && x.trailing_zeros() >= y.trailing_zeros())
        );
        if let Some(q) = q {
            // The quotient is reduced, times y it is x, and it is the smallest, less than 2^(pow -
            // s).
            assert!(q.mod_power_of_2_is_reduced(pow));
            assert_eq!(q.mod_power_of_2_mul(y, pow), x);
            if y == U::ZERO {
                assert_eq!(q, U::ZERO);
            } else {
                assert!(q.mod_power_of_2_is_reduced(pow - y.trailing_zeros()));
            }
        }
        // For an odd divisor, this is multiplication by the inverse.
        if y.odd() {
            assert_eq!(
                q,
                Some(x.mod_power_of_2_mul(y.mod_power_of_2_inverse(pow).unwrap(), pow))
            );
        }
        // A quotient exists exactly when one exists for mod_div with modulus 2^pow, and for an odd
        // divisor the two agree.
        if pow < U::WIDTH {
            let r = x.mod_div(y, U::power_of_2(pow));
            assert_eq!(r.is_some(), q.is_some());
            if y.odd() {
                assert_eq!(r, q);
            }
        }
    });
}

#[test]
fn mod_power_of_2_div_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_div_properties_helper);

    // For u8, the result is the smallest quotient found by searching.
    unsigned_triple_gen_var_11::<u8>().test_properties(|(x, y, pow)| {
        let brute = (0..=u8::MAX)
            .take(1 << pow)
            .find(|q| q.mod_power_of_2_mul(y, pow) == x);
        assert_eq!(x.mod_power_of_2_div(y, pow), brute);
    });
}
