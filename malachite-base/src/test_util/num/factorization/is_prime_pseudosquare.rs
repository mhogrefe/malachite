// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::Parity;
use crate::num::factorization::is_prime::{redc_context_128, redc_pow_128, to_redc_128};
use crate::num::factorization::traits::Primes;
use crate::num::modular::traits::{DivisibleBy, ModPow, ModPowerOf2};

// The primality proofs here use the pseudosquares L_p: the least positive nonsquare L = 1 mod 8
// that is a quadratic residue of every odd prime q <= p. If n has no prime factor up to a bound B,
// and n / B < L_p, then n is prime if and only if q^((n - 1) / 2) = ±1 mod n for every prime q <=
// p, with -1 occurring for some q when n = 1 mod 8, and with 2^((n - 1) / 2) = -1 mod n when n = 5
// mod 8 (Lukes, Patterson, and Williams).

// This is FLINT_PSEUDOSQUARES_CUTOFF from ulong_extras.h, FLINT 3.7.0-dev.
const PSEUDOSQUARES_CUTOFF: usize = 1000;

// These are flint_pseudosquares from ulong_extras/is_prime_pseudosquare.c, FLINT 3.7.0-dev, where
// FLINT64 is defined: L_p for the first 52 odd primes p (repeated where L_p does not change).
const PSEUDOSQUARES: [u64; 52] = [
    17,
    73,
    241,
    1009,
    2641,
    8089,
    18001,
    53881,
    87481,
    117049,
    515761,
    1083289,
    3206641,
    3818929,
    9257329,
    22000801,
    48473881,
    48473881,
    175244281,
    427733329,
    427733329,
    898716289,
    2805544681,
    2805544681,
    2805544681,
    10310263441,
    23616331489,
    85157610409,
    85157610409,
    196265095009,
    196265095009,
    2871842842801,
    2871842842801,
    2871842842801,
    26250887023729,
    26250887023729,
    112434732901969,
    112434732901969,
    112434732901969,
    178936222537081,
    178936222537081,
    696161110209049,
    696161110209049,
    2854909648103881,
    6450045516630769,
    6450045516630769,
    11641399247947921,
    11641399247947921,
    190621428905186449,
    196640248121928601,
    712624335095093521,
    1773855791877850321,
];

// These are flint_fmpz_pseudosquares from fmpz/is_prime_pseudosquare.c, FLINT 3.7.0-dev, where
// FLINT64 is defined, as u128s: the 52 values above, followed by 22 more.
const FMPZ_PSEUDOSQUARES: [u128; 74] = {
    let mut ps = [0; 74];
    let mut i = 0;
    while i < 52 {
        ps[i] = PSEUDOSQUARES[i] as u128;
        i += 1;
    }
    let more: [(u64, u64); 22] = [
        (2327687064124474441, 0),
        (6384991873059836689, 0),
        (8019204661305419761, 0),
        (10198100582046287689, 0),
        (10198100582046287689, 0),
        (10198100582046287689, 0),
        (14508056099771532121, 3),
        (6022180988239908185, 11),
        (17043829275960758433, 28),
        (14326875581237116289, 50),
        (14326875581237116289, 50),
        (2380547819961928673, 116),
        (2380547819961928673, 116),
        (2380547819961928673, 116),
        (17010621086940159057, 739),
        (7213663464718498801, 1875),
        (9655140963601468961, 5393),
        (9655140963601468961, 5393),
        (12668036736679956625, 16011),
        (12668036736679956625, 16011),
        (1410807067550026393, 198156),
        (3271894284933966433, 229581),
    ];
    while i < 74 {
        let (lo, hi) = more[i - 52];
        ps[i] = ((hi as u128) << 64) | lo as u128;
        i += 1;
    }
    ps
};

fn first_primes(n: usize) -> Vec<u64> {
    u64::primes().take(n).collect()
}

