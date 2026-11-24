// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::is_prime::u128_is_strong_probable_prime_odd;
use malachite_base::num::factorization::traits::{IsPrime, IsStrongProbablePrime};
use malachite_base::test_util::generators::{unsigned_gen, unsigned_gen_var_29, unsigned_pair_gen};
use malachite_base::test_util::num::factorization::is_strong_probable_prime::*;

#[test]
fn test_is_strong_probable_prime() {
    let test = |n: u64, a: u64, out| {
        assert_eq!(n.is_strong_probable_prime(a), out);
        assert_eq!(u128::from(n).is_strong_probable_prime(u128::from(a)), out);
        assert_eq!(
            is_strong_probable_prime_naive(u128::from(n), u128::from(a)),
            out
        );
        if let Ok(n) = u32::try_from(n) {
            assert_eq!(n.is_strong_probable_prime(u32::try_from(a).unwrap()), out);
        }
    };
    // - n <= 1
    test(0, 2, false);
    test(1, 2, false);
    // - a == 0 || a == 1 || a == nm1
    test(2, 0, true);
    test(2, 2, true);
    // - n.even()
    test(4, 2, false);
    // - n.odd()
    test(5, 2, true);
    test(2, 3, true);
    test(3, 2, true);
    test(4, 2, false);
    test(4, 3, true);
    test(9, 2, false);
    test(15, 4, false);
    test(2047, 2, true);
    test(2047, 3, false);
    test(1373653, 2, true);
    test(1373653, 3, true);
    test(1373653, 5, false);
    test(25326001, 2, true);
    test(25326001, 3, true);
    test(25326001, 5, true);
    test(25326001, 7, false);
    test(3215031751, 2, true);
    test(3215031751, 7, true);
    test(3215031751, 11, false);
    test(3825123056546413051, 2, true);
    test(3825123056546413051, 37, false);
    test(3825123056546413051, 41, false);
    test(18446744073709551557, 2, true);
    test(18446744073709551615, 2, false);
    test(18446744073709551615, 5, false);

    let test = |n: u128, a: u128, out| {
        assert_eq!(n.is_strong_probable_prime(a), out);
        assert_eq!(is_strong_probable_prime_naive(n, a), out);
    };
    // - n >= 2^64 && (a == 0 || a == 1 || a == nm1) in u128_is_strong_probable_prime
    test(18446744073709551616, 0, true);
    // - n >= 2^64 && n.even() in u128_is_strong_probable_prime
    test(18446744073709551616, 2, false);
    // - n >= 2^64 && n.odd() in u128_is_strong_probable_prime
    // - a == 2 in u128_is_strong_probable_prime_odd
    // - exp < 128 in redc_two_pow_128
    // - y == ctx.one || y == minus_one is false at first in u128_is_strong_probable_prime_odd
    // - a later square is -1 in u128_is_strong_probable_prime_odd
    test(18446744073709551617, 2, true);
    test(221360928884514619393, 2, true);
    test(149935135831111235534849, 2, false);
    // - exp >= 128 in redc_two_pow_128
    test(4873482210321473860534273, 2, true);
    test(148726874094283259905, 2, false);
    test(338953138925153547590470800371487866881, 2, false);
    // - a != 2 in u128_is_strong_probable_prime_odd
    // - every square fails in u128_is_strong_probable_prime_odd
    test(18446744073709551617, 3, false);
    // - y == ctx.one || y == minus_one at first in u128_is_strong_probable_prime_odd
    test(18446744073709551667, 2, true);
    // - !overflow_1 && !overflow_2 && t >= ctx.n in redc_mul_128
    test(36893488147419103233, 2, false);
    // - overflow_1 in redc_mul_128
    // - s == 0 in rem_256_by_128
    test(329648542954659136480144150949525454849, 2, false);
    // - overflow_2 in redc_mul_128
    test(340282366920938463463374607431768208739, 3, true);
    // Sorenson and Webster's psi_12: a strong pseudoprime to the first 12 prime bases
    test(318665857834031151167461, 2, true);
    test(318665857834031151167461, 3, true);
    test(318665857834031151167461, 41, false);
    // psi_13: a strong pseudoprime to the first 13 prime bases
    test(3317044064679887385961981, 2, true);
    test(3317044064679887385961981, 41, true);
    test(618970019642690137449562111, 2, true);
    test(170141183460469231731687303715884105727, 3, true);
    test(340282366920938463463374607431768211297, 41, true);
    test(18446744073709551629, 2, true);
    test(340282366920938463463374607431768211455, 2, false);
    test(170141183460469231731687303715884105728, 3, false);
}

#[test]
fn is_strong_probable_prime_properties() {
    unsigned_pair_gen::<u64, u64>().test_properties(|(n, a)| {
        let b = n.is_strong_probable_prime(a);
        assert_eq!(
            is_strong_probable_prime_naive(u128::from(n), u128::from(a)),
            b
        );
        assert_eq!(u128::from(n).is_strong_probable_prime(u128::from(a)), b);
        if n.is_prime() {
            assert!(b);
        }
    });
    unsigned_pair_gen::<u128, u128>().test_properties(|(n, a)| {
        assert_eq!(
            n.is_strong_probable_prime(a),
            is_strong_probable_prime_naive(n, a)
        );
    });
    unsigned_pair_gen::<u32, u32>().test_properties(|(n, a)| {
        assert_eq!(
            n.is_strong_probable_prime(a),
            u64::from(n).is_strong_probable_prime(u64::from(a))
        );
    });
    unsigned_pair_gen::<u8, u8>().test_properties(|(n, a)| {
        assert_eq!(
            n.is_strong_probable_prime(a),
            u64::from(n).is_strong_probable_prime(u64::from(a))
        );
    });
    unsigned_pair_gen::<u16, u16>().test_properties(|(n, a)| {
        assert_eq!(
            n.is_strong_probable_prime(a),
            u64::from(n).is_strong_probable_prime(u64::from(a))
        );
    });
    unsigned_pair_gen::<usize, usize>().test_properties(|(n, a)| {
        assert_eq!(
            n.is_strong_probable_prime(a),
            u64::try_from(n)
                .unwrap()
                .is_strong_probable_prime(u64::try_from(a).unwrap())
        );
    });
    // The two-limb Montgomery code, on odd moduli of every size
    unsigned_pair_gen::<u64, u64>().test_properties(|(n, a)| {
        let n = u128::from(n | 1);
        if n > 1 {
            let a = u128::from(a) % n;
            if a > 1 && a != n - 1 {
                assert_eq!(
                    u128_is_strong_probable_prime_odd(n, a),
                    is_strong_probable_prime_naive(n, a)
                );
            }
        }
    });
    // The base-2 path, on odd moduli of every size
    unsigned_gen::<u128>().test_properties(|n| {
        let n = n | 1;
        if n > 3 {
            assert_eq!(
                u128_is_strong_probable_prime_odd(n, 2),
                is_strong_probable_prime_naive(n, 2)
            );
        }
    });
    unsigned_gen_var_29::<u64>().test_properties(|p| {
        for a in [2, 3, 5, 7, 1234567, u64::MAX] {
            assert!(p.is_strong_probable_prime(a));
        }
    });
}
