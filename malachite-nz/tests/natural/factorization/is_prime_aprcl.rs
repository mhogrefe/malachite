// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::num::factorization::is_prime_aprcl::u128_is_prime_aprcl;
use malachite_base::num::factorization::traits::{IsPrime, IsProbablePrime, IsStrongProbablePrime};
use malachite_base::test_util::generators::unsigned_gen_var_38;
use malachite_nz::natural::Natural;
use malachite_nz::natural::factorization::is_prime_aprcl::natural_is_prime_aprcl;
use malachite_nz::test_util::generators::{natural_gen_var_11, natural_gen_var_12};

// The Carmichael numbers (6k + 1)(12k + 1)(18k + 1), for k > start, that are base-2 strong probable
// primes.
fn carmichael_pseudoprimes(start: u64) -> impl Iterator<Item = Natural> {
    (start + 1..).filter_map(|k| {
        let (a, b, c) = (6 * k + 1, 12 * k + 1, 18 * k + 1);
        if !(a.is_prime() && b.is_prime() && c.is_prime()) {
            return None;
        }
        let n = Natural::from(a) * Natural::from(b) * Natural::from(c);
        if n.is_strong_probable_prime(&Natural::TWO) {
            Some(n)
        } else {
            None
        }
    })
}

fn mersenne(p: u64) -> Natural {
    Natural::power_of_2(p) - Natural::ONE
}

#[test]
fn test_natural_is_prime_aprcl() {
    let test = |n: &Natural, out| {
        assert_eq!(natural_is_prime_aprcl(n), out);
        assert_eq!(n.is_prime(), out);
    };
    // 2^128 + 51
    test(
        &Natural::from_str("340282366920938463463374607431768211507").unwrap(),
        true,
    );
    // 2^128 + 1, a base-2 strong pseudoprime
    test(
        &Natural::from_str("340282366920938463463374607431768211457").unwrap(),
        false,
    );
    // a base-2 strong pseudoprime
    test(
        &Natural::from_str("1971171711155257004609808164365888160584741").unwrap(),
        false,
    );
    test(&mersenne(521), true);
    test(&mersenne(607), true);
    // more than 16 limbs, where FLINT would switch to its fmpz_mod-based arithmetic
    test(&mersenne(1279), true);
    test(&(mersenne(607) * mersenne(521)), false);
    // - gcd(n, s) > 1
    test(
        &Natural::from_str("594641182536899171686359977315952157339").unwrap(),
        false,
    );
    // - v == 1 in check_22_mpn
    // - (L_2) shown by a check with p = 2, k = 1
    test(
        &Natural::from_str("1421277983357097886078192509515139813024864432569").unwrap(),
        true,
    );
    // - additional tests with p = 2 and p odd
    test(
        &Natural::from_str("3299227479755839416682855924802464938529302811").unwrap(),
        true,
    );
    // - v = 1 or 3 mod 8 in check_2k_mpn
    test(&(Natural::power_of_2(1030) + Natural::from(2185u32)), true);
    // - R is grown beyond the table
    // - q > MAX_Q
    test(
        &((Natural::from(3u32) << 20010u32) + Natural::from(3u32)),
        false,
    );
}

#[test]
fn natural_is_prime_aprcl_properties() {
    natural_gen_var_12().test_properties_with_limit(100, |n| {
        assert!(natural_is_prime_aprcl(&n));
        assert!(n.is_prime());
    });

    natural_gen_var_11().test_properties_with_limit(200, |n| {
        let prime = n.is_probable_prime();
        assert_eq!(natural_is_prime_aprcl(&n), prime);
        assert_eq!(n.is_prime(), prime);
    });

    // The Natural test agrees with the u128 one.
    unsigned_gen_var_38().test_properties_with_limit(1000, |x| {
        assert_eq!(
            natural_is_prime_aprcl(&Natural::from(x)),
            u128_is_prime_aprcl(x)
        );
    });

    for n in carmichael_pseudoprimes(10_000_000_000_000).take(10) {
        assert!(!natural_is_prime_aprcl(&n));
    }
}
