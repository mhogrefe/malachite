// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CheckedBinomialCoefficient, CheckedFactorial, CheckedRisingFactorial, Parity, RisingFactorial,
    UnsignedAbs,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::SaturatingFrom;
use malachite_base::test_util::generators::{
    signed_unsigned_pair_gen_var_1, unsigned_pair_gen_var_2,
};
use std::panic::catch_unwind;

fn rising_factorial_helper<
    T: CheckedRisingFactorial + PrimitiveUnsigned + RisingFactorial<Output = T>,
>() {
    let test = |x: T, n: u64, out: Option<T>| {
        assert_eq!(x.checked_rising_factorial(n), out);
        if let Some(out) = out {
            assert_eq!(x.rising_factorial(n), out);
        }
    };
    // - n == 0
    test(T::ZERO, 0, Some(T::ONE));
    test(T::MAX, 0, Some(T::ONE));
    // - a zero product stays zero, whatever n is
    test(T::ZERO, 3, Some(T::ZERO));
    test(T::ZERO, u64::MAX, Some(T::ZERO));
    // - single factor
    test(T::ONE, 1, Some(T::ONE));
    test(T::MAX, 1, Some(T::MAX));
    // - general products
    test(T::ONE, 3, Some(T::exact_from(6)));
    test(T::TWO, 3, Some(T::exact_from(24)));
    // - the factor increment itself can overflow
    test(T::MAX, 2, None);
}

#[test]
fn test_rising_factorial() {
    apply_fn_to_unsigneds!(rising_factorial_helper);
    // type-specific magnitudes
    assert_eq!(3u16.checked_rising_factorial(4), Some(360));
    assert_eq!(3u8.checked_rising_factorial(4), None);
    assert_eq!(2u64.checked_rising_factorial(19), Some(2432902008176640000));
    // - a nonzero product that overflows
    assert_eq!(2u64.checked_rising_factorial(20), None);
}

fn rising_factorial_signed_helper<
    T: CheckedRisingFactorial + PrimitiveSigned + RisingFactorial<Output = T>,
>() {
    let test = |x: T, n: u64, out: Option<T>| {
        assert_eq!(x.checked_rising_factorial(n), out);
        if let Some(out) = out {
            assert_eq!(x.rising_factorial(n), out);
        }
    };
    // - an all-negative factor sequence, odd and even lengths
    test(T::exact_from(-5), 3, Some(T::exact_from(-60)));
    test(T::exact_from(-4), 2, Some(T::exact_from(12)));
    test(T::exact_from(-3), 3, Some(T::exact_from(-6)));
    // - a factor sequence that reaches zero
    test(T::exact_from(-3), 4, Some(T::ZERO));
    // - a factor sequence that crosses zero
    test(T::exact_from(-2), 5, Some(T::ZERO));
    // - a zero-spanning sequence whose partial products would overflow: the span must be detected
    //   before multiplying, since the result is an exactly representable zero. The base is clamped
    //   so that the span genuinely reaches zero at every width, including i128, whose most negative
    //   values are more than u64::MAX steps below zero.
    test(T::saturating_from(-1000000i64), u64::MAX, Some(T::ZERO));
    test(T::NEGATIVE_ONE, 1, Some(T::NEGATIVE_ONE));
    // - the most negative value is a valid single factor
    test(T::MIN, 1, Some(T::MIN));
    // - the factor increment overflows at the positive end
    test(T::MAX, 2, None);
}

#[test]
fn test_rising_factorial_signed() {
    apply_fn_to_signeds!(rising_factorial_signed_helper);
    // type-specific magnitudes: (-30)(-29) = 870 overflows i8 but fits i16
    assert_eq!((-30i8).checked_rising_factorial(2), None);
    assert_eq!((-30i16).checked_rising_factorial(2), Some(870));
    // the row that first caught the missing span check: the sequence from -100 crosses zero after
    // partial products far beyond i8
    assert_eq!((-100i8).checked_rising_factorial(101), Some(0));
}

fn rising_factorial_fail_helper<T: PrimitiveUnsigned + RisingFactorial<Output = T>>() {
    assert_panic!(T::MAX.rising_factorial(2));
}

#[test]
fn rising_factorial_fail() {
    apply_fn_to_unsigneds!(rising_factorial_fail_helper);
}

fn rising_factorial_properties_helper<
    T: CheckedBinomialCoefficient
        + CheckedFactorial
        + CheckedRisingFactorial
        + PrimitiveUnsigned
        + RisingFactorial<Output = T>,
>() {
    unsigned_pair_gen_var_2::<T, u64>().test_properties(|(x, n)| {
        let rf = x.checked_rising_factorial(n);
        if let Some(rf) = rf {
            assert_eq!(x.rising_factorial(n), rf);
            // the recurrence x^(n + 1) = x^(n) * (x + n)
            if let (Some(next), Some(factor)) = (
                x.checked_rising_factorial(n + 1),
                x.checked_add(T::exact_from(n)),
            ) {
                assert_eq!(next, rf.checked_mul(factor).unwrap());
            }
            // the identity x^(n) = binomial(x + n - 1, n) * n!
            if n != 0 && x != T::ZERO {
                let top = x.checked_add(T::exact_from(n - 1)).unwrap();
                if let (Some(b), Some(f)) = (
                    T::checked_binomial_coefficient(top, T::exact_from(n)),
                    T::checked_factorial(n),
                ) {
                    assert_eq!(rf, b.checked_mul(f).unwrap());
                }
            }
        }
        assert_eq!(x.checked_rising_factorial(0), Some(T::ONE));
        assert_eq!(x.checked_rising_factorial(1), Some(x));
    });
}

#[test]
fn rising_factorial_properties() {
    apply_fn_to_unsigneds!(rising_factorial_properties_helper);
}

fn rising_factorial_signed_properties_helper<
    T: CheckedRisingFactorial + PrimitiveSigned + RisingFactorial<Output = T> + UnsignedAbs,
>()
where
    <T as UnsignedAbs>::Output: CheckedRisingFactorial + PrimitiveUnsigned,
{
    signed_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, n)| {
        let rf = x.checked_rising_factorial(n);
        // a negative base whose factor sequence reaches or crosses zero gives exactly zero
        if x < T::ZERO
            && n != 0
            && x.unsigned_abs() <= <T as UnsignedAbs>::Output::saturating_from(n - 1)
        {
            assert_eq!(rf, Some(T::ZERO));
        }
        if let Some(rf) = rf {
            assert_eq!(x.rising_factorial(n), rf);
            if x > T::ZERO {
                // agreement with the unsigned form
                let urf = x.unsigned_abs().checked_rising_factorial(n);
                if let Some(urf) = urf {
                    assert_eq!(rf.unsigned_abs(), urf);
                    assert!(rf > T::ZERO);
                }
            }
            // sign: zero within span, else (-1)^n for negative bases
            if x < T::ZERO && rf != T::ZERO {
                assert_eq!(rf < T::ZERO, n.odd());
            }
        }
        assert_eq!(x.checked_rising_factorial(0), Some(T::ONE));
        assert_eq!(x.checked_rising_factorial(1), Some(x));
    });
}

#[test]
fn rising_factorial_signed_properties() {
    apply_fn_to_signeds!(rising_factorial_signed_properties_helper);
}
