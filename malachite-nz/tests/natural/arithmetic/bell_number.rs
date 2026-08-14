// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{BellNumber, BinomialCoefficient};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::bell_number::{
    bell_numbers_prefix, bell_numbers_prefix_multi_mod, exhaustive_bell_numbers,
};

#[test]
fn test_bell_number() {
    let test = |n, out: &str| {
        assert_eq!(Natural::bell_number(n).to_string(), out);
    };
    // (expected values computed independently with python, checked against OEIS A000110)
    // - table values, including both edges (0 and 25)
    // - the triangle tiers: first computed value (26); 26 through 42 end in the two-word phase,
    //   with 42 the last; 43 through 58 reach the three-word phase, with 58 the last
    // - the multi-mod path: 59 is the first value past the triangle, and any multi-mod row also
    //   exercises the prime search past composites, the divisor-table sieve's prime and composite
    //   arms, and both parities of the alternating sum
    // - the 192-bit accumulator's carry word fires only once enough near-maximal products
    //   accumulate: 300 is the ONLY row that exercises it (zero carries at every n up to 150, 25
    //   carries at 300), so this row must not be shrunk
    test(0, "1");
    test(1, "1");
    test(2, "2");
    test(3, "5");
    test(25, "4638590332229999353");
    test(26, "49631246523618756274");
    test(42, "35742549198872617291353508656626642567");
    test(43, "552950118797165484321714693280737767385");
    test(
        58,
        "2507136358984296114560786627437574942253015623445622326263",
    );
    test(
        59,
        "49176743336309621659000944152624896853591018248919168867818",
    );
    test(
        100,
        "475853912767648336587907688413872078263636696868256114666163346375591144978924426226727240\
        44217756306953557882560751",
    );
    test(
        300,
        "959371716083927027730901259745824464366976125348652409046510145230850544907555579496709799\
        142209444781336170346170352748392345291060010709424197788352499537914256940310914826447949\
        395189961813099149494692401231162646683541446980527690006673361217561798767040997641677127\
        264331114304587320531501160780182462582786582463894498265316092431820400318291048940208208\
        112803846317328016001249011765970685010420303590751027295294867366087340556636411710038009\
        9645",
    );
}

#[test]
fn bell_number_recurrence() {
    // B(n + 1) = sum_{k=0}^{n} C(n, k) B(k), walked far enough to cross the table edge (26), both
    // triangle width boundaries (42, 58), and into the multi-modular tier.
    let mut bells = vec![Natural::bell_number(0)];
    for n in 0..120u64 {
        let mut next = Natural::from(0u32);
        for (k, b) in bells.iter().enumerate() {
            next += Natural::binomial_coefficient(Natural::from(n), Natural::from(k as u64)) * b;
        }
        bells.push(next);
        assert_eq!(
            bells[usize::try_from(n).unwrap() + 1],
            Natural::bell_number(n + 1),
            "B({})",
            n + 1
        );
    }
}

#[test]
fn bell_number_properties() {
    // Touchard's congruence: B(n + p) = B(n) + B(n + 1) mod p for prime p.
    unsigned_gen_var_5::<u8>().test_properties(|n| {
        let n = u64::from(n);
        for p in [2u64, 3, 5, 7, 11, 13, 31] {
            let lhs = Natural::bell_number(n + p) % Natural::from(p);
            let rhs = (Natural::bell_number(n) + Natural::bell_number(n + 1)) % Natural::from(p);
            assert_eq!(lhs, rhs, "Touchard at n = {n}, p = {p}");
        }
    });
}

#[test]
fn bell_numbers_iterator_agrees_with_bell_number() {
    // The iterator is the bignum Bell triangle; check it against the tiered single-value function
    // across the table, the fixed-width triangle, and into the multi-modular range.
    for (n, b) in exhaustive_bell_numbers().take(120).enumerate() {
        assert_eq!(b, Natural::bell_number(u64::exact_from(n)), "B({n})");
    }
}

#[test]
fn test_bell_numbers_prefix() {
    // - the empty prefix, and the one-element prefix
    assert!(bell_numbers_prefix(0).is_empty());
    assert_eq!(bell_numbers_prefix(1), vec![Natural::from(1u32)]);
    // - a prefix served by the triangle, agreeing with the iterator
    let prefix = bell_numbers_prefix(80);
    assert_eq!(prefix.len(), 80);
    for (n, b) in exhaustive_bell_numbers().take(80).enumerate() {
        assert_eq!(prefix[n], b, "prefix[{n}]");
    }
    // - the multimodular batch, tested directly at a length the dispatch would send to the
    //   triangle, so that the per-entry prime slicing is exercised affordably; entries 0 and 1 take
    //   the one-prime minimum, and the tail needs several
    let batch = bell_numbers_prefix_multi_mod(120);
    for (n, b) in exhaustive_bell_numbers().take(120).enumerate() {
        assert_eq!(batch[n], b, "batch[{n}]");
    }
    // - the word triangle's length guards: length 1 fills only the leading entry, length 2 also the
    //   second, and length 3 is the first to run the triangle proper
    assert_eq!(bell_numbers_prefix_multi_mod(1), vec![Natural::from(1u32)]);
    assert_eq!(
        bell_numbers_prefix_multi_mod(2),
        vec![Natural::from(1u32), Natural::from(1u32)]
    );
    assert_eq!(
        bell_numbers_prefix_multi_mod(3),
        vec![Natural::from(1u32), Natural::from(1u32), Natural::from(2u32)]
    );
}

#[test]
fn test_bell_numbers_prefix_dispatch_high() {
    // - the dispatch's multimodular arm engages exactly at the threshold; this row is release-scale
    //   (the length-5000 batch runs ~90 primes over a 25-million-step triangle each) and pins the
    //   arm that no smaller row can reach
    let prefix = bell_numbers_prefix(5000);
    assert_eq!(prefix.len(), 5000);
    assert_eq!(prefix[0], Natural::from(1u32));
    assert_eq!(prefix[4999], Natural::bell_number(4999));
}
