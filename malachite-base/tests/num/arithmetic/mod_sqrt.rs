// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::ModSqrt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{unsigned_gen, unsigned_pair_gen_var_16};
use std::panic::catch_unwind;

#[test]
fn test_mod_sqrt() {
    fn test<T: ModSqrt<T, Output = T> + PrimitiveUnsigned>(x: T, m: T, out: Option<T>) {
        assert_eq!(x.mod_sqrt(m), out);
    }
    // - x <= 1
    test(0u8, 1, Some(0));
    test(0u16, 2, Some(0));
    test(1u32, 2, Some(1));
    test(1u64, 100, Some(1));
    // - small moduli use the exhaustive search
    test(2u8, 3, None);
    test(4u16, 5, Some(2));
    test(2u32, 7, Some(3));
    test(4u64, 6, Some(2));
    test(3u128, 6, None);
    test(2usize, 53, None);
    // - m >= 600 and even, or a perfect square
    test(4u32, 600, None);
    test(4u64, 841, None);
    // - composite m = 3 mod 4 with Jacobi symbol 1: a value that is not a root; matches FLINT
    test(3u16, 611, Some(183));
    // - composite m = 1 mod 8: the Tonelli-Shanks iteration cap; matches FLINT
    test(2u32, 609, None);
    // - prime m = 1 mod 8: Tonelli-Shanks
    test(12909u32, 65537, Some(50618));
    test(12909u64, 65537, Some(50618));
    test(12909u128, 65537, Some(50618));
    // The largest moduli, where FLINT's `n_sqrtmod` computes `(m + 1) / 4` and `(m + 3) / 8` with
    // wrapping arithmetic. These exponents are instead computed exactly here, matching what FLINT's
    // `fmpz_sqrtmod` computes for the same moduli. All these moduli are composite, so the results
    // do not mean much; the tests pin the behavior down.
    // - m = u32::MAX = 3 mod 4
    test(2u32, u32::MAX, Some(1));
    // - m = u64::MAX = 3 mod 4
    test(2u64, u64::MAX, Some(1));
    // - m = 2 ^ 32 - 3 = 5 mod 8
    test(3u32, u32::MAX - 2, Some(3858954816));
    // - m = 2 ^ 64 - 3 = 5 mod 8
    test(3u64, u64::MAX - 2, Some(7342525817502822421));
}

fn mod_sqrt_fail_helper<T: ModSqrt<T, Output = T> + PrimitiveUnsigned>() {
    assert_panic!(T::from(3u8).mod_sqrt(T::from(3u8)));
    assert_panic!(T::from(30u8).mod_sqrt(T::from(3u8)));
    assert_panic!(T::ZERO.mod_sqrt(T::ZERO));
}

#[test]
fn mod_sqrt_fail() {
    apply_fn_to_unsigneds!(mod_sqrt_fail_helper);
}

fn mod_sqrt_properties_helper<T: ModSqrt<T, Output = T> + PrimitiveUnsigned>() {
    unsigned_pair_gen_var_16::<T>().test_properties(|(x, m)| {
        let result = x.mod_sqrt(m);
        if let Some(r) = result {
            assert!(r < m);
        }
        if x <= T::ONE {
            assert_eq!(result, Some(x));
        }
        // For odd moduli the sub-600 search is exhaustive.
        if m.odd() && m < T::saturating_from(600u16) {
            if let Some(r) = result {
                assert_eq!(r.mod_mul(r, m), x);
            } else {
                let mut t = T::ZERO;
                while t < m {
                    assert_ne!(t.mod_mul(t, m), x);
                    t += T::ONE;
                }
            }
        }
    });

    unsigned_gen::<T>().test_properties(|m| {
        if m != T::ZERO {
            assert_eq!(T::ZERO.mod_sqrt(m), Some(T::ZERO));
        }
    });
}

#[test]
fn mod_sqrt_properties() {
    apply_fn_to_unsigneds!(mod_sqrt_properties_helper);

    // widths are consistent with each other
    unsigned_pair_gen_var_16::<u8>().test_properties(|(x, m)| {
        let result = x.mod_sqrt(m);
        assert_eq!(
            u16::from(x).mod_sqrt(u16::from(m)).map(u8::exact_from),
            result
        );
        assert_eq!(
            u32::from(x).mod_sqrt(u32::from(m)).map(u8::exact_from),
            result
        );
    });
    unsigned_pair_gen_var_16::<u64>().test_properties(|(x, m)| {
        let result = x.mod_sqrt(m);
        assert_eq!(
            u128::from(x).mod_sqrt(u128::from(m)).map(u64::exact_from),
            result
        );
        if let (Ok(x_usize), Ok(m_usize)) = (usize::try_from(x), usize::try_from(m)) {
            assert_eq!(x_usize.mod_sqrt(m_usize).map(u64::exact_from), result);
        }
    });
}
