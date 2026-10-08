// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::HeightRef;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::arithmetic::max_bits::vec_max_bits;
use malachite_nz::test_util::generators::{integer_polynomial_gen, integer_vec_gen};
use malachite_nz::test_util::integer_vector::arithmetic::max_bits::vec_max_bits_naive;

#[test]
fn test_vec_max_bits() {
    let test = |xs: &[&str], out| {
        let xs: Vec<Integer> = xs.iter().map(|x| Integer::from_str(x).unwrap()).collect();
        assert_eq!(vec_max_bits(&xs), out);
        assert_eq!(vec_max_bits_naive(&xs), out);
    };
    // - !goto_bignum
    test(&[], (0, false));
    test(&["0"], (0, false));
    test(&["1", "-2", "3"], (2, true));
    // - goto_bignum
    // - len == max_limbs
    test(&["5431415992672452776"], (63, false));
    // - is_small(x) in the bignum loop
    test(&["5808697462834698497", "0"], (63, false));
    // - len > max_limbs
    test(&["89519572834232417966"], (67, false));
    test(&["18446744073709551616", "1", "-1"], (65, true));
    test(
        &["36893488147419103232", "-18446744073709551616"],
        (66, true),
    );
    // - len < max_limbs
    test(
        &["340282366920938463463374607431768211456", "18446744073709551616"],
        (129, false),
    );
}

#[test]
fn vec_max_bits_properties() {
    integer_vec_gen().test_properties(|xs| {
        let (bits, negative) = vec_max_bits(&xs);
        assert_eq!(vec_max_bits_naive(&xs), (bits, negative));
        // The count is 0 exactly when every element is 0.
        assert_eq!(bits == 0, xs.iter().all(|x| *x == 0u32));
        assert!(xs.iter().all(|x| x.significant_bits() <= bits));
        assert_eq!(negative, xs.iter().any(|x| *x < 0u32));
    });

    integer_polynomial_gen().test_properties(|p| {
        // For a polynomial, the count is the number of significant bits of the height.
        assert_eq!(
            vec_max_bits(p.coefficients_asc()).0,
            p.height_ref().significant_bits()
        );
    });
}
