// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivisibleBy, Lcm};
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::landau_function::landau_function_prefix;

#[test]
fn test_landau_function_prefix() {
    // - the empty and one-element prefixes, and the g(0) = g(1) = 1 edge
    assert!(landau_function_prefix(0).is_empty());
    assert_eq!(landau_function_prefix(1), vec![Natural::from(1u32)]);
    // - the OEIS A000793 prefix, crossing the first repeated values (g(5) = g(6) = 6) and the first
    //   prime-power parts beyond single primes
    let prefix = landau_function_prefix(20);
    assert_eq!(
        prefix.iter().map(ToString::to_string).collect::<Vec<_>>(),
        [
            "1", "1", "2", "3", "4", "6", "6", "12", "15", "20", "30", "30", "60", "60", "84",
            "105", "140", "210", "210", "420"
        ]
    );
    // - a mid-size value (computed independently with python)
    assert_eq!(landau_function_prefix(101)[100].to_string(), "232792560");
    // - a value whose optimal partition includes prime powers past the word-ladder's early levels,
    //   and a prefix long enough for the pmax prime bound to bind
    assert_eq!(
        landau_function_prefix(300)[299].to_string(),
        "179967741245412120"
    );
}

#[test]
fn landau_function_properties() {
    let prefix = landau_function_prefix(150);
    for n in 1..150usize {
        // g is nondecreasing
        assert!(prefix[n] >= prefix[n - 1], "monotone at {n}");
        // each value is a genuine lcm-of-partition witness: g(n) is divisible by g(n - p^k)
        // whenever the DP chose that step, but universally, g(n) >= g(n - 1) and g(n) divides
        // lcm(1..n); check the latter
    }
    let mut l = Natural::from(1u32);
    for i in 1..150u32 {
        l = (&l).lcm(Natural::from(i));
    }
    for (n, g) in prefix.iter().enumerate() {
        assert!((&l).divisible_by(g), "g({n}) divides lcm(1..149)");
    }
}
