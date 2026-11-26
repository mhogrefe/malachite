// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{CeilingRoot, FloorSqrt};
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::factorization::is_prime_n_pm_1::{
    u128_divisor_in_residue_class_lenstra, u128_is_prime_morrison, u128_is_prime_n_pm_1,
    u128_is_prime_pocklington,
};
use malachite_base::num::factorization::traits::{
    IsPrime, IsProbablePrime, IsStrongProbablePrime, Primes,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::DivisibleBy;
use malachite_base::num::number_theory::traits::Gcd;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::generators::unsigned_gen_var_37;
use malachite_nz::natural::Natural;
use malachite_nz::natural::factorization::is_prime_n_pm_1::*;
use malachite_nz::test_util::generators::natural_gen_var_10;

#[allow(clippy::unnecessary_wraps)]
fn natural_bpsw(n: &Natural) -> Option<bool> {
    Some(n.is_probable_prime())
}

#[allow(clippy::unnecessary_wraps)]
fn u128_bpsw(n: u128) -> Option<bool> {
    Some(n.is_probable_prime())
}

#[allow(clippy::unnecessary_wraps)]
fn natural_u128_bpsw(n: &Natural) -> Option<bool> {
    Some(u128::try_from(n).map_or_else(|_| n.is_probable_prime(), |n| n.is_probable_prime()))
}

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
fn test_natural_is_prime_n_pm_1() {
    let test = |s, out: Option<bool>| {
        assert_eq!(
            natural_is_prime_n_pm_1(&Natural::from_str(s).unwrap(), natural_bpsw),
            out
        );
    };
    // - the cofactor is a probable prime
    // - the cofactor of n - 1 is proven prime
    test("63161458952695267098826952693", Some(false));
    // the Pocklington test tries MAX_BASES bases
    test("3557725523452902604315321", None);
    // gcd(c, n) > 1 in the Pocklington test
    test("3560180573160531764146441", Some(false));
    // - a composite that passes the Pocklington test and fails the Morrison test
    // - m.get_bit(i) and !m.get_bit(i) in chain_full
    // - !k.get_bit(i) for the trailing bits in chain_mul, and later bits, some set
    // - t odd in v_to_u
    // - (D / n) != -1 for a = 3
    // - U(n + 1) != 0 mod n
    test("7606458251360678619060387029", Some(false));
    // t even in v_to_u
    test("3516933665536429328255189", Some(false));
    // - D < 0
    // - B fails for some q in the Morrison test, so the next B is tried
    // - (F2 - 1)^2 > n
    test("25940733853654056959", Some(true));
    // d < 0 in Lenstra's search
    test("1955726010998559856033", Some(true));
    // a cofactor of n ∓ 1 is a base-2 strong probable prime
    test("6918168948694482649459111379", Some(false));
    // - Fc / 2 is tested for feasibility
    // - F1^3 >= n: the Brillhart, Lehmer, Selfridge test
    // - c1^2 - 4 * c2 is a square: a composite that passes the Pocklington test
    test("72826661698115367821889989", Some(false));
    // the n ± 1 tests are infeasible
    test("2987890803607662983827793", None);
    // F1^2 >= n
    test("19599665578316398591", Some(true));
    // c1^2 < 4 * c2 in the Brillhart, Lehmer, Selfridge test
    test("17333996803607330248857501716836011779", Some(true));
    // lcm(F1, F2)^2 > n
    test("21388606831909575289", Some(true));
    // the Morrison test tries MAX_BASES values of B
    test("32084241741947701151", None);
}

#[test]
fn test_natural_divisor_in_residue_class_lenstra() {
    let test = |n, r, s, out: Option<&str>| {
        assert_eq!(
            natural_divisor_in_residue_class_lenstra(
                &Natural::from_str(n).unwrap(),
                &Natural::from_str(r).unwrap(),
                &Natural::from_str(s).unwrap()
            ),
            out.map(|d| Natural::from_str(d).unwrap())
        );
    };
    // a1 = 0 mod s, so a1 = s
    test(
        "82638531881657401066017709023064249",
        "92474358423078527",
        "287469184229644693",
        None,
    );
    // r divides n
    test(
        "13441485661156449826595319292658184990765356697115478900051",
        "116975925975290502257",
        "79573203819100991004002556229",
        Some("116975925975290502257"),
    );
    // a1 == 0, and y * s + r2 divides n
    test(
        "139225578909909402959957563731409",
        "105800685851",
        "106810163683",
        Some("106810163683"),
    );
    // a1 != 0 and b1 != 0, and d / a1 divides n
    test(
        "44914556355953928301808929",
        "2201356412794",
        "4500475900533",
        Some("6701832313327"),
    );
    // d / b1 divides n
    test(
        "12797090377983803845483307",
        "188489335",
        "467771647",
        Some("30863569256491"),
    );
}

#[test]
fn natural_is_prime_n_pm_1_properties() {
    natural_gen_var_10().test_properties(|n| {
        assert!(n.is_prime());
    });

    // The Natural n ± 1 tests agree with the u128 ones on two-limb inputs.
    unsigned_gen_var_37().test_properties(|x| {
        let n = Natural::from(x);
        assert_eq!(
            natural_is_prime_n_pm_1(&n, natural_u128_bpsw),
            u128_is_prime_n_pm_1(x, u128_bpsw)
        );
        assert_eq!(n.is_prime(), x.is_prime());
    });

    for n in carmichael_pseudoprimes(10_000_000_000_000).take(20) {
        assert!(!n.is_prime());
        assert_ne!(natural_is_prime_n_pm_1(&n, natural_bpsw), Some(true));
    }
}

#[test]
fn natural_is_prime_pocklington_and_morrison_properties() {
    // The Natural tests agree with the u128 ones on two-limb inputs, given the same primes.
    unsigned_gen_var_37().test_properties(|x| {
        let n = Natural::from(x);
        let ps: Vec<u64> = u64::primes_less_than(&1000).collect();
        let pm1: Vec<u64> = ps
            .iter()
            .copied()
            .filter(|&p| (x - 1) % u128::from(p) == 0)
            .collect();
        let pp1: Vec<u64> = ps
            .iter()
            .copied()
            .filter(|&p| (x + 1) % u128::from(p) == 0)
            .collect();
        let (r_alt, f_alt, c_alt) = u128_is_prime_pocklington(x, &pm1, u128_bpsw);
        assert_eq!(
            natural_is_prime_pocklington(&n, &pm1, natural_u128_bpsw),
            (r_alt, Natural::from(f_alt), Natural::from(c_alt))
        );
        let (r_alt, f_alt, c_alt) = u128_is_prime_morrison(x, &pp1, u128_bpsw);
        assert_eq!(
            natural_is_prime_morrison(&n, &pp1, natural_u128_bpsw),
            (r_alt, Natural::from(f_alt), Natural::from(c_alt))
        );
    });
}

#[test]
fn natural_divisor_in_residue_class_lenstra_properties() {
    // n = pq, with n^(1/3) < s <= n^(1/2) and r = p mod s, so that p is a divisor of n congruent to
    // r mod s. On two-limb inputs, the Natural search agrees with the u128 one.
    let mut xs = random_primitive_ints::<u64>(EXAMPLE_SEED);
    let mut count = 0;
    while count < 1000 {
        let p_bits = 22 + xs.next().unwrap() % 140;
        let q_bits = 22 + xs.next().unwrap() % 140;
        let p = random_natural_with_bits(&mut xs, p_bits);
        let q = random_natural_with_bits(&mut xs, q_bits);
        if p == q || !p.is_probable_prime() || !q.is_probable_prime() {
            continue;
        }
        let n = &p * &q;
        if n.significant_bits() <= 64 {
            continue;
        }
        let lo = (&n).ceiling_root(3);
        let hi = (&n).floor_sqrt();
        if lo >= hi {
            continue;
        }
        let range = &hi - &lo;
        let s = lo
            + random_natural_with_bits(&mut xs, range.significant_bits() + 8) % range
            + Natural::ONE;
        let r = &p % &s;
        if r == 0u32 || (&r).gcd(&s) != 1u32 {
            continue;
        }
        count += 1;
        let d = natural_divisor_in_residue_class_lenstra(&n, &r, &s).unwrap();
        assert!(d != 1u32 && d != n && (&n).divisible_by(&d));
        assert_eq!(&d % &s, r);
        if let Ok(x) = u128::try_from(&n) {
            let alt = u128_divisor_in_residue_class_lenstra(
                x,
                u128::try_from(&r).unwrap(),
                u128::try_from(&s).unwrap(),
            )
            .unwrap();
            assert_eq!(d, alt);
        }
    }
}

fn random_natural_with_bits(xs: &mut impl Iterator<Item = u64>, bits: u64) -> Natural {
    let mut x = Natural::ZERO;
    for i in 0..bits.div_ceil(64) {
        x |= Natural::from(xs.next().unwrap()) << (i << 6);
    }
    let x = x >> (bits.div_ceil(64) * 64 - bits);
    x | (Natural::ONE << (bits - 1)) | Natural::ONE
}
