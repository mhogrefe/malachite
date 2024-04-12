// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::extended_gcd::extended_gcd_unsigned_binary;
use malachite_base::num::arithmetic::traits::{ExtendedGcd, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::test_util::generators::{
    signed_gen, signed_pair_gen, unsigned_gen, unsigned_pair_gen_var_27,
};
use malachite_base::test_util::num::arithmetic::extended_gcd::extended_gcd_unsigned_euclidean;
use std::cmp::min;

#[test]
fn test_extended_gcd() {
    fn test_u<
        U: ExtendedGcd<Cofactor = S> + PrimitiveUnsigned + WrappingFrom<S>,
        S: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
    >(
        a: U,
        b: U,
        gcd: U,
        x: S,
        y: S,
    ) {
        assert_eq!(a.extended_gcd(b), (gcd, x, y));
        assert_eq!(extended_gcd_unsigned_euclidean(a, b), (gcd, x, y));
        assert_eq!(extended_gcd_unsigned_binary::<U, S>(a, b), (gcd, x, y));
    }
    test_u::<u8, _>(0, 0, 0, 0, 0);
    test_u::<u8, _>(0, 1, 1, 0, 1);
    test_u::<u8, _>(1, 0, 1, 1, 0);
    test_u::<u8, _>(1, 1, 1, 0, 1);
    test_u::<u16, _>(0, 6, 6, 0, 1);
    test_u::<u32, _>(6, 0, 6, 1, 0);
    test_u::<u64, _>(1, 6, 1, 1, 0);
    test_u::<usize, _>(6, 1, 1, 0, 1);
    test_u::<u128, _>(6, 6, 6, 0, 1);
    test_u::<u128, _>(8, 12, 4, -1, 1);
    test_u::<u8, _>(54, 24, 6, 1, -2);
    test_u::<u16, _>(42, 56, 14, -1, 1);
    test_u::<u32, _>(48, 18, 6, -1, 3);
    test_u::<u64, _>(3, 5, 1, 2, -1);
    test_u::<usize, _>(12, 60, 12, 1, 0);
    test_u::<u128, _>(12, 90, 6, -7, 1);
    test_u::<u16, _>(240, 46, 2, -9, 47);

    fn test_s<U: PrimitiveUnsigned, S: ExtendedGcd<Gcd = U> + PrimitiveSigned>(
        a: S,
        b: S,
        gcd: U,
        x: S,
        y: S,
    ) {
        assert_eq!(a.extended_gcd(b), (gcd, x, y));
    }
    test_s::<_, i8>(0, 0, 0, 0, 0);
    test_s::<_, i8>(0, 1, 1, 0, 1);
    test_s::<_, i8>(0, -1, 1, 0, -1);
    test_s::<_, i8>(1, 0, 1, 1, 0);
    test_s::<_, i8>(-1, 0, 1, -1, 0);
    test_s::<_, i8>(1, 1, 1, 0, 1);
    test_s::<_, i8>(1, -1, 1, 0, -1);
    test_s::<_, i8>(-1, 1, 1, 0, 1);
    test_s::<_, i8>(-1, -1, 1, 0, -1);
    test_s::<_, i16>(0, 6, 6, 0, 1);
    test_s::<_, i16>(0, -6, 6, 0, -1);
    test_s::<_, i32>(6, 0, 6, 1, 0);
    test_s::<_, i32>(-6, 0, 6, -1, 0);
    test_s::<_, i64>(1, 6, 1, 1, 0);
    test_s::<_, i64>(1, -6, 1, 1, 0);
    test_s::<_, i64>(-1, 6, 1, -1, 0);
    test_s::<_, i64>(-1, -6, 1, -1, 0);
    test_s::<_, isize>(6, 1, 1, 0, 1);
    test_s::<_, isize>(6, -1, 1, 0, -1);
    test_s::<_, isize>(-6, 1, 1, 0, 1);
    test_s::<_, isize>(-6, -1, 1, 0, -1);
    test_s::<_, i128>(6, 6, 6, 0, 1);
    test_s::<_, i128>(6, -6, 6, 0, -1);
    test_s::<_, i128>(-6, 6, 6, 0, 1);
    test_s::<_, i128>(-6, -6, 6, 0, -1);
    test_s::<_, i128>(8, 12, 4, -1, 1);
    test_s::<_, i8>(54, 24, 6, 1, -2);
    test_s::<_, i16>(42, 56, 14, -1, 1);
    test_s::<_, i32>(48, 18, 6, -1, 3);
    test_s::<_, i64>(3, 5, 1, 2, -1);
    test_s::<_, i128>(12, 90, 6, -7, 1);
    test_s::<_, i16>(240, 46, 2, -9, 47);
    test_s::<_, i16>(240, -46, 2, -9, -47);
    test_s::<_, i16>(-240, 46, 2, 9, 47);
    test_s::<_, i16>(-240, -46, 2, 9, -47);
    test_s::<_, i8>(-128, -128, 128, 0, -1);
    test_s::<_, i8>(0, -128, 128, 0, -1);
    test_s::<_, i8>(-128, 0, 128, -1, 0);
    test_s::<_, isize>(12, 60, 12, 1, 0);
    test_s::<_, isize>(-12, 60, 12, -1, 0);
    test_s::<_, isize>(12, -60, 12, 1, 0);
    test_s::<_, isize>(-12, -60, 12, -1, 0);
    test_s::<_, isize>(60, 12, 12, 0, 1);
    test_s::<_, isize>(-60, 12, 12, 0, 1);
    test_s::<_, isize>(60, -12, 12, 0, -1);
    test_s::<_, isize>(-60, -12, 12, 0, -1);
}

fn extended_gcd_properties_helper_unsigned<
    U: ExtendedGcd<Cofactor = S> + PrimitiveUnsigned + WrappingFrom<S>,
    S: TryFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>() {
    unsigned_pair_gen_var_27::<U>().test_properties(|(a, b)| {
        let (gcd, x, y) = a.extended_gcd(b);
        assert_eq!(a.gcd(b), gcd);
        assert_eq!(
            S::wrapping_from(a)
                .wrapping_mul(x)
                .wrapping_add(S::wrapping_from(b).wrapping_mul(y)),
            S::wrapping_from(gcd)
        );
        if let (Ok(a), Ok(b), Ok(gcd)) = (S::try_from(a), S::try_from(b), S::try_from(gcd)) {
            if let (Some(ax), Some(by)) = (a.checked_mul(x), b.checked_mul(y)) {
                assert_eq!(ax + by, gcd);
            }
        }

        // uniqueness
        if a != U::ZERO && b != U::ZERO && gcd != min(a, b) {
            assert!(x.unsigned_abs() <= (b / gcd) >> 1);
            assert!(y.unsigned_abs() <= (a / gcd) >> 1);
        }

        let reverse = b.extended_gcd(a);
        if a == b {
            assert_eq!(reverse, (gcd, x, y));
        } else {
            assert_eq!(reverse, (gcd, y, x));
        }

        assert_eq!(extended_gcd_unsigned_euclidean(a, b), (gcd, x, y));
        assert_eq!(extended_gcd_unsigned_binary::<U, S>(a, b), (gcd, x, y));
    });

    unsigned_gen::<U>().test_properties(|x| {
        if x != U::ZERO {
            assert_eq!(x.extended_gcd(x), (x, S::ZERO, S::ONE));
            assert_eq!(x.extended_gcd(U::ZERO), (x, S::ONE, S::ZERO));
            assert_eq!(U::ZERO.extended_gcd(x), (x, S::ZERO, S::ONE));
        }
        if x != U::ONE {
            assert_eq!(U::ONE.extended_gcd(x), (U::ONE, S::ONE, S::ZERO));
        }
        assert_eq!(x.extended_gcd(U::ONE), (U::ONE, S::ZERO, S::ONE));
    });
}

fn extended_gcd_properties_helper_signed<
    U: TryFrom<S> + PrimitiveUnsigned,
    S: ExtendedGcd<Gcd = U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>() {
    signed_pair_gen::<S>().test_properties(|(a, b)| {
        let (gcd, x, y) = a.extended_gcd(b);
        assert!(gcd >= U::ZERO);
        let abs_a = a.unsigned_abs();
        let abs_b = b.unsigned_abs();
        assert_eq!(abs_a.gcd(abs_b), gcd);
        let s_gcd = a.wrapping_mul(x).wrapping_add(b.wrapping_mul(y));
        assert_eq!(s_gcd.unsigned_abs(), gcd);
        if let (Some(ax), Some(by)) = (a.checked_mul(x), b.checked_mul(y)) {
            assert_eq!(U::exact_from(ax + by), gcd);
        }

        // uniqueness
        if a != S::ZERO && b != S::ZERO && gcd != min(abs_a, abs_b) {
            assert!(x.unsigned_abs() <= (abs_b / gcd) >> 1);
            assert!(y.unsigned_abs() <= (abs_a / gcd) >> 1);
        }

        let reverse = b.extended_gcd(a);
        if a == b {
            assert_eq!(reverse, (gcd, x, y));
        } else if b != S::MIN && a == -b {
            assert_eq!(reverse, (gcd, x, -y));
        } else {
            assert_eq!(reverse, (gcd, y, x));
        }
    });

    signed_gen::<S>().test_properties(|x| {
        if x != S::ZERO {
            assert_eq!(
                x.extended_gcd(x),
                (
                    x.unsigned_abs(),
                    S::ZERO,
                    if x >= S::ZERO {
                        S::ONE
                    } else {
                        S::NEGATIVE_ONE
                    }
                )
            );
            if x != S::MIN {
                assert_eq!(
                    x.extended_gcd(-x),
                    (
                        x.unsigned_abs(),
                        S::ZERO,
                        if x < S::ZERO { S::ONE } else { S::NEGATIVE_ONE }
                    )
                );
            }
            assert_eq!(
                x.extended_gcd(S::ZERO),
                (
                    x.unsigned_abs(),
                    if x >= S::ZERO {
                        S::ONE
                    } else {
                        S::NEGATIVE_ONE
                    },
                    S::ZERO
                )
            );
            assert_eq!(
                S::ZERO.extended_gcd(x),
                (
                    x.unsigned_abs(),
                    S::ZERO,
                    if x >= S::ZERO {
                        S::ONE
                    } else {
                        S::NEGATIVE_ONE
                    }
                )
            );
        }
        if x.unsigned_abs() != U::ONE {
            assert_eq!(S::ONE.extended_gcd(x), (U::ONE, S::ONE, S::ZERO));
        }
        assert_eq!(x.extended_gcd(S::ONE), (U::ONE, S::ZERO, S::ONE));
    });
}

#[test]
fn extended_gcd_properties() {
    apply_fn_to_unsigned_signed_pairs!(extended_gcd_properties_helper_unsigned);
    apply_fn_to_unsigned_signed_pairs!(extended_gcd_properties_helper_signed);
}
