// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::arithmetic::max_limbs::vec_max_limbs;
use malachite_nz::test_util::generators::integer_vec_gen;
use malachite_nz::test_util::integer_vector::arithmetic::max_limbs::vec_max_limbs_naive;

fn parse(xs: &[&str]) -> Vec<Integer> {
    xs.iter().map(|x| Integer::from_str(x).unwrap()).collect()
}

#[test]
fn test_vec_max_limbs() {
    let test = |xs: &[&str], out| {
        let xs = parse(xs);
        assert_eq!(vec_max_limbs(&xs), out);
        assert_eq!(vec_max_limbs_naive(&xs), out);
    };
    test(&[], 0);
    test(&["0", "0"], 0);
    // - limbs > max_limbs
    test(&["1", "-2"], 1);
    // - limbs <= max_limbs
    #[cfg(not(feature = "32_bit_limbs"))]
    test(&["-18446744073709551616", "5"], 2);
    #[cfg(feature = "32_bit_limbs")]
    test(&["-18446744073709551616", "5"], 3);
    #[cfg(not(feature = "32_bit_limbs"))]
    test(
        &["3", "-340282366920938463463374607431768211456", "18446744073709551616"],
        3,
    );
    #[cfg(feature = "32_bit_limbs")]
    test(
        &["3", "-340282366920938463463374607431768211456", "18446744073709551616"],
        5,
    );
}

#[test]
fn vec_max_limbs_properties() {
    integer_vec_gen().test_properties(|xs| {
        let limbs = vec_max_limbs(&xs);
        assert_eq!(vec_max_limbs_naive(&xs), limbs);
        // Negating every element changes nothing.
        let negated: Vec<Integer> = xs.iter().map(|x| -x).collect();
        assert_eq!(vec_max_limbs(&negated), limbs);
    });
}