// Proves a u64 prime or composite with pseudosquares.
//
// This is n_is_prime_pseudosquare from ulong_extras/is_prime_pseudosquare.c, FLINT 3.7.0-dev, where
// FLINT64 is defined.
pub fn u64_is_prime_pseudosquare(n: u64) -> bool {
    if n < 2 {
        return false;
    } else if n.even() {
        return n == 2;
    }
    let primes = first_primes(PSEUDOSQUARES_CUTOFF + 1);
    for &p in &primes[..PSEUDOSQUARES_CUTOFF] {
        if p * p > n {
            return true;
        }
        if n.divisible_by(p) {
            return false;
        }
    }
    let b = primes[PSEUDOSQUARES_CUTOFF];
    let nb = (n - 1) / b + 1;
    let mut m1 = false;
    let i = PSEUDOSQUARES
        .iter()
        .position(|&l| l > nb)
        .unwrap_or(PSEUDOSQUARES.len());
    let exp = (n - 1) >> 1;
    for &p in &primes[..=i] {
        let m = p.mod_pow(exp, n);
        if m != 1 && m != n - 1 {
            return false;
        } else if m == n - 1 {
            m1 = true;
        }
    }
    let mod8 = n.mod_power_of_2(3);
    if mod8 == 3 || mod8 == 7 {
        return true;
    }
    if mod8 == 5 {
        assert_eq!(
            2u64.mod_pow(exp, n),
            n - 1,
            "{n} is a probable prime, but not prime"
        );
        return true;
    }
    if m1 {
        return true;
    }
    for &p in &primes[i + 1..=PSEUDOSQUARES.len()] {
        let m = p.mod_pow(exp, n);
        if m == n - 1 {
            return true;
        }
        assert_eq!(m, 1, "{n} is a probable prime, but not prime");
    }
    panic!("{n} is a probable prime, but not prime");
}

// Proves a u128 prime or composite with pseudosquares, or returns `None` if n is too large for the
// table of pseudosquares, which it is from about 2^94.7 on.
//
// This is fmpz_is_prime_pseudosquare from fmpz/is_prime_pseudosquare.c, FLINT 3.7.0-dev, where
// FLINT64 is defined.
pub fn u128_is_prime_pseudosquare(n: u128) -> Option<bool> {
    if let Ok(n) = u64::try_from(n) {
        return Some(u64_is_prime_pseudosquare(n));
    }
    let primes = first_primes(PSEUDOSQUARES_CUTOFF + 1);
    if primes[..PSEUDOSQUARES_CUTOFF]
        .iter()
        .any(|&p| n.divisible_by(u128::from(p)))
    {
        return Some(false);
    }
    let b = u128::from(primes[PSEUDOSQUARES_CUTOFF]);
    let nm1 = n - 1;
    let nb = nm1 / b + 1;
    let mut m1 = false;
    let i = FMPZ_PSEUDOSQUARES.iter().position(|&l| l > nb)?;
    let exp = nm1 >> 1;
    let ctx = redc_context_128(n);
    let minus_one = n - ctx.one;
    let pow = |p: u64| redc_pow_128(to_redc_128(u128::from(p), &ctx), exp, &ctx);
    for &p in &primes[..=i] {
        let m = pow(p);
        if m != ctx.one && m != minus_one {
            return Some(false);
        }
        if m == minus_one {
            m1 = true;
        }
    }
    let mod8 = n.mod_power_of_2(3);
    if mod8 == 3 || mod8 == 7 {
        return Some(true);
    }
    if mod8 == 5 {
        assert_eq!(pow(2), minus_one, "{n} is a probable prime, but not prime");
        return Some(true);
    }
    if m1 {
        return Some(true);
    }
    for &p in &primes[i + 1..=FMPZ_PSEUDOSQUARES.len()] {
        let m = pow(p);
        if m == minus_one {
            return Some(true);
        }
        assert_eq!(m, ctx.one, "{n} is a probable prime, but not prime");
    }
    panic!("{n} is a probable prime, but not prime");
}
