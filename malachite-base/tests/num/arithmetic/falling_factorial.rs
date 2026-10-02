// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CheckedBinomialCoefficient, CheckedFactorial, CheckedFallingFactorial, CheckedRisingFactorial,
    FallingFactorial,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_pair_gen_var_2;
use std::panic::catch_unwind;

fn falling_factorial_helper<
    T: CheckedFallingFactorial + FallingFactorial<Output = T> + PrimitiveUnsigned,
>() {
    let test = |x: T, n: u64, out: Option<T>| {
        assert_eq!(x.checked_falling_factorial(n), out);
        if let Some(out) = out {
            assert_eq!(x.falling_factorial(n), out);
        }
    };
    // - n == 0
    test(T::ZERO, 0, Some(T::ONE));
    test(T::MAX, 0, Some(T::ONE));
    // - n > x: a zero factor
    test(T::ZERO, 1, Some(T::ZERO));
    test(T::exact_from(3), 4, Some(T::ZERO));
    // - n > x, with partial products that would overflow before the zero factor. The base is
    //   clamped below n at every width, including u128, whose maximum exceeds u64::MAX.
    test(T::saturating_from(u64::MAX - 1), u64::MAX, Some(T::ZERO));
    // - single factor
    test(T::ONE, 1, Some(T::ONE));
    test(T::MAX, 1, Some(T::MAX));
    // - general products
    test(T::exact_from(3), 3, Some(T::exact_from(6)));
    test(T::exact_from(6), 3, Some(T::exact_from(120)));
    // - a nonzero product that overflows
    test(T::MAX, 2, None);
}

#[test]
fn test_falling_factorial() {
    apply_fn_to_unsigneds!(falling_factorial_helper);
    // type-specific magnitudes
    assert_eq!(6u16.checked_falling_factorial(4), Some(360));
    assert_eq!(6u8.checked_falling_factorial(4), None);
    assert_eq!(
        20u64.checked_falling_factorial(20),
        Some(2432902008176640000)
    );
    // - a nonzero product that overflows
    assert_eq!(21u64.checked_falling_factorial(21), None);
    // - x = 255 and n = 300: the zero factor is detected despite n not fitting in a u8
    assert_eq!(u8::MAX.checked_falling_factorial(300), Some(0));
}

fn falling_factorial_fail_helper<T: FallingFactorial<Output = T> + PrimitiveUnsigned>() {
    assert_panic!(T::MAX.falling_factorial(2));
}

#[test]
fn falling_factorial_fail() {
    apply_fn_to_unsigneds!(falling_factorial_fail_helper);
}

fn falling_factorial_properties_helper<
    T: CheckedBinomialCoefficient
        + CheckedFactorial
        + CheckedFallingFactorial
        + CheckedRisingFactorial
        + FallingFactorial<Output = T>
        + PrimitiveUnsigned,
>() {
    unsigned_pair_gen_var_2::<T, u64>().test_properties(|(x, n)| {
        let ff = x.checked_falling_factorial(n);
        if let Some(ff) = ff {
            assert_eq!(x.falling_factorial(n), ff);
        }
        if n != 0 && x < T::saturating_from(n) {
            assert_eq!(ff, Some(T::ZERO));
        } else if n != 0 {
            let low = x - T::exact_from(n - 1);
            // the falling factorial of x is the rising factorial of x - n + 1
            assert_eq!(low.checked_rising_factorial(n), ff);
            // the identity x^(n) = binomial(x, n) * n!
            if let (Some(ff), Some(b), Some(f)) = (
                ff,
                T::checked_binomial_coefficient(x, T::exact_from(n)),
                T::checked_factorial(n),
            ) {
                assert_eq!(ff, b.checked_mul(f).unwrap());
            }
            // the recurrence x^(n + 1) = x^(n) * (x - n)
            if let (Some(ff), Some(next)) = (ff, x.checked_falling_factorial(n + 1)) {
                assert_eq!(next, ff * (low - T::ONE));
            }
        }
        assert_eq!(x.checked_falling_factorial(0), Some(T::ONE));
        assert_eq!(x.checked_falling_factorial(1), Some(x));
    });
}

#[test]
fn falling_factorial_properties() {
    apply_fn_to_unsigneds!(falling_factorial_properties_helper);
}
