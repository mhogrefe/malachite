// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CeilingRoot, FloorSqrt};
use malachite_base::num::factorization::is_prime_n_pm_1::*;
use malachite_base::num::factorization::traits::{IsPrime, IsProbablePrime, IsStrongProbablePrime};
use malachite_base::num::modular::traits::DivisibleBy;
use malachite_base::num::number_theory::traits::Gcd;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::generators::{unsigned_gen, unsigned_gen_var_37};
use malachite_base::test_util::num::factorization::is_prime_pseudosquare::*;

#[allow(clippy::unnecessary_wraps)]
fn bpsw(n: u128) -> Option<bool> {
    Some(n.is_probable_prime())
}

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
fn test_u128_is_prime_n_pm_1() {
    let test = |n: u128, out: Option<bool>| {
        assert_eq!(u128_is_prime_n_pm_1(n, bpsw), out);
    };
    // a negative value in Wide::is_square
    test(27756197243397708409, Some(true));
    // a^(n - 1) != 1 mod n in the Pocklington test
    test(24550722711038714373, Some(false));
    // gcd(c, n) > 1 in the Pocklington test
    test(3654087221953909040329, Some(false));
    // the Pocklington test tries MAX_BASES bases
    test(3566873579109787269649, None);
    // the Morrison test tries MAX_BASES values of B
    test(32084241741947701151, None);
    // Fc overflows a u128
    test(66595311765912685213099927535616001, Some(false));
    // c1^2 < 4 * c2 in the Brillhart, Lehmer, Selfridge test
    test(17333996803607330248857501716836011779, Some(true));
    // c1^2 - 4 * c2 is a square: a composite that passes the Pocklington test
    test(72826661698115367821889989, Some(false));
    // a composite that passes the Pocklington test and fails the Morrison test
    test(7606458251360678619060387029, Some(false));
    // (F2 - 1)^2 overflows, so (F2 - 1)^2 > n
    test(22193738963681804287, Some(true));
    // lcm(F1, F2)^2 overflows
    test(1905374076109737606031532189681, Some(true));
}

#[test]
fn test_u128_is_prime_morrison() {
    // U(n + 1) != 0 mod n
    assert_eq!(
        u128_is_prime_morrison(18634650061818470401, &[2, 49253], bpsw),
        (Some(false), 18634650061818470402, 1)
    );
}

#[test]
fn u128_is_prime_n_pm_1_properties() {
    unsigned_gen_var_37().test_properties(|n| {
        assert!(n.is_prime());
        assert_ne!(u128_is_prime_n_pm_1(n, bpsw), Some(false));
        if let Some(p) = u128_is_prime_pseudosquare(n) {
            assert!(p);
        }
    });

    for n in carmichael_pseudoprimes(1_400_000).take(100) {
        assert!(!n.is_prime());
        assert_ne!(u128_is_prime_n_pm_1(n, bpsw), Some(true));
    }
    for n in carmichael_pseudoprimes(14_000_000).take(100) {
        assert!(!n.is_prime());
        assert_ne!(u128_is_prime_n_pm_1(n, bpsw), Some(true));
    }
}

#[test]
fn test_u128_divisor_in_residue_class_lenstra() {
    let test = |n: u128, r: u128, s: u128, out: Option<u128>| {
        assert_eq!(u128_divisor_in_residue_class_lenstra(n, r, s), out);
    };
    // n prime: no divisors besides 1 and n
    test(
        340282366920938463463374607431768211297,
        1,
        18446744073709551557,
        None,
    );
    // d = 0 in Wide::sqrt_rem
    test(
        4350616935433401973201,
        4284678761,
        15418631960,
        Some(65959206601),
    );
    // a negative quotient in Wide::div_round_toward_zero_u64
    test(
        134147777188213541187925217,
        113059365911,
        1181188814507,
        None,
    );
    // a1 = 0 mod s, so a1 = s
    test(791043618031439356121, 13451762218, 23562466033, None);
    // r divides n
    test(
        592064504393168384713159094099,
        1198598838389,
        451913364563089,
        Some(1198598838389),
    );
    // a1 == 0, and y * s + r2 divides n
    test(27423052821567576937, 3126950, 5691709, Some(2702197));
    // a1 != 0 and b1 != 0, and d / a1 divides n
    test(
        515200916650223610948777648053381,
        613668026778819,
        793527570971242,
        Some(7595466049792536001),
    );
    // d / b1 divides n
    test(
        120835575517438054650,
        11936524,
        12964889,
        Some(389630079150),
    );
}

#[test]
fn u128_divisor_in_residue_class_lenstra_properties() {
    // n = pq, with s > n^(1/3) and r = p mod s, so that p is a divisor of n congruent to r mod s.
    let mut xs = random_primitive_ints::<u64>(EXAMPLE_SEED);
    let mut count = 0;
    while count < 1000 {
        let p_bits = 22 + xs.next().unwrap() % 42;
        let q_bits = 22 + xs.next().unwrap() % 42;
        let p = (xs.next().unwrap() >> (64 - p_bits)) | (1 << (p_bits - 1)) | 1;
        let q = (xs.next().unwrap() >> (64 - q_bits)) | (1 << (q_bits - 1)) | 1;
        if !p.is_prime() || !q.is_prime() || p == q {
            continue;
        }
        let n = u128::from(p) * u128::from(q);
        if n >> 64 == 0 {
            continue;
        }
        // n^(1/3) < s <= n^(1/2)
        let lo = n.ceiling_root(3);
        let hi = n.floor_sqrt();
        if lo >= hi {
            continue;
        }
        let s = lo + u128::from(xs.next().unwrap()) % (hi - lo) + 1;
        let r = u128::from(p) % s;
        if r == 0 || r.gcd(s) != 1 {
            continue;
        }
        count += 1;
        let d = u128_divisor_in_residue_class_lenstra(n, r, s).unwrap();
        assert!(d != 1 && d != n && n.divisible_by(d));
        assert_eq!(d % s, r);
    }
}

#[test]
fn test_u64_is_prime_pseudosquare() {
    for n in 0..100000u64 {
        assert_eq!(u64_is_prime_pseudosquare(n), n.is_prime());
    }
}

#[test]
fn u64_is_prime_pseudosquare_properties() {
    unsigned_gen::<u64>().test_properties(|n| {
        assert_eq!(u64_is_prime_pseudosquare(n), n.is_prime());
    });
}

#[test]
fn test_u128_is_prime_pseudosquare() {
    let test = |n: u128, out: Option<bool>| {
        assert_eq!(u128_is_prime_pseudosquare(n), out);
    };
    test(18446744073709551557, Some(true));
    test(18446744073709551629, Some(true));
    test(4533471823554859405148161, Some(true));
    test(3557725523452902604315321, Some(false));
    test(618970019642690137449562111, Some(true));
    test(340282366920938463463374607431768211297, None);
}

#[test]
fn u128_is_prime_pseudosquare_properties() {
    unsigned_gen::<u128>().test_properties(|n| {
        if let Some(p) = u128_is_prime_pseudosquare(n) {
            assert_eq!(p, n.is_probable_prime());
        }
    });
}
