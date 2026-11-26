// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Two;
use malachite_base::num::factorization::traits::{IsPrime, IsProbablePrime, IsStrongProbablePrime};
use malachite_base::test_util::generators::{unsigned_gen, unsigned_gen_var_37};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{natural_gen, natural_gen_var_10, natural_pair_gen};

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

#[test]
fn test_is_prime() {
    let test = |s, out| {
        let n = Natural::from_str(s).unwrap();
        assert_eq!(n.is_prime(), out);
        assert_eq!(n.is_probable_prime(), out);
    };
    // n < 2^128
    test("0", false);
    test("97", true);
    // the largest prime below 2^128: neither n - 1 nor n + 1 has a large enough factored part, so
    // APR-CL proves it
    test("340282366920938463463374607431768211297", true);
    // n even
    test("340282366920938463463374607431768211458", false);
    // trial division finds a factor
    test("1235679190629732963253495290965596683049", false);
    // n is not a base-2 strong probable prime
    test("2512243197273404973128543252749318682333", false);
    // - 2^128 + 1, a base-2 strong pseudoprime: the n ± 1 tests
    // - q | n - 1
    // - q | n + 1
    // - a cofactor of n ∓ 1 is 1
    // - a base fails for some q in the Pocklington test, so the next base is tried
    // - a^(n - 1) != 1 mod n in the Pocklington test
    test("340282366920938463463374607431768211457", false);
    // 2^128 + 51
    test("340282366920938463463374607431768211507", true);
    // - Lenstra's search: i even and i odd, q < c1, s1 in range, d a square, a1 == 0, i even && a0
    //   == 0, the chain ending, and no divisor found
    // - F1 and F2 are both even in the combined test
    // - lcm(F1, F2)^3 > n: Lenstra's divisors in residue classes
    test(
        "711077930961772430278273020597743461199094952693191409663",
        true,
    );
    // (F2 - 1)^3 > n: the improved n + 1 test
    test(
        "36202327674806051080147073122864486363860901248239399365855455404627",
        true,
    );
    // a cofactor of n ∓ 1 needs a full primality proof
    test(
        "2561057508037765751645002084668727897769761021341326081261567",
        true,
    );
    // 2^521 - 1
    test(
        "686479766013060971498190079908139321726943530014330540939446345918554318339765605212255964\
        0661454554977296311391480858037121987999716643812574028291115057151",
        true,
    );
    // a base-2 strong pseudoprime
    test("1971171711155257004609808164365888160584741", false);
}

#[test]
fn is_prime_properties() {
    natural_gen().test_properties(|n| {
        let prime = n.is_prime();
        assert_eq!(n.is_probable_prime(), prime);
        if let Ok(x) = u128::try_from(&n) {
            assert_eq!(x.is_prime(), prime);
        }
    });

    natural_pair_gen().test_properties(|(a, b)| {
        if a > 1u32 && b > 1u32 {
            assert!(!(a * b).is_prime());
        }
    });

    natural_gen_var_10().test_properties(|n| {
        assert!(n.is_prime());
    });

    unsigned_gen::<u128>().test_properties(|x| {
        assert_eq!(Natural::from(x).is_prime(), x.is_prime());
    });

    unsigned_gen_var_37().test_properties(|x| {
        assert!(Natural::from(x).is_prime());
    });

    for n in carmichael_pseudoprimes(10_000_000_000_000).take(20) {
        assert!(!n.is_prime());
    }
}
