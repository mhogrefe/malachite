// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::traits::{IsPrime, IsProbablePrime};
use malachite_base::test_util::generators::{unsigned_gen, unsigned_gen_var_29};
use malachite_base::test_util::num::factorization::is_lucas_probable_prime::*;
use malachite_base::test_util::num::factorization::is_strong_probable_prime::*;

fn bpsw_naive(n: u128) -> bool {
    is_strong_probable_prime_naive(n, 2) && is_lucas_probable_prime_naive(n)
}

#[test]
fn test_is_probable_prime() {
    for n in 0..100000u32 {
        let out = n.is_prime();
        assert_eq!(n.is_probable_prime(), out);
        assert_eq!(u64::from(n).is_probable_prime(), out);
        assert_eq!(u128::from(n).is_probable_prime(), out);
        assert_eq!(bpsw_naive(u128::from(n)), out);
    }
    let test = |n: u128, out| {
        assert_eq!(n.is_probable_prime(), out);
        assert_eq!(bpsw_naive(n), out);
    };
    // - n < 2^64
    test(0, false);
    // - ll_is_prime decides n
    test(18446744073709551616, false);
    // - ll_is_prime leaves n open, so a Lucas test decides it
    test(4533471823554859405148161, true);
    test(18446744073709551557, true);
    test(18446744073709551615, false);
    test(18446744073709551629, true);
    test(318665857834031151167461, false);
    test(3317044064679887385961981, false);
    test(618970019642690137449562111, true);
    test(170141183460469231731687303715884105727, true);
    test(340282366920938463463374607431768211297, true);
    test(340282366920938463463374607431768211455, false);
    test(170141183460469231731687303715884105728, false);
    // The primes nearest 2^64 and 2^128
    for n in (1u128 << 64)..(1u128 << 64) + 13 {
        test(n, false);
    }
    for n in u128::MAX - 157..=u128::MAX {
        test(n, false);
    }
}

#[test]
fn is_probable_prime_properties() {
    unsigned_gen::<u128>().test_properties(|n| {
        assert_eq!(n.is_probable_prime(), bpsw_naive(n));
    });
    unsigned_gen::<u64>().test_properties(|n| {
        assert_eq!(n.is_probable_prime(), n.is_prime());
        assert_eq!(u128::from(n).is_probable_prime(), n.is_prime());
    });
    unsigned_gen::<u32>().test_properties(|n| {
        assert_eq!(n.is_probable_prime(), n.is_prime());
    });
    unsigned_gen::<usize>().test_properties(|n| {
        assert_eq!(n.is_probable_prime(), n.is_prime());
    });
    // Products of two primes are composite.
    unsigned_gen_var_29::<u64>().test_properties(|p| {
        let q = (p + 1..=u64::MAX).find(IsPrime::is_prime);
        if let Some(q) = q {
            assert!(!(u128::from(p) * u128::from(q)).is_probable_prime());
        }
        assert!(!(u128::from(p) * u128::from(p)).is_probable_prime());
        assert!(!(u128::from(p) * 2305843009213693951).is_probable_prime());
    });
}
