// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::BinomialCoefficient;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    signed_gen, signed_gen_var_2, signed_pair_gen_var_11, signed_pair_gen_var_12, unsigned_gen,
    unsigned_gen_var_1, unsigned_pair_gen_var_28, unsigned_pair_gen_var_44,
};
use std::panic::catch_unwind;

#[test]
fn test_binomial_coefficient() {
    fn test_u<T: PrimitiveUnsigned>(n: T, k: T, out: T) {
        assert_eq!(T::binomial_coefficient(n, k), out);
    }
    test_u::<u8>(0, 0, 1);

    test_u::<u8>(1, 0, 1);
    test_u::<u8>(1, 1, 1);

    test_u::<u8>(2, 0, 1);
    test_u::<u8>(2, 1, 2);
    test_u::<u8>(2, 2, 1);

    test_u::<u8>(3, 0, 1);
    test_u::<u8>(3, 1, 3);
    test_u::<u8>(3, 2, 3);
    test_u::<u8>(3, 3, 1);

    test_u::<u8>(4, 0, 1);
    test_u::<u8>(4, 1, 4);
    test_u::<u8>(4, 2, 6);
    test_u::<u8>(4, 3, 4);
    test_u::<u8>(4, 4, 1);

    test_u::<u8>(1, 2, 0);
    test_u::<u8>(10, 5, 252);
    test_u::<u128>(100, 50, 100891344545564193334812497256);

    fn test_i<T: PrimitiveSigned>(n: T, k: T, out: T) {
        assert_eq!(T::binomial_coefficient(n, k), out);
    }
    test_i::<i8>(0, 0, 1);

    test_i::<i8>(1, 0, 1);
    test_i::<i8>(1, 1, 1);

    test_i::<i8>(2, 0, 1);
    test_i::<i8>(2, 1, 2);
    test_i::<i8>(2, 2, 1);

    test_i::<i8>(3, 0, 1);
    test_i::<i8>(3, 1, 3);
    test_i::<i8>(3, 2, 3);
    test_i::<i8>(3, 3, 1);

    test_i::<i8>(4, 0, 1);
    test_i::<i8>(4, 1, 4);
    test_i::<i8>(4, 2, 6);
    test_i::<i8>(4, 3, 4);
    test_i::<i8>(4, 4, 1);

    test_i::<i8>(1, 2, 0);
    test_i::<i16>(10, 5, 252);
    test_i::<i128>(100, 50, 100891344545564193334812497256);

    test_i::<i8>(-1, 0, 1);
    test_i::<i8>(-1, 1, -1);

    test_i::<i8>(-2, 0, 1);
    test_i::<i8>(-2, 1, -2);
    test_i::<i8>(-2, 2, 3);

    test_i::<i8>(-3, 0, 1);
    test_i::<i8>(-3, 1, -3);
    test_i::<i8>(-3, 2, 6);
    test_i::<i8>(-3, 3, -10);

    test_i::<i8>(-1, 2, 1);
    test_i::<i16>(-10, 5, -2002);
    test_i::<i128>(-80, 50, 1828256793482238093393785743858493760);

    test_i::<i8>(-128, 1, -128);
    test_i::<i8>(-2, 127, -128);
}

#[test]
pub fn binomial_coefficient_fail() {
    assert_panic!(u8::binomial_coefficient(11, 5));
    assert_panic!(u128::binomial_coefficient(1000000, 1000));
    assert_panic!(i8::binomial_coefficient(11, 5));
    assert_panic!(i128::binomial_coefficient(1000000, 1000));
    assert_panic!(i8::binomial_coefficient(1, -1));
}

#[test]
fn test_checked_binomial_coefficient() {
    fn test_u<T: PrimitiveUnsigned>(n: T, k: T, out: Option<T>) {
        assert_eq!(T::checked_binomial_coefficient(n, k), out);
    }
    test_u::<u8>(0, 0, Some(1));

    test_u::<u8>(1, 0, Some(1));
    test_u::<u8>(1, 1, Some(1));

    test_u::<u8>(2, 0, Some(1));
    test_u::<u8>(2, 1, Some(2));
    test_u::<u8>(2, 2, Some(1));

    test_u::<u8>(3, 0, Some(1));
    test_u::<u8>(3, 1, Some(3));
    test_u::<u8>(3, 2, Some(3));
    test_u::<u8>(3, 3, Some(1));

    test_u::<u8>(4, 0, Some(1));
    test_u::<u8>(4, 1, Some(4));
    test_u::<u8>(4, 2, Some(6));
    test_u::<u8>(4, 3, Some(4));
    test_u::<u8>(4, 4, Some(1));

    test_u::<u8>(1, 2, Some(0));
    test_u::<u8>(10, 5, Some(252));
    test_u::<u128>(100, 50, Some(100891344545564193334812497256));

    test_u::<u8>(11, 5, None);
    test_u::<u128>(1000000, 1000, None);

    fn test_i<T: PrimitiveSigned>(n: T, k: T, out: Option<T>) {
        assert_eq!(T::checked_binomial_coefficient(n, k), out);
    }
    test_i::<i8>(0, 0, Some(1));

    test_i::<i8>(1, 0, Some(1));
    test_i::<i8>(1, 1, Some(1));

    test_i::<i8>(2, 0, Some(1));
    test_i::<i8>(2, 1, Some(2));
    test_i::<i8>(2, 2, Some(1));

    test_i::<i8>(3, 0, Some(1));
    test_i::<i8>(3, 1, Some(3));
    test_i::<i8>(3, 2, Some(3));
    test_i::<i8>(3, 3, Some(1));

    test_i::<i8>(4, 0, Some(1));
    test_i::<i8>(4, 1, Some(4));
    test_i::<i8>(4, 2, Some(6));
    test_i::<i8>(4, 3, Some(4));
    test_i::<i8>(4, 4, Some(1));

    test_i::<i8>(1, 2, Some(0));
    test_i::<i16>(10, 5, Some(252));
    test_i::<i128>(100, 50, Some(100891344545564193334812497256));

    test_i::<i8>(-1, 0, Some(1));
    test_i::<i8>(-1, 1, Some(-1));

    test_i::<i8>(-2, 0, Some(1));
    test_i::<i8>(-2, 1, Some(-2));
    test_i::<i8>(-2, 2, Some(3));

    test_i::<i8>(-3, 0, Some(1));
    test_i::<i8>(-3, 1, Some(-3));
    test_i::<i8>(-3, 2, Some(6));
    test_i::<i8>(-3, 3, Some(-10));

    test_i::<i8>(-1, 2, Some(1));
    test_i::<i16>(-10, 5, Some(-2002));
    test_i::<i128>(-80, 50, Some(1828256793482238093393785743858493760));

    test_i::<i8>(-128, 1, Some(-128));
    test_i::<i8>(-2, 127, Some(-128));

    test_i::<i8>(11, 5, None);
    test_i::<i128>(1000000, 1000, None);
    test_i::<i8>(1, -1, None);
}

