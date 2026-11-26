// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::is_prime_aprcl::u128_is_prime_aprcl;
use malachite_base::num::factorization::traits::{IsPrime, IsProbablePrime, IsStrongProbablePrime};
use malachite_base::test_util::generators::{unsigned_gen_var_37, unsigned_gen_var_38};

// The Carmichael numbers (6k + 1)(12k + 1)(18k + 1), for k > start, that are at least 2^64 and are
// base-2 strong probable primes.
fn carmichael_pseudoprimes(start: u64) -> impl Iterator<Item = u128> {
    (start + 1..).filter_map(|k| {
        let (a, b, c) = (6 * k + 1, 12 * k + 1, 18 * k + 1);
        if !(a.is_prime() && b.is_prime() && c.is_prime()) {
            return None;
        }
        let n = u128::from(a) * u128::from(b) * u128::from(c);
        if n >> 64 != 0 && n.is_strong_probable_prime(2) {
            Some(n)
        } else {
            None
        }
    })
}

#[test]
fn test_u128_is_prime_aprcl() {
    let test = |n: u128, out| {
        assert_eq!(u128_is_prime_aprcl(n), out);
        assert_eq!(n.is_prime(), out);
    };
    // the smallest prime above 2^64
    test(18446744073709551629, true);
    test(4533471823554859405148161, true);
    // 2^89 - 1
    test(618970019642690137449562111, true);
    // 2^127 - 1
    test(170141183460469231731687303715884105727, true);
    // the largest prime below 2^128, which the n - 1 and n + 1 tests cannot settle
    test(340282366920938463463374607431768211297, true);
    // 2^128 - 1
    test(340282366920938463463374607431768211455, false);
    // 2^64 + 1
    test(18446744073709551617, false);
    // the square of a prime
    test(18446744202558570721, false);
    // the product of the two largest primes below 2^64
    test(340282366920938460843936948965011886881, false);
    // a Carmichael base-2 strong pseudoprime
    test(3557725523452902604315321, false);
    // - gcd(n, s) > 1
    test(23682254247937312867, false);
    // - (L_2) shown by a check with p = 2, k = 1
    test(30427915392650887609, true);
    // - additional test with p = 2
    test(30550640989149658231, true);
    // - additional test with p odd
    test(33470509479069635981, true);
}

#[test]
fn u128_is_prime_aprcl_properties() {
    unsigned_gen_var_37().test_properties(|n| {
        assert!(u128_is_prime_aprcl(n));
        assert!(n.is_prime());
    });

    unsigned_gen_var_38().test_properties(|n| {
        let prime = n.is_probable_prime();
        assert_eq!(u128_is_prime_aprcl(n), prime);
        assert_eq!(n.is_prime(), prime);
    });

    for n in carmichael_pseudoprimes(1_400_000).take(50) {
        assert!(!u128_is_prime_aprcl(n));
    }
    for n in carmichael_pseudoprimes(14_000_000).take(50) {
        assert!(!u128_is_prime_aprcl(n));
    }
}
