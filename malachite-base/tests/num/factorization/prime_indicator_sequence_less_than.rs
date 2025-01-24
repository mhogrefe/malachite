// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::itertools::Itertools;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::primes::{
    prime_indicator_sequence, prime_indicator_sequence_less_than,
};
use malachite_base::test_util::generators::unsigned_gen_var_5;

fn test_prime_indicator_sequence_helper(limit: u64, out: &str) {
    let s: String = prime_indicator_sequence_less_than(limit)
        .take(1000)
        .map(|b| if b { '1' } else { '0' })
        .collect();
    assert_eq!(s, out);
}

#[test]
pub fn test_prime_indicator_sequence_less_than() {
    test_prime_indicator_sequence_helper(0, "");
    test_prime_indicator_sequence_helper(1, "");
    test_prime_indicator_sequence_helper(2, "0");
    test_prime_indicator_sequence_helper(3, "01");
    test_prime_indicator_sequence_helper(4, "011");
    test_prime_indicator_sequence_helper(5, "0110");
    test_prime_indicator_sequence_helper(6, "01101");
    test_prime_indicator_sequence_helper(7, "011010");
    test_prime_indicator_sequence_helper(8, "0110101");
    test_prime_indicator_sequence_helper(9, "01101010");
    test_prime_indicator_sequence_helper(10, "011010100");
    test_prime_indicator_sequence_helper(
        100,
        "01101010001010001010001000001010000010001010001000001000001010000010001010000010001000001\
        0000000100",
    );
    test_prime_indicator_sequence_helper(
        1000,
        "01101010001010001010001000001010000010001010001000001000001010000010001010000010001000001\
        000000010001010001010001000000000000010001000001010000000001010000010000010001000001000001\
        010000000001010001010000000000010000000000010001010001000001010000000001000001000001000001\
        010000010001010000000001000000000000010001010001000000000000010000010000000001010001000001\
        000000010000010000010001000001000000010001000000010000000001010000000001010000010001000001\
        000000010001010001000000000001000000010001000000010001000001000000000001010000000000000000\
        010000010000000001000001000001010000010000000001000001000001010000010000010001010000000000\
        010000000001010001000001000001010000000000010001000001000000010000000001000000010000000001\
        000000010000010000010001000000010000010001000000010001000000000000010000000001000000000001\
        010000000001010001010000000001000000000000010001010001000000000000010001010001000000000000\
        000000010001000000010000000001000000010001000001000001000000000000010001000001000001000000\
        0100000100",
    );
}

#[test]
fn prime_indicator_sequence_less_than_properties() {
    unsigned_gen_var_5().test_properties(|limit| {
        let bs = prime_indicator_sequence_less_than(limit).collect_vec();
        let len = usize::exact_from(limit.saturating_sub(1));
        assert_eq!(bs.len(), len);
        assert_eq!(prime_indicator_sequence().take(len).collect_vec(), bs);
    });
}