fn binomial_coefficient_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_44::<T>().test_properties(|(n, k)| {
        let b = T::binomial_coefficient(n, k);
        assert_eq!(b == T::ZERO, n < k);
        if n >= k {
            assert_eq!(T::binomial_coefficient(n, n - k), b);
        }
        if n != T::ZERO && k != T::ZERO {
            let c = T::binomial_coefficient(n - T::ONE, k - T::ONE);
            assert_eq!(T::binomial_coefficient(n - T::ONE, k) + c, b);
            let gcd = n.gcd(k);
            assert_eq!(c / (k / gcd) * (n / gcd), b);
        }
    });

    unsigned_gen::<T>().test_properties(|n| {
        assert_eq!(T::binomial_coefficient(n, T::ZERO), T::ONE);
        assert_eq!(T::binomial_coefficient(n, T::ONE), n);
    });

    unsigned_gen_var_1::<T>().test_properties(|n| {
        assert_eq!(T::binomial_coefficient(n, n), T::ONE);
        assert_eq!(T::binomial_coefficient(n, n - T::ONE), n);
        assert_eq!(T::binomial_coefficient(T::ZERO, n), T::ZERO);
    });
}

fn binomial_coefficient_helper_signed<T: PrimitiveSigned>() {
    signed_pair_gen_var_12::<T>().test_properties(|(n, k)| {
        let b = T::binomial_coefficient(n, k);
        assert_eq!(b == T::ZERO, n >= T::ZERO && n < k);
        if n >= k {
            assert_eq!(T::binomial_coefficient(n, n - k), b);
        }
        if n != T::MIN && k != T::ZERO {
            if let Some(c) = T::checked_binomial_coefficient(n - T::ONE, k) {
                assert_eq!(c + T::binomial_coefficient(n - T::ONE, k - T::ONE), b);
            }
        }
        if n != T::MIN {
            if let Some(s) = (n - T::ONE).checked_add(k) {
                if let Some(mut b_alt) = T::checked_binomial_coefficient(s, k) {
                    if k.odd() {
                        b_alt.neg_assign();
                    }
                    assert_eq!(T::binomial_coefficient(-n, k), b_alt);
                }
            }
        }
    });

    signed_gen::<T>().test_properties(|n| {
        assert_eq!(T::binomial_coefficient(n, T::ONE), n);
        assert_eq!(T::binomial_coefficient(n, T::ZERO), T::ONE);
    });

    signed_gen_var_2::<T>().test_properties(|n| {
        assert_eq!(T::binomial_coefficient(n, n), T::ONE);
        if n != T::ZERO {
            assert_eq!(T::binomial_coefficient(n, n - T::ONE), n);
            assert_eq!(T::binomial_coefficient(T::ZERO, n), T::ZERO);
        }
    });
}

#[test]
fn binomial_coefficient_properties() {
    apply_fn_to_unsigneds!(binomial_coefficient_helper_unsigned);
    apply_fn_to_signeds!(binomial_coefficient_helper_signed);
}

fn checked_binomial_coefficient_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_28::<T, T>().test_properties(|(n, k)| {
        if let Some(b) = T::checked_binomial_coefficient(n, k) {
            assert_eq!(T::binomial_coefficient(n, k), b);
        }
    });
}

fn checked_binomial_coefficient_helper_signed<T: PrimitiveSigned>() {
    signed_pair_gen_var_11::<T, T>().test_properties(|(n, k)| {
        if let Some(b) = T::checked_binomial_coefficient(n, k) {
            assert_eq!(T::binomial_coefficient(n, k), b);
        }
    });
}

#[test]
fn checked_binomial_coefficient_properties() {
    apply_fn_to_unsigneds!(checked_binomial_coefficient_helper_unsigned);
    apply_fn_to_signeds!(checked_binomial_coefficient_helper_signed);
}
